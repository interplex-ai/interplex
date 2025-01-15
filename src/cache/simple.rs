use crate::cache::{Cacheable, CachedObject};
use std::collections::HashMap;
use std::error::Error;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::async_trait;

pub struct SimpleCache {
    store: Arc<RwLock<HashMap<String, CachedObject>>>,
}

pub fn new_simple_cache() -> SimpleCache {
    SimpleCache {
        store: Arc::new(RwLock::new(HashMap::new())),
    }
}

#[async_trait]
impl Cacheable for SimpleCache {
    async fn get(&self, key: &str) -> Result<CachedObject, Box<dyn Error>> {
        let store = self.store.read().await;
        if let Some(value) = store.get(key) {
            // Directly clone the string value
            return Ok(value.clone());
        }
        Err("Value not found".into())
    }

    async fn set(&self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        let mut store = self.store.write().await;
        let tokens: Vec<&str> = key.split_whitespace().collect();
        let tokenized_key = tokens.join("_"); // Join tokens with an underscore

        store.insert(tokenized_key, CachedObject { value });
        Ok(())
    }

    async fn remove(&self, key: &str) -> Result<(), Box<dyn Error>> {
        let mut store = self.store.write().await;
        store.remove(key);
        Ok(())
    }

    async fn list_keys(&self) -> Result<Vec<String>, Box<dyn Error>> {
        let store = self.store.read().await;
        let keys: Vec<String> = store.keys().cloned().collect();
        Ok(keys)
    }

    async fn purge(&self) -> Result<(), Box<dyn Error>> {
        let mut store = self.store.write().await;
        store.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn test_set_and_get() {
        let cache = new_simple_cache();

        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Get the value
        let get_result = cache.get("key1").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());
    }

    #[tokio::test]
    async fn test_get_non_existent_key() {
        let cache = new_simple_cache();

        // Try to get a value for a non-existent key
        let get_result = cache.get("non_existent_key").await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_update_value() {
        let cache = new_simple_cache();

        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Update the value
        let set_result = cache.set("key1", "value2".to_string()).await;
        assert!(set_result.is_ok());

        // Get the updated value
        let get_result = cache.get("key1").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value2".to_string());
    }

    #[tokio::test]
    async fn test_tokenized_key() {
        let cache = new_simple_cache();

        // Set a value with a tokenizable key
        let set_result = cache
            .set("token key with spaces", "value1".to_string())
            .await;
        assert!(set_result.is_ok());

        // Get the value using the tokenized key
        let get_result = cache.get("token_key_with_spaces").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());
    }

    #[tokio::test]
    async fn test_empty_key() {
        let cache = new_simple_cache();

        // Set a value with an empty key
        let set_result = cache.set("", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Get the value using the empty key
        let get_result = cache.get("").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());
    }

    #[tokio::test]
    async fn test_list_keys_disk() {

        let cache = new_simple_cache();
        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Set another value
        let set_result = cache.set("key2", "value2".to_string()).await;
        assert!(set_result.is_ok());

        // List keys
        let keys = cache.list_keys().await.unwrap();
        assert_eq!(keys.len(), 2);
        assert!(keys.contains(&"key1".to_string()));
        assert!(keys.contains(&"key2".to_string()));
    }

}
