//! Drift coordination is database-driven and independent of broker delivery.
//! Each restart replays the earliest required block through the new live boundary.
use anyhow::{Context, Result};
use fhevm_engine_common::drift_revert::{
    self, DriftRevertSignal, SignalStatus, POLL_INTERVAL, POLL_QUERY_TIMEOUT,
};
use tokio_util::sync::CancellationToken;

use super::catchup::ManualCatchupArgs;
use crate::database::tfhe_event_propagate::Database;

#[derive(Default, Debug)]
pub(super) struct RecoveryPlan {
    manual_range: Option<(u64, u64)>,
    drift_start: Option<u64>,
}

impl RecoveryPlan {
    pub fn observe(&mut self, signal: &DriftRevertSignal) -> Result<()> {
        let start = u64::try_from(signal.offending_host_block_number)
            .context("negative drift recovery start")?;
        self.drift_start =
            Some(self.drift_start.map_or(start, |old| old.min(start)));
        Ok(())
    }

    pub fn resolve(
        &mut self,
        args: &ManualCatchupArgs,
        live: u64,
    ) -> Result<Option<(u64, u64)>> {
        if self.manual_range.is_none() && args.enabled() {
            self.manual_range = Some(args.resolve(live)?);
        }
        match (self.manual_range, self.drift_start) {
            (manual, None) => Ok(manual),
            (manual, Some(drift)) => {
                let start = manual.map_or(drift, |(start, _)| start.min(drift));
                let end = manual.map_or(live.saturating_sub(1), |(_, end)| {
                    end.max(live.saturating_sub(1))
                });
                // A drift at the fresh live boundary is covered by live itself.
                Ok((start <= end).then_some((start, end)))
            }
        }
    }
}

pub(super) fn same_checkpoint(
    current: Option<&DriftRevertSignal>,
    accepted: Option<&DriftRevertSignal>,
) -> bool {
    match (current, accepted) {
        (None, None) => true,
        (Some(current), Some(accepted)) => {
            current.id == accepted.id
                && current.offending_host_block_number
                    == accepted.offending_host_block_number
                && current.status == SignalStatus::Done
        }
        _ => false,
    }
}

pub(super) async fn read_signal(
    db: &Database,
) -> Result<Option<DriftRevertSignal>> {
    tokio::time::timeout(POLL_QUERY_TIMEOUT, async {
        let pool = db.pool().await;
        drift_revert::latest_signal_for_chain(
            &pool,
            db.chain_id.as_u64() as i64,
        )
        .await
    })
    .await
    .context("drift signal query timed out")?
}

pub(super) enum CleanupOutcome {
    Stopped,
    Ready(Option<DriftRevertSignal>),
}

/// No block handlers run here. Failed cleanup keeps the consumer stopped until
/// an operator re-drives or acknowledges the signal, matching the legacy gate.
pub(super) async fn wait_for_cleanup(
    db: &Database,
    plan: &mut RecoveryPlan,
    cancel: &CancellationToken,
) -> Result<CleanupOutcome> {
    loop {
        let signal = tokio::select! {
            _ = cancel.cancelled() => return Ok(CleanupOutcome::Stopped),
            result = read_signal(db) => result?,
        };
        match signal {
            None => return Ok(CleanupOutcome::Ready(None)),
            Some(signal) => {
                plan.observe(&signal)?;
                if signal.status == SignalStatus::Done {
                    return Ok(CleanupOutcome::Ready(Some(signal)));
                }
                tracing::info!(drift_id = signal.id, start = signal.offending_host_block_number, status = ?signal.status, "Waiting for drift cleanup before restarting consumers");
            }
        }
        tokio::select! {
            _ = cancel.cancelled() => return Ok(CleanupOutcome::Stopped),
            _ = tokio::time::sleep(POLL_INTERVAL) => {},
        }
    }
}

/// Poll even on a quiet chain or while broker startup/publication is blocked.
pub(super) async fn wait_for_drift(
    db: &Database,
    accepted: Option<&DriftRevertSignal>,
) -> Result<DriftRevertSignal> {
    loop {
        let signal = read_signal(db).await?;
        if !same_checkpoint(signal.as_ref(), accepted) {
            return signal
                .context("drift signal disappeared; refusing stale ingestion");
        }
        tokio::time::sleep(POLL_INTERVAL).await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn signal(id: i64, start: i64, status: SignalStatus) -> DriftRevertSignal {
        DriftRevertSignal {
            id,
            host_chain_id: 1,
            offending_host_block_number: start,
            status,
        }
    }

    #[test]
    fn recovery_without_manual_catchup_covers_through_new_live_boundary() {
        let mut plan = RecoveryPlan::default();
        let args = ManualCatchupArgs::default();
        assert_eq!(plan.resolve(&args, 1000).unwrap(), None);
        plan.observe(&signal(1, 800, SignalStatus::Pending))
            .unwrap();
        assert_eq!(plan.resolve(&args, 1100).unwrap(), Some((800, 1099)));
        plan.observe(&signal(1, 700, SignalStatus::Pending))
            .unwrap();
        plan.observe(&signal(2, 900, SignalStatus::Done)).unwrap();
        assert_eq!(plan.resolve(&args, 1200).unwrap(), Some((700, 1199)));
    }

    #[test]
    fn manual_bounds_remain_fixed_across_restarts_and_extend_for_recovery() {
        let mut plan = RecoveryPlan::default();
        let args = ManualCatchupArgs {
            catchup_from_block: Some(-100),
            catchup_up_to_block: Some(1500),
        };
        assert_eq!(plan.resolve(&args, 1000).unwrap(), Some((900, 1500)));
        plan.observe(&signal(1, 950, SignalStatus::Done)).unwrap();
        assert_eq!(plan.resolve(&args, 1100).unwrap(), Some((900, 1500)));
        plan.observe(&signal(2, 800, SignalStatus::Pending))
            .unwrap();
        assert_eq!(plan.resolve(&args, 1600).unwrap(), Some((800, 1599)));
    }

    #[test]
    fn changed_or_unfinished_drift_invalidates_subscription() {
        let done = signal(1, 100, SignalStatus::Done);
        assert!(same_checkpoint(None, None));
        assert!(same_checkpoint(Some(&done), Some(&done)));
        assert!(!same_checkpoint(Some(&done), None));
        assert!(!same_checkpoint(None, Some(&done)));
        for status in [
            SignalStatus::Pending,
            SignalStatus::Reverting,
            SignalStatus::Failed("failed".into()),
        ] {
            assert!(!same_checkpoint(
                Some(&signal(1, 100, status)),
                Some(&done)
            ));
        }
        assert!(!same_checkpoint(
            Some(&signal(1, 90, SignalStatus::Done)),
            Some(&done)
        ));
        assert!(!same_checkpoint(
            Some(&signal(2, 100, SignalStatus::Done)),
            Some(&done)
        ));
    }

    #[test]
    fn recovery_start_at_or_after_live_needs_no_historical_replay() {
        let mut plan = RecoveryPlan::default();
        plan.observe(&signal(1, 100, SignalStatus::Done)).unwrap();
        assert_eq!(
            plan.resolve(&ManualCatchupArgs::default(), 100).unwrap(),
            None
        );
        assert!(plan.observe(&signal(2, -1, SignalStatus::Done)).is_err());
    }
}
