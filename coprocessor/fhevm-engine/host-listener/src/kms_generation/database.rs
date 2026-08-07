use std::ops::DerefMut;

use alloy::rpc::types::Log;
use sqlx::{Pool, Postgres, Transaction};
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

#[derive(sqlx::FromRow)]
struct PendingKeyActivationRow {
    chain_id: i64,
    block_hash: Vec<u8>,
    key_id: Vec<u8>,
    key_digest_server: Option<Vec<u8>>,
    key_digest_public: Option<Vec<u8>>,
    has_server_key: Option<bool>,
    has_public_key: Option<bool>,
    storage_urls: Vec<String>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
pub(crate) struct PendingCompressedKeyMaterial {
    pub chain_id: i64,
    pub block_hash: Vec<u8>,
    pub block_number: i64,
    pub transaction_hash: Option<Vec<u8>>,
    pub key_id: Vec<u8>,
    pub key_material_id: Vec<u8>,
    pub key_digest: Vec<u8>,
    pub storage_urls: Vec<String>,
}

#[derive(Debug, sqlx::FromRow)]
pub(crate) struct AppliedCompressedKeyMaterial {
    pub chain_id: i64,
    pub block_number: i64,
    pub key_id: Vec<u8>,
    pub key_digest: Vec<u8>,
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
        .filter(|d| d.keyType == 0 || d.keyType == 3)
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

pub(crate) async fn insert_compressed_key_material_event(
    tx: &mut Transaction<'_, Postgres>,
    material: KMSGeneration::CompressedKeyMaterialAdded,
    log: Log,
    chain_id: ChainId,
    block_hash: &[u8],
    block_number: u64,
) -> anyhow::Result<()> {
    let digest = material
        .keyDigests
        .iter()
        .find(|digest| digest.keyType == 3)
        .ok_or_else(|| {
            anyhow::anyhow!(
                "compressed key material event has no compressed-XOF digest"
            )
        })?;
    sqlx::query(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id, block_hash, block_number, transaction_hash,
            key_id, key_material_id, key_digest_server, storage_urls,
            key_content_compressed_xof_keyset
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, ''::BYTEA)
        ON CONFLICT (chain_id, block_hash, key_id) DO NOTHING
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(block_hash)
    .bind(block_number as i64)
    .bind(log.transaction_hash.map(|hash| hash.to_vec()))
    .bind(key_id_to_database_bytes(material.keyId))
    .bind(key_id_to_database_bytes(material.keyMaterialId))
    .bind(digest.digest.as_ref())
    .bind(material.kmsNodeStorageUrls)
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

pub(crate) async fn apply_ready_compressed_key_materials(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<PendingCompressedKeyMaterial>> {
    let rows = sqlx::query_as::<_, PendingCompressedKeyMaterial>(
        r#"
        SELECT e.chain_id, e.block_hash, e.block_number, e.transaction_hash,
               e.key_id, e.key_material_id, e.key_digest_server AS key_digest,
               e.storage_urls
        FROM kms_key_activation_events AS e
        INNER JOIN host_chain_blocks_valid AS b
          ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash
        WHERE e.status = 'ready'
          AND b.block_status = 'finalized'
          AND e.key_digest_public IS NULL
          AND e.key_content_public IS NULL
          AND e.key_content_sks_key IS NULL
          AND e.key_content_compressed_xof_keyset IS NOT NULL
          AND e.key_id = (
              SELECT key_id FROM keys ORDER BY sequence_number DESC LIMIT 1
          )
        FOR UPDATE OF e SKIP LOCKED
        "#,
    )
    .fetch_all(tx.deref_mut())
    .await?;

    for row in &rows {
        let updated = sqlx::query(
            r#"
            UPDATE keys
            SET compressed_xof_keyset = e.key_content_compressed_xof_keyset
            FROM kms_key_activation_events AS e
            WHERE e.chain_id = $1 AND e.block_hash = $2 AND e.key_id = $3
              AND keys.sequence_number = (
                  SELECT MAX(sequence_number) FROM keys
              )
              AND keys.key_id = e.key_id
              AND e.key_content_compressed_xof_keyset IS NOT NULL
            "#,
        )
        .bind(row.chain_id)
        .bind(&row.block_hash)
        .bind(&row.key_id)
        .execute(tx.deref_mut())
        .await?;
        if updated.rows_affected() == 0 {
            anyhow::bail!(
                "compressed key material references unknown key_id {:?}",
                row.key_id
            );
        }

        sqlx::query(
            r#"
            UPDATE kms_key_activation_events
            SET status = 'activated', last_updated_at = NOW()
            WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3
            "#,
        )
        .bind(row.chain_id)
        .bind(&row.block_hash)
        .bind(&row.key_id)
        .execute(tx.deref_mut())
        .await?;
    }
    Ok(rows)
}

pub(crate) async fn applied_compressed_key_materials(
    db_pool: &Pool<Postgres>,
) -> anyhow::Result<Vec<AppliedCompressedKeyMaterial>> {
    Ok(sqlx::query_as(
        r#"
        SELECT e.chain_id, e.block_number, e.key_id,
               e.key_digest_server AS key_digest
        FROM kms_key_activation_events AS e
        WHERE e.status = 'activated'
          AND e.key_digest_public IS NULL
          AND e.key_content_public IS NULL
          AND e.key_content_sks_key IS NULL
          AND e.key_content_compressed_xof_keyset IS NOT NULL
        "#,
    )
    .fetch_all(db_pool)
    .await?)
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
    let rows = sqlx::query_as::<_, PendingKeyActivationRow>(
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
            AND key_content_compressed_xof_keyset IS DISTINCT FROM ''::BYTEA
            AND (
                key_content_sks_key IS NULL AND key_digest_server IS NOT NULL
                OR key_content_public IS NULL AND key_digest_public IS NOT NULL
            )
        FOR UPDATE SKIP LOCKED
        "#,
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

pub(crate) async fn all_pending_compressed_key_materials_to_download(
    tx: &mut Transaction<'_, Postgres>,
) -> anyhow::Result<Vec<PendingCompressedKeyMaterial>> {
    Ok(sqlx::query_as::<_, PendingCompressedKeyMaterial>(
        r#"
        SELECT e.chain_id, e.block_hash, e.block_number, e.transaction_hash,
               e.key_id, e.key_material_id, e.key_digest_server AS key_digest, e.storage_urls
        FROM kms_key_activation_events AS e
        INNER JOIN host_chain_blocks_valid AS b
          ON e.chain_id = b.chain_id AND e.block_hash = b.block_hash
        WHERE e.status = 'pending'
          AND b.block_status = 'finalized'
          AND e.key_digest_public IS NULL
          AND e.key_digest_server IS NOT NULL
          AND e.key_material_id IS NOT NULL
          AND e.key_content_public IS NULL
          AND e.key_content_sks_key IS NULL
          AND octet_length(e.key_content_compressed_xof_keyset) = 0
          AND e.key_id = (
              SELECT key_id FROM keys ORDER BY sequence_number DESC LIMIT 1
          )
        FOR UPDATE OF e SKIP LOCKED
        "#,
    )
    .fetch_all(tx.deref_mut())
    .await?)
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

pub(crate) async fn set_ready_compressed_key_material(
    tx: &mut Transaction<'_, Postgres>,
    material: &PendingCompressedKeyMaterial,
    key_content: &[u8],
) -> anyhow::Result<()> {
    let result = sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET status = 'ready',
            key_content_compressed_xof_keyset = $1,
            last_updated_at = NOW(),
            last_error = NULL
        WHERE chain_id = $2 AND block_hash = $3 AND key_id = $4
        "#,
    )
    .bind(key_content)
    .bind(material.chain_id)
    .bind(&material.block_hash)
    .bind(&material.key_id)
    .execute(tx.deref_mut())
    .await?;
    if result.rows_affected() == 0 {
        anyhow::bail!("compressed key material staging row disappeared")
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

pub async fn mark_compressed_key_material_error(
    tx: &mut Transaction<'_, Postgres>,
    error_msg: &str,
    material: &PendingCompressedKeyMaterial,
) {
    if let Err(error) = sqlx::query(
        r#"
        UPDATE kms_key_activation_events
        SET last_error = $1, retry_count = retry_count + 1, last_updated_at = NOW()
        WHERE chain_id = $2 AND block_hash = $3 AND key_id = $4
        "#,
    )
    .bind(error_msg)
    .bind(material.chain_id)
    .bind(&material.block_hash)
    .bind(&material.key_id)
    .execute(tx.deref_mut())
    .await
    {
        error!(%error, key_id = ?material.key_id, "Failed to update compressed key material error");
    }
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

        sqlx::query!(
            r#"
            INSERT INTO kms_key_activation_events (
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
            VALUES ($1, $2, 1, $3, $4, $5, $6, $7, $8)
            "#,
            chain_id.as_i64(),
            &block_hash,
            vec![3_u8; 32],
            &key_id,
            &existing_sks,
            vec![4_u8; 32],
            vec![5_u8; 32],
            &storage_urls as _,
        )
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

        let row = sqlx::query!(
            r#"
            SELECT
                status AS "status!",
                key_content_sks_key AS "key_content_sks_key!",
                key_content_public AS "key_content_public!"
             FROM kms_key_activation_events
             WHERE chain_id = $1 AND block_hash = $2 AND key_id = $3
            "#,
            chain_id.as_i64(),
            &block_hash,
            &key_id,
        )
        .fetch_one(&pool)
        .await?;

        assert_eq!(row.status, "ready");
        assert_eq!(row.key_content_sks_key, existing_sks);
        assert_eq!(row.key_content_public, public_key);

        Ok(())
    }

    #[tokio::test]
    async fn compressed_material_updates_only_the_compressed_representation(
    ) -> anyhow::Result<()> {
        let db = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let pool = sqlx::PgPool::connect(db.db_url()).await?;
        let chain_id = 12345_i64;
        let original_block = vec![1_u8; 32];
        let migration_block = vec![2_u8; 32];
        let next_key_block = vec![5_u8; 32];
        let key_id = vec![3_u8; 32];
        let key_material_id = vec![4_u8; 32];
        let next_key_id = vec![6_u8; 32];
        let legacy_public = b"legacy-public".to_vec();
        let legacy_server = b"legacy-server".to_vec();
        let compressed = b"compressed-xof".to_vec();

        sqlx::query(
            "INSERT INTO keys (chain_id, block_hash, key_id_gw, key_id, pks_key, sks_key)
             VALUES ($1, $2, $3, $3, $4, $5)",
        )
        .bind(chain_id)
        .bind(&original_block)
        .bind(&key_id)
        .bind(&legacy_public)
        .bind(&legacy_server)
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO host_chain_blocks_valid
             (chain_id, block_hash, block_number, block_status)
             VALUES ($1, $2, 10, 'pending')",
        )
        .bind(chain_id)
        .bind(&migration_block)
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO kms_key_activation_events
             (chain_id, block_hash, block_number, transaction_hash,
              key_id, key_material_id, key_digest_server, storage_urls,
              key_content_compressed_xof_keyset)
             VALUES ($1, $2, 10, $3, $4, $5, $6, ARRAY[]::TEXT[], ''::BYTEA)",
        )
        .bind(chain_id)
        .bind(&migration_block)
        .bind(vec![9_u8; 32])
        .bind(&key_id)
        .bind(&key_material_id)
        .bind(vec![11_u8; 32])
        .execute(&pool)
        .await?;
        sqlx::query(
            "INSERT INTO kms_key_activation_events
             (chain_id, block_hash, block_number, transaction_hash,
              key_id, key_digest_server, key_digest_public, storage_urls,
              key_content_sks_key, key_content_compressed_xof_keyset)
             VALUES ($1, $2, 11, $3, $4, $5, $6, ARRAY[]::TEXT[], $7, $8)",
        )
        .bind(chain_id)
        .bind(&next_key_block)
        .bind(vec![8_u8; 32])
        .bind(&next_key_id)
        .bind(vec![7_u8; 32])
        .bind(vec![10_u8; 32])
        .bind(b"next-legacy-server".as_slice())
        .bind(b"next-compressed-xof".as_slice())
        .execute(&pool)
        .await?;

        let mut tx = pool.begin().await?;
        let ordinary = all_pending_key_activations_to_download(&mut tx).await?;
        assert_eq!(ordinary.len(), 1);
        assert_eq!(ordinary[0].key_id, next_key_id);
        let migration =
            all_pending_compressed_key_materials_to_download(&mut tx).await?;
        assert!(migration.is_empty());
        tx.rollback().await?;
        sqlx::query(
            "UPDATE host_chain_blocks_valid SET block_status = 'finalized'
             WHERE chain_id = $1 AND block_hash = $2",
        )
        .bind(chain_id)
        .bind(&migration_block)
        .execute(&pool)
        .await?;

        let mut tx = pool.begin().await?;
        let migration =
            all_pending_compressed_key_materials_to_download(&mut tx).await?;
        assert_eq!(migration.len(), 1);
        assert_eq!(migration[0].key_id, key_id);
        assert_eq!(migration[0].key_material_id, key_material_id);
        tx.rollback().await?;
        sqlx::query(
            "UPDATE kms_key_activation_events
             SET key_content_compressed_xof_keyset = $1, status = 'ready'
             WHERE key_id = $2",
        )
        .bind(&compressed)
        .bind(&key_id)
        .execute(&pool)
        .await?;

        let mut tx = pool.begin().await?;
        assert_eq!(
            apply_ready_compressed_key_materials(&mut tx).await?.len(),
            1
        );
        tx.commit().await?;

        let row = sqlx::query(
            "SELECT pks_key, sks_key, compressed_xof_keyset FROM keys WHERE key_id = $1",
        )
        .bind(&key_id)
        .fetch_one(&pool)
        .await?;
        assert_eq!(row.try_get::<Vec<u8>, _>("pks_key")?, legacy_public);
        assert_eq!(row.try_get::<Vec<u8>, _>("sks_key")?, legacy_server);
        assert_eq!(
            row.try_get::<Vec<u8>, _>("compressed_xof_keyset")?,
            compressed
        );
        let status = sqlx::query_scalar::<_, String>(
            "SELECT status FROM kms_key_activation_events WHERE key_id = $1",
        )
        .bind(&key_id)
        .fetch_one(&pool)
        .await?;
        assert_eq!(status, "activated");
        let applied = applied_compressed_key_materials(&pool).await?;
        assert_eq!(applied.len(), 1);
        assert_eq!(applied[0].chain_id, chain_id);
        assert_eq!(applied[0].block_number, 10);
        assert_eq!(applied[0].key_id, key_id);
        Ok(())
    }
}
