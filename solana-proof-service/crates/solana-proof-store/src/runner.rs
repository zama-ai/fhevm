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

/// Backoff floor for the ingest loop: Yellowstone reconnects, and now RPC
/// recovery retries when the endpoint is unreachable, share this schedule.
const INGEST_INITIAL_BACKOFF: Duration = Duration::from_millis(200);
/// Backoff ceiling. An unreachable RPC endpoint is retried indefinitely at this
/// cadence, so the WARN log fires at most this often — not once per second.
const INGEST_MAX_BACKOFF: Duration = Duration::from_secs(5);

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
/// [`YellowstoneSourceError::ReplayCursorExpired`] invoke a bounded RPC fill,
/// then the loop resubscribes from the durable checkpoint. Providers that
/// reject `from_slot` entirely ([`YellowstoneSourceError::ReplayUnsupported`])
/// fail closed — RPC catch-up cannot change that capability (see
/// fhevm-internal #1823 for cursorless staging). When `recovery` is `None`,
/// gaps surface as [`RunnerError::RecoveryRequired`] immediately.
pub async fn run_sequential_ingest(
    source: &YellowstoneBlockSource,
    store: &SqlProofStore,
    recovery: Option<&RpcRecoveryClient>,
    cancel: CancellationToken,
    hooks: IngestHooks<'_>,
) -> Result<(), RunnerError> {
    let mut backoff = INGEST_INITIAL_BACKOFF;

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
                            // Successful fill is meaningful progress: collapse the
                            // backoff so a recovery-only cadence does not stay at
                            // the ceiling between fills.
                            backoff = INGEST_INITIAL_BACKOFF;
                            continue;
                        }
                        Ok(Recovered::Cancelled) => return Ok(()),
                        Ok(Recovered::Unreachable(message)) => {
                            if !back_off_after_unreachable(&message, &cancel, &mut backoff).await {
                                return Ok(());
                            }
                            continue;
                        }
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
                    backoff = (backoff * 2).min(INGEST_MAX_BACKOFF);
                    continue;
                }
                Err(YellowstoneSourceError::ReplayUnsupported(message)) => {
                    return Err(RunnerError::Source(
                        YellowstoneSourceError::ReplayUnsupported(message),
                    ));
                }
                Err(YellowstoneSourceError::ReplayCursorExpired(message)) => {
                    warn!(
                        %message,
                        "Yellowstone inclusive replay cursor expired; attempting RPC catch-up"
                    );
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
                            backoff = INGEST_INITIAL_BACKOFF;
                            continue;
                        }
                        Recovered::Cancelled => return Ok(()),
                        Recovered::Unreachable(message) => {
                            if !back_off_after_unreachable(&message, &cancel, &mut backoff).await {
                                return Ok(());
                            }
                            continue;
                        }
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
                backoff = (backoff * 2).min(INGEST_MAX_BACKOFF);
            }
            Err(RunnerError::Source(YellowstoneSourceError::ReplayUnsupported(message))) => {
                return Err(RunnerError::Source(
                    YellowstoneSourceError::ReplayUnsupported(message),
                ));
            }
            Err(RunnerError::Source(YellowstoneSourceError::ReplayCursorExpired(message))) => {
                warn!(
                    %message,
                    "Yellowstone stream reported expired replay cursor; attempting RPC catch-up"
                );
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
                        backoff = INGEST_INITIAL_BACKOFF;
                        continue;
                    }
                    Recovered::Cancelled => return Ok(()),
                    Recovered::Unreachable(message) => {
                        if !back_off_after_unreachable(&message, &cancel, &mut backoff).await {
                            return Ok(());
                        }
                        continue;
                    }
                }
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
                        backoff = INGEST_INITIAL_BACKOFF;
                        // Resubscribe from durable checkpoint (inclusive replay).
                        continue;
                    }
                    Recovered::Cancelled => return Ok(()),
                    Recovered::Unreachable(message) => {
                        if !back_off_after_unreachable(&message, &cancel, &mut backoff).await {
                            return Ok(());
                        }
                        continue;
                    }
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
    /// The RPC recovery endpoint was unreachable (transport-level). The endpoint
    /// may not exist yet (e2e bring-up) or be flapping; the caller backs off and
    /// retries the loop instead of failing the writer closed.
    Unreachable(String),
}

/// Confirmed-tip read outcome, separating a retryable unreachable endpoint from
/// a terminal read failure so a connection-refused tip read never crashes the
/// writer at startup.
enum TipRead {
    Tip(u64),
    Unreachable(String),
    Cancelled,
}

/// Back off after an unreachable RPC recovery endpoint, growing the delay toward
/// [`INGEST_MAX_BACKOFF`]. Returns `false` when cancelled during the wait so the
/// caller can stop; `true` to retry the ingest loop. Logs at WARN once per
/// backoff interval (not once per failed request).
async fn back_off_after_unreachable(
    message: &str,
    cancel: &CancellationToken,
    backoff: &mut Duration,
) -> bool {
    warn!(
        %message,
        ?backoff,
        "RPC recovery endpoint unreachable; staying unready and retrying with backoff"
    );
    tokio::select! {
        _ = cancel.cancelled() => false,
        _ = tokio::time::sleep(*backoff) => {
            *backoff = (*backoff * 2).min(INGEST_MAX_BACKOFF);
            true
        }
    }
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
            let confirmed_tip = match read_confirmed_tip(client, cancel).await? {
                TipRead::Tip(tip) => tip,
                TipRead::Unreachable(message) => return Ok(Recovered::Unreachable(message)),
                TipRead::Cancelled => return Ok(Recovered::Cancelled),
            };
            (gap_end - 1, confirmed_tip)
        }
        GapHint::UntilExclusive(Some(gap_end)) if gap_end <= checkpoint.slot + 1 => {
            return Err(recovery_required(format!(
                "recovery gap end slot {gap_end} does not extend past checkpoint {}",
                checkpoint.slot
            )));
        }
        GapHint::FromCheckpointForward => {
            let tip = match read_confirmed_tip(client, cancel).await? {
                TipRead::Tip(tip) => tip,
                TipRead::Unreachable(message) => return Ok(Recovered::Unreachable(message)),
                TipRead::Cancelled => return Ok(Recovered::Cancelled),
            };
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

    recover_range_incremental(RecoverRange {
        client,
        store,
        from_slot,
        to_slot,
        cancel,
        on_recovered_progress,
        confirmed_tip,
        expect_first_slot: None,
    })
    .await
}

/// Read the confirmed tip, mapping a transport-unreachable endpoint to a
/// retryable [`TipRead::Unreachable`] and cancellation to [`TipRead::Cancelled`].
/// Only integrity/logical read failures (RPC error, invalid, bound, history
/// unavailable) stay terminal — the boundary the fix pins.
async fn read_confirmed_tip(
    client: &RpcRecoveryClient,
    cancel: &CancellationToken,
) -> Result<TipRead, RunnerError> {
    match client.get_confirmed_slot(cancel).await {
        Ok(tip) => Ok(TipRead::Tip(tip)),
        Err(RecoveryError::Transport(message)) => Ok(TipRead::Unreachable(format!(
            "RPC recovery endpoint unreachable while reading tip: {message}"
        ))),
        Err(RecoveryError::Cancelled) => Ok(TipRead::Cancelled),
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
    let confirmed_tip = match read_confirmed_tip(client, cancel).await? {
        TipRead::Tip(tip) => tip,
        TipRead::Unreachable(message) => return Ok(Recovered::Unreachable(message)),
        TipRead::Cancelled => return Ok(Recovered::Cancelled),
    };
    if confirmed_tip < bootstrap_slot {
        return Err(recovery_required(format!(
            "confirmed tip {confirmed_tip} is behind bootstrap slot {bootstrap_slot}"
        )));
    }
    // Recover the bounded inclusive range through the observed confirmed tip.
    // Bound exhaustion stays fail-closed (history_incomplete / recovery_required).
    recover_range_incremental(RecoverRange {
        client,
        store,
        from_slot: bootstrap_slot,
        to_slot: confirmed_tip,
        cancel,
        on_recovered_progress,
        confirmed_tip,
        expect_first_slot: Some(bootstrap_slot),
    })
    .await
}

/// Split a recovery list/fetch error along the transport-vs-integrity boundary:
/// an unreachable endpoint ([`RecoveryError::Transport`]) is a retryable
/// [`Recovered::Unreachable`], while every integrity/logical class (RPC error,
/// history unavailable, bound exhausted, invalid data) stays a terminal
/// [`RunnerError::RecoveryRequired`].
fn map_recovery_list_error(err: RecoveryError, context: &str) -> Result<Recovered, RunnerError> {
    match err {
        RecoveryError::Cancelled => Ok(Recovered::Cancelled),
        RecoveryError::Transport(message) => Ok(Recovered::Unreachable(format!(
            "{context} RPC endpoint unreachable: {message}"
        ))),
        RecoveryError::RpcError(message) => Err(recovery_required(format!(
            "{context} RPC returned an error: {message}"
        ))),
        RecoveryError::BoundExhausted(message) => Err(recovery_required(format!(
            "{context} bound exhausted: {message}"
        ))),
        RecoveryError::HistoryUnavailable(message) => Err(recovery_required(format!(
            "{context} RPC history unavailable: {message}"
        ))),
        RecoveryError::Invalid(message) => Err(recovery_required(format!(
            "{context} invalid RPC data: {message}"
        ))),
    }
}

struct RecoverRange<'a> {
    client: &'a RpcRecoveryClient,
    store: &'a SqlProofStore,
    from_slot: u64,
    to_slot: u64,
    cancel: &'a CancellationToken,
    on_recovered_progress: Option<&'a (dyn Fn(u64) + Send + Sync)>,
    confirmed_tip: u64,
    expect_first_slot: Option<u64>,
}

/// List slots, then fetch+apply each block before the next `getBlock` so the
/// durable checkpoint advances during catch-up (not only after a full Vec).
async fn recover_range_incremental(range: RecoverRange<'_>) -> Result<Recovered, RunnerError> {
    let RecoverRange {
        client,
        store,
        from_slot,
        to_slot,
        cancel,
        on_recovered_progress,
        confirmed_tip,
        expect_first_slot,
    } = range;
    let slots = match client.list_existing_slots(from_slot, to_slot, cancel).await {
        Ok(slots) => slots,
        Err(err) => return map_recovery_list_error(err, "recovery"),
    };
    if slots.is_empty() {
        return Err(recovery_required(format!(
            "RPC recovery returned no blocks for range [{from_slot}, {to_slot}]"
        )));
    }
    if let Some(expected) = expect_first_slot {
        if slots[0] != expected {
            return Err(recovery_required(format!(
                "bootstrap recovery expected first slot {expected}, got {}",
                slots[0]
            )));
        }
    }

    for slot in slots {
        if cancel.is_cancelled() {
            return Ok(Recovered::Cancelled);
        }
        let block = match client.fetch_completed_block(slot, cancel).await {
            Ok(block) => block,
            Err(RecoveryError::Cancelled) => return Ok(Recovered::Cancelled),
            Err(err) => return map_recovery_list_error(err, "recovery"),
        };
        match apply_recovered_blocks(
            store,
            std::slice::from_ref(&block),
            cancel,
            on_recovered_progress,
            confirmed_tip,
        )
        .await?
        {
            Recovered::Cancelled => return Ok(Recovered::Cancelled),
            // Applying to the store touches no RPC, so it never reports the
            // endpoint unreachable; propagate defensively rather than panic.
            unreachable @ Recovered::Unreachable(_) => return Ok(unreachable),
            Recovered::Filled { .. } => {}
        }
    }
    Ok(Recovered::Filled { confirmed_tip })
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
                // Stream-level replay rejection must take the same path as a
                // subscribe-time rejection (classify_status on gRPC status).
                Err(error @ YellowstoneSourceError::ReplayUnsupported(_))
                | Err(error @ YellowstoneSourceError::ReplayCursorExpired(_)) => {
                    return Err(RunnerError::Source(error));
                }
                Err(error) => return Err(RunnerError::Source(error)),
            },
        };

        match store.apply_completed_block(&block).await? {
            ApplyOutcome::Applied => {
                // Meaningful stream progress — safe to collapse reconnect delay.
                *backoff = INGEST_INITIAL_BACKOFF;
                info!(slot = block.slot, "applied completed block");
                if let Some(on_progress) = hooks.on_progress {
                    on_progress(block.slot);
                }
            }
            ApplyOutcome::AlreadyApplied => {
                *backoff = INGEST_INITIAL_BACKOFF;
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
    use solana_proof_source::{
        history_complete_justified, BlockCheckpoint, CompletedBlock, RecoveryBounds,
        RpcRecoveryConfig,
    };

    fn cp(slot: u64) -> BlockCheckpoint {
        BlockCheckpoint {
            slot,
            block_hash: [slot as u8; 32],
        }
    }

    /// System program id: 32 base58 `1`s decode to 32 zero bytes, so this needs
    /// no bs58 dev-dependency to build a valid `RpcRecoveryClient`.
    const VALID_PROGRAM_ID: &str = "11111111111111111111111111111111";

    fn unreachable_recovery_client() -> RpcRecoveryClient {
        // Port 1 is not listening: `send()` fails with connection-refused, the
        // exact e2e bring-up condition before the validator exists.
        RpcRecoveryClient::new(RpcRecoveryConfig {
            rpc_url: "http://127.0.0.1:1".to_owned(),
            program_id: VALID_PROGRAM_ID.to_owned(),
            bounds: RecoveryBounds::default(),
            bootstrap_slot: None,
        })
        .expect("valid recovery client")
    }

    #[tokio::test]
    async fn tip_read_when_endpoint_unreachable_is_retryable_not_terminal() {
        // The startup failure mode: reading the confirmed tip against an RPC
        // that does not exist yet must yield a retryable Unreachable, never a
        // terminal RunnerError that would exit the writer and kill the process.
        let client = unreachable_recovery_client();
        let cancel = CancellationToken::new();
        let outcome = read_confirmed_tip(&client, &cancel)
            .await
            .expect("transport failure must not be a terminal RunnerError");
        assert!(
            matches!(outcome, TipRead::Unreachable(_)),
            "expected TipRead::Unreachable, got a different outcome"
        );
    }

    #[tokio::test]
    async fn tip_read_respects_cancellation() {
        let client = unreachable_recovery_client();
        let cancel = CancellationToken::new();
        cancel.cancel();
        let outcome = read_confirmed_tip(&client, &cancel).await.unwrap();
        assert!(matches!(outcome, TipRead::Cancelled));
    }

    #[test]
    fn recovery_error_boundary_transport_retries_integrity_terminates() {
        // Transport-class unreachability is retryable (stays alive)...
        assert!(matches!(
            map_recovery_list_error(RecoveryError::Transport("refused".into()), "recovery"),
            Ok(Recovered::Unreachable(_))
        ));
        // ...cancellation stops cleanly...
        assert!(matches!(
            map_recovery_list_error(RecoveryError::Cancelled, "recovery"),
            Ok(Recovered::Cancelled)
        ));
        // ...and every integrity/logical class stays terminal (fail-closed).
        for terminal in [
            RecoveryError::RpcError("method not found".into()),
            RecoveryError::HistoryUnavailable("Block cleaned up".into()),
            RecoveryError::BoundExhausted("span too wide".into()),
            RecoveryError::Invalid("malformed block".into()),
        ] {
            assert!(
                matches!(
                    map_recovery_list_error(terminal.clone(), "recovery"),
                    Err(RunnerError::RecoveryRequired { .. })
                ),
                "expected {terminal:?} to stay terminal"
            );
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
