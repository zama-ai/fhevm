use alloy::{
    primitives::{Address, Bytes, B256},
    rpc::types::TransactionReceipt,
};
use dashmap::DashMap;
use std::sync::Arc;
use std::time::{Duration, Instant};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use super::sender::{RetryConfig, TransactionError, TransactionManager, TxConfig};
use crate::core::errors::TransactionServiceError;

/// Represents the current state of a transaction
#[derive(Debug, Clone, PartialEq)]
pub enum TransactionState {
    /// Transaction is waiting to be retried
    WaitingRetry {
        attempts: u32,
        last_attempt: Instant,
    },
    /// Transaction has been sent and is pending confirmation
    Pending {
        hash: B256,
        attempts: u32,
        last_attempt: Instant,
    },
    /// Transaction has been confirmed
    Confirmed,
    /// Transaction has failed permanently
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
    /// When the transaction was first submitted
    timestamp: Instant,
    /// Current state of the transaction
    state: TransactionState,
    // Flag for immediate cleanup
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
    const PENDING_TIMEOUT: Duration = Duration::from_secs(600); // 10 minutes

    fn check_pending_timeout(&self, record: &TransactionRecord, now: Instant) -> bool {
        if let TransactionState::Pending { last_attempt, .. } = record.state {
            return now.duration_since(last_attempt) > Self::PENDING_TIMEOUT;
        }
        false
    }

    pub fn get_transaction_manager(&self) -> &Arc<TransactionManager> {
        &self.manager
    }

    /// Logs state transitions for monitoring and debugging
    fn log_state_transition(
        &self,
        request_id: &Uuid,
        old_state: &TransactionState,
        new_state: &TransactionState,
    ) {
        match (old_state, new_state) {
            (
                TransactionState::WaitingRetry { attempts, .. },
                TransactionState::Pending {
                    hash,
                    attempts: new_attempts,
                    ..
                },
            ) => {
                info!(
                    ?request_id,
                    ?hash,
                    old_attempts = ?attempts,
                    new_attempts = ?new_attempts,
                    "Transaction moved from WaitingRetry to Pending"
                );
            }
            (TransactionState::Pending { hash, attempts, .. }, TransactionState::Confirmed) => {
                info!(
                    ?request_id,
                    ?hash,
                    ?attempts,
                    "Transaction confirmed successfully"
                );
            }
            (
                TransactionState::Pending { hash, attempts, .. },
                TransactionState::Failed { reason },
            ) => {
                error!(?request_id, ?hash, ?attempts, ?reason, "Transaction failed");
            }
            (
                TransactionState::Pending { hash, attempts, .. },
                TransactionState::WaitingRetry {
                    attempts: new_attempts,
                    ..
                },
            ) => {
                warn!(
                    ?request_id,
                    ?hash,
                    old_attempts = ?attempts,
                    new_attempts = ?new_attempts,
                    "Transaction needs retry"
                );
            }
            (
                TransactionState::WaitingRetry { attempts, .. },
                TransactionState::Failed { reason },
            ) => {
                error!(
                    ?request_id,
                    ?attempts,
                    ?reason,
                    "Transaction failed after max retries"
                );
            }
            _ => {
                debug!(?request_id, ?old_state, ?new_state, "State transition");
            }
        }
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
        let now = Instant::now();

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
            timestamp: now,
            state: TransactionState::WaitingRetry {
                attempts: 0,
                last_attempt: now,
            },
            ready_for_cleanup: false,
        };

        self.transactions.insert(request_id, record);

        match self.try_send_transaction(request_id).await {
            Ok(hash) => {
                info!(?request_id, ?hash, "Transaction submitted successfully");
                Ok(hash)
            }
            Err(e) => {
                warn!(
                    ?request_id,
                    ?e,
                    "Initial transaction submission failed, will retry"
                );
                Err(TransactionServiceError::from(e))
            }
        }
    }

    pub async fn get_transaction_receipt(
        &self,
        tx_hash: B256,
    ) -> Result<TransactionReceipt, TransactionServiceError> {
        // First check if we already have this receipt cached
        for record in self.transactions.iter() {
            if let TransactionState::Pending { hash, .. } = record.state {
                if hash == tx_hash {
                    // If we find this transaction in our records, use the manager to get the receipt
                    return self
                        .manager
                        .wait_for_receipt(tx_hash, &record.config)
                        .await
                        .map_err(TransactionServiceError::from);
                }
            }
        }

        // If not found in records, just do a direct call
        match self.manager.provider.get_transaction_receipt(tx_hash).await {
            Ok(Some(receipt)) => Ok(receipt),
            Ok(None) => Err(TransactionServiceError::Failed(
                "Receipt not available yet".into(),
            )),
            Err(e) => Err(TransactionServiceError::Network(e.to_string())),
        }
    }

    /// Attempts to send a transaction
    async fn try_send_transaction(
        &self,
        request_id: Uuid,
    ) -> Result<B256, TransactionServiceError> {
        // Get a snapshot of the record first
        let record = self
            .transactions
            .get(&request_id)
            .ok_or_else(|| TransactionServiceError::Failed("Transaction record not found".into()))?
            .clone();

        let hash = self
            .manager
            .send_transaction(
                record.target,
                record.calldata.clone(),
                Some(record.config.clone()),
            )
            .await?;

        // Update state in a separate, shorter critical section
        self.transactions.entry(request_id).and_modify(|record| {
            let old_state = record.state.clone();
            let new_state = TransactionState::Pending {
                hash,
                attempts: match old_state {
                    TransactionState::WaitingRetry { attempts, .. } => attempts + 1,
                    _ => 1,
                },
                last_attempt: Instant::now(),
            };
            self.log_state_transition(&request_id, &old_state, &new_state);
            record.state = new_state;
        });

        Ok(hash)
    }

    /// Maintains transaction states and handles retries
    pub async fn maintain_transactions(&self) -> Result<(), TransactionServiceError> {
        let now = Instant::now();
        let total_transactions = self.transactions.len();

        debug!(total_transactions, "Starting transaction maintenance cycle");

        let mut to_update = Vec::new();
        let mut to_process = Vec::new();

        // Collect all transactions that need processing
        {
            let entries: Vec<_> = self
                .transactions
                .iter()
                .map(|r| (*r.key(), r.value().clone()))
                .collect();

            for (request_id, record) in entries {
                match &record.state {
                    TransactionState::Pending {
                        hash,
                        attempts,
                        last_attempt: _,
                    } => {
                        // Instead of handling_pending_transaction, use our new method
                        if self.check_pending_timeout(&record, now) {
                            warn!(
                                ?request_id,
                                ?hash,
                                elapsed = ?now.duration_since(record.timestamp).as_secs(),
                                "Transaction monitoring timed out, moving back to retry queue"
                            );

                            // Move back to retry queue
                            to_update.push((
                                request_id,
                                TransactionState::WaitingRetry {
                                    attempts: *attempts,
                                    last_attempt: now,
                                },
                            ));
                        } else {
                            // Use our improved status checking
                            match self.check_transaction_status(*hash, &record.config).await {
                                Ok(new_state) => {
                                    // Only update if the state actually changed
                                    if !matches!(new_state, TransactionState::Pending { .. }) {
                                        to_update.push((request_id, new_state));
                                    }
                                }
                                Err(e) => {
                                    warn!(
                                        ?request_id,
                                        ?hash,
                                        error = %e,
                                        "Error checking transaction status"
                                    );
                                    // Keep current state, we'll retry on next cycle
                                }
                            }
                        }
                    }
                    TransactionState::WaitingRetry {
                        attempts,
                        last_attempt,
                    } => {
                        if self.should_retry(&record.config, *attempts, *last_attempt, now) {
                            to_process.push(request_id);
                        }
                    }
                    _ => {}
                }
            }
        }

        // Apply updates
        for (request_id, new_state) in to_update {
            if let Some(mut record) = self.transactions.get_mut(&request_id) {
                let old_state = record.state.clone();
                self.log_state_transition(&request_id, &old_state, &new_state);
                record.state = new_state;
            }
        }

        // Process retries
        for request_id in to_process {
            if let Err(e) = self.try_send_transaction(request_id).await {
                error!(?request_id, ?e, "Retry attempt failed");
                self.handle_retry_failure(request_id).await;
            }
        }

        self.cleanup_old_transactions(now);
        Ok(())
    }

    async fn handle_retry_failure(&self, request_id: Uuid) {
        // Get the current record
        if let Some(mut record) = self.transactions.get_mut(&request_id) {
            let default_config = RetryConfig::default();
            let retry_config = record
                .config
                .retry_config
                .as_ref()
                .unwrap_or(&default_config);

            match record.state {
                TransactionState::WaitingRetry { attempts, .. } => {
                    let old_state = record.state.clone();

                    // Check if we've hit max retries
                    if attempts >= retry_config.max_attempts {
                        let new_state = TransactionState::Failed {
                            reason: "Max retry attempts exceeded".into(),
                        };
                        self.log_state_transition(&request_id, &old_state, &new_state);
                        record.state = new_state;
                    } else {
                        // Update the attempts count and last_attempt time
                        let new_state = TransactionState::WaitingRetry {
                            attempts: attempts + 1,
                            last_attempt: Instant::now(),
                        };
                        self.log_state_transition(&request_id, &old_state, &new_state);
                        record.state = new_state;
                    }
                }
                _ => {
                    // This shouldn't happen, but log it if it does
                    warn!(
                        ?request_id,
                        state = ?record.state,
                        "Unexpected state in handle_retry_failure"
                    );
                }
            }
        }
    }

    /// Determines if a transaction should be retried
    fn should_retry(
        &self,
        config: &TxConfig,
        attempts: u32,
        last_attempt: Instant,
        now: Instant,
    ) -> bool {
        let default_config = RetryConfig::default();
        let retry_config = config.retry_config.as_ref().unwrap_or(&default_config);

        if attempts >= retry_config.max_attempts {
            debug!(
                ?attempts,
                max_attempts = ?retry_config.max_attempts,
                "Max retry attempts exceeded"
            );
            return false;
        }

        let delay = retry_config
            .base_delay
            .mul_f64(1.5f64.powi(attempts as i32))
            .min(retry_config.max_delay);

        now.duration_since(last_attempt) >= delay
    }

    /// Cleans up old transactions that are no longer needed
    fn cleanup_old_transactions(&self, now: Instant) {
        const CONFIRMED_CLEANUP_THRESHOLD: Duration = Duration::from_secs(30); // Reduce from 1 hour to 30 seconds
        const GENERAL_CLEANUP_THRESHOLD: Duration = Duration::from_secs(3600); // 1 hour for others

        let ready_to_remove: Vec<_> = self
            .transactions
            .iter()
            .filter_map(|entry| {
                if entry.value().ready_for_cleanup {
                    Some(*entry.key())
                } else {
                    None
                }
            })
            .collect();

        for request_id in ready_to_remove {
            if let Some((_, record)) = self.transactions.remove(&request_id) {
                debug!(
                    ?request_id,
                    state = ?record.state,
                    "Cleaned up confirmed transaction immediately"
                );
            }
        }

        // Collect records to remove first
        let to_remove: Vec<_> = self
            .transactions
            .iter()
            .filter_map(|entry| {
                let request_id = *entry.key();
                let record = entry.value();

                match record.state {
                    TransactionState::Confirmed | TransactionState::Failed { .. } => {
                        // Use shorter time for confirmed/failed transactions
                        if now.duration_since(record.timestamp) > CONFIRMED_CLEANUP_THRESHOLD {
                            Some(request_id)
                        } else {
                            None
                        }
                    }
                    _ => {
                        // Use longer time for other states
                        if now.duration_since(record.timestamp) > GENERAL_CLEANUP_THRESHOLD {
                            Some(request_id)
                        } else {
                            None
                        }
                    }
                }
            })
            .collect();

        // Remove in separate  operations
        for request_id in to_remove {
            if let Some((_, record)) = self.transactions.remove(&request_id) {
                debug!(
                    ?request_id,
                    state = ?record.state,
                    age = ?now.duration_since(record.timestamp).as_secs(),
                    "Cleaned up  transaction"
                );
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
                    Ok(TransactionState::Confirmed)
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
                    attempts: 1, // This should be the actual attempt count from context
                    last_attempt: Instant::now(),
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
        for mut record in self.transactions.iter_mut() {
            if let TransactionState::Pending { hash, .. } = record.state {
                if hash == tx_hash {
                    record.ready_for_cleanup = true;
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

    /// Spawns maintenance tasks
    pub fn spawn_maintenance_tasks(self: Arc<Self>) {
        // Spawn maintenance task
        tokio::spawn(async move {
            let mut interval = tokio::time::interval(Duration::from_secs(15));
            loop {
                interval.tick().await;
                if let Err(e) = self.maintain_transactions().await {
                    error!(error = %e, "Error in maintain_transactions");
                    tokio::time::sleep(Duration::from_secs(30)).await;
                }
            }
        });
    }
}
