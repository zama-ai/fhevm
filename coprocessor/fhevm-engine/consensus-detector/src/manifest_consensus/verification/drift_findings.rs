use std::collections::{BTreeMap, BTreeSet, HashSet};

use alloy_primitives::{Address, B256, U256};
use block_manifest::{
    dyadic_range_digest, BlockCiphertextDescriptor, CiphertextStatus, DetailedRange,
    ManifestBlockEntry, ManifestPayload,
};
use sqlx::{Postgres, Transaction};

use crate::manifest_consensus::ExecutionError;

use crate::manifest_consensus::manifest_archive::{
    load_highest_archived_revision, load_local_manifest_covering_block,
    load_previous_local_manifests, AuthenticatedManifest, MAX_HISTORY_PREDECESSORS,
};

use super::localization_cache::without_completed_history;

use super::consensus_analysis::{
    detailed_scope, evaluate_quorum, CommitmentGroup, CommitmentScope, QuorumEvaluation,
    ScopeEvaluation, VerificationOutcome,
};

#[derive(Clone, Debug)]
struct DriftHandleFinding {
    consensus_epoch: String,
    block_number: i64,
    block_hash: B256,
    handle: B256,
    local: Option<BlockCiphertextDescriptor>,
    observed: Option<BlockCiphertextDescriptor>,
    observed_commitment_digest: B256,
    observed_has_quorum: bool,
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct DriftLocalization {
    pub complete: bool,
    pub drifted_block_count: Option<i64>,
    pub drifted_handle_count: Option<i64>,
}

pub(crate) struct DriftLocalizationContext<'a> {
    pub(crate) peer_publishers: &'a [Address],
    pub(crate) required_quorum: usize,
    pub(crate) completed_history: &'a HashSet<CommitmentScope>,
}

/// Atomically maintains the local drift inventory alongside a completed
/// verification decision. One row per handle; `target_*` is the quorum
/// descriptor when a computed threshold group exists.
/// A later concordant manifest closes findings for the exact covered block
/// hashes, with a revision guard against stale workers.
pub(crate) async fn apply_evaluation_to_drift_handles(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    evaluation: &QuorumEvaluation,
    context: DriftLocalizationContext<'_>,
) -> Result<DriftLocalization, ExecutionError> {
    let local_manifest = manifests
        .iter()
        .find(|manifest| manifest.signed.payload.publisher == local_publisher)
        .ok_or_else(|| internal("local manifest is missing from drift evaluation"))?;

    match evaluation.outcome {
        VerificationOutcome::Drift => {
            let (findings, localization_complete) = attributed_findings(
                trx,
                manifests,
                local_publisher,
                context.peer_publishers,
                context.required_quorum,
                &without_completed_history(evaluation, context.completed_history),
            )
            .await?;
            let block_count = i64::try_from(
                findings
                    .iter()
                    .map(|finding| (finding.block_number, finding.block_hash))
                    .collect::<BTreeSet<_>>()
                    .len(),
            )
            .map_err(|_| internal("drifted block count exceeds BIGINT"))?;
            let handle_count = i64::try_from(findings.len())
                .map_err(|_| internal("drifted handle count exceeds BIGINT"))?;
            for finding in findings {
                upsert_finding(trx, task_id, local_manifest, &finding).await?;
            }
            return Ok(DriftLocalization {
                complete: localization_complete,
                drifted_block_count: localization_complete.then_some(block_count),
                drifted_handle_count: localization_complete.then_some(handle_count),
            });
        }
        VerificationOutcome::Consensus => {
            resolve_covered_findings(trx, task_id, local_manifest).await?;
        }
        _ => {}
    }
    Ok(DriftLocalization {
        complete: true,
        drifted_block_count: (evaluation.outcome == VerificationOutcome::Consensus).then_some(0),
        drifted_handle_count: (evaluation.outcome == VerificationOutcome::Consensus).then_some(0),
    })
}

async fn attributed_findings(
    trx: &mut Transaction<'_, Postgres>,
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    peer_publishers: &[Address],
    required_quorum: usize,
    evaluation: &QuorumEvaluation,
) -> Result<(Vec<DriftHandleFinding>, bool), ExecutionError> {
    let mut scanner = HistoricalScanner {
        trx,
        local_publisher,
        peer_publishers,
        required_quorum,
        visited: HashSet::new(),
    };
    let mut result = scanner
        .localize_evaluation(
            manifests,
            evaluation,
            HistoricalWindow {
                first: U256::ZERO,
                last: U256::MAX,
            },
        )
        .await?;
    result.keep_one_finding_per_handle();
    Ok((result.findings, result.complete))
}

#[derive(Default)]
struct LocalizationResult {
    findings: Vec<DriftHandleFinding>,
    complete: bool,
}

impl LocalizationResult {
    fn complete() -> Self {
        Self {
            findings: Vec::new(),
            complete: true,
        }
    }

    fn incomplete() -> Self {
        Self::default()
    }

    fn merge(&mut self, mut other: Self) {
        self.findings.append(&mut other.findings);
        self.complete &= other.complete;
    }

    /// Inventory is one row per handle. Prefer the quorum group's descriptor.
    fn keep_one_finding_per_handle(&mut self) {
        let mut by_handle = BTreeMap::new();
        for finding in std::mem::take(&mut self.findings) {
            match by_handle.entry(finding.handle) {
                std::collections::btree_map::Entry::Vacant(slot) => {
                    slot.insert(finding);
                }
                std::collections::btree_map::Entry::Occupied(mut existing)
                    if finding.observed_has_quorum && !existing.get().observed_has_quorum =>
                {
                    existing.insert(finding);
                }
                std::collections::btree_map::Entry::Occupied(_) => {}
            }
        }
        self.findings = by_handle.into_values().collect();
    }
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct HistoricalWindow {
    first: U256,
    last: U256,
}

struct HistoricalScanner<'transaction, 'connection, 'peers> {
    trx: &'transaction mut Transaction<'connection, Postgres>,
    local_publisher: Address,
    peer_publishers: &'peers [Address],
    required_quorum: usize,
    visited: HashSet<(B256, HistoricalWindow)>,
}

impl HistoricalScanner<'_, '_, '_> {
    async fn localize_evaluation(
        &mut self,
        manifests: &[AuthenticatedManifest],
        evaluation: &QuorumEvaluation,
        window: HistoricalWindow,
    ) -> Result<LocalizationResult, ExecutionError> {
        let mut result = LocalizationResult::complete();
        for scope_evaluation in &evaluation.scopes {
            let Some(local_digest) = scope_evaluation.local_digest else {
                continue;
            };
            if scope_evaluation.groups.len() <= 1 {
                continue;
            }
            match &scope_evaluation.scope {
                CommitmentScope::Detailed { .. } => {
                    let Some(scope_window) = intersect_window(&scope_evaluation.scope, window)
                    else {
                        continue;
                    };
                    for observed_group in scope_evaluation
                        .groups
                        .iter()
                        .filter(|group| group.digest != local_digest)
                    {
                        let observed_has_quorum =
                            scope_evaluation.quorum_digest == Some(observed_group.digest);
                        result.findings.extend(detailed_findings(
                            manifests,
                            self.local_publisher,
                            &scope_evaluation.scope,
                            local_digest,
                            observed_group,
                            observed_has_quorum,
                            scope_window,
                        )?);
                    }
                }
                CommitmentScope::Historical { .. } => {
                    let Some(scope_window) = intersect_window(&scope_evaluation.scope, window)
                    else {
                        continue;
                    };
                    let nested = Box::pin(self.localize_historical(
                        manifests,
                        scope_evaluation,
                        scope_window,
                    ))
                    .await?;
                    result.merge(nested);
                }
            }
        }
        Ok(result)
    }

    async fn localize_historical(
        &mut self,
        manifests: &[AuthenticatedManifest],
        scope_evaluation: &ScopeEvaluation,
        window: HistoricalWindow,
    ) -> Result<LocalizationResult, ExecutionError> {
        let CommitmentScope::Historical {
            last,
            end_block_hash,
            ..
        } = &scope_evaluation.scope
        else {
            return Err(internal(
                "historical localization called for a detailed scope",
            ));
        };
        let local = manifests
            .iter()
            .find(|manifest| manifest.signed.payload.publisher == self.local_publisher)
            .ok_or_else(|| internal("local manifest is missing from historical localization"))?;
        if !local
            .signed
            .payload
            .historical_ranges
            .iter()
            .any(|range| range.end_block_number == *last && range.end_block_hash == *end_block_hash)
        {
            return Err(internal("local historical range is missing"));
        }
        let Some(covering) = load_local_manifest_covering_block(
            self.trx,
            self.local_publisher,
            local.signed.payload.version,
            local.signed.payload.coprocessor_context_id,
            i64_from_u256("manifest host chain id", local.signed.payload.host_chain_id)?,
            &local.signed.payload.consensus_epoch,
            *last,
            *end_block_hash,
        )
        .await?
        else {
            return Ok(LocalizationResult::incomplete());
        };
        if !self.visited.insert((covering.digest, window)) {
            return Ok(LocalizationResult::complete());
        }

        let host_chain_id = i64_from_u256(
            "covering manifest host chain id",
            covering.signed.payload.host_chain_id,
        )?;
        let version = covering.signed.payload.version;
        let coprocessor_context_id = covering.signed.payload.coprocessor_context_id;
        let consensus_epoch = covering.signed.payload.consensus_epoch.clone();
        let predecessors = load_previous_local_manifests(
            self.trx,
            self.local_publisher,
            &covering,
            MAX_HISTORY_PREDECESSORS,
        )
        .await?;
        let mut anchors = vec![covering];
        anchors.extend(predecessors);
        let local_digest = scope_evaluation
            .local_digest
            .expect("historical scope was selected with a local digest");

        for anchor in anchors {
            let publication_block_number = i64_from_u256(
                "history anchor publication block number",
                anchor.signed.payload.publication_block_number,
            )?;
            let publication_block_hash = anchor.signed.payload.publication_block_hash;
            let mut comparison_manifests = vec![anchor];
            for publisher in self.peer_publishers {
                if let Some(manifest) = load_highest_archived_revision(
                    self.trx,
                    *publisher,
                    version,
                    coprocessor_context_id,
                    host_chain_id,
                    &consensus_epoch,
                    publication_block_number,
                    publication_block_hash,
                )
                .await?
                {
                    comparison_manifests.push(manifest);
                }
            }

            let derived_digests = comparison_manifests
                .iter()
                .filter_map(|manifest| {
                    derive_historical_scope_digest(
                        &manifest.signed.payload,
                        &scope_evaluation.scope,
                    )
                    .map(|digest| (manifest.signed.payload.publisher, digest))
                })
                .collect::<BTreeMap<_, _>>();
            // Every advertised commitment must be reproduced by its publisher's
            // archived history. Still compare handles when this fails: known
            // findings remain useful, but the contradiction is not localized.
            let reconstructs_target = derived_digests.get(&self.local_publisher)
                == Some(&local_digest)
                && scope_evaluation.groups.iter().all(|group| {
                    group
                        .publishers
                        .iter()
                        .all(|publisher| derived_digests.get(publisher) == Some(&group.digest))
                });
            let comparison = evaluate_quorum(
                &comparison_manifests,
                self.local_publisher,
                self.required_quorum,
            );
            if comparison.outcome == VerificationOutcome::Drift {
                let mut result =
                    Box::pin(self.localize_evaluation(&comparison_manifests, &comparison, window))
                        .await?;
                result.complete &= reconstructs_target;
                return Ok(result);
            }
        }
        Ok(LocalizationResult::incomplete())
    }
}

#[derive(Clone)]
struct DerivedRange {
    start: U256,
    end: U256,
    scale: u32,
    end_block_hash: B256,
    digest: B256,
}

enum MaterializedRange {
    Empty,
    Available(DerivedRange),
    Missing,
}

pub(crate) fn derive_historical_scope_digest(
    payload: &ManifestPayload,
    scope: &CommitmentScope,
) -> Option<B256> {
    let CommitmentScope::Historical {
        first,
        last,
        scale,
        end_block_hash,
    } = scope
    else {
        return None;
    };
    let mut available = BTreeMap::new();
    let mut consensus_epoch_start = payload.detailed_range.first_block_number;
    for range in &payload.historical_ranges {
        consensus_epoch_start = consensus_epoch_start.min(range.start_block_number);
        let size = U256::ONE.checked_shl(range.scale as usize)?;
        let virtual_start = range
            .end_block_number
            .checked_add(U256::ONE)?
            .checked_sub(size)?;
        available.insert(
            (virtual_start, range.scale),
            DerivedRange {
                start: range.start_block_number,
                end: range.end_block_number,
                scale: range.scale,
                end_block_hash: range.end_block_hash,
                digest: range.digest,
            },
        );
    }
    for block in &payload.detailed_range.blocks {
        consensus_epoch_start = consensus_epoch_start.min(block.block_number);
        available.insert(
            (block.block_number, 0),
            DerivedRange {
                start: block.block_number,
                end: block.block_number,
                scale: 0,
                end_block_hash: block.block_hash,
                digest: block.block_content_digest,
            },
        );
    }
    let size = U256::ONE.checked_shl(*scale as usize)?;
    let virtual_start = last.checked_add(U256::ONE)?.checked_sub(size)?;
    let MaterializedRange::Available(range) = materialize_derived_range(
        payload,
        virtual_start,
        *scale,
        consensus_epoch_start,
        &mut available,
    )?
    else {
        return None;
    };
    (range.start == *first
        && range.end == *last
        && range.scale == *scale
        && range.end_block_hash == *end_block_hash)
        .then_some(range.digest)
}

fn materialize_derived_range(
    payload: &ManifestPayload,
    virtual_start: U256,
    scale: u32,
    consensus_epoch_start: U256,
    available: &mut BTreeMap<(U256, u32), DerivedRange>,
) -> Option<MaterializedRange> {
    if let Some(range) = available.get(&(virtual_start, scale)) {
        return Some(MaterializedRange::Available(range.clone()));
    }
    let size = U256::ONE.checked_shl(scale as usize)?;
    let end = virtual_start.checked_add(size.checked_sub(U256::ONE)?)?;
    if end < consensus_epoch_start {
        return Some(MaterializedRange::Empty);
    }
    let child_scale = scale.checked_sub(1)?;
    let child_size = U256::ONE.checked_shl(child_scale as usize)?;
    let right_start = virtual_start.checked_add(child_size)?;
    let left = materialize_derived_range(
        payload,
        virtual_start,
        child_scale,
        consensus_epoch_start,
        available,
    )?;
    let right = materialize_derived_range(
        payload,
        right_start,
        child_scale,
        consensus_epoch_start,
        available,
    )?;
    let range = match (left, right) {
        (MaterializedRange::Available(left), MaterializedRange::Available(right))
            if left.end.checked_add(U256::ONE) == Some(right.start) =>
        {
            DerivedRange {
                start: left.start,
                end: right.end,
                scale,
                end_block_hash: right.end_block_hash,
                digest: dyadic_range_digest(
                    payload.version,
                    payload.coprocessor_context_id,
                    payload.host_chain_id,
                    left.start,
                    right.end,
                    scale,
                    right.end_block_hash,
                    left.digest,
                    right.digest,
                ),
            }
        }
        (MaterializedRange::Empty, MaterializedRange::Available(right)) => DerivedRange {
            start: right.start,
            end: right.end,
            scale,
            end_block_hash: right.end_block_hash,
            digest: right.digest,
        },
        (MaterializedRange::Missing, _) | (_, MaterializedRange::Missing) => {
            return Some(MaterializedRange::Missing)
        }
        _ => return Some(MaterializedRange::Missing),
    };
    available.insert((virtual_start, scale), range.clone());
    Some(MaterializedRange::Available(range))
}

fn detailed_findings(
    manifests: &[AuthenticatedManifest],
    local_publisher: Address,
    scope: &CommitmentScope,
    local_digest: B256,
    observed_group: &CommitmentGroup,
    observed_has_quorum: bool,
    window: HistoricalWindow,
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
    compare_detailed_ranges(
        &local_manifest.signed.payload.detailed_range,
        &observed_manifest.signed.payload.detailed_range,
        &local_manifest.signed.payload.consensus_epoch,
        observed_group.digest,
        observed_has_quorum,
        window,
    )
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

fn compare_detailed_ranges(
    local: &DetailedRange,
    observed: &DetailedRange,
    local_manifest_epoch: &str,
    observed_commitment_digest: B256,
    observed_has_quorum: bool,
    window: HistoricalWindow,
) -> Result<Vec<DriftHandleFinding>, ExecutionError> {
    compare_blocks(
        local.blocks.iter().filter(|block| {
            block.block_number >= window.first && block.block_number <= window.last
        }),
        observed.blocks.iter().filter(|block| {
            block.block_number >= window.first && block.block_number <= window.last
        }),
        local_manifest_epoch,
        observed_commitment_digest,
        observed_has_quorum,
    )
}

fn compare_blocks<'a>(
    local: impl IntoIterator<Item = &'a ManifestBlockEntry>,
    observed: impl IntoIterator<Item = &'a ManifestBlockEntry>,
    local_manifest_epoch: &str,
    observed_commitment_digest: B256,
    observed_has_quorum: bool,
) -> Result<Vec<DriftHandleFinding>, ExecutionError> {
    let observed_blocks = observed
        .into_iter()
        .map(|block| ((block.block_number, block.block_hash), block))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    for local_block in local {
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
            if same_consensus_material(local_descriptor, observed_descriptor) {
                continue;
            }
            findings.push(DriftHandleFinding {
                consensus_epoch: local_manifest_epoch.to_owned(),
                block_number,
                block_hash: local_block.block_hash,
                handle,
                local: local_descriptor.cloned(),
                observed: observed_descriptor.cloned(),
                observed_commitment_digest,
                observed_has_quorum,
            });
        }
    }
    Ok(findings)
}

fn same_consensus_material(
    local: Option<&BlockCiphertextDescriptor>,
    observed: Option<&BlockCiphertextDescriptor>,
) -> bool {
    match (local, observed) {
        (Some(local), Some(observed)) => local.consensus_digest() == observed.consensus_digest(),
        (None, None) => true,
        _ => false,
    }
}

fn intersect_window(scope: &CommitmentScope, window: HistoricalWindow) -> Option<HistoricalWindow> {
    match scope {
        CommitmentScope::Detailed { first, last, .. }
        | CommitmentScope::Historical { first, last, .. } => {
            let first = (*first).max(window.first);
            let last = (*last).min(window.last);
            (first <= last).then_some(HistoricalWindow { first, last })
        }
    }
}

// Preserve status distinctions: the digest-presence booleans cannot distinguish
// an absent handle from an errored or uncomputed descriptor. For multiple
// differences, absence/status comes first, then ct64, metadata, and ct128.
fn drift_reason(
    local: Option<&BlockCiphertextDescriptor>,
    observed: Option<&BlockCiphertextDescriptor>,
) -> &'static str {
    match (local.map(|d| &d.status), observed.map(|d| &d.status)) {
        (None, _) => "missing_here",
        (_, None) => "unknown_on_peer",
        (Some(CiphertextStatus::Error { .. }), _) => "error_here",
        (_, Some(CiphertextStatus::Error { .. })) => "error_on_peer",
        (Some(CiphertextStatus::Uncomputed), _) => "uncomputed_here",
        (_, Some(CiphertextStatus::Uncomputed)) => "uncomputed_on_peer",
        (
            Some(CiphertextStatus::Computed {
                ct64_digest: local_ct64,
                keyset_id: local_key,
                ..
            }),
            Some(CiphertextStatus::Computed {
                ct64_digest: peer_ct64,
                keyset_id: peer_key,
                ..
            }),
        ) => {
            if local_ct64 != peer_ct64 {
                "ct64_mismatch"
            } else if local_key != peer_key {
                "metadata_mismatch"
            } else {
                "ct128_mismatch"
            }
        }
    }
}

async fn upsert_finding(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
    local_manifest: &AuthenticatedManifest,
    finding: &DriftHandleFinding,
) -> Result<(), ExecutionError> {
    let payload = &local_manifest.signed.payload;
    let local = finding.local.as_ref();
    let observed = finding.observed.as_ref();
    let local_keyset_id = u256_bytes(local.and_then(BlockCiphertextDescriptor::keyset_id));
    let observed_keyset_id = u256_bytes(observed.and_then(BlockCiphertextDescriptor::keyset_id));
    let local_ct64_digest = digest_bytes(local.and_then(BlockCiphertextDescriptor::ct64_digest));
    let observed_ct64_digest =
        digest_bytes(observed.and_then(BlockCiphertextDescriptor::ct64_digest));
    let local_ct128_digest = digest_bytes(local.and_then(BlockCiphertextDescriptor::ct128_digest));
    let observed_ct128_digest =
        digest_bytes(observed.and_then(BlockCiphertextDescriptor::ct128_digest));
    let local_ct128_format = local
        .and_then(BlockCiphertextDescriptor::ct128_format)
        .map(|format| format as u8 as i16);
    let observed_ct128_format = observed
        .and_then(BlockCiphertextDescriptor::ct128_format)
        .map(|format| format as u8 as i16);
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    let quorum = finding.observed_has_quorum.then_some(observed).flatten();
    let target_ct64_digest = digest_bytes(quorum.and_then(BlockCiphertextDescriptor::ct64_digest));
    let target_keyset_id = u256_bytes(quorum.and_then(BlockCiphertextDescriptor::keyset_id));
    let target_ct128_digest =
        digest_bytes(quorum.and_then(BlockCiphertextDescriptor::ct128_digest));
    let target_ct128_format = quorum
        .and_then(BlockCiphertextDescriptor::ct128_format)
        .map(|format| format as u8 as i16);
    let reason = drift_reason(local, observed);
    sqlx::query!(
        r#"
        INSERT INTO drifted_handle (
            consensus_epoch, coprocessor_context_id, host_chain_id,
            block_number, block_hash, handle, status,
            local_present, observed_present, local_keyset_id, observed_keyset_id,
            local_ct64_digest, observed_ct64_digest,
            local_ct128_digest, observed_ct128_digest, local_ct128_format,
            observed_ct128_format, observed_commitment_digest, target_ct64_digest,
            target_keyset_id, target_ct128_digest, target_ct128_format,
            last_observed_task_id, reason
        ) VALUES (
            $1, $2, $3, $4, $5, $6, 'unresolved',
            $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18,
            $19, $20, $21, $22, $23
        )
        ON CONFLICT (consensus_epoch, coprocessor_context_id, host_chain_id,
                     block_hash, handle)
        DO UPDATE SET
            status = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.status
                ELSE 'unresolved'
            END,
            reason = CASE
                WHEN drifted_handle.reason = 'ct64_mismatch' THEN drifted_handle.reason
                WHEN drifted_handle.can_be_healed THEN drifted_handle.reason
                ELSE EXCLUDED.reason
            END,
            local_present = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.local_present
                ELSE EXCLUDED.local_present
            END,
            observed_present = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_present
                ELSE EXCLUDED.observed_present
            END,
            local_keyset_id = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.local_keyset_id
                ELSE EXCLUDED.local_keyset_id
            END,
            observed_keyset_id = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_keyset_id
                ELSE EXCLUDED.observed_keyset_id
            END,
            local_ct64_digest = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.local_ct64_digest
                ELSE EXCLUDED.local_ct64_digest
            END,
            observed_ct64_digest = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_ct64_digest
                ELSE EXCLUDED.observed_ct64_digest
            END,
            local_ct128_digest = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.local_ct128_digest
                ELSE EXCLUDED.local_ct128_digest
            END,
            observed_ct128_digest = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_ct128_digest
                ELSE EXCLUDED.observed_ct128_digest
            END,
            local_ct128_format = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.local_ct128_format
                ELSE EXCLUDED.local_ct128_format
            END,
            observed_ct128_format = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_ct128_format
                ELSE EXCLUDED.observed_ct128_format
            END,
            observed_commitment_digest = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.observed_commitment_digest
                ELSE EXCLUDED.observed_commitment_digest
            END,
            target_ct64_digest = COALESCE(drifted_handle.target_ct64_digest, EXCLUDED.target_ct64_digest),
            target_keyset_id = COALESCE(drifted_handle.target_keyset_id, EXCLUDED.target_keyset_id),
            target_ct128_digest = COALESCE(drifted_handle.target_ct128_digest, EXCLUDED.target_ct128_digest),
            target_ct128_format = COALESCE(drifted_handle.target_ct128_format, EXCLUDED.target_ct128_format),
            last_observed_task_id = COALESCE(EXCLUDED.last_observed_task_id, drifted_handle.last_observed_task_id),
            resolved_task_id = CASE
                WHEN drifted_handle.can_be_healed THEN drifted_handle.resolved_task_id
                ELSE NULL
            END
        WHERE drifted_handle.healed_at IS NULL
          AND (drifted_handle.last_observed_task_id IS NULL
               OR drifted_handle.last_observed_task_id <= EXCLUDED.last_observed_task_id)
        "#,
        finding.consensus_epoch,
        context.as_slice(), host_chain_id, finding.block_number,
        finding.block_hash.as_slice(), finding.handle.as_slice(),
        local.is_some(), observed.is_some(), local_keyset_id, observed_keyset_id,
        local_ct64_digest, observed_ct64_digest,
        local_ct128_digest, observed_ct128_digest, local_ct128_format,
        observed_ct128_format, finding.observed_commitment_digest.as_slice(),
        target_ct64_digest, target_keyset_id, target_ct128_digest, target_ct128_format,
        task_id, reason,
    )
    .execute(trx.as_mut())
    .await?;
    Ok(())
}

async fn resolve_covered_findings(
    trx: &mut Transaction<'_, Postgres>,
    task_id: i64,
    local_manifest: &AuthenticatedManifest,
) -> Result<(), ExecutionError> {
    let payload = &local_manifest.signed.payload;
    let context = payload.coprocessor_context_id.to_be_bytes::<32>();
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    for block in &payload.detailed_range.blocks {
        sqlx::query!(
            r#"
            UPDATE drifted_handle
                   SET status = 'resolved',
                   last_observed_task_id = $5,
                   resolved_task_id = $5
             WHERE consensus_epoch = $1
               AND coprocessor_context_id = $2
               AND host_chain_id = $3
               AND block_hash = $4
               AND status = 'unresolved'
               AND detection_kind <> 'inferred'
               AND NOT can_be_healed
               AND healed_at IS NULL
               AND last_observed_task_id <= $5
            "#,
            payload.consensus_epoch.clone(),
            context.as_slice(),
            host_chain_id,
            block.block_hash.as_slice(),
            task_id,
        )
        .execute(trx.as_mut())
        .await?;
    }
    Ok(())
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

#[cfg(test)]
mod reason_tests {
    use super::*;
    use block_manifest::CiphertextFormat;

    fn handle_finding(handle: u8, quorum: bool) -> DriftHandleFinding {
        DriftHandleFinding {
            consensus_epoch: "legacy".into(),
            block_number: 1,
            block_hash: B256::ZERO,
            handle: B256::repeat_byte(handle),
            local: None,
            observed: None,
            observed_commitment_digest: B256::repeat_byte(if quorum { 1 } else { 2 }),
            observed_has_quorum: quorum,
        }
    }

    #[test]
    fn inventory_keeps_one_finding_preferring_quorum() {
        let mut result = LocalizationResult {
            findings: vec![
                handle_finding(1, false),
                handle_finding(1, true),
                handle_finding(2, false),
            ],
            complete: true,
        };
        result.keep_one_finding_per_handle();
        assert_eq!(result.findings.len(), 2);
        let one = result
            .findings
            .iter()
            .find(|finding| finding.handle == B256::repeat_byte(1))
            .unwrap();
        assert!(one.observed_has_quorum);
        assert_eq!(one.observed_commitment_digest, B256::repeat_byte(1));
    }

    #[test]
    fn reasons_distinguish_absence_status_and_material() {
        let h = B256::repeat_byte(1);
        let computed = BlockCiphertextDescriptor::computed(
            h,
            U256::ONE,
            None,
            h,
            h,
            CiphertextFormat::CompressedOnCpu,
        );
        let error = BlockCiphertextDescriptor::from_computation_error(h, None);
        let pending = BlockCiphertextDescriptor::from_uncomputed(h);
        for (local, peer, expected) in [
            (None, Some(&computed), "missing_here"),
            (Some(&computed), None, "unknown_on_peer"),
            (Some(&error), Some(&computed), "error_here"),
            (Some(&computed), Some(&error), "error_on_peer"),
            (Some(&pending), Some(&computed), "uncomputed_here"),
            (Some(&computed), Some(&pending), "uncomputed_on_peer"),
        ] {
            assert_eq!(drift_reason(local, peer), expected);
        }
        for (ct64, ct128, key, expected) in [
            (2, 2, 2, "ct64_mismatch"),
            (1, 2, 1, "ct128_mismatch"),
            (1, 1, 2, "metadata_mismatch"),
        ] {
            let peer = BlockCiphertextDescriptor::computed(
                h,
                U256::from(key),
                None,
                B256::repeat_byte(ct64),
                B256::repeat_byte(ct128),
                CiphertextFormat::CompressedOnCpu,
            );
            assert_eq!(drift_reason(Some(&computed), Some(&peer)), expected);
        }
        let gateway_only = BlockCiphertextDescriptor::computed(
            h,
            U256::ONE,
            Some(U256::from(7)),
            h,
            h,
            CiphertextFormat::CompressedOnCpu,
        );
        assert_ne!(computed, gateway_only);
        assert!(same_consensus_material(
            Some(&computed),
            Some(&gateway_only)
        ));
        let ct128_and_gateway = BlockCiphertextDescriptor::computed(
            h,
            U256::ONE,
            Some(U256::from(7)),
            h,
            B256::repeat_byte(2),
            CiphertextFormat::CompressedOnCpu,
        );
        assert!(!same_consensus_material(
            Some(&computed),
            Some(&ct128_and_gateway)
        ));
        assert_eq!(
            drift_reason(Some(&computed), Some(&ct128_and_gateway)),
            "ct128_mismatch"
        );
    }

    #[test]
    fn provenance_only_differences_are_not_handle_findings() {
        let handle = B256::repeat_byte(1);
        let other = B256::repeat_byte(2);
        let drifted = B256::repeat_byte(3);
        let local = vec![
            BlockCiphertextDescriptor::computed(
                handle,
                U256::ONE,
                Some(U256::from(7)),
                handle,
                handle,
                CiphertextFormat::CompressedOnCpu,
            ),
            BlockCiphertextDescriptor::from_computation_error(other, Some("gpu Display".into())),
            BlockCiphertextDescriptor::computed(
                drifted,
                U256::ONE,
                Some(U256::from(7)),
                B256::repeat_byte(0x11),
                drifted,
                CiphertextFormat::CompressedOnCpu,
            ),
        ];
        let observed = vec![
            BlockCiphertextDescriptor::computed(
                handle,
                U256::ONE,
                None,
                handle,
                handle,
                CiphertextFormat::CompressedOnCpu,
            ),
            BlockCiphertextDescriptor::from_computation_error(other, Some("cpu Display".into())),
            BlockCiphertextDescriptor::computed(
                drifted,
                U256::ONE,
                None,
                B256::repeat_byte(0x22),
                drifted,
                CiphertextFormat::CompressedOnCpu,
            ),
        ];
        let local_block = test_block(local);
        let observed_block = test_block(observed);
        let findings = compare_blocks(
            [&local_block],
            [&observed_block],
            "legacy",
            B256::repeat_byte(0x11),
            true,
        )
        .expect("compare provenance-mixed blocks");
        assert_eq!(findings.len(), 1);
        assert_eq!(findings[0].handle, drifted);
    }

    fn test_block(ciphertexts: Vec<BlockCiphertextDescriptor>) -> ManifestBlockEntry {
        ManifestBlockEntry {
            block_number: U256::from(42),
            block_hash: B256::repeat_byte(0xaa),
            parent_block_hash: B256::repeat_byte(0xa9),
            block_content_digest: B256::ZERO,
            ciphertexts,
        }
    }
}
