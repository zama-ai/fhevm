use fhevm_engine_common::{
    db_keys::{read_keys_from_large_object, DbKeyId},
    utils::safe_deserialize_sns_key,
};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{ExecutionError, KeySet};

const SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 1_150 * 1_000_000; // ~1.1 GB

async fn fetch_latest_key_id(pool: &PgPool) -> Result<Option<(DbKeyId, i64)>, ExecutionError> {
    let record = sqlx::query(
        "SELECT key_id, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = record {
        let key_id: DbKeyId = record.try_get("key_id")?;
        let sequence_number: i64 = record.try_get("sequence_number")?;
        Ok(Some((key_id, sequence_number)))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_latest_keyset(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
) -> Result<Option<(DbKeyId, KeySet)>, ExecutionError> {
    let Some((key_id, _sequence_number)) = fetch_latest_key_id(pool).await? else {
        return Ok(None);
    };

    let keyset = fetch_keyset_by_id(cache, pool, &key_id).await?;
    Ok(keyset.map(|keys| (key_id, keys)))
}

async fn fetch_keyset_by_id(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
    key_id: &DbKeyId,
) -> Result<Option<KeySet>, ExecutionError> {
    {
        let mut cache = cache.write().await;
        if let Some(keys) = cache.get(key_id) {
            info!(key_id = hex::encode(key_id), "Cache hit");
            return Ok(Some(keys.clone()));
        }
    }

    info!(key_id = hex::encode(key_id), "Cache miss");

    let blob = read_keys_from_large_object(
        pool,
        key_id.clone(),
        "sns_pk",
        SKS_KEY_WITH_NOISE_SQUASHING_SIZE,
    )
    .await?;
    info!(
        bytes_len = blob.len(),
        "Fetched sns_pk/sks_ns bytes from LOB"
    );
    if blob.is_empty() {
        return Ok(None);
    }

    #[cfg(not(feature = "gpu"))]
    let server_key: tfhe::ServerKey = safe_deserialize_sns_key(&blob)?;

    #[cfg(feature = "gpu")]
    let server_key = {
        let compressed_server_key: tfhe::CompressedServerKey = safe_deserialize_sns_key(&blob)?;
        info!("Deserialized sns_pk/sks_ns to CompressedServerKey");

        let server_key = compressed_server_key.decompress_to_gpu();
        info!("Decompressed sns_pk/sks_ns to CudaServerKey");
        server_key
    };

    // Optionally retrieve the ClientKey for testing purposes
    let client_key = fetch_client_key(pool, key_id).await?;

    let key_set = KeySet {
        key_id: key_id.clone(),
        client_key,
        server_key,
    };

    let mut cache = cache.write().await;
    cache.put(key_id.clone(), key_set.clone());
    Ok(Some(key_set))
}

pub async fn fetch_client_key(
    pool: &PgPool,
    key_id: &DbKeyId,
) -> anyhow::Result<Option<tfhe::ClientKey>> {
    let keys = sqlx::query("SELECT cks_key FROM keys WHERE key_id = $1")
        .bind(key_id)
        .fetch_optional(pool)
        .await?;

    if let Some(keys) = keys {
        if let Ok(cks) = keys.try_get::<Vec<u8>, _>(0) {
            if !cks.is_empty() {
                info!(bytes_len = cks.len(), "Retrieved cks");
                let client_key: tfhe::ClientKey = safe_deserialize_sns_key(&cks)?;
                return Ok(Some(client_key));
            }
        }
    }
    Ok(None)
}
