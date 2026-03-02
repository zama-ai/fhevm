use fhevm_engine_common::{
    db_keys::{read_keys_from_large_object_by_key_id_gw, DbKeyId},
    utils::safe_deserialize_sns_key,
};
use sqlx::{PgPool, Row};
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{ExecutionError, KeySet};

const SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 1_150 * 1_000_000; // ~1.1 GB

async fn fetch_latest_key_id_gw(pool: &PgPool) -> Result<Option<(DbKeyId, i64)>, ExecutionError> {
    let record = sqlx::query(
        "SELECT key_id_gw, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = record {
        let key_id_gw: DbKeyId = record.try_get("key_id_gw")?;
        let sequence_number: i64 = record.try_get("sequence_number")?;
        Ok(Some((key_id_gw, sequence_number)))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_latest_keyset(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
) -> Result<Option<(DbKeyId, KeySet)>, ExecutionError> {
    let Some((key_id_gw, _sequence_number)) = fetch_latest_key_id_gw(pool).await? else {
        return Ok(None);
    };

    let keyset = fetch_keyset_by_id(cache, pool, &key_id_gw).await?;
    Ok(keyset.map(|keys| (key_id_gw, keys)))
}

async fn fetch_keyset_by_id(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
    key_id_gw: &DbKeyId,
) -> Result<Option<KeySet>, ExecutionError> {
    {
        let mut cache = cache.write().await;
        if let Some(keys) = cache.get(key_id_gw) {
            info!(key_id_gw = hex::encode(key_id_gw), "Cache hit");
            return Ok(Some(keys.clone()));
        }
    }

    info!(key_id_gw = hex::encode(key_id_gw), "Cache miss");

    let blob = read_keys_from_large_object_by_key_id_gw(
        pool,
        key_id_gw.clone(),
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
    let client_key = fetch_client_key(pool, key_id_gw).await?;

    let key_set = KeySet {
        key_id_gw: key_id_gw.clone(),
        client_key,
        server_key,
    };

    let mut cache = cache.write().await;
    cache.put(key_id_gw.clone(), key_set.clone());
    Ok(Some(key_set))
}

pub async fn fetch_client_key(
    pool: &PgPool,
    key_id_gw: &DbKeyId,
) -> anyhow::Result<Option<tfhe::ClientKey>> {
    let keys = sqlx::query("SELECT cks_key FROM keys WHERE key_id_gw = $1")
        .bind(key_id_gw)
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
