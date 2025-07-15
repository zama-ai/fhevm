use crate::{
    core::event::{PublicDecryptRequest, PublicDecryptResponse},
    metrics::{cache_operation, CacheOperation, CacheType},
    store::key_value_db::KVStore,
};
use alloy::primitives::U256;
use anyhow::Result;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tracing::debug;
use xxhash_rust::xxh3::Xxh3;

const PUB_DECRYPT_RESPONSE_CACHE_PREFIX: &str = "PUB-DECRYPT-RESPONSE-CACHE";
const PUB_DECRYPT_REQUEST_CACHE_PREFIX: &str = "PUB-DECRYPT-REQUEST-CACHE";

/// PublicDecryptRequest -> U256 (on-chain decryption ID)
use dashmap::DashMap;
use tokio::sync::{Mutex, Notify};

pub struct PublicDecryptRequestCacheStore {
    kv_store: Arc<dyn KVStore>,
    in_flight: DashMap<String, Arc<Notify>>,
    key_locks: DashMap<String, Arc<Mutex<()>>>,
}

impl PublicDecryptRequestCacheStore {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self {
            kv_store,
            in_flight: DashMap::new(),
            key_locks: DashMap::new(),
        }
    }

    fn make_key(request: &PublicDecryptRequest) -> String {
        let mut hasher = Xxh3::with_seed(0);
        request.hash(&mut hasher);
        let hashed = hasher.finish();
        format!("{PUB_DECRYPT_REQUEST_CACHE_PREFIX}:{hashed}")
    }

    /// Persist the decryption ID and notify all waiters.
    pub async fn persist_value(
        &self,
        request: &PublicDecryptRequest,
        on_chain_decryption_id: U256,
    ) -> Result<()> {
        let key = Self::make_key(request);
        let value = serde_json::to_string(&on_chain_decryption_id)?;
        self.kv_store.put(&key, &value).await?;
        debug!("Cache added for {key} with {on_chain_decryption_id}");

        // Notify all waiters and clean up
        if let Some((_, notify)) = self.in_flight.remove(&key) {
            notify.notify_waiters();
        }
        if let Some((_, lock)) = self.key_locks.remove(&key) {
            // Drop the lock
            drop(lock);
        }
        Ok(())
    }

    /// Get the decryption ID for a request, deduplicating concurrent requests.
    /// Returns Ok(Some(decryption_id)) if found, Ok(None) if leader (should send request), and Ok(Some(decryption_id)) after notification for followers.
    pub async fn get_value(&self, request: &PublicDecryptRequest) -> Result<Option<U256>> {
        let key = Self::make_key(request);

        // Acquire per-key lock to serialize check/insert
        let lock = self
            .key_locks
            .entry(key.clone())
            .or_insert_with(|| Arc::new(Mutex::new(())))
            .clone();
        let _guard = lock.lock().await;

        // Check persistent cache first
        if let Some(value) = self.kv_store.get(&key).await? {
            let on_chain_decryption_id = serde_json::from_str(&value)?;
            debug!("Cache hit on {key} with {on_chain_decryption_id}");
            cache_operation(CacheType::PublicDecrypt, CacheOperation::Hit);
            return Ok(Some(on_chain_decryption_id));
        }

        // If not present, set up Notify for in-flight requests
        let entry = self.in_flight.entry(key.clone());
        let is_leader;
        let notify = match entry {
            dashmap::mapref::entry::Entry::Occupied(e) => {
                is_leader = false;
                e.get().clone()
            }
            dashmap::mapref::entry::Entry::Vacant(e) => {
                is_leader = true;
                e.insert(Arc::new(Notify::new())).clone()
            }
        };

        // Release lock before waiting
        drop(_guard);

        if is_leader {
            // Leader: do not wait, caller should send request and later call persist_value
            debug!("Cache miss on {key}, leader elected, caller should send request");
            cache_operation(CacheType::PublicDecrypt, CacheOperation::Miss);
            return Ok(None);
        } else {
            // Follower: wait for notification
            notify.notified().await;

            // After notification, check persistent cache again
            if let Some(value) = self.kv_store.get(&key).await? {
                let on_chain_decryption_id = serde_json::from_str(&value)?;
                debug!("Cache hit after notify on {key} with {on_chain_decryption_id}");
                cache_operation(CacheType::PublicDecrypt, CacheOperation::Hit);
                return Ok(Some(on_chain_decryption_id));
            }
            anyhow::bail!("Cache miss after notify on {key}");
        }
    }
}

/// U256 (on-chain decryption ID) -> PublicDecryptResponse
pub struct PublicDecryptResponseCacheStore {
    kv_store: Arc<dyn KVStore>,
}

impl PublicDecryptResponseCacheStore {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self { kv_store }
    }

    fn make_key(on_chain_decryption_id: U256) -> String {
        format!("{PUB_DECRYPT_RESPONSE_CACHE_PREFIX}:{on_chain_decryption_id}")
    }

    /// Store a PublicDecryptResponse for a given decryption ID.
    pub async fn persist_value(
        &self,
        on_chain_decryption_id: U256,
        gw_response: PublicDecryptResponse,
    ) -> Result<()> {
        let key = Self::make_key(on_chain_decryption_id);
        let value = serde_json::to_string(&gw_response)?;
        self.kv_store.put(&key, &value).await?;
        debug!("Cache added for {key} with {gw_response:?}");
        Ok(())
    }

    /// Retrieve a PublicDecryptResponse for a given decryption ID.
    pub async fn get_value(
        &self,
        on_chain_decryption_id: U256,
    ) -> Result<Option<PublicDecryptResponse>> {
        let key = Self::make_key(on_chain_decryption_id);
        if let Some(value) = self.kv_store.get(&key).await? {
            let gw_response = serde_json::from_str(&value)?;
            debug!("Cache hit on {key} with {gw_response:?}");
            return Ok(Some(gw_response));
        }
        debug!("Cache miss on {key}");
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

    // Helper function to construct a dummy PublicDecryptRequest.
    fn dummy_request(handles: Vec<[u8; 32]>) -> PublicDecryptRequest {
        PublicDecryptRequest {
            ct_handles: handles,
        }
    }

    #[tokio::test]
    async fn test_pub_decrypt_request_cache_store() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = PublicDecryptRequestCacheStore::new(kv_store);
        let request = dummy_request(vec![[1u8; 32]]);
        let decryption_id = U256::from(42);

        // Initially, the value should not exist.
        let retrieved = cache_store.get_value(&request).await.unwrap();
        assert!(retrieved.is_none());

        // Store a dummy decryption ID.
        cache_store
            .persist_value(&request, decryption_id)
            .await
            .unwrap();

        // Retrieve and verify.
        let retrieved = cache_store.get_value(&request).await.unwrap();
        assert_eq!(retrieved, Some(decryption_id));
    }

    #[tokio::test]
    async fn test_pub_decrypt_response_cache_store() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = PublicDecryptResponseCacheStore::new(kv_store);
        let decryption_id = U256::from(42);

        // Initially, the value should not exist.
        let retrieved = cache_store.get_value(decryption_id).await.unwrap();
        assert!(retrieved.is_none());

        // Store a dummy response.
        let response = dummy_response(decryption_id, b"test_value");
        cache_store
            .persist_value(decryption_id, response.clone())
            .await
            .unwrap();

        // Retrieve and verify.
        let retrieved = cache_store.get_value(decryption_id).await.unwrap();
        assert_eq!(retrieved, Some(response.clone()));
    }
}
