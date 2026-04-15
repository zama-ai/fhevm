//! EVM Block Fetcher - Production-ready module for fetching EVM blocks with receipts.
//!
//! Provides 5 different fetching strategies with parallel tokio tasks, cancellation support,
//! smart retry on RPC errors, and optional block verification.
//!
//! # Strategies
//!
//! 1. **Block + eth_getBlockReceipts**: Most efficient when supported. 2 parallel tasks.
//! 2. **Block + Batch Receipts**: Single HTTP request with batched JSON-RPC.
//! 3. **Block + Chunked Batch Receipts**: Batches in configurable chunk sizes.
//! 4. **Block + Individual Receipts**: One task per receipt, maximum parallelism.
//! 5. **Block + Sequential Receipts**: One receipt at a time, rate-limit friendly.
//!
//! # Error Handling
//!
//! The fetcher classifies RPC errors into three categories:
//! - **Unrecoverable**: `UnsupportedMethod`, `BatchUnsupported`, `DeserializationError` - fail immediately
//! - **RateLimited**: `RateLimited` (HTTP 429) - retry with exponential backoff (500ms -> 1s -> 2s -> ... -> max)
//! - **Recoverable**: `TransportError`, `NotFound`, etc. - retry with fixed interval
//!
//! # Example
//!
//! ```ignore
//! use listener_core::blockchain::evm_block_fetcher::{EvmBlockFetcher, FetchConfig};
//! use tokio_util::sync::CancellationToken;
//!
//! let fetcher = EvmBlockFetcher::new(provider)
//!     .with_verify_block(true)
//!     .with_retry_interval(Duration::from_millis(500));
//!
//! let block = fetcher.fetch_block_with_block_receipts_by_number(12345).await?;
//! ```

use std::collections::HashMap;
use std::future::Future;
use std::time::Duration;

use alloy::network::{AnyRpcBlock, AnyTransactionReceipt};
use alloy::primitives::B256;
use thiserror::Error;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, warn};
use uuid::Uuid;

use super::evm_block_computer::{BlockVerificationError, EvmBlockComputer};
use super::sem_evm_rpc_provider::{RpcProviderError, SemEvmRpcProvider};

/// Errors that can occur during block fetching.
///
/// Includes both operational errors and unrecoverable RPC errors.
/// Recoverable RPC errors (transport issues, rate limits) trigger retry internally.
#[derive(Error, Debug)]
pub enum BlockFetchError {
    #[error("Fetch cancelled")]
    Cancelled,

    #[error("Block verification failed: {0}")]
    VerificationFailed(#[from] BlockVerificationError),

    #[error(
        "Receipt count mismatch: block has {block_tx_count} transactions but got {receipt_count} receipts"
    )]
    ReceiptCountMismatch {
        block_tx_count: usize,
        receipt_count: usize,
    },

    #[error("Block not found after fetch")]
    BlockNotFound,

    #[error("Missing receipt for transaction {tx_hash}")]
    MissingReceipt { tx_hash: B256 },

    #[error("RPC method not supported: {method} - {details}")]
    UnsupportedMethod { method: String, details: String },

    #[error("Batch requests not supported by RPC node")]
    BatchUnsupported,

    #[error("RPC deserialization error in {method}: {details}")]
    DeserializationError { method: String, details: String },
}

/// A successfully fetched block with all its receipts.
#[derive(Debug, Clone)]
pub struct FetchedBlock {
    /// Unique identifier for this fetch operation (useful for distributed tracing/logging).
    pub fetch_id: Uuid,
    /// The fetched block with full transaction details.
    pub block: AnyRpcBlock,
    /// Receipts indexed by transaction hash for O(1) lookup.
    pub receipts: HashMap<B256, AnyTransactionReceipt>,
}

impl FetchedBlock {
    /// Get a receipt by transaction hash.
    pub fn get_receipt(&self, tx_hash: &B256) -> Option<&AnyTransactionReceipt> {
        self.receipts.get(tx_hash)
    }

    /// Get receipts ordered by transaction index in the block.
    pub fn receipts_ordered(&self) -> Vec<&AnyTransactionReceipt> {
        let tx_hashes: Vec<B256> = self.block.transactions.hashes().collect();
        tx_hashes
            .iter()
            .filter_map(|hash| self.receipts.get(hash))
            .collect()
    }

    /// Get the number of transactions in the block.
    pub fn transaction_count(&self) -> usize {
        self.block.transactions.len()
    }
}

/// Configuration for block fetching operations.
#[derive(Clone)]
pub struct FetchConfig {
    /// Interval between retry attempts on RPC failure.
    pub retry_interval: Duration,
    /// Whether to verify block integrity after fetching.
    pub verify_block: bool,
    /// Token for cancelling the fetch operation.
    pub cancellation_token: CancellationToken,
    /// Maximum backoff duration (ms) for rate-limit retries (exponential backoff cap).
    pub max_exponential_backoff_ms: u64,
}

impl Default for FetchConfig {
    fn default() -> Self {
        Self {
            retry_interval: Duration::from_millis(500),
            verify_block: false,
            cancellation_token: CancellationToken::new(),
            max_exponential_backoff_ms: 20_000,
        }
    }
}

/// EVM block fetcher with multiple fetching strategies.
///
/// Provides production-ready block fetching with:
/// - Parallel task execution
/// - Infinite retry on RPC errors with triage with critical errors
/// - Cancellation support via CancellationToken
/// - Optional block verification
#[derive(Clone)]
pub struct EvmBlockFetcher {
    provider: SemEvmRpcProvider,
    config: FetchConfig,
}

impl EvmBlockFetcher {
    /// Create a new block fetcher with default configuration.
    pub fn new(provider: SemEvmRpcProvider) -> Self {
        Self {
            provider,
            config: FetchConfig::default(),
        }
    }

    /// Set the retry interval for RPC failures.
    pub fn with_retry_interval(mut self, interval: Duration) -> Self {
        self.config.retry_interval = interval;
        self
    }

    /// Enable or disable block verification after fetching.
    pub fn with_verify_block(mut self, verify: bool) -> Self {
        self.config.verify_block = verify;
        self
    }

    /// Set the cancellation token for this fetcher.
    pub fn with_cancellation_token(mut self, token: CancellationToken) -> Self {
        self.config.cancellation_token = token;
        self
    }

    /// Set the maximum exponential backoff duration (ms) for rate-limit retries.
    pub fn with_max_exponential_backoff_ms(mut self, max_ms: u64) -> Self {
        self.config.max_exponential_backoff_ms = max_ms;
        self
    }

    /// Fetch a block and its receipts using eth_getBlockReceipts (by block number).
    ///
    /// This is the most efficient strategy when the RPC node supports eth_getBlockReceipts.
    /// Spawns 2 parallel tasks: one for the block, one for all receipts.
    pub async fn fetch_block_with_block_receipts_by_number(
        &self,
        block_number: u64,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let mut join_set: JoinSet<Result<FetchTaskResult, BlockFetchError>> = JoinSet::new();

        // Spawn block fetch task
        let provider = self.provider.clone();
        let retry_interval = self.config.retry_interval;
        let max_backoff = self.config.max_exponential_backoff_ms;
        let token = child_token.clone();
        join_set.spawn(async move {
            let block = retry_with_cancel(
                || provider.get_block_by_number(block_number),
                retry_interval,
                max_backoff,
                &token,
            )
            .await?;
            Ok(FetchTaskResult::Block(Box::new(block)))
        });

        // Spawn receipts fetch task
        let provider = self.provider.clone();
        let token = child_token.clone();
        join_set.spawn(async move {
            let receipts = retry_with_cancel(
                || provider.get_block_receipts(block_number),
                retry_interval,
                max_backoff,
                &token,
            )
            .await?;
            Ok(FetchTaskResult::Receipts(receipts))
        });

        self.collect_results(fetch_id, join_set, child_token).await
    }

    /// Fetch a block and its receipts using eth_getBlockReceipts (by block hash).
    pub async fn fetch_block_with_block_receipts_by_hash(
        &self,
        block_hash: B256,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let mut join_set: JoinSet<Result<FetchTaskResult, BlockFetchError>> = JoinSet::new();

        // Spawn block fetch task
        let provider = self.provider.clone();
        let retry_interval = self.config.retry_interval;
        let max_backoff = self.config.max_exponential_backoff_ms;
        let token = child_token.clone();
        let hash_str = format!("{:?}", block_hash);
        join_set.spawn(async move {
            let block = retry_with_cancel(
                || provider.get_block_by_hash(hash_str.clone()),
                retry_interval,
                max_backoff,
                &token,
            )
            .await?;
            Ok(FetchTaskResult::Block(Box::new(block)))
        });

        // For receipts by hash, we need to first get block number from the block
        // So we fetch block first, then receipts
        let provider = self.provider.clone();
        let token = child_token.clone();
        let hash_str = format!("{:?}", block_hash);
        join_set.spawn(async move {
            // First get the block to find its number
            let block = retry_with_cancel(
                || provider.get_block_by_hash(hash_str.clone()),
                retry_interval,
                max_backoff,
                &token,
            )
            .await?;
            let block_number = block.header.number;

            // Then get receipts by block number
            let receipts = retry_with_cancel(
                || provider.get_block_receipts(block_number),
                retry_interval,
                max_backoff,
                &token,
            )
            .await?;
            Ok(FetchTaskResult::Receipts(receipts))
        });

        self.collect_results(fetch_id, join_set, child_token).await
    }

    /// Fetch a block and its receipts using batched JSON-RPC (by block number).
    ///
    /// Single HTTP request with all receipt requests batched together.
    pub async fn fetch_block_with_batch_receipts_by_number(
        &self,
        block_number: u64,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_number(block_number),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let tx_hashes: Vec<B256> = block.transactions.hashes().collect();

        if tx_hashes.is_empty() {
            return self.build_fetched_block(fetch_id, block, vec![]);
        }

        // Fetch all receipts in a single batch
        let receipts = retry_with_cancel(
            || self.provider.get_transaction_receipts_batch(&tx_hashes),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Fetch a block and its receipts using batched JSON-RPC (by block hash).
    pub async fn fetch_block_with_batch_receipts_by_hash(
        &self,
        block_hash: B256,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let hash_str = format!("{:?}", block_hash);

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_hash(hash_str.clone()),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let tx_hashes: Vec<B256> = block.transactions.hashes().collect();

        if tx_hashes.is_empty() {
            return self.build_fetched_block(fetch_id, block, vec![]);
        }

        // Fetch all receipts in a single batch
        let receipts = retry_with_cancel(
            || self.provider.get_transaction_receipts_batch(&tx_hashes),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Fetch a block and its receipts using chunked batched JSON-RPC (by block number).
    ///
    /// Receipts are fetched in parallel chunks of `batch_size`.
    pub async fn fetch_block_by_number_with_parallel_batched_receipts(
        &self,
        block_number: u64,
        batch_receipt_size_range: usize,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_number(block_number),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self
            .fetch_receipts_chunked(&block, batch_receipt_size_range, &child_token)
            .await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Fetch a block and its receipts using chunked batched JSON-RPC (by block hash).
    pub async fn fetch_block_by_hash_with_parallel_batched_receipts(
        &self,
        block_hash: B256,
        batch_size: usize,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let hash_str = format!("{:?}", block_hash);

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_hash(hash_str.clone()),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self
            .fetch_receipts_chunked(&block, batch_size, &child_token)
            .await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Helper to fetch receipts in chunks.
    async fn fetch_receipts_chunked(
        &self,
        block: &AnyRpcBlock,
        batch_size: usize,
        cancel_token: &CancellationToken,
    ) -> Result<Vec<AnyTransactionReceipt>, BlockFetchError> {
        let tx_hashes: Vec<B256> = block.transactions.hashes().collect();

        if tx_hashes.is_empty() {
            return Ok(vec![]);
        }

        let chunks: Vec<Vec<B256>> = tx_hashes
            .chunks(batch_size)
            .map(|chunk| chunk.to_vec())
            .collect();

        let mut join_set: JoinSet<Result<Vec<AnyTransactionReceipt>, BlockFetchError>> =
            JoinSet::new();

        for chunk in chunks {
            let provider = self.provider.clone();
            let retry_interval = self.config.retry_interval;
            let max_backoff = self.config.max_exponential_backoff_ms;
            let token = cancel_token.clone();

            join_set.spawn(async move {
                retry_with_cancel(
                    || provider.get_transaction_receipts_batch(&chunk),
                    retry_interval,
                    max_backoff,
                    &token,
                )
                .await
            });
        }

        let mut all_receipts = Vec::with_capacity(tx_hashes.len());

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(chunk_receipts)) => {
                    all_receipts.extend(chunk_receipts);
                }
                Ok(Err(e)) => {
                    cancel_token.cancel();
                    // Drain remaining tasks
                    while join_set.join_next().await.is_some() {}
                    return Err(e);
                }
                Err(join_err) => {
                    cancel_token.cancel();
                    while join_set.join_next().await.is_some() {}
                    // JoinError means task panicked - treat as cancelled
                    warn!(error = %join_err, "Task panicked during chunked receipt fetch");
                    return Err(BlockFetchError::Cancelled);
                }
            }
        }

        Ok(all_receipts)
    }

    /// Fetch a block and its receipts individually (by block number).
    ///
    /// One task per receipt - maximum parallelism but more RPC calls.
    pub async fn fetch_block_with_individual_receipts_by_number(
        &self,
        block_number: u64,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_number(block_number),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self.fetch_receipts_individual(&block, &child_token).await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Fetch a block and its receipts individually (by block hash).
    pub async fn fetch_block_with_individual_receipts_by_hash(
        &self,
        block_hash: B256,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let hash_str = format!("{:?}", block_hash);

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_hash(hash_str.clone()),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self.fetch_receipts_individual(&block, &child_token).await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Helper to fetch receipts individually (one task per receipt).
    async fn fetch_receipts_individual(
        &self,
        block: &AnyRpcBlock,
        cancel_token: &CancellationToken,
    ) -> Result<Vec<AnyTransactionReceipt>, BlockFetchError> {
        let tx_hashes: Vec<B256> = block.transactions.hashes().collect();

        if tx_hashes.is_empty() {
            return Ok(vec![]);
        }

        let mut join_set: JoinSet<Result<(usize, AnyTransactionReceipt), BlockFetchError>> =
            JoinSet::new();

        for (index, tx_hash) in tx_hashes.iter().enumerate() {
            let provider = self.provider.clone();
            let retry_interval = self.config.retry_interval;
            let max_backoff = self.config.max_exponential_backoff_ms;
            let token = cancel_token.clone();
            let hash_str = tx_hash.to_string();

            join_set.spawn(async move {
                let receipt = retry_with_cancel(
                    || provider.get_transaction_receipt(hash_str.clone()),
                    retry_interval,
                    max_backoff,
                    &token,
                )
                .await?;
                Ok((index, receipt))
            });
        }

        let mut indexed_receipts: Vec<(usize, AnyTransactionReceipt)> =
            Vec::with_capacity(tx_hashes.len());

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(indexed_receipt)) => {
                    indexed_receipts.push(indexed_receipt);
                }
                Ok(Err(e)) => {
                    cancel_token.cancel();
                    while join_set.join_next().await.is_some() {}
                    return Err(e);
                }
                Err(join_err) => {
                    cancel_token.cancel();
                    while join_set.join_next().await.is_some() {}
                    warn!(error = %join_err, "Task panicked during individual receipt fetch");
                    return Err(BlockFetchError::Cancelled);
                }
            }
        }

        // Sort by index to maintain transaction order
        indexed_receipts.sort_by_key(|(idx, _)| *idx);
        Ok(indexed_receipts.into_iter().map(|(_, r)| r).collect())
    }

    /// Fetch a block and its receipts sequentially (by block number).
    ///
    /// Receipts are fetched one at a time - slowest but most RPC-friendly.
    /// Use this strategy for rate-limited RPC providers.
    pub async fn fetch_block_with_sequential_receipts_by_number(
        &self,
        block_number: u64,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_number(block_number),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self.fetch_receipts_sequential(&block, &child_token).await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Fetch a block and its receipts sequentially (by block hash).
    pub async fn fetch_block_with_sequential_receipts_by_hash(
        &self,
        block_hash: B256,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let fetch_id = Uuid::new_v4();
        let child_token = self.config.cancellation_token.child_token();
        let hash_str = format!("{:?}", block_hash);

        // First fetch the block to get transaction hashes
        let block = retry_with_cancel(
            || self.provider.get_block_by_hash(hash_str.clone()),
            self.config.retry_interval,
            self.config.max_exponential_backoff_ms,
            &child_token,
        )
        .await?;

        let receipts = self.fetch_receipts_sequential(&block, &child_token).await?;
        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Helper to fetch receipts sequentially (one at a time, no parallelism).
    async fn fetch_receipts_sequential(
        &self,
        block: &AnyRpcBlock,
        cancel_token: &CancellationToken,
    ) -> Result<Vec<AnyTransactionReceipt>, BlockFetchError> {
        let tx_hashes: Vec<B256> = block.transactions.hashes().collect();

        if tx_hashes.is_empty() {
            return Ok(vec![]);
        }

        let mut receipts = Vec::with_capacity(tx_hashes.len());

        for tx_hash in tx_hashes {
            let hash_str = tx_hash.to_string();
            let receipt = retry_with_cancel(
                || self.provider.get_transaction_receipt(hash_str.clone()),
                self.config.retry_interval,
                self.config.max_exponential_backoff_ms,
                cancel_token,
            )
            .await?;
            receipts.push(receipt);
        }

        Ok(receipts)
    }

    /// Collect results from parallel block and receipts fetch tasks.
    async fn collect_results(
        &self,
        fetch_id: Uuid,
        mut join_set: JoinSet<Result<FetchTaskResult, BlockFetchError>>,
        child_token: CancellationToken,
    ) -> Result<FetchedBlock, BlockFetchError> {
        let mut block: Option<AnyRpcBlock> = None;
        let mut receipts: Option<Vec<AnyTransactionReceipt>> = None;

        while let Some(result) = join_set.join_next().await {
            match result {
                Ok(Ok(FetchTaskResult::Block(b))) => {
                    block = Some(*b);
                }
                Ok(Ok(FetchTaskResult::Receipts(r))) => {
                    receipts = Some(r);
                }
                Ok(Err(e)) => {
                    child_token.cancel();
                    while join_set.join_next().await.is_some() {}
                    return Err(e);
                }
                Err(join_err) => {
                    child_token.cancel();
                    while join_set.join_next().await.is_some() {}
                    warn!(error = %join_err, "Task panicked during fetch");
                    return Err(BlockFetchError::Cancelled);
                }
            }
        }

        let block = block.ok_or(BlockFetchError::BlockNotFound)?;
        let receipts = receipts.unwrap_or_default();

        self.build_fetched_block(fetch_id, block, receipts)
    }

    /// Build a FetchedBlock from block and receipts, optionally verifying.
    fn build_fetched_block(
        &self,
        fetch_id: Uuid,
        block: AnyRpcBlock,
        receipts: Vec<AnyTransactionReceipt>,
    ) -> Result<FetchedBlock, BlockFetchError> {
        // Verify receipt count matches transaction count
        let tx_count = block.transactions.len();
        if receipts.len() != tx_count {
            return Err(BlockFetchError::ReceiptCountMismatch {
                block_tx_count: tx_count,
                receipt_count: receipts.len(),
            });
        }

        // Build receipts HashMap
        let receipts_map: HashMap<B256, AnyTransactionReceipt> = receipts
            .into_iter()
            .map(|r| (r.transaction_hash, r))
            .collect();

        // Verify all transactions have receipts
        for tx_hash in block.transactions.hashes() {
            if !receipts_map.contains_key(&tx_hash) {
                return Err(BlockFetchError::MissingReceipt { tx_hash });
            }
        }

        // Optional block verification
        if self.config.verify_block {
            let receipts_ordered: Vec<AnyTransactionReceipt> = block
                .transactions
                .hashes()
                .filter_map(|h| receipts_map.get(&h).cloned())
                .collect();

            EvmBlockComputer::verify_block(&block, &receipts_ordered)?;
        }

        Ok(FetchedBlock {
            fetch_id,
            block,
            receipts: receipts_map,
        })
    }
}

/// Internal enum for distinguishing task results.
enum FetchTaskResult {
    Block(Box<AnyRpcBlock>),
    Receipts(Vec<AnyTransactionReceipt>),
}

/// Retry an RPC operation until it succeeds, is cancelled, or hits an unrecoverable error.
///
/// # Retry strategy (via `ErrorClassification`)
/// - **Unrecoverable**: fail immediately, no retry.
/// - **RateLimited**: exponential backoff starting at 500ms, doubling each attempt,
///   capped at `max_exponential_backoff_ms`. Sequence: 500ms -> 1s -> 2s -> 4s -> ... -> max.
///   The backoff counter resets when a non-rate-limit attempt succeeds or errors differently.
/// - **Recoverable**: fixed `retry_interval` sleep, then retry.
///
/// # Cancellation
/// Checked (via `biased` select) at two points per iteration:
/// 1. Before the operation is attempted.
/// 2. During the backoff/retry sleep.
///    This guarantees prompt exit when the token is cancelled, even mid-backoff.
async fn retry_with_cancel<T, F, Fut>(
    operation: F,
    retry_interval: Duration,
    max_exponential_backoff_ms: u64,
    cancel_token: &CancellationToken,
) -> Result<T, BlockFetchError>
where
    F: Fn() -> Fut,
    Fut: Future<Output = Result<T, RpcProviderError>>,
{
    let mut rate_limit_attempt: u32 = 0;

    loop {
        tokio::select! {
            biased;
            _ = cancel_token.cancelled() => {
                return Err(BlockFetchError::Cancelled);
            }
            result = operation() => {
                match result {
                    Ok(value) => return Ok(value),
                    Err(e) => {
                        match classify_error(&e) {
                            ErrorClassification::Unrecoverable(fatal) => {
                                error!(error = %e, "Unrecoverable RPC error, not retrying");
                                return Err(fatal);
                            }
                            ErrorClassification::RateLimited => {
                                let backoff_ms = (500u64 * 2u64.saturating_pow(rate_limit_attempt))
                                    .min(max_exponential_backoff_ms);
                                rate_limit_attempt = rate_limit_attempt.saturating_add(1);
                                warn!(error = %e, backoff_ms, "Rate limited, backing off...");
                                tokio::select! {
                                    _ = cancel_token.cancelled() => return Err(BlockFetchError::Cancelled),
                                    _ = tokio::time::sleep(Duration::from_millis(backoff_ms)) => {}
                                }
                            }
                            ErrorClassification::Recoverable => {
                                rate_limit_attempt = 0;
                                warn!(error = %e, "RPC failed, retrying...");
                                tokio::select! {
                                    _ = cancel_token.cancelled() => return Err(BlockFetchError::Cancelled),
                                    _ = tokio::time::sleep(retry_interval) => {}
                                }
                            }
                        }
                    }
                }
            }
        }
    }
}

/// Classification of RPC errors for retry strategy selection.
#[derive(Debug)]
enum ErrorClassification {
    /// Fatal — stop retrying immediately.
    Unrecoverable(BlockFetchError),
    /// Transient — retry with fixed interval.
    Recoverable,
    /// Rate limited — retry with exponential backoff.
    RateLimited,
}

/// Classify RPC errors into unrecoverable, recoverable, or rate-limited.
///
/// # Unrecoverable Errors
/// - `UnsupportedMethod` - RPC node doesn't support the method (e.g., eth_getBlockReceipts)
/// - `BatchUnsupported` - RPC node doesn't support batch requests
/// - `DeserializationError` - Response format issue (unlikely to self-heal)
///
/// # Rate Limited
/// - `RateLimited` - HTTP 429 or rate limit message detected
///
/// # Recoverable Errors
/// - `TransportError` - Network issues, timeouts
/// - `BatchError` - Batch request failed (could be transient)
/// - `SerdeError` - Serialization error (transient)
/// - `SemaphoreClosed` - Internal semaphore issue
/// - `NotFound` - Block/receipt not found YET (node may not have propagated data)
fn classify_error(error: &RpcProviderError) -> ErrorClassification {
    match error {
        RpcProviderError::UnsupportedMethod { method, details } => {
            ErrorClassification::Unrecoverable(BlockFetchError::UnsupportedMethod {
                method: method.clone(),
                details: details.clone(),
            })
        }
        RpcProviderError::BatchUnsupported => {
            ErrorClassification::Unrecoverable(BlockFetchError::BatchUnsupported)
        }
        RpcProviderError::DeserializationError { method, details } => {
            ErrorClassification::Unrecoverable(BlockFetchError::DeserializationError {
                method: method.clone(),
                details: details.clone(),
            })
        }
        RpcProviderError::RateLimited(_) => ErrorClassification::RateLimited,
        // All other errors are recoverable (transient)
        _ => ErrorClassification::Recoverable,
    }
}

#[cfg(test)]
#[path = "./tests/evm_block_fetcher_tests.rs"]
mod tests;
