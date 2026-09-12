use std::collections::BTreeSet;

use alloy_primitives::{B256, U256};
use block_manifest::{block_content_digest, HistoricalRange, ManifestBlockEntry, ManifestVersion};
use sqlx::{Postgres, Transaction};
use tracing::warn;

use crate::manifest_consensus::ExecutionError;

use crate::manifest_consensus::{
    lineage::{RangeFrontier, RangeNode},
    manifest_archive::{load_manifest_by_reference, ManifestReference},
    publication::{
        block_discovery::PendingBlock,
        manifest_builder::{b20_address, b256, i64_from_u256, internal, non_negative_u256},
    },
};

use super::manifest_frontier::{
    append_leaf, derive_range_scale, range_size, rebuild_frontier_from_manifest,
    validate_canonical_frontier,
};

pub(crate) async fn load_detailed_lineage(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    coprocessor_context_id: U256,
) -> Result<(Vec<PendingBlock>, Option<ManifestReference>), ExecutionError> {
    // A generation is an independent verification epoch. Its first manifest
    // starts a new lineage instead of inheriting detailed blocks or compact
    // history from the preceding generation.
    let floor = lineage_floor(trx, &target.generation, target.host_chain_id).await?;
    let mut lineage = vec![target.clone()];
    let mut parent_hash = target.parent_block_hash.clone();
    let mut generation = target.generation.clone();
    let last_published_manifest = loop {
        if let Some(parent) =
            load_block_by_hash(trx, target.host_chain_id, &generation, &parent_hash).await?
        {
            if parent.manifest_published {
                break Some(manifest_reference(&parent)?);
            }
            if parent.block_content_digest.is_none() {
                return Err(predecessor_unsealed(
                    parent.host_chain_id,
                    parent.block_number,
                ));
            }
            parent_hash.clone_from(&parent.parent_block_hash);
            generation.clone_from(&parent.generation);
            lineage.push(parent);
            continue;
        }
        // Not in publication state. If the host block is still in-generation
        // and has producer inventory, insert it unsealed and wait for ordinary
        // sealing. True gaps (no producers) are empty-sealed. Below the
        // generation floor this is the origin.
        let Some(host) = load_host_block(trx, target.host_chain_id, &parent_hash).await? else {
            break None;
        };
        if host.block_number < floor {
            break None;
        }
        if host_block_has_producers(trx, target.host_chain_id, &host).await? {
            insert_unsealed_lineage_block(
                trx,
                &generation,
                target.host_chain_id,
                &host,
                target.publication_cadence,
            )
            .await?;
            return Err(predecessor_unsealed(
                target.host_chain_id,
                host.block_number,
            ));
        }
        insert_empty_lineage_block(
            trx,
            &generation,
            target.host_chain_id,
            &host,
            target.publication_cadence,
            coprocessor_context_id,
        )
        .await?;
        let Some(parent) =
            load_block_by_hash(trx, target.host_chain_id, &generation, &parent_hash).await?
        else {
            break None;
        };
        parent_hash.clone_from(&parent.parent_block_hash);
        generation.clone_from(&parent.generation);
        lineage.push(parent);
    };
    lineage.reverse();
    validate_contiguous_lineage(&lineage)?;
    Ok((lineage, last_published_manifest))
}

struct HostBlock {
    block_number: i64,
    block_hash: Vec<u8>,
    parent_hash: Vec<u8>,
}

async fn lineage_floor(
    trx: &mut Transaction<'_, Postgres>,
    generation: &str,
    host_chain_id: i64,
) -> Result<i64, ExecutionError> {
    if let Some(start_block) = sqlx::query_scalar!(
        r#"
        SELECT start_block
          FROM generation_block_window
         WHERE generation = $1
           AND host_chain_id = $2
        "#,
        generation,
        host_chain_id,
    )
    .fetch_optional(trx.as_mut())
    .await?
    {
        return Ok(start_block);
    }
    sqlx::query_scalar!(
        r#"
        SELECT MIN(block_number) AS "floor?"
          FROM block_manifest_state
         WHERE generation = $1
           AND host_chain_id = $2
        "#,
        generation,
        host_chain_id,
    )
    .fetch_one(trx.as_mut())
    .await?
    .ok_or_else(|| internal("generation has no publication state to bound lineage"))
}

async fn load_host_block(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    block_hash: &[u8],
) -> Result<Option<HostBlock>, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT block_number,
               block_hash,
               parent_hash
          FROM host_chain_blocks_valid
         WHERE chain_id = $1
           AND block_hash = $2
           AND block_status <> 'orphaned'
           AND OCTET_LENGTH(parent_hash) = 32
         LIMIT 1
        "#,
        host_chain_id,
        block_hash,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    Ok(row.map(|row| HostBlock {
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_hash: row
            .parent_hash
            .expect("queried host blocks have a 32-byte parent hash"),
    }))
}

async fn host_block_has_producers(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    host: &HostBlock,
) -> Result<bool, ExecutionError> {
    let exists = sqlx::query_scalar!(
        r#"
        SELECT EXISTS (
            SELECT 1
              FROM handle_producer_block
             WHERE host_chain_id = $1
               AND producer_block_number = $2
               AND producer_block_hash = $3
        ) AS "exists!"
        "#,
        host_chain_id,
        host.block_number,
        &host.block_hash,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(exists)
}

async fn insert_unsealed_lineage_block(
    trx: &mut Transaction<'_, Postgres>,
    generation: &str,
    host_chain_id: i64,
    host: &HostBlock,
    publication_cadence: i64,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        INSERT INTO block_manifest_state (
            generation,
            host_chain_id,
            block_number,
            block_hash,
            parent_block_hash,
            publication_cadence
        )
        VALUES ($1, $2, $3, $4, $5, $6)
        ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
        "#,
        generation,
        host_chain_id,
        host.block_number,
        &host.block_hash,
        &host.parent_hash,
        publication_cadence,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

fn predecessor_unsealed(host_chain_id: i64, block_number: i64) -> ExecutionError {
    ExecutionError::PredecessorUnsealed {
        host_chain_id,
        block_number,
    }
}

async fn insert_empty_lineage_block(
    trx: &mut Transaction<'_, Postgres>,
    generation: &str,
    host_chain_id: i64,
    host: &HostBlock,
    publication_cadence: i64,
    coprocessor_context_id: U256,
) -> Result<(), ExecutionError> {
    let digest = block_content_digest(
        ManifestVersion::V1,
        coprocessor_context_id,
        non_negative_u256("host chain id", host_chain_id)?,
        non_negative_u256("block number", host.block_number)?,
        b256("block hash", &host.block_hash)?,
        &[],
    )
    .map_err(|err| internal(err.to_string()))?;
    sqlx::query!(
        r#"
        INSERT INTO block_manifest_state (
            generation,
            host_chain_id,
            block_number,
            block_hash,
            parent_block_hash,
            publication_cadence,
            block_content_digest,
            block_handle_count
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, 0)
        ON CONFLICT (generation, host_chain_id, block_hash) DO NOTHING
        "#,
        generation,
        host_chain_id,
        host.block_number,
        &host.block_hash,
        &host.parent_hash,
        publication_cadence,
        digest.as_slice(),
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

fn manifest_reference(block: &PendingBlock) -> Result<ManifestReference, ExecutionError> {
    Ok(ManifestReference {
        generation: block.generation.clone(),
        publisher: b20_address(
            "previous manifest publisher",
            block
                .manifest_publisher
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
    generation: &str,
    block_hash: &[u8],
) -> Result<Option<PendingBlock>, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT generation,
               host_chain_id,
               block_number,
               block_hash,
               parent_block_hash,
               publication_cadence,
               block_content_digest,
               block_handle_count,
               manifest_revision,
               manifest_publisher,
               manifest_digest,
               manifest_published
          FROM block_manifest_state
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND generation = $3
         LIMIT 1
        "#,
        host_chain_id,
        block_hash,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    Ok(row.map(|row| PendingBlock {
        generation: row.generation,
        host_chain_id: row.host_chain_id,
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_block_hash: row.parent_block_hash,
        publication_cadence: row.publication_cadence,
        block_content_digest: row.block_content_digest,
        block_handle_count: row.block_handle_count,
        manifest_revision: row.manifest_revision,
        manifest_publisher: row.manifest_publisher,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

pub(crate) async fn load_frontier(
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
    let generation = archived.signed.payload.consensus_epoch.as_str();
    for range in reconstructed_ranges {
        persist_range(trx, host_chain_id, generation, &range).await?;
    }

    validate_frontier_tip(&frontier, block_number, manifest.block_hash)?;
    Ok(frontier)
}

async fn hydrate_historical_ranges(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    payload: &block_manifest::ManifestPayload,
) -> Result<Vec<RangeNode>, ExecutionError> {
    // Wire history is newest-to-oldest. The merge algorithm uses the reverse
    // order and also needs boundary hashes omitted from the compact wire form.
    let mut ranges = Vec::with_capacity(payload.historical_ranges.len());
    for (index, historical) in payload.historical_ranges.iter().rev().enumerate() {
        let start = i64_from_u256("historical range start", historical.start_block_number)?;
        let end = i64_from_u256("historical range end", historical.end_block_number)?;
        let manifest_scale = i32::try_from(historical.scale)
            .map_err(|_| internal("historical range scale exceeds INTEGER"))?;
        let valid_scale = if index == 0 {
            let virtual_start = end
                .checked_add(1)
                .and_then(|upper| upper.checked_sub(range_size(manifest_scale).ok()?));
            virtual_start.is_some_and(|virtual_start| start >= virtual_start && start <= end)
        } else {
            derive_range_scale(start, end).ok() == Some(manifest_scale)
        };
        if !valid_scale {
            return Err(internal(format!(
                "historical range [{start}, {end}] is invalid at scale {manifest_scale}",
            )));
        }
        let boundary = sqlx::query!(
            r#"
            SELECT range_start_block_hash,
                   range_start_parent_block_hash
              FROM block_range_commitment
             WHERE host_chain_id = $1
               AND generation = $6
               AND range_start = $2
               AND range_end = $3
               AND range_end_block_hash = $4
               AND range_digest = $5
            "#,
            host_chain_id,
            start,
            end,
            historical.end_block_hash.as_slice(),
            historical.digest.as_slice(),
            payload.consensus_epoch.as_str(),
        )
        .fetch_optional(trx.as_mut())
        .await?
        .ok_or_else(|| {
            internal(format!(
                "BlockRange [{start}, {end}] referenced by the previous manifest is missing",
            ))
        })?;
        ranges.push(RangeNode {
            start_block_number: start,
            end,
            scale: manifest_scale,
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
    let manifest_generation = manifest.generation.clone();
    let mut generation = manifest_generation.clone();
    let mut visited = BTreeSet::new();
    let mut reverse_lineage = Vec::new();

    loop {
        if !visited.insert(next_hash.clone()) {
            return Err(internal(format!(
                "cycle while reconstructing block lineage ending at chain {host_chain_id} block {expected_block_number}",
            )));
        }
        let Some(block) = load_block_by_hash(trx, host_chain_id, &generation, &next_hash).await?
        else {
            break;
        };
        next_hash.clone_from(&block.parent_block_hash);
        generation.clone_from(&block.generation);
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
            &manifest_generation,
            &mut frontier,
            RangeNode {
                start_block_number: block.block_number,
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

pub(crate) fn historical_ranges(frontier: &RangeFrontier) -> Vec<HistoricalRange> {
    frontier
        .as_slice()
        .iter()
        .rev()
        .map(|range| HistoricalRange {
            start_block_number: non_negative_u256("range start", range.start_block_number)
                .expect("validated database range"),
            end_block_number: non_negative_u256("range end", range.end)
                .expect("validated database range"),
            scale: u32::try_from(range.scale).expect("validated database range scale"),
            end_block_hash: range.end_block_hash,
            digest: range.digest,
        })
        .collect()
}

pub(crate) async fn append_detailed_blocks(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    generation: &str,
    frontier: &mut RangeFrontier,
    blocks: &[ManifestBlockEntry],
) -> Result<(), ExecutionError> {
    for block in blocks {
        let number = i64_from_u256("block number", block.block_number)?;
        persist_frontier_leaf(
            trx,
            host_chain_id,
            coprocessor_context_id,
            generation,
            frontier,
            RangeNode {
                start_block_number: number,
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
    generation: &str,
    frontier: &mut RangeFrontier,
    leaf: RangeNode,
) -> Result<(), ExecutionError> {
    persist_range(trx, host_chain_id, generation, &leaf).await?;
    for parent in append_leaf(host_chain_id, coprocessor_context_id, frontier, leaf)? {
        persist_range(trx, host_chain_id, generation, &parent).await?;
    }
    Ok(())
}

async fn persist_range(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    generation: &str,
    range: &RangeNode,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        INSERT INTO block_range_commitment (
            generation,
            host_chain_id,
            range_start,
            range_end,
            range_start_block_hash,
            range_start_parent_block_hash,
            range_end_block_hash,
            range_digest
        )
        VALUES ($8, $1, $2, $3, $4, $5, $6, $7)
        ON CONFLICT DO NOTHING
        "#,
        host_chain_id,
        range.start_block_number,
        range.end,
        range.start_block_hash.as_slice(),
        range.start_parent_block_hash.as_slice(),
        range.end_block_hash.as_slice(),
        range.digest.as_slice(),
        generation,
    )
    .execute(trx.as_mut())
    .await?;

    let stored = sqlx::query!(
        r#"
        SELECT range_start_block_hash,
               range_start_parent_block_hash,
               range_digest
          FROM block_range_commitment
         WHERE host_chain_id = $1
           AND generation = $6
           AND range_start = $2
           AND range_end = $3
           AND range_end_block_hash = $4
           AND range_digest = $5
        "#,
        host_chain_id,
        range.start_block_number,
        range.end,
        range.end_block_hash.as_slice(),
        range.digest.as_slice(),
        generation,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if stored.range_start_block_hash.as_slice() != range.start_block_hash.as_slice()
        || stored.range_start_parent_block_hash.as_slice()
            != range.start_parent_block_hash.as_slice()
        || stored.range_digest.as_slice() != range.digest.as_slice()
    {
        return Err(internal(format!(
            "immutable dyadic range conflict for chain {} range [{}, {}]",
            host_chain_id, range.start_block_number, range.end,
        )));
    }
    Ok(())
}
