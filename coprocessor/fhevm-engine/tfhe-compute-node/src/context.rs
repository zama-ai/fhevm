use fhevm_engine_common::msg_broker::create_send_channel;
use fhevm_engine_common::tenant_keys::{
    fetch_tenant_server_key, FetchTenantKeyResult, TfheTenantKeys,
};

use fhevm_engine_common::types::{Handle, SupportedFheCiphertexts};
use futures_util::stream::StreamExt;
use redis::{AsyncCommands, Client};
use std::num::NonZeroUsize;
use std::sync::Arc;
use std::time::Instant;

use tokio::sync::RwLock;
use tracing::{error, info};

use crate::{
    CiphertextInfo, ComputeError, RedisCiphertextRecord, CACHE_HITS_COUNTER,
    REDIS_BATCH_STORE_OVERHEAD, REDIS_HITS_COUNTER, REDIS_SUB_COUNTER, REDIS_SUB_OVERHEAD,
};

#[derive(Clone)]
pub(crate) struct Context {
    redis: redis::Client,
    multiplexed_conn: redis::aio::MultiplexedConnection,

    pool: sqlx::PgPool,

    // RabbitMQ channel for sending messages about completed partitions back to the dispatcher
    complete_partition_channel: lapin::Channel,

    // caches
    key_cache: Arc<RwLock<lru::LruCache<i64, TfheTenantKeys>>>,
    ciphertext_cache: Arc<RwLock<lru::LruCache<Handle, CiphertextInfo>>>,

    // Observability
    otel_ctx: opentelemetry::Context,

    current_key_id: Arc<RwLock<Option<i64>>>,
}

impl Context {
    pub(crate) async fn create(
        db_url: &str,
        redis_url: &str,
        rmq_uri: &str,
        tenant_key_cache_size: i32,
        ciphertext_cache_size: i32,
    ) -> Result<Self, ComputeError> {
        let otel_ctx = opentelemetry::Context::current();
        let key_cache: Arc<RwLock<lru::LruCache<i64, TfheTenantKeys>>> =
            Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
                NonZeroUsize::new(tenant_key_cache_size as usize).unwrap(),
            )));
        let ciphertext_cache: Arc<RwLock<lru::LruCache<Handle, CiphertextInfo>>> =
            Arc::new(RwLock::new(lru::LruCache::new(
                NonZeroUsize::new(ciphertext_cache_size as usize).unwrap(),
            )));

        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect(db_url)
            .await
            .expect("valid db url");

        let redis = Client::open(redis_url)?;
        let multiplexed_conn = redis.get_multiplexed_async_connection().await?;

        let complete_partition_channel = create_send_channel(&rmq_uri, "shared_tfhe_queue").await?;

        Ok(Self {
            redis,
            multiplexed_conn,
            pool,
            key_cache,
            ciphertext_cache,
            otel_ctx,
            current_key_id: Arc::new(RwLock::new(None)),
            complete_partition_channel,
        })
    }

    pub(crate) fn get_otel_ctx(&self) -> opentelemetry::Context {
        self.otel_ctx.clone()
    }

    pub(crate) async fn get_current_key(&self) -> Result<FetchTenantKeyResult, ComputeError> {
        let key_id = {
            let guard = self.current_key_id.read().await;
            *guard
        };

        match key_id {
            Some(id) => self
                .get_keys_cache(id)
                .await
                .map_err(|e| ComputeError::Other(e.to_string())),
            None => Err(ComputeError::MissingKeyId),
        }
    }

    pub(crate) async fn set_key_id(&self, key_id: i64) {
        let mut guard = self.current_key_id.write().await;
        *guard = Some(key_id);
    }

    pub(crate) async fn get_keys_cache(
        &self,
        key_id: i64,
    ) -> Result<FetchTenantKeyResult, ComputeError> {
        let keys = fetch_tenant_server_key(key_id, &self.pool, &self.key_cache, true)
            .await
            .map_err(|e| ComputeError::Other(e.to_string()))?;
        Ok(keys)
    }

    pub(crate) async fn cache_store(&self, ct: &CiphertextInfo) {
        let mut cache = self.ciphertext_cache.write().await;
        if cache.contains(&ct.handle) {
            return;
        }
        cache.put(ct.handle.clone(), ct.clone());
    }

    pub(crate) async fn cache_get(&self, handle: &Handle) -> Option<CiphertextInfo> {
        let cache = self.ciphertext_cache.read().await;
        cache.peek(handle).cloned()
    }

    /// Store the given ciphertext in Redis with the handle as the key.
    #[allow(dead_code)]
    pub(crate) async fn redis_store(&self, ct: CiphertextInfo) -> Result<(), ComputeError> {
        let key = hex::encode(&ct.handle);
        let entry = RedisCiphertextRecord {
            ct_type: ct.ciphertext.type_num(),
            raw_ct: Some(ct.ciphertext.serialize().1),
            compressed_ct: None,
        };

        let bytes = postcard::to_allocvec(&entry)?;
        let mut conn = self.multiplexed_conn.clone();
        conn.set::<_, _, ()>(key, bytes).await?;
        Ok(())
    }

    /// Batch store ciphertexts in Redis using pipelining for better performance.
    ///
    /// For backwards compatibility, it can store in PostgreSQL as well
    pub(crate) async fn batch_store_ciphertexts(
        &self,
        cts: Vec<CiphertextInfo>,
    ) -> Result<(), ComputeError> {
        self.redis_batch_store(cts).await
        // TODO: Spawn a task to insert new ciphertexts into PostgreSQL for backwards compatibility.
    }

    async fn redis_batch_store(&self, cts: Vec<CiphertextInfo>) -> Result<(), ComputeError> {
        let start_time = Instant::now();

        let mut conn = self.multiplexed_conn.clone();
        let mut pipe = redis::pipe();
        for ct in cts.into_iter() {
            let bytes = postcard::to_allocvec(&RedisCiphertextRecord {
                ct_type: ct.ciphertext.type_num(),
                raw_ct: Some(ct.ciphertext.serialize().1),
                compressed_ct: None,
            })?;
            pipe.set(hex::encode(&ct.handle), bytes);
        }

        pipe.query_async::<()>(&mut conn).await?;

        let elapsed = start_time.elapsed();
        REDIS_BATCH_STORE_OVERHEAD.observe(elapsed.as_secs_f64());
        info!(elapsed = ?elapsed, "Batch stored computed ciphertexts in Redis");
        Ok(())
    }

    pub(crate) fn deserialize_redis_entry(
        &self,
        handle: Handle,
        payload: Vec<u8>,
    ) -> Result<CiphertextInfo, ComputeError> {
        let entry: RedisCiphertextRecord = postcard::from_bytes(&payload)?;

        let ciphertext = if entry.raw_ct.is_some() {
            SupportedFheCiphertexts::deserialize(
                entry.ct_type,
                &entry.raw_ct.expect("Raw ciphertext must be present"),
            )
            .map_err(|e| ComputeError::Tfhe(e.to_string()))?
        } else {
            SupportedFheCiphertexts::decompress_no_memcheck(
                entry.ct_type,
                &entry
                    .compressed_ct
                    .expect("Compressed ciphertext must be present"),
            )
            .map_err(|e| ComputeError::Tfhe(e.to_string()))?
        };

        Ok(CiphertextInfo { handle, ciphertext })
    }

    /// Check if the ciphertext with the given handle exists in any of
    /// local cache
    /// Redis.
    /// If not, wait for it to be published via Redis keyspace notifications.
    pub(crate) async fn get_or_wait_for_handle(
        &self,
        handle: &Handle,
    ) -> Result<CiphertextInfo, ComputeError> {
        let key: String = hex::encode(handle);

        // Check local cache
        if let Some(ct) = self.cache_get(handle).await {
            info!(handle = %hex::encode(handle), "Cache hit for ciphertext");
            CACHE_HITS_COUNTER.inc();
            return Ok(ct);
        }

        // No local cache hit, proceed to Redis
        let mut conn = self.multiplexed_conn.clone();

        // Check Redis directly
        if let Ok(Some(bytes)) = conn.get::<_, Option<Vec<u8>>>(&key).await {
            info!(handle = %hex::encode(handle), "Redis hit for ciphertext");
            REDIS_HITS_COUNTER.inc();

            let ct = self.deserialize_redis_entry(handle.clone(), bytes)?;
            self.cache_store(&ct).await;
            return Ok(ct);
        }

        info!(handle = %hex::encode(handle), "Redis-subscribe for ciphertext");
        REDIS_SUB_COUNTER.inc();

        // Subscribe to keyspace notifications
        let mut pubsub = self.redis.get_async_pubsub().await?;
        let channel = format!("__keyspace@0__:{}", key);
        pubsub.subscribe(&channel).await?;

        let mut stream = pubsub.on_message();

        let start_time = Instant::now();

        // Wait until the key is actually set, then fetch it
        while let Some(msg) = stream.next().await {
            let payload: String = msg.get_payload()?;

            if payload == "set" || payload == "hset" {
                let elapsed = start_time.elapsed();
                REDIS_SUB_OVERHEAD.observe(elapsed.as_secs_f64());
                info!(handle = %hex::encode(handle), elapsed = ?elapsed, "Received Redis notification for ciphertext");

                if let Ok(Some(value)) = conn.get::<_, Option<Vec<u8>>>(&key).await {
                    let ct = self.deserialize_redis_entry(handle.clone(), value.clone())?;
                    self.cache_store(&ct).await;
                    return Ok(ct);
                }
            }

            if start_time.elapsed() > std::time::Duration::from_secs(30) {
                error!(handle = %hex::encode(handle), "Timeout while waiting for Redis notification for ciphertext");
                return Err(ComputeError::Other(
                    "Timeout while waiting for ciphertext".to_string(),
                ));
            }
        }

        error!(handle = %hex::encode(handle), "Stream ended before receiving Redis notification for ciphertext");

        Err(ComputeError::Other(
            "Stream ended before key was set".to_string(),
        ))
    }

    pub async fn send_partition_complete(&self, payload: Vec<u8>) -> Result<(), ComputeError> {
        let confirm = self
            .complete_partition_channel
            .basic_publish(
                "",
                "fhe_partition_execution_complete_queue",
                lapin::options::BasicPublishOptions::default(),
                &payload,
                lapin::BasicProperties::default(),
            )
            .await?;

        let confirm = confirm.await?;
        info!(confirm = ?confirm, "Sent partition complete message to dispatcher");

        Ok(())
    }
}
