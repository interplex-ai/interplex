use clap::Parser;
use crate::server::ServerConfiguration;
use crate::server::ServerFactory;
use log::info;
mod cache;
mod server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    memory_only_cache: bool
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    
    let arg = Args::parse();
    
    env_logger::init();
    let server_config = ServerConfiguration { port: 8080 , memory_only_cache: arg.memory_only_cache, 
    cache_path: "./cached".to_string()};
 
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
