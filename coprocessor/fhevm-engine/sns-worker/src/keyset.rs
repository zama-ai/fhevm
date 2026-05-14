use fhevm_engine_common::{
    db_keys::{read_sns_pk_with_fallback, DbKeyId, SnsPkEncoding},
    utils::safe_deserialize_sns_key,
};
use sqlx::PgPool;
use std::sync::Arc;
use tokio::sync::RwLock;
use tracing::info;

use crate::{ExecutionError, KeySet};

#[cfg(not(feature = "gpu"))]
fn decode_server_key(
    blob: &[u8],
    encoding: SnsPkEncoding,
) -> Result<tfhe::ServerKey, ExecutionError> {
    match encoding {
        SnsPkEncoding::Compressed => {
            let csks: tfhe::CompressedServerKey = safe_deserialize_sns_key(blob)?;
            info!("Decompressing compressed sns_pk to ServerKey");
            Ok(csks.decompress())
        }
        SnsPkEncoding::Legacy => Ok(safe_deserialize_sns_key(blob)?),
    }
}

// GPU requires a CompressedServerKey. The XOF ingest path lands one in
// sns_pk_compressed; the legacy fallback path lands a plain ServerKey
// in sns_pk which the deserialize below cannot consume. LFS test
// fixtures put a CompressedServerKey into the sns_pk column directly,
// so we still attempt the deserialize on Legacy rows and wrap the
// failure with an operator-facing diagnostic.
#[cfg(feature = "gpu")]
fn decode_server_key(
    blob: &[u8],
    encoding: SnsPkEncoding,
) -> Result<tfhe::CudaServerKey, ExecutionError> {
    let compressed_server_key: tfhe::CompressedServerKey =
        safe_deserialize_sns_key(blob).map_err(|err| match encoding {
            SnsPkEncoding::Compressed => anyhow::anyhow!(
                "failed to deserialize CompressedServerKey from sns_pk_compressed: {err}"
            ),
            SnsPkEncoding::Legacy => anyhow::anyhow!(
                "GPU coprocessor cannot read a legacy ServerKey-format key (sns_pk_compressed is NULL); \
                 rotate kms-core to publish CompressedXofKeySet so the host-listener can ingest it into the compressed column. \
                 Underlying deserialize error: {err}"
            ),
        })?;
    info!("Deserialized sns_pk/sks_ns to CompressedServerKey");
    let server_key = compressed_server_key.decompress_to_gpu();
    info!("Decompressed sns_pk/sks_ns to CudaServerKey");
    Ok(server_key)
}

/// Receive-buffer hint for a legacy plain-ServerKey sns_pk LOB. Sized
/// for the production NS-enabled key (decompressed).
const SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 1_150 * 1_000_000; // ~1.1 GB
/// Receive-buffer hint for a CompressedServerKey sns_pk_compressed LOB.
/// Compressed XOF blobs are ~3-4x smaller than the decompressed
/// equivalent; sizing the buffer to the legacy capacity would burn
/// virtual address space (and physical RAM on strict-overcommit
/// hosts) before any read happens.
const COMPRESSED_SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 350 * 1_000_000; // ~350 MB

async fn fetch_latest_key_id_gw(pool: &PgPool) -> Result<Option<(DbKeyId, i64)>, ExecutionError> {
    let record = sqlx::query!(
        "SELECT key_id_gw, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?;

    if let Some(record) = record {
        let key_id_gw: DbKeyId = record.key_id_gw;
        let sequence_number: i64 = record.sequence_number;
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

    let (blob, encoding) = read_sns_pk_with_fallback(
        pool,
        key_id_gw.clone(),
        COMPRESSED_SKS_KEY_WITH_NOISE_SQUASHING_SIZE,
        SKS_KEY_WITH_NOISE_SQUASHING_SIZE,
    )
    .await?;
    info!(
        bytes_len = blob.len(),
        ?encoding,
        "Fetched sns_pk/sks_ns bytes from LOB"
    );
    if blob.is_empty() {
        return Ok(None);
    }

    let server_key = decode_server_key(&blob, encoding)?;

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
