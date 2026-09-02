use crate::manifest_consensus::{
    lineage::RangeFrontier,
    publication::{
        block_discovery::PendingBlock,
        manifest_history::{
            append_detailed_blocks, historical_ranges, load_detailed_lineage, load_frontier,
        },
    },
    Config as ManifestConsensusConfig, ExecutionError,
};
use alloy_primitives::{Address, B256, U256};
use block_manifest::{
    block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, CiphertextFormat,
    DetailedRange, ManifestBlockEntry, ManifestPayload, ManifestVersion,
};
use sqlx::{Postgres, Transaction};
use std::collections::HashMap;
use tracing::error;

/// A validated manifest payload together with the frontier that must be
/// persisted if publication succeeds.
#[derive(Debug)]
pub(crate) struct PreparedManifest {
    pub payload: ManifestPayload,
    pub history_frontier: RangeFrontier,
}

pub(crate) type CiphertextDescriptor = BlockCiphertextDescriptor;

/// Returns whether every ciphertext produced and initially allowed in the
/// block is ready to enter the manifest.
///
/// A handle is ready when it has durable ct64/ct128 digests and format, or
/// when its matching `computations` row is `is_error`. Terminal errors have
/// no SNS digest; they are sealed as `is_error` descriptors. Missing
/// ciphertext can also be sealed as `is_uncomputed` after
/// [`missing_handles_are_uncomputed`] becomes true.
pub(crate) async fn is_block_manifest_ready(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<bool, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH block_handles AS MATERIALIZED (
            SELECT DISTINCT producer.handle,
                   d.key_id_gw,
                   d.ciphertext AS ct64_digest,
                   d.ciphertext128 AS ct128_digest,
                   d.ciphertext128_format AS ct128_format,
                   EXISTS (
                       SELECT 1
                         FROM computations c
                        WHERE c.host_chain_id = producer.host_chain_id
                          AND c.block_number = producer.producer_block_number
                          AND c.output_handle = producer.handle
                          AND c.is_error
                   ) AS is_error
              FROM handle_producer_block producer
              LEFT JOIN ciphertext_digest d
                ON d.host_chain_id = producer.host_chain_id
               AND d.handle = producer.handle
             WHERE producer.host_chain_id = $1
               AND producer.producer_block_number = $2
               AND producer.producer_block_hash = $3
        )
        SELECT NOT EXISTS (
                   SELECT 1
                     FROM block_handles
                    WHERE NOT is_error
                      AND (
                          key_id_gw IS NULL
                          OR ct64_digest IS NULL
                          OR ct128_digest IS NULL
                          OR ct128_format IS NULL
                      )
               ) AS "ready!"
        "#,
        block.host_chain_id,
        block.block_number,
        &block.block_hash,
    )
    .fetch_one(trx.as_mut())
    .await?;

    Ok(row.ready)
}

/// Returns whether this unsealed block may seal leftover missing ciphertext as
/// `is_uncomputed`: the host chain has advanced by more than
/// `--incomplete-manifest-max-lag` due manifests, and no handle in the block
/// has been computed for `--incomplete-block-timeout`. A newly computed handle
/// resets the stall.
pub(crate) async fn missing_handles_are_uncomputed(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
    consensus: &ManifestConsensusConfig,
) -> Result<bool, ExecutionError> {
    let Some(lag) = consensus.incomplete_seal_lag_blocks(block.publication_cadence) else {
        return Ok(false);
    };
    let Some(stall_secs) = consensus.incomplete_block_timeout_secs() else {
        return Ok(false);
    };
    let uncomputed = sqlx::query_scalar!(
        r#"
        WITH computed AS (
            SELECT MAX(at) AS last_at
              FROM (
                SELECT d.created_at AT TIME ZONE 'UTC' AS at
                  FROM handle_producer_block producer
                  JOIN ciphertext_digest d
                    ON d.host_chain_id = producer.host_chain_id
                   AND d.handle = producer.handle
                 WHERE producer.host_chain_id = $1
                   AND producer.producer_block_number = $2
                   AND producer.producer_block_hash = $3
                   AND (d.ciphertext IS NOT NULL OR d.ciphertext128 IS NOT NULL)
                UNION ALL
                SELECT c.completed_at AT TIME ZONE 'UTC'
                  FROM handle_producer_block producer
                  JOIN computations c
                    ON c.host_chain_id = producer.host_chain_id
                   AND c.block_number = producer.producer_block_number
                   AND c.output_handle = producer.handle
                 WHERE producer.host_chain_id = $1
                   AND producer.producer_block_number = $2
                   AND producer.producer_block_hash = $3
                   AND c.completed_at IS NOT NULL
              ) computed_events
        ), inventory AS (
            SELECT MAX(producer.created_at) AS last_at
              FROM handle_producer_block producer
             WHERE producer.host_chain_id = $1
               AND producer.producer_block_number = $2
               AND producer.producer_block_hash = $3
        ), progress AS (
            SELECT COALESCE(computed.last_at, inventory.last_at, state.created_at) AS last_at
              FROM computed
              CROSS JOIN inventory
              LEFT JOIN block_manifest_state state
                ON state.host_chain_id = $1
               AND state.block_hash = $3
               AND state.generation = $6
        ), tip AS (
            SELECT MAX(block_number) AS tip
              FROM host_chain_blocks_valid
             WHERE chain_id = $1
               AND block_status <> 'orphaned'
        )
        SELECT (tip.tip IS NOT NULL
                AND tip.tip - $2 > $4
                AND progress.last_at IS NOT NULL
                AND progress.last_at <= NOW() - ($5::BIGINT * INTERVAL '1 second')
               ) AS "uncomputed!"
          FROM tip
          CROSS JOIN progress
        "#,
        block.host_chain_id,
        block.block_number,
        &block.block_hash,
        lag,
        stall_secs,
        block.generation,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(uncomputed)
}

/// Loads manifest descriptors in raw handle order and rejects duplicate or
/// incomplete handles. Call only after `is_block_manifest_ready` succeeds, or
/// with `allow_uncomputed` after [`missing_handles_are_uncomputed`].
pub(crate) async fn load_manifest_descriptors(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
    allow_uncomputed: bool,
) -> Result<Vec<CiphertextDescriptor>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT producer.handle AS "handle!",
               d.key_id_gw AS "key_id_gw?",
               d.ciphertext AS "ct64_digest?",
               d.ciphertext128 AS "ct128_digest?",
               d.ciphertext128_format AS "ct128_format?",
               EXISTS (
                   SELECT 1
                     FROM computations c
                    WHERE c.host_chain_id = producer.host_chain_id
                      AND c.block_number = producer.producer_block_number
                      AND c.output_handle = producer.handle
                      AND c.is_error
               ) AS "is_error!",
               (
                   SELECT c.error_message
                     FROM computations c
                    WHERE c.host_chain_id = producer.host_chain_id
                      AND c.block_number = producer.producer_block_number
                      AND c.output_handle = producer.handle
                      AND c.is_error
                    ORDER BY c.error_message NULLS LAST
                    LIMIT 1
               ) AS "error_message?"
          FROM handle_producer_block producer
          LEFT JOIN ciphertext_digest d
            ON d.host_chain_id = producer.host_chain_id
           AND d.handle = producer.handle
         WHERE producer.host_chain_id = $1
           AND producer.producer_block_number = $2
           AND producer.producer_block_hash = $3
         ORDER BY producer.handle
        "#,
        block.host_chain_id,
        block.block_number,
        &block.block_hash,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let keyset_ids = load_keyset_ids(trx, block.host_chain_id).await?;

    let mut descriptors = Vec::with_capacity(rows.len());
    let mut previous_handle: Option<Vec<u8>> = None;

    for row in rows {
        if previous_handle.as_deref() == Some(row.handle.as_slice()) {
            return Err(internal(format!(
                "duplicate allowed handle {} in chain {} block {}",
                hex::encode(&row.handle),
                block.host_chain_id,
                block.block_number,
            )));
        }
        previous_handle = Some(row.handle.clone());

        if row.is_error {
            if row.handle.len() != 32 {
                return Err(internal(format!(
                    "invalid handle length {} in chain {} block {}",
                    row.handle.len(),
                    block.host_chain_id,
                    block.block_number,
                )));
            }
            descriptors.push(CiphertextDescriptor::from_computation_error(
                B256::from_slice(&row.handle),
                row.error_message.filter(|message| !message.is_empty()),
            ));
            continue;
        }

        let (Some(ct64_digest), Some(ct128_digest), Some(ct128_format)) =
            (row.ct64_digest, row.ct128_digest, row.ct128_format)
        else {
            if allow_uncomputed {
                if row.handle.len() != 32 {
                    return Err(internal(format!(
                        "invalid handle length {} in chain {} block {}",
                        row.handle.len(),
                        block.host_chain_id,
                        block.block_number,
                    )));
                }
                descriptors.push(CiphertextDescriptor::from_uncomputed(B256::from_slice(
                    &row.handle,
                )));
                continue;
            }
            return Err(internal(format!(
                "manifest-ready handle {} became incomplete in chain {} block {}",
                hex::encode(&row.handle),
                block.host_chain_id,
                block.block_number,
            )));
        };
        let gateway_key_id = row.key_id_gw;
        let keyset_id = gateway_key_id
            .as_ref()
            .and_then(|gateway_key_id| keyset_ids.get(gateway_key_id))
            .ok_or_else(|| {
                internal(format!(
                    "no keyset ID maps manifest handle {} to its local Gateway key ID in chain {} block {}",
                    hex::encode(&row.handle),
                    block.host_chain_id,
                    block.block_number,
                ))
            })?;

        let ct128_format = match ct128_format {
            10 => CiphertextFormat::UncompressedOnCpu,
            11 => CiphertextFormat::CompressedOnCpu,
            20 => CiphertextFormat::UncompressedOnGpu,
            21 => CiphertextFormat::CompressedOnGpu,
            _ => {
                return Err(internal(format!(
                    "invalid ct128 format {ct128_format} for handle {}",
                    hex::encode(&row.handle),
                )));
            }
        };

        for (name, value) in [
            ("handle", row.handle.as_slice()),
            ("keyset id", keyset_id.as_slice()),
            ("ct64 digest", ct64_digest.as_slice()),
            ("ct128 digest", ct128_digest.as_slice()),
        ] {
            if value.len() != 32 {
                return Err(internal(format!(
                    "invalid {name} length {} in chain {} block {}",
                    value.len(),
                    block.host_chain_id,
                    block.block_number,
                )));
            }
        }

        descriptors.push(CiphertextDescriptor::computed(
            B256::from_slice(&row.handle),
            U256::from_be_slice(keyset_id),
            gateway_key_id.as_deref().map(U256::from_be_slice),
            B256::from_slice(&ct64_digest),
            B256::from_slice(&ct128_digest),
            ct128_format,
        ));
    }

    Ok(descriptors)
}

async fn load_keyset_ids(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
) -> Result<HashMap<Vec<u8>, Vec<u8>>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT key_id_gw, key_id
          FROM keys
         WHERE chain_id = $1
         ORDER BY sequence_number
        "#,
        host_chain_id,
    )
    .fetch_all(trx.as_mut())
    .await?;
    let mut keyset_ids = HashMap::with_capacity(rows.len());
    for row in rows {
        let gateway_key_id = row.key_id_gw;
        let keyset_id = row.key_id;
        if let Some(previous) = keyset_ids.insert(gateway_key_id.clone(), keyset_id.clone()) {
            if previous != keyset_id {
                return Err(internal(format!(
                    "Gateway key ID {} maps to conflicting keyset IDs on chain {host_chain_id}",
                    hex::encode(gateway_key_id),
                )));
            }
        }
    }
    Ok(keyset_ids)
}

/// Seals the block digest exactly once. A zero-row update is always reported
/// with the current state so an unexpected repeat is diagnosable.
pub(crate) async fn seal_block_content(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
    coprocessor_context_id: U256,
    descriptors: &[CiphertextDescriptor],
) -> Result<B256, ExecutionError> {
    if block.block_content_digest.is_some() {
        error!(
            host_chain_id = block.host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            block_handle_count = block.block_handle_count,
            manifest_revision = block.manifest_revision,
            manifest_published = block.manifest_published,
            "Block content sealing requested for an already sealed row"
        );
        return Err(internal(format!(
            "block content already sealed for chain {} block {}",
            block.host_chain_id, block.block_number,
        )));
    }

    let digest = block_content_digest(
        ManifestVersion::V1,
        coprocessor_context_id,
        non_negative_u256("host chain id", block.host_chain_id)?,
        non_negative_u256("block number", block.block_number)?,
        b256("block hash", &block.block_hash)?,
        descriptors,
    )
    .map_err(|err| internal(err.to_string()))?;
    let block_handle_count = i64::try_from(descriptors.len())
        .map_err(|_| internal("manifest descriptor count exceeds BIGINT"))?;

    let result = sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET block_content_digest = $3,
               block_handle_count = $4,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $5
           AND block_content_digest IS NULL
           AND block_handle_count IS NULL
        "#,
        block.host_chain_id,
        &block.block_hash,
        digest.as_slice(),
        block_handle_count,
        block.generation,
    )
    .execute(trx.as_mut())
    .await?;

    if result.rows_affected() != 1 {
        let current = sqlx::query!(
            r#"
            SELECT block_content_digest,
                   block_handle_count,
                   manifest_revision,
                   manifest_published
              FROM block_manifest_state
             WHERE host_chain_id = $1
               AND block_hash = $2
               AND generation = $3
            "#,
            block.host_chain_id,
            &block.block_hash,
            block.generation,
        )
        .fetch_optional(trx.as_mut())
        .await?;
        let failure_reason = match current.as_ref() {
            None => "row_missing",
            Some(row) if row.block_content_digest.is_some() && row.block_handle_count.is_some() => {
                "already_sealed_concurrently"
            }
            Some(_) => "partial_seal_state",
        };
        error!(
            host_chain_id = block.host_chain_id,
            block_number = block.block_number,
            block_hash = %hex::encode(&block.block_hash),
            failure_reason,
            stored_block_content_digest = ?current.as_ref()
                .and_then(|row| row.block_content_digest.as_deref())
                .map(hex::encode),
            stored_block_handle_count = ?current.as_ref().and_then(|row| row.block_handle_count),
            "Block content sealing updated no row"
        );
        return Err(internal(format!(
            "block content sealing updated no row for chain {} block {}: {failure_reason}",
            block.host_chain_id, block.block_number,
        )));
    }

    Ok(digest)
}

/// Drops an unpublished seal so the next pass reseals from live tables.
/// Published rows are left untouched: that would be a new revision.
async fn discard_unpublished_seal(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<(), ExecutionError> {
    let result = sqlx::query!(
        r#"
        UPDATE block_manifest_state
           SET block_content_digest = NULL,
               block_handle_count = NULL,
               publication_error_count = 0,
               publication_last_error = NULL,
               publication_next_retry_at = NULL,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $3
           AND manifest_published = FALSE
           AND block_content_digest IS NOT NULL
        "#,
        block.host_chain_id,
        &block.block_hash,
        block.generation,
    )
    .execute(trx.as_mut())
    .await?;
    if result.rows_affected() != 1 {
        return Err(internal(format!(
            "failed to discard unpublished seal for chain {} block {}",
            block.host_chain_id, block.block_number,
        )));
    }
    Ok(())
}

fn stale_seal(block: &PendingBlock, reason: impl Into<String>) -> ExecutionError {
    ExecutionError::StaleBlockSeal {
        host_chain_id: block.host_chain_id,
        block_number: block.block_number,
        reason: reason.into(),
    }
}

pub(crate) async fn prepare_manifest(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    coprocessor_context_id: U256,
    publisher: Address,
) -> Result<PreparedManifest, ExecutionError> {
    if target.manifest_published || target.manifest_digest.is_some() {
        return Err(internal(format!(
            "manifest preparation requested for an already published row at chain {} block {}",
            target.host_chain_id, target.block_number,
        )));
    }

    let revision = u64::try_from(target.manifest_revision)
        .map_err(|_| internal("manifest revision is negative"))?;
    let (lineage, last_published_manifest) =
        load_detailed_lineage(trx, target, coprocessor_context_id).await?;
    let blocks = load_detailed_blocks(trx, &lineage, coprocessor_context_id).await?;
    let detailed_range = build_detailed_range(target, coprocessor_context_id, blocks)?;
    let host_chain_id = non_negative_u256("host chain id", target.host_chain_id)?;

    let generation = target.generation.clone();
    let mut history_frontier = match last_published_manifest.as_ref() {
        Some(previous) => {
            load_frontier(trx, target.host_chain_id, coprocessor_context_id, previous).await?
        }
        None => RangeFrontier::default(),
    };
    let historical_ranges = historical_ranges(&history_frontier);
    append_detailed_blocks(
        trx,
        target.host_chain_id,
        coprocessor_context_id,
        &target.generation,
        &mut history_frontier,
        &detailed_range.blocks,
    )
    .await?;

    let payload = ManifestPayload {
        version: ManifestVersion::V1,
        consensus_epoch: generation,
        publisher,
        coprocessor_context_id,
        host_chain_id,
        publication_block_number: non_negative_u256("block number", target.block_number)?,
        publication_block_hash: b256("block hash", &target.block_hash)?,
        publication_parent_block_hash: b256("parent block hash", &target.parent_block_hash)?,
        revision,
        detailed_range,
        historical_ranges,
    };
    payload
        .validate()
        .map_err(|err| internal(format!("prepared manifest is invalid: {err}")))?;

    Ok(PreparedManifest {
        payload,
        history_frontier,
    })
}

async fn load_detailed_blocks(
    trx: &mut Transaction<'_, Postgres>,
    lineage: &[PendingBlock],
    coprocessor_context_id: U256,
) -> Result<Vec<ManifestBlockEntry>, ExecutionError> {
    let mut blocks = Vec::with_capacity(lineage.len());
    for block in lineage {
        blocks.push(load_detailed_block(trx, block, coprocessor_context_id).await?);
    }
    Ok(blocks)
}

async fn load_detailed_block(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
    coprocessor_context_id: U256,
) -> Result<ManifestBlockEntry, ExecutionError> {
    let descriptors = load_manifest_descriptors(trx, block, true).await?;
    let stored_count = block
        .block_handle_count
        .ok_or_else(|| internal("detailed-range block has no descriptor count"))?;
    if usize::try_from(stored_count).ok() != Some(descriptors.len()) {
        if block.manifest_published {
            return Err(internal(format!(
                "descriptor count changed for published chain {} block {}: stored {}, loaded {}",
                block.host_chain_id,
                block.block_number,
                stored_count,
                descriptors.len(),
            )));
        }
        discard_unpublished_seal(trx, block).await?;
        return Err(stale_seal(
            block,
            format!(
                "descriptor count changed: stored {}, loaded {}",
                stored_count,
                descriptors.len()
            ),
        ));
    }

    let stored_digest = b256(
        "block content digest",
        block
            .block_content_digest
            .as_deref()
            .ok_or_else(|| internal("detailed-range block is not sealed"))?,
    )?;
    let recomputed_digest = block_content_digest(
        ManifestVersion::V1,
        coprocessor_context_id,
        non_negative_u256("host chain id", block.host_chain_id)?,
        non_negative_u256("block number", block.block_number)?,
        b256("block hash", &block.block_hash)?,
        &descriptors,
    )
    .map_err(|err| internal(err.to_string()))?;
    if stored_digest != recomputed_digest {
        if block.manifest_published {
            return Err(internal(format!(
                "stored block digest conflicts with descriptors for published chain {} block {}",
                block.host_chain_id, block.block_number,
            )));
        }
        discard_unpublished_seal(trx, block).await?;
        return Err(stale_seal(
            block,
            "stored block digest conflicts with live descriptors",
        ));
    }

    Ok(ManifestBlockEntry {
        block_number: non_negative_u256("block number", block.block_number)?,
        block_hash: b256("block hash", &block.block_hash)?,
        parent_block_hash: b256("parent block hash", &block.parent_block_hash)?,
        block_content_digest: stored_digest,
        ciphertexts: descriptors,
    })
}

fn build_detailed_range(
    target: &PendingBlock,
    coprocessor_context_id: U256,
    blocks: Vec<ManifestBlockEntry>,
) -> Result<DetailedRange, ExecutionError> {
    let first_block_number = blocks
        .first()
        .ok_or_else(|| internal("empty detailed range"))?
        .block_number;
    let last_block_number = blocks.last().expect("checked non-empty").block_number;
    let block_digests = blocks
        .iter()
        .map(|block| block.block_content_digest)
        .collect::<Vec<_>>();
    let digest = detailed_range_digest(
        ManifestVersion::V1,
        coprocessor_context_id,
        non_negative_u256("host chain id", target.host_chain_id)?,
        first_block_number,
        last_block_number,
        &block_digests,
    );
    Ok(DetailedRange {
        first_block_number,
        last_block_number,
        digest,
        blocks,
    })
}

pub(super) fn non_negative_u256(field: &str, value: i64) -> Result<U256, ExecutionError> {
    let value = u64::try_from(value)
        .map_err(|_| internal(format!("{field} must be non-negative, got {value}")))?;
    Ok(U256::from(value))
}

pub(super) fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

pub(super) fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let value: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(value))
}

pub(super) fn b20_address(field: &str, value: &[u8]) -> Result<Address, ExecutionError> {
    let value: [u8; 20] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 20 bytes, got {}", value.len())))?;
    Ok(Address::from(value))
}

pub(super) fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
