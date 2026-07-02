use std::ops::DerefMut;

use alloy::rpc::types::Log;
use sqlx::{Postgres, Transaction};
use tracing::{error, info};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::db_keys::write_large_object_in_chunks_tx;

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
    use sqlx::Row;
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

// ----------------------------------------------------------------------------
// RFC-029 one-time compressed-key migration
// ----------------------------------------------------------------------------

#[derive(Debug, Clone)]
pub(crate) struct PendingCompressedKeyMaterial {
    pub chain_id: ChainId,
    pub block_hash: Vec<u8>,
    pub key_id: Vec<u8>,
    pub digest_server: Option<Vec<u8>>,
    pub storage_urls: Vec<String>,
}

pub(crate) async fn insert_compressed_key_material_event(
    tx: &mut Transaction<'_, Postgres>,
    event: KMSGeneration::CompressedKeyMaterialAdded,
    log: Log,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    let digest_server = event
        .keyDigests
        .iter()
        .filter(|d| d.keyType == 0)
        .map(|d| d.digest.to_vec())
        .next();
    sqlx::query!(
        r#"
        INSERT INTO compressed_key_material_events (
            chain_id, block_hash, block_number, transaction_hash,
            key_id, key_digest_server, storage_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (chain_id, block_hash, key_id) DO NOTHING
        "#,
        chain_id.as_i64(),
        block_hash,
        block_number as i64,
        log.transaction_hash.map(|txh| txh.to_vec()),
        &key_id_to_database_bytes(event.keyId),
        digest_server,
        &event.kmsNodeStorageUrls,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn insert_compressed_key_cutover_event(
    tx: &mut Transaction<'_, Postgres>,
    event: KMSGeneration::CompressedKeyCutoverScheduled,
    log: Log,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> Result<(), sqlx::Error> {
    let host_cutovers = serde_json::to_string(
        &event
            .hostChainCutovers
            .iter()
            .map(|c| {
                serde_json::json!({
                    "chain_id": c.chainId.to_string(),
                    "cutover_block": c.cutoverBlock,
                })
            })
            .collect::<Vec<_>>(),
    )
    .expect("static json shape");
    sqlx::query!(
        r#"
        INSERT INTO compressed_key_cutover_events (
            chain_id, block_hash, block_number, transaction_hash,
            key_id, gateway_cutover_block, host_cutovers
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT (chain_id, block_hash, key_id) DO NOTHING
        "#,
        chain_id.as_i64(),
        block_hash,
        block_number as i64,
        log.transaction_hash.map(|txh| txh.to_vec()),
        &key_id_to_database_bytes(event.keyId),
        event.gatewayCutoverBlock as i64,
        host_cutovers,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn cancel_orphaned_migration_events(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let materials = sqlx::query!(
        "
        UPDATE compressed_key_material_events AS e
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
    let cutovers = sqlx::query!(
        "
        UPDATE compressed_key_cutover_events AS e
        SET status = 'cancelled'
        FROM host_chain_blocks_valid AS b
        WHERE
            e.status = 'pending'
            AND e.chain_id = b.chain_id
            AND e.block_hash = b.block_hash
            AND b.block_status = 'orphaned'
        "
    )
    .execute(tx.deref_mut())
    .await?;
    let total = materials.rows_affected() + cutovers.rows_affected();
    if total > 0 {
        info!(
            "Marked {total} pending compressed-key migration events as cancelled due to orphaned blocks"
        );
    }
    Ok(total)
}

/// Promotes finalized cutover schedules into the canonical
/// `compressed_key_cutover` tables. The schedule is single-assignment:
/// a second, different schedule for the same key is a loud error
/// (never silently swallowed); an identical replay is a no-op.
pub(crate) async fn apply_finalized_cutover_schedules(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let to_apply = sqlx::query!(
        r#"
        SELECT e.chain_id, e.block_hash, e.key_id,
               e.gateway_cutover_block, e.host_cutovers
        FROM compressed_key_cutover_events AS e
        INNER JOIN host_chain_blocks_valid AS b
            ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash
        WHERE e.status = 'pending' AND b.block_status = 'finalized'
        FOR UPDATE OF e SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;

    let mut done = 0;
    for row in to_apply {
        let existing = sqlx::query!(
            "SELECT gateway_cutover_block FROM compressed_key_cutover WHERE key_id = $1",
            &row.key_id,
        )
        .fetch_optional(tx.deref_mut())
        .await?;

        if let Some(existing) = existing {
            if existing.gateway_cutover_block == row.gateway_cutover_block {
                info!(key_id = ?row.key_id, "Compressed-key cutover schedule already applied, ignoring replay");
            } else {
                // Should be unreachable: the contract enforces
                // single-assignment. Refuse to overwrite and scream.
                error!(
                    key_id = ?row.key_id,
                    stored = existing.gateway_cutover_block,
                    incoming = row.gateway_cutover_block,
                    "CONFLICTING compressed-key cutover schedule observed; keeping the stored schedule"
                );
                sqlx::query!(
                    "UPDATE compressed_key_cutover_events SET status = 'error',
                     last_error = 'conflicting schedule', last_updated_at = NOW()
                     WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3",
                    row.chain_id,
                    &row.block_hash,
                    &row.key_id,
                )
                .execute(tx.deref_mut())
                .await?;
                continue;
            }
        } else {
            sqlx::query!(
                "INSERT INTO compressed_key_cutover (key_id, gateway_cutover_block)
                 VALUES ($1, $2)",
                &row.key_id,
                row.gateway_cutover_block,
            )
            .execute(tx.deref_mut())
            .await?;
            let host_cutovers: Vec<serde_json::Value> =
                serde_json::from_str(&row.host_cutovers)?;
            for cutover in host_cutovers {
                let host_chain_id: i64 = cutover["chain_id"]
                    .as_str()
                    .ok_or_else(|| anyhow::anyhow!("missing chain_id"))?
                    .parse()?;
                let cutover_block = cutover["cutover_block"]
                    .as_i64()
                    .ok_or_else(|| anyhow::anyhow!("missing cutover_block"))?;
                sqlx::query!(
                    "INSERT INTO compressed_key_cutover_hosts (key_id, chain_id, cutover_block)
                     VALUES ($1, $2, $3)",
                    &row.key_id,
                    host_chain_id,
                    cutover_block,
                )
                .execute(tx.deref_mut())
                .await?;
            }
            info!(key_id = ?row.key_id, "Applied compressed-key cutover schedule");
        }

        sqlx::query!(
            "UPDATE compressed_key_cutover_events SET status = 'applied',
             last_updated_at = NOW()
             WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3",
            row.chain_id,
            &row.block_hash,
            &row.key_id,
        )
        .execute(tx.deref_mut())
        .await?;
        done += 1;

        // Wake workers so they observe the new selection policy.
        sqlx::query!("SELECT pg_notify('work_available', 'cutover_scheduled')")
            .execute(tx.deref_mut())
            .await?;
    }
    Ok(done)
}

/// Copies verified, finalized compressed material into
/// `keys.compressed_xof_keyset` — but only for keys whose cutover
/// schedule is already applied. Until then the bytes stay staged in
/// the event row: filling the keys column earlier would flip the
/// default (COALESCE) read path on local ingestion timing.
pub(crate) async fn apply_finalized_compressed_key_materials(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<u64> {
    let query = sqlx::query!(
        r#"
        WITH applicable AS (
            SELECT e.chain_id, e.block_hash, e.key_id,
                   e.key_content_compressed_xof_keyset AS blob
            FROM compressed_key_material_events AS e
            INNER JOIN host_chain_blocks_valid AS b
                ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash
            INNER JOIN compressed_key_cutover AS c
                ON c.key_id = e.key_id
            WHERE e.status = 'ready'
                AND b.block_status = 'finalized'
                AND e.key_content_compressed_xof_keyset IS NOT NULL
            FOR UPDATE OF e SKIP LOCKED
        ),
        updated_keys AS (
            UPDATE keys
            SET compressed_xof_keyset = applicable.blob
            FROM applicable
            WHERE keys.key_id_gw = applicable.key_id
            RETURNING keys.key_id_gw
        )
        UPDATE compressed_key_material_events AS e
        SET status = 'applied', last_updated_at = NOW()
        FROM applicable
        WHERE e.chain_id = applicable.chain_id
            AND e.block_hash = applicable.block_hash
            AND e.key_id = applicable.key_id
        "#
    )
    .execute(tx.deref_mut())
    .await?;
    if query.rows_affected() > 0 {
        info!(
            "Applied {} compressed-key materials to the keys table",
            query.rows_affected()
        );
        sqlx::query!(
            "SELECT pg_notify('work_available', 'compressed_material_applied')"
        )
        .execute(tx.deref_mut())
        .await?;
    }
    Ok(query.rows_affected())
}

pub(crate) async fn all_pending_compressed_key_materials_to_download(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<PendingCompressedKeyMaterial>> {
    let rows = sqlx::query!(
        r#"
        SELECT chain_id, block_hash, key_id, key_digest_server, storage_urls
        FROM compressed_key_material_events
        WHERE status = 'pending'
        FOR UPDATE SKIP LOCKED
        "#
    )
    .fetch_all(tx.deref_mut())
    .await?;
    rows.into_iter()
        .map(|row| {
            Ok(PendingCompressedKeyMaterial {
                chain_id: ChainId::try_from(row.chain_id)?,
                block_hash: row.block_hash,
                key_id: row.key_id,
                digest_server: row.key_digest_server,
                storage_urls: row.storage_urls,
            })
        })
        .collect()
}

pub(crate) async fn set_ready_compressed_key_material(
    tx: &mut Transaction<'_, Postgres>,
    pending: &PendingCompressedKeyMaterial,
    blob: Vec<u8>,
) -> anyhow::Result<()> {
    sqlx::query!(
        r#"
        UPDATE compressed_key_material_events
        SET status = 'ready', key_content_compressed_xof_keyset = $4,
            last_updated_at = NOW()
        WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3
        "#,
        pending.chain_id.as_i64(),
        &pending.block_hash,
        &pending.key_id,
        blob,
    )
    .execute(tx.deref_mut())
    .await?;
    Ok(())
}

pub(crate) async fn mark_compressed_key_material_error(
    tx: &mut Transaction<'_, Postgres>,
    error_message: &str,
    pending: PendingCompressedKeyMaterial,
) {
    let result = sqlx::query!(
        r#"
        UPDATE compressed_key_material_events
        SET status = 'pending', retry_count = retry_count + 1,
            last_error = $4, last_updated_at = NOW()
        WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3
        "#,
        pending.chain_id.as_i64(),
        &pending.block_hash,
        &pending.key_id,
        error_message,
    )
    .execute(tx.deref_mut())
    .await;
    if let Err(err) = result {
        error!(error = %err, "Failed to mark compressed key material error");
    }
}

pub(crate) async fn count_compressed_key_material_remaining_pending(
    db_pool: &sqlx::Pool<Postgres>,
) -> anyhow::Result<u64> {
    let row = sqlx::query_scalar!(
        "SELECT COUNT(*) FROM compressed_key_material_events WHERE status = 'pending'",
    )
    .fetch_one(db_pool)
    .await?;
    Ok(row.unwrap_or(0) as u64)
}
