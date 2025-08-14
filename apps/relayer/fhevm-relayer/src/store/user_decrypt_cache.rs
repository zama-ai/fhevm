use crate::{
    core::event::{RequestValidity, UserDecryptRequest, UserDecryptResponse},
    metrics::{cache_operation, CacheOperation, CacheType},
    store::key_value_db::KVStore,
};
use alloy::primitives::{Bytes, U256};
use anyhow::Result;
use dashmap::mapref::entry::Entry;
use dashmap::DashMap;
use std::hash::{Hash, Hasher};
use std::sync::Arc;
use tokio::sync::{Mutex, Notify};
use tracing::debug;
use xxhash_rust::xxh3::Xxh3;

const USER_DECRYPT_RESPONSE_CACHE_PREFIX: &str = "USER-DECRYPT-RESPONSE-CACHE";
const USER_DECRYPT_REQUEST_CACHE_PREFIX: &str = "USER-DECRYPT-REQUEST-CACHE";

// TODO: figure out if we shouldn't also store the raw request itself to compare
// hash to make sure no collision happened
// TODO: reconsider the hash function we use

/// RelayerRequestId -> UserDecryptResponse
pub struct UserDecryptResponseCacheStore {
    kv_store: Arc<dyn KVStore>,
}

impl UserDecryptResponseCacheStore {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self { kv_store }
    }

    fn make_key(on_chain_decryption_id: U256) -> String {
        // Not sure if creating a new hasher each time is pertinent here
        format!("{USER_DECRYPT_RESPONSE_CACHE_PREFIX}:{on_chain_decryption_id}")
    }

    /// Store a PublicDecryptResponse for a given handle.
    pub async fn persist_value(
        &self,
        on_chain_decryption_id: U256,
        response: UserDecryptResponse,
    ) -> Result<()> {
        let key = Self::make_key(on_chain_decryption_id);
        let value = serde_json::to_string(&response)?;
        self.kv_store.put(&key, &value).await?;
        Ok(())
    }

    /// Retrieve a PublicDecryptResponse for a given handle.
    pub async fn get_value(
        &self,
        on_chain_decryption_id: U256,
    ) -> Result<Option<UserDecryptResponse>> {
        let key = Self::make_key(on_chain_decryption_id);
        if let Some(value) = self.kv_store.get(&key).await? {
            let response = serde_json::from_str(&value)?;
            debug!("Cache hit on {key} with {response}");
            cache_operation(CacheType::UserDecryptResponse, CacheOperation::Hit);
            return Ok(Some(response));
        }
        debug!("Cache miss on {key}");
        cache_operation(CacheType::UserDecryptResponse, CacheOperation::Miss);
        Ok(None)
    }
}

/// UserDecryptRequest -> RelayerRequestId
pub struct UserDecryptRequestCacheStore {
    kv_store: Arc<dyn KVStore>,
    in_flight: DashMap<String, Arc<Notify>>,
    key_locks: DashMap<String, Arc<Mutex<()>>>,
}

// TODO: at the moment we cache full requests, so sets of ciphertexts.
// If the same list of ciphertexts is requested but in different requests, or one ciphertext is
// missing from the following request, we would cache-miss.
// We need to check if from the KMS response it would be possible to cache the requests and results
// separately using hash(single-cipher, pub-key) as key to be more flexible, and request only needed
// ciphertexts.

impl UserDecryptRequestCacheStore {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self {
            kv_store,
            in_flight: DashMap::new(),
            key_locks: DashMap::new(),
        }
    }

    pub fn make_key(request: &UserDecryptRequest) -> String {
        let mut hasher = Xxh3::with_seed(0);
        // NOTE: we can safely remove the EIP-712 signature and the request validity from the cache
        // because these are only used on-chain prior to receiving a decryption-id.
        // So if these are invalid we would never get a decryption-id and thus not store a value in
        // cache.
        // If the value is valid then we get a decryption-id thus waiting for the KMS to fulfill
        // the request.
        // But any request with a same set of ciphertexts and public-key would have the same
        // result.
        let modified_request = UserDecryptRequest {
            ct_handle_contract_pairs: request.ct_handle_contract_pairs.clone(),
            request_validity: RequestValidity {
                start_timestamp: U256::default(),
                duration_days: U256::default(),
            },
            contracts_chain_id: request.contracts_chain_id,
            contract_addresses: request.contract_addresses.clone(),
            user_address: request.user_address,
            signature: Bytes::default(),
            public_key: request.public_key.clone(),
            extra_data: request.extra_data.clone(),
        };
        modified_request.hash(&mut hasher);
        let hashed = hasher.finish();
        format!("{USER_DECRYPT_REQUEST_CACHE_PREFIX}:{hashed}")
    }

    pub async fn unlock(&self, request: &UserDecryptRequest) {
        let key = Self::make_key(request);
        if let Some((_, notify)) = self.in_flight.remove(&key) {
            notify.notify_waiters();
        }
    }

    pub async fn persist_value(
        &self,
        request: &UserDecryptRequest,
        on_chain_decryption_id: U256,
    ) -> Result<()> {
        let key = Self::make_key(request);
        let value = serde_json::to_string(&on_chain_decryption_id)?;
        self.kv_store.put(&key, &value).await?;

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

    pub async fn get_value(&self, request: &UserDecryptRequest) -> Result<Option<U256>> {
        let key = Self::make_key(request);

        // Retry loop with leader election for cache coordination
        loop {
            // Acquire per-key lock to serialize check/insert
            let lock = self
                .key_locks
                .entry(key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone();
            let _guard = lock.lock().await;

            // Check persistent cache
            if let Some(value) = self.kv_store.get(&key).await? {
                let response = serde_json::from_str(&value)?;
                debug!("Cache hit on {key} with {response}");
                cache_operation(CacheType::UserDecryptRequest, CacheOperation::Hit);
                return Ok(Some(response));
            }

            // If not present, set up Notify for in-flight requests
            let entry = self.in_flight.entry(key.clone());
            let is_leader: bool;
            let notify = match entry {
                Entry::Occupied(e) => {
                    debug!("Cache request is not leader.");
                    is_leader = false;
                    e.get().clone()
                }
                Entry::Vacant(e) => {
                    debug!("Cache request is leader.");
                    is_leader = true;
                    e.insert(Arc::new(Notify::new())).clone()
                }
            };

            // Release lock before waiting
            drop(_guard);

            if is_leader {
                // Leader: do not wait, caller should send request and later call persist_value
                debug!("Cache miss on {key}, leader elected, caller should send request");
                cache_operation(CacheType::UserDecryptRequest, CacheOperation::Miss);
                return Ok(None);
            } else {
                // Follower: wait for notification
                debug!("Waiting for leader to notify.");
                notify.notified().await;
                continue;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::RequestValidity;
    use crate::store::key_value_db::InMemoryKVStore;
    use alloy::primitives::{Address, Bytes, U256};
    use prometheus::Registry;
    use std::sync::Arc;

    // Helper function to initialize metrics for tests
    fn init_metrics_for_test() {
        let registry = Registry::new();
        crate::metrics::init_cache_metrics(&registry);
    }

    fn dummy_response(gateway_request_id: U256) -> UserDecryptResponse {
        UserDecryptResponse {
            gateway_request_id,
            reencrypted_shares: vec![],
            signatures: vec![],
            extra_data: Bytes::default(),
        }
    }

    fn dummy_request(contract_chain_id: u64) -> UserDecryptRequest {
        UserDecryptRequest {
            ct_handle_contract_pairs: vec![],
            request_validity: RequestValidity {
                start_timestamp: U256::default(),
                duration_days: U256::default(),
            },
            contracts_chain_id: contract_chain_id,
            contract_addresses: vec![],
            user_address: Address::default(),
            signature: Bytes::default(),
            public_key: Bytes::default(),
            extra_data: Bytes::default(),
        }
    }

    #[tokio::test]
    async fn test_user_decrypt_request_cache_store() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = UserDecryptRequestCacheStore::new(kv_store);
        let request = dummy_request(0);

        // Initially, the value should not exist.
        let retrieved = cache_store.get_value(&request).await.unwrap();
        assert!(retrieved.is_none());

        let on_chain_decryption_id = U256::default();
        cache_store
            .persist_value(&request, on_chain_decryption_id)
            .await
            .unwrap();

        // Retrieve and verify.
        let retrieved = cache_store.get_value(&request).await.unwrap();
        assert_eq!(retrieved, Some(on_chain_decryption_id));

        // Overwrite with a new response.
        let on_chain_decryption_id = U256::from(0);
        cache_store
            .persist_value(&request, on_chain_decryption_id)
            .await
            .unwrap();
        let retrieved = cache_store.get_value(&request).await.unwrap();
        assert_eq!(retrieved, Some(on_chain_decryption_id));
    }

    #[tokio::test]
    async fn test_user_decrypt_request_cache_store_handles_multiple() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = UserDecryptRequestCacheStore::new(kv_store);
        let on_chain_decryption_id_1 = U256::from(7);
        let request_1 = dummy_request(0);
        let on_chain_decryption_id_2 = U256::from(5);
        let request_2 = dummy_request(3);

        cache_store
            .persist_value(&request_1, on_chain_decryption_id_1)
            .await
            .unwrap();
        cache_store
            .persist_value(&request_2, on_chain_decryption_id_2)
            .await
            .unwrap();

        let retrieved1 = cache_store.get_value(&request_1).await.unwrap();
        let retrieved2 = cache_store.get_value(&request_2).await.unwrap();
        assert_eq!(retrieved1, Some(on_chain_decryption_id_1));
        assert_eq!(retrieved2, Some(on_chain_decryption_id_2));
    }

    #[tokio::test]
    async fn test_user_decrypt_response_cache_store() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = UserDecryptResponseCacheStore::new(kv_store);
        let on_chain_decryption_id = U256::from(3);
        let response = dummy_response(U256::from(0));

        // Initially, the value should not exist.
        let retrieved = cache_store.get_value(on_chain_decryption_id).await.unwrap();
        assert!(retrieved.is_none());

        let on_chain_decryption_id = U256::default();
        cache_store
            .persist_value(on_chain_decryption_id, response.clone())
            .await
            .unwrap();

        // Retrieve and verify.
        let retrieved = cache_store.get_value(on_chain_decryption_id).await.unwrap();
        assert_eq!(retrieved, Some(response));

        // Overwrite with a new response.
        let response = dummy_response(U256::from(4));
        cache_store
            .persist_value(on_chain_decryption_id, response.clone())
            .await
            .unwrap();
        let retrieved = cache_store.get_value(on_chain_decryption_id).await.unwrap();
        assert_eq!(retrieved, Some(response));
    }

    #[tokio::test]
    async fn test_user_decrypt_response_cache_store_handles_multiple() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache_store = UserDecryptResponseCacheStore::new(kv_store);
        let on_chain_decryption_id_1 = U256::from(2);
        let response_1 = dummy_response(U256::from(0));
        let on_chain_decryption_id_2 = U256::from(3);
        let response_2 = dummy_response(U256::from(3));

        cache_store
            .persist_value(on_chain_decryption_id_1, response_1.clone())
            .await
            .unwrap();
        cache_store
            .persist_value(on_chain_decryption_id_2, response_2.clone())
            .await
            .unwrap();

        let retrieved1 = cache_store
            .get_value(on_chain_decryption_id_1)
            .await
            .unwrap();
        let retrieved2 = cache_store
            .get_value(on_chain_decryption_id_2)
            .await
            .unwrap();
        assert_eq!(retrieved1, Some(response_1));
        assert_eq!(retrieved2, Some(response_2));
    }
}
