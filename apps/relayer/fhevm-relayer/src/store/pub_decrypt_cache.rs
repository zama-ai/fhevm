use crate::{
    core::event::PublicDecryptResponse,
    metrics::{cache_operation, CacheOperation, CacheType},
    store::key_value_db::KVStore,
};
use anyhow::Result;
use std::sync::Arc;
use tracing::debug;

const PUB_DECRYPT_CACHE_PREFIX: &str = "PUB-DECRYPT-CACHE";

pub struct PublicDecryptCacheStore {
    kv_store: Arc<dyn KVStore>,
}

impl PublicDecryptCacheStore {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self { kv_store }
    }

    fn make_key(handles: &[[u8; 32]]) -> String {
        let flattened_handles: Vec<u8> = handles.iter().flat_map(|arr| arr.to_vec()).collect();
        format!(
            "{}:{}",
            PUB_DECRYPT_CACHE_PREFIX,
            hex::encode(flattened_handles)
        )
    }

    /// Store a PublicDecryptResponse for a given handle.
    pub async fn persist_value(
        &self,
        handles: &[[u8; 32]],
        gw_response: PublicDecryptResponse,
    ) -> Result<()> {
        let key = Self::make_key(handles);
        let value = serde_json::to_string(&gw_response)?;
        self.kv_store.put(&key, &value).await?;
        Ok(())
    }

    /// Retrieve a PublicDecryptResponse for a given handle.
    pub async fn get_value(&self, handles: &[[u8; 32]]) -> Result<Option<PublicDecryptResponse>> {
        let key = Self::make_key(handles);
        if let Some(value) = self.kv_store.get(&key).await? {
            let gw_response = serde_json::from_str(&value)?;
            debug!("Cache hit on {key}");
            cache_operation(CacheType::PublicDecrypt, CacheOperation::Hit);
            return Ok(Some(gw_response));
        }
        debug!("Cache miss on {key}");
        cache_operation(CacheType::PublicDecrypt, CacheOperation::Miss);
        Ok(None)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::store::key_value_db::InMemoryKVStore;
    use alloy::primitives::{Bytes, U256};
    use prometheus::Registry;
    use std::sync::Arc;

    // Helper function to initialize metrics for tests
    fn init_metrics_for_test() {
        let registry = Registry::new();
        crate::metrics::init_cache_metrics(&registry);
    }

    // Helper function to construct a dummy PublicDecryptResponse.
    fn dummy_response(gateway_request_id: U256, decrypted_value: &[u8]) -> PublicDecryptResponse {
        PublicDecryptResponse {
            gateway_request_id,
            decrypted_value: Bytes::copy_from_slice(decrypted_value),
            signatures: vec![],
        }
    }

    #[tokio::test]
    async fn test_pub_decrypt_cache_store() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = PublicDecryptCacheStore::new(kv_store);
        let handles: Vec<[u8; 32]> = vec![[1u8; 32]];

        // Initially, the value should not exist.
        let retrieved = cache_store.get_value(&handles).await.unwrap();
        assert!(retrieved.is_none());

        // Store a dummy response.
        let response = dummy_response(U256::from(1u64), b"test_value");
        cache_store
            .persist_value(&handles, response.clone())
            .await
            .unwrap();

        // Retrieve and verify.
        let retrieved = cache_store.get_value(&handles).await.unwrap();
        assert_eq!(retrieved, Some(response.clone()));

        // Overwrite with a new response.
        let new_response = dummy_response(U256::from(2u64), b"new_value");
        cache_store
            .persist_value(&handles, new_response.clone())
            .await
            .unwrap();
        let retrieved = cache_store.get_value(&handles).await.unwrap();
        assert_eq!(retrieved, Some(new_response));
    }

    #[tokio::test]
    async fn test_pub_decrypt_cache_store_handles_multiple() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = PublicDecryptCacheStore::new(kv_store);
        let handles1: Vec<[u8; 32]> = vec![[1u8; 32]];
        let response1 = dummy_response(U256::from(1u64), b"value1");
        let handles2: Vec<[u8; 32]> = vec![[2u8; 32]];
        let response2 = dummy_response(U256::from(2u64), b"value2");

        cache_store
            .persist_value(&handles1, response1.clone())
            .await
            .unwrap();
        cache_store
            .persist_value(&handles2, response2.clone())
            .await
            .unwrap();

        let retrieved1 = cache_store.get_value(&handles1).await.unwrap();
        let retrieved2 = cache_store.get_value(&handles2).await.unwrap();
        assert_eq!(retrieved1, Some(response1));
        assert_eq!(retrieved2, Some(response2));
    }
}
