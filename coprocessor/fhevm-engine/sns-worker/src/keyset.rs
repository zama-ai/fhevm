use std::sync::Arc;

use fhevm_engine_common::{
    tenant_keys::read_keys_from_large_object, utils::safe_deserialize_sns_key,
};
use sqlx::{PgPool, Row};
use tokio::sync::RwLock;
use tracing::info;

use crate::{ExecutionError, KeySet};

const SKS_KEY_WITH_NOISE_SQUASHING_SIZE: usize = 1_150 * 1_000_000; // ~1.1 GB

/// Retrieve the keyset from the database
pub(crate) async fn fetch_keyset(
    cache: &Arc<RwLock<lru::LruCache<String, KeySet>>>,
    pool: &PgPool,
    tenant_api_key: &String,
) -> Result<Option<KeySet>, ExecutionError> {
    let mut cache = cache.write().await;
    if let Some(keys) = cache.get(tenant_api_key) {
        info!(tenant_api_key, "Cache hit");
        return Ok(Some(keys.clone()));
    }

    info!(tenant_api_key, "Cache miss");
    let Some((client_key, server_key)) = fetch_keys(pool, tenant_api_key).await? else {
        return Ok(None);
    };
    let key_set: KeySet = KeySet {
        client_key,
        server_key,
    };

    cache.push(tenant_api_key.clone(), key_set.clone());
    Ok(Some(key_set))
}

/// Retrieve both the ClientKey and ServerKey from the tenants table
///
/// The ServerKey is stored in a large object (LOB) in the database.
/// ServerKey must be generated with enable_noise_squashing option.
///
/// The ClientKey is stored in a bytea column and is optional. It's used only
/// for decrypting on testing.
pub async fn fetch_keys(
    pool: &PgPool,
    tenant_api_key: &String,
) -> anyhow::Result<Option<(Option<tfhe::ClientKey>, tfhe::ServerKey)>> {
    let blob = read_keys_from_large_object(
        pool,
        tenant_api_key,
        "sns_pk",
        SKS_KEY_WITH_NOISE_SQUASHING_SIZE,
    )
    .await?;
    info!(bytes_len = blob.len(), "Retrieved sns_pk");
    if blob.is_empty() {
        return Ok(None);
    }

    let server_key: tfhe::ServerKey = safe_deserialize_sns_key(&blob)?;

    let keys = sqlx::query(
        "
                SELECT cks_key FROM tenants
                WHERE tenant_api_key = $1::uuid
            ",
    )
    .bind(tenant_api_key)
    .fetch_one(pool)
    .await?;

    if let Ok(cks) = keys.try_get::<Vec<u8>, _>(0) {
        if !cks.is_empty() {
            info!(bytes_len = cks.len(), "Retrieved cks");
            let client_key: tfhe::ClientKey = safe_deserialize_sns_key(&cks)?;
            return Ok(Some((Some(client_key), server_key)));
        }
    }

    Ok(Some((None, server_key)))
}
