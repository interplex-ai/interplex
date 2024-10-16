use ::interplex_ai_schemas_community_neoeinstein_prost::schema::v1::SetRequest;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_client::*;
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::*;

// mod interplex_ai_schemas_community_neoeinstein_prost {
//     tonic::include_proto!("interplex_ai_schemas_community_neoeinstein_prost.schema.v1");
// }

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut client = CacheServiceClient::connect("http://0.0.0.0:8080").await?;

    let request = tonic::Request::new(SetRequest {
        key: "1".to_string(),
        value: "2".to_string()
    });

    let response = client.set(request).await?;

    println!("RESPONSE={:?}", response);

    Ok(())
}