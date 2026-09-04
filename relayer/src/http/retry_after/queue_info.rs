//! Queue information types for dynamic retry-after computation.
//!
//! This module provides types that combine queue state information from
//! different throttlers for ETA computation in HTTP handlers.

use serde::{Deserialize, Serialize};

// Re-export the base queue info types from their source modules
pub use crate::gateway::arbitrum::transaction::tx_throttler::TxQueueInfo;
pub use crate::readiness::throttler::ReadinessQueueInfo;

/// Combined queue info for decrypt operations (user-decrypt and public-decrypt).
///
/// Decrypt operations pass through two queues:
/// 1. Readiness queue (concurrency-based) - checks if ciphertexts are ready
/// 2. TX queue (TPS-based) - sends the transaction to the gateway
#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct DecryptQueueInfo {
    /// Readiness queue info (concurrency-limited)
    pub readiness: ReadinessQueueInfo,
    /// TX queue info (TPS-limited)
    pub tx: TxQueueInfo,
}

impl DecryptQueueInfo {
    /// Create a new DecryptQueueInfo from readiness and TX queue info.
    pub fn new(readiness: ReadinessQueueInfo, tx: TxQueueInfo) -> Self {
        Self { readiness, tx }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tx_queue_info_creation() {
        let info = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
            position: None,
        };
        assert_eq!(info.size, 100);
        assert_eq!(info.drain_rate_tps, 20);
    }

    #[test]
    fn test_readiness_queue_info_creation() {
        let info = ReadinessQueueInfo {
            size: 50,
            max_concurrency: 250,
            position: None,
        };
        assert_eq!(info.size, 50);
        assert_eq!(info.max_concurrency, 250);
    }

    #[test]
    fn test_decrypt_queue_info_creation() {
        let readiness = ReadinessQueueInfo {
            size: 50,
            max_concurrency: 250,
            position: None,
        };
        let tx = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
            position: None,
        };
        let info = DecryptQueueInfo::new(readiness, tx);
        assert_eq!(info.readiness.size, 50);
        assert_eq!(info.tx.size, 100);
    }
}
