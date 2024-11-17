use crate::cache::simple::{new_simple_cache, SimpleCache};
use crate::cache::disk::{new_disk_cache};
use crate::cache::Cacheable;
use interplex_ai_schemas_community_neoeinstein_prost::schema::v1::{DeleteRequest, DeleteResponse, GetRequest, GetResponse, SetRequest, SetResponse};
use interplex_ai_schemas_community_neoeinstein_tonic::schema::v1::tonic::cache_service_server::CacheService;
use tonic::{async_trait, Request, Response, Status};

pub struct MyCacheService {
    cache: Box<dyn Cacheable + Send + Sync>,
    memory_only_cache: bool
}

impl MyCacheService {
    pub(crate) fn new(memory_only_cache: bool, cache_path: &str) -> Self {
        
        if memory_only_cache {
            return MyCacheService {
                cache: Box::new(new_simple_cache()),
                memory_only_cache: memory_only_cache
            }
        }
        MyCacheService {
            cache: Box::new(new_disk_cache(cache_path)),
            memory_only_cache: memory_only_cache
        }
    }
}

#[async_trait]
impl CacheService for MyCacheService {
    async fn set(&self, request: Request<SetRequest>) -> Result<Response<SetResponse>, Status> {
        println!("Received request: {:?}", request);

        let req = request.into_inner();
        let key = req.key;
        let value = req.value;

        self.cache
            .set(&key, value)
            .await
            .map_err(|e| Status::internal(format!("Cache set error: {}", e)))?;

        let reply = SetResponse {};
        Ok(Response::new(reply))
    }

    async fn get(&self, request: Request<GetRequest>) -> Result<Response<GetResponse>, Status> {
        let key = request.into_inner().key;

        let v = self
            .cache
            .get(&key)
            .await
            .map_err(|e| Status::internal(format!("Cache get error: {}", e)))?;

        let value= v.value;
        let reply = GetResponse { value };
        Ok(Response::new(reply))
    }

    async fn delete(&self, request: Request<DeleteRequest>) -> Result<Response<DeleteResponse>, Status> {
        let key = request.into_inner().key;
        self.cache.remove(&key).await  .map_err(|e| Status::internal(format!("Cache remove error: {}", e)))?;
        let reply = DeleteResponse {};
        Ok(Response::new(reply))
    }
}
