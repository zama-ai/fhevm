use std::collections::HashMap;
use std::time::{Duration, Instant};

use alloy::primitives::{Address, FixedBytes, B256};
use fhevm_engine_common::utils::to_hex;
use sqlx::{Pool, Postgres, Row};
use tracing::{debug, warn};

use crate::metrics::{
    CONSENSUS_LATENCY_BLOCKS_HISTOGRAM, CONSENSUS_TIMEOUT_COUNTER, DRIFT_DETECTED_COUNTER,
    MISSING_SUBMISSION_COUNTER, POST_CONSENSUS_COMPLETION_BLOCKS_HISTOGRAM,
};

use fhevm_gateway_bindings::ciphertext_commits::CiphertextCommits;

#[derive(Clone, Copy, Debug)]
pub(crate) struct EventContext {
    pub(crate) block_number: u64,
    pub(crate) block_hash: Option<B256>,
    pub(crate) tx_hash: Option<B256>,
    pub(crate) log_index: Option<u64>,
    pub(crate) observed_at: Instant,
}

type CiphertextDigest = FixedBytes<32>;

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DigestPair {
    ciphertext_digest: CiphertextDigest,
    ciphertext128_digest: CiphertextDigest,
}

#[derive(Clone, Copy, Debug)]
struct Submission {
    sender: Address,
    digests: DigestPair,
}

#[derive(Clone, Debug)]
struct ConsensusState {
    context: EventContext,
    received_at: Instant,
    digests: DigestPair,
    senders: Vec<Address>,
}

#[derive(Clone, Debug)]
struct HandleState {
    first_seen_block: u64,
    first_seen_block_hash: Option<B256>,
    first_seen_at: Instant,
    last_seen_block: u64,
    expected_senders: Vec<Address>,
    submissions: Vec<Submission>,
    consensus: Option<ConsensusState>,
    local_consensus_checked: bool,
    drift_reported: bool,
}

impl HandleState {
    fn new(context: EventContext, expected_senders: Vec<Address>) -> Self {
        let submission_capacity = expected_senders.len();
        Self {
            first_seen_block: context.block_number,
            first_seen_block_hash: context.block_hash,
            first_seen_at: context.observed_at,
            last_seen_block: context.block_number,
            expected_senders,
            submissions: Vec::with_capacity(submission_capacity),
            consensus: None,
            local_consensus_checked: false,
            drift_reported: false,
        }
    }
}

enum HandleOutcome {
    Pending,
    LocalDigestNeverAppeared,
    NotAllCoprocessorsSubmitted,
    GatewayNeverReachedConsensus,
}

pub(crate) struct DriftDetector {
    current_expected_senders: Vec<Address>,
    /// Handles waiting for consensus or post-consensus grace. Bounded implicitly:
    /// `evict_stale` removes entries after `drift_no_consensus_timeout` (no consensus)
    /// or `drift_post_consensus_grace` (consensus reached). Steady-state size is
    /// proportional to handle throughput * timeout duration.
    open_handles: HashMap<CiphertextDigest, HandleState>,
    local_node_id: String,
    drift_no_consensus_timeout: Duration,
    drift_post_consensus_grace: Duration,
    deferred_drift_detected: u64,
    deferred_consensus_timeout: u64,
    deferred_missing_submission: u64,
    replaying: bool,
}

impl DriftDetector {
    pub(crate) fn new(
        expected_senders: Vec<Address>,
        drift_no_consensus_timeout: Duration,
        drift_post_consensus_grace: Duration,
    ) -> Self {
        Self {
            current_expected_senders: expected_senders,
            open_handles: HashMap::new(),
            local_node_id: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_owned()),
            drift_no_consensus_timeout,
            drift_post_consensus_grace,
            deferred_drift_detected: 0,
            deferred_consensus_timeout: 0,
            deferred_missing_submission: 0,
            replaying: false,
        }
    }

    pub(crate) fn set_replaying(&mut self, replaying: bool) {
        self.replaying = replaying;
    }

    pub(crate) fn set_current_expected_senders(&mut self, expected_senders: Vec<Address>) {
        self.current_expected_senders = expected_senders;
    }

    pub(crate) fn observe_submission(
        &mut self,
        event: CiphertextCommits::AddCiphertextMaterial,
        context: EventContext,
    ) {
        let handle = event.ctHandle;
        let digests = DigestPair {
            ciphertext_digest: event.ciphertextDigest,
            ciphertext128_digest: event.snsCiphertextDigest,
        };

        let state = self
            .open_handles
            .entry(handle)
            .or_insert_with(|| HandleState::new(context, self.current_expected_senders.clone()));
        state.last_seen_block = context.block_number;

        if let Some(existing) = state
            .submissions
            .iter()
            .find(|submission| submission.sender == event.coprocessorTxSender)
        {
            if !self.replaying && existing.digests != digests {
                warn!(
                    handle = %handle,
                    host_chain_id = chain_id_from_handle(handle),
                    local_node_id = %self.local_node_id,
                    block_number = context.block_number,
                    block_hash = ?context.block_hash,
                    tx_hash = ?context.tx_hash,
                    log_index = ?context.log_index,
                    sender = %event.coprocessorTxSender,
                    previous_ciphertext_digest = %existing.digests.ciphertext_digest,
                    previous_ciphertext128_digest = %existing.digests.ciphertext128_digest,
                    new_ciphertext_digest = %digests.ciphertext_digest,
                    new_ciphertext128_digest = %digests.ciphertext128_digest,
                    "Same coprocessor submitted different digests for one handle"
                );
            }
            return;
        }

        state.submissions.push(Submission {
            sender: event.coprocessorTxSender,
            digests,
        });

        if !self.replaying && !state.drift_reported && has_multiple_variants(&state.submissions) {
            let variants = variant_summaries(&state.submissions);
            let seen: Vec<String> = state
                .submissions
                .iter()
                .map(|s| s.sender.to_string())
                .collect();
            let missing: Vec<String> = state
                .expected_senders
                .iter()
                .filter(|s| !state.submissions.iter().any(|sub| sub.sender == **s))
                .map(ToString::to_string)
                .collect();
            warn!(
                handle = %handle,
                host_chain_id = chain_id_from_handle(handle),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                block_number = context.block_number,
                block_hash = ?context.block_hash,
                tx_hash = ?context.tx_hash,
                log_index = ?context.log_index,
                variant_count = variants.len(),
                variants = ?variants,
                seen_senders = ?seen,
                missing_senders = ?missing,
                source = "peer_submission",
                "Drift detected: observed multiple digest variants for handle"
            );
            state.drift_reported = true;
            self.deferred_drift_detected += 1;
        }

        self.finish_if_complete(handle);
    }

    pub(crate) async fn handle_consensus(
        &mut self,
        event: CiphertextCommits::AddCiphertextMaterialConsensus,
        context: EventContext,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
        let handle = event.ctHandle;
        let state = self
            .open_handles
            .entry(handle)
            .or_insert_with(|| HandleState::new(context, self.current_expected_senders.clone()));
        state.last_seen_block = context.block_number;
        state.consensus = Some(ConsensusState {
            context,
            received_at: context.observed_at,
            digests: DigestPair {
                ciphertext_digest: event.ciphertextDigest,
                ciphertext128_digest: event.snsCiphertextDigest,
            },
            senders: event.coprocessorTxSenders,
        });
        state.local_consensus_checked = false;
        if !self.replaying {
            CONSENSUS_LATENCY_BLOCKS_HISTOGRAM
                .observe(context.block_number.saturating_sub(state.first_seen_block) as f64);
        }
        self.try_check_local_consensus(handle, db_pool).await
    }

    async fn refresh_pending_consensus_checks(
        &mut self,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
        let handles = self
            .open_handles
            .iter()
            .filter_map(|(handle, state)| {
                (state.consensus.is_some() && !state.local_consensus_checked).then_some(*handle)
            })
            .collect::<Vec<_>>();

        for handle in handles {
            self.try_check_local_consensus(handle, db_pool).await?;
        }

        Ok(())
    }

    async fn try_check_local_consensus(
        &mut self,
        handle: CiphertextDigest,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
        if self.replaying {
            // During rebuild replay, skip DB queries. The handle will be re-checked
            // via refresh_pending_consensus_checks once replay finishes.
            return Ok(());
        }

        let Some(state) = self.open_handles.get(&handle) else {
            return Ok(());
        };
        let Some(consensus) = &state.consensus else {
            return Ok(());
        };

        let row = sqlx::query(
            "SELECT ciphertext, ciphertext128 FROM ciphertext_digest WHERE handle = $1",
        )
        .bind(handle.as_slice())
        .fetch_optional(db_pool)
        .await?;

        let local_digests = row.and_then(|r| {
            let ct: Option<Vec<u8>> = r.get("ciphertext");
            let ct128: Option<Vec<u8>> = r.get("ciphertext128");
            ct.zip(ct128)
        });
        let Some((local_ciphertext_digest, local_ciphertext128_digest)) = local_digests else {
            debug!(
                handle = %handle,
                host_chain_id = chain_id_from_handle(handle),
                local_node_id = %self.local_node_id,
                block_number = consensus.context.block_number,
                tx_hash = ?consensus.context.tx_hash,
                "Local digests not yet available; deferring drift check"
            );
            return Ok(());
        };

        if consensus.digests.ciphertext_digest.as_slice() != local_ciphertext_digest.as_slice()
            || consensus.digests.ciphertext128_digest.as_slice()
                != local_ciphertext128_digest.as_slice()
        {
            let local_digests = DigestPair {
                ciphertext_digest: FixedBytes::from(
                    <[u8; 32]>::try_from(local_ciphertext_digest.as_slice())
                        .map_err(|_| anyhow::anyhow!("local ciphertext digest not 32 bytes"))?,
                ),
                ciphertext128_digest: FixedBytes::from(
                    <[u8; 32]>::try_from(local_ciphertext128_digest.as_slice())
                        .map_err(|_| anyhow::anyhow!("local ciphertext128 digest not 32 bytes"))?,
                ),
            };
            let local_variant_sender_count =
                sender_count_for_variant(&state.submissions, local_digests);
            let consensus_variant_sender_count =
                sender_count_for_variant(&state.submissions, consensus.digests);
            let observed_variants = variant_summaries(&state.submissions);
            warn!(
                handle = %handle,
                host_chain_id = chain_id_from_handle(handle),
                local_node_id = %self.local_node_id,
                block_number = consensus.context.block_number,
                block_hash = ?consensus.context.block_hash,
                tx_hash = ?consensus.context.tx_hash,
                log_index = ?consensus.context.log_index,
                consensus_ciphertext_digest = %consensus.digests.ciphertext_digest,
                consensus_ciphertext128_digest = %consensus.digests.ciphertext128_digest,
                local_ciphertext_digest = %to_hex(&local_ciphertext_digest),
                local_ciphertext128_digest = %to_hex(&local_ciphertext128_digest),
                local_matches_observed_variant = local_variant_sender_count > 0,
                local_variant_sender_count,
                consensus_variant_sender_count,
                observed_variant_count = observed_variants.len(),
                observed_variants = ?observed_variants,
                source = "consensus",
                "Drift detected: local digest does not match consensus"
            );
            self.deferred_drift_detected += 1;
        }

        let Some(state) = self.open_handles.get_mut(&handle) else {
            return Ok(());
        };
        state.local_consensus_checked = true;
        self.finish_if_complete(handle);
        Ok(())
    }

    fn evict_stale(&mut self, now: Instant) {
        let mut finished = Vec::new();

        for (handle, state) in &self.open_handles {
            match self.classify_handle(state, now) {
                HandleOutcome::Pending => {}
                HandleOutcome::LocalDigestNeverAppeared => {
                    let Some(consensus) = state.consensus.as_ref() else {
                        continue;
                    };
                    warn!(
                        handle = %handle,
                        host_chain_id = chain_id_from_handle(*handle),
                        local_node_id = %self.local_node_id,
                        first_seen_block = state.first_seen_block,
                        first_seen_block_hash = ?state.first_seen_block_hash,
                        last_seen_block = state.last_seen_block,
                        consensus_block = consensus.context.block_number,
                        consensus_block_hash = ?consensus.context.block_hash,
                        consensus_tx_hash = ?consensus.context.tx_hash,
                        consensus_log_index = ?consensus.context.log_index,
                        "Consensus was observed but local digests never became available for comparison"
                    );
                    finished.push(*handle);
                }
                HandleOutcome::NotAllCoprocessorsSubmitted => {
                    let Some(consensus) = state.consensus.as_ref() else {
                        continue;
                    };
                    let variants = variant_summaries(&state.submissions);
                    warn!(
                        handle = %handle,
                        host_chain_id = chain_id_from_handle(*handle),
                        local_node_id = %self.local_node_id,
                        first_seen_block = state.first_seen_block,
                        first_seen_block_hash = ?state.first_seen_block_hash,
                        last_seen_block = state.last_seen_block,
                        consensus_block = consensus.context.block_number,
                        consensus_block_hash = ?consensus.context.block_hash,
                        consensus_tx_hash = ?consensus.context.tx_hash,
                        consensus_log_index = ?consensus.context.log_index,
                        consensus_senders = ?consensus.senders.iter().map(ToString::to_string).collect::<Vec<_>>(),
                        consensus_ciphertext_digest = %consensus.digests.ciphertext_digest,
                        consensus_ciphertext128_digest = %consensus.digests.ciphertext128_digest,
                        seen_senders = ?state.submissions.iter().map(|s| s.sender.to_string()).collect::<Vec<_>>(),
                        missing_senders = ?state.expected_senders.iter()
                            .filter(|s| !state.submissions.iter().any(|sub| sub.sender == **s))
                            .map(ToString::to_string).collect::<Vec<_>>(),
                        variant_count = variants.len(),
                        variants = ?variants,
                        "Not all expected coprocessors submitted before post-consensus grace period expired"
                    );
                    self.deferred_missing_submission += 1;
                    finished.push(*handle);
                }
                HandleOutcome::GatewayNeverReachedConsensus => {
                    let variants = variant_summaries(&state.submissions);
                    warn!(
                        handle = %handle,
                        host_chain_id = chain_id_from_handle(*handle),
                        local_node_id = %self.local_node_id,
                        first_seen_block = state.first_seen_block,
                        first_seen_block_hash = ?state.first_seen_block_hash,
                        last_seen_block = state.last_seen_block,
                        seen_senders = ?state.submissions.iter().map(|s| s.sender.to_string()).collect::<Vec<_>>(),
                        missing_senders = ?state.expected_senders.iter()
                            .filter(|s| !state.submissions.iter().any(|sub| sub.sender == **s))
                            .map(ToString::to_string).collect::<Vec<_>>(),
                        variant_count = variants.len(),
                        variants = ?variants,
                        "Handle timed out before consensus was observed"
                    );
                    self.deferred_consensus_timeout += 1;
                    finished.push(*handle);
                }
            }
        }

        for handle in finished {
            self.open_handles.remove(&handle);
        }
    }

    pub(crate) fn flush_metrics(&mut self) {
        DRIFT_DETECTED_COUNTER.inc_by(self.deferred_drift_detected);
        CONSENSUS_TIMEOUT_COUNTER.inc_by(self.deferred_consensus_timeout);
        MISSING_SUBMISSION_COUNTER.inc_by(self.deferred_missing_submission);
        self.deferred_drift_detected = 0;
        self.deferred_consensus_timeout = 0;
        self.deferred_missing_submission = 0;
    }

    fn evaluate_open_handles(&mut self, now: Instant) {
        if self.replaying {
            return;
        }

        let drift_handles = self
            .open_handles
            .iter()
            .filter_map(|(handle, state)| {
                (!state.drift_reported && has_multiple_variants(&state.submissions))
                    .then_some(*handle)
            })
            .collect::<Vec<_>>();

        for handle in drift_handles {
            let Some(state) = self.open_handles.get_mut(&handle) else {
                continue;
            };
            let variants = variant_summaries(&state.submissions);
            warn!(
                handle = %handle,
                host_chain_id = chain_id_from_handle(handle),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                last_seen_block = state.last_seen_block,
                variant_count = variants.len(),
                variants = ?variants,
                seen_senders = ?state.submissions.iter().map(|s| s.sender.to_string()).collect::<Vec<_>>(),
                missing_senders = ?state.expected_senders.iter()
                    .filter(|s| !state.submissions.iter().any(|sub| sub.sender == **s))
                    .map(ToString::to_string).collect::<Vec<_>>(),
                source = "peer_submission",
                "Drift detected: observed multiple digest variants for handle"
            );
            state.drift_reported = true;
            self.deferred_drift_detected += 1;
        }

        self.finalize_completed_without_consensus();

        self.evict_stale(now);
    }

    pub(crate) fn earliest_open_block(&self) -> Option<u64> {
        self.open_handles
            .values()
            .map(|state| state.first_seen_block)
            .min()
    }

    fn finalize_completed_without_consensus(&mut self) {
        // Invariant: the gateway emits consensus as part of processing the final
        // agreeing submission. Once every expected sender has submitted, the
        // absence of a consensus event is already anomalous, so we alert
        // immediately instead of waiting for `no_consensus_timeout`.
        let completed_without_consensus = self
            .open_handles
            .iter()
            .filter_map(|(handle, state)| {
                (state.submissions.len() == state.expected_senders.len()
                    && state.consensus.is_none())
                .then_some(*handle)
            })
            .collect::<Vec<_>>();

        for handle in completed_without_consensus {
            let Some(state) = self.open_handles.get(&handle) else {
                continue;
            };

            let variants = variant_summaries(&state.submissions);
            warn!(
                handle = %handle,
                host_chain_id = chain_id_from_handle(handle),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                last_seen_block = state.last_seen_block,
                seen_senders = ?state.submissions.iter().map(|s| s.sender.to_string()).collect::<Vec<_>>(),
                variant_count = variants.len(),
                variants = ?variants,
                "All expected coprocessors submitted but no consensus event was observed"
            );
            self.deferred_consensus_timeout += 1;
            self.open_handles.remove(&handle);
        }
    }

    fn finish_if_complete(&mut self, handle: CiphertextDigest) {
        let Some(state) = self.open_handles.get(&handle) else {
            return;
        };

        if state.submissions.len() < state.expected_senders.len() {
            return;
        }

        if state.consensus.is_some() {
            if !state.local_consensus_checked {
                return;
            }
            let consensus_block = state.consensus.as_ref().unwrap().context.block_number;
            POST_CONSENSUS_COMPLETION_BLOCKS_HISTOGRAM
                .observe(state.last_seen_block.saturating_sub(consensus_block) as f64);
            self.open_handles.remove(&handle);
        }
    }

    /// Finalize a normal log-polling batch: check deferred consensus results,
    /// alert on completed-without-consensus handles, and evict stale handles.
    pub(crate) async fn end_of_batch(&mut self, db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
        self.refresh_pending_consensus_checks(db_pool).await?;
        self.finalize_completed_without_consensus();
        self.evict_stale(Instant::now());
        Ok(())
    }

    /// Finalize a rebuild replay: check deferred consensus results and evaluate
    /// all open handles against the current chain tip. Called by
    /// `rebuild_drift_detector` in `gw_listener.rs` after log replay completes.
    pub(crate) async fn end_of_rebuild(&mut self, db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
        self.refresh_pending_consensus_checks(db_pool).await?;
        self.evaluate_open_handles(Instant::now());
        Ok(())
    }

    fn classify_handle(&self, state: &HandleState, now: Instant) -> HandleOutcome {
        if let Some(consensus) = &state.consensus {
            if !state.local_consensus_checked {
                return if now.duration_since(consensus.received_at)
                    >= self.drift_no_consensus_timeout
                {
                    HandleOutcome::LocalDigestNeverAppeared
                } else {
                    HandleOutcome::Pending
                };
            }

            if state.submissions.len() < state.expected_senders.len() {
                return if now.duration_since(consensus.received_at)
                    >= self.drift_post_consensus_grace
                {
                    HandleOutcome::NotAllCoprocessorsSubmitted
                } else {
                    HandleOutcome::Pending
                };
            }

            unreachable!("handle should have been removed by finish_if_complete");
        }

        if now.duration_since(state.first_seen_at) >= self.drift_no_consensus_timeout {
            HandleOutcome::GatewayNeverReachedConsensus
        } else {
            HandleOutcome::Pending
        }
    }
}

/// Extracts the host chain ID from a ciphertext handle.
/// Handles encode the chain ID in bytes 22..30 as a big-endian u64.
fn chain_id_from_handle(handle: CiphertextDigest) -> u64 {
    u64::from_be_bytes(handle[22..30].try_into().expect("handle is 32 bytes"))
}

fn has_multiple_variants(submissions: &[Submission]) -> bool {
    let Some(first) = submissions.first() else {
        return false;
    };
    submissions[1..].iter().any(|s| s.digests != first.digests)
}

fn variant_summaries(submissions: &[Submission]) -> Vec<String> {
    let mut variants: Vec<(DigestPair, Vec<Address>)> = Vec::new();

    for submission in submissions {
        if let Some((_, senders)) = variants
            .iter_mut()
            .find(|(digests, _)| *digests == submission.digests)
        {
            senders.push(submission.sender);
        } else {
            variants.push((submission.digests, vec![submission.sender]));
        }
    }

    variants
        .into_iter()
        .map(|(digests, senders)| {
            format!(
                "ct64={} ct128={} senders={:?}",
                digests.ciphertext_digest,
                digests.ciphertext128_digest,
                senders
                    .iter()
                    .map(ToString::to_string)
                    .collect::<Vec<String>>()
            )
        })
        .collect()
}

fn sender_count_for_variant(submissions: &[Submission], digests: DigestPair) -> usize {
    submissions
        .iter()
        .filter(|submission| submission.digests == digests)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    #[test]
    fn rebuild_preserves_state_across_batches() {
        let sender_a = Address::from([0x11; 20]);
        let sender_b = Address::from([0x22; 20]);
        let sender_c = Address::from([0x33; 20]);
        let handle = FixedBytes::from([0x44; 32]);
        let digest_a = FixedBytes::from([0x55; 32]);
        let digest_b = FixedBytes::from([0x66; 32]);
        let digest_128 = FixedBytes::from([0x77; 32]);
        let base = Instant::now();
        let mut detector = DriftDetector::new(
            vec![sender_a, sender_b, sender_c],
            Duration::from_secs(50),
            Duration::from_secs(10),
        );

        detector.set_replaying(true);

        detector.observe_submission(
            make_submission_event(handle, digest_a, digest_128, sender_a),
            context_at(100, base),
        );
        detector.observe_submission(
            make_submission_event(handle, digest_b, digest_128, sender_b),
            context_at(103, base),
        );

        let state = detector
            .open_handles
            .get(&handle)
            .expect("handle is tracked");
        assert_eq!(state.first_seen_block, 100);
        assert_eq!(state.last_seen_block, 103);
        assert_eq!(state.submissions.len(), 2);
        assert!(has_multiple_variants(&state.submissions));
        assert!(!state.drift_reported);
        assert_eq!(detector.deferred_drift_detected, 0);

        detector.set_replaying(false);
        detector.evaluate_open_handles(base);

        let state = detector
            .open_handles
            .get(&handle)
            .expect("handle remains open");
        assert!(state.drift_reported);
        assert_eq!(state.first_seen_block, 100);
        assert_eq!(state.last_seen_block, 103);
        assert_eq!(state.submissions.len(), 2);
        assert_eq!(detector.deferred_drift_detected, 1);
        assert_eq!(detector.deferred_consensus_timeout, 0);
    }

    fn make_submission_event(
        handle: CiphertextDigest,
        ciphertext_digest: CiphertextDigest,
        ciphertext128_digest: CiphertextDigest,
        sender: Address,
    ) -> CiphertextCommits::AddCiphertextMaterial {
        CiphertextCommits::AddCiphertextMaterial {
            ctHandle: handle,
            keyId: U256::from(1),
            ciphertextDigest: ciphertext_digest,
            snsCiphertextDigest: ciphertext128_digest,
            coprocessorTxSender: sender,
        }
    }

    fn make_consensus_event(
        handle: CiphertextDigest,
        ciphertext_digest: CiphertextDigest,
        ciphertext128_digest: CiphertextDigest,
        senders: Vec<Address>,
    ) -> CiphertextCommits::AddCiphertextMaterialConsensus {
        CiphertextCommits::AddCiphertextMaterialConsensus {
            ctHandle: handle,
            keyId: U256::from(1),
            ciphertextDigest: ciphertext_digest,
            snsCiphertextDigest: ciphertext128_digest,
            coprocessorTxSenders: senders,
        }
    }

    fn context(block_number: u64) -> EventContext {
        EventContext {
            block_number,
            block_hash: None,
            tx_hash: None,
            log_index: None,
            observed_at: Instant::now(),
        }
    }

    fn context_at(block_number: u64, at: Instant) -> EventContext {
        EventContext {
            block_number,
            block_hash: None,
            tx_hash: None,
            log_index: None,
            observed_at: at,
        }
    }

    fn submit_digest_event_and_drift_check(
        d: &mut DriftDetector,
        handle: CiphertextDigest,
        ct: impl Into<CiphertextDigest>,
        ct128: impl Into<CiphertextDigest>,
        sender: Address,
        block: u64,
    ) {
        d.observe_submission(
            make_submission_event(handle, ct.into(), ct128.into(), sender),
            context(block),
        );
    }

    fn submit_at(
        d: &mut DriftDetector,
        handle: CiphertextDigest,
        ct: impl Into<CiphertextDigest>,
        ct128: impl Into<CiphertextDigest>,
        sender: Address,
        block: u64,
        at: Instant,
    ) {
        d.observe_submission(
            make_submission_event(handle, ct.into(), ct128.into(), sender),
            context_at(block, at),
        );
    }

    fn senders() -> Vec<Address> {
        vec![
            Address::left_padding_from(&[1]),
            Address::left_padding_from(&[2]),
            Address::left_padding_from(&[3]),
        ]
    }

    fn detector() -> DriftDetector {
        DriftDetector::new(senders(), Duration::from_secs(5), Duration::from_secs(2))
    }

    fn make_consensus_state(
        block_number: u64,
        ciphertext_digest: CiphertextDigest,
        ciphertext128_digest: CiphertextDigest,
        senders: Vec<Address>,
    ) -> ConsensusState {
        make_consensus_state_at(
            block_number,
            ciphertext_digest,
            ciphertext128_digest,
            senders,
            Instant::now(),
        )
    }

    fn make_consensus_state_at(
        block_number: u64,
        ciphertext_digest: CiphertextDigest,
        ciphertext128_digest: CiphertextDigest,
        senders: Vec<Address>,
        at: Instant,
    ) -> ConsensusState {
        ConsensusState {
            context: EventContext {
                block_number,
                block_hash: None,
                tx_hash: None,
                log_index: None,
                observed_at: at,
            },
            received_at: at,
            digests: DigestPair {
                ciphertext_digest,
                ciphertext128_digest,
            },
            senders,
        }
    }

    #[test]
    fn earliest_open_block_tracks_oldest_open_handle() {
        let mut detector = detector();
        let senders = senders();
        let handle_a = FixedBytes::from([1u8; 32]);
        let handle_b = FixedBytes::from([2u8; 32]);
        let digest_a = DigestPair {
            ciphertext_digest: FixedBytes::from([3u8; 32]),
            ciphertext128_digest: FixedBytes::from([4u8; 32]),
        };
        let digest_b = DigestPair {
            ciphertext_digest: FixedBytes::from([5u8; 32]),
            ciphertext128_digest: FixedBytes::from([6u8; 32]),
        };

        assert_eq!(detector.earliest_open_block(), None);

        submit_digest_event_and_drift_check(
            &mut detector,
            handle_b,
            digest_b.ciphertext_digest,
            digest_b.ciphertext128_digest,
            senders[0],
            20,
        );
        submit_digest_event_and_drift_check(
            &mut detector,
            handle_a,
            digest_a.ciphertext_digest,
            digest_a.ciphertext128_digest,
            senders[0],
            10,
        );

        assert_eq!(detector.earliest_open_block(), Some(10));

        submit_digest_event_and_drift_check(
            &mut detector,
            handle_a,
            digest_a.ciphertext_digest,
            digest_a.ciphertext128_digest,
            senders[1],
            11,
        );
        submit_digest_event_and_drift_check(
            &mut detector,
            handle_a,
            digest_a.ciphertext_digest,
            digest_a.ciphertext128_digest,
            senders[2],
            12,
        );
        detector.finalize_completed_without_consensus();

        assert_eq!(detector.earliest_open_block(), Some(20));
    }

    #[test]
    fn rebuild_replays_silently_then_alerts_once_on_evaluate() {
        let mut detector = detector();
        let handle = FixedBytes::from([7u8; 32]);
        let senders = senders();
        let base = Instant::now();

        detector.set_replaying(true);
        submit_at(
            &mut detector,
            handle,
            [8u8; 32],
            [9u8; 32],
            senders[0],
            10,
            base,
        );
        submit_at(
            &mut detector,
            handle,
            [10u8; 32],
            [11u8; 32],
            senders[1],
            11,
            base,
        );

        assert_eq!(detector.deferred_drift_detected, 0);
        assert!(!detector.open_handles.get(&handle).unwrap().drift_reported);

        detector.set_replaying(false);
        detector.evaluate_open_handles(base);

        assert_eq!(detector.deferred_drift_detected, 1);
        assert!(detector.open_handles.get(&handle).unwrap().drift_reported);
    }

    #[test]
    fn consensus_handle_is_not_dropped_until_local_check_completes() {
        let mut detector = detector();
        let handle = FixedBytes::from([12u8; 32]);
        let senders = senders();
        let base = Instant::now();
        let state = HandleState {
            first_seen_block: 10,
            first_seen_block_hash: None,
            first_seen_at: base,
            last_seen_block: 12,
            expected_senders: senders.clone(),
            submissions: vec![
                Submission {
                    sender: senders[0],
                    digests: DigestPair {
                        ciphertext_digest: FixedBytes::from([13u8; 32]),
                        ciphertext128_digest: FixedBytes::from([14u8; 32]),
                    },
                },
                Submission {
                    sender: senders[1],
                    digests: DigestPair {
                        ciphertext_digest: FixedBytes::from([13u8; 32]),
                        ciphertext128_digest: FixedBytes::from([14u8; 32]),
                    },
                },
                Submission {
                    sender: senders[2],
                    digests: DigestPair {
                        ciphertext_digest: FixedBytes::from([13u8; 32]),
                        ciphertext128_digest: FixedBytes::from([14u8; 32]),
                    },
                },
            ],
            consensus: Some(make_consensus_state(
                12,
                FixedBytes::from([13u8; 32]),
                FixedBytes::from([14u8; 32]),
                senders,
            )),
            local_consensus_checked: false,
            drift_reported: false,
        };
        detector.open_handles.insert(handle, state);

        detector.finish_if_complete(handle);
        assert!(detector.open_handles.contains_key(&handle));

        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;
        detector.finish_if_complete(handle);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn matching_submissions_keep_single_variant() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);
        let digests = DigestPair {
            ciphertext_digest: FixedBytes::from([2u8; 32]),
            ciphertext128_digest: FixedBytes::from([3u8; 32]),
        };

        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            digests.ciphertext_digest,
            digests.ciphertext128_digest,
            senders()[0],
            10,
        );
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            digests.ciphertext_digest,
            digests.ciphertext128_digest,
            senders()[1],
            11,
        );

        let state = detector.open_handles.get(&handle).unwrap();
        assert!(!has_multiple_variants(&state.submissions));
        assert_eq!(detector.deferred_drift_detected, 0);
    }

    #[test]
    fn differing_submissions_trigger_drift_once() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
        );
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [9u8; 32],
            [3u8; 32],
            senders()[1],
            11,
        );

        assert_eq!(detector.deferred_drift_detected, 1);
        let state = detector.open_handles.get(&handle).unwrap();
        assert!(has_multiple_variants(&state.submissions));
    }

    #[test]
    fn handle_keeps_expected_senders_snapshot_after_rotation() {
        let mut detector = detector();
        let old_senders = senders();
        let handle_before_rotation = FixedBytes::from([21u8; 32]);
        let handle_after_rotation = FixedBytes::from([22u8; 32]);
        let new_sender = Address::left_padding_from(&[4]);

        submit_digest_event_and_drift_check(
            &mut detector,
            handle_before_rotation,
            [2u8; 32],
            [3u8; 32],
            old_senders[0],
            10,
        );

        let mut rotated_senders = old_senders.clone();
        rotated_senders.push(new_sender);
        detector.set_current_expected_senders(rotated_senders.clone());

        for (i, sender) in old_senders.iter().copied().enumerate().skip(1) {
            submit_digest_event_and_drift_check(
                &mut detector,
                handle_before_rotation,
                [2u8; 32],
                [3u8; 32],
                sender,
                11 + i as u64,
            );
        }
        detector.finalize_completed_without_consensus();

        assert!(!detector.open_handles.contains_key(&handle_before_rotation));
        assert_eq!(detector.deferred_consensus_timeout, 1);

        for (i, sender) in rotated_senders.iter().copied().take(3).enumerate() {
            submit_digest_event_and_drift_check(
                &mut detector,
                handle_after_rotation,
                [4u8; 32],
                [5u8; 32],
                sender,
                20 + i as u64,
            );
        }

        assert!(detector.open_handles.contains_key(&handle_after_rotation));

        submit_digest_event_and_drift_check(
            &mut detector,
            handle_after_rotation,
            [4u8; 32],
            [5u8; 32],
            new_sender,
            23,
        );
        detector.finalize_completed_without_consensus();

        assert!(!detector.open_handles.contains_key(&handle_after_rotation));
        assert_eq!(detector.deferred_consensus_timeout, 2);
    }

    #[test]
    fn all_expected_submissions_without_consensus_alert_and_drop_after_finalize() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        for (i, sender) in senders().into_iter().enumerate() {
            submit_digest_event_and_drift_check(
                &mut detector,
                handle,
                [2u8; 32],
                [3u8; 32],
                sender,
                10 + i as u64,
            );
        }

        assert!(detector.open_handles.contains_key(&handle));
        assert_eq!(detector.deferred_consensus_timeout, 0);

        detector.finalize_completed_without_consensus();

        assert_eq!(detector.deferred_consensus_timeout, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn consensus_on_final_submission_survives_finalize_pass() {
        let mut detector = detector();
        let handle = FixedBytes::from([23u8; 32]);
        let expected = senders();

        for (i, sender) in expected.iter().copied().enumerate() {
            submit_digest_event_and_drift_check(
                &mut detector,
                handle,
                [2u8; 32],
                [3u8; 32],
                sender,
                10 + i as u64,
            );
        }

        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(make_consensus_state(
            12,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            expected,
        ));
        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;

        detector.finalize_completed_without_consensus();

        assert_eq!(detector.deferred_consensus_timeout, 0);
        assert!(detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn consensus_with_missing_submission_after_grace_alerts_and_drops() {
        let mut detector = detector(); // post_consensus_grace = 2s
        let handle = FixedBytes::from([1u8; 32]);
        let base = Instant::now();

        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
            base,
        );
        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[1],
            11,
            base,
        );

        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(make_consensus_state_at(
            12,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            vec![senders()[0], senders()[1]],
            base,
        ));
        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;

        // base + 2s: elapsed since consensus (base) = 2s >= 2s grace, should evict.
        detector.evict_stale(base + Duration::from_secs(2));

        assert_eq!(detector.deferred_missing_submission, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn timeout_without_consensus_alerts_and_drops() {
        let mut detector = detector(); // no_consensus_timeout = 5s
        let handle = FixedBytes::from([1u8; 32]);
        let base = Instant::now();

        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
            base,
        );

        // base + 5s: elapsed since first_seen (base) = 5s >= 5s timeout, should evict.
        detector.evict_stale(base + Duration::from_secs(5));

        assert_eq!(detector.deferred_consensus_timeout, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn missing_submission_within_grace_period_is_not_evicted() {
        let mut detector = detector(); // post_consensus_grace = 2s
        let handle = FixedBytes::from([1u8; 32]);
        let base = Instant::now();

        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
            base,
        );
        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[1],
            11,
            base,
        );

        // Inject consensus at base + 1s and mark local check done.
        let consensus_at = base + Duration::from_secs(1);
        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(make_consensus_state_at(
            12,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            vec![senders()[0], senders()[1]],
            consensus_at,
        ));
        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;

        // consensus_at + 1s: 1 < 2 (grace), should NOT evict.
        detector.evict_stale(consensus_at + Duration::from_secs(1));

        assert_eq!(detector.deferred_missing_submission, 0);
        assert!(detector.open_handles.contains_key(&handle));

        // consensus_at + 2s: 2 >= 2 (grace), should evict.
        detector.evict_stale(consensus_at + Duration::from_secs(2));

        assert_eq!(detector.deferred_missing_submission, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn timeout_within_no_consensus_window_is_not_evicted() {
        let mut detector = detector(); // no_consensus_timeout = 5s
        let handle = FixedBytes::from([1u8; 32]);
        let base = Instant::now();

        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
            base,
        );

        // base + 4s: 4 < 5 (timeout window), should NOT evict.
        detector.evict_stale(base + Duration::from_secs(4));

        assert_eq!(detector.deferred_consensus_timeout, 0);
        assert!(detector.open_handles.contains_key(&handle));

        // base + 5s: 5 >= 5, should evict.
        detector.evict_stale(base + Duration::from_secs(5));

        assert_eq!(detector.deferred_consensus_timeout, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn consensus_before_any_submission_creates_handle_state() {
        let mut detector = detector(); // post_consensus_grace = 2s
        let handle = FixedBytes::from([0xBE; 32]);
        let digest = FixedBytes::from([0xAA; 32]);
        let digest128 = FixedBytes::from([0xBB; 32]);
        let base = Instant::now();

        // Manually inject consensus without any prior observe_submission.
        // This simulates consensus arriving before any peer submission is seen.
        detector.open_handles.insert(
            handle,
            HandleState {
                first_seen_block: 20,
                first_seen_block_hash: None,
                first_seen_at: base,
                last_seen_block: 20,
                expected_senders: senders(),
                submissions: Vec::new(),
                consensus: Some(make_consensus_state_at(
                    20,
                    digest,
                    digest128,
                    senders(),
                    base,
                )),
                local_consensus_checked: true,
                drift_reported: false,
            },
        );

        // Handle should remain open (0 submissions != 3 expected senders).
        detector.finish_if_complete(handle);
        assert!(detector.open_handles.contains_key(&handle));

        // After grace period (2s), should alert about missing submissions.
        detector.evict_stale(base + Duration::from_secs(3));
        assert_eq!(detector.deferred_missing_submission, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn equivocation_warns_but_does_not_duplicate_submission() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);
        let sender = senders()[0];

        // First submission from sender.
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            sender,
            10,
        );

        // Same sender, different digests (equivocation).
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [9u8; 32],
            [3u8; 32],
            sender,
            11,
        );

        let state = detector.open_handles.get(&handle).unwrap();
        // Should still have only 1 submission (the first one).
        assert_eq!(state.submissions.len(), 1);
        assert_eq!(
            state.submissions[0].digests.ciphertext_digest,
            FixedBytes::from([2u8; 32])
        );
        // No drift_detected metric (equivocation is not multi-variant drift).
        assert_eq!(detector.deferred_drift_detected, 0);
    }

    #[test]
    fn duplicate_submission_same_digests_is_ignored() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);
        let sender = senders()[0];

        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            sender,
            10,
        );

        // Exact same submission again.
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            sender,
            11,
        );

        let state = detector.open_handles.get(&handle).unwrap();
        assert_eq!(state.submissions.len(), 1);
        assert_eq!(detector.deferred_drift_detected, 0);
    }

    #[test]
    fn local_check_not_ready_evicts_after_timeout() {
        // Consensus arrives but local_consensus_checked stays false (simulating
        // the DB digest never becoming available). After no_consensus_timeout
        // the handle should be evicted with a warning.
        let mut detector = detector(); // no_consensus_timeout = 5s
        let handle = FixedBytes::from([0xDD; 32]);
        let base = Instant::now();

        submit_at(
            &mut detector,
            handle,
            [2u8; 32],
            [3u8; 32],
            senders()[0],
            10,
            base,
        );

        let consensus_at = base + Duration::from_secs(1);
        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(make_consensus_state_at(
            12,
            FixedBytes::from([2u8; 32]),
            FixedBytes::from([3u8; 32]),
            vec![senders()[0]],
            consensus_at,
        ));
        // local_consensus_checked remains false (default).

        // Within timeout: consensus_at + 4s = 4 < 5, should not evict.
        detector.evict_stale(consensus_at + Duration::from_secs(4));
        assert!(detector.open_handles.contains_key(&handle));

        // At timeout: consensus_at + 5s = 5 >= 5, should evict.
        detector.evict_stale(consensus_at + Duration::from_secs(5));
        assert!(!detector.open_handles.contains_key(&handle));
        // This path (consensus observed, local digests never available) should not
        // bump consensus_timeout or missing_submission — it's a distinct warning.
        assert_eq!(detector.deferred_consensus_timeout, 0);
        assert_eq!(detector.deferred_missing_submission, 0);
        assert_eq!(detector.deferred_drift_detected, 0);
    }

    #[test]
    fn flush_resets_counters() {
        let mut detector = detector();
        detector.deferred_drift_detected = 1;
        detector.deferred_consensus_timeout = 2;
        detector.deferred_missing_submission = 3;

        detector.flush_metrics();

        assert_eq!(detector.deferred_drift_detected, 0);
        assert_eq!(detector.deferred_consensus_timeout, 0);
        assert_eq!(detector.deferred_missing_submission, 0);
    }

    use serial_test::serial;
    use sqlx::postgres::PgPoolOptions;
    use std::time::Duration;
    use test_harness::db_utils::insert_ciphertext_digest;
    use test_harness::instance::ImportMode;

    async fn setup_db() -> (Pool<Postgres>, Option<test_harness::instance::DBInstance>) {
        let instance = test_harness::instance::setup_test_db(ImportMode::None)
            .await
            .expect("test db");
        let pool = PgPoolOptions::new()
            .max_connections(2)
            .acquire_timeout(Duration::from_secs(5))
            .connect(instance.db_url.as_str())
            .await
            .expect("pool");
        sqlx::query("TRUNCATE ciphertext_digest")
            .execute(&pool)
            .await
            .expect("truncate");
        (pool, Some(instance))
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_mismatch_increments_drift_metric() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAA; 32];

        insert_ciphertext_digest(
            &pool,
            12345,
            [0u8; 32],
            &handle,
            &[0xBB; 32],
            &[0xCC; 32],
            0,
        )
        .await
        .unwrap();

        let mut detector = detector();
        detector
            .handle_consensus(
                make_consensus_event(
                    FixedBytes::from(handle),
                    FixedBytes::from([0xFF; 32]),
                    FixedBytes::from([0xCC; 32]),
                    vec![senders()[0], senders()[1], senders()[2]],
                ),
                context(10),
                &pool,
            )
            .await
            .unwrap();

        assert_eq!(detector.deferred_drift_detected, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn rebuild_defers_consensus_check_until_alerts_resume() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAB; 32];

        insert_ciphertext_digest(
            &pool,
            12345,
            [0u8; 32],
            &handle,
            &[0xBB; 32],
            &[0xCC; 32],
            0,
        )
        .await
        .unwrap();

        let mut detector = detector();
        detector.set_replaying(true);
        detector
            .handle_consensus(
                make_consensus_event(
                    FixedBytes::from(handle),
                    FixedBytes::from([0xFF; 32]),
                    FixedBytes::from([0xCC; 32]),
                    vec![senders()[0], senders()[1], senders()[2]],
                ),
                context(10),
                &pool,
            )
            .await
            .unwrap();

        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(!state.local_consensus_checked);
        assert_eq!(detector.deferred_drift_detected, 0);

        detector.set_replaying(false);
        detector
            .refresh_pending_consensus_checks(&pool)
            .await
            .unwrap();

        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(state.local_consensus_checked);
        assert_eq!(detector.deferred_drift_detected, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_defers_when_local_digests_are_null() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAC; 32];

        // Insert a row with NULL ciphertext digests (digest computation not yet complete).
        sqlx::query(
            "INSERT INTO ciphertext_digest (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128, txn_limited_retries_count)
             VALUES (12345, $1, $2, $3, $4, 0)",
        )
        .bind(&[0u8; 32][..])
        .bind(&handle[..])
        .bind(None::<&[u8]>)
        .bind(None::<&[u8]>)
        .execute(&pool)
        .await
        .unwrap();

        let mut detector = detector();
        detector
            .handle_consensus(
                make_consensus_event(
                    FixedBytes::from(handle),
                    FixedBytes::from([0xFF; 32]),
                    FixedBytes::from([0xCC; 32]),
                    vec![senders()[0], senders()[1], senders()[2]],
                ),
                context(10),
                &pool,
            )
            .await
            .unwrap();

        // Consensus was processed but local check should be deferred (digests NULL).
        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(!state.local_consensus_checked);
        assert_eq!(detector.deferred_drift_detected, 0);

        // Now populate the digests (simulating the worker finishing computation).
        let local_ct = vec![0xBBu8; 32];
        let local_ct128 = vec![0xCCu8; 32];
        sqlx::query(
            "UPDATE ciphertext_digest SET ciphertext = $1, ciphertext128 = $2 WHERE handle = $3",
        )
        .bind(&local_ct)
        .bind(&local_ct128)
        .bind(&handle[..])
        .execute(&pool)
        .await
        .unwrap();

        // refresh should now complete the check and detect the mismatch.
        detector
            .refresh_pending_consensus_checks(&pool)
            .await
            .unwrap();

        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(state.local_consensus_checked);
        // Consensus digest [0xFF] != local digest [0xBB] → drift.
        assert_eq!(detector.deferred_drift_detected, 1);
    }

    #[tokio::test]
    #[serial(db)]
    async fn unexpected_sender_does_not_block_completion() {
        let (pool, _inst) = setup_db().await;
        let handle_bytes = [0xAE; 32];
        let handle = FixedBytes::from(handle_bytes);
        let digest = FixedBytes::from([0xEE; 32]);
        let digest128 = FixedBytes::from([0xDD; 32]);

        insert_ciphertext_digest(
            &pool,
            12345,
            [0u8; 32],
            &handle_bytes,
            &[0xEE; 32],
            &[0xDD; 32],
            0,
        )
        .await
        .unwrap();

        let mut detector = detector(); // expects 3 senders

        // Submit from 3 expected senders + 1 unexpected sender.
        for &sender in &senders() {
            submit_digest_event_and_drift_check(
                &mut detector,
                handle,
                digest,
                digest128,
                sender,
                10,
            );
        }
        let unexpected_sender = Address::left_padding_from(&[99]);
        submit_digest_event_and_drift_check(
            &mut detector,
            handle,
            digest,
            digest128,
            unexpected_sender,
            10,
        );

        // Process consensus.
        detector
            .handle_consensus(
                make_consensus_event(handle, digest, digest128, senders()),
                context(11),
                &pool,
            )
            .await
            .unwrap();

        // Handle should be completed and removed (not stuck open).
        assert!(
            !detector.open_handles.contains_key(&handle),
            "handle with unexpected sender should still complete"
        );
    }

    #[tokio::test]
    #[serial(db)]
    async fn consensus_no_drift_when_local_digests_match() {
        let (pool, _inst) = setup_db().await;
        let handle = [0xAD; 32];
        let digest = [0xEE; 32];
        let digest128 = [0xDD; 32];

        insert_ciphertext_digest(&pool, 12345, [0u8; 32], &handle, &digest, &digest128, 0)
            .await
            .unwrap();

        let mut detector = detector();
        detector
            .handle_consensus(
                make_consensus_event(
                    FixedBytes::from(handle),
                    FixedBytes::from(digest),
                    FixedBytes::from(digest128),
                    vec![senders()[0], senders()[1], senders()[2]],
                ),
                context(10),
                &pool,
            )
            .await
            .unwrap();

        // Digests match → no drift, local check complete.
        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(state.local_consensus_checked);
        assert_eq!(detector.deferred_drift_detected, 0);
    }
}
