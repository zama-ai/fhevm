use std::ops::DerefMut;

use serde::{Deserialize, Serialize};
use sqlx::types::Json;
use sqlx::{PgPool, Postgres, Row, Transaction};
use tokio_util::bytes::Bytes;
use tracing::info;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::db_keys::{write_large_object_in_chunks_tx, DbKeyId};

const CHUNK_SIZE: usize = 128 * 1024 * 1024; // 128MB
const RETRY_DELAY_SQL: &str = "10 seconds";

#[derive(Debug, Default)]
pub(crate) struct KeyRecord {
    pub key_id_gw: DbKeyId,
    pub pks_key: Bytes,
    pub sks_key: Bytes,
    pub sns_pk: Bytes,
}

impl KeyRecord {
    pub fn is_valid(&self) -> bool {
        !self.key_id_gw.is_empty()
            && !self.pks_key.is_empty()
            && !self.sks_key.is_empty()
            && !self.sns_pk.is_empty()
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub(crate) struct StoredKeyDigest {
    pub key_type: u8,
    pub digest: Vec<u8>,
}

#[derive(Debug, Clone)]
pub(crate) struct PreparedKeyActivation {
    pub key_id_gw: [u8; 32],
    pub key_digests: Vec<StoredKeyDigest>,
    pub s3_bucket_urls: Vec<String>,
    pub transaction_hash: Vec<u8>,
    pub log_index: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct PreparedCrsActivation {
    pub crs_id: [u8; 32],
    pub crs_digest: Vec<u8>,
    pub s3_bucket_urls: Vec<String>,
    pub transaction_hash: Vec<u8>,
    pub log_index: i64,
}

#[derive(Debug, Clone)]
pub(crate) struct StagedKeyActivation {
    pub sequence_number: i64,
    pub chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub key_id_gw: Vec<u8>,
    pub key_digests: Vec<StoredKeyDigest>,
    pub s3_bucket_urls: Vec<String>,
}

#[derive(Debug, Clone)]
pub(crate) struct StagedCrsActivation {
    pub sequence_number: i64,
    pub chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub crs_id: Vec<u8>,
    pub crs_digest: Vec<u8>,
    pub s3_bucket_urls: Vec<String>,
}

pub(crate) async fn insert_key_activation_event_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &PreparedKeyActivation,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            log_index,
            key_id_gw,
            key_digests,
            s3_bucket_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (chain_id, block_hash, transaction_hash, log_index)
        DO NOTHING
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(block_hash)
    .bind(block_number as i64)
    .bind(&activation.transaction_hash)
    .bind(activation.log_index)
    .bind(&activation.key_id_gw[..])
    .bind(Json(&activation.key_digests))
    .bind(&activation.s3_bucket_urls)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn insert_crs_activation_event_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &PreparedCrsActivation,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        INSERT INTO kms_crs_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            log_index,
            crs_id,
            crs_digest,
            s3_bucket_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT (chain_id, block_hash, transaction_hash, log_index)
        DO NOTHING
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(block_hash)
    .bind(block_number as i64)
    .bind(&activation.transaction_hash)
    .bind(activation.log_index)
    .bind(&activation.crs_id[..])
    .bind(&activation.crs_digest)
    .bind(&activation.s3_bucket_urls)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn list_finalized_pending_key_activations(
    pool: &PgPool,
    chain_id: ChainId,
    limit: i64,
) -> anyhow::Result<Vec<StagedKeyActivation>> {
    let rows = sqlx::query(&format!(
        r#"
        SELECT
            e.sequence_number,
            e.block_hash,
            e.key_id_gw,
            e.key_digests,
            e.s3_bucket_urls
        FROM kms_key_activation_events e
        JOIN host_chain_blocks_valid b
          ON b.chain_id = e.chain_id AND b.block_hash = e.block_hash
        WHERE e.chain_id = $1
          AND (
                e.download_status = 'pending'
                OR (
                    e.download_status = 'failed'
                    AND (
                        e.last_attempt_at IS NULL
                        OR e.last_attempt_at <= NOW() - INTERVAL '{RETRY_DELAY_SQL}'
                    )
                )
              )
          AND b.block_status = 'finalized'
        ORDER BY e.sequence_number
        LIMIT $2
        "#
    ))
    .bind(chain_id.as_i64())
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            let Json(key_digests) = row.try_get("key_digests")?;
            Ok(StagedKeyActivation {
                sequence_number: row.try_get("sequence_number")?,
                chain_id,
                block_hash: row.try_get("block_hash")?,
                key_id_gw: row.try_get("key_id_gw")?,
                key_digests,
                s3_bucket_urls: row.try_get("s3_bucket_urls")?,
            })
        })
        .collect()
}

pub(crate) async fn list_finalized_pending_crs_activations(
    pool: &PgPool,
    chain_id: ChainId,
    limit: i64,
) -> anyhow::Result<Vec<StagedCrsActivation>> {
    let rows = sqlx::query(&format!(
        r#"
        SELECT
            e.sequence_number,
            e.block_hash,
            e.crs_id,
            e.crs_digest,
            e.s3_bucket_urls
        FROM kms_crs_activation_events e
        JOIN host_chain_blocks_valid b
          ON b.chain_id = e.chain_id AND b.block_hash = e.block_hash
        WHERE e.chain_id = $1
          AND (
                e.download_status = 'pending'
                OR (
                    e.download_status = 'failed'
                    AND (
                        e.last_attempt_at IS NULL
                        OR e.last_attempt_at <= NOW() - INTERVAL '{RETRY_DELAY_SQL}'
                    )
                )
              )
          AND b.block_status = 'finalized'
        ORDER BY e.sequence_number
        LIMIT $2
        "#
    ))
    .bind(chain_id.as_i64())
    .bind(limit)
    .fetch_all(pool)
    .await?;

    rows.into_iter()
        .map(|row| {
            Ok(StagedCrsActivation {
                sequence_number: row.try_get("sequence_number")?,
                chain_id,
                block_hash: row.try_get("block_hash")?,
                crs_id: row.try_get("crs_id")?,
                crs_digest: row.try_get("crs_digest")?,
                s3_bucket_urls: row.try_get("s3_bucket_urls")?,
            })
        })
        .collect()
}

pub(crate) async fn mark_key_activation_failed(
    pool: &PgPool,
    activation: &StagedKeyActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET download_status = 'failed',
            retry_count = retry_count + 1,
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn mark_crs_activation_failed(
    pool: &PgPool,
    activation: &StagedCrsActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_crs_activation_events
        SET download_status = 'failed',
            retry_count = retry_count + 1,
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn mark_key_activation_digest_mismatch(
    pool: &PgPool,
    activation: &StagedKeyActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET download_status = 'digest_mismatch',
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn mark_crs_activation_digest_mismatch(
    pool: &PgPool,
    activation: &StagedCrsActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_crs_activation_events
        SET download_status = 'digest_mismatch',
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn mark_key_activation_invalid_event(
    pool: &PgPool,
    activation: &StagedKeyActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET download_status = 'invalid_event',
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn mark_crs_activation_invalid_event(
    pool: &PgPool,
    activation: &StagedCrsActivation,
    error: &str,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_crs_activation_events
        SET download_status = 'invalid_event',
            last_error = $2,
            last_attempt_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .bind(error)
    .execute(pool)
    .await?;
    Ok(())
}

pub(crate) async fn materialize_key_activation_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedKeyActivation,
    key_record: &KeyRecord,
) -> anyhow::Result<()> {
    match lock_key_activation_state_tx(tx, activation.sequence_number).await? {
        ActivationMaterializationState::Ready => {}
        ActivationMaterializationState::Orphaned => {
            mark_key_activation_orphaned_tx(tx, activation).await?;
            return Ok(());
        }
        ActivationMaterializationState::Skip => return Ok(()),
    }

    if has_active_key_tx(tx, &activation.key_id_gw).await? {
        mark_key_activation_materialized_tx(tx, activation).await?;
        return Ok(());
    }

    let inserted_sequence_number: Option<i64> = sqlx::query_scalar(
        r#"
        INSERT INTO keys (
            key_id,
            key_id_gw,
            pks_key,
            sks_key,
            sns_pk,
            status,
            chain_id,
            block_hash
        )
        VALUES ('', $1, $2, $3, NULL, 'active', $4, $5)
        ON CONFLICT DO NOTHING
        RETURNING sequence_number
        "#,
    )
    .bind(&key_record.key_id_gw)
    .bind(key_record.pks_key.as_ref())
    .bind(key_record.sks_key.as_ref())
    .bind(activation.chain_id.as_i64())
    .bind(&activation.block_hash)
    .fetch_optional(tx.deref_mut())
    .await?;

    let Some(sequence_number) = inserted_sequence_number else {
        if has_active_key_tx(tx, &activation.key_id_gw).await? {
            mark_key_activation_materialized_tx(tx, activation).await?;
            return Ok(());
        }
        anyhow::bail!(
            "ActivateKey insert did not create an active key row for sequence {}",
            activation.sequence_number
        );
    };

    let oid =
        write_large_object_in_chunks_tx(tx, &key_record.sns_pk, CHUNK_SIZE)
            .await?;
    sqlx::query(
        r#"
        UPDATE keys
        SET sns_pk = $2
        WHERE sequence_number = $1
        "#,
    )
    .bind(sequence_number)
    .bind(oid)
    .execute(tx.deref_mut())
    .await?;

    mark_key_activation_materialized_tx(tx, activation).await?;
    Ok(())
}

pub(crate) async fn materialize_crs_activation_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedCrsActivation,
    crs: &[u8],
) -> anyhow::Result<()> {
    match lock_crs_activation_state_tx(tx, activation.sequence_number).await? {
        ActivationMaterializationState::Ready => {}
        ActivationMaterializationState::Orphaned => {
            mark_crs_activation_orphaned_tx(tx, activation).await?;
            return Ok(());
        }
        ActivationMaterializationState::Skip => return Ok(()),
    }

    if has_active_crs_tx(tx, &activation.crs_id).await? {
        mark_crs_activation_materialized_tx(tx, activation).await?;
        return Ok(());
    }

    info!(id = ?activation.crs_id, "Inserting crs");
    let rows_affected = sqlx::query(
        r#"
        INSERT INTO crs (crs_id, crs, status, chain_id, block_hash)
        VALUES ($1, $2, 'active', $3, $4)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(&activation.crs_id)
    .bind(crs)
    .bind(activation.chain_id.as_i64())
    .bind(&activation.block_hash)
    .execute(tx.deref_mut())
    .await?
    .rows_affected();

    if rows_affected == 0 {
        if has_active_crs_tx(tx, &activation.crs_id).await? {
            mark_crs_activation_materialized_tx(tx, activation).await?;
            return Ok(());
        }
        anyhow::bail!(
            "ActivateCrs insert did not create an active CRS row for sequence {}",
            activation.sequence_number
        );
    }

    mark_crs_activation_materialized_tx(tx, activation).await?;
    Ok(())
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
enum ActivationMaterializationState {
    Ready,
    Orphaned,
    Skip,
}

async fn lock_key_activation_state_tx(
    tx: &mut Transaction<'_, Postgres>,
    sequence_number: i64,
) -> Result<ActivationMaterializationState, sqlx::Error> {
    lock_activation_state_tx(tx, "kms_key_activation_events", sequence_number)
        .await
}

async fn lock_crs_activation_state_tx(
    tx: &mut Transaction<'_, Postgres>,
    sequence_number: i64,
) -> Result<ActivationMaterializationState, sqlx::Error> {
    lock_activation_state_tx(tx, "kms_crs_activation_events", sequence_number)
        .await
}

async fn lock_activation_state_tx(
    tx: &mut Transaction<'_, Postgres>,
    table_name: &str,
    sequence_number: i64,
) -> Result<ActivationMaterializationState, sqlx::Error> {
    let row = sqlx::query(&format!(
        r#"
        SELECT e.download_status, b.block_status
        FROM {table_name} e
        JOIN host_chain_blocks_valid b
          ON b.chain_id = e.chain_id AND b.block_hash = e.block_hash
        WHERE e.sequence_number = $1
        FOR UPDATE OF e, b
        "#
    ))
    .bind(sequence_number)
    .fetch_optional(tx.deref_mut())
    .await?;

    let Some(row) = row else {
        return Ok(ActivationMaterializationState::Skip);
    };

    let download_status: String = row.try_get("download_status")?;
    let block_status: String = row.try_get("block_status")?;

    if !matches!(download_status.as_str(), "pending" | "failed") {
        return Ok(ActivationMaterializationState::Skip);
    }

    if block_status == "finalized" {
        Ok(ActivationMaterializationState::Ready)
    } else if block_status == "orphaned" {
        Ok(ActivationMaterializationState::Orphaned)
    } else {
        Ok(ActivationMaterializationState::Skip)
    }
}

async fn has_active_key_tx(
    tx: &mut Transaction<'_, Postgres>,
    key_id_gw: &[u8],
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM keys
            WHERE key_id_gw = $1 AND status = 'active'
        )
        "#,
    )
    .bind(key_id_gw)
    .fetch_one(tx.deref_mut())
    .await
}

async fn has_active_crs_tx(
    tx: &mut Transaction<'_, Postgres>,
    crs_id: &[u8],
) -> Result<bool, sqlx::Error> {
    sqlx::query_scalar(
        r#"
        SELECT EXISTS(
            SELECT 1
            FROM crs
            WHERE crs_id = $1 AND status = 'active'
        )
        "#,
    )
    .bind(crs_id)
    .fetch_one(tx.deref_mut())
    .await
}

async fn mark_key_activation_materialized_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedKeyActivation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET download_status = 'materialized',
            last_error = NULL,
            last_attempt_at = NOW(),
            materialized_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

async fn mark_key_activation_orphaned_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedKeyActivation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET download_status = 'orphaned',
            last_error = NULL
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

async fn mark_crs_activation_materialized_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedCrsActivation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_crs_activation_events
        SET download_status = 'materialized',
            last_error = NULL,
            last_attempt_at = NOW(),
            materialized_at = NOW()
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

async fn mark_crs_activation_orphaned_tx(
    tx: &mut Transaction<'_, Postgres>,
    activation: &StagedCrsActivation,
) -> Result<(), sqlx::Error> {
    sqlx::query(
        r#"
        UPDATE kms_crs_activation_events
        SET download_status = 'orphaned',
            last_error = NULL
        WHERE sequence_number = $1
        "#,
    )
    .bind(activation.sequence_number)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}
