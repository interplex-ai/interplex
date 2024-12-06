pub mod disk;
pub mod simple;

use std::error::Error;
use tonic::async_trait;

#[async_trait]
pub trait Cacheable {
    async fn get(&self, key: &str) -> Result<CachedObject, Box<dyn Error>>;
    async fn set(&self, key: &str, value: String) -> Result<(), Box<dyn Error>>;
    async fn remove(&self, key: &str) -> Result<(), Box<dyn Error>>;
}

#[derive(Clone)]
pub struct CachedObject {
    pub(crate) value: String,
}
