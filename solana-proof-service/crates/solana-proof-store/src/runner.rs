//! Cancellation-safe sequential completed-block runner.
//!
//! Pulls [`YellowstoneSubscription::next_block`] and applies each block through
//! [`SqlProofStore`]. On a contiguous parent-chain gap, bounded RPC recovery
//! fills missing blocks into the same store boundary, then ingest resumes from
//! the durable checkpoint (inclusive replay). Recovery is never the live source.

use std::time::Duration;

use solana_proof_source::{
    history_complete_justified, RecoveryError, RpcRecoveryClient, YellowstoneBlockSource,
    YellowstoneSourceError, YellowstoneSubscription,
};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

use crate::store::{ApplyOutcome, SqlProofStore, StoreError};

#[derive(thiserror::Error, Debug)]
pub enum RunnerError {
    #[error(transparent)]
    Store(#[from] StoreError),
    #[error(transparent)]
    Source(#[from] YellowstoneSourceError),
    #[error("ingest integrity halted: {0}")]
    IntegrityHalted(String),
    /// Contiguous parent link missing, or recovery could not prove/fill history.
    /// Proofs/readiness stay fail-closed while this is outstanding.
    #[error("{reason}")]
    RecoveryRequired {
        reason: String,
        /// Exclusive end of `[checkpoint+1, gap_end)` when known from a typed gap.
        gap_end_slot: Option<u64>,
    },
}

fn recovery_required(reason: impl Into<String>) -> RunnerError {
    RunnerError::RecoveryRequired {
        reason: reason.into(),
        gap_end_slot: None,
    }
}

fn recovery_required_until(reason: impl Into<String>, gap_end_slot: u64) -> RunnerError {
    RunnerError::RecoveryRequired {
        reason: reason.into(),
        gap_end_slot: Some(gap_end_slot),
    }
}

/// Optional hooks for HTTP readiness (disconnect / apply-or-replay progress / recovery).
#[derive(Default)]
pub struct IngestHooks<'a> {
    /// Fired after a live Yellowstone block is Applied or AlreadyApplied.
    /// That is the continuity proof: subscription + durable cursor are usable.
    pub on_progress: Option<&'a (dyn Fn(u64) + Send + Sync)>,
    /// Durable progress from RPC recovery only. Must not prove Yellowstone continuity.
    pub on_recovered_progress: Option<&'a (dyn Fn(u64) + Send + Sync)>,
    pub on_disconnected: Option<&'a (dyn Fn() + Send + Sync)>,
    /// Fired with `true` when bounded RPC recovery starts and `false` when it
    /// ends, so readiness can return `recovery_required` while a fill is in flight.
    pub on_recovery: Option<&'a (dyn Fn(bool) + Send + Sync)>,
}

/// Runs until `cancel` is cancelled or a durable integrity halt is observed.
///
/// Progress hooks fire only after Applied / AlreadyApplied so readiness never
/// treats a bare gRPC subscribe as a live, continuity-checked source.
///
/// When `recovery` is `Some`, [`RunnerError::RecoveryRequired`] and Yellowstone
/// ancestry / replay-unavailable signals invoke a bounded RPC fill, then the
/// loop resubscribes from the durable checkpoint. When `recovery` is `None`,
/// gaps surface as [`RunnerError::RecoveryRequired`] immediately (fail closed).
pub async fn run_sequential_ingest(
    source: &YellowstoneBlockSource,
    store: &SqlProofStore,
    recovery: Option<&RpcRecoveryClient>,
    cancel: CancellationToken,
    hooks: IngestHooks<'_>,
) -> Result<(), RunnerError> {
    let mut backoff = Duration::from_millis(200);
    const MAX_BACKOFF: Duration = Duration::from_secs(5);

    loop {
        if cancel.is_cancelled() {
            return Ok(());
        }

        let checkpoint = store.checkpoint().await?;
        let status = store.integrity_status().await?;
        if status.integrity_halted {
            return Err(RunnerError::IntegrityHalted(
                status
                    .integrity_halt_reason
                    .unwrap_or_else(|| "integrity halted".to_owned()),
            ));
        }

        // Bootstrap A: empty store + configured start → recover from bootstrap
        // before the first Yellowstone subscribe so history_start can match.
        if checkpoint.is_none() {
            if let Some(client) = recovery {
                if let Some(bootstrap_slot) = client.config().bootstrap_slot {
                    match with_recovery_gate(hooks.on_recovery, async {
                        recover_bootstrap_from_start(
                            client,
                            store,
                            bootstrap_slot,
                            &cancel,
                            hooks.on_recovered_progress,
                        )
                        .await
                    })
                    .await
                    {
                        Ok(Recovered::Filled { confirmed_tip }) => {
                            maybe_mark_history_complete(client, store, confirmed_tip).await?;
                            continue;
                        }
                        Ok(Recovered::Cancelled) => return Ok(()),
                        Err(error) => return Err(error),
                    }
                }
            }
        }

        // Do not reset backoff on subscribe alone: a flaky stream that
        // connects then dies immediately would otherwise spin at 200ms forever.
        // Select on cancel so connect/subscribe cannot outlive shutdown.
        let subscription = tokio::select! {
            _ = cancel.cancelled() => return Ok(()),
            result = source.subscribe(checkpoint) => match result {
                Ok(subscription) => subscription,
                Err(YellowstoneSourceError::Retryable(message)) => {
                    warn!(%message, ?backoff, "yellowstone subscribe failed; backing off");
                    tokio::select! {
                        _ = cancel.cancelled() => return Ok(()),
                        _ = tokio::time::sleep(backoff) => {}
                    }
                    backoff = (backoff * 2).min(MAX_BACKOFF);
                    continue;
                }
                Err(YellowstoneSourceError::ReplayUnavailable(message)) => {
                    warn!(%message, "yellowstone inclusive replay unavailable; attempting RPC recovery");
                    match with_recovery_gate(hooks.on_recovery, async {
                        attempt_recovery(
                            recovery,
                            store,
                            GapHint::FromCheckpointForward,
                            &cancel,
                            hooks.on_recovered_progress,
                        )
                        .await
                    })
                    .await?
                    {
                        Recovered::Filled { confirmed_tip } => {
                            if let Some(client) = recovery {
                                maybe_mark_history_complete(client, store, confirmed_tip).await?;
                            }
                            continue;
                        }
                        Recovered::Cancelled => return Ok(()),
                    }
                }
                Err(error) => return Err(RunnerError::Source(error)),
            },
        };

        match drive_subscription(subscription, store, &cancel, &mut backoff, &hooks).await {
            Ok(()) => return Ok(()),
            Err(RunnerError::Source(YellowstoneSourceError::Retryable(message))) => {
                if let Some(on_disconnected) = hooks.on_disconnected {
                    on_disconnected();
                }
                warn!(%message, ?backoff, "yellowstone stream failed; reconnecting");
                tokio::select! {
                    _ = cancel.cancelled() => return Ok(()),
                    _ = tokio::time::sleep(backoff) => {}
                }
                backoff = (backoff * 2).min(MAX_BACKOFF);
            }
            Err(RunnerError::RecoveryRequired {
                reason,
                gap_end_slot,
            }) => {
                warn!(%reason, ?gap_end_slot, "ingest gap requires bounded RPC recovery");
                match with_recovery_gate(hooks.on_recovery, async {
                    attempt_recovery(
                        recovery,
                        store,
                        GapHint::UntilExclusive(gap_end_slot),
                        &cancel,
                        hooks.on_recovered_progress,
                    )
                    .await
                })
                .await?
                {
                    Recovered::Filled { confirmed_tip } => {
                        if let Some(client) = recovery {
                            maybe_mark_history_complete(client, store, confirmed_tip).await?;
                        }
                        // Resubscribe from durable checkpoint (inclusive replay).
                        continue;
                    }
                    Recovered::Cancelled => return Ok(()),
                }
            }
            Err(error) => return Err(error),
        }
    }
}

enum GapHint {
    /// Recover `(checkpoint, tip]` is not available without a tip RPC; recover
    /// nothing beyond signaling unavailability when no checkpoint exists.
    FromCheckpointForward,
    /// Fill `[checkpoint+1, gap_end_slot)` when `gap_end_slot` is known.
    UntilExclusive(Option<u64>),
}

#[derive(Debug, PartialEq, Eq)]
enum Recovered {
    /// Non-empty apply succeeded. `confirmed_tip` is the tip used to justify
    /// (or refuse) `history_complete`.
    Filled {
        confirmed_tip: u64,
    },
    Cancelled,
}

struct RecoveryGate<'a> {
    on_recovery: Option<&'a (dyn Fn(bool) + Send + Sync)>,
}

impl<'a> RecoveryGate<'a> {
    fn enter(on_recovery: Option<&'a (dyn Fn(bool) + Send + Sync)>) -> Self {
        if let Some(cb) = on_recovery {
            cb(true);
        }
        Self { on_recovery }
    }
}

impl Drop for RecoveryGate<'_> {
    fn drop(&mut self) {
        if let Some(cb) = self.on_recovery {
            cb(false);
        }
    }
}

async fn with_recovery_gate<T>(
    on_recovery: Option<&(dyn Fn(bool) + Send + Sync)>,
    fut: impl std::future::Future<Output = T>,
) -> T {
    let _gate = RecoveryGate::enter(on_recovery);
    fut.await
}

async fn attempt_recovery(
    recovery: Option<&RpcRecoveryClient>,
    store: &SqlProofStore,
    hint: GapHint,
    cancel: &CancellationToken,
    on_recovered_progress: Option<&(dyn Fn(u64) + Send + Sync)>,
) -> Result<Recovered, RunnerError> {
    let Some(client) = recovery else {
        return Err(recovery_required(
            "bounded RPC recovery is not configured".to_owned(),
        ));
    };
    if cancel.is_cancelled() {
        return Ok(Recovered::Cancelled);
    }

    let checkpoint = store.checkpoint().await?.ok_or_else(|| {
        recovery_required("recovery requested without a durable checkpoint".to_owned())
    })?;

    let (to_slot, confirmed_tip) = match hint {
        GapHint::UntilExclusive(Some(gap_end)) if gap_end > checkpoint.slot + 1 => {
            let confirmed_tip = read_confirmed_tip(client, cancel).await?;
            (gap_end - 1, confirmed_tip)
        }
        GapHint::UntilExclusive(Some(gap_end)) if gap_end <= checkpoint.slot + 1 => {
            return Err(recovery_required(format!(
                "recovery gap end slot {gap_end} does not extend past checkpoint {}",
                checkpoint.slot
            )));
        }
        GapHint::FromCheckpointForward => {
            let tip = read_confirmed_tip(client, cancel).await?;
            if tip <= checkpoint.slot {
                return Err(recovery_required(format!(
                    "confirmed tip {tip} is not ahead of checkpoint {}; cannot catch up after replay-unavailable",
                    checkpoint.slot
                )));
            }
            let span = tip - checkpoint.slot;
            if span > client.config().bounds.max_slots {
                return Err(recovery_required(format!(
                    "recovery bound exhausted: tip catch-up span {span} exceeds max_slots {}",
                    client.config().bounds.max_slots
                )));
            }
            (tip, tip)
        }
        GapHint::UntilExclusive(None) => {
            return Err(recovery_required(
                "RPC recovery requires a bounded gap end slot".to_owned(),
            ));
        }
        GapHint::UntilExclusive(_) => unreachable!(),
    };

    let from_slot = checkpoint.slot + 1;
    info!(
        from_slot,
        to_slot, "fetching bounded RPC recovery blocks at confirmed commitment"
    );

    let blocks = match client
        .fetch_completed_blocks(from_slot, to_slot, cancel)
        .await
    {
        Ok(blocks) => blocks,
        Err(RecoveryError::Cancelled) => return Ok(Recovered::Cancelled),
        Err(RecoveryError::BoundExhausted(message)) => {
            return Err(recovery_required(format!(
                "recovery bound exhausted: {message}"
            )));
        }
        Err(RecoveryError::HistoryUnavailable(message)) => {
            return Err(recovery_required(format!(
                "RPC history unavailable: {message}"
            )));
        }
        Err(RecoveryError::Transport(message)) => {
            return Err(recovery_required(format!(
                "RPC recovery transport failure: {message}"
            )));
        }
        Err(RecoveryError::Invalid(message)) => {
            return Err(recovery_required(format!(
                "invalid RPC recovery data: {message}"
            )));
        }
    };

    apply_recovered_blocks(store, &blocks, cancel, on_recovered_progress, confirmed_tip).await
}

async fn read_confirmed_tip(
    client: &RpcRecoveryClient,
    cancel: &CancellationToken,
) -> Result<u64, RunnerError> {
    match client.get_confirmed_slot(cancel).await {
        Ok(tip) => Ok(tip),
        Err(RecoveryError::Transport(message)) => Err(recovery_required(format!(
            "RPC recovery transport failure while reading tip: {message}"
        ))),
        Err(RecoveryError::Cancelled) => Err(recovery_required(
            "RPC recovery cancelled while reading tip".to_owned(),
        )),
        Err(err) => Err(recovery_required(format!(
            "failed to read confirmed tip for recovery: {err}"
        ))),
    }
}

async fn recover_bootstrap_from_start(
    client: &RpcRecoveryClient,
    store: &SqlProofStore,
    bootstrap_slot: u64,
    cancel: &CancellationToken,
    on_recovered_progress: Option<&(dyn Fn(u64) + Send + Sync)>,
) -> Result<Recovered, RunnerError> {
    if cancel.is_cancelled() {
        return Ok(Recovered::Cancelled);
    }
    let confirmed_tip = read_confirmed_tip(client, cancel).await?;
    if confirmed_tip < bootstrap_slot {
        return Err(recovery_required(format!(
            "confirmed tip {confirmed_tip} is behind bootstrap slot {bootstrap_slot}"
        )));
    }
    // Recover the bounded inclusive range through the observed confirmed tip.
    // Bound exhaustion stays fail-closed (history_incomplete / recovery_required).
    let blocks = match client
        .fetch_completed_blocks(bootstrap_slot, confirmed_tip, cancel)
        .await
    {
        Ok(blocks) => blocks,
        Err(RecoveryError::Cancelled) => return Ok(Recovered::Cancelled),
        Err(RecoveryError::BoundExhausted(message)) => {
            return Err(recovery_required(format!(
                "bootstrap recovery bound exhausted from slot {bootstrap_slot} through tip {confirmed_tip}: {message}"
            )));
        }
        Err(RecoveryError::HistoryUnavailable(message)) => {
            return Err(recovery_required(format!(
                "bootstrap RPC history unavailable from slot {bootstrap_slot}: {message}"
            )));
        }
        Err(RecoveryError::Transport(message)) => {
            return Err(recovery_required(format!(
                "bootstrap RPC transport failure from slot {bootstrap_slot}: {message}"
            )));
        }
        Err(RecoveryError::Invalid(message)) => {
            return Err(recovery_required(format!(
                "bootstrap recovery invalid data from slot {bootstrap_slot}: {message}"
            )));
        }
    };
    if blocks.is_empty() {
        return Err(recovery_required(format!(
            "bootstrap range [{bootstrap_slot}, {confirmed_tip}] has no confirmed blocks"
        )));
    }
    if blocks[0].slot != bootstrap_slot {
        return Err(recovery_required(format!(
            "bootstrap recovery expected first slot {bootstrap_slot}, got {}",
            blocks[0].slot
        )));
    }
    apply_recovered_blocks(store, &blocks, cancel, on_recovered_progress, confirmed_tip).await
}

/// Empty successful fetches must fail closed — never `Filled`, never mark complete.
fn reject_empty_recovery(
    blocks: &[solana_proof_source::CompletedBlock],
) -> Result<(), RunnerError> {
    if blocks.is_empty() {
        Err(recovery_required(
            "RPC recovery returned no blocks for the requested range".to_owned(),
        ))
    } else {
        Ok(())
    }
}

async fn apply_recovered_blocks(
    store: &SqlProofStore,
    blocks: &[solana_proof_source::CompletedBlock],
    cancel: &CancellationToken,
    on_recovered_progress: Option<&(dyn Fn(u64) + Send + Sync)>,
    confirmed_tip: u64,
) -> Result<Recovered, RunnerError> {
    reject_empty_recovery(blocks)?;
    for block in blocks {
        if cancel.is_cancelled() {
            return Ok(Recovered::Cancelled);
        }
        match store.apply_completed_block(block).await? {
            ApplyOutcome::Applied | ApplyOutcome::AlreadyApplied => {
                info!(slot = block.slot, "applied recovered completed block");
                if let Some(on_recovered_progress) = on_recovered_progress {
                    on_recovered_progress(block.slot);
                }
            }
            ApplyOutcome::RecoveryRequired {
                reason,
                gap_end_slot,
            } => {
                return Err(recovery_required_until(
                    format!("recovered block still requires recovery: {reason}"),
                    gap_end_slot,
                ));
            }
            ApplyOutcome::IntegrityHalted { reason } => {
                return Err(RunnerError::IntegrityHalted(format!(
                    "conflicting recovered ancestry: {reason}"
                )));
            }
        }
    }
    Ok(Recovered::Filled { confirmed_tip })
}

async fn maybe_mark_history_complete(
    client: &RpcRecoveryClient,
    store: &SqlProofStore,
    confirmed_tip: u64,
) -> Result<(), RunnerError> {
    let status = store.integrity_status().await?;
    if status.history_complete || status.integrity_halted {
        return Ok(());
    }
    if history_complete_justified(
        client.config().bootstrap_slot,
        status.history_start.as_ref(),
        status.checkpoint.as_ref(),
        confirmed_tip,
    ) {
        store.set_history_complete_after_recovery(true).await?;
        info!(
            bootstrap_slot = ?client.config().bootstrap_slot,
            confirmed_tip,
            "history_complete set after recovery proved continuity from configured start through confirmed tip"
        );
    }
    Ok(())
}

async fn drive_subscription(
    mut subscription: YellowstoneSubscription,
    store: &SqlProofStore,
    cancel: &CancellationToken,
    backoff: &mut Duration,
    hooks: &IngestHooks<'_>,
) -> Result<(), RunnerError> {
    const INITIAL_BACKOFF: Duration = Duration::from_millis(200);
    loop {
        let block = tokio::select! {
            _ = cancel.cancelled() => {
                info!("ingest cancelled");
                return Ok(());
            }
            block = subscription.next_block() => match block {
                Ok(block) => block,
                // Program-filtered streams can skip empty slots; ancestry gaps
                // need bounded RPC recovery, not a silent skip or retry loop.
                Err(YellowstoneSourceError::Ancestry {
                    slot,
                    parent_slot,
                    expected_parent_slot,
                    ..
                }) => {
                    return Err(recovery_required_until(
                        format!(
                            "contiguous ingest gap at slot {slot}: parent slot {parent_slot} does not extend previous applied slot {expected_parent_slot}; recovery required"
                        ),
                        slot,
                    ));
                }
                Err(error) => return Err(RunnerError::Source(error)),
            },
        };

        match store.apply_completed_block(&block).await? {
            ApplyOutcome::Applied => {
                // Meaningful stream progress — safe to collapse reconnect delay.
                *backoff = INITIAL_BACKOFF;
                info!(slot = block.slot, "applied completed block");
                if let Some(on_progress) = hooks.on_progress {
                    on_progress(block.slot);
                }
            }
            ApplyOutcome::AlreadyApplied => {
                *backoff = INITIAL_BACKOFF;
                info!(slot = block.slot, "exact replay no-op");
                if let Some(on_progress) = hooks.on_progress {
                    on_progress(block.slot);
                }
            }
            ApplyOutcome::RecoveryRequired {
                reason,
                gap_end_slot,
            } => {
                return Err(recovery_required_until(reason, gap_end_slot));
            }
            ApplyOutcome::IntegrityHalted { reason } => {
                return Err(RunnerError::IntegrityHalted(reason));
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_proof_source::{history_complete_justified, BlockCheckpoint, CompletedBlock};

    fn cp(slot: u64) -> BlockCheckpoint {
        BlockCheckpoint {
            slot,
            block_hash: [slot as u8; 32],
        }
    }

    #[test]
    fn empty_recovery_is_not_filled() {
        let err = reject_empty_recovery(&[]).unwrap_err();
        assert!(matches!(
            err,
            RunnerError::RecoveryRequired { reason, gap_end_slot: None }
                if reason.contains("no blocks")
        ));
    }

    #[test]
    fn nonempty_recovery_passes_empty_guard() {
        let block = CompletedBlock {
            slot: 1,
            block_hash: [1; 32],
            parent_slot: 0,
            parent_hash: [0; 32],
            block_time: None,
            block_height: None,
            executed_transaction_count: 0,
            transactions: vec![],
        };
        assert!(reject_empty_recovery(&[block]).is_ok());
    }

    #[test]
    fn bootstrap_single_slot_with_tip_ahead_does_not_justify_complete() {
        let start = cp(10);
        assert!(!history_complete_justified(
            Some(10),
            Some(&start),
            Some(&start),
            50
        ));
    }

    #[test]
    fn catch_up_to_confirmed_tip_justifies_complete() {
        let start = cp(10);
        let tip = cp(50);
        assert!(history_complete_justified(
            Some(10),
            Some(&start),
            Some(&tip),
            50
        ));
    }

    #[test]
    fn bound_exhaustion_never_justifies_complete_without_tip_match() {
        // After a partial fill, durable tip behind confirmed tip must not flip.
        let start = cp(10);
        let partial = cp(20);
        assert!(!history_complete_justified(
            Some(10),
            Some(&start),
            Some(&partial),
            100
        ));
    }

    #[test]
    fn recovery_gate_toggles_callback() {
        use std::sync::atomic::{AtomicUsize, Ordering};
        let starts = AtomicUsize::new(0);
        let ends = AtomicUsize::new(0);
        let cb = |active: bool| {
            if active {
                starts.fetch_add(1, Ordering::SeqCst);
            } else {
                ends.fetch_add(1, Ordering::SeqCst);
            }
        };
        {
            let _gate = RecoveryGate::enter(Some(&cb));
            assert_eq!(starts.load(Ordering::SeqCst), 1);
            assert_eq!(ends.load(Ordering::SeqCst), 0);
        }
        assert_eq!(ends.load(Ordering::SeqCst), 1);
    }
}
