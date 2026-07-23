use std::collections::BTreeSet;

use alloy_primitives::{B256, U256};
use ciphertext_attestation::manifest::{
    HistoricalRange, ManifestBlockEntry, ManifestReference, ManifestVersion,
};
use sqlx::{Postgres, Transaction};
use tracing::warn;

use crate::ExecutionError;

use super::{
    lineage::{RangeFrontier, RangeNode},
    manifest::{b20_address, b256, i64_from_u256, internal, non_negative_u256, PendingBlock},
    manifest_archive::load_manifest_by_reference,
    manifest_frontier::{append_leaf, rebuild_frontier_from_manifest, validate_canonical_frontier},
};

pub(super) async fn load_detailed_lineage(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
) -> Result<(Vec<PendingBlock>, Option<ManifestReference>), ExecutionError> {
    let mut lineage = vec![target.clone()];
    let mut parent_hash = target.parent_block_hash.clone();
    let previous_manifest = loop {
        let Some(parent) = load_block_by_hash(trx, target.host_chain_id, &parent_hash).await?
        else {
            break None;
        };
        if parent.manifest_published {
            break Some(manifest_reference(&parent)?);
        }
        parent_hash.clone_from(&parent.parent_block_hash);
        lineage.push(parent);
    };
    lineage.reverse();
    validate_contiguous_lineage(&lineage)?;
    Ok((lineage, previous_manifest))
}

fn manifest_reference(block: &PendingBlock) -> Result<ManifestReference, ExecutionError> {
    Ok(ManifestReference {
        publisher: b20_address(
            "previous manifest publisher",
            block
                .last_manifest_publisher
                .as_deref()
                .ok_or_else(|| internal("published manifest row has no publisher"))?,
        )?,
        block_number: non_negative_u256("block number", block.block_number)?,
        block_hash: b256("block hash", &block.block_hash)?,
        revision: u64::try_from(block.manifest_revision)
            .map_err(|_| internal("negative manifest revision"))?,
        manifest_digest: b256(
            "previous manifest digest",
            block
                .manifest_digest
                .as_deref()
                .ok_or_else(|| internal("published manifest row has no digest"))?,
        )?,
    })
}

fn validate_contiguous_lineage(lineage: &[PendingBlock]) -> Result<(), ExecutionError> {
    if lineage.windows(2).any(|pair| {
        pair[0].block_number.checked_add(1) != Some(pair[1].block_number)
            || pair[0].block_hash != pair[1].parent_block_hash
    }) {
        return Err(internal("detailed manifest lineage is not contiguous"));
    }
    Ok(())
}

async fn load_block_by_hash(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    block_hash: &[u8],
) -> Result<Option<PendingBlock>, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT host_chain_id,
               block_number,
               block_hash,
               parent_block_hash,
               publication_cadence,
               block_content_digest,
               descriptor_count,
               manifest_revision,
               last_manifest_publisher,
               manifest_digest,
               manifest_published
          FROM block_consensus
         WHERE host_chain_id = $1
           AND block_hash = $2
        "#,
        host_chain_id,
        block_hash,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    Ok(row.map(|row| PendingBlock {
        host_chain_id: row.host_chain_id,
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_block_hash: row.parent_block_hash,
        publication_cadence: row.publication_cadence,
        block_content_digest: row.block_content_digest,
        descriptor_count: row.descriptor_count,
        manifest_revision: row.manifest_revision,
        last_manifest_publisher: row.last_manifest_publisher,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

pub(super) async fn load_frontier(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    manifest: &ManifestReference,
) -> Result<RangeFrontier, ExecutionError> {
    let block_number = i64_from_u256("manifest block number", manifest.block_number)?;
    let manifest_revision = i64::try_from(manifest.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let Some(archived) = load_manifest_by_reference(
        trx,
        ManifestVersion::V1,
        coprocessor_context_id,
        host_chain_id,
        manifest,
    )
    .await?
    else {
        warn!(
            host_chain_id,
            block_number,
            manifest_revision,
            manifest_digest = %manifest.manifest_digest,
            "Stored previous manifest is missing; rebuilding its frontier from block lineage"
        );
        return reconstruct_frontier_from_lineage(
            trx,
            host_chain_id,
            coprocessor_context_id,
            manifest,
        )
        .await;
    };

    let historical_ranges =
        hydrate_historical_ranges(trx, host_chain_id, &archived.signed.payload).await?;
    let (frontier, reconstructed_ranges) =
        rebuild_frontier_from_manifest(&archived.signed.payload, historical_ranges)?;
    for range in reconstructed_ranges {
        persist_range(trx, host_chain_id, &range).await?;
    }

    validate_frontier_tip(&frontier, block_number, manifest.block_hash)?;
    Ok(frontier)
}

async fn hydrate_historical_ranges(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    payload: &ciphertext_attestation::manifest::ManifestPayload,
) -> Result<Vec<RangeNode>, ExecutionError> {
    // Wire history is newest-to-oldest. The merge algorithm uses the reverse
    // order and also needs boundary hashes omitted from the compact wire form.
    let mut ranges = Vec::with_capacity(payload.historical_ranges.len());
    for historical in payload.historical_ranges.iter().rev() {
        let start = i64_from_u256("historical range start", historical.start_block_number)?;
        let end = i64_from_u256("historical range end", historical.end_block_number)?;
        let scale = i32::try_from(historical.scale)
            .map_err(|_| internal("historical range scale exceeds INTEGER"))?;
        let boundary = sqlx::query!(
            r#"
            SELECT range_start_block_hash,
                   range_start_parent_block_hash
              FROM block_consensus_range
             WHERE host_chain_id = $1
               AND range_start = $2
               AND range_end = $3
               AND scale = $4
               AND range_end_block_hash = $5
               AND range_digest = $6
            "#,
            host_chain_id,
            start,
            end,
            scale,
            historical.end_block_hash.as_slice(),
            historical.digest.as_slice(),
        )
        .fetch_optional(trx.as_mut())
        .await?
        .ok_or_else(|| {
            internal(format!(
                "BlockRange [{start}, {end}] referenced by the previous manifest is missing",
            ))
        })?;
        ranges.push(RangeNode {
            start,
            end,
            scale,
            start_block_hash: b256("range start block hash", &boundary.range_start_block_hash)?,
            start_parent_block_hash: b256(
                "range start parent block hash",
                &boundary.range_start_parent_block_hash,
            )?,
            end_block_hash: historical.end_block_hash,
            digest: historical.digest,
        });
    }
    Ok(ranges)
}

/// Rebuilds the predecessor's canonical frontier when its signed local body
/// has been lost. The immutable reference is authoritative; the predecessor
/// row must still carry the exact referenced identity and digest.
async fn reconstruct_frontier_from_lineage(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    manifest: &ManifestReference,
) -> Result<RangeFrontier, ExecutionError> {
    let expected_block_number = i64_from_u256("manifest block number", manifest.block_number)?;
    let expected_revision = i64::try_from(manifest.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let mut next_hash = manifest.block_hash.as_slice().to_vec();
    let mut visited = BTreeSet::new();
    let mut reverse_lineage = Vec::new();

    loop {
        if !visited.insert(next_hash.clone()) {
            return Err(internal(format!(
                "cycle while reconstructing block lineage ending at chain {host_chain_id} block {expected_block_number}",
            )));
        }
        let Some(block) = load_block_by_hash(trx, host_chain_id, &next_hash).await? else {
            break;
        };
        next_hash.clone_from(&block.parent_block_hash);
        reverse_lineage.push(block);
    }

    let predecessor = reverse_lineage.first().ok_or_else(|| {
        internal(format!(
            "cannot reconstruct missing previous manifest for chain {host_chain_id} block {expected_block_number}: predecessor block is missing",
        ))
    })?;
    validate_predecessor_identity(
        predecessor,
        host_chain_id,
        expected_block_number,
        expected_revision,
        manifest,
    )?;

    reverse_lineage.reverse();
    validate_contiguous_lineage(&reverse_lineage).map_err(|_| {
        internal(format!(
            "non-contiguous block lineage while reconstructing missing manifest for chain {host_chain_id} block {expected_block_number}",
        ))
    })?;

    let mut frontier = RangeFrontier::default();
    for block in reverse_lineage {
        let digest = b256(
            "block content digest",
            block.block_content_digest.as_deref().ok_or_else(|| {
                internal(format!(
                    "unsealed block {} while reconstructing missing manifest for chain {host_chain_id}",
                    block.block_number,
                ))
            })?,
        )?;
        persist_frontier_leaf(
            trx,
            host_chain_id,
            coprocessor_context_id,
            &mut frontier,
            RangeNode {
                start: block.block_number,
                end: block.block_number,
                scale: 0,
                start_block_hash: b256("block hash", &block.block_hash)?,
                start_parent_block_hash: b256("parent block hash", &block.parent_block_hash)?,
                end_block_hash: b256("block hash", &block.block_hash)?,
                digest,
            },
        )
        .await?;
    }

    validate_frontier_tip(&frontier, expected_block_number, manifest.block_hash)?;
    Ok(frontier)
}

fn validate_predecessor_identity(
    predecessor: &PendingBlock,
    host_chain_id: i64,
    expected_block_number: i64,
    expected_revision: i64,
    manifest: &ManifestReference,
) -> Result<(), ExecutionError> {
    let stored_digest = b256(
        "previous manifest digest",
        predecessor
            .manifest_digest
            .as_deref()
            .ok_or_else(|| internal("published predecessor block has no manifest digest"))?,
    )?;
    if predecessor.block_number != expected_block_number
        || predecessor.block_hash.as_slice() != manifest.block_hash.as_slice()
        || predecessor.manifest_revision != expected_revision
        || !predecessor.manifest_published
        || stored_digest != manifest.manifest_digest
    {
        return Err(internal(format!(
            "predecessor block identity mismatch while reconstructing missing manifest for chain {host_chain_id} block {expected_block_number}",
        )));
    }
    Ok(())
}

fn validate_frontier_tip(
    frontier: &RangeFrontier,
    expected_block_number: i64,
    expected_block_hash: B256,
) -> Result<(), ExecutionError> {
    let tip = frontier.as_slice().last();
    if tip.map(|range| range.end) != Some(expected_block_number)
        || tip.is_some_and(|range| range.end_block_hash != expected_block_hash)
    {
        return Err(internal(format!(
            "reconstructed frontier does not end at previous manifest block {expected_block_number}",
        )));
    }
    validate_canonical_frontier(frontier.as_slice())
}

pub(super) fn historical_ranges(frontier: &RangeFrontier) -> Vec<HistoricalRange> {
    frontier
        .as_slice()
        .iter()
        .rev()
        .map(|range| HistoricalRange {
            start_block_number: non_negative_u256("range start", range.start)
                .expect("validated database range"),
            end_block_number: non_negative_u256("range end", range.end)
                .expect("validated database range"),
            scale: u32::try_from(range.scale).expect("validated database range scale"),
            end_block_hash: range.end_block_hash,
            digest: range.digest,
        })
        .collect()
}

pub(super) async fn append_detailed_blocks(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    frontier: &mut RangeFrontier,
    blocks: &[ManifestBlockEntry],
) -> Result<(), ExecutionError> {
    for block in blocks {
        let number = i64_from_u256("block number", block.block_number)?;
        persist_frontier_leaf(
            trx,
            host_chain_id,
            coprocessor_context_id,
            frontier,
            RangeNode {
                start: number,
                end: number,
                scale: 0,
                start_block_hash: block.block_hash,
                start_parent_block_hash: block.parent_block_hash,
                end_block_hash: block.block_hash,
                digest: block.block_content_digest,
            },
        )
        .await?;
    }
    Ok(())
}

async fn persist_frontier_leaf(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    frontier: &mut RangeFrontier,
    leaf: RangeNode,
) -> Result<(), ExecutionError> {
    persist_range(trx, host_chain_id, &leaf).await?;
    for parent in append_leaf(host_chain_id, coprocessor_context_id, frontier, leaf)? {
        persist_range(trx, host_chain_id, &parent).await?;
    }
    Ok(())
}

async fn persist_range(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    range: &RangeNode,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        INSERT INTO block_consensus_range (
            host_chain_id,
            range_start,
            range_end,
            scale,
            range_start_block_hash,
            range_start_parent_block_hash,
            range_end_block_hash,
            range_digest
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
        ON CONFLICT DO NOTHING
        "#,
        host_chain_id,
        range.start,
        range.end,
        range.scale,
        range.start_block_hash.as_slice(),
        range.start_parent_block_hash.as_slice(),
        range.end_block_hash.as_slice(),
        range.digest.as_slice(),
    )
    .execute(trx.as_mut())
    .await?;

    let stored = sqlx::query!(
        r#"
        SELECT scale,
               range_start_block_hash,
               range_start_parent_block_hash,
               range_digest
          FROM block_consensus_range
         WHERE host_chain_id = $1
           AND range_start = $2
           AND range_end = $3
           AND range_end_block_hash = $4
           AND range_digest = $5
        "#,
        host_chain_id,
        range.start,
        range.end,
        range.end_block_hash.as_slice(),
        range.digest.as_slice(),
    )
    .fetch_one(trx.as_mut())
    .await?;
    if stored.scale != range.scale
        || stored.range_start_block_hash.as_slice() != range.start_block_hash.as_slice()
        || stored.range_start_parent_block_hash.as_slice()
            != range.start_parent_block_hash.as_slice()
        || stored.range_digest.as_slice() != range.digest.as_slice()
    {
        return Err(internal(format!(
            "immutable dyadic range conflict for chain {} range [{}, {}]",
            host_chain_id, range.start, range.end,
        )));
    }
    Ok(())
}
