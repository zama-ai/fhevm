use crate::errors::TransactionServiceError;
use alloy::primitives::{Address, Bytes, B256};
use alloy::rpc::types::TransactionReceipt;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tokio::sync::Mutex;
use tracing::{error, info, warn};

use super::{TransactionManager, TxConfig};

#[derive(Debug)]
pub struct TransactionService {
    manager: Arc<TransactionManager>,
    pending_txs: Mutex<Vec<PendingTransaction>>,
}

#[derive(Debug)]
struct PendingTransaction {
    target: Address,
    calldata: Bytes,
    config: TxConfig,
    timestamp: Instant,
    attempts: u32,
}
impl TransactionService {
    pub async fn new(
        rpc_url: &str,
        private_key_env: &str,
        chain_id: u64,
    ) -> Result<Arc<Self>, TransactionServiceError> {
        let private_key = match std::env::var(private_key_env) {
            Ok(key) => {
                info!(
                    "Using private key from environment variable: {}",
                    private_key_env
                );
                key
            }
            Err(_) => {
                warn!(
                    "Private key environment variable {} not found, using development key",
                    private_key_env
                );
                // Default development private key (do NOT use in production!)
                "7136d8dc72f873124f4eded25f3525a20f6cee4296564c76b44f1d582c57640f".to_string()
            }
        };

        let manager = TransactionManager::new(rpc_url, &private_key, chain_id)
            .await
            .map_err(|e| TransactionServiceError::Failed(e.to_string()))?;

        Ok(Arc::new(Self {
            manager: Arc::new(manager),
            pending_txs: Mutex::new(Vec::new()),
        }))
    }
    pub async fn submit_transaction(
        self: &Arc<Self>,
        target: Address,
        calldata: Bytes,
        config: TxConfig,
    ) -> Result<B256, TransactionServiceError> {
        let mut pending = self.pending_txs.lock().await;

        // Check for similar pending transactions
        for existing_tx in pending.iter() {
            if existing_tx.target == target && existing_tx.calldata == calldata {
                warn!(
                    target = ?target,
                    "Similar transaction already pending, might want to wait"
                );
            }
        }

        let tx_hash = match self
            .manager
            .send_transaction(target, calldata.clone(), Some(config.clone()))
            .await
        {
            Ok(hash) => hash,
            Err(e) => {
                error!(?e, ?target, "Transaction submission failed");
                return Err(TransactionServiceError::Failed(e.to_string()));
            }
        };

        // Add to pending transactions
        pending.push(PendingTransaction {
            target,
            calldata,
            config,
            timestamp: Instant::now(),
            attempts: 0,
        });

        Ok(tx_hash)
    }

    pub async fn get_transaction_status(
        &self,
        hash: B256,
    ) -> Result<Option<bool>, TransactionServiceError> {
        self.manager
            .wait_for_confirmation(hash, 1)
            .await
            .map(Some)
            .map_err(|e| TransactionServiceError::Failed(e.to_string()))
    }

    pub async fn get_transaction_receipt(
        &self,
        hash: B256,
    ) -> Result<Option<TransactionReceipt>, TransactionServiceError> {
        self.manager
            .provider
            .get_transaction_receipt(hash)
            .await
            .map_err(|e| TransactionServiceError::Failed(e.to_string()))
    }

    // Periodic cleanup of stale pending transactions
    pub async fn cleanup_pending(&self) {
        let mut pending = self.pending_txs.lock().await;
        let now = Instant::now();
        pending.retain(|tx| {
            let age = now.duration_since(tx.timestamp);
            if age > Duration::from_secs(300) {
                // 5 minutes timeout
                warn!(target = ?tx.target, "Removing stale pending transaction");
                false
            } else {
                true
            }
        });
    }
}
