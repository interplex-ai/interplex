use ::interplex_ai_schemas_community_neoeinstein_prost::schema::v1::SetRequest;
use ::interplex_ai_schemas_community_neoeinstein_prost::schema::v1::GetRequest;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_client::CacheServiceClient;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::*;
use tokio::time::Instant;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CacheServiceClient::connect("http://0.0.0.0:8080").await?;

    // Start timer
    let start = Instant::now();

    // Set 1000 values
    for i in 0..1000 {
        let request = tonic::Request::new(SetRequest {
            key: i.to_string(),
            value: (i * 2).to_string(), // just an example value
        });
        client.set(request).await?;
    }

    // Get 1000 values
    for i in 0..1000 {
        let request = tonic::Request::new(GetRequest {
            key: i.to_string(),
        });
        let response = client.get(request).await?;
        println!("RESPONSE={:?}", response);
    }

    // End timer and print duration
    let duration = start.elapsed();
    println!("Total time taken: {:?}", duration);

    Ok(())
}
