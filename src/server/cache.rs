use interplex_ai_schemas_community_neoeinstein_prost::schema::v1::{SetRequest, SetResponse};
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::CacheService;
use tonic::{async_trait, Request, Response, Status};

#[derive(Default)]
pub struct MyCacheService;

#[async_trait]
impl CacheService for MyCacheService {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        println!("Received request: {:?}", request);
        let reply = SetResponse {
            // Fill in the response fields as per your schema
        };
        Ok(Response::new(reply))
    }
}
