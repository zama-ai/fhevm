use std::ops::DerefMut;

use sqlx::{Pool, Postgres, Transaction};
use tracing::info;

use fhevm_engine_common::tenant_keys::write_large_object_in_chunks_tx;

use crate::{ChainId, KeyType, TenantId};

pub async fn tenant_id(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
) -> anyhow::Result<Option<TenantId>> {
    let rows = sqlx::query!(
        "SELECT tenant_id FROM tenants WHERE chain_id = $1",
        chain_id as i64
    )
    .fetch_all(db_pool)
    .await?;
    if rows.len() > 1 {
        anyhow::bail!("Multiple tenants found for chain_id {chain_id}");
    } else if rows.is_empty() {
        return Ok(None);
    }
    info!(
        "Found tenant_id {} for chain_id {}",
        rows[0].tenant_id, chain_id
    );
    Ok(Some(rows[0].tenant_id as TenantId))
}

const CHUNK_SIZE: usize = 128 * 1024 * 1024; // 128MB

pub async fn update_tenant_key(
    tx: &mut Transaction<'_, Postgres>,
    key_id: &[u8],
    key_type: KeyType,
    key_bytes: &[u8],
    reduced_key_bytes: Option<Vec<u8>>,
    tenant_id: TenantId,
    host_chain_id: ChainId,
) -> anyhow::Result<()> {
    let query = match key_type {
        KeyType::ServerKey => {
            info!(tenant_id, host_chain_id, key_id, "Updating server key");
            let Some(reduced_key_bytes) = reduced_key_bytes else {
                anyhow::bail!("Reduced key bytes must be provided for server key");
            };
            let oid = write_large_object_in_chunks_tx(tx, key_bytes, CHUNK_SIZE).await?;
            sqlx::query!(
                "UPDATE tenants
                SET
                    sns_pk = $1,
                    sks_key = $2,
                    key_id = $3
                WHERE tenant_id = $4 AND chain_id = $5",
                oid,
                reduced_key_bytes,
                key_id,
                tenant_id as i32,
                host_chain_id as i64,
            )
        }
        KeyType::PublicKey => {
            info!(tenant_id, host_chain_id, key_id, "Updating public key");
            sqlx::query!(
                "UPDATE tenants
                SET
                    pks_key = $1,
                    key_id = $2
                WHERE tenant_id = $3 AND chain_id = $4",
                key_bytes,
                key_id,
                tenant_id as i32,
                host_chain_id as i64,
            )
        }
    };
    let result = query.execute(tx.deref_mut()).await?;
    if result.rows_affected() == 0 {
        anyhow::bail!(
            "No tenant found for tenant_id {} and chain_id {}",
            tenant_id,
            host_chain_id
        );
    }
    Ok(())
}

pub async fn update_tenant_crs(
    tx: &mut Transaction<'_, Postgres>,
    key_bytes: &[u8],
    tenant_id: TenantId,
    chain_id: ChainId,
) -> anyhow::Result<()> {
    info!(tenant_id, chain_id, "Updating crs");
    let query = sqlx::query!(
        "UPDATE tenants
        SET public_params = $1
        WHERE tenant_id = $2 AND chain_id = $3",
        key_bytes,
        tenant_id as i32,
        chain_id as i64,
    );
    let result = query.execute(tx.deref_mut()).await?;
    if result.rows_affected() == 0 {
        anyhow::bail!(
            "No tenant found for tenant_id {} and chain_id {}",
            tenant_id,
            chain_id
        );
    }
    Ok(())
}
