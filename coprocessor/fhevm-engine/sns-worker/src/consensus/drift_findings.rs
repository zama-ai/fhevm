use std::collections::{BTreeMap, HashSet};

use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::manifest::{
    dyadic_range_digest, BlockCiphertextDescriptor, DetailedRange, HistoricalRange,
    ManifestBlockEntry, ManifestPayload,
};
use sqlx::{Postgres, Transaction};

use crate::ExecutionError;

use super::{
    consensus_analysis::{
        detailed_scope, CommitmentGroup, CommitmentScope, QuorumEvaluation, ScopeEvaluation,
        VerificationOutcome,
    },
    manifest_archive::{load_manifest_by_reference, AuthenticatedManifest},
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum FindingKind {
    MissingHandle,
    UnexpectedHandle,
    DescriptorMismatch,
}

impl FindingKind {
    fn as_db_str(self) -> &'static str {
        match self {
            Self::MissingHandle => "missing_handle",
            Self::UnexpectedHandle => "unexpected_handle",
            Self::DescriptorMismatch => "descriptor_mismatch",
        }
    }
}

#[derive(Clone, Debug)]
struct DriftHandleFinding {
    block_number: i64,
    block_hash: B256,
    handle: B256,
    kind: FindingKind,
    local: Option<BlockCiphertextDescriptor>,
    observed: Option<BlockCiphertextDescriptor>,
    observed_publisher: Address,
    observed_manifest_digest: B256,
    observed_commitment_digest: B256,
    observed_has_quorum: bool,
}

/// Atomically maintains the local drift inventory alongside a completed
/// verification decision. Every observed content difference is retained;
/// `observed_has_quorum` distinguishes an actionable remediation reference.
/// A later concordant manifest closes findings for the exact covered block
/// hashes, with a revision guard against stale workers.
pub(crate) async fn apply_evaluation_to_drift_handles(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    evaluation: &QuorumEvaluation,
) -> Result<bool, ExecutionError> {
    let local_manifest = manifests
        .iter()
        .find(|manifest| manifest.signed.payload.publisher == local_publisher)
        .ok_or_else(|| internal("local manifest is missing from drift evaluation"))?;

    match evaluation.outcome {
        VerificationOutcome::Drift => {
            let (findings, localization_complete) =
                attributed_findings(trx, manifests, local_publisher, evaluation).await?;
            for finding in findings {
                upsert_finding(trx, target_id, local_manifest, &finding).await?;
            }
            return Ok(localization_complete);
        }
        VerificationOutcome::Consensus => {
            resolve_covered_findings(trx, target_id, local_manifest).await?;
        }
        _ => {}
    }
    Ok(true)
}

async fn attributed_findings(
    trx: &mut Transaction<'_, Postgres>,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    evaluation: &QuorumEvaluation,
) -> Result<(Vec<DriftHandleFinding>, bool), ExecutionError> {
    let mut findings = Vec::new();
    let mut localization_complete = true;
    for scope_evaluation in &evaluation.scopes {
        let Some(local_digest) = scope_evaluation.local_digest else {
            continue;
        };
        if scope_evaluation.groups.len() <= 1 {
            continue;
        }
        for observed_group in scope_evaluation
            .groups
            .iter()
            .filter(|group| group.digest != local_digest)
        {
            let localized = localize_disagreement(
                trx,
                manifests,
                local_publisher,
                scope_evaluation,
                observed_group,
            )
            .await?;
            match localized {
                Some(group_findings) => findings.extend(group_findings),
                None => localization_complete = false,
            }
        }
    }
    Ok((findings, localization_complete))
}

async fn localize_disagreement(
    trx: &mut Transaction<'_, Postgres>,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope_evaluation: &ScopeEvaluation,
    observed_group: &CommitmentGroup,
) -> Result<Option<Vec<DriftHandleFinding>>, ExecutionError> {
    let local_digest = scope_evaluation
        .local_digest
        .expect("caller selected a scope with a local digest");
    let observed_has_quorum = scope_evaluation.quorum_digest == Some(observed_group.digest);
    match &scope_evaluation.scope {
        CommitmentScope::Detailed { .. } => detailed_findings(
            manifests,
            local_publisher,
            &scope_evaluation.scope,
            local_digest,
            observed_group,
            observed_has_quorum,
        )
        .map(Some),
        CommitmentScope::Historical { .. } => {
            historical_findings(
                trx,
                manifests,
                local_publisher,
                &scope_evaluation.scope,
                local_digest,
                observed_group,
                observed_has_quorum,
            )
            .await
        }
    }
}

fn detailed_findings(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope: &CommitmentScope,
    local_digest: B256,
    observed_group: &CommitmentGroup,
    observed_has_quorum: bool,
) -> Result<Vec<DriftHandleFinding>, ExecutionError> {
    let observed_publisher = observed_group
        .publishers
        .first()
        .copied()
        .ok_or_else(|| internal("observed digest has no representative publisher"))?;
    let local_manifest =
        manifest_for_detailed_scope(manifests, local_publisher, scope, local_digest)
            .ok_or_else(|| internal("local detailed manifest is missing"))?;
    let observed_manifest =
        manifest_for_detailed_scope(manifests, observed_publisher, scope, observed_group.digest)
            .ok_or_else(|| internal("observed detailed manifest is missing"))?;
    compare_ranges(
        &local_manifest.signed.payload.detailed_range,
        &observed_manifest.signed.payload.detailed_range,
        observed_publisher,
        observed_manifest.digest,
        observed_group.digest,
        observed_has_quorum,
    )
}

async fn historical_findings(
    trx: &mut Transaction<'_, Postgres>,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope: &CommitmentScope,
    local_digest: B256,
    observed_group: &CommitmentGroup,
    observed_has_quorum: bool,
) -> Result<Option<Vec<DriftHandleFinding>>, ExecutionError> {
    let local_manifest =
        manifest_for_historical_scope(manifests, local_publisher, scope, local_digest)
            .ok_or_else(|| internal("local historical manifest is missing"))?;
    let local_range = historical_range(&local_manifest.signed.payload, scope)
        .ok_or_else(|| internal("local historical range is missing"))?;
    let Some(local_blocks) = load_historical_blocks(trx, local_manifest, local_range).await? else {
        return Ok(None);
    };

    for observed_publisher in &observed_group.publishers {
        let observed_manifest = manifest_for_historical_scope(
            manifests,
            *observed_publisher,
            scope,
            observed_group.digest,
        )
        .ok_or_else(|| internal("observed historical manifest is missing"))?;
        let observed_range = historical_range(&observed_manifest.signed.payload, scope)
            .ok_or_else(|| internal("observed historical range is missing"))?;
        let Some(observed_blocks) =
            load_historical_blocks(trx, observed_manifest, observed_range).await?
        else {
            continue;
        };
        return compare_ranges(
            &detailed_from_historical(local_range, local_blocks),
            &detailed_from_historical(observed_range, observed_blocks),
            *observed_publisher,
            observed_manifest.digest,
            observed_group.digest,
            observed_has_quorum,
        )
        .map(Some);
    }
    Ok(None)
}

fn manifest_for_detailed_scope<'a>(
    manifests: &'a [AuthenticatedManifest],
    publisher: Address,
    scope: &CommitmentScope,
    digest: B256,
) -> Option<&'a AuthenticatedManifest> {
    manifests.iter().find(|manifest| {
        let payload = &manifest.signed.payload;
        payload.publisher == publisher
            && payload.detailed_range.digest == digest
            && detailed_scope(&payload.detailed_range).as_ref() == Some(scope)
    })
}

fn manifest_for_historical_scope<'a>(
    manifests: &'a [AuthenticatedManifest],
    publisher: Address,
    scope: &CommitmentScope,
    digest: B256,
) -> Option<&'a AuthenticatedManifest> {
    manifests.iter().find(|manifest| {
        manifest.signed.payload.publisher == publisher
            && historical_range(&manifest.signed.payload, scope)
                .is_some_and(|range| range.digest == digest)
    })
}

fn historical_range<'a>(
    payload: &'a ManifestPayload,
    scope: &CommitmentScope,
) -> Option<&'a HistoricalRange> {
    let CommitmentScope::Historical {
        first,
        last,
        scale,
        end_block_hash,
    } = scope
    else {
        return None;
    };
    payload.historical_ranges.iter().find(|range| {
        range.start_block_number == *first
            && range.end_block_number == *last
            && range.scale == *scale
            && range.end_block_hash == *end_block_hash
    })
}

async fn load_historical_blocks(
    trx: &mut Transaction<'_, Postgres>,
    tip: &AuthenticatedManifest,
    range: &HistoricalRange,
) -> Result<Option<Vec<ManifestBlockEntry>>, ExecutionError> {
    let start = range.start_block_number;
    let end = range.end_block_number;
    let expected_count = end
        .checked_sub(start)
        .and_then(|width| width.checked_add(U256::ONE))
        .and_then(|width| usize::try_from(width).ok())
        .ok_or_else(|| internal("historical range width exceeds memory limits"))?;
    let mut blocks = BTreeMap::<U256, ManifestBlockEntry>::new();
    let mut current = tip.clone();
    let mut visited = HashSet::new();

    loop {
        let payload = &current.signed.payload;
        let identity = (
            payload.publication_block_number,
            payload.publication_block_hash,
            payload.revision,
            current.digest,
        );
        if !visited.insert(identity) {
            return Err(internal("cycle in archived manifest predecessor chain"));
        }

        for block in &payload.detailed_range.blocks {
            if block.block_number < start || block.block_number > end {
                continue;
            }
            if let Some(previous) = blocks.insert(block.block_number, block.clone()) {
                if previous != *block {
                    return Err(internal(format!(
                        "conflicting archived block {} while localizing historical drift",
                        block.block_number,
                    )));
                }
            }
        }
        if blocks.len() == expected_count {
            break;
        }

        let Some(reference) = payload.previous_manifest.as_ref() else {
            return Ok(None);
        };
        let Some(previous) = load_manifest_by_reference(
            trx,
            payload.version,
            payload.coprocessor_context_id,
            i64_from_u256("manifest host chain id", payload.host_chain_id)?,
            reference,
        )
        .await?
        else {
            return Ok(None);
        };
        current = previous;
    }

    let blocks = blocks.into_values().collect::<Vec<_>>();
    for (index, block) in blocks.iter().enumerate() {
        let expected_number = start + U256::from(index);
        if block.block_number != expected_number {
            return Ok(None);
        }
        if let Some(previous) = index.checked_sub(1).map(|previous| &blocks[previous]) {
            if block.parent_block_hash != previous.block_hash {
                return Err(internal(
                    "archived historical blocks are not one contiguous lineage",
                ));
            }
        }
    }
    if blocks.last().map(|block| block.block_hash) != Some(range.end_block_hash) {
        return Err(internal(
            "archived historical blocks do not end at the committed block hash",
        ));
    }
    let root = recompute_historical_root(tip, range, &blocks)?;
    if root != range.digest {
        return Err(internal(format!(
            "archived historical blocks recompute to {root}, expected {}",
            range.digest,
        )));
    }
    Ok(Some(blocks))
}

fn recompute_historical_root(
    tip: &AuthenticatedManifest,
    range: &HistoricalRange,
    blocks: &[ManifestBlockEntry],
) -> Result<B256, ExecutionError> {
    #[derive(Clone, Copy)]
    struct Node {
        start: U256,
        end: U256,
        end_block_hash: B256,
        digest: B256,
    }

    let mut nodes = blocks
        .iter()
        .map(|block| Node {
            start: block.block_number,
            end: block.block_number,
            end_block_hash: block.block_hash,
            digest: block.block_content_digest,
        })
        .collect::<Vec<_>>();
    for scale in 1..=range.scale {
        if !nodes.len().is_multiple_of(2) {
            return Err(internal("historical range has an incomplete dyadic level"));
        }
        nodes = nodes
            .chunks_exact(2)
            .map(|children| {
                let left = children[0];
                let right = children[1];
                if left.end + U256::ONE != right.start {
                    return Err(internal("historical dyadic children are not adjacent"));
                }
                Ok(Node {
                    start: left.start,
                    end: right.end,
                    end_block_hash: right.end_block_hash,
                    digest: dyadic_range_digest(
                        tip.signed.payload.version,
                        tip.signed.payload.coprocessor_context_id,
                        tip.signed.payload.host_chain_id,
                        left.start,
                        right.end,
                        scale,
                        right.end_block_hash,
                        left.digest,
                        right.digest,
                    ),
                })
            })
            .collect::<Result<Vec<_>, ExecutionError>>()?;
    }
    match nodes.as_slice() {
        [root] if root.start == range.start_block_number && root.end == range.end_block_number => {
            Ok(root.digest)
        }
        _ => Err(internal(
            "historical blocks do not form the committed dyadic range",
        )),
    }
}

fn detailed_from_historical(
    range: &HistoricalRange,
    blocks: Vec<ManifestBlockEntry>,
) -> DetailedRange {
    DetailedRange {
        first_block_number: range.start_block_number,
        last_block_number: range.end_block_number,
        digest: range.digest,
        blocks,
    }
}

fn compare_ranges(
    local: &DetailedRange,
    observed: &DetailedRange,
    observed_publisher: Address,
    observed_manifest_digest: B256,
    observed_commitment_digest: B256,
    observed_has_quorum: bool,
) -> Result<Vec<DriftHandleFinding>, ExecutionError> {
    let observed_blocks = observed
        .blocks
        .iter()
        .map(|block| ((block.block_number, block.block_hash), block))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for local_block in &local.blocks {
        let Some(observed_block) =
            observed_blocks.get(&(local_block.block_number, local_block.block_hash))
        else {
            continue;
        };
        let block_number = i64_from_u256("drift block number", local_block.block_number)?;
        let local_descriptors = local_block
            .ciphertexts
            .iter()
            .map(|descriptor| (descriptor.handle, descriptor))
            .collect::<BTreeMap<_, _>>();
        let observed_descriptors = observed_block
            .ciphertexts
            .iter()
            .map(|descriptor| (descriptor.handle, descriptor))
            .collect::<BTreeMap<_, _>>();
        for handle in local_descriptors
            .keys()
            .chain(observed_descriptors.keys())
            .copied()
            .collect::<std::collections::BTreeSet<_>>()
        {
            let local_descriptor = local_descriptors.get(&handle).copied();
            let observed_descriptor = observed_descriptors.get(&handle).copied();
            if local_descriptor == observed_descriptor {
                continue;
            }
            let kind = match (local_descriptor, observed_descriptor) {
                (None, Some(_)) => FindingKind::MissingHandle,
                (Some(_), None) => FindingKind::UnexpectedHandle,
                (Some(_), Some(_)) => FindingKind::DescriptorMismatch,
                (None, None) => unreachable!("handle originated from one descriptor map"),
            };
            findings.push(DriftHandleFinding {
                block_number,
                block_hash: local_block.block_hash,
                handle,
                kind,
                local: local_descriptor.cloned(),
                observed: observed_descriptor.cloned(),
                observed_publisher,
                observed_manifest_digest,
                observed_commitment_digest,
                observed_has_quorum,
            });
        }
    }
    Ok(findings)
}

async fn upsert_finding(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    local_manifest: &AuthenticatedManifest,
    finding: &DriftHandleFinding,
) -> Result<(), ExecutionError> {
    let payload = &local_manifest.signed.payload;
    let local = finding.local.as_ref();
    let observed = finding.observed.as_ref();
    let local_revision = i64::try_from(payload.revision)
        .map_err(|_| internal("local manifest revision exceeds BIGINT"))?;
    let local_keyset_id = u256_bytes(local.map(|descriptor| descriptor.keyset_id));
    let observed_keyset_id = u256_bytes(observed.map(|descriptor| descriptor.keyset_id));
    let local_gateway_key_id = u256_bytes(local.and_then(|descriptor| descriptor.gateway_key_id));
    let local_ct64_digest = digest_bytes(local.map(|descriptor| descriptor.ct64_digest));
    let observed_ct64_digest = digest_bytes(observed.map(|descriptor| descriptor.ct64_digest));
    let local_ct128_digest = digest_bytes(local.map(|descriptor| descriptor.ct128_digest));
    let observed_ct128_digest = digest_bytes(observed.map(|descriptor| descriptor.ct128_digest));
    let local_ct128_format = local.map(|descriptor| descriptor.ct128_format as u8 as i16);
    let observed_ct128_format = observed.map(|descriptor| descriptor.ct128_format as u8 as i16);
    let keyset_mismatch = descriptor_field_differs(local, observed, |value| value.keyset_id);
    let ct64_digest_mismatch = descriptor_field_differs(local, observed, |value| value.ct64_digest);
    let ct128_digest_mismatch =
        descriptor_field_differs(local, observed, |value| value.ct128_digest);
    let ct128_format_mismatch =
        descriptor_field_differs(local, observed, |value| value.ct128_format);
    let version = i16::from(u8::from(payload.version));
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;

    sqlx::query!(
        r#"
        INSERT INTO block_consensus_drift_handle (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            block_number,
            block_hash,
            handle,
            status,
            finding_kind,
            local_present,
            observed_present,
            local_keyset_id,
            observed_keyset_id,
            local_gateway_key_id,
            local_ct64_digest,
            observed_ct64_digest,
            local_ct128_digest,
            observed_ct128_digest,
            local_ct128_format,
            observed_ct128_format,
            keyset_mismatch,
            ct64_digest_mismatch,
            ct128_digest_mismatch,
            ct128_format_mismatch,
            observed_publisher,
            observed_manifest_digest,
            observed_commitment_digest,
            observed_has_quorum,
            first_detected_target_id,
            last_observed_target_id,
            first_detected_local_manifest_digest,
            last_observed_local_manifest_digest,
            last_local_manifest_revision
        )
        VALUES (
            $1, $2, $3, $4, $5, $6, $7, 'unresolved', $8,
            $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19,
            $20, $21, $22, $23, $24, $25, $26, $27, $28, $28, $29, $29, $30
        )
        ON CONFLICT (
            local_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            block_hash,
            handle,
            observed_commitment_digest
        ) DO UPDATE
           SET block_number = EXCLUDED.block_number,
               status = 'unresolved',
               finding_kind = EXCLUDED.finding_kind,
               local_present = EXCLUDED.local_present,
               observed_present = EXCLUDED.observed_present,
               local_keyset_id = EXCLUDED.local_keyset_id,
               observed_keyset_id = EXCLUDED.observed_keyset_id,
               local_gateway_key_id = EXCLUDED.local_gateway_key_id,
               local_ct64_digest = EXCLUDED.local_ct64_digest,
               observed_ct64_digest = EXCLUDED.observed_ct64_digest,
               local_ct128_digest = EXCLUDED.local_ct128_digest,
               observed_ct128_digest = EXCLUDED.observed_ct128_digest,
               local_ct128_format = EXCLUDED.local_ct128_format,
               observed_ct128_format = EXCLUDED.observed_ct128_format,
               keyset_mismatch = EXCLUDED.keyset_mismatch,
               ct64_digest_mismatch = EXCLUDED.ct64_digest_mismatch,
               ct128_digest_mismatch = EXCLUDED.ct128_digest_mismatch,
               ct128_format_mismatch = EXCLUDED.ct128_format_mismatch,
               observed_publisher = EXCLUDED.observed_publisher,
               observed_manifest_digest = EXCLUDED.observed_manifest_digest,
               observed_has_quorum = EXCLUDED.observed_has_quorum,
               last_observed_target_id = EXCLUDED.last_observed_target_id,
               last_observed_local_manifest_digest =
                   EXCLUDED.last_observed_local_manifest_digest,
               last_local_manifest_revision = EXCLUDED.last_local_manifest_revision,
               last_observed_at = NOW(),
               resolved_target_id = NULL,
               resolved_local_manifest_digest = NULL,
               resolved_local_manifest_revision = NULL,
               resolved_at = NULL,
               updated_at = NOW()
         WHERE block_consensus_drift_handle.last_observed_target_id
                   <= EXCLUDED.last_observed_target_id
        "#,
        payload.publisher.as_slice(),
        version,
        context.as_slice(),
        host_chain_id,
        finding.block_number,
        finding.block_hash.as_slice(),
        finding.handle.as_slice(),
        finding.kind.as_db_str(),
        local.is_some(),
        observed.is_some(),
        local_keyset_id,
        observed_keyset_id,
        local_gateway_key_id,
        local_ct64_digest,
        observed_ct64_digest,
        local_ct128_digest,
        observed_ct128_digest,
        local_ct128_format,
        observed_ct128_format,
        keyset_mismatch,
        ct64_digest_mismatch,
        ct128_digest_mismatch,
        ct128_format_mismatch,
        finding.observed_publisher.as_slice(),
        finding.observed_manifest_digest.as_slice(),
        finding.observed_commitment_digest.as_slice(),
        finding.observed_has_quorum,
        target_id,
        local_manifest.digest.as_slice(),
        local_revision,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

async fn resolve_covered_findings(
    trx: &mut Transaction<'_, Postgres>,
    target_id: i64,
    local_manifest: &AuthenticatedManifest,
) -> Result<(), ExecutionError> {
    let payload = &local_manifest.signed.payload;
    let local_revision = i64::try_from(payload.revision)
        .map_err(|_| internal("local manifest revision exceeds BIGINT"))?;
    let version = i16::from(u8::from(payload.version));
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    for block in &payload.detailed_range.blocks {
        sqlx::query!(
            r#"
            UPDATE block_consensus_drift_handle
               SET status = 'resolved',
                   last_observed_target_id = $6,
                   last_observed_local_manifest_digest = $7,
                   last_local_manifest_revision = $8,
                   last_observed_at = NOW(),
                   resolved_target_id = $6,
                   resolved_local_manifest_digest = $7,
                   resolved_local_manifest_revision = $8,
                   resolved_at = NOW(),
                   updated_at = NOW()
             WHERE local_publisher = $1
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND block_hash = $5
               AND status = 'unresolved'
               AND last_observed_target_id <= $6
            "#,
            payload.publisher.as_slice(),
            version,
            context.as_slice(),
            host_chain_id,
            block.block_hash.as_slice(),
            target_id,
            local_manifest.digest.as_slice(),
            local_revision,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(())
}

fn descriptor_field_differs<T: PartialEq>(
    local: Option<&BlockCiphertextDescriptor>,
    observed: Option<&BlockCiphertextDescriptor>,
    field: impl Fn(&BlockCiphertextDescriptor) -> T,
) -> bool {
    match (local, observed) {
        (Some(local), Some(observed)) => field(local) != field(observed),
        _ => false,
    }
}

fn u256_bytes(value: Option<U256>) -> Option<Vec<u8>> {
    value.map(|value| value.to_be_bytes::<32>().to_vec())
}

fn digest_bytes(value: Option<B256>) -> Option<Vec<u8>> {
    value.map(|value| value.to_vec())
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}
