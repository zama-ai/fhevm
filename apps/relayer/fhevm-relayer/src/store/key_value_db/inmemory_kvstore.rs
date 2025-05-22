use anyhow::Result;
use async_trait::async_trait;
use dashmap::DashMap;
use tracing::debug;

use super::kvstore::KVStore;

/// In-memory implementation of the KVStore trait.
/// This implementation is meant for testing and development purposes.
pub struct InMemoryKVStore {
    /// Main data store
    data: DashMap<String, String>,
}

impl InMemoryKVStore {
    pub fn new() -> Self {
        Self {
            data: DashMap::new(),
        }
    }
}

impl Default for InMemoryKVStore {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl KVStore for InMemoryKVStore {
    async fn put(&self, key: &str, value: &str) -> Result<()> {
        self.data.insert(key.to_string(), value.to_string());
        debug!("Stored value for key: {}", key);
        Ok(())
    }

    async fn get(&self, key: &str) -> Result<Option<String>> {
        let result = self.data.get(key).map(|v| v.value().clone());
        debug!(
            "Retrieved value for key: {}, found: {}",
            key,
            result.is_some()
        );
        Ok(result)
    }

    async fn delete(&self, key: &str) -> Result<()> {
        self.data.remove(key);
        debug!("Deleted key: {}", key);
        Ok(())
    }

    async fn get_by_prefix(&self, prefix: &str) -> Result<Vec<(String, String)>> {
        let mut pairs: Vec<(String, String)> = self
            .data
            .iter()
            .filter_map(|entry| {
                let k = entry.key();
                let v = entry.value();
                if k.starts_with(prefix) {
                    Some((k.clone(), v.clone()))
                } else {
                    None
                }
            })
            .collect();
        pairs.sort_by(|a, b| a.0.cmp(&b.0));
        debug!("Retrieved {} pairs with prefix: {}", pairs.len(), prefix);
        Ok(pairs)
    }

    async fn delete_by_prefix(&self, prefix: &str) -> Result<()> {
        let keys_to_remove: Vec<String> = self
            .data
            .iter()
            .filter_map(|entry| {
                let k = entry.key();
                if k.starts_with(prefix) {
                    Some(k.clone())
                } else {
                    None
                }
            })
            .collect();
        for key in keys_to_remove {
            self.data.remove(&key);
            debug!("Deleted key by prefix: {}", key);
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use super::*;
    use crate::store::key_value_db::kvstore_tests::suite::run_kvstore_interface_tests;

    #[tokio::test]
    async fn test_inmemory_kvstore_interface() {
        let store = Arc::new(InMemoryKVStore::default());
        run_kvstore_interface_tests(store).await.unwrap();
    }
}
