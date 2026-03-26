use crate::gateway::arbitrum::transaction::tx_throttler::{GatewayTxTask, TxThrottlingSender};
use crate::readiness::throttler::{ReadinessItem, ReadinessSender};

pub async fn bounce_check(tx_throttler: TxThrottlingSender<GatewayTxTask>) -> bool {
    let tx_full = tx_throttler.is_queue_full().await;
    tx_full
}

pub struct BounceChecker<R> {
    tx_throttler: TxThrottlingSender<GatewayTxTask>,
    readiness_throttler: ReadinessSender<R>,
    retry_after_seconds: u32,
}

impl<R: ReadinessItem + Send + Sync + 'static> BounceChecker<R> {
    pub fn new(
        tx_throttler: TxThrottlingSender<GatewayTxTask>,
        readiness_throttler: ReadinessSender<R>,
        retry_after_seconds: u32,
    ) -> Self {
        Self {
            tx_throttler,
            readiness_throttler,
            retry_after_seconds,
        }
    }

    /// Returns `Ok(())` if capacity is available, `Err(retry_after_seconds)` if full.
    pub async fn check(&self) -> Result<(), u32> {
        if self.tx_throttler.is_queue_full().await || self.readiness_throttler.is_queue_full().await
        {
            Err(self.retry_after_seconds)
        } else {
            Ok(())
        }
    }

    pub fn readiness_throttler(&self) -> &ReadinessSender<R> {
        &self.readiness_throttler
    }

    pub fn tx_throttler(&self) -> &TxThrottlingSender<GatewayTxTask> {
        &self.tx_throttler
    }
}
