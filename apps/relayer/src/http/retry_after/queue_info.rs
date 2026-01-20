//! Queue information types for dynamic retry-after computation.
//!
//! This module provides types that combine queue state information from
//! different throttlers for ETA computation in HTTP handlers.

use serde::{Deserialize, Serialize};

// Re-export the base queue info types from their source modules
pub use crate::gateway::arbitrum::transaction::tx_throttler::TxQueueInfo;
pub use crate::gateway::readiness_check::readiness_throttler::ReadinessQueueInfo;

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

/// Combined queue info by request type.
///
/// Different request types have different queue structures:
/// - Input proof: Only passes through TX throttler
/// - User/Public decrypt: Passes through readiness queue, then TX throttler
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RequestQueueInfo {
    /// Input proof: only TX throttler (single queue)
    InputProof(TxQueueInfo),
    /// User decrypt: readiness + TX throttler (dual queue)
    UserDecrypt(DecryptQueueInfo),
    /// Public decrypt: readiness + TX throttler (dual queue)
    PublicDecrypt(DecryptQueueInfo),
}

impl RequestQueueInfo {
    /// Create queue info for input proof requests.
    pub fn input_proof(tx: TxQueueInfo) -> Self {
        Self::InputProof(tx)
    }

    /// Create queue info for user decrypt requests.
    pub fn user_decrypt(readiness: ReadinessQueueInfo, tx: TxQueueInfo) -> Self {
        Self::UserDecrypt(DecryptQueueInfo::new(readiness, tx))
    }

    /// Create queue info for public decrypt requests.
    pub fn public_decrypt(readiness: ReadinessQueueInfo, tx: TxQueueInfo) -> Self {
        Self::PublicDecrypt(DecryptQueueInfo::new(readiness, tx))
    }

    /// Get the TX queue info (available for all request types).
    pub fn tx_queue_info(&self) -> &TxQueueInfo {
        match self {
            RequestQueueInfo::InputProof(tx) => tx,
            RequestQueueInfo::UserDecrypt(info) => &info.tx,
            RequestQueueInfo::PublicDecrypt(info) => &info.tx,
        }
    }

    /// Get the readiness queue info (only for decrypt operations).
    pub fn readiness_queue_info(&self) -> Option<&ReadinessQueueInfo> {
        match self {
            RequestQueueInfo::InputProof(_) => None,
            RequestQueueInfo::UserDecrypt(info) => Some(&info.readiness),
            RequestQueueInfo::PublicDecrypt(info) => Some(&info.readiness),
        }
    }

    /// Get the decrypt queue info (only for decrypt operations).
    pub fn decrypt_queue_info(&self) -> Option<&DecryptQueueInfo> {
        match self {
            RequestQueueInfo::InputProof(_) => None,
            RequestQueueInfo::UserDecrypt(info) => Some(info),
            RequestQueueInfo::PublicDecrypt(info) => Some(info),
        }
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

    #[test]
    fn test_request_queue_info_input_proof() {
        let tx = TxQueueInfo {
            size: 100,
            drain_rate_tps: 20,
            position: None,
        };
        let info = RequestQueueInfo::input_proof(tx);

        assert!(matches!(info, RequestQueueInfo::InputProof(_)));
        assert_eq!(info.tx_queue_info().size, 100);
        assert!(info.readiness_queue_info().is_none());
        assert!(info.decrypt_queue_info().is_none());
    }

    #[test]
    fn test_request_queue_info_user_decrypt() {
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
        let info = RequestQueueInfo::user_decrypt(readiness, tx);

        assert!(matches!(info, RequestQueueInfo::UserDecrypt(_)));
        assert_eq!(info.tx_queue_info().size, 100);
        assert_eq!(info.readiness_queue_info().unwrap().size, 50);
        assert!(info.decrypt_queue_info().is_some());
    }

    #[test]
    fn test_request_queue_info_public_decrypt() {
        let readiness = ReadinessQueueInfo {
            size: 30,
            max_concurrency: 250,
            position: None,
        };
        let tx = TxQueueInfo {
            size: 80,
            drain_rate_tps: 25,
            position: None,
        };
        let info = RequestQueueInfo::public_decrypt(readiness, tx);

        assert!(matches!(info, RequestQueueInfo::PublicDecrypt(_)));
        assert_eq!(info.tx_queue_info().size, 80);
        assert_eq!(info.tx_queue_info().drain_rate_tps, 25);
        assert_eq!(info.readiness_queue_info().unwrap().max_concurrency, 250);
    }
}
