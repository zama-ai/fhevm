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

/// Runs until `cancel` is cancelled or a durable integrity halt is observed.
pub async fn run_sequential_ingest(
    source: &YellowstoneBlockSource,
    store: &SqlProofStore,
    cancel: CancellationToken,
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

        let subscription = match source.subscribe(checkpoint).await {
            Ok(subscription) => {
                backoff = Duration::from_millis(200);
                subscription
            }
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
        };

        match drive_subscription(subscription, store, &cancel).await {
            Ok(()) => return Ok(()),
            Err(RunnerError::Source(YellowstoneSourceError::Retryable(message))) => {
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
                    return Err(RunnerError::RecoveryRequired(format!(
                        "contiguous ingest gap at slot {slot}: parent slot {parent_slot} does not extend previous applied slot {expected_parent_slot}; recovery required"
                    )));
                }
                Err(error) => return Err(RunnerError::Source(error)),
            },
        };

        match store.apply_completed_block(&block).await? {
            ApplyOutcome::Applied => {
                info!(slot = block.slot, "applied completed block");
            }
            ApplyOutcome::AlreadyApplied => {
                info!(slot = block.slot, "exact replay no-op");
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
