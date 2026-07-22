//! Cancellation-safe sequential completed-block runner.
//!
//! Pulls [`YellowstoneSubscription::next_block`] and applies each block through
//! [`SqlProofStore`]. The source never owns persistence; reconnect resumes from
//! the durable checkpoint. Bounded RPC recovery is intentionally not here.

use std::time::Duration;

use solana_proof_source::{
    YellowstoneBlockSource, YellowstoneSourceError, YellowstoneSubscription,
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
    /// Contiguous parent link missing (filtered-stream gap). Bounded RPC
    /// recovery must fill intermediate blocks; this is not a silent skip.
    #[error("contiguous ingest gap requires recovery: {0}")]
    RecoveryRequired(String),
}

/// Optional hooks for HTTP readiness (disconnect / apply-or-replay progress).
#[derive(Default)]
pub struct IngestHooks<'a> {
    /// Fired after a block is applied or recognized as an exact replay no-op.
    /// That is the continuity proof: subscription + durable cursor are usable.
    pub on_progress: Option<&'a (dyn Fn(u64) + Send + Sync)>,
    pub on_disconnected: Option<&'a (dyn Fn() + Send + Sync)>,
}

/// Runs until `cancel` is cancelled or a durable integrity halt is observed.
///
/// Progress hooks fire only after Applied / AlreadyApplied so readiness never
/// treats a bare gRPC subscribe as a live, continuity-checked source.
pub async fn run_sequential_ingest(
    source: &YellowstoneBlockSource,
    store: &SqlProofStore,
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
            Err(error) => return Err(error),
        }
    }
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
                    return Err(RunnerError::RecoveryRequired(format!(
                        "contiguous ingest gap at slot {slot}: parent slot {parent_slot} does not extend previous applied slot {expected_parent_slot}; recovery required"
                    )));
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
            ApplyOutcome::RecoveryRequired { reason } => {
                return Err(RunnerError::RecoveryRequired(reason));
            }
            ApplyOutcome::IntegrityHalted { reason } => {
                return Err(RunnerError::IntegrityHalted(reason));
            }
        }
    }
}
