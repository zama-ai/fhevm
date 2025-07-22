use alloy::primitives::{Address, TxHash};
use alloy::providers::Provider;
use alloy::rpc::types::TransactionRequest;
use dashmap::DashMap;
use futures::lock::Mutex;
use std::collections::VecDeque;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::time::Duration;
use tokio::sync::oneshot;
use tracing::{debug, error, info, trace, warn};

use crate::error::{Error, Result};

/// A transaction pending in the queue with its result channel
#[derive(Debug)]
struct PendingTransaction {
    tx_request: TransactionRequest,
    result_sender: oneshot::Sender<Result<TxHash>>,
}

/// Sequential nonce manager that processes transactions per wallet in order
/// while keeping block polling completely independent
#[derive(Clone)]
pub struct SequentialNonceManager<P> {
    /// Cached nonces per address (u64::MAX = uninitialized)
    nonces: Arc<DashMap<Address, Arc<Mutex<u64>>>>,

    /// Transaction queues per address for sequential processing
    transaction_queues: Arc<DashMap<Address, Arc<Mutex<VecDeque<PendingTransaction>>>>>,

    /// Processing flags to ensure only one processor per address
    processing_flags: Arc<DashMap<Address, Arc<AtomicBool>>>,

    /// Provider for blockchain calls
    provider: Arc<P>,
}

impl<P> SequentialNonceManager<P>
where
    P: Provider + Clone + Send + Sync + 'static,
{
    /// Create a new sequential nonce manager
    pub fn new(provider: Arc<P>) -> Self {
        Self {
            nonces: Arc::new(DashMap::new()),
            transaction_queues: Arc::new(DashMap::new()),
            processing_flags: Arc::new(DashMap::new()),
            provider,
        }
    }

    /// Queue a transaction for sequential processing
    /// This method is NON-BLOCKING and returns immediately
    /// The actual transaction sending happens asynchronously in background
    pub async fn send_transaction_queued(&self, mut tx: TransactionRequest) -> Result<TxHash> {
        let address = if let Some(addr) = tx.from {
            addr
        } else {
            match self.provider.get_accounts().await {
                Ok(accounts) if !accounts.is_empty() => {
                    let wallet_address = accounts[0];
                    tx.from = Some(wallet_address); // Set it for queue processing
                    info!(
                        "Using provider wallet address for sequential processing: {}",
                        wallet_address
                    );
                    wallet_address
                }
                Ok(_) => {
                    return Err(Error::Transport(
                        "No accounts available from provider".to_string(),
                    ));
                }
                Err(e) => {
                    return Err(Error::Transport(format!(
                        "Failed to get provider accounts: {e}"
                    )));
                }
            }
        };

        let (result_sender, result_receiver) = oneshot::channel();
        let pending_tx = PendingTransaction {
            tx_request: tx,
            result_sender,
        };

        let queue = self
            .transaction_queues
            .entry(address)
            .or_insert_with(|| Arc::new(Mutex::new(VecDeque::new())))
            .clone();

        {
            let mut queue_guard = queue.lock().await;
            queue_guard.push_back(pending_tx);
            debug!(
                "Queued transaction for address {}, queue size: {}",
                address,
                queue_guard.len()
            );
        }

        // Start processing if not already running (non-blocking spawn)
        self.start_processing_if_needed(address).await;

        // Wait for result from background processor
        result_receiver
            .await
            .map_err(|_| Error::Channel("Transaction cancelled".to_string()))?
    }

    /// Start background processing for an address if not already running
    /// This spawns a tokio task that runs independently
    async fn start_processing_if_needed(&self, address: Address) {
        let processing_flag = self
            .processing_flags
            .entry(address)
            .or_insert_with(|| Arc::new(AtomicBool::new(false)))
            .clone();

        // Only start if not already processing
        if !processing_flag.swap(true, Ordering::SeqCst) {
            let manager = self.clone();

            tokio::spawn(async move {
                debug!("Started transaction processor for address {}", address);
                manager.process_transaction_queue(address).await;
                debug!("Stopped transaction processor for address {}", address);
            });
        }
    }

    /// Background task that processes transactions sequentially per address
    async fn process_transaction_queue(&self, address: Address) {
        let queue = self.transaction_queues.get(&address).unwrap().clone();
        let processing_flag = self.processing_flags.get(&address).unwrap().clone();

        loop {
            // Get next transaction from queue
            let pending_tx = {
                let mut queue_guard = queue.lock().await;
                queue_guard.pop_front()
            };

            match pending_tx {
                Some(pending) => {
                    // Process single transaction with proper nonce management
                    let result = self.process_single_transaction(pending.tx_request).await;

                    // Send result back to caller (non-blocking)
                    let _ = pending.result_sender.send(result);
                }
                None => {
                    // Queue is empty, stop processing
                    processing_flag.store(false, Ordering::SeqCst);
                    break;
                }
            }
        }
    }

    /// Process a single transaction with proper nonce management and retry logic
    async fn process_single_transaction(&self, mut tx: TransactionRequest) -> Result<TxHash> {
        let address = tx.from.unwrap();

        // Get next nonce with proper synchronization
        let nonce = self.get_next_nonce_internal(address).await?;
        tx.nonce = Some(nonce);

        trace!(
            "Processing transaction for {} with nonce {}",
            address, nonce
        );

        // Send transaction with retry logic
        match self.send_with_enhanced_retry(tx, nonce).await {
            Ok(tx_hash) => {
                info!("Transaction sent: {} (nonce: {})", tx_hash, nonce);
                Ok(tx_hash)
            }
            Err(e) => {
                // On failure, refresh nonce from chain to recover
                warn!("Transaction failed (nonce: {}): {}", nonce, e);
                if let Err(refresh_err) = self.refresh_nonce_from_chain(address).await {
                    error!("Failed to refresh nonce: {}", refresh_err);
                }
                Err(e)
            }
        }
    }

    /// Get next nonce with proper synchronization
    pub async fn get_next_nonce_internal(&self, address: Address) -> Result<u64> {
        const NONE: u64 = u64::MAX;

        let nonce_mutex = self
            .nonces
            .entry(address)
            .or_insert_with(|| Arc::new(Mutex::new(NONE)))
            .clone();

        let mut nonce_guard = nonce_mutex.lock().await;

        if *nonce_guard == NONE {
            // First time - get from chain
            let chain_nonce = self
                .provider
                .get_transaction_count(address)
                .await
                .map_err(|e| Error::Transport(format!("Failed to get nonce: {e}")))?;
            *nonce_guard = chain_nonce;
            debug!("Initialized nonce for {} to {}", address, chain_nonce);
        }

        let current_nonce = *nonce_guard;
        *nonce_guard += 1;

        Ok(current_nonce)
    }

    /// Send transaction with retry logic and nonce recovery
    /// Uses constant retry strategy: 100ms, then 500ms delays with gas price bumping
    async fn send_with_enhanced_retry(
        &self,
        mut tx: TransactionRequest,
        expected_nonce: u64,
    ) -> Result<TxHash> {
        const MAX_RETRIES: usize = 3;
        let mut attempt = 0;

        while attempt < MAX_RETRIES {
            // Ensure nonce is always set correctly for retries
            tx.nonce = Some(expected_nonce);

            // Set aggressive gas price from first attempt to prevent gas failures
            if attempt == 0 {
                if let Some(gas_price) = tx.gas_price {
                    tx.gas_price = Some(gas_price * 700 / 100); // 600% boost from start
                    info!(
                        "Set aggressive gas price {} for first attempt (nonce: {})",
                        tx.gas_price.unwrap(),
                        expected_nonce
                    );
                }
            }

            match self.provider.send_transaction(tx.clone()).await {
                Ok(pending_tx) => {
                    let tx_hash = *pending_tx.tx_hash();
                    info!(
                        "Transaction sent: {} (nonce: {}, attempt: {})",
                        tx_hash,
                        expected_nonce,
                        attempt + 1
                    );
                    return Ok(tx_hash);
                }
                Err(e) => {
                    let error_msg = e.to_string().to_lowercase();

                    // Check if it's a gas-related error that we can retry
                    if error_msg.contains("gas")
                        || error_msg.contains("underpriced")
                        || error_msg.contains("insufficient")
                    {
                        warn!(
                            "Gas-related error on attempt {} (nonce: {}): {}",
                            attempt + 1,
                            expected_nonce,
                            e
                        );

                        if attempt < MAX_RETRIES - 1 {
                            // Bump gas price by 20% for retry
                            if let Some(gas_price) = tx.gas_price {
                                tx.gas_price = Some(gas_price * 120 / 100);
                                info!("Bumped gas price to {} for retry", tx.gas_price.unwrap());
                            }

                            // Efficient retry delays: 100ms, then 500ms
                            let delay_ms = if attempt == 0 { 100 } else { 500 };
                            tokio::time::sleep(Duration::from_millis(delay_ms)).await;

                            attempt += 1;
                            continue;
                        }
                    }

                    // Non-retryable error or max retries exceeded
                    error!(
                        "Transaction failed permanently (nonce: {}): {}",
                        expected_nonce, e
                    );
                    return Err(Error::Transport(e.to_string()));
                }
            }
        }

        Err(Error::Transport("Max retries exceeded".to_string()))
    }

    /// Refresh nonce from chain (used for recovery)
    async fn refresh_nonce_from_chain(&self, address: Address) -> Result<()> {
        let chain_nonce = self
            .provider
            .get_transaction_count(address)
            .await
            .map_err(|e| Error::Transport(format!("Failed to get chain nonce: {e}")))?;

        if let Some(nonce_mutex) = self.nonces.get(&address) {
            let mut nonce_guard = nonce_mutex.lock().await;
            *nonce_guard = chain_nonce;
            debug!("Refreshed nonce for {} to {}", address, chain_nonce);
        }

        Ok(())
    }

    /// Get current queue size for monitoring
    pub async fn get_queue_size(&self, address: Address) -> usize {
        if let Some(queue) = self.transaction_queues.get(&address) {
            let queue_guard = queue.lock().await;
            queue_guard.len()
        } else {
            0
        }
    }

    /// Get total queued transactions across all addresses
    pub async fn get_total_queued(&self) -> usize {
        let mut total = 0;
        for entry in self.transaction_queues.iter() {
            let queue_guard = entry.value().lock().await;
            total += queue_guard.len();
        }
        total
    }
}
