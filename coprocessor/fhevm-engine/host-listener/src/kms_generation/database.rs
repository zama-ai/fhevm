use std::ops::DerefMut;

use alloy::rpc::types::Log;
use sqlx::{Postgres, Row, Transaction};
use tracing::{error, info, warn};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::db_keys::write_large_object_in_chunks_tx;
use fhevm_engine_common::material_version::{
    MaterialMigrationStatus, MaterialVersion, MIGRATION_SCHEDULE_CHANNEL,
};

use crate::contracts::KMSGeneration;
use crate::kms_generation::key_id_to_database_bytes;
use crate::kms_generation::sks_key::PreparedServerKey;

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

/// RFC-029: remember a `KeyMaterialMigrationScheduled` event until its block is finalized.
pub(crate) async fn insert_key_material_migration_scheduled(
    tx: &mut Transaction<'_, Postgres>,
    scheduled: KMSGeneration::KeyMaterialMigrationScheduled,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    // RFC-029 is a one-time cutover to v1 (the contract guards this too); ignore
    // any other target rather than keeping a schedule the selectors can't honor.
    let material_version = scheduled.materialVersion.to::<u64>() as i16;
    if material_version != MaterialVersion::MIGRATED_V1.0 {
        warn!(
            material_version,
            "ignoring KeyMaterialMigrationScheduled with non-v1 target"
        );
        return Ok(());
    }

    let key_id = key_id_to_database_bytes(scheduled.keyId);
    let host_chain_ids: Vec<i64> = scheduled
        .hostChainIds
        .iter()
        .map(|c| c.to::<u64>() as i64)
        .collect();
    let host_target_blocks: Vec<i64> = scheduled
        .hostMigrationBlocks
        .iter()
        .map(|b| b.to::<u64>() as i64)
        .collect();
    let gateway_block = scheduled.gatewayMigrationBlock.to::<u64>() as i64;

    sqlx::query(
        "INSERT INTO kms_key_material_schedule_events \
            (chain_id, block_hash, block_number, key_id, host_chain_ids, host_target_blocks, gateway_target_block) \
         VALUES ($1, $2, $3, $4, $5, $6, $7) \
         ON CONFLICT (chain_id, block_hash, key_id) DO NOTHING",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash)
    .bind(block_number as i64)
    .bind(key_id.to_vec())
    .bind(&host_chain_ids)
    .bind(&host_target_blocks)
    .bind(gateway_block)
    .execute(tx.deref_mut())
    .await?;

    info!(
        gateway_block,
        chains = host_chain_ids.len(),
        block_number,
        "RFC-029 migration schedule recorded (applies on finalization)"
    );
    Ok(())
}

/// RFC-029: apply schedules whose event block is finalized.
pub(crate) async fn apply_finalized_migration_schedules(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<u64, sqlx::Error> {
    let ready = sqlx::query(
        "SELECT e.chain_id, e.block_hash, e.key_id, e.host_chain_ids, e.host_target_blocks, \
                e.gateway_target_block \
         FROM kms_key_material_schedule_events AS e \
         INNER JOIN host_chain_blocks_valid AS b \
            ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash \
         WHERE e.status = 'pending' AND b.block_status = 'finalized' \
         FOR UPDATE OF e SKIP LOCKED",
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut applied = 0u64;
    for row in ready {
        let event_chain_id: i64 = row.try_get("chain_id")?;
        let event_block_hash: Vec<u8> = row.try_get("block_hash")?;
        let key_id: Vec<u8> = row.try_get("key_id")?;
        let host_chain_ids: Vec<i64> = row.try_get("host_chain_ids")?;
        let host_target_blocks: Vec<i64> = row.try_get("host_target_blocks")?;
        let gateway_block: i64 = row.try_get("gateway_target_block")?;

        // RFC-029 is a one-time cutover and KMSGeneration enforces single-assignment of the schedule,
        // so the first finalized schedule wins: DO NOTHING (never rewrite cutover blocks the fleet may
        // already have crossed).
        sqlx::query(
            "INSERT INTO material_version_host_schedule (host_chain_id, target_block) \
             SELECT * FROM unnest($1::bigint[], $2::bigint[]) \
             ON CONFLICT (host_chain_id) DO NOTHING",
        )
        .bind(&host_chain_ids)
        .bind(&host_target_blocks)
        .execute(tx.deref_mut())
        .await?;

        sqlx::query(
            "INSERT INTO material_version_gateway_schedule (singleton, target_block) \
             VALUES (TRUE, $1) \
             ON CONFLICT (singleton) DO NOTHING",
        )
        .bind(gateway_block)
        .execute(tx.deref_mut())
        .await?;

        sqlx::query(
            "UPDATE keys \
             SET material_migration_status = $2 \
             WHERE key_id = $1",
        )
        .bind(&key_id)
        .bind(MaterialMigrationStatus::SCHEDULED)
        .execute(tx.deref_mut())
        .await?;

        sqlx::query(
            "UPDATE kms_key_material_schedule_events \
             SET status = 'applied', last_updated_at = NOW() \
             WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3",
        )
        .bind(event_chain_id)
        .bind(&event_block_hash)
        .bind(&key_id)
        .execute(tx.deref_mut())
        .await?;
        applied += 1;
    }

    if applied > 0 {
        // Wake the workers' notify-driven schedule reload.
        sqlx::query("SELECT pg_notify($1, '')")
            .bind(MIGRATION_SCHEDULE_CHANNEL)
            .execute(tx.deref_mut())
            .await?;
        info!(
            applied,
            "RFC-029 migration schedule(s) applied + workers notified"
        );
    }
    Ok(applied)
}

/// RFC-029: cancel pending cutover schedules whose scheduling block was orphaned,
/// mirroring [`cancel_orphaned_key_activations`].
pub(crate) async fn cancel_orphaned_migration_schedules(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query(
        "UPDATE kms_key_material_schedule_events AS e \
         SET status = 'cancelled', last_updated_at = NOW() \
         FROM host_chain_blocks_valid AS b \
         WHERE e.status = 'pending' \
           AND e.chain_id = b.chain_id AND e.block_hash = b.block_hash \
           AND b.block_status = 'orphaned'",
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() > 0 {
        info!(
            cancelled = query.rows_affected(),
            "RFC-029 cancelled pending migration schedule(s) on orphaned blocks"
        );
    }
    Ok(query.rows_affected())
}

pub(crate) async fn cancel_orphaned_key_material(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query(
        "UPDATE kms_key_material_events AS e \
         SET status = 'cancelled', last_updated_at = NOW() \
         FROM host_chain_blocks_valid AS b \
         WHERE e.status = 'pending' \
           AND e.chain_id = b.chain_id AND e.block_hash = b.block_hash \
           AND b.block_status = 'orphaned'",
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() > 0 {
        info!(
            cancelled = query.rows_affected(),
            "RFC-029 cancelled key-material event(s) on orphaned blocks"
        );
    }
    Ok(query.rows_affected())
}

/// RFC-029: a finalized `KeyMaterialAdded` event waiting for S3 download.
#[derive(Debug, Clone)]
pub(crate) struct PendingKeyMaterial {
    pub chain_id: i64,
    pub block_hash: Vec<u8>,
    pub key_id: Vec<u8>,
    pub key_digest: Option<Vec<u8>>,
    pub storage_urls: Vec<String>,
}

/// Remember a `KeyMaterialAdded` event until its block is finalized.
pub(crate) async fn insert_key_material_added(
    tx: &mut Transaction<'_, Postgres>,
    added: KMSGeneration::KeyMaterialAdded,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    // The migrated server-key (XOF keyset) digest is the keyType==0 entry.
    let key_digest = added
        .keyDigests
        .iter()
        .find(|d| d.keyType == 0)
        .map(|d| d.digest.to_vec());
    let material_version = added.materialVersion.to::<u64>() as i16;
    if material_version != MaterialVersion::MIGRATED_V1.0 {
        warn!(
            material_version,
            "ignoring KeyMaterialAdded with non-v1 material"
        );
        return Ok(());
    }
    let urls = added.kmsNodeStorageUrls.clone();
    sqlx::query(
        "INSERT INTO kms_key_material_events (\
            chain_id, block_hash, block_number, \
            key_id, key_digest, storage_urls) \
         VALUES ($1, $2, $3, $4, $5, $6) \
         ON CONFLICT (chain_id, block_hash, key_id) DO NOTHING",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash)
    .bind(block_number as i64)
    .bind(key_id_to_database_bytes(added.keyId).to_vec())
    .bind(key_digest)
    .bind(&urls)
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

/// Finalized v1 key-material rows awaiting an S3 download.
pub(crate) async fn all_pending_key_material_to_download(
    tx: &mut Transaction<'_, Postgres>,
) -> Result<Vec<PendingKeyMaterial>, sqlx::Error> {
    let rows = sqlx::query(
        "SELECT e.chain_id, e.block_hash, e.key_id, e.key_digest, e.storage_urls \
         FROM kms_key_material_events AS e \
         INNER JOIN host_chain_blocks_valid AS b \
            ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash \
         WHERE e.status = 'pending' AND b.block_status = 'finalized' \
         FOR UPDATE OF e SKIP LOCKED",
    )
    .fetch_all(tx.deref_mut())
    .await?;
    Ok(rows
        .into_iter()
        .map(|r| PendingKeyMaterial {
            chain_id: r.get("chain_id"),
            block_hash: r.get("block_hash"),
            key_id: r.get("key_id"),
            key_digest: r.get("key_digest"),
            storage_urls: r.get("storage_urls"),
        })
        .collect())
}

/// Store downloaded migrated material in the single compressed-XOF column.
pub(crate) async fn publish_key_material(
    tx: &mut Transaction<'_, Postgres>,
    pending: &PendingKeyMaterial,
    bytes: &[u8],
) -> Result<u64, sqlx::Error> {
    let query = sqlx::query(
        "WITH upd AS (\
            UPDATE keys \
            SET compressed_xof_keyset = $4, \
                material_migration_status = COALESCE(material_migration_status, $5) \
            WHERE key_id = $3 \
            RETURNING key_id\
        ) \
        UPDATE kms_key_material_events AS e \
        SET status = 'published', last_updated_at = NOW() \
        FROM upd \
        WHERE e.chain_id = $1 AND e.block_hash = $2 AND e.key_id = upd.key_id",
    )
    .bind(pending.chain_id)
    .bind(&pending.block_hash)
    .bind(&pending.key_id)
    .bind(bytes)
    .bind(MaterialMigrationStatus::MATERIAL_READY)
    .execute(tx.deref_mut())
    .await?;
    Ok(query.rows_affected())
}

pub(crate) async fn mark_key_material_error(
    tx: &mut Transaction<'_, Postgres>,
    error: &str,
    pending: PendingKeyMaterial,
) {
    // Mirror mark_key_activation_error: leave status = 'pending' so the download
    // loop retries next cycle. A transient S3 failure must not strand the v1
    // material -- that would halt every post-cutover worker indefinitely.
    warn!(
        error,
        "RFC-029 v1 key-material download failed; staying pending for retry"
    );
    let _ = sqlx::query(
        "UPDATE kms_key_material_events \
         SET last_updated_at = NOW() \
         WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3",
    )
    .bind(pending.chain_id)
    .bind(&pending.block_hash)
    .bind(&pending.key_id)
    .execute(tx.deref_mut())
    .await;
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
                pks_key, sks_key, sns_pk,
                compressed_xof_keyset
            )
            SELECT
                e.chain_id, e.block_hash, e.key_id, e.key_id,
                e.key_content_public, e.key_content_sks_key, e.key_content_sns_pk,
                e.key_content_compressed_xof_keyset
            FROM kms_key_activation_events AS e
            WHERE
                e.chain_id = $1
                AND e.block_hash = $2
                AND e.key_id = $3
                -- Legacy decompressed columns are populated by both the
                -- XOF and legacy ingest paths, so they are the
                -- always-available gate.
                AND e.key_content_public IS NOT NULL
                AND e.key_content_sks_key IS NOT NULL
            ON CONFLICT (chain_id, block_hash, key_id_gw) DO UPDATE
            SET pks_key               = EXCLUDED.pks_key,
                sks_key               = EXCLUDED.sks_key,
                sns_pk                = COALESCE(EXCLUDED.sns_pk, keys.sns_pk),
                -- compressed_xof_keyset must move in lockstep with the
                -- legacy decompressed pair: a format rollback
                -- (XOF -> ServerKey) on a replayed activation
                -- would otherwise leave the decompressed blob updated
                -- but the compressed blob pointing at stale bytes.
                compressed_xof_keyset = EXCLUDED.compressed_xof_keyset,
                key_id_gw             = EXCLUDED.key_id_gw
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

pub(crate) async fn set_ready_key_activation(
    tx: &mut Transaction<'_, Postgres>,
    activation: &PendingKeyActivation,
    server_key: Option<PreparedServerKey>,
    public_key: Option<Vec<u8>>,
) -> anyhow::Result<()> {
    let (sns_pk, sks_key, compressed_xof_keyset) =
        if let Some(prepared) = server_key {
            (
                Some(prepared.sns_pk),
                Some(prepared.sks_key),
                prepared.compressed_xof_keyset,
            )
        } else {
            (None, None, None)
        };
    let server_key_updated = sks_key.is_some();
    let sns_pk_oid = if let Some(sns_pk) = sns_pk {
        Some(write_large_object_in_chunks_tx(tx, &sns_pk, CHUNK_SIZE).await?)
    } else {
        None
    };
    let query = sqlx::query!(
        r#"
        UPDATE kms_key_activation_events
        SET
            status = CASE
                WHEN COALESCE($2, key_content_sks_key) IS NOT NULL
                     AND COALESCE($3, key_content_public) IS NOT NULL
                THEN 'ready'
                ELSE status
            END,
            key_content_sns_pk = COALESCE($1, key_content_sns_pk),
            key_content_sks_key = COALESCE($2, key_content_sks_key),
            key_content_public = COALESCE($3, key_content_public),
            key_content_compressed_xof_keyset = CASE
                WHEN $4 THEN $5
                ELSE key_content_compressed_xof_keyset
            END,
            last_updated_at = NOW()
        WHERE chain_id = $6 AND block_hash = $7 AND key_id = $8
        "#,
        sns_pk_oid,
        sks_key,
        public_key,
        server_key_updated,
        compressed_xof_keyset,
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

#[cfg(test)]
mod tests {
    use super::*;
    use fhevm_engine_common::chain_id::ChainId;
    use test_harness::instance::{setup_test_db, ImportMode};

    #[tokio::test]
    async fn set_ready_key_activation_preserves_existing_server_content_until_public_arrives(
    ) -> anyhow::Result<()> {
        let db = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let pool = sqlx::PgPool::connect(db.db_url()).await?;

        let chain_id = ChainId::try_from(12345_u64)?;
        let block_hash = vec![1_u8; 32];
        let key_id = vec![2_u8; 32];
        let existing_sks = b"existing-sks".to_vec();
        let public_key = b"public-key".to_vec();
        let storage_urls: Vec<String> = Vec::new();

        sqlx::query(
            "INSERT INTO kms_key_activation_events (
                chain_id,
                block_hash,
                block_number,
                transaction_hash,
                key_id,
                key_content_sks_key,
                key_digest_server,
                key_digest_public,
                storage_urls
            )
            VALUES ($1, $2, 1, $3, $4, $5, $6, $7, $8)",
        )
        .bind(chain_id.as_i64())
        .bind(&block_hash)
        .bind(vec![3_u8; 32])
        .bind(&key_id)
        .bind(&existing_sks)
        .bind(vec![4_u8; 32])
        .bind(vec![5_u8; 32])
        .bind(&storage_urls)
        .execute(&pool)
        .await?;

        let activation = PendingKeyActivation {
            chain_id,
            block_hash: block_hash.clone(),
            key_id: key_id.clone(),
            digest_server: Some(vec![4_u8; 32]),
            digest_public: Some(vec![5_u8; 32]),
            has_server_key: true,
            has_public_key: false,
            storage_urls,
        };

        let mut tx = pool.begin().await?;
        set_ready_key_activation(
            &mut tx,
            &activation,
            None,
            Some(public_key.clone()),
        )
        .await?;
        tx.commit().await?;

        let row = sqlx::query(
            "SELECT status, key_content_sks_key, key_content_public
             FROM kms_key_activation_events
             WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3",
        )
        .bind(chain_id.as_i64())
        .bind(&block_hash)
        .bind(&key_id)
        .fetch_one(&pool)
        .await?;

        let status: String = row.try_get("status")?;
        let sks_key: Vec<u8> = row.try_get("key_content_sks_key")?;
        let stored_public_key: Vec<u8> = row.try_get("key_content_public")?;

        assert_eq!(status, "ready");
        assert_eq!(sks_key, existing_sks);
        assert_eq!(stored_public_key, public_key);

        Ok(())
    }
}
