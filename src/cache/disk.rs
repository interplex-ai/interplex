use crate::cache::{Cacheable, CachedObject};
use anyhow::Result;
use log::{info, warn};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::error::Error;
use std::fs::{read_to_string, write};
use std::path::PathBuf;
use std::sync::Arc;
use tokio::sync::RwLock;
use tonic::async_trait;

#[derive(Clone, Serialize, Deserialize)]
pub struct DiskCachedObject {
    value: String,
}

pub struct DiskCache {
    store: Arc<RwLock<HashMap<String, DiskCachedObject>>>,
    cache_dir: PathBuf,
}

pub fn new_disk_cache(cache_dir: &str) -> DiskCache {
    std::fs::create_dir_all(cache_dir).unwrap();
    DiskCache {
        store: Arc::new(RwLock::new(HashMap::new())),
        cache_dir: PathBuf::from(cache_dir),
    }
}

impl DiskCache {
    fn get_file_path(&self, key: &str) -> PathBuf {
        self.cache_dir.join(key)
    }
}

#[async_trait]
impl Cacheable for DiskCache {
    async fn get(&self, key: &str) -> Result<CachedObject, Box<dyn Error>> {
        // Check in-memory cache first
        {
            let store = self.store.read().await;
            if let Some(value) = store.get(key) {
                info!("Cache hit for key: {key}");
                return Ok(CachedObject {
                    value: value.clone().value,
                });
            }
        }

        // Check disk cache
        let file_path = self.get_file_path(key);
        if file_path.exists() {
            info!("Cache hit on disk for key: {key}");
            let contents = read_to_string(&file_path)?;
            let cached_object: DiskCachedObject = serde_json::from_str(&contents)?;
            self.store
                .write()
                .await
                .insert(key.to_string(), cached_object.clone());
            return Ok(CachedObject {
                value: cached_object.value,
            });
        }

        warn!("Cache miss for key: {}", key);
        Err("Value not found".into())
    }

    async fn set(&self, key: &str, value: String) -> Result<(), Box<dyn Error>> {
        if key.is_empty() {
            return Err("Key not found".into());
        }
        let cached_object = DiskCachedObject {
            value: value.clone(),
        };

        // Update in-memory cache
        self.store
            .write()
            .await
            .insert(key.to_string(), cached_object.clone());

        // Update disk cache
        let file_path = self.get_file_path(key);
        let contents = serde_json::to_string(&cached_object)?;
        write(&file_path, contents)?;

        info!(
            "Cached value for key: {} at {}",
            key,
            file_path.as_path().to_str().unwrap()
        );
        Ok(())
    }

    async fn remove(&self, key: &str) -> std::result::Result<(), Box<dyn Error>> {
        let file_path = self.get_file_path(key);
        let contents = read_to_string(&file_path)?;
        self.store.write().await.remove(key);
        info!("Deleted key: {}", key);
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
        // pugre the cache directory
        let _ = std::fs::remove_dir_all(&self.cache_dir);
        info!("Purged cache");
        Ok(())
    }
}
#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::NamedTempFile;

    use std::path::PathBuf;

    fn get_cache_dir() -> PathBuf {
        NamedTempFile::new().unwrap().path().to_path_buf()
    }

    #[tokio::test]
    async fn test_set_and_get_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Get the value
        let get_result = cache.get("key1").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());
    }

    #[tokio::test]
    async fn test_get_non_existent_key_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Try to get a value for a non-existent key
        let get_result = cache.get("non_existent_key").await;
        assert!(get_result.is_err());
    }

    #[tokio::test]
    async fn test_update_value_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

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
    async fn test_tokenized_key_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Set a value with a tokenizable key
        let set_result = cache
            .set("token key with spaces", "value1".to_string())
            .await;
        assert!(set_result.is_ok());

        // Get the value using the tokenized key
        let get_result = cache.get("token key with spaces").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());
    }

    #[tokio::test]
    async fn test_empty_key_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Set a value with an empty key
        let set_result = cache.set("", "value1".to_string()).await;
        assert!(set_result.is_err());
    }
    #[tokio::test]
    async fn test_delete_key_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Get the value
        let get_result = cache.get("key1").await;
        assert!(get_result.is_ok());
        assert_eq!(get_result.unwrap().value, "value1".to_string());

        // Delete the value
        let _ = cache.remove("key1").await;
        // Check that the cache is empty
        let store = cache.store.read().await;
        assert_eq!(store.len(), 0);
    }

    #[tokio::test]
    async fn test_list_keys_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

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

    #[tokio::test]
    async fn test_purge_disk() {
        let cache_dir = get_cache_dir();
        let cache = new_disk_cache(cache_dir.to_str().unwrap());

        // Set a value
        let set_result = cache.set("key1", "value1".to_string()).await;
        assert!(set_result.is_ok());

        // Set another value
        let set_result = cache.set("key2", "value2".to_string()).await;
        assert!(set_result.is_ok());

        // Purge the cache
        let _ = cache.purge().await;
        let store = cache.store.read().await;
        assert_eq!(store.len(), 0);
    }
}
