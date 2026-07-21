use crate::{
    consensus::{
        lineage::RangeFrontier,
        manifest_archive::load_manifest_revision,
        manifest_history::{
            append_detailed_blocks, historical_ranges, load_detailed_lineage, load_frontier,
        },
    },
    ExecutionError,
};
use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::{
    manifest::{
        block_content_digest, detailed_range_digest, BlockCiphertextDescriptor, DetailedRange,
        ManifestBlockEntry, ManifestPayload, ManifestReference, ManifestVersion,
    },
    CiphertextFormat,
};
use sqlx::{PgPool, Postgres, Transaction};
use std::collections::HashMap;
use tracing::error;

const MANIFEST_ADVISORY_LOCK_DOMAIN: i64 = 0x4d41_4e49_4645_5354;

#[derive(Clone, Debug)]
pub(crate) struct PendingBlock {
    pub host_chain_id: i64,
    pub block_number: i64,
    pub block_hash: Vec<u8>,
    pub parent_block_hash: Vec<u8>,
    pub publication_cadence: i64,
    pub block_content_digest: Option<Vec<u8>>,
    pub descriptor_count: Option<i64>,
    pub manifest_revision: i64,
    pub last_manifest_publisher: Option<Vec<u8>>,
    pub manifest_digest: Option<Vec<u8>>,
    pub manifest_published: bool,
}

#[derive(Debug)]
pub(crate) struct PreparedManifest {
    pub payload: ManifestPayload,
    pub history_frontier: RangeFrontier,
}

#[derive(Debug)]
pub(crate) struct ManifestProgressCursor {
    block_number: i64,
    block_hash: Vec<u8>,
}

impl ManifestProgressCursor {
    pub(crate) fn start() -> Self {
        Self {
            block_number: -1,
            block_hash: Vec::new(),
        }
    }

    pub(crate) fn advance_to(&mut self, block: &PendingBlock) {
        self.block_number = block.block_number;
        self.block_hash.clone_from(&block.block_hash);
    }
}

pub(crate) type CiphertextDescriptor = BlockCiphertextDescriptor;

pub(crate) async fn pending_chain_ids(pool: &PgPool) -> Result<Vec<i64>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT DISTINCT host_chain_id
          FROM block_consensus
         WHERE block_content_digest IS NULL
            OR (manifest_required AND manifest_published = FALSE)
         ORDER BY host_chain_id
        "#,
    )
    .fetch_all(pool)
    .await?;
    Ok(rows.into_iter().map(|row| row.host_chain_id).collect())
}

/// Takes the chain-scoped transaction lock before selecting the earliest work
/// after `cursor`. A caller can advance the cursor past blocked work and inspect
/// competing lineages without allowing concurrent workers to publish the same
/// candidate.
pub(crate) async fn lock_next_block_to_progress(
    trx: &mut Transaction<'_, Postgres>,
    host_chain_id: i64,
    cursor: &ManifestProgressCursor,
) -> Result<Option<PendingBlock>, ExecutionError> {
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
        WITH RECURSIVE blocked_descendants AS (
            SELECT child.host_chain_id,
                   child.block_hash
              FROM block_consensus blocker
              JOIN block_consensus child
                ON child.host_chain_id = blocker.host_chain_id
               AND child.parent_block_hash = blocker.block_hash
             WHERE blocker.host_chain_id = $1
               AND (
                    blocker.block_content_digest IS NULL
                    OR (blocker.manifest_required AND blocker.manifest_published = FALSE)
               )
            UNION
            SELECT child.host_chain_id,
                   child.block_hash
              FROM blocked_descendants blocked
              JOIN block_consensus child
                ON child.host_chain_id = blocked.host_chain_id
               AND child.parent_block_hash = blocked.block_hash
        )
        SELECT candidate.host_chain_id,
               candidate.block_number,
               candidate.block_hash,
               candidate.parent_block_hash,
               candidate.publication_cadence,
               candidate.block_content_digest,
               candidate.descriptor_count,
               candidate.manifest_revision,
               candidate.last_manifest_publisher,
               candidate.manifest_digest,
               candidate.manifest_published
          FROM block_consensus candidate
         WHERE candidate.host_chain_id = $1
           AND (
                candidate.block_content_digest IS NULL
                OR (candidate.manifest_required AND candidate.manifest_published = FALSE)
           )
           AND NOT EXISTS (
                SELECT 1
                  FROM blocked_descendants blocked
                 WHERE blocked.host_chain_id = candidate.host_chain_id
                   AND blocked.block_hash = candidate.block_hash
           )
           AND (
                candidate.block_number > $2
                OR (
                    candidate.block_number = $2
                    AND candidate.block_hash > $3
                )
           )
         ORDER BY candidate.block_number, candidate.block_hash
         LIMIT 1
           FOR UPDATE
        "#,
        host_chain_id,
        cursor.block_number,
        &cursor.block_hash,
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

/// Discovers children only below host blocks whose lineage can still change.
/// Finalized and orphaned parents are closed after their currently visible
/// children have been copied, keeping polling cost bounded by the finality window.
pub(crate) async fn discover_known_children(pool: &PgPool) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH inserted AS (
            INSERT INTO block_consensus (
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash,
                   parent.publication_cadence
              FROM block_consensus parent
              JOIN host_chain_blocks_valid parent_host
                ON parent_host.chain_id = parent.host_chain_id
               AND parent_host.block_hash = parent.block_hash
              JOIN host_chain_blocks_valid child
                ON child.chain_id = parent.host_chain_id
               AND child.parent_hash = parent.block_hash
             WHERE NOT parent.children_discovery_complete
               AND parent_host.block_status <> 'orphaned'
               AND child.block_status <> 'orphaned'
               AND OCTET_LENGTH(child.parent_hash) = 32
            ON CONFLICT (host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        ), closed AS (
            UPDATE block_consensus block
               SET children_discovery_complete = TRUE,
                   updated_at = NOW()
              FROM host_chain_blocks_valid host
             WHERE NOT block.children_discovery_complete
               AND host.chain_id = block.host_chain_id
               AND host.block_hash = block.block_hash
               AND host.block_status IN ('finalized', 'orphaned')
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
    )
    .fetch_one(pool)
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
}

pub(crate) async fn discover_block_children(
    trx: &mut Transaction<'_, Postgres>,
    block: &PendingBlock,
) -> Result<u64, ExecutionError> {
    let row = sqlx::query!(
        r#"
        WITH RECURSIVE descendants AS (
            SELECT child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash
              FROM host_chain_blocks_valid child
             WHERE child.chain_id = $1
               AND child.parent_hash = $2
               AND child.block_status <> 'orphaned'
            UNION
            SELECT child.chain_id,
                   child.block_number,
                   child.block_hash,
                   child.parent_hash
              FROM descendants parent
              JOIN host_chain_blocks_valid child
                ON child.chain_id = parent.chain_id
               AND child.parent_hash = parent.block_hash
             WHERE child.block_status <> 'orphaned'
        ), inserted AS (
            INSERT INTO block_consensus (
                host_chain_id,
                block_number,
                block_hash,
                parent_block_hash,
                publication_cadence
            )
            SELECT chain_id,
                   block_number,
                   block_hash,
                   parent_hash,
                   $3
              FROM descendants
             WHERE OCTET_LENGTH(parent_hash) = 32
            ON CONFLICT (host_chain_id, block_hash) DO NOTHING
            RETURNING 1
        )
        SELECT COUNT(*) AS "inserted!" FROM inserted
        "#,
        block.host_chain_id,
        &block.block_hash,
        block.publication_cadence,
    )
    .fetch_one(trx.as_mut())
    .await?;
    Ok(u64::try_from(row.inserted).expect("insert count is non-negative"))
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
    let supersedes = load_superseded_revision(trx, target, coprocessor_context_id).await?;

    let (lineage, previous_manifest) = load_detailed_lineage(trx, target).await?;
    let blocks = load_detailed_blocks(trx, &lineage, coprocessor_context_id).await?;
    let detailed_range = build_detailed_range(target, coprocessor_context_id, blocks)?;
    let host_chain_id = non_negative_u256("host chain id", target.host_chain_id)?;

    let mut history_frontier = match previous_manifest.as_ref() {
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
        publisher,
        coprocessor_context_id,
        host_chain_id,
        publication_block_number: non_negative_u256("block number", target.block_number)?,
        publication_block_hash: b256("block hash", &target.block_hash)?,
        publication_parent_block_hash: b256("parent block hash", &target.parent_block_hash)?,
        revision,
        supersedes,
        detailed_range,
        historical_ranges,
        full_consensus_checkpoint: None,
        previous_manifest,
    };
    payload
        .validate()
        .map_err(|err| internal(format!("prepared manifest is invalid: {err}")))?;

    Ok(PreparedManifest {
        payload,
        history_frontier,
    })
}

async fn load_superseded_revision(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    coprocessor_context_id: U256,
) -> Result<Option<ManifestReference>, ExecutionError> {
    if target.manifest_revision == 0 {
        return Ok(None);
    }
    let previous_revision = target
        .manifest_revision
        .checked_sub(1)
        .ok_or_else(|| internal("manifest revision underflow"))?;
    let previous = load_manifest_revision(
        trx,
        b20_address(
            "previous manifest publisher",
            target
                .last_manifest_publisher
                .as_deref()
                .ok_or_else(|| internal("superseded manifest has no publisher"))?,
        )?,
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
            target.manifest_revision, previous_revision, target.host_chain_id, target.block_number,
        ))
    })?;
    Ok(Some(ManifestReference {
        publisher: previous.signed.payload.publisher,
        block_number: previous.signed.payload.publication_block_number,
        block_hash: previous.signed.payload.publication_block_hash,
        revision: previous.signed.payload.revision,
        manifest_digest: previous.digest,
    }))
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

pub(crate) async fn mark_manifest_published(
    trx: &mut Transaction<'_, Postgres>,
    target: &PendingBlock,
    publisher: Address,
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
               last_manifest_publisher = $7,
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
        publisher.as_slice(),
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

#[cfg(test)]
#[path = "manifest_soak_tests.rs"]
mod soak_tests;
