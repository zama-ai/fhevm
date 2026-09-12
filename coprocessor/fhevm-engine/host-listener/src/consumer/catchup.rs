//! Startup-only manual replay. The observed live block is a fixed reference,
//! not necessarily the RPC head (the live queue may contain a backlog).

use anyhow::{bail, Context, Result};

#[derive(Clone, Debug, Default)]
pub struct ManualCatchupArgs {
    pub catchup_from_block: Option<i64>,

    pub catchup_up_to_block: Option<i64>,
}

impl ManualCatchupArgs {
    pub fn validate(&self) -> Result<()> {
        if self.catchup_up_to_block.is_some()
            && self.catchup_from_block.is_none()
        {
            bail!("--catchup-up-to-block requires --catchup-from-block");
        }
        if let (Some(from), Some(to)) =
            (self.catchup_from_block, self.catchup_up_to_block)
        {
            if from >= 0 && to >= 0 && from > to {
                bail!("--catchup-from-block must not exceed --catchup-up-to-block");
            }
        }
        Ok(())
    }

    pub fn enabled(&self) -> bool {
        self.catchup_from_block.is_some()
    }

    pub fn resolve(&self, live_block: u64) -> Result<(u64, u64)> {
        self.validate()?;
        let from = self
            .catchup_from_block
            .context("manual catchup is disabled")?;
        let resolve = |value: i64| {
            if value < 0 {
                live_block.saturating_sub(value.unsigned_abs())
            } else {
                value as u64
            }
        };
        let start = resolve(from);
        let end = resolve(self.catchup_up_to_block.unwrap_or(-1));
        if start > end {
            bail!("Invalid manual catchup range [{start}, {end}] at live block {live_block}");
        }
        Ok((start, end))
    }
}

/// Temporary guard: retain the earliest drift boundary even after cleanup.
/// A later commit adds replay restart; manual replay must not drive live recovery.
#[derive(Clone, Default)]
pub(super) struct DriftBoundary {
    earliest: std::sync::Arc<tokio::sync::Mutex<Option<u64>>>,
}

impl DriftBoundary {
    pub async fn should_skip(
        &self,
        block: u64,
        offending_block: Option<i64>,
    ) -> bool {
        let mut earliest = self.earliest.lock().await;
        if let Some(offending_block) = offending_block {
            let boundary = offending_block.max(0) as u64;
            *earliest =
                Some(earliest.map_or(boundary, |old| old.min(boundary)));
        }
        earliest.is_some_and(|boundary| block >= boundary)
    }
}

/// One reference per process startup, shared by concurrent handler invocations.
#[derive(Clone)]
pub(super) struct LiveReference {
    chain_id: u64,
    sender: std::sync::Arc<
        tokio::sync::Mutex<Option<tokio::sync::oneshot::Sender<u64>>>,
    >,
}

impl LiveReference {
    pub fn new(chain_id: u64) -> (Self, tokio::sync::oneshot::Receiver<u64>) {
        let (sender, receiver) = tokio::sync::oneshot::channel();
        (
            Self {
                chain_id,
                sender: std::sync::Arc::new(tokio::sync::Mutex::new(Some(
                    sender,
                ))),
            },
            receiver,
        )
    }

    pub async fn observe(&self, payload: &consumer::BlockPayload) {
        if payload.chain_id == self.chain_id
            && payload.flow == primitives::event::BlockFlow::Live
        {
            if let Some(sender) = self.sender.lock().await.take() {
                let _ = sender.send(payload.block_number);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn drift_boundary_skips_inclusive_tail_and_never_moves_forward() {
        let guard = DriftBoundary::default();
        assert!(!guard.should_skip(1000, None).await);
        assert!(!guard.should_skip(99, Some(100)).await);
        assert!(guard.should_skip(100, Some(100)).await);
        assert!(guard.should_skip(101, Some(100)).await);
        // Cleanup or a later signal must not reopen this replay's invalid tail.
        assert!(guard.should_skip(100, None).await);
        assert!(guard.should_skip(100, Some(200)).await);
        assert!(guard.clone().should_skip(50, Some(50)).await);
        assert!(guard.should_skip(75, None).await);
        assert!(!guard.should_skip(49, None).await);
    }

    fn args(from: i64, to: Option<i64>) -> ManualCatchupArgs {
        ManualCatchupArgs {
            catchup_from_block: Some(from),
            catchup_up_to_block: to,
        }
    }

    #[tokio::test]
    async fn reference_uses_only_the_first_live_block_on_the_right_chain() {
        use primitives::event::BlockFlow;
        let (reference, mut receiver) = LiveReference::new(1);
        let mut payload = consumer::BlockPayload {
            chain_id: 1,
            flow: BlockFlow::Catchup,
            block_number: 100,
            block_hash: Default::default(),
            parent_hash: Default::default(),
            timestamp: 0,
            transactions: vec![],
        };
        reference.observe(&payload).await;
        payload.flow = BlockFlow::Reorged;
        reference.observe(&payload).await;
        payload.flow = BlockFlow::Live;
        payload.chain_id = 2;
        reference.observe(&payload).await;
        assert!(matches!(
            receiver.try_recv(),
            Err(tokio::sync::oneshot::error::TryRecvError::Empty)
        ));
        payload.chain_id = 1;
        reference.observe(&payload).await;
        payload.block_number = 200;
        reference.clone().observe(&payload).await;
        assert_eq!(receiver.await.unwrap(), 100);
    }

    #[test]
    fn inclusive_ranges_and_relative_bounds() {
        for (from, to, expected) in [
            (10, Some(20), (10, 20)),
            (42, Some(42), (42, 42)),
            (-100, Some(-10), (900, 990)),
            (10, None, (10, 999)),
            (-100, None, (900, 999)),
            (0, Some(0), (0, 0)),
            (i64::MIN, None, (0, 999)),
        ] {
            assert_eq!(args(from, to).resolve(1000).unwrap(), expected);
        }
        assert_eq!(args(-100, None).resolve(10).unwrap(), (0, 9));
    }

    #[test]
    fn omitted_end_matches_minus_one_and_keeps_live_boundary_out_of_replay() {
        assert_eq!(args(-100, None).resolve(1000).unwrap(), (900, 999));
        assert_eq!(
            args(-100, None).resolve(1000).unwrap(),
            args(-100, Some(-1)).resolve(1000).unwrap()
        );
        assert_eq!(args(-1, None).resolve(1000).unwrap(), (999, 999));
        assert!(args(1000, None).resolve(1000).is_err());
        // Explicit ends remain inclusive, including the first live block.
        assert_eq!(args(1000, Some(1000)).resolve(1000).unwrap(), (1000, 1000));
    }

    #[test]
    fn default_end_saturates_at_genesis() {
        assert_eq!(args(0, None).resolve(0).unwrap(), (0, 0));
        assert_eq!(args(-1, None).resolve(1).unwrap(), (0, 0));
        assert_eq!(
            args(0, None).resolve(0).unwrap(),
            args(0, Some(-1)).resolve(0).unwrap()
        );
    }

    #[test]
    fn invalid_ranges_are_rejected() {
        assert!(args(20, Some(10)).validate().is_err());
        assert!(args(-10, Some(-20)).resolve(1000).is_err());
        assert!(args(1001, None).resolve(1000).is_err());
        assert!(ManualCatchupArgs::default().resolve(1000).is_err());
    }

    #[test]
    fn absolute_bounds_do_not_depend_on_a_backlogged_live_reference() {
        assert_eq!(args(2000, Some(3000)).resolve(1000).unwrap(), (2000, 3000));
        assert_eq!(args(-100, Some(3000)).resolve(1000).unwrap(), (900, 3000));
    }
}
