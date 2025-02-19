use fhevm_engine_common::utils::safe_deserialize_key;
use sqlx::postgres::types::Oid;
use sqlx::postgres::PgRow;
use sqlx::{PgPool, Row};
use tokio::join;

use tracing::info;

use crate::switch_and_squash::{SnsClientKey, SwitchAndSquashKey};
use crate::{ExecutionError, KeySet};

/// Retrieve the keyset from the database
pub(crate) async fn fetch_keyset(pool: &PgPool, tenant_id: i32) -> Result<KeySet, ExecutionError> {
    let (server_key, sns_key, sns_secret_key) = join!(
        read_sks_key(pool, tenant_id),
        read_sns_pk_from_lo(pool, tenant_id),
        read_sns_sk_from_lo(pool, tenant_id)
    );

    let server_key = server_key?;
    let sns_key = sns_key?;
    let sns_secret_key = sns_secret_key?;

    let key_set = KeySet {
        sns_key,
        sns_secret_key,
        server_key,
    };

    Ok(key_set)
}

/// Retrieve the SwitchAndSquashKey from the large object table
async fn read_sns_pk_from_lo(pool: &PgPool, tenant_id: i32) -> anyhow::Result<SwitchAndSquashKey> {
    let bytes = read_keys_from_lo(pool, tenant_id, "sns_pk").await?;
    info!(target: "sns", "Retrieved sns_pk bytes length: {:?}", bytes.len());
    let sns_pk: SwitchAndSquashKey = bincode::deserialize(&bytes)?;
    anyhow::Ok(sns_pk)
}

/// Retrieve the SnsClientKey from the large object table
///
/// SnsClientKey is supposed to be used only for testing purposes
pub async fn read_sns_sk_from_lo(
    pool: &PgPool,
    tenant_id: i32,
) -> anyhow::Result<Option<SnsClientKey>> {
    let bytes = read_keys_from_lo(pool, tenant_id, "sns_sk")
        .await
        .map_or(vec![], |b| b);

    if bytes.is_empty() {
        return Ok(None);
    }

    info!(target: "sns", "Retrieved sns_sk bytes length: {:?}", bytes.len());
    let sns_sk: SnsClientKey = bincode::deserialize(&bytes)?;
    anyhow::Ok(Some(sns_sk))
}

/// Retrieve the keys from the large object saved with OID in tenant table
async fn read_keys_from_lo(
    pool: &PgPool,
    tenant_id: i32,
    keys_column_name: &str,
) -> anyhow::Result<Vec<u8>> {
    // Query the sns_pk column for the given tenant ID
    let query = format!(
        "SELECT {} FROM tenants WHERE tenant_id = $1",
        keys_column_name
    );

    let row: PgRow = sqlx::query(&query).bind(tenant_id).fetch_one(pool).await?;
    let oid: Oid = row.try_get(0)?;

    info!(target: "sns", "Retrieved oid: {:?}", oid);

    // Retrieve the large object data
    let bytes: Vec<u8> = sqlx::query_scalar("SELECT lo_get($1)")
        .bind(oid)
        .fetch_one(pool)
        .await?;

    anyhow::Ok(bytes)
}

/// Retrieve the ServerKey from the tenants table
pub async fn read_sks_key(pool: &PgPool, tenant_id: i32) -> anyhow::Result<tfhe::ServerKey> {
    let sks = sqlx::query(
        "
            SELECT sks_key
            FROM tenants
            WHERE tenant_id = $1
        ",
    )
    .bind(tenant_id)
    .fetch_one(pool)
    .await?;

    let sks_key: Vec<u8> = sks.try_get(0)?;
    let server_key: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;

    Ok(server_key)
}
