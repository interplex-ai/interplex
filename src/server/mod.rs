use crate::server::cache::MyCacheService;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::CacheServiceServer;
use log::info;
use std::error::Error;
use tonic::transport::Server as tserver;
use tonic_reflection::server::Builder;

mod cache;

#[derive(Default)]
pub struct ServerFactory {
    server_configuration: ServerConfiguration,
}
impl ServerFactory {
    pub fn with_configuration(mut self, server_configuration: ServerConfiguration) -> Self {
        self.server_configuration = server_configuration;
        self
    }
    pub fn build(&self) -> Server {
        Server {
            server_config: self.server_configuration.clone(),
        }
    }
}
#[derive(Default, Clone)]
pub struct ServerConfiguration {
    pub(crate) port: i64,
    pub(crate) memory_only_cache: bool,
    pub(crate) cache_path: String,
}
pub struct Server {
    server_config: ServerConfiguration,
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let port = self.server_config.port.to_string();
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        info!("Server listening on port {}", port);
        let my_cache_service = MyCacheService::new(
            self.server_config.memory_only_cache,
            self.server_config.cache_path.as_str(),
        );

        let reflection_service = Builder::configure()
            .register_encoded_file_descriptor_set(
                interplex_ai_schemas_community_neoeinstein_prost::schema::v1::FILE_DESCRIPTOR_SET,
            )
            .build()
            .unwrap();

        Ok(tserver::builder()
            .add_service(CacheServiceServer::new(my_cache_service))
            .add_service(reflection_service)
            .serve(addr)
            .await?)
    }
}

// Integration test for the server

#[cfg(test)]
mod tests {
    use super::*;
    use interplex_ai_schemas_community_neoeinstein_prost::schema::v1::{DeleteRequest, GetRequest, SetRequest};
    use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_client::CacheServiceClient;
    use std::env;
    use std::path::PathBuf;

    use tokio::runtime::Runtime;
    use tokio::time::Instant;
    use tonic::Request;

    fn get_temp_dir_path() -> PathBuf {
        env::temp_dir()
    }

    #[test]
    fn test_server_startup() {
        // Initialize the tokio runtime
        let rt = Runtime::new().unwrap();

        let tmpfile = format!("{}/{}", get_temp_dir_path().to_str().unwrap(), "cached");
        // Create server configuration
        let server_config = ServerConfiguration {
            port: 50051,
            memory_only_cache: false,
            cache_path: tmpfile.to_string(),
        };

        // Build and start the server
        let server = ServerFactory::default()
            .with_configuration(server_config.clone())
            .build();
        rt.spawn(async move {
            server.start().await.expect("Server failed to start");
        });

        // Allow the server some time to start
        rt.block_on(async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });

        // Create a gRPC client and attempt to connect to the server
        let addr = format!("http://localhost:{}", server_config.clone().port);
        let mut client = rt.block_on(async {
            CacheServiceClient::connect(addr)
                .await
                .expect("Failed to connect to server")
        });

        // Perform a simple gRPC call to check if the server responds
        let request = Request::new(SetRequest {
            key: "1".to_string(),
            value: "2".to_string(), // populate the request with appropriate fields
        });
        let response = rt.block_on(async {
            client
                .set(request)
                .await
                .expect("Failed to get response on set_request")
        });

        // Let's now get the cache response
        let get_request = GetRequest {
            key: "1".to_string(),
        };
        let response = rt.block_on(async {
            client
                .get(get_request)
                .await
                .expect("Failed to get response on get_request")
        });

        assert_eq!(response.into_inner().value, "2");

        // Let's delete from the cache

        let delete_request = DeleteRequest {
            key: "1".to_string(),
        };

        let response = rt.block_on(async {
            client.delete(delete_request).await.expect("Failed to delete get_request")
        });

        // delete the caching file
        std::fs::remove_dir_all(tmpfile).expect("Failed to delete cache file");
    }
    #[test]
    fn test_server_performance() {
        let rt = Runtime::new().unwrap();

        let tmpfile = format!("{}/{}", get_temp_dir_path().to_str().unwrap(), "cached2");
        let server_config = ServerConfiguration {
            port: 50052,
            memory_only_cache: false,
            cache_path: tmpfile.clone(),
        };

        let server = ServerFactory::default()
            .with_configuration(server_config.clone())
            .build();

        rt.spawn(async move {
            server.start().await.expect("Server failed to start");
        });

        rt.block_on(async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });

        let addr = format!("http://localhost:{}", server_config.port);
        let mut client = rt.block_on(async {
            CacheServiceClient::connect(addr)
                .await
                .expect("Failed to connect to server")
        });

        let start = Instant::now();

        for i in 0..1000 {
            let request = Request::new(SetRequest {
                key: i.to_string(),
                value: (i * 2).to_string(),
            });
            rt.block_on(async {
                client.set(request).await.expect("Failed to set value");
            });
        }

        for i in 0..1000 {
            let request = Request::new(GetRequest { key: i.to_string() });
            rt.block_on(async {
                client.get(request).await.expect("Failed to get value");
            });
        }

        let duration = start.elapsed();
        println!("Total time taken: {:?}", duration);

        std::fs::remove_dir_all(tmpfile).expect("Failed to delete cache file");
    }
}
