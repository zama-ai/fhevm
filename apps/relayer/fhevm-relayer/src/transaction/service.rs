use alloy::{
    primitives::{Address, Bytes, B256},
    rpc::types::TransactionReceipt,
};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{error, info, warn};
use uuid::Uuid;

use super::sender::{RetryConfig, TransactionManager, TxConfig};
use crate::errors::TransactionServiceError;

#[derive(Debug, Clone, PartialEq)]
pub enum TransactionStatus {
    Pending,
    Confirmed,
    Failed { reason: String },
}

#[derive(Debug, Clone)]
struct PendingTransaction {
    target: Address,
    calldata: Bytes,
    config: TxConfig,
    timestamp: Instant,
    attempts: u32,
    status: TransactionStatus,
    hash: Option<B256>,
    last_attempt: Instant,
    retry_config: RetryConfig,
    request_id: Uuid,
}

impl PendingTransaction {
    fn should_retry(&self, current_time: Instant) -> bool {
        if self.attempts >= self.retry_config.max_attempts {
            return false;
        }

        let delay = self
            .retry_config
            .base_delay
            .mul_f64(1.5f64.powi(self.attempts as i32))
            .min(self.retry_config.max_delay);

        current_time.duration_since(self.last_attempt) >= delay
    }
}

#[derive(Clone, Debug)]
pub struct TransactionService {
    manager: Arc<TransactionManager>,
    pending_txs: Arc<DashMap<B256, PendingTransaction>>,
    retry_queue: Arc<DashMap<Uuid, PendingTransaction>>,
}

impl TransactionService {
    fn generate_request_id() -> Uuid {
        let ctx = uuid::v1::Context::new(0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = uuid::v1::Timestamp::from_unix(&ctx, now.as_secs(), now.subsec_nanos());
        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        Uuid::new_v1(ts, &node_id).expect("Failed to generate UUID")
    }
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
            pending_txs: Arc::new(DashMap::new()),
            retry_queue: Arc::new(DashMap::new()),
        }))
    }

    pub async fn submit_transaction(
        self: &Arc<Self>,
        target: Address,
        calldata: Bytes,
        config: TxConfig,
    ) -> Result<B256, TransactionServiceError> {
        let request_id = Self::generate_request_id();
        let now = Instant::now();

        let pending_tx = PendingTransaction {
            target,
            calldata: calldata.clone(),
            config: config.clone(),
            timestamp: now,
            attempts: 0,
            status: TransactionStatus::Pending,
            hash: None,
            last_attempt: now,
            retry_config: config.retry_config.as_ref().cloned().unwrap_or_default(),
            request_id,
        };

        match self
            .manager
            .send_transaction(target, calldata, Some(config))
            .await
        {
            Ok(tx_hash) => {
                let mut tx = pending_tx;
                tx.hash = Some(tx_hash);
                tx.attempts = 1;
                self.pending_txs.insert(tx_hash, tx);
                Ok(tx_hash)
            }
            Err(e) => {
                let mut tx = pending_tx;
                tx.attempts = 1;
                self.retry_queue.insert(request_id, tx);
                Err(TransactionServiceError::Failed(e.to_string()))
            }
        }
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

    pub async fn maintain_pending_transactions(&self) -> Result<(), TransactionServiceError> {
        let mut to_process = Vec::new();

        // First, collect all transactions we need to process
        // This reduces the time we hold locks
        {
            for entry in self.pending_txs.iter() {
                let hash = *entry.key();
                let tx = entry.value().clone();
                if tx.status == TransactionStatus::Pending {
                    to_process.push((hash, tx));
                }
            }
        }

        // Process transactions without holding the iterator lock
        for (hash, tx) in to_process {
            let new_status = if Instant::now().duration_since(tx.timestamp)
                > Duration::from_secs(tx.config.timeout_secs.unwrap_or(300))
            {
                Some(TransactionStatus::Failed {
                    reason: "Transaction timeout exceeded".into(),
                })
            } else {
                match self.manager.wait_for_confirmation(hash, 1).await {
                    Ok(true) => Some(TransactionStatus::Confirmed),
                    Ok(false) => Some(TransactionStatus::Failed {
                        reason: "Transaction failed on-chain".into(),
                    }),
                    Err(e) => {
                        warn!(?hash, ?e, "Failed to check transaction status");
                        None
                    }
                }
            };

            // Only update if we have a new status
            if let Some(status) = new_status {
                if let Some(mut tx_entry) = self.pending_txs.get_mut(&hash) {
                    tx_entry.status = status.clone();
                }

                // If failed and can retry, move to retry queue
                if matches!(status, TransactionStatus::Failed { .. })
                    && tx.attempts < tx.retry_config.max_attempts
                {
                    self.pending_txs.remove(&hash);
                    self.retry_queue.insert(tx.request_id, tx);
                }
            }
        }

        Ok(())
    }

    pub async fn maintain_retry_queue(&self) -> Result<(), TransactionServiceError> {
        let now = Instant::now();
        let mut to_retry = Vec::new();

        // Collect transactions that need retry
        {
            for entry in self.retry_queue.iter() {
                let request_id = *entry.key();
                let tx = entry.value().clone();
                if tx.should_retry(now) {
                    to_retry.push((request_id, tx));
                }
            }
        }

        // Process retries without holding the iterator lock
        for (request_id, tx) in to_retry {
            match self
                .manager
                .send_transaction(tx.target, tx.calldata.clone(), Some(tx.config.clone()))
                .await
            {
                Ok(tx_hash) => {
                    let mut new_tx = tx.clone();
                    new_tx.status = TransactionStatus::Pending;
                    new_tx.hash = Some(tx_hash);
                    new_tx.attempts += 1;
                    new_tx.last_attempt = now;

                    // Atomic operations: remove from retry queue and add to pending
                    self.retry_queue.remove(&request_id);
                    self.pending_txs.insert(tx_hash, new_tx);

                    info!(
                        ?tx_hash,
                        attempts = ?tx.attempts,
                        "Retry successful, moving to pending"
                    );
                }
                Err(e) => {
                    if tx.attempts >= tx.retry_config.max_attempts {
                        self.retry_queue.remove(&request_id);
                    }
                    error!(?e, ?request_id, "Retry attempt failed");
                }
            }
        }

        Ok(())
    }

    pub fn spawn_maintenance_tasks(self: Arc<Self>) {
        // Spawn pending transactions maintenance task
        let service_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(5));
            loop {
                interval.tick().await;
                if let Err(e) = service_clone.maintain_pending_transactions().await {
                    error!("Error in maintain_pending_transactions: {}", e);
                    tokio::time::sleep(Duration::from_secs(10)).await;
                }
            }
        });

        // Spawn retry queue maintenance task
        let service_clone = self.clone();
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(30));
            loop {
                interval.tick().await;
                if let Err(e) = service_clone.maintain_retry_queue().await {
                    error!("Error in maintain_retry_queue: {}", e);
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            }
        });
    }
}
