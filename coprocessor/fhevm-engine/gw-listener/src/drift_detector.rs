use std::collections::HashMap;

use alloy::primitives::{Address, FixedBytes, B256};
use fhevm_engine_common::chain_id::ChainId;
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
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct DigestPair {
    ciphertext_digest: FixedBytes<32>,
    ciphertext128_digest: FixedBytes<32>,
}

#[derive(Clone, Copy, Debug)]
struct Submission {
    sender: Address,
    digests: DigestPair,
}

#[derive(Clone, Debug)]
struct ConsensusState {
    block_number: u64,
    block_hash: Option<B256>,
    tx_hash: Option<B256>,
    log_index: Option<u64>,
    digests: DigestPair,
    senders: Vec<Address>,
}

#[derive(Clone, Debug)]
struct HandleState {
    first_seen_block: u64,
    first_seen_block_hash: Option<B256>,
    last_seen_block: u64,
    expected_senders: Vec<Address>,
    submissions: Vec<Submission>,
    consensus: Option<ConsensusState>,
    local_consensus_checked: bool,
    missing_submission_reported: bool,
    drift_reported: bool,
}

#[derive(Default)]
struct DeferredMetrics {
    drift_detected: u64,
    consensus_timeout: u64,
    missing_submission: u64,
}

pub(crate) struct DriftDetector {
    current_expected_senders: Vec<Address>,
    open_handles: HashMap<FixedBytes<32>, HandleState>,
    host_chain_id: ChainId,
    local_node_id: String,
    no_consensus_timeout_blocks: u64,
    post_consensus_grace_blocks: u64,
    deferred_metrics: DeferredMetrics,
    alerts_enabled: bool,
}

impl DriftDetector {
    pub(crate) fn new(
        expected_senders: Vec<Address>,
        host_chain_id: ChainId,
        no_consensus_timeout_blocks: u64,
        post_consensus_grace_blocks: u64,
    ) -> Self {
        Self {
            current_expected_senders: expected_senders,
            open_handles: HashMap::new(),
            host_chain_id,
            local_node_id: std::env::var("HOSTNAME").unwrap_or_else(|_| "unknown".to_owned()),
            no_consensus_timeout_blocks,
            post_consensus_grace_blocks,
            deferred_metrics: DeferredMetrics::default(),
            alerts_enabled: true,
        }
    }

    pub(crate) fn set_alerts_enabled(&mut self, alerts_enabled: bool) {
        self.alerts_enabled = alerts_enabled;
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
        let expected_senders = self.current_expected_senders.clone();

        let state = self
            .open_handles
            .entry(handle)
            .or_insert_with(|| new_handle_state(context, expected_senders));
        state.last_seen_block = context.block_number;

        if let Some(existing) = state
            .submissions
            .iter()
            .find(|submission| submission.sender == event.coprocessorTxSender)
        {
            if self.alerts_enabled && existing.digests != digests {
                warn!(
                    handle = %handle,
                    host_chain_id = self.host_chain_id.as_i64(),
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

        if self.alerts_enabled && !state.drift_reported {
            let variants = variant_summaries(&state.submissions);
            if variants.len() > 1 {
                warn!(
                    handle = %handle,
                    host_chain_id = self.host_chain_id.as_i64(),
                    local_node_id = %self.local_node_id,
                    first_seen_block = state.first_seen_block,
                    first_seen_block_hash = ?state.first_seen_block_hash,
                    block_number = context.block_number,
                    block_hash = ?context.block_hash,
                    tx_hash = ?context.tx_hash,
                    log_index = ?context.log_index,
                    variant_count = variants.len(),
                    variants = ?variants,
                    seen_senders = ?seen_sender_strings(&state.submissions),
                    missing_senders = ?missing_sender_strings(&state.expected_senders, &state.submissions),
                    source = "peer_submission",
                    "Drift detected: observed multiple digest variants for handle"
                );
                state.drift_reported = true;
                self.deferred_metrics.drift_detected += 1;
            }
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
        let expected_senders = self.current_expected_senders.clone();
        let state = self
            .open_handles
            .entry(handle)
            .or_insert_with(|| new_handle_state(context, expected_senders));
        state.last_seen_block = context.block_number;
        state.consensus = Some(ConsensusState {
            block_number: context.block_number,
            block_hash: context.block_hash,
            tx_hash: context.tx_hash,
            log_index: context.log_index,
            digests: DigestPair {
                ciphertext_digest: event.ciphertextDigest,
                ciphertext128_digest: event.snsCiphertextDigest,
            },
            senders: event.coprocessorTxSenders.clone(),
        });
        state.local_consensus_checked = false;
        CONSENSUS_LATENCY_BLOCKS_HISTOGRAM
            .observe(context.block_number.saturating_sub(state.first_seen_block) as f64);
        self.try_check_local_consensus(handle, db_pool).await
    }

    pub(crate) async fn refresh_pending_consensus_checks(
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
        handle: FixedBytes<32>,
        db_pool: &Pool<Postgres>,
    ) -> anyhow::Result<()> {
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

        let Some(row) = row else {
            debug!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                block_number = consensus.block_number,
                tx_hash = ?consensus.tx_hash,
                "Consensus arrived before local digest was available; deferring drift check"
            );
            return Ok(());
        };

        let local_ciphertext_digest: Option<Vec<u8>> = row.get("ciphertext");
        let local_ciphertext128_digest: Option<Vec<u8>> = row.get("ciphertext128");

        let (Some(local_ciphertext_digest), Some(local_ciphertext128_digest)) =
            (local_ciphertext_digest, local_ciphertext128_digest)
        else {
            debug!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                block_number = consensus.block_number,
                tx_hash = ?consensus.tx_hash,
                "Consensus arrived before local digests were ready; deferring drift check"
            );
            return Ok(());
        };

        if self.alerts_enabled
            && (consensus.digests.ciphertext_digest.as_slice()
                != local_ciphertext_digest.as_slice()
                || consensus.digests.ciphertext128_digest.as_slice()
                    != local_ciphertext128_digest.as_slice())
        {
            let local_digests =
                digest_pair_from_db_digests(&local_ciphertext_digest, &local_ciphertext128_digest)?;
            let local_variant_sender_count =
                sender_count_for_variant(&state.submissions, local_digests);
            let consensus_variant_sender_count =
                sender_count_for_variant(&state.submissions, consensus.digests);
            let observed_variants = variant_summaries(&state.submissions);
            warn!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                block_number = consensus.block_number,
                block_hash = ?consensus.block_hash,
                tx_hash = ?consensus.tx_hash,
                log_index = ?consensus.log_index,
                consensus_senders = ?address_strings(&consensus.senders),
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
            self.deferred_metrics.drift_detected += 1;
        }

        if !self.alerts_enabled {
            return Ok(());
        }

        let Some(state) = self.open_handles.get_mut(&handle) else {
            return Ok(());
        };
        state.local_consensus_checked = true;
        self.finish_if_complete(handle);
        Ok(())
    }

    pub(crate) fn evict_stale(&mut self, current_block: u64) {
        let mut finished = Vec::new();

        for (handle, state) in &mut self.open_handles {
            if let Some(consensus) = &state.consensus {
                if !state.local_consensus_checked {
                    if current_block.saturating_sub(consensus.block_number)
                        < self.no_consensus_timeout_blocks
                    {
                        continue;
                    }

                    warn!(
                        handle = %handle,
                        host_chain_id = self.host_chain_id.as_i64(),
                        local_node_id = %self.local_node_id,
                        first_seen_block = state.first_seen_block,
                        first_seen_block_hash = ?state.first_seen_block_hash,
                        last_seen_block = state.last_seen_block,
                        consensus_block = consensus.block_number,
                        consensus_block_hash = ?consensus.block_hash,
                        consensus_tx_hash = ?consensus.tx_hash,
                        consensus_log_index = ?consensus.log_index,
                        "Consensus was observed but local digests never became available for comparison"
                    );
                    finished.push(*handle);
                    continue;
                }

                if state.submissions.len() != state.expected_senders.len() {
                    if state.missing_submission_reported
                        || current_block.saturating_sub(consensus.block_number)
                            < self.post_consensus_grace_blocks
                    {
                        continue;
                    }

                    warn!(
                        handle = %handle,
                        host_chain_id = self.host_chain_id.as_i64(),
                        local_node_id = %self.local_node_id,
                        first_seen_block = state.first_seen_block,
                        first_seen_block_hash = ?state.first_seen_block_hash,
                        last_seen_block = state.last_seen_block,
                        consensus_block = consensus.block_number,
                        consensus_block_hash = ?consensus.block_hash,
                        consensus_tx_hash = ?consensus.tx_hash,
                        consensus_log_index = ?consensus.log_index,
                        consensus_senders = ?address_strings(&consensus.senders),
                        consensus_ciphertext_digest = %consensus.digests.ciphertext_digest,
                        consensus_ciphertext128_digest = %consensus.digests.ciphertext128_digest,
                        seen_senders = ?seen_sender_strings(&state.submissions),
                        missing_senders = ?missing_sender_strings(&state.expected_senders, &state.submissions),
                        variant_count = variant_summaries(&state.submissions).len(),
                        variants = ?variant_summaries(&state.submissions),
                        "Consensus reached but some coprocessors never submitted this handle"
                    );
                    self.deferred_metrics.missing_submission += 1;
                    state.missing_submission_reported = true;
                    finished.push(*handle);
                    continue;
                }

                continue;
            }

            if current_block.saturating_sub(state.first_seen_block)
                < self.no_consensus_timeout_blocks
            {
                continue;
            }

            warn!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                last_seen_block = state.last_seen_block,
                seen_senders = ?seen_sender_strings(&state.submissions),
                missing_senders = ?missing_sender_strings(&state.expected_senders, &state.submissions),
                variant_count = variant_summaries(&state.submissions).len(),
                variants = ?variant_summaries(&state.submissions),
                "Handle timed out before consensus was observed"
            );
            self.deferred_metrics.consensus_timeout += 1;
            finished.push(*handle);
        }

        for handle in finished {
            self.open_handles.remove(&handle);
        }
    }

    pub(crate) fn flush_metrics(&mut self) {
        if self.deferred_metrics.drift_detected == 0
            && self.deferred_metrics.consensus_timeout == 0
            && self.deferred_metrics.missing_submission == 0
        {
            return;
        }

        DRIFT_DETECTED_COUNTER.inc_by(self.deferred_metrics.drift_detected);
        CONSENSUS_TIMEOUT_COUNTER.inc_by(self.deferred_metrics.consensus_timeout);
        MISSING_SUBMISSION_COUNTER.inc_by(self.deferred_metrics.missing_submission);
        self.deferred_metrics = DeferredMetrics::default();
    }

    pub(crate) fn evaluate_open_handles(&mut self, current_block: u64) {
        if !self.alerts_enabled {
            return;
        }

        let drift_handles = self
            .open_handles
            .iter()
            .filter_map(|(handle, state)| {
                (!state.drift_reported && variant_summaries(&state.submissions).len() > 1)
                    .then_some(*handle)
            })
            .collect::<Vec<_>>();

        for handle in drift_handles {
            let Some(state) = self.open_handles.get_mut(&handle) else {
                continue;
            };
            warn!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                last_seen_block = state.last_seen_block,
                variant_count = variant_summaries(&state.submissions).len(),
                variants = ?variant_summaries(&state.submissions),
                seen_senders = ?seen_sender_strings(&state.submissions),
                missing_senders = ?missing_sender_strings(&state.expected_senders, &state.submissions),
                source = "peer_submission",
                "Drift detected: observed multiple digest variants for handle"
            );
            state.drift_reported = true;
            self.deferred_metrics.drift_detected += 1;
        }

        self.finalize_completed_without_consensus();

        self.evict_stale(current_block);
    }

    pub(crate) fn earliest_open_block(&self) -> Option<u64> {
        self.open_handles
            .values()
            .map(|state| state.first_seen_block)
            .min()
    }

    pub(crate) fn finalize_completed_without_consensus(&mut self) {
        if !self.alerts_enabled {
            return;
        }

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

            warn!(
                handle = %handle,
                host_chain_id = self.host_chain_id.as_i64(),
                local_node_id = %self.local_node_id,
                first_seen_block = state.first_seen_block,
                first_seen_block_hash = ?state.first_seen_block_hash,
                last_seen_block = state.last_seen_block,
                seen_senders = ?seen_sender_strings(&state.submissions),
                variant_count = variant_summaries(&state.submissions).len(),
                variants = ?variant_summaries(&state.submissions),
                "All expected coprocessors submitted but no consensus event was observed"
            );
            self.deferred_metrics.consensus_timeout += 1;
            self.open_handles.remove(&handle);
        }
    }

    fn finish_if_complete(&mut self, handle: FixedBytes<32>) {
        let Some(state) = self.open_handles.get(&handle) else {
            return;
        };

        if state.submissions.len() != state.expected_senders.len() {
            return;
        }

        if state.consensus.is_some() {
            if !state.local_consensus_checked {
                return;
            }
            let consensus_block = state
                .consensus
                .as_ref()
                .map(|consensus| consensus.block_number)
                .unwrap_or(state.last_seen_block);
            POST_CONSENSUS_COMPLETION_BLOCKS_HISTOGRAM
                .observe(state.last_seen_block.saturating_sub(consensus_block) as f64);
            self.open_handles.remove(&handle);
        }
    }
}

fn new_handle_state(context: EventContext, expected_senders: Vec<Address>) -> HandleState {
    let submission_capacity = expected_senders.len();
    HandleState {
        first_seen_block: context.block_number,
        first_seen_block_hash: context.block_hash,
        last_seen_block: context.block_number,
        expected_senders,
        submissions: Vec::with_capacity(submission_capacity),
        consensus: None,
        local_consensus_checked: false,
        missing_submission_reported: false,
        drift_reported: false,
    }
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
                address_strings(&senders)
            )
        })
        .collect()
}

fn seen_sender_strings(submissions: &[Submission]) -> Vec<String> {
    address_strings(
        &submissions
            .iter()
            .map(|submission| submission.sender)
            .collect::<Vec<_>>(),
    )
}

fn missing_sender_strings(expected_senders: &[Address], submissions: &[Submission]) -> Vec<String> {
    let seen = submissions
        .iter()
        .map(|submission| submission.sender)
        .collect::<Vec<_>>();
    address_strings(
        &expected_senders
            .iter()
            .copied()
            .filter(|sender| !seen.contains(sender))
            .collect::<Vec<_>>(),
    )
}

fn sender_count_for_variant(submissions: &[Submission], digests: DigestPair) -> usize {
    submissions
        .iter()
        .filter(|submission| submission.digests == digests)
        .count()
}

fn digest_pair_from_db_digests(
    ciphertext_digest: &[u8],
    ciphertext128_digest: &[u8],
) -> anyhow::Result<DigestPair> {
    let ciphertext_digest: [u8; 32] = ciphertext_digest.try_into().map_err(|_| {
        anyhow::anyhow!(
            "Failed to convert local ciphertext digest to [u8; 32] (len={}): 0x{}",
            ciphertext_digest.len(),
            to_hex(ciphertext_digest)
        )
    })?;
    let ciphertext128_digest: [u8; 32] = ciphertext128_digest.try_into().map_err(|_| {
        anyhow::anyhow!(
            "Failed to convert local ciphertext128 digest to [u8; 32] (len={}): 0x{}",
            ciphertext128_digest.len(),
            to_hex(ciphertext128_digest)
        )
    })?;

    Ok(DigestPair {
        ciphertext_digest: FixedBytes::from(ciphertext_digest),
        ciphertext128_digest: FixedBytes::from(ciphertext128_digest),
    })
}

fn address_strings(addresses: &[Address]) -> Vec<String> {
    addresses.iter().map(ToString::to_string).collect()
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
        let mut detector = DriftDetector::new(
            vec![sender_a, sender_b, sender_c],
            ChainId::try_from(12345_u64).unwrap(),
            50,
            10,
        );

        detector.set_alerts_enabled(false);

        detector.observe_submission(
            make_submission_event(handle, digest_a, digest_128, sender_a),
            EventContext {
                block_number: 100,
                block_hash: None,
                tx_hash: None,
                log_index: Some(0),
            },
        );
        detector.observe_submission(
            make_submission_event(handle, digest_b, digest_128, sender_b),
            EventContext {
                block_number: 103,
                block_hash: None,
                tx_hash: None,
                log_index: Some(0),
            },
        );

        let state = detector
            .open_handles
            .get(&handle)
            .expect("handle is tracked");
        assert_eq!(state.first_seen_block, 100);
        assert_eq!(state.last_seen_block, 103);
        assert_eq!(state.submissions.len(), 2);
        assert_eq!(variant_summaries(&state.submissions).len(), 2);
        assert!(!state.drift_reported);
        assert_eq!(detector.deferred_metrics.drift_detected, 0);

        detector.set_alerts_enabled(true);
        detector.evaluate_open_handles(103);

        let state = detector
            .open_handles
            .get(&handle)
            .expect("handle remains open");
        assert!(state.drift_reported);
        assert_eq!(state.first_seen_block, 100);
        assert_eq!(state.last_seen_block, 103);
        assert_eq!(state.submissions.len(), 2);
        assert_eq!(detector.deferred_metrics.drift_detected, 1);
        assert_eq!(detector.deferred_metrics.consensus_timeout, 0);
    }

    fn make_submission_event(
        handle: FixedBytes<32>,
        ciphertext_digest: FixedBytes<32>,
        ciphertext128_digest: FixedBytes<32>,
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
        handle: FixedBytes<32>,
        ciphertext_digest: FixedBytes<32>,
        ciphertext128_digest: FixedBytes<32>,
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
        }
    }

    fn senders() -> Vec<Address> {
        vec![
            Address::left_padding_from(&[1]),
            Address::left_padding_from(&[2]),
            Address::left_padding_from(&[3]),
        ]
    }

    fn detector() -> DriftDetector {
        DriftDetector::new(senders(), ChainId::try_from(12345_u64).unwrap(), 5, 2)
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

        detector.observe_submission(
            make_submission_event(
                handle_b,
                digest_b.ciphertext_digest,
                digest_b.ciphertext128_digest,
                senders[0],
            ),
            context(20),
        );
        detector.observe_submission(
            make_submission_event(
                handle_a,
                digest_a.ciphertext_digest,
                digest_a.ciphertext128_digest,
                senders[0],
            ),
            context(10),
        );

        assert_eq!(detector.earliest_open_block(), Some(10));

        detector.observe_submission(
            make_submission_event(
                handle_a,
                digest_a.ciphertext_digest,
                digest_a.ciphertext128_digest,
                senders[1],
            ),
            context(11),
        );
        detector.observe_submission(
            make_submission_event(
                handle_a,
                digest_a.ciphertext_digest,
                digest_a.ciphertext128_digest,
                senders[2],
            ),
            context(12),
        );
        detector.finalize_completed_without_consensus();

        assert_eq!(detector.earliest_open_block(), Some(20));
    }

    #[test]
    fn rebuild_replays_silently_then_alerts_once_on_evaluate() {
        let mut detector = detector();
        let handle = FixedBytes::from([7u8; 32]);
        let senders = senders();

        detector.set_alerts_enabled(false);
        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([8u8; 32]),
                FixedBytes::from([9u8; 32]),
                senders[0],
            ),
            context(10),
        );
        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([10u8; 32]),
                FixedBytes::from([11u8; 32]),
                senders[1],
            ),
            context(11),
        );

        assert_eq!(detector.deferred_metrics.drift_detected, 0);
        assert!(!detector.open_handles.get(&handle).unwrap().drift_reported);

        detector.set_alerts_enabled(true);
        detector.evaluate_open_handles(11);

        assert_eq!(detector.deferred_metrics.drift_detected, 1);
        assert!(detector.open_handles.get(&handle).unwrap().drift_reported);
    }

    #[test]
    fn consensus_handle_is_not_dropped_until_local_check_completes() {
        let mut detector = detector();
        let handle = FixedBytes::from([12u8; 32]);
        let senders = senders();
        let state = HandleState {
            first_seen_block: 10,
            first_seen_block_hash: None,
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
            consensus: Some(ConsensusState {
                block_number: 12,
                block_hash: None,
                tx_hash: None,
                log_index: None,
                digests: DigestPair {
                    ciphertext_digest: FixedBytes::from([13u8; 32]),
                    ciphertext128_digest: FixedBytes::from([14u8; 32]),
                },
                senders,
            }),
            local_consensus_checked: false,
            missing_submission_reported: false,
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

        detector.observe_submission(
            make_submission_event(
                handle,
                digests.ciphertext_digest,
                digests.ciphertext128_digest,
                senders()[0],
            ),
            context(10),
        );
        detector.observe_submission(
            make_submission_event(
                handle,
                digests.ciphertext_digest,
                digests.ciphertext128_digest,
                senders()[1],
            ),
            context(11),
        );

        let state = detector.open_handles.get(&handle).unwrap();
        assert_eq!(variant_summaries(&state.submissions).len(), 1);
        assert_eq!(detector.deferred_metrics.drift_detected, 0);
    }

    #[test]
    fn differing_submissions_trigger_drift_once() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([2u8; 32]),
                FixedBytes::from([3u8; 32]),
                senders()[0],
            ),
            context(10),
        );
        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([9u8; 32]),
                FixedBytes::from([3u8; 32]),
                senders()[1],
            ),
            context(11),
        );

        assert_eq!(detector.deferred_metrics.drift_detected, 1);
        let state = detector.open_handles.get(&handle).unwrap();
        assert_eq!(variant_summaries(&state.submissions).len(), 2);
    }

    #[test]
    fn handle_keeps_expected_senders_snapshot_after_rotation() {
        let mut detector = detector();
        let old_senders = senders();
        let handle_before_rotation = FixedBytes::from([21u8; 32]);
        let handle_after_rotation = FixedBytes::from([22u8; 32]);
        let new_sender = Address::left_padding_from(&[4]);

        detector.observe_submission(
            make_submission_event(
                handle_before_rotation,
                FixedBytes::from([2u8; 32]),
                FixedBytes::from([3u8; 32]),
                old_senders[0],
            ),
            context(10),
        );

        let mut rotated_senders = old_senders.clone();
        rotated_senders.push(new_sender);
        detector.set_current_expected_senders(rotated_senders.clone());

        for (index, sender) in old_senders.iter().copied().enumerate().skip(1) {
            detector.observe_submission(
                make_submission_event(
                    handle_before_rotation,
                    FixedBytes::from([2u8; 32]),
                    FixedBytes::from([3u8; 32]),
                    sender,
                ),
                context(11 + index as u64),
            );
        }
        detector.finalize_completed_without_consensus();

        assert!(!detector.open_handles.contains_key(&handle_before_rotation));
        assert_eq!(detector.deferred_metrics.consensus_timeout, 1);

        for (index, sender) in rotated_senders.iter().copied().take(3).enumerate() {
            detector.observe_submission(
                make_submission_event(
                    handle_after_rotation,
                    FixedBytes::from([4u8; 32]),
                    FixedBytes::from([5u8; 32]),
                    sender,
                ),
                context(20 + index as u64),
            );
        }

        assert!(detector.open_handles.contains_key(&handle_after_rotation));

        detector.observe_submission(
            make_submission_event(
                handle_after_rotation,
                FixedBytes::from([4u8; 32]),
                FixedBytes::from([5u8; 32]),
                new_sender,
            ),
            context(23),
        );
        detector.finalize_completed_without_consensus();

        assert!(!detector.open_handles.contains_key(&handle_after_rotation));
        assert_eq!(detector.deferred_metrics.consensus_timeout, 2);
    }

    #[test]
    fn all_expected_submissions_without_consensus_alert_and_drop_after_finalize() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        for (index, sender) in senders().into_iter().enumerate() {
            detector.observe_submission(
                make_submission_event(
                    handle,
                    FixedBytes::from([2u8; 32]),
                    FixedBytes::from([3u8; 32]),
                    sender,
                ),
                context(10 + index as u64),
            );
        }

        assert!(detector.open_handles.contains_key(&handle));
        assert_eq!(detector.deferred_metrics.consensus_timeout, 0);

        detector.finalize_completed_without_consensus();

        assert_eq!(detector.deferred_metrics.consensus_timeout, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn consensus_on_final_submission_survives_finalize_pass() {
        let mut detector = detector();
        let handle = FixedBytes::from([23u8; 32]);
        let expected = senders();

        for (index, sender) in expected.iter().copied().enumerate() {
            detector.observe_submission(
                make_submission_event(
                    handle,
                    FixedBytes::from([2u8; 32]),
                    FixedBytes::from([3u8; 32]),
                    sender,
                ),
                context(10 + index as u64),
            );
        }

        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(ConsensusState {
            block_number: 12,
            block_hash: None,
            tx_hash: None,
            log_index: None,
            digests: DigestPair {
                ciphertext_digest: FixedBytes::from([2u8; 32]),
                ciphertext128_digest: FixedBytes::from([3u8; 32]),
            },
            senders: expected,
        });
        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;

        detector.finalize_completed_without_consensus();

        assert_eq!(detector.deferred_metrics.consensus_timeout, 0);
        assert!(detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn consensus_with_missing_submission_after_grace_alerts_and_drops() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([2u8; 32]),
                FixedBytes::from([3u8; 32]),
                senders()[0],
            ),
            context(10),
        );
        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([2u8; 32]),
                FixedBytes::from([3u8; 32]),
                senders()[1],
            ),
            context(11),
        );

        let pool = None::<Pool<Postgres>>;
        drop(pool);

        detector.open_handles.get_mut(&handle).unwrap().consensus = Some(ConsensusState {
            block_number: 12,
            block_hash: None,
            tx_hash: None,
            log_index: None,
            digests: DigestPair {
                ciphertext_digest: FixedBytes::from([2u8; 32]),
                ciphertext128_digest: FixedBytes::from([3u8; 32]),
            },
            senders: vec![senders()[0], senders()[1]],
        });
        detector
            .open_handles
            .get_mut(&handle)
            .unwrap()
            .local_consensus_checked = true;

        detector.evict_stale(14);

        assert_eq!(detector.deferred_metrics.missing_submission, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn timeout_without_consensus_alerts_and_drops() {
        let mut detector = detector();
        let handle = FixedBytes::from([1u8; 32]);

        detector.observe_submission(
            make_submission_event(
                handle,
                FixedBytes::from([2u8; 32]),
                FixedBytes::from([3u8; 32]),
                senders()[0],
            ),
            context(10),
        );

        detector.evict_stale(15);

        assert_eq!(detector.deferred_metrics.consensus_timeout, 1);
        assert!(!detector.open_handles.contains_key(&handle));
    }

    #[test]
    fn flush_resets_counters() {
        let mut detector = detector();
        detector.deferred_metrics.drift_detected = 1;
        detector.deferred_metrics.consensus_timeout = 2;
        detector.deferred_metrics.missing_submission = 3;

        detector.flush_metrics();

        assert_eq!(detector.deferred_metrics.drift_detected, 0);
        assert_eq!(detector.deferred_metrics.consensus_timeout, 0);
        assert_eq!(detector.deferred_metrics.missing_submission, 0);
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

        assert_eq!(detector.deferred_metrics.drift_detected, 1);
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
        detector.set_alerts_enabled(false);
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
        assert_eq!(detector.deferred_metrics.drift_detected, 0);

        detector.set_alerts_enabled(true);
        detector
            .refresh_pending_consensus_checks(&pool)
            .await
            .unwrap();

        let state = detector
            .open_handles
            .get(&FixedBytes::from(handle))
            .unwrap();
        assert!(state.local_consensus_checked);
        assert_eq!(detector.deferred_metrics.drift_detected, 1);
    }
}
