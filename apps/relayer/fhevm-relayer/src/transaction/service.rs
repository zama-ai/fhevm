use alloy::{
    primitives::{Address, Bytes, B256},
    rpc::types::TransactionReceipt,
};
use dashmap::DashMap;
use rand::Rng;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::sender::{TransactionError, TransactionManager, TxConfig};
use crate::core::errors::TransactionServiceError;

/// Represents the current state of a transaction
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    /// Starting state: Transaction is ready to be sent
    Ready,

    /// In progress: Transaction has been submitted and is awaiting confirmation
    Pending {
        hash: B256,
        submit_time: Instant,
        attempts: u32,
    },

    /// Success state: Transaction has been confirmed
    Confirmed { receipt: Arc<TransactionReceipt> },

    /// Failure state: Transaction has failed
    Failed { reason: String },
}

/// Records all information about a transaction
#[derive(Debug, Clone)]
struct TransactionRecord {
    /// Target contract address
    target: Address,
    /// Transaction calldata
    calldata: Bytes,
    /// Transaction configuration
    config: TxConfig,
    /// Current state of the transaction
    state: TransactionState,

    /// When the transaction should be cleaned up (None = not ready)
    cleanup_after: Option<Instant>,
    /// Flag for immediate cleanup
    ready_for_cleanup: bool,
}

/// Main service for managing transactions
#[derive(Clone, Debug)]
pub struct TransactionService {
    /// Transaction manager for interacting with the blockchain
    manager: Arc<TransactionManager>,
    /// Single source of truth for all transaction states
    transactions: Arc<DashMap<Uuid, TransactionRecord>>,
}

impl TransactionService {
    pub fn get_transaction_manager(&self) -> &Arc<TransactionManager> {
        &self.manager
    }

    /// Creates a new instance of TransactionService
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
            transactions: Arc::new(DashMap::new()),
        }))
    }

    fn generate_request_id() -> Uuid {
        let ctx = uuid::v1::Context::new(0);
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .expect("Time went backwards");
        let ts = uuid::v1::Timestamp::from_unix(&ctx, now.as_secs(), now.subsec_nanos());
        let node_id = [0x01, 0x23, 0x45, 0x67, 0x89, 0xab];
        Uuid::new_v1(ts, &node_id)
    }

    /// Submits a new transaction
    pub async fn submit_transaction(
        self: &Arc<Self>,
        target: Address,
        calldata: Bytes,
        config: TxConfig,
    ) -> Result<B256, TransactionServiceError> {
        let request_id = Self::generate_request_id();

        info!(
            ?request_id,
            ?target,
            calldata_size = calldata.len(),
            "Submitting new transaction"
        );

        let record = TransactionRecord {
            target,
            calldata: calldata.clone(),
            config: config.clone(),
            state: TransactionState::Ready,
            cleanup_after: None,
            ready_for_cleanup: false,
        };

        self.transactions.insert(request_id, record);
        self.process_transaction(request_id).await
    }

    async fn process_transaction(&self, request_id: Uuid) -> Result<B256, TransactionServiceError> {
        // Get a snapshot of the record
        let record = match self.transactions.get(&request_id) {
            Some(record) => record.clone(),
            None => {
                return Err(TransactionServiceError::Failed(
                    "Transaction not found".into(),
                ))
            }
        };

        match record.state {
            TransactionState::Ready => {
                // Send the transaction to the network
                self.send_transaction(request_id, &record).await
            }
            TransactionState::Pending { hash, .. } => {
                // Return the hash for already pending transactions
                Ok(hash)
            }
            TransactionState::Confirmed { .. } => {
                // Transaction is already confirmed, extract hash from receipt
                match self.get_tx_hash_from_record(&record) {
                    Some(hash) => Ok(hash),
                    None => Err(TransactionServiceError::Failed(
                        "Could not retrieve transaction hash from confirmed transaction".into(),
                    )),
                }
            }
            TransactionState::Failed { reason } => Err(TransactionServiceError::Failed(reason)),
        }
    }

    fn get_tx_hash_from_record(&self, record: &TransactionRecord) -> Option<B256> {
        match &record.state {
            TransactionState::Pending { hash, .. } => Some(*hash),
            TransactionState::Confirmed { receipt } => Some(receipt.transaction_hash),
            _ => None,
        }
    }

    async fn send_transaction(
        &self,
        request_id: Uuid,
        record: &TransactionRecord,
    ) -> Result<B256, TransactionServiceError> {
        // Try to send the transaction
        match self
            .manager
            .send_transaction(
                record.target,
                record.calldata.clone(),
                Some(record.config.clone()),
            )
            .await
        {
            Ok(hash) => {
                // Update state to pending
                self.transactions.entry(request_id).and_modify(|record| {
                    record.state = TransactionState::Pending {
                        hash,
                        submit_time: Instant::now(),
                        attempts: 1,
                    };
                });

                Ok(hash)
            }
            Err(e) => {
                // Update state to failed
                self.transactions.entry(request_id).and_modify(|record| {
                    record.state = TransactionState::Failed {
                        reason: format!("Transaction submission failed: {}", e),
                    };

                    // Mark for cleanup after 5 minutes
                    record.cleanup_after = Some(Instant::now() + Duration::from_secs(300));
                });

                Err(e.into())
            }
        }
    }

    pub async fn get_transaction_receipt(
        &self,
        tx_hash: B256,
    ) -> Result<TransactionReceipt, TransactionServiceError> {
        // First check if we already have this receipt in our records
        for record in self.transactions.iter() {
            match &record.state {
                TransactionState::Confirmed { receipt } if receipt.transaction_hash == tx_hash => {
                    return Ok(receipt.as_ref().clone());
                }
                TransactionState::Pending { hash, .. } if *hash == tx_hash => {
                    // Found a pending transaction, wait for receipt with default timeout
                    return self
                        .wait_for_receipt(tx_hash, Duration::from_secs(60))
                        .await;
                }
                _ => {}
            }
        }

        // If not found in our records, do a direct provider call
        match self.manager.provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => Ok(receipt),
            Ok(None) => Err(TransactionServiceError::Failed(
                "Receipt not available yet".into(),
            )),
            Err(e) => Err(TransactionServiceError::Network(e.to_string())),
        }
    }

    /// Maintains transaction states and handles retries
    pub async fn maintain_transactions(&self) -> Result<(), TransactionServiceError> {
        let now = Instant::now();
        let total_transactions = self.transactions.len();

        debug!(total_transactions, "Starting transaction maintenance cycle");

        // Step 1: Clean up any transactions marked for cleanup
        self.cleanup_transactions(now);

        // Step 2: Process pending transactions
        let pending_transactions: Vec<_> = self
            .transactions
            .iter()
            .filter_map(|entry| {
                if let TransactionState::Pending { .. } = entry.value().state {
                    Some(*entry.key())
                } else {
                    None
                }
            })
            .collect();

        for request_id in pending_transactions {
            if let Err(e) = self.check_pending_transaction(request_id).await {
                warn!(?request_id, ?e, "Error checking transaction status");
            }
        }

        Ok(())
    }

    async fn check_pending_transaction(
        &self,
        request_id: Uuid,
    ) -> Result<(), TransactionServiceError> {
        // Get a snapshot of the record
        let record = match self.transactions.get(&request_id) {
            Some(record) => record.clone(),
            None => return Ok(()), // Already removed, nothing to do
        };

        if let TransactionState::Pending {
            hash,
            submit_time,
            attempts,
        } = record.state
        {
            // Check for timeout
            let elapsed = Instant::now().duration_since(submit_time);
            let timeout_secs = record.config.timeout_secs.unwrap_or(60);

            if elapsed > Duration::from_secs(timeout_secs) {
                // Transaction has timed out
                self.transactions.entry(request_id).and_modify(|record| {
                    record.state = TransactionState::Failed {
                        reason: format!(
                            "Transaction timed out after {} seconds",
                            elapsed.as_secs()
                        ),
                    };
                    record.cleanup_after = Some(Instant::now() + Duration::from_secs(300));
                });

                info!(
                    ?request_id,
                    ?hash,
                    elapsed_secs = elapsed.as_secs(),
                    timeout_secs,
                    "Transaction timed out"
                );

                return Ok(());
            }

            // Check for receipt
            match self.manager.provider.get_transaction_receipt(hash).await {
                Ok(Some(receipt)) => {
                    // We have a receipt - update state based on status
                    let success = receipt.status();

                    self.transactions.entry(request_id).and_modify(|record| {
                        if success {
                            record.state = TransactionState::Confirmed {
                                receipt: Arc::new(receipt.clone()),
                            };

                            info!(
                                ?request_id,
                                ?hash,
                                block_number = ?receipt.block_number,
                                gas_used = ?receipt.gas_used,
                                "Transaction confirmed successfully"
                            );
                        } else {
                            record.state = TransactionState::Failed {
                                reason: "Transaction reverted on chain".into(),
                            };

                            error!(
                                ?request_id,
                                ?hash,
                                block_number = ?receipt.block_number,
                                "Transaction reverted on chain"
                            );
                        }

                        // Mark for cleanup after 1 minute
                        record.cleanup_after = Some(Instant::now() + Duration::from_secs(60));
                    });
                }
                Ok(None) => {
                    // No receipt yet, continue waiting
                    debug!(
                        ?request_id,
                        ?hash,
                        attempts,
                        elapsed_secs = elapsed.as_secs(),
                        "Transaction still pending, no receipt available"
                    );
                }
                Err(e) => {
                    // Error getting receipt - log and continue waiting
                    warn!(
                        ?request_id,
                        ?hash,
                        error = %e,
                        "Error getting receipt, will retry later"
                    );
                }
            }
        }

        Ok(())
    }

    fn cleanup_transactions(&self, now: Instant) {
        // Find transactions ready for cleanup
        let to_remove: Vec<_> = self
            .transactions
            .iter()
            .filter_map(|entry| {
                if let Some(cleanup_time) = entry.value().cleanup_after {
                    if now >= cleanup_time {
                        Some(*entry.key())
                    } else {
                        None
                    }
                } else {
                    None
                }
            })
            .collect();

        // Remove them
        for request_id in to_remove {
            if let Some((_, record)) = self.transactions.remove(&request_id) {
                debug!(
                    ?request_id,
                    state = ?record.state,
                    "Cleaned up transaction"
                );
            }
        }
    }

    pub fn get_transaction_state(&self, request_id: Uuid) -> Option<TransactionState> {
        self.transactions
            .get(&request_id)
            .map(|record| record.state.clone())
    }

    pub fn cancel_transaction(&self, request_id: Uuid) -> Result<(), TransactionServiceError> {
        if let Some(mut record) = self.transactions.get_mut(&request_id) {
            // We can only cancel transactions that are pending
            if let TransactionState::Pending { hash, .. } = record.state {
                record.state = TransactionState::Failed {
                    reason: "Transaction canceled by user".into(),
                };

                record.cleanup_after = Some(Instant::now() + Duration::from_secs(60));

                info!(?request_id, ?hash, "Transaction canceled by user");

                Ok(())
            } else {
                Err(TransactionServiceError::Failed(
                    "Cannot cancel transaction that is not pending".into(),
                ))
            }
        } else {
            Err(TransactionServiceError::Failed(
                "Transaction not found".into(),
            ))
        }
    }

    pub async fn submit_and_wait(
        self: &Arc<Self>,
        target: Address,
        calldata: Bytes,
        config: TxConfig,
    ) -> Result<TransactionReceipt, TransactionServiceError> {
        let tx_hash = self
            .submit_transaction(target, calldata, config.clone())
            .await?;

        let timeout = Duration::from_secs(config.timeout_secs.unwrap_or(60));
        self.wait_for_receipt(tx_hash, timeout).await
    }

    pub async fn wait_for_receipt(
        &self,
        tx_hash: B256,
        timeout: Duration,
    ) -> Result<TransactionReceipt, TransactionServiceError> {
        let start = Instant::now();
        let base_delay = Duration::from_millis(200); // Start with 200ms
        let max_delay = Duration::from_secs(10); // Cap at 10 seconds
        let mut attempt = 0;

        // Introduce an **initial delay** before the first attempt
        // TODO: Make this configurable
        tokio::time::sleep(Duration::from_millis(400)).await;

        info!(
            ?tx_hash,
            ?attempt,
            elapsed = ?start.elapsed().as_millis(),
            "First attempt to get receipt (ms)"
        );

        loop {
            // Check timeout
            if start.elapsed() > timeout {
                return Err(TransactionServiceError::Timeout(timeout.as_secs()));
            }

            // Try to get receipt
            match self.manager.provider.get_transaction_receipt(tx_hash).await {
                Ok(Some(receipt)) => return Ok(receipt),
                Ok(None) => {
                    // Calculate exponential backoff with jitter
                    let backoff_base = base_delay.mul_f64(1.5f64.powi(attempt));
                    let jitter = 0.8 + (0.4 * rand::rng().random::<f64>());
                    let delay = backoff_base.mul_f64(jitter).min(max_delay);

                    info!(
                        ?tx_hash,
                        attempt = attempt + 1,
                        delay_ms = ?delay.as_millis(),
                        elapsed = ?start.elapsed().as_secs(),
                        "Receipt not available yet, waiting with backoff"
                    );

                    tokio::time::sleep(delay).await;
                    attempt += 1;
                }
                Err(e) => {
                    // Log error and use a shorter retry delay for network errors
                    warn!(?tx_hash, ?e, "Error getting receipt");
                    tokio::time::sleep(base_delay).await;
                }
            }
        }
    }

    pub async fn check_transaction_status(
        &self,
        tx_hash: B256,
        config: &TxConfig,
    ) -> Result<TransactionState, TransactionServiceError> {
        match self
            .get_transaction_receipt_with_retries(tx_hash, config)
            .await
        {
            Ok(receipt) => {
                if receipt.status() {
                    self.mark_transaction_for_cleanup(tx_hash);
                    // Create with receipt field
                    let receipt_arc = Arc::new(receipt);
                    Ok(TransactionState::Confirmed {
                        receipt: receipt_arc,
                    })
                } else {
                    Ok(TransactionState::Failed {
                        reason: "Transaction reverted on chain".into(),
                    })
                }
            }
            Err(TransactionServiceError::Timeout(_)) => {
                // For timeouts, we keep waiting - transaction might still succeed
                Ok(TransactionState::Pending {
                    hash: tx_hash,
                    submit_time: Instant::now(), // Use submit_time instead of last_attempt
                    attempts: 1, // This should be the actual attempt count from context
                })
            }
            Err(e) => {
                // For other errors, mark as failed
                Ok(TransactionState::Failed {
                    reason: format!("Transaction failed: {}", e),
                })
            }
        }
    }

    fn mark_transaction_for_cleanup(&self, tx_hash: B256) {
        for mut entry in self.transactions.iter_mut() {
            if let TransactionState::Pending { hash, .. } = entry.state {
                if hash == tx_hash {
                    // Need to update the whole record to add ready_for_cleanup
                    entry.value_mut().ready_for_cleanup = true;
                    break;
                }
            }
        }
    }

    /// Gets a transaction receipt
    pub async fn get_transaction_receipt_with_retries(
        &self,
        tx_hash: B256,
        config: &TxConfig,
    ) -> Result<TransactionReceipt, TransactionServiceError> {
        match self.manager.wait_for_receipt(tx_hash, config).await {
            Ok(receipt) => Ok(receipt),
            Err(e) => Err(match e {
                TransactionError::TransactionTimeout(secs) => {
                    TransactionServiceError::Timeout(secs)
                }
                TransactionError::TransactionFailed(reason) => {
                    TransactionServiceError::Failed(reason)
                }
                TransactionError::RpcError(err) => TransactionServiceError::Network(err),
                _ => TransactionServiceError::Failed(format!("Receipt error: {}", e)),
            }),
        }
    }

    pub fn spawn_maintenance_tasks(self: Arc<Self>) {
        // Spawn maintenance task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));

            loop {
                interval.tick().await;

                if let Err(e) = self.maintain_transactions().await {
                    error!(error = %e, "Error in maintain_transactions");
                    // Add a small delay after errors to prevent CPU spinning
                    tokio::time::sleep(Duration::from_secs(5)).await;
                }
            }
        });

        info!("Transaction maintenance task spawned");
    }
}
