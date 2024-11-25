use crate::server::ServerConfiguration;
use crate::server::ServerFactory;
use clap::Parser;
use log::info;
use std::env;
mod cache;
mod server;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[arg(short, long, default_value_t = false)]
    memory_only_cache: bool,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let arg = Args::parse();

    let home_dir = env::home_dir().expect("Home directory not found");

    let default_cache_path = format!("{}/.cached", home_dir.as_path().to_str().unwrap());

    env_logger::init();
    let server_config = ServerConfiguration {
        port: 8080,
        memory_only_cache: arg.memory_only_cache,
        cache_path: default_cache_path,
    };

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
