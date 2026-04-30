use std::ops::DerefMut;

use alloy::rpc::types::Log;
use sqlx::{Postgres, Transaction};
use tracing::{error, info};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::db_keys::write_large_object_in_chunks_tx;

use crate::contracts::KMSGeneration;
use crate::kms_generation::key_id_to_database_bytes;

const CHUNK_SIZE: usize = 128 * 1024 * 1024; // 128MB

#[derive(Debug, Clone)]
pub(crate) struct PendingKeyActivation {
    pub chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub key_id: Vec<u8>,
    pub digest_server: Option<Vec<u8>>,
    pub digest_public: Option<Vec<u8>>,
    pub has_server_key: bool,
    pub has_public_key: bool,
    pub storage_urls: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct PendingCrsActivation {
    pub chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub crs_id: Vec<u8>,
    pub digest: Vec<u8>,
    pub storage_urls: Vec<String>,
}

pub(crate) async fn insert_key_activation_event(
    tx: &mut Transaction<'_, Postgres>,
    activation: KMSGeneration::ActivateKey,
    log: Log,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    let transaction_hash = log.transaction_hash.map(|txh| txh.to_vec());
    let digest_server = activation
        .keyDigests
        .iter()
        .filter(|d| d.keyType == 0)
        .map(|d| d.digest.to_vec())
        .next();
    let digest_public = activation
        .keyDigests
        .iter()
        .filter(|d| d.keyType == 1)
        .map(|d| d.digest.to_vec())
        .next();
    let urls = activation.kmsNodeStorageUrls.clone();
    sqlx::query!(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            key_id,
            key_digest_server,
            key_digest_public,
            storage_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (chain_id, block_hash, key_id)
        DO NOTHING
        "#,
        chain_id.as_i64(),
        block_hash,
        block_number as i64,
        transaction_hash,
        &key_id_to_database_bytes(activation.keyId),
        digest_server,
        digest_public,
        &urls
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn insert_crs_activation_event(
    tx: &mut Transaction<'_, Postgres>,
    activation: KMSGeneration::ActivateCrs,
    log: Log,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query!(
        r#"
        INSERT INTO kms_crs_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            crs_id,
            crs_digest,
            storage_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (chain_id, block_hash, crs_id)
        DO NOTHING
        "#,
        chain_id.as_i64(),
        block_hash,
        block_number as i64,
        log.transaction_hash.map(|txh| txh.to_vec()),
        &key_id_to_database_bytes(activation.crsId),
        activation.crsDigest.to_vec(),
        &activation.kmsNodeStorageUrls
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn count_key_activation_remaining_pending(
    db_pool: &sqlx::Pool<Postgres>,
) -> anyhow::Result<u64> {
    let row = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM kms_key_activation_events
        WHERE status = 'pending'
        "#,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.unwrap_or(0) as u64)
}

pub(crate) async fn count_crs_activation_remaining_pending(
    db_pool: &sqlx::Pool<Postgres>,
) -> anyhow::Result<u64> {
    let row = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)
        FROM kms_crs_activation_events
        WHERE status = 'pending'
        "#,
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.unwrap_or(0) as u64)
}

pub(crate) async fn cancel_orphaned_key_activations(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let query = sqlx::query!(
        "
        UPDATE kms_key_activation_events AS e
        SET status = 'cancelled'
        FROM host_chain_blocks_valid AS b
        WHERE
            e.status IN ('pending', 'ready')
            AND e.chain_id = b.chain_id
            AND e.block_hash = b.block_hash
            AND b.block_status = 'orphaned'
        "
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() > 0 {
        info!("Marked {} pending key activations as cancelled due to orphaned blocks", query.rows_affected());
    }
    Ok(query.rows_affected())
}

pub(crate) async fn cancel_orphaned_crs_activations(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let query = sqlx::query!(
        "
        UPDATE kms_crs_activation_events AS e
        SET status = 'cancelled'
        FROM host_chain_blocks_valid AS b
        WHERE
            e.status IN ('pending', 'ready')
            AND e.chain_id = b.chain_id
            AND e.block_hash = b.block_hash
            AND b.block_status = 'orphaned'
        "
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() > 0 {
        info!("Marked {} pending CRS activations as cancelled due to orphaned blocks", query.rows_affected());
    }
    Ok(query.rows_affected())
}

pub(crate) async fn activate_ready_key_activations(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let to_activate = sqlx::query!(
        r#"
        SELECT e.chain_id, e.block_hash, e.key_id
        FROM kms_key_activation_events AS e
        INNER JOIN host_chain_blocks_valid AS b
            ON e.chain_id = b.chain_id
            AND e.block_hash = b.block_hash
        WHERE
            e.status = 'ready'
            AND b.block_status = 'finalized'
            AND e.key_content_public IS NOT NULL
            AND e.key_content_sks_key IS NOT NULL
        FOR UPDATE OF e SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut done = 0;
    if to_activate.is_empty() {
        info!("No ready key activation to activate");
        return Ok(0);
    } else {
        info!(
            ?to_activate,
            len = to_activate.len(),
            "Ready to activate key activations"
        );
    }
    for row in to_activate {
        let chain_id = row.chain_id;
        let block_hash = row.block_hash;
        let key_id = row.key_id;

        let update_result = sqlx::query!(
            r#"
            INSERT INTO keys (
                chain_id, block_hash, key_id_gw, key_id,
                pks_key, sks_key, sns_pk
            )
            SELECT
                e.chain_id, e.block_hash, e.key_id, e.key_id,
                e.key_content_public, e.key_content_sks_key, e.key_content_sns_pk
            FROM kms_key_activation_events AS e
            WHERE
                e.chain_id = $1
                AND e.block_hash = $2
                AND e.key_id = $3
                AND e.key_content_public IS NOT NULL
                AND e.key_content_sks_key IS NOT NULL
            ON CONFLICT (chain_id, block_hash, key_id_gw) DO UPDATE
            SET pks_key   = EXCLUDED.pks_key,
                sks_key   = EXCLUDED.sks_key,
                sns_pk    = COALESCE(EXCLUDED.sns_pk, keys.sns_pk),
                key_id_gw = EXCLUDED.key_id_gw
            "#,
            chain_id,
            &block_hash,
            &key_id
        )
        .execute(tx.deref_mut())
        .await?;
        if update_result.rows_affected() == 0 {
            error!(
                chain_id,
                block_hash = ?block_hash,
                key_id = ?key_id,
                "Failed to upsert keys table with activated key content for activation"
            );
            continue;
        }

        let update_result = sqlx::query!(
            r#"
            UPDATE kms_key_activation_events AS e
            SET status = 'activated'
            WHERE
                e.chain_id = $1
                AND e.block_hash = $2
                AND e.key_id = $3
            "#,
            chain_id,
            &block_hash,
            &key_id
        )
        .execute(tx.deref_mut())
        .await?;
        if update_result.rows_affected() == 0 {
            error!(
                chain_id,
                block_hash = ?block_hash,
                key_id = ?key_id,
                "Failed to update key activation status to activated for ready activation"
            );
        } else {
            done += 1;
        }
    }
    Ok(done)
}

pub(crate) async fn activate_ready_crs_activations(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let to_activate = sqlx::query!(
        r#"
        SELECT e.chain_id, e.block_hash, e.crs_id
        FROM kms_crs_activation_events AS e
        INNER JOIN host_chain_blocks_valid AS b
            ON e.chain_id = b.chain_id
            AND e.block_hash = b.block_hash
        WHERE
            e.status = 'ready'
            AND b.block_status = 'finalized'
            AND e.crs_content IS NOT NULL
        FOR UPDATE OF e SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut done = 0;
    if to_activate.is_empty() {
        info!("No ready CRS activation to activate");
        return Ok(0);
    }
    for row in to_activate {
        let chain_id = row.chain_id;
        let block_hash = row.block_hash;
        let crs_id = row.crs_id;

        let update_result = sqlx::query!(
            r#"
            INSERT INTO crs (chain_id, block_hash, crs_id, crs)
            SELECT e.chain_id, e.block_hash, e.crs_id, e.crs_content
            FROM kms_crs_activation_events AS e
            WHERE
                e.chain_id = $1
                AND e.block_hash = $2
                AND e.crs_id = $3
                AND e.crs_content IS NOT NULL
            ON CONFLICT (chain_id, block_hash, crs_id) DO UPDATE
            SET crs = EXCLUDED.crs
            "#,
            chain_id,
            &block_hash,
            &crs_id
        )
        .execute(tx.deref_mut())
        .await?;
        if update_result.rows_affected() == 0 {
            error!(
                chain_id,
                block_hash = ?block_hash,
                crs_id = ?crs_id,
                "Failed to upsert crs table with activated CRS content for activation"
            );
            continue;
        }

        let update_result = sqlx::query!(
            r#"
            UPDATE kms_crs_activation_events AS e
            SET status = 'activated'
            WHERE
                e.chain_id = $1
                AND e.block_hash = $2
                AND e.crs_id = $3
            "#,
            chain_id,
            &block_hash,
            &crs_id
        )
        .execute(tx.deref_mut())
        .await?;
        if update_result.rows_affected() == 0 {
            error!(
                chain_id,
                block_hash = ?block_hash,
                crs_id = ?crs_id,
                "Failed to update CRS activation status to activated for ready activation"
            );
        } else {
            done += 1;
        }
    }
    Ok(done)
}

pub(crate) async fn all_pending_key_activations_to_download(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<PendingKeyActivation>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            chain_id,
            block_hash,
            key_id,
            key_digest_server,
            key_digest_public,
            key_content_sks_key IS NOT NULL AS has_server_key,
            key_content_public IS NOT NULL AS has_public_key,
            storage_urls
        FROM kms_key_activation_events
        WHERE
            status = 'pending'
            AND (
                key_content_sks_key IS NULL AND key_digest_server IS NOT NULL
                OR key_content_public IS NULL AND key_digest_public IS NOT NULL
            )
        FOR UPDATE SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let Ok(chain_id) = ChainId::try_from(row.chain_id) else {
            // not possible due to db constraint
            error!(
                ?row.chain_id,
                ?row.block_hash,
                ?row.key_id,
                "Invalid chain_id for key activation in db"
            );
            continue;
        };
        result.push(PendingKeyActivation {
            chain_id,
            block_hash: row.block_hash,
            key_id: row.key_id,
            digest_server: row.key_digest_server,
            digest_public: row.key_digest_public,
            has_server_key: row.has_server_key.unwrap_or(false),
            has_public_key: row.has_public_key.unwrap_or(false),
            storage_urls: row.storage_urls,
        });
    }
    Ok(result)
}

pub(crate) async fn all_pending_crs_activations_to_download(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<PendingCrsActivation>> {
    let rows = sqlx::query!(
        r#"
        SELECT
            chain_id,
            block_hash,
            crs_id,
            crs_digest,
            storage_urls
        FROM kms_crs_activation_events
        WHERE
            status = 'pending'
            AND crs_content IS NULL
        FOR UPDATE SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut result = Vec::with_capacity(rows.len());
    for row in rows {
        let Ok(chain_id) = ChainId::try_from(row.chain_id) else {
            error!(
                ?row.chain_id,
                ?row.block_hash,
                ?row.crs_id,
                "Invalid chain_id for CRS activation in db"
            );
            continue;
        };
        result.push(PendingCrsActivation {
            chain_id,
            block_hash: row.block_hash,
            crs_id: row.crs_id,
            digest: row.crs_digest,
            storage_urls: row.storage_urls,
        });
    }
    Ok(result)
}

pub async fn set_ready_key_activation(
    tx: &mut Transaction<'_, Postgres>,
    activation: &PendingKeyActivation,
    sns_pk: Option<Vec<u8>>,
    sks_key: Option<Vec<u8>>,
    public_key: Option<Vec<u8>>,
) -> anyhow::Result<()> {
    let sns_pk_oid = if let Some(sns_pk) = sns_pk {
        Some(write_large_object_in_chunks_tx(tx, &sns_pk, CHUNK_SIZE).await?)
    } else {
        None
    };
    let query = sqlx::query!(
        r#"
        UPDATE kms_key_activation_events
        SET
            status = 'ready',
            key_content_sns_pk = $1,
            key_content_sks_key = $2,
            key_content_public = $3,
            last_updated_at = NOW()
        WHERE chain_id = $4 AND block_hash = $5 AND key_id = $6
        "#,
        sns_pk_oid,
        sks_key,
        public_key,
        activation.chain_id.as_i64(),
        activation.block_hash,
        activation.key_id,
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() == 0 {
        anyhow::bail!("Failed to update downloaded keys for activation with key_id {:?} and block_hash {:?}",
            activation.key_id, activation.block_hash);
    }
    Ok(())
}

pub async fn set_ready_crs_activation(
    tx: &mut Transaction<'_, Postgres>,
    activation: &PendingCrsActivation,
    crs_content: Option<Vec<u8>>,
) -> anyhow::Result<()> {
    let query = sqlx::query!(
        r#"
        UPDATE kms_crs_activation_events
        SET
            status = 'ready',
            crs_content = $1,
            last_updated_at = NOW()
        WHERE chain_id = $2 AND block_hash = $3 AND crs_id = $4
        "#,
        crs_content,
        activation.chain_id.as_i64(),
        activation.block_hash,
        activation.crs_id,
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() == 0 {
        anyhow::bail!("Failed to update downloaded CRS for activation with crs_id {:?} and block_hash {:?}",
            activation.crs_id, activation.block_hash);
    }
    Ok(())
}

pub async fn mark_key_activation_error(
    tx: &mut Transaction<'_, Postgres>,
    error_msg: &str,
    activation: PendingKeyActivation,
) {
    if let Err(err) = sqlx::query!(
        r#"
        UPDATE kms_key_activation_events
        SET last_error = $1, last_updated_at = NOW(), retry_count = COALESCE(retry_count, 0) + 1
        WHERE chain_id = $2 AND block_hash = $3 AND key_id = $4
        "#,
        error_msg,
        activation.chain_id.as_i64(),
        activation.block_hash,
        activation.key_id,
    )
    .execute(tx.deref_mut())
    .await
    {
        error!(error = ?err, key_id = ?activation.key_id, "Failed to update key activation error");
    };
    // no need to bubble up as we already log the error when we catch it, and this is a best effort to update the error message and counter in the database
}

pub async fn mark_crs_activation_error(
    tx: &mut Transaction<'_, Postgres>,
    error_msg: &str,
    activation: PendingCrsActivation,
) {
    if let Err(err) = sqlx::query!(
        r#"
        UPDATE kms_crs_activation_events
        SET last_error = $1, last_updated_at = NOW(), retry_count = COALESCE(retry_count, 0) + 1
        WHERE chain_id = $2 AND block_hash = $3 AND crs_id = $4
        "#,
        error_msg,
        activation.chain_id.as_i64(),
        activation.block_hash,
        activation.crs_id,
    )
    .execute(tx.deref_mut())
    .await
    {
        error!(error = ?err, crs_id = ?activation.crs_id, "Failed to update CRS activation error");
    };
    // no need to bubble up as we already log the error when we catch it, and this is a best effort to update the error message and counter in the database
}
