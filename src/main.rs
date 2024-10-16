use crate::server::ServerConfiguration;
use crate::server::ServerFactory;
use log::info;
mod server;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::init();
    let server_config = ServerConfiguration { port: 8080 };
    info!("Starting interplex");
    let server_factory = ServerFactory::default()
        .with_configuration(server_config)
        .build();
    server_factory
        .start()
        .await
        .expect("failed to start server");

    Ok(())
}
