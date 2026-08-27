use crate::manifest_consensus::{
    lineage::RangeFrontier,
    publication::{
        block_discovery::PendingBlock,
        manifest_history::{
            append_detailed_blocks, historical_ranges, load_detailed_lineage, load_frontier,
        },
    },
    ExecutionError,
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
/// block has both durable digests and its format ready for the manifest.
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
                   d.ciphertext128_format AS ct128_format
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
                    WHERE key_id_gw IS NULL
                       OR ct64_digest IS NULL
                       OR ct128_digest IS NULL
                       OR ct128_format IS NULL
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

/// Loads manifest descriptors in raw handle order and rejects duplicate or
/// incomplete handles. Call only after `is_block_manifest_ready` succeeds.
pub(crate) async fn load_manifest_descriptors(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<Vec<CiphertextDescriptor>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT producer.handle AS "handle!",
               d.key_id_gw AS "key_id_gw?",
               d.ciphertext AS "ct64_digest?",
               d.ciphertext128 AS "ct128_digest?",
               d.ciphertext128_format AS "ct128_format?"
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

        let (Some(ct64_digest), Some(ct128_digest), Some(ct128_format)) =
            (row.ct64_digest, row.ct128_digest, row.ct128_format)
        else {
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

        descriptors.push(CiphertextDescriptor {
            handle: B256::from_slice(&row.handle),
            keyset_id: U256::from_be_slice(keyset_id),
            gateway_key_id: gateway_key_id.as_deref().map(U256::from_be_slice),
            ct64_digest: B256::from_slice(&ct64_digest),
            ct128_digest: B256::from_slice(&ct128_digest),
            ct128_format,
        });
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
    let (lineage, last_published_manifest) = load_detailed_lineage(trx, target).await?;
    let blocks = load_detailed_blocks(trx, &lineage, coprocessor_context_id).await?;
    let detailed_range = build_detailed_range(target, coprocessor_context_id, blocks)?;
    let host_chain_id = non_negative_u256("host chain id", target.host_chain_id)?;

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
        &mut history_frontier,
        &detailed_range.blocks,
    )
    .await?;

    let payload = ManifestPayload {
        version: ManifestVersion::V1,
        generation: u64::try_from(target.generation)
            .map_err(|_| internal("manifest generation is negative"))?,
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
    let descriptors = load_manifest_descriptors(trx, block).await?;
    let stored_count = block
        .block_handle_count
        .ok_or_else(|| internal("detailed-range block has no descriptor count"))?;
    if usize::try_from(stored_count).ok() != Some(descriptors.len()) {
        return Err(internal(format!(
            "descriptor count changed for chain {} block {}: stored {}, loaded {}",
            block.host_chain_id,
            block.block_number,
            stored_count,
            descriptors.len(),
        )));
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
        return Err(internal(format!(
            "stored block digest conflicts with descriptors for chain {} block {}",
            block.host_chain_id, block.block_number,
        )));
    }

    Ok(ManifestBlockEntry {
        generation: u64::try_from(block.generation)
            .map_err(|_| internal("block generation is negative"))?,
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
