use crate::{
    core::{
        errors::EventProcessingError,
        event::{
            PublicDecryptRequest, PublicDecryptResponse, RequestValidity, UserDecryptRequest,
            UserDecryptResponse,
        },
        job_id::JobId,
    },
    metrics::{cache_operation, CacheOperation, CacheType},
    store::key_value_db::KVStore,
};
use alloy::primitives::{Bytes, U256};
use anyhow::Result;
use dashmap::{mapref::entry::Entry, DashMap};
use std::hash::{Hash, Hasher};
use std::marker::PhantomData;
use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, Notify};
use tracing::debug;
use xxhash_rust::xxh3::Xxh3;

#[derive(Debug, Error)]
pub enum CacheError {
    #[error("Failed to access request cache: {0}")]
    RequestCacheAccess(String),
    #[error("Failed to access response cache: {0}")]
    ResponseCacheAccess(String),
    #[error("Failed to persist to cache: {0}")]
    CachePersistence(String),
}

impl From<CacheError> for EventProcessingError {
    fn from(err: CacheError) -> Self {
        EventProcessingError::HandlerError(err.to_string())
    }
}

#[derive(Debug)]
pub enum CacheResult<T> {
    Hit(T),           // Response found in cache
    InProgress(U256), // Request already sent, waiting for response
    NotFound,         // First time seeing this request
}

/// Trait for generating cache keys from requests
pub trait CacheKeyGenerator {
    fn generate_request_key(&self, prefix: &str) -> String;
}

impl CacheKeyGenerator for PublicDecryptRequest {
    fn generate_request_key(&self, prefix: &str) -> String {
        let mut hasher = Xxh3::with_seed(0);
        self.hash(&mut hasher);
        let hashed = hasher.finish();
        format!("{prefix}:{hashed}")
    }
}

impl CacheKeyGenerator for UserDecryptRequest {
    fn generate_request_key(&self, prefix: &str) -> String {
        let mut hasher = Xxh3::with_seed(0);
        // NOTE: we can safely remove the EIP-712 signature and the request validity from the cache
        // because these are only used on-chain prior to receiving a decryption-id.
        let modified_request = UserDecryptRequest {
            ct_handle_contract_pairs: self.ct_handle_contract_pairs.clone(),
            request_validity: RequestValidity {
                start_timestamp: U256::default(),
                duration_days: U256::default(),
            },
            contracts_chain_id: self.contracts_chain_id,
            contract_addresses: self.contract_addresses.clone(),
            user_address: self.user_address,
            signature: Bytes::default(),
            public_key: self.public_key.clone(),
            extra_data: self.extra_data.clone(),
        };
        modified_request.hash(&mut hasher);
        let hashed = hasher.finish();
        format!("{prefix}:{hashed}")
    }
}

/// Generic cache implementation for decrypt operations
pub struct DecryptionCache<TRequest, TResponse>
where
    TRequest: CacheKeyGenerator + Send + Sync,
    TResponse: Send + Sync,
{
    kv_store: Arc<dyn KVStore>,
    request_in_flight: DashMap<String, Arc<Notify>>,
    request_key_locks: DashMap<String, Arc<Mutex<()>>>,
    job_id_mappings: Arc<DashMap<U256, JobId>>,
    request_prefix: &'static str,
    response_prefix: &'static str,
    cache_type: CacheType,
    _phantom_request: PhantomData<TRequest>,
    _phantom_response: PhantomData<TResponse>,
}

impl<TRequest, TResponse> DecryptionCache<TRequest, TResponse>
where
    TRequest: CacheKeyGenerator + Send + Sync,
    TResponse:
        Clone + serde::Serialize + for<'de> serde::Deserialize<'de> + Send + Sync + std::fmt::Debug,
{
    fn new_inner(
        kv_store: Arc<dyn KVStore>,
        request_prefix: &'static str,
        response_prefix: &'static str,
        cache_type: CacheType,
    ) -> Self {
        Self {
            kv_store,
            request_in_flight: DashMap::new(),
            request_key_locks: DashMap::new(),
            job_id_mappings: Arc::new(DashMap::new()),
            request_prefix,
            response_prefix,
            cache_type,
            _phantom_request: PhantomData,
            _phantom_response: PhantomData,
        }
    }

    fn make_response_key(&self, on_chain_decryption_id: U256) -> String {
        format!("{}:{}", self.response_prefix, on_chain_decryption_id)
    }

    pub async fn check(&self, request: &TRequest) -> Result<CacheResult<TResponse>, CacheError> {
        let decryption_id_opt = self
            .get_request_mapping(request)
            .await
            .map_err(|e| CacheError::RequestCacheAccess(e.to_string()))?;

        match decryption_id_opt {
            None => Ok(CacheResult::NotFound),
            Some(decryption_id) => match self.get_response(decryption_id).await {
                Ok(Some(response)) => Ok(CacheResult::Hit(response)),
                Ok(None) => Ok(CacheResult::InProgress(decryption_id)),
                Err(e) => Err(CacheError::ResponseCacheAccess(e.to_string())),
            },
        }
    }

    /// Get the decryption ID for a request, deduplicating concurrent requests.
    async fn get_request_mapping(&self, request: &TRequest) -> Result<Option<U256>> {
        let key = request.generate_request_key(self.request_prefix);

        loop {
            // Acquire per-key lock to serialize check/insert
            let lock = self
                .request_key_locks
                .entry(key.clone())
                .or_insert_with(|| Arc::new(Mutex::new(())))
                .clone();
            let _guard = lock.lock().await;

            // Check persistent cache first
            if let Some(value) = self.kv_store.get(&key).await? {
                let on_chain_decryption_id = serde_json::from_str(&value)?;
                debug!("Cache hit on {key} with {on_chain_decryption_id}");
                cache_operation(self.cache_type, CacheOperation::Hit);
                return Ok(Some(on_chain_decryption_id));
            }

            // If not present, set up Notify for in-flight requests
            let entry = self.request_in_flight.entry(key.clone());
            let is_leader;
            let notify = match entry {
                Entry::Occupied(e) => {
                    is_leader = false;
                    e.get().clone()
                }
                Entry::Vacant(e) => {
                    is_leader = true;
                    e.insert(Arc::new(Notify::new())).clone()
                }
            };

            // Release lock before waiting
            drop(_guard);

            if is_leader {
                // Leader: do not wait, caller should send request and later call store_request_mapping
                debug!("Cache miss on {key}, leader elected, caller should send request");
                cache_operation(self.cache_type, CacheOperation::Miss);
                return Ok(None);
            } else {
                // Follower: wait for notification
                debug!("Waiting for leader to notify.");
                notify.notified().await;
                continue;
            }
        }
    }

    /// Retrieve a response for a given decryption ID.
    async fn get_response(&self, on_chain_decryption_id: U256) -> Result<Option<TResponse>> {
        let key = self.make_response_key(on_chain_decryption_id);
        if let Some(value) = self.kv_store.get(&key).await? {
            let response = serde_json::from_str(&value)?;
            debug!("Cache hit on {key} with {response:?}");
            return Ok(Some(response));
        }
        debug!("Cache miss on {key}");
        Ok(None)
    }

    pub async fn store_request_mapping(
        &self,
        request: &TRequest,
        decryption_id: U256,
        job_id: JobId,
    ) -> Result<(), CacheError> {
        let key = request.generate_request_key(self.request_prefix);
        let value = serde_json::to_string(&decryption_id)
            .map_err(|e| CacheError::CachePersistence(e.to_string()))?;
        self.kv_store
            .put(&key, &value)
            .await
            .map_err(|e| CacheError::CachePersistence(e.to_string()))?;
        
        // Store the JobId mapping for response dispatching
        self.job_id_mappings.insert(decryption_id, job_id);
        
        debug!("Cache added for {key} with {decryption_id} and job_id={}", job_id);

        // Notify all waiters and clean up
        if let Some((_, notify)) = self.request_in_flight.remove(&key) {
            notify.notify_waiters();
        }
        if let Some((_, lock)) = self.request_key_locks.remove(&key) {
            // Drop the lock
            drop(lock);
        }
        Ok(())
    }

    pub async fn store_response(
        &self,
        decryption_id: U256,
        response: TResponse,
    ) -> Result<(), CacheError> {
        let key = self.make_response_key(decryption_id);
        let value = serde_json::to_string(&response)
            .map_err(|e| CacheError::CachePersistence(e.to_string()))?;
        self.kv_store
            .put(&key, &value)
            .await
            .map_err(|e| CacheError::CachePersistence(e.to_string()))?;
        debug!("Cache added for {key} with {response:?}");
        Ok(())
    }

    pub async fn unlock_request(&self, request: &TRequest) -> Result<(), CacheError> {
        let key = request.generate_request_key(self.request_prefix);
        if let Some((_, notify)) = self.request_in_flight.remove(&key) {
            notify.notify_waiters();
        }
        Ok(())
    }


    pub fn get_job_id_for_decryption_id(&self, decryption_id: U256) -> Option<JobId> {
        self.job_id_mappings
            .get(&decryption_id)
            .map(|entry| *entry.value())
    }

    pub fn cleanup_mapping(&self, decryption_id: &U256) {
        self.job_id_mappings.remove(decryption_id);
    }
}

impl<TRequest, TResponse> Clone for DecryptionCache<TRequest, TResponse>
where
    TRequest: CacheKeyGenerator + Send + Sync,
    TResponse: Send + Sync,
{
    fn clone(&self) -> Self {
        Self {
            kv_store: self.kv_store.clone(),
            request_in_flight: DashMap::new(),
            request_key_locks: DashMap::new(),
            job_id_mappings: self.job_id_mappings.clone(),
            request_prefix: self.request_prefix,
            response_prefix: self.response_prefix,
            cache_type: self.cache_type,
            _phantom_request: PhantomData,
            _phantom_response: PhantomData,
        }
    }
}

// Type aliases and constructor functions
pub type PublicDecryptCache = DecryptionCache<PublicDecryptRequest, PublicDecryptResponse>;
pub type UserDecryptCache = DecryptionCache<UserDecryptRequest, UserDecryptResponse>;

impl PublicDecryptCache {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self::new_inner(
            kv_store,
            "PUB-DECRYPT-REQUEST-CACHE",
            "PUB-DECRYPT-RESPONSE-CACHE",
            CacheType::PublicDecrypt,
        )
    }
}

impl UserDecryptCache {
    pub fn new(kv_store: Arc<dyn KVStore>) -> Self {
        Self::new_inner(
            kv_store,
            "USER-DECRYPT-REQUEST-CACHE",
            "USER-DECRYPT-RESPONSE-CACHE",
            CacheType::UserDecryptRequest,
        )
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::RequestValidity;
    use crate::core::job_id::JobId;
    use crate::store::key_value_db::InMemoryKVStore;
    use alloy::primitives::{Address, Bytes, U256};
    use prometheus::Registry;
    use std::sync::Arc;

    // Helper function to initialize metrics for tests
    fn init_metrics_for_test() {
        let registry = Registry::new();
        crate::metrics::init_cache_metrics(&registry);
    }

    // Helper function to construct a dummy PublicDecryptResponse.
    fn dummy_public_response(
        gateway_request_id: U256,
        decrypted_value: &[u8],
    ) -> PublicDecryptResponse {
        PublicDecryptResponse {
            gateway_request_id,
            decrypted_value: Bytes::copy_from_slice(decrypted_value),
            signatures: vec![],
            extra_data: Bytes::default(),
        }
    }

    // Helper function to construct a dummy PublicDecryptRequest.
    fn dummy_public_request(handles: Vec<[u8; 32]>) -> PublicDecryptRequest {
        PublicDecryptRequest {
            ct_handles: handles,
            extra_data: Bytes::default(),
        }
    }

    fn dummy_user_response(gateway_request_id: U256) -> UserDecryptResponse {
        UserDecryptResponse {
            gateway_request_id,
            reencrypted_shares: vec![],
            signatures: vec![],
            extra_data: Bytes::default(),
        }
    }

    fn dummy_user_request(contract_chain_id: u64) -> UserDecryptRequest {
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
    async fn test_public_decrypt_cache() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache = PublicDecryptCache::new(kv_store);
        let request = dummy_public_request(vec![[1u8; 32]]);
        let decryption_id = U256::from(42);

        // Initially, should be NotFound
        let result = cache.check(&request).await.unwrap();
        assert!(matches!(result, CacheResult::NotFound));

        // Store request mapping
        let job_id = JobId::from_uuid_v7(uuid::Uuid::now_v7());
        cache
            .store_request_mapping(&request, decryption_id, job_id)
            .await
            .unwrap();

        // Should now be InProgress
        let result = cache.check(&request).await.unwrap();
        assert!(matches!(result, CacheResult::InProgress(_)));

        // Store response
        let response = dummy_public_response(decryption_id, b"test_value");
        cache
            .store_response(decryption_id, response.clone())
            .await
            .unwrap();

        // Should now be Hit
        let result = cache.check(&request).await.unwrap();
        if let CacheResult::Hit(cached_response) = result {
            assert_eq!(
                cached_response.gateway_request_id,
                response.gateway_request_id
            );
            assert_eq!(cached_response.decrypted_value, response.decrypted_value);
        } else {
            panic!("Expected CacheResult::Hit");
        }
    }

    #[tokio::test]
    async fn test_user_decrypt_cache() {
        init_metrics_for_test();
        let kv_store = Arc::new(InMemoryKVStore::default());
        let cache = UserDecryptCache::new(kv_store);
        let request = dummy_user_request(0);
        let decryption_id = U256::from(42);

        // Initially, should be NotFound
        let result = cache.check(&request).await.unwrap();
        assert!(matches!(result, CacheResult::NotFound));

        // Store request mapping
        let job_id = JobId::from_uuid_v7(uuid::Uuid::now_v7());
        cache
            .store_request_mapping(&request, decryption_id, job_id)
            .await
            .unwrap();

        // Should now be InProgress
        let result = cache.check(&request).await.unwrap();
        assert!(matches!(result, CacheResult::InProgress(_)));

        // Store response
        let response = dummy_user_response(decryption_id);
        cache
            .store_response(decryption_id, response.clone())
            .await
            .unwrap();

        // Should now be Hit
        let result = cache.check(&request).await.unwrap();
        if let CacheResult::Hit(cached_response) = result {
            assert_eq!(
                cached_response.gateway_request_id,
                response.gateway_request_id
            );
        } else {
            panic!("Expected CacheResult::Hit");
        }
    }
}
