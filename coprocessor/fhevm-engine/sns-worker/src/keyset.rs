use fhevm_engine_common::{
    db_keys::{
        read_compressed_sns_key_material, read_default_sns_key_material, DbKeyId,
        StoredServerKeyFormat,
    },
    utils::safe_deserialize_sns_key,
};
use sqlx::PgPool;
use std::sync::Arc;
use tfhe::xof_key_set::CompressedXofKeySet;
use tokio::sync::RwLock;
use tracing::info;

use crate::{ExecutionError, KeySet};

#[cfg(not(feature = "gpu"))]
fn decode_server_key(
    blob: &[u8],
    stored_format: StoredServerKeyFormat,
) -> Result<tfhe::ServerKey, ExecutionError> {
    match stored_format {
        StoredServerKeyFormat::CompressedXof => {
            let kxs: CompressedXofKeySet = safe_deserialize_sns_key(blob)?;
            info!("Decompressing CompressedXofKeySet to ServerKey");
            let (_public_key, server_key) = kxs.decompress()?.into_raw_parts();
            Ok(server_key)
        }
        StoredServerKeyFormat::Legacy => Ok(safe_deserialize_sns_key(blob)?),
    }
}

// GPU requires a CudaServerKey. The XOF ingest path lands a
// CompressedXofKeySet in compressed_xof_keyset; the legacy fallback
// path lands a plain ServerKey in sns_pk which the GPU path cannot
// consume.
#[cfg(feature = "gpu")]
fn decode_server_key(
    blob: &[u8],
    stored_format: StoredServerKeyFormat,
) -> Result<tfhe::CudaServerKey, ExecutionError> {
    if stored_format == StoredServerKeyFormat::Legacy {
        return Err(anyhow::anyhow!(
            "GPU coprocessor cannot read a legacy ServerKey-format key (compressed_xof_keyset is NULL); \
             rotate kms-core to publish CompressedXofKeySet so the host-listener can ingest it into the compressed column"
        )
        .into());
    }

    let kxs: CompressedXofKeySet = safe_deserialize_sns_key(blob).map_err(|err| {
        anyhow::anyhow!(
            "failed to deserialize CompressedXofKeySet from compressed_xof_keyset: {err}"
        )
    })?;
    info!("Deserialized compressed_xof_keyset to CompressedXofKeySet");
    let (_public_key, server_key) = kxs.decompress_to_gpu()?.into_raw_parts();
    info!("Decompressed compressed_xof_keyset to CudaServerKey");
    Ok(server_key)
}

/// Receive-buffer hint for a legacy plain-ServerKey sns_pk LOB. Sized
/// for the production NS-enabled key (decompressed).
const SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 1_150 * 1_000_000; // ~1.1 GB

struct LatestActiveKey {
    key_id_gw: DbKeyId,
    sequence_number: i64,
}

async fn fetch_latest_active_key(pool: &PgPool) -> Result<Option<LatestActiveKey>, ExecutionError> {
    let record = sqlx::query!(
        "SELECT key_id_gw, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = record {
        Ok(Some(LatestActiveKey {
            key_id_gw: record.key_id_gw,
            sequence_number: record.sequence_number,
        }))
    } else {
        Ok(None)
    }
}

pub(crate) async fn fetch_latest_keyset(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
) -> Result<Option<(DbKeyId, KeySet)>, ExecutionError> {
    let Some(active_key) = fetch_latest_active_key(pool).await? else {
        return Ok(None);
    };

    let keyset = fetch_default_keyset(cache, pool, &active_key).await?;
    Ok(keyset.map(|keys| (active_key.key_id_gw, keys)))
}

/// Fetches the compressed (RFC-029 v1) keyset for the latest key.
pub(crate) async fn fetch_migrated_keyset(pool: &PgPool) -> Result<Option<KeySet>, ExecutionError> {
    let Some(active_key) = fetch_latest_active_key(pool).await? else {
        return Ok(None);
    };
    let Some(blob) = read_compressed_sns_key_material(pool, active_key.sequence_number).await?
    else {
        return Ok(None);
    };
    if blob.is_empty() {
        return Ok(None);
    }
    let server_key = decode_server_key(&blob, StoredServerKeyFormat::CompressedXof)?;
    let client_key = fetch_client_key_for_active_key(pool, &active_key).await?;
    Ok(Some(KeySet {
        key_id_gw: active_key.key_id_gw,
        sequence_number: active_key.sequence_number,
        client_key,
        server_key,
    }))
}

async fn fetch_default_keyset(
    cache: &Arc<RwLock<lru::LruCache<DbKeyId, KeySet>>>,
    pool: &PgPool,
    active_key: &LatestActiveKey,
) -> Result<Option<KeySet>, ExecutionError> {
    {
        let mut cache = cache.write().await;
        if let Some(keys) = cache.get(&active_key.key_id_gw) {
            if keys.sequence_number == active_key.sequence_number {
                info!(
                    key_id_gw = hex::encode(&active_key.key_id_gw),
                    sequence_number = active_key.sequence_number,
                    "Cache hit"
                );
                return Ok(Some(keys.clone()));
            }
            info!(
                key_id_gw = hex::encode(&active_key.key_id_gw),
                cached_sequence_number = keys.sequence_number,
                latest_sequence_number = active_key.sequence_number,
                "Cache entry is stale"
            );
        }
    }

    info!(
        key_id_gw = hex::encode(&active_key.key_id_gw),
        sequence_number = active_key.sequence_number,
        "Cache miss"
    );

    let (blob, stored_format) = read_default_sns_key_material(
        pool,
        active_key.sequence_number,
        SKS_KEY_WITH_NOISE_SQUASHING_SIZE,
    )
    .await?;
    info!(
        bytes_len = blob.len(),
        ?stored_format,
        "Fetched SNS key material"
    );
    if blob.is_empty() {
        return Ok(None);
    }

    let server_key = decode_server_key(&blob, stored_format)?;

    // Optionally retrieve the ClientKey for testing purposes
    let client_key = fetch_client_key_for_active_key(pool, active_key).await?;

    let key_set = KeySet {
        key_id_gw: active_key.key_id_gw.clone(),
        sequence_number: active_key.sequence_number,
        client_key,
        server_key,
    };

    let mut cache = cache.write().await;
    cache.put(active_key.key_id_gw.clone(), key_set.clone());
    Ok(Some(key_set))
}

async fn fetch_client_key_for_active_key(
    pool: &PgPool,
    active_key: &LatestActiveKey,
) -> anyhow::Result<Option<tfhe::ClientKey>> {
    let keys = sqlx::query!(
        "SELECT cks_key FROM keys WHERE sequence_number = $1",
        active_key.sequence_number
    )
    .fetch_optional(pool)
    .await?;

    if let Some(cks) = keys {
        if let Some(cks) = cks.cks_key {
            if !cks.is_empty() {
                info!(
                    bytes_len = cks.len(),
                    sequence_number = active_key.sequence_number,
                    "Retrieved cks"
                );
                let client_key: tfhe::ClientKey = safe_deserialize_sns_key(&cks)?;
                return Ok(Some(client_key));
            }
        }
    }
    Ok(None)
}

#[cfg(test)]
pub async fn fetch_client_key(
    pool: &PgPool,
    key_id_gw: &DbKeyId,
) -> anyhow::Result<Option<tfhe::ClientKey>> {
    let keys = sqlx::query!("SELECT cks_key FROM keys WHERE key_id_gw = $1", key_id_gw)
        .fetch_optional(pool)
        .await?;

    if let Some(cks) = keys {
        if let Some(cks) = cks.cks_key {
            if !cks.is_empty() {
                info!(bytes_len = cks.len(), "Retrieved cks");
                let client_key: tfhe::ClientKey = safe_deserialize_sns_key(&cks)?;
                return Ok(Some(client_key));
            }
        }
    }
    Ok(None)
}
