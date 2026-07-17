use crate::{
    consensus::{
        lineage::{RangeFrontier, RangeNode},
        manifest_archive::{load_manifest_by_reference, load_manifest_revision},
    },
    ExecutionError,
};
use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::{
    manifest::{
        block_content_digest, detailed_range_digest, dyadic_range_digest,
        BlockCiphertextDescriptor, DetailedRange, HistoricalRange, ManifestBlockEntry,
        ManifestPayload, ManifestReference, ManifestVersion,
    },
    CiphertextFormat,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::{BTreeMap, BTreeSet, HashMap};
use tracing::{error, warn};

const MANIFEST_ADVISORY_LOCK_DOMAIN: i64 = 0x4d41_4e49_4645_5354;

#[derive(Clone, Debug)]
pub(crate) struct PendingBlock {
    pub host_chain_id: i64,
    pub block_number: i64,
    pub block_hash: Vec<u8>,
    pub parent_block_hash: Vec<u8>,
    pub block_content_digest: Option<Vec<u8>>,
    pub descriptor_count: Option<i64>,
    pub manifest_revision: i64,
    pub manifest_digest: Option<Vec<u8>>,
    pub manifest_published: bool,
}

#[derive(Debug)]
pub(crate) struct PreparedManifest {
    pub payload: ManifestPayload,
    pub next_frontier: RangeFrontier,
}

pub(crate) type CiphertextDescriptor = BlockCiphertextDescriptor;

pub(crate) async fn pending_chain_ids(
    pool: &PgPool,
    publication_cadence: i64,
) -> Result<Vec<i64>, ExecutionError> {
    validate_cadence(publication_cadence)?;
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT host_chain_id
          FROM block_consensus
         WHERE block_content_digest IS NULL
            OR (
                manifest_published = FALSE
                AND MOD(block_number, $1) = 0
            )
         ORDER BY host_chain_id
        "#,
        publication_cadence,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|row| row.host_chain_id).collect())
}

/// Takes the chain-scoped transaction lock before selecting its earliest work.
/// This prevents another publisher from skipping a locked earlier block on the
/// same lineage and publishing a later commitment.
pub(crate) async fn lock_next_block_to_progress(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    publication_cadence: i64,
) -> Result<Option<PendingBlock>, ExecutionError> {
    validate_cadence(publication_cadence)?;
    let advisory_key = host_chain_id ^ MANIFEST_ADVISORY_LOCK_DOMAIN;
    let lock = sqlx::query!(
        r#"SELECT pg_try_advisory_xact_lock($1) AS "locked!""#,
        advisory_key,
    )
    .fetch_one(trx.as_mut())
    .await?;
    if !lock.locked {
        return Ok(None);
    }

    let row = sqlx::query!(
        r#"
        SELECT host_chain_id,
               block_number,
               block_hash,
               parent_block_hash,
               block_content_digest,
               descriptor_count,
               manifest_revision,
               manifest_digest,
               manifest_published
          FROM block_consensus
         WHERE host_chain_id = $1
           AND (
                block_content_digest IS NULL
                OR (
                    manifest_published = FALSE
                    AND MOD(block_number, $2) = 0
                )
           )
         ORDER BY block_number, block_hash
         LIMIT 1
           FOR UPDATE
        "#,
        host_chain_id,
        publication_cadence,
    )
    .fetch_optional(trx.as_mut())
    .await?;

    Ok(row.map(|row| PendingBlock {
        host_chain_id: row.host_chain_id,
        block_number: row.block_number,
        block_hash: row.block_hash,
        parent_block_hash: row.parent_block_hash,
        block_content_digest: row.block_content_digest,
        descriptor_count: row.descriptor_count,
        manifest_revision: row.manifest_revision,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

/// Discovers one new generation after every known commitment row. Repeating
/// this cheap, idempotent query lets empty blocks enter the lineage even though
/// no ciphertext upload could have seeded them directly.
pub(crate) async fn discover_known_children(pool: &PgPool) -> Result<u64, ExecutionError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO block_consensus (
            host_chain_id,
            block_number,
            block_hash,
            parent_block_hash
        )
        SELECT child.chain_id,
               child.block_number,
               child.block_hash,
               child.parent_hash
          FROM host_chain_blocks_valid child
          JOIN block_consensus parent
            ON parent.host_chain_id = child.chain_id
           AND parent.block_hash = child.parent_hash
         WHERE child.parent_hash IS NOT NULL
           AND OCTET_LENGTH(child.parent_hash) = 32
           AND child.block_status <> 'orphaned'
        ON CONFLICT (host_chain_id, block_hash) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;
    Ok(result.rows_affected())
}

pub(crate) async fn discover_block_children(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<u64, ExecutionError> {
    let result = sqlx::query!(
        r#"
        INSERT INTO block_consensus (
            host_chain_id,
            block_number,
            block_hash,
            parent_block_hash
        )
        SELECT child.chain_id,
               child.block_number,
               child.block_hash,
               child.parent_hash
          FROM host_chain_blocks_valid child
         WHERE child.chain_id = $1
           AND child.parent_hash = $2
           AND OCTET_LENGTH(child.parent_hash) = 32
           AND child.block_status <> 'orphaned'
        ON CONFLICT (host_chain_id, block_hash) DO NOTHING
        "#,
        block.host_chain_id,
        &block.block_hash,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(result.rows_affected())
}

/// Returns whether every allowed ciphertext from the block is ready for the
/// manifest. Errors are reported separately from work that is still pending.
pub(crate) async fn is_block_manifest_ready(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<bool, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH block_operations AS MATERIALIZED (
            SELECT c.output_handle AS handle,
                   c.is_completed AS operation_completed,
                   c.is_error AS operation_error,
                   p.is_completed AS sns_completed,
                   p.is_error AS sns_error,
                   d.key_id_gw,
                   d.ciphertext AS ct64_digest,
                   d.ciphertext128 AS ct128_digest,
                   d.ciphertext128_format AS ct128_format
              FROM computations_branch c
              LEFT JOIN pbs_computations_branch p
                ON p.host_chain_id = c.host_chain_id
               AND p.handle = c.output_handle
               AND p.producer_block_hash = c.producer_block_hash
               AND p.block_hash = $3
              LEFT JOIN ciphertext_digest_branch d
                ON d.host_chain_id = c.host_chain_id
               AND d.handle = c.output_handle
               AND d.producer_block_hash = c.producer_block_hash
               AND d.block_hash = $3
             WHERE c.host_chain_id = $1
               AND c.block_number = $2
               AND c.producer_block_hash = $3
               AND c.is_allowed = TRUE
        )
        SELECT (
                   SELECT handle
                     FROM block_operations
                    WHERE operation_error
                       OR sns_error IS TRUE
                    ORDER BY handle
                    LIMIT 1
               ) AS "error_handle?",
               NOT EXISTS (
                   SELECT 1
                     FROM block_operations
                    WHERE NOT operation_completed
                       OR sns_completed IS DISTINCT FROM TRUE
                       OR key_id_gw IS NULL
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

    if let Some(error_handle) = row.error_handle {
        return Err(ExecutionError::InternalError(format!(
            "failed allowed handle {} in chain {} block {}",
            hex::encode(error_handle),
            block.host_chain_id,
            block.block_number,
        )));
    }

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
        SELECT c.output_handle AS "handle!",
               d.key_id_gw AS "key_id_gw?",
               d.ciphertext AS "ct64_digest?",
               d.ciphertext128 AS "ct128_digest?",
               d.ciphertext128_format AS "ct128_format?"
          FROM computations_branch c
          LEFT JOIN ciphertext_digest_branch d
            ON d.host_chain_id = c.host_chain_id
           AND d.handle = c.output_handle
           AND d.producer_block_hash = c.producer_block_hash
           AND d.block_hash = $3
         WHERE c.host_chain_id = $1
           AND c.block_number = $2
           AND c.producer_block_hash = $3
           AND c.is_allowed = TRUE
         ORDER BY c.output_handle, c.transaction_id
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
            descriptor_count = block.descriptor_count,
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
    let descriptor_count = i64::try_from(descriptors.len())
        .map_err(|_| internal("manifest descriptor count exceeds BIGINT"))?;

    let result = sqlx::query!(
        r#"
        UPDATE block_consensus
           SET block_content_digest = $3,
               descriptor_count = $4,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND block_content_digest IS NULL
           AND descriptor_count IS NULL
        "#,
        block.host_chain_id,
        &block.block_hash,
        digest.as_slice(),
        descriptor_count,
    )
    .execute(trx.as_mut())
    .await?;

    if result.rows_affected() != 1 {
        let current = sqlx::query!(
            r#"
            SELECT block_content_digest,
                   descriptor_count,
                   manifest_revision,
                   manifest_published
              FROM block_consensus
             WHERE host_chain_id = $1
               AND block_hash = $2
            "#,
            block.host_chain_id,
            &block.block_hash,
        )
        .fetch_optional(trx.as_mut())
        .await?;
        let failure_reason = match current.as_ref() {
            None => "row_missing",
            Some(row) if row.block_content_digest.is_some() && row.descriptor_count.is_some() => {
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
            stored_descriptor_count = ?current.as_ref().and_then(|row| row.descriptor_count),
            "Block content sealing updated no row"
        );
        return Err(internal(format!(
            "block content sealing updated no row for chain {} block {}: {failure_reason}",
            block.host_chain_id, block.block_number,
        )));
    }

    Ok(digest)
}

/// Reopens one already published local manifest after its block material was
/// replayed. The signed body of the previous revision remains immutable in the
/// manifest archive and is used as the new revision's `supersedes` reference.
///
/// Callers must update the underlying ciphertext material in the same
/// transaction before requesting the revision. Dependent later publication
/// blocks must be queued separately in parent-before-child order.
#[allow(
    dead_code,
    reason = "production remediation will call this revision transition"
)]
pub(crate) async fn queue_manifest_revision_after_replay(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    block_hash: B256,
    expected_revision: i64,
) -> Result<i64, ExecutionError> {
    if expected_revision < 0 {
        return Err(internal("expected manifest revision is negative"));
    }
    let next_revision = expected_revision
        .checked_add(1)
        .ok_or_else(|| internal("manifest revision overflow"))?;
    let row = sqlx::query!(
        r#"
        UPDATE block_consensus
           SET block_content_digest = NULL,
               descriptor_count = NULL,
               detailed_range_start = NULL,
               detailed_range_digest = NULL,
               manifest_revision = $4,
               manifest_digest = NULL,
               manifest_published = FALSE,
               manifest_published_at = NULL,
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND manifest_revision = $3
           AND manifest_digest IS NOT NULL
           AND manifest_published = TRUE
        RETURNING manifest_revision
        "#,
        host_chain_id,
        block_hash.as_slice(),
        expected_revision,
        next_revision,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    row.map(|row| row.manifest_revision).ok_or_else(|| {
        internal(format!(
            "manifest replay could not queue revision {next_revision} for chain {host_chain_id} block {block_hash}",
        ))
    })
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
    let supersedes = if target.manifest_revision == 0 {
        None
    } else {
        let previous_revision = target
            .manifest_revision
            .checked_sub(1)
            .ok_or_else(|| internal("manifest revision underflow"))?;
        let previous = load_manifest_revision(
            trx,
            publisher,
            ManifestVersion::V1,
            coprocessor_context_id,
            target.host_chain_id,
            target.block_number,
            b256("block hash", &target.block_hash)?,
            previous_revision,
        )
        .await?
        .ok_or_else(|| {
            internal(format!(
                "manifest revision {} cannot supersede missing revision {} for chain {} block {}",
                target.manifest_revision,
                previous_revision,
                target.host_chain_id,
                target.block_number,
            ))
        })?;
        Some(ManifestReference {
            block_number: previous.signed.payload.publication_block_number,
            block_hash: previous.signed.payload.publication_block_hash,
            revision: previous.signed.payload.revision,
            manifest_digest: previous.digest,
        })
    };

    let (lineage, previous_manifest) = load_detailed_lineage(trx, target).await?;
    let mut blocks = Vec::with_capacity(lineage.len());
    for block in &lineage {
        let descriptors = load_manifest_descriptors(trx, block).await?;
        let stored_count = block
            .descriptor_count
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
        let recomputed = block_content_digest(
            ManifestVersion::V1,
            coprocessor_context_id,
            non_negative_u256("host chain id", block.host_chain_id)?,
            non_negative_u256("block number", block.block_number)?,
            b256("block hash", &block.block_hash)?,
            &descriptors,
        )
        .map_err(|err| internal(err.to_string()))?;
        if stored_digest != recomputed {
            return Err(internal(format!(
                "stored block digest conflicts with descriptors for chain {} block {}",
                block.host_chain_id, block.block_number,
            )));
        }

        blocks.push(ManifestBlockEntry {
            block_number: non_negative_u256("block number", block.block_number)?,
            block_hash: b256("block hash", &block.block_hash)?,
            parent_block_hash: b256("parent block hash", &block.parent_block_hash)?,
            block_content_digest: stored_digest,
            ciphertexts: descriptors,
        });
    }

    let first = blocks
        .first()
        .ok_or_else(|| internal("empty detailed range"))?;
    let last = blocks.last().expect("checked non-empty");
    let block_digests: Vec<_> = blocks
        .iter()
        .map(|block| block.block_content_digest)
        .collect();
    let host_chain_id = non_negative_u256("host chain id", target.host_chain_id)?;
    let detailed_digest = detailed_range_digest(
        ManifestVersion::V1,
        coprocessor_context_id,
        host_chain_id,
        first.block_number,
        last.block_number,
        &block_digests,
    );

    let mut frontier = match previous_manifest.as_ref() {
        Some(previous) => {
            load_frontier_from_previous_manifest(
                trx,
                target.host_chain_id,
                coprocessor_context_id,
                publisher,
                previous,
            )
            .await?
        }
        None => RangeFrontier::default(),
    };
    let historical_ranges = frontier
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
        .collect();

    for block in &blocks {
        let number = i64::try_from(block.block_number)
            .map_err(|_| internal("block number exceeds BIGINT"))?;
        let leaf = RangeNode {
            start: number,
            end: number,
            scale: 0,
            start_block_hash: block.block_hash,
            start_parent_block_hash: block.parent_block_hash,
            end_block_hash: block.block_hash,
            digest: block.block_content_digest,
        };
        append_leaf_to_frontier(
            trx,
            target.host_chain_id,
            coprocessor_context_id,
            &mut frontier,
            leaf,
        )
        .await?;
    }

    let payload = ManifestPayload {
        version: ManifestVersion::V1,
        publisher,
        coprocessor_context_id,
        host_chain_id,
        publication_block_number: non_negative_u256("block number", target.block_number)?,
        publication_block_hash: b256("block hash", &target.block_hash)?,
        publication_parent_block_hash: b256("parent block hash", &target.parent_block_hash)?,
        revision,
        supersedes,
        detailed_range: DetailedRange {
            first_block_number: first.block_number,
            last_block_number: last.block_number,
            digest: detailed_digest,
            blocks,
        },
        historical_ranges,
        full_consensus_checkpoint: None,
        previous_manifest,
    };
    payload
        .validate()
        .map_err(|err| internal(format!("prepared manifest is invalid: {err}")))?;

    Ok(PreparedManifest {
        payload,
        next_frontier: frontier,
    })
}

pub(crate) async fn mark_manifest_published(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    detailed_range_start: i64,
    detailed_range_digest: B256,
    manifest_digest: B256,
) -> Result<(), ExecutionError> {
    let result = sqlx::query!(
        r#"
        UPDATE block_consensus
           SET detailed_range_start = $3,
               detailed_range_digest = $4,
               manifest_digest = $5,
               manifest_published = TRUE,
               manifest_published_at = NOW(),
               updated_at = NOW()
         WHERE host_chain_id = $1
           AND block_hash = $2
           AND manifest_revision = $6
           AND manifest_digest IS NULL
           AND manifest_published = FALSE
        "#,
        target.host_chain_id,
        &target.block_hash,
        detailed_range_start,
        detailed_range_digest.as_slice(),
        manifest_digest.as_slice(),
        target.manifest_revision,
    )
    .execute(trx.as_mut())
    .await?;
    if result.rows_affected() != 1 {
        return Err(internal(format!(
            "manifest publication updated no row for chain {} block {}",
            target.host_chain_id, target.block_number,
        )));
    }
    Ok(())
}

async fn load_detailed_lineage(
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
            let digest = b256(
                "previous manifest digest",
                parent
                    .manifest_digest
                    .as_deref()
                    .ok_or_else(|| internal("published manifest row has no digest"))?,
            )?;
            break Some(ManifestReference {
                block_number: non_negative_u256("block number", parent.block_number)?,
                block_hash: b256("block hash", &parent.block_hash)?,
                revision: u64::try_from(parent.manifest_revision)
                    .map_err(|_| internal("negative manifest revision"))?,
                manifest_digest: digest,
            });
        }
        parent_hash = parent.parent_block_hash.clone();
        lineage.push(parent);
    };
    lineage.reverse();

    for pair in lineage.windows(2) {
        if pair[0].block_number.checked_add(1) != Some(pair[1].block_number)
            || pair[0].block_hash != pair[1].parent_block_hash
        {
            return Err(internal("detailed manifest lineage is not contiguous"));
        }
    }
    Ok((lineage, previous_manifest))
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
               block_content_digest,
               descriptor_count,
               manifest_revision,
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
        block_content_digest: row.block_content_digest,
        descriptor_count: row.descriptor_count,
        manifest_revision: row.manifest_revision,
        manifest_digest: row.manifest_digest,
        manifest_published: row.manifest_published,
    }))
}

async fn load_frontier_from_previous_manifest(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    publisher: Address,
    manifest: &ManifestReference,
) -> Result<RangeFrontier, ExecutionError> {
    let block_number = i64::try_from(manifest.block_number)
        .map_err(|_| internal("manifest block number exceeds BIGINT"))?;
    let manifest_revision = i64::try_from(manifest.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let archived = load_manifest_by_reference(
        trx,
        publisher,
        ManifestVersion::V1,
        coprocessor_context_id,
        host_chain_id,
        manifest,
    )
    .await?;
    let Some(archived) = archived else {
        warn!(
            host_chain_id,
            block_number,
            manifest_revision,
            manifest_digest = %manifest.manifest_digest,
            "Stored previous manifest is missing; rebuilding its frontier from block lineage"
        );
        return reconstruct_frontier_from_block_lineage(
            trx,
            host_chain_id,
            coprocessor_context_id,
            manifest,
        )
        .await;
    };
    let signed = archived.signed;

    // Historical entries are encoded newest-to-oldest. Hydrate their omitted
    // starting boundary hashes from the immutable BlockRange rows and restore
    // the in-memory order expected by the merge algorithm.
    let mut ranges = Vec::with_capacity(signed.payload.historical_ranges.len());
    for historical in signed.payload.historical_ranges.iter().rev() {
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

    let (frontier, reconstructed_ranges) = frontier_from_previous_payload(&signed.payload, ranges)?;
    for range in reconstructed_ranges {
        persist_range_root(trx, host_chain_id, &range).await?;
    }

    if frontier.as_slice().last().map(|range| range.end) != Some(block_number)
        || frontier
            .as_slice()
            .last()
            .is_some_and(|range| range.end_block_hash != manifest.block_hash)
    {
        return Err(internal(format!(
            "reconstructed frontier does not end at previous manifest block {block_number}",
        )));
    }
    validate_frontier(frontier.as_slice())?;
    Ok(frontier)
}

/// Rebuilds the predecessor's canonical frontier when its signed local body
/// has been lost. The predecessor reference remains authoritative: its block
/// row must still carry exactly the published identity and digest referenced
/// by the successor. Every dyadic root is then recomputed from sealed block
/// leaves and checked by the immutable range persistence path.
async fn reconstruct_frontier_from_block_lineage(
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
        next_hash = block.parent_block_hash.clone();
        reverse_lineage.push(block);
    }

    let predecessor = reverse_lineage.first().ok_or_else(|| {
        internal(format!(
            "cannot reconstruct missing previous manifest for chain {host_chain_id} block {expected_block_number}: predecessor block is missing",
        ))
    })?;
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

    reverse_lineage.reverse();
    for pair in reverse_lineage.windows(2) {
        if pair[0].block_number.checked_add(1) != Some(pair[1].block_number)
            || pair[0].block_hash != pair[1].parent_block_hash
        {
            return Err(internal(format!(
                "non-contiguous block lineage while reconstructing missing manifest for chain {host_chain_id} block {expected_block_number}",
            )));
        }
    }

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
        let leaf = RangeNode {
            start: block.block_number,
            end: block.block_number,
            scale: 0,
            start_block_hash: b256("block hash", &block.block_hash)?,
            start_parent_block_hash: b256("parent block hash", &block.parent_block_hash)?,
            end_block_hash: b256("block hash", &block.block_hash)?,
            digest,
        };
        append_leaf_to_frontier(
            trx,
            host_chain_id,
            coprocessor_context_id,
            &mut frontier,
            leaf,
        )
        .await?;
    }

    if frontier.as_slice().last().map(|range| range.end) != Some(expected_block_number)
        || frontier
            .as_slice()
            .last()
            .is_some_and(|range| range.end_block_hash != manifest.block_hash)
    {
        return Err(internal(format!(
            "fully reconstructed frontier does not end at previous manifest block {expected_block_number}",
        )));
    }
    validate_frontier(frontier.as_slice())?;
    Ok(frontier)
}

async fn append_leaf_to_frontier(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    coprocessor_context_id: U256,
    frontier: &mut RangeFrontier,
    leaf: RangeNode,
) -> Result<(), ExecutionError> {
    persist_range_root(trx, host_chain_id, &leaf).await?;
    let parents =
        advance_frontier_in_memory(host_chain_id, coprocessor_context_id, frontier, leaf)?;
    for parent in parents {
        persist_range_root(trx, host_chain_id, &parent).await?;
    }
    Ok(())
}

fn frontier_from_previous_payload(
    previous: &ManifestPayload,
    historical_ranges: Vec<RangeNode>,
) -> Result<(RangeFrontier, Vec<RangeNode>), ExecutionError> {
    if historical_ranges.len() != previous.historical_ranges.len() {
        return Err(internal(
            "hydrated BlockRange count does not match the previous manifest",
        ));
    }
    for (range, historical) in historical_ranges
        .iter()
        .zip(previous.historical_ranges.iter().rev())
    {
        if range.start != i64_from_u256("historical range start", historical.start_block_number)?
            || range.end != i64_from_u256("historical range end", historical.end_block_number)?
            || range.scale
                != i32::try_from(historical.scale)
                    .map_err(|_| internal("historical range scale exceeds INTEGER"))?
            || range.end_block_hash != historical.end_block_hash
            || range.digest != historical.digest
        {
            return Err(internal(
                "hydrated BlockRange does not match the previous manifest",
            ));
        }
    }

    let host_chain_id = i64_from_u256("host chain id", previous.host_chain_id)?;
    let mut frontier = RangeFrontier::from_ranges(historical_ranges);
    validate_frontier(frontier.as_slice())?;
    let mut reconstructed_ranges = Vec::new();
    for block in &previous.detailed_range.blocks {
        let number = i64_from_u256("block number", block.block_number)?;
        let leaf = RangeNode {
            start: number,
            end: number,
            scale: 0,
            start_block_hash: block.block_hash,
            start_parent_block_hash: block.parent_block_hash,
            end_block_hash: block.block_hash,
            digest: block.block_content_digest,
        };
        reconstructed_ranges.push(leaf.clone());
        reconstructed_ranges.extend(advance_frontier_in_memory(
            host_chain_id,
            previous.coprocessor_context_id,
            &mut frontier,
            leaf,
        )?);
    }
    validate_frontier(frontier.as_slice())?;
    Ok((frontier, reconstructed_ranges))
}

fn advance_frontier_in_memory(
    host_chain_id: i64,
    coprocessor_context_id: U256,
    frontier: &mut RangeFrontier,
    leaf: RangeNode,
) -> Result<Vec<RangeNode>, ExecutionError> {
    validate_frontier(frontier.as_slice())?;
    if leaf.scale != 0 || leaf.start != leaf.end {
        return Err(internal("only a size-one BlockRange can advance history"));
    }
    if let Some(newest) = frontier.as_slice().last() {
        if newest.end.checked_add(1) != Some(leaf.start)
            || newest.end_block_hash != leaf.start_parent_block_hash
        {
            return Err(internal(
                "new BlockRange is not contiguous with the canonical history",
            ));
        }
    }

    // The preceding canonical history plus the next leaf contains everything
    // needed to materialize the next canonical history. Parent ranges are built
    // lazily from child digests; no historical block digest is recomputed.
    let mut available = BTreeMap::new();
    for range in frontier.as_slice() {
        insert_available_range(&mut available, range.clone())?;
    }
    insert_available_range(&mut available, leaf.clone())?;

    let mut created_parents = Vec::new();
    let mut newest_to_oldest = Vec::new();
    let mut upper = leaf
        .end
        .checked_add(1)
        .ok_or_else(|| internal("BlockRange upper boundary overflow"))?;
    let mut previous_scale = 0;

    while upper > 0 {
        let scale = canonical_history_scale(upper, previous_scale)?;
        let size = range_size(scale)?;
        let Some(start) = upper.checked_sub(size) else {
            break;
        };
        let Some(range) = materialize_range(
            host_chain_id,
            coprocessor_context_id,
            start,
            scale,
            &mut available,
            &mut created_parents,
        )?
        else {
            // The first unavailable canonical range is the history boundary.
            // Older locally known fragments are intentionally not substituted,
            // because that would make the decomposition start-dependent.
            break;
        };
        newest_to_oldest.push(range);
        upper = start;
        previous_scale = scale;
    }

    newest_to_oldest.reverse();
    *frontier.as_mut_vec() = newest_to_oldest;
    validate_frontier(frontier.as_slice())?;
    Ok(created_parents)
}

type RangeKey = (i64, i32);

fn insert_available_range(
    available: &mut BTreeMap<RangeKey, RangeNode>,
    range: RangeNode,
) -> Result<(), ExecutionError> {
    let key = (range.start, range.scale);
    if let Some(existing) = available.get(&key) {
        if existing != &range {
            return Err(internal(format!(
                "conflicting BlockRange [{}, {}] at scale {}",
                range.start, range.end, range.scale,
            )));
        }
        return Ok(());
    }
    available.insert(key, range);
    Ok(())
}

fn materialize_range(
    host_chain_id: i64,
    coprocessor_context_id: U256,
    start: i64,
    scale: i32,
    available: &mut BTreeMap<RangeKey, RangeNode>,
    created_parents: &mut Vec<RangeNode>,
) -> Result<Option<RangeNode>, ExecutionError> {
    if let Some(range) = available.get(&(start, scale)) {
        return Ok(Some(range.clone()));
    }
    let Some(child_scale) = scale.checked_sub(1) else {
        return Ok(None);
    };
    if child_scale < 0 {
        return Ok(None);
    }
    let child_size = range_size(child_scale)?;
    let right_start = start
        .checked_add(child_size)
        .ok_or_else(|| internal("dyadic child boundary overflow"))?;
    let Some(left) = materialize_range(
        host_chain_id,
        coprocessor_context_id,
        start,
        child_scale,
        available,
        created_parents,
    )?
    else {
        return Ok(None);
    };
    let Some(right) = materialize_range(
        host_chain_id,
        coprocessor_context_id,
        right_start,
        child_scale,
        available,
        created_parents,
    )?
    else {
        return Ok(None);
    };
    if !left.is_left_sibling_of(&right) {
        return Err(internal(format!(
            "BlockRanges [{}, {}] and [{}, {}] are not lineage siblings",
            left.start, left.end, right.start, right.end,
        )));
    }

    let parent = RangeNode {
        start: left.start,
        end: right.end,
        scale,
        start_block_hash: left.start_block_hash,
        start_parent_block_hash: left.start_parent_block_hash,
        end_block_hash: right.end_block_hash,
        digest: dyadic_range_digest(
            ManifestVersion::V1,
            coprocessor_context_id,
            non_negative_u256("host chain id", host_chain_id)?,
            non_negative_u256("range start", left.start)?,
            non_negative_u256("range end", right.end)?,
            u32::try_from(scale).map_err(|_| internal("negative range scale"))?,
            right.end_block_hash,
            left.digest,
            right.digest,
        ),
    };
    insert_available_range(available, parent.clone())?;
    created_parents.push(parent.clone());
    Ok(Some(parent))
}

fn canonical_history_scale(upper: i64, previous_scale: i32) -> Result<i32, ExecutionError> {
    let larger_scale = previous_scale
        .checked_add(1)
        .ok_or_else(|| internal("dyadic range scale overflow"))?;
    let larger_size = range_size(larger_scale)?;
    Ok(if upper.rem_euclid(larger_size) == 0 {
        larger_scale
    } else {
        previous_scale
    })
}

fn range_size(scale: i32) -> Result<i64, ExecutionError> {
    let scale = u32::try_from(scale).map_err(|_| internal("negative dyadic range scale"))?;
    1_i64
        .checked_shl(scale)
        .ok_or_else(|| internal("dyadic range scale exceeds BIGINT"))
}

async fn persist_range_root(
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

fn validate_frontier(frontier: &[RangeNode]) -> Result<(), ExecutionError> {
    for range in frontier {
        let size = range_size(range.scale)?;
        if range
            .end
            .checked_sub(range.start)
            .and_then(|v| v.checked_add(1))
            != Some(size)
            || range.start.rem_euclid(size) != 0
        {
            return Err(internal(format!(
                "unaligned dyadic range [{}, {}] at scale {}",
                range.start, range.end, range.scale,
            )));
        }
    }
    for pair in frontier.windows(2) {
        if pair[0].end.checked_add(1) != Some(pair[1].start)
            || pair[0].end_block_hash != pair[1].start_parent_block_hash
        {
            return Err(internal("range frontier is not one contiguous lineage"));
        }
    }

    let Some(newest) = frontier.last() else {
        return Ok(());
    };
    let mut upper = newest
        .end
        .checked_add(1)
        .ok_or_else(|| internal("BlockRange upper boundary overflow"))?;
    let mut previous_scale = 0;
    for range in frontier.iter().rev() {
        let expected_scale = canonical_history_scale(upper, previous_scale)?;
        let expected_size = range_size(expected_scale)?;
        let expected_start = upper
            .checked_sub(expected_size)
            .ok_or_else(|| internal("canonical BlockRange starts before block zero"))?;
        if range.scale != expected_scale
            || range.start != expected_start
            || range.end.checked_add(1) != Some(upper)
        {
            return Err(internal(format!(
                "non-canonical BlockRange [{}, {}] at scale {}; expected [{}, {}] at scale {}",
                range.start,
                range.end,
                range.scale,
                expected_start,
                upper - 1,
                expected_scale,
            )));
        }
        upper = range.start;
        previous_scale = range.scale;
    }
    Ok(())
}

fn validate_cadence(publication_cadence: i64) -> Result<(), ExecutionError> {
    if publication_cadence <= 0 {
        return Err(internal(format!(
            "manifest publication cadence must be positive, got {publication_cadence}",
        )));
    }
    Ok(())
}

fn non_negative_u256(field: &str, value: i64) -> Result<U256, ExecutionError> {
    let value = u64::try_from(value)
        .map_err(|_| internal(format!("{field} must be non-negative, got {value}")))?;
    Ok(U256::from(value))
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let value: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(value))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}

#[cfg(test)]
mod tests {
    use super::*;
    use ciphertext_attestation::manifest::{DetailedRange, HistoricalRange};

    const HOST_CHAIN_ID: i64 = 7;

    fn hash(number: u8) -> B256 {
        B256::repeat_byte(number)
    }

    fn leaf(number: i64) -> RangeNode {
        RangeNode {
            start: number,
            end: number,
            scale: 0,
            start_block_hash: hash(number as u8),
            start_parent_block_hash: hash(number.wrapping_sub(1) as u8),
            end_block_hash: hash(number as u8),
            digest: B256::repeat_byte(number as u8 + 100),
        }
    }

    fn block(number: i64) -> ManifestBlockEntry {
        let block_hash = hash(number as u8);
        let digest = block_content_digest(
            ManifestVersion::V1,
            U256::ONE,
            U256::from(HOST_CHAIN_ID as u64),
            U256::from(number as u64),
            block_hash,
            &[],
        )
        .unwrap();
        ManifestBlockEntry {
            block_number: U256::from(number as u64),
            block_hash,
            parent_block_hash: hash(number.wrapping_sub(1) as u8),
            block_content_digest: digest,
            ciphertexts: Vec::new(),
        }
    }

    #[test]
    fn creates_frontier_from_scratch() {
        let mut frontier = RangeFrontier::default();
        let mut created = Vec::new();
        for number in 0..=2 {
            created.extend(
                advance_frontier_in_memory(HOST_CHAIN_ID, U256::ONE, &mut frontier, leaf(number))
                    .unwrap(),
            );
        }

        assert_eq!(created.len(), 1);
        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(0, 1, 1), (2, 2, 0)],
        );
    }

    #[test]
    fn rejects_a_non_canonical_dyadic_cover() {
        let ranges = vec![leaf(0), leaf(1), leaf(2)];
        assert!(validate_frontier(&ranges)
            .unwrap_err()
            .to_string()
            .contains("non-canonical BlockRange"),);
    }

    #[test]
    fn creates_the_unique_right_anchored_history() {
        let mut frontier = RangeFrontier::default();
        for number in 0..=7 {
            advance_frontier_in_memory(HOST_CHAIN_ID, U256::ONE, &mut frontier, leaf(number))
                .unwrap();
        }

        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .rev()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(6, 7, 1), (4, 5, 1), (0, 3, 2)],
        );
    }

    #[test]
    fn stops_before_the_first_unavailable_canonical_range() {
        let mut frontier = RangeFrontier::default();
        for number in 1..=7 {
            advance_frontier_in_memory(HOST_CHAIN_ID, U256::ONE, &mut frontier, leaf(number))
                .unwrap();
        }

        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .rev()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(6, 7, 1), (4, 5, 1)],
        );
    }

    #[test]
    fn recreates_frontier_from_previous_manifest() {
        let blocks = vec![block(2), block(3), block(4)];
        let block_digests = blocks
            .iter()
            .map(|block| block.block_content_digest)
            .collect::<Vec<_>>();
        let historical_digest = B256::repeat_byte(0x90);
        let previous = ManifestPayload {
            version: ManifestVersion::V1,
            publisher: Address::ZERO,
            coprocessor_context_id: U256::ONE,
            host_chain_id: U256::from(HOST_CHAIN_ID as u64),
            publication_block_number: U256::from(4),
            publication_block_hash: hash(4),
            publication_parent_block_hash: hash(3),
            revision: 0,
            supersedes: None,
            detailed_range: DetailedRange {
                first_block_number: U256::from(2),
                last_block_number: U256::from(4),
                digest: detailed_range_digest(
                    ManifestVersion::V1,
                    U256::ONE,
                    U256::from(HOST_CHAIN_ID as u64),
                    U256::from(2),
                    U256::from(4),
                    &block_digests,
                ),
                blocks,
            },
            historical_ranges: vec![HistoricalRange {
                start_block_number: U256::ZERO,
                end_block_number: U256::ONE,
                scale: 1,
                end_block_hash: hash(1),
                digest: historical_digest,
            }],
            full_consensus_checkpoint: None,
            previous_manifest: None,
        };
        previous.validate().unwrap();

        let historical = RangeNode {
            start: 0,
            end: 1,
            scale: 1,
            start_block_hash: hash(0),
            start_parent_block_hash: hash(u8::MAX),
            end_block_hash: hash(1),
            digest: historical_digest,
        };
        let (frontier, reconstructed) =
            frontier_from_previous_payload(&previous, vec![historical]).unwrap();

        assert_eq!(reconstructed.len(), 4);
        assert_eq!(
            frontier
                .as_slice()
                .iter()
                .map(|range| (range.start, range.end, range.scale))
                .collect::<Vec<_>>(),
            vec![(0, 1, 1), (2, 3, 1), (4, 4, 0)],
        );
        assert_eq!(frontier.as_slice().last().unwrap().end_block_hash, hash(4));
    }
}

#[cfg(test)]
#[path = "manifest_soak_tests.rs"]
mod soak_tests;
