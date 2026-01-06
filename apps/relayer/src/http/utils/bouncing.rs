use crate::gateway::arbitrum::transaction::throttler::{GatewayTxTask, ThrottlingSender};

pub async fn bounce_check(tx_throttler: ThrottlingSender<GatewayTxTask>) -> bool {
    // NOTE: add bounce check for readiness queue here as well.
    let tx_full = tx_throttler.is_queue_full().await;
    tx_full
}
