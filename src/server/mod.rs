use crate::server::cache::MyCacheService;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::CacheServiceServer;
use log::info;
use std::error::Error;
use tonic::transport::Server as tserver;

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
            server_config: self.server_configuration,
        }
    }
}
#[derive(Default, Copy, Clone)]
pub struct ServerConfiguration {
    pub(crate) port: i64,
}
pub struct Server {
    server_config: ServerConfiguration,
}

impl Server {
    pub async fn start(&self) -> Result<(), Box<dyn Error>> {
        let port = self.server_config.port.to_string();
        let addr = format!("0.0.0.0:{}", port).parse().unwrap();
        info!("Server listening on port {}", port);
        let my_cache_service = MyCacheService;
        Ok(tserver::builder()
            .add_service(CacheServiceServer::new(my_cache_service))
            .serve(addr)
            .await?)
    }
}

#[cfg(test)]
mod tests {
    use crate::server::{ServerConfiguration, ServerFactory};
    #[test]
    pub fn test_server_port() {
        let config = ServerConfiguration { port: 8080 };
        let factory = ServerFactory::default().with_configuration(config);
        let server = factory.build();
        assert_eq!(server.server_config.port, 8080);
    }
}
