use std::time::{Duration, Instant};

use alloy::primitives::B256;
use thiserror::Error;
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{debug, error, info, warn};

use broker::{Broker, Publisher};

use primitives::event::{BlockFlow, CatchupPayload, ReorgBacktrackEvent};

use super::publisher::{self, PublisherError};
use crate::config::{BlockStartConfig, BlockchainConfig, PublishConfig};
use crate::store::SqlError;
use crate::{
    blockchain::evm::{
        evm_block_fetcher::{BlockFetchError, EvmBlockFetcher, FetchedBlock},
        sem_evm_rpc_provider::{RpcProviderError, SemEvmRpcProvider},
    },
    config::BlockFetcherStrategy,
    core::slot_buffer::{AsyncSlotBuffer, BufferError},
    store::models::{BlockStatus, NewDatabaseBlock, UpsertResult},
    store::repositories::Repositories,
};

/// Errors that can occur during EvmListener operations.
#[derive(Error, Debug)]
pub enum EvmListenerError {
    #[error("Could not fetch block: {source}")]
    CouldNotFetchBlock {
        #[source]
        source: BlockFetchError,
    },

    #[error("Could not compute block: {source}")]
    CouldNotComputeBlock {
        #[source]
        source: BlockFetchError,
    },

    #[error("Database error: {source}")]
    DatabaseError {
        #[source]
        source: SqlError,
    },

    #[error("Could not fetch chain height: {source}")]
    ChainHeightError {
        #[source]
        source: RpcProviderError,
    },

    #[error("Buffer error: {source}")]
    SlotBufferError {
        #[source]
        source: BufferError,
    },

    #[error("Broker publish error: {message}")]
    BrokerPublishError { message: String },

    #[error("Payload build error: {source}")]
    PayloadBuildError {
        #[source]
        source: PublisherError,
    },

    #[error("Invariant violation: {message}")]
    InvariantViolation { message: String },

    #[error("Message processing error: {message}")]
    MessageProcessingError { message: String },
}

/// Outcome of a single cursor iteration.
///
/// Reorgs are a normal operational event on blockchains (not an error),
/// so they are represented as a distinct result variant rather than an error.
#[derive(Debug)]
pub enum CursorResult {
    /// All blocks in the batch were validated and inserted as canonical.
    Complete,
    /// Chain hasn't advanced beyond the DB tip. Nothing to fetch.
    UpToDate,
    /// Reorg detected: the parent hash of the block at this number
    /// did not match the expected hash from the previous canonical block.
    ReorgDetected {
        block_number: u64,
        block_hash: B256,
        parent_hash: B256,
    },
}

#[derive(Clone)]
pub struct EvmListener {
    provider: SemEvmRpcProvider,
    repositories: Repositories,
    broker: Broker,
    event_publisher: Publisher,
    publish_config: PublishConfig,
    chain_id: u64,
    range_size: usize,
    fetcher_strategy: BlockFetcherStrategy,
    batch_receipts_size_range: Option<usize>,
    compute_block: bool,
    compute_block_allow_skipping: bool,
    loop_delay_ms: u64,
    finality_depth: u64,
    max_exponential_backoff_ms: u64,
}

impl EvmListener {
    pub fn new(
        provider: SemEvmRpcProvider,
        repositories: Repositories,
        broker: Broker,
        event_publisher: Publisher,
        blockchain_settings: &BlockchainConfig,
    ) -> Self {
        let compute_block = blockchain_settings
            .strategy
            .compute_block
            .unwrap_or_default();

        Self {
            provider,
            repositories,
            broker,
            event_publisher,
            publish_config: blockchain_settings.strategy.publish.clone(),
            chain_id: blockchain_settings.chain_id,
            range_size: blockchain_settings.strategy.range_size,
            fetcher_strategy: blockchain_settings.strategy.block_fetcher.clone(),
            batch_receipts_size_range: Some(blockchain_settings.strategy.batch_receipts_size_range),
            compute_block,
            compute_block_allow_skipping: blockchain_settings.strategy.compute_block_allow_skipping,
            loop_delay_ms: blockchain_settings.strategy.loop_delay_ms,
            finality_depth: blockchain_settings.finality_depth,
            max_exponential_backoff_ms: blockchain_settings.strategy.max_exponential_backoff_ms,
        }
    }

    /// Returns the chain ID this listener is configured for.
    pub fn chain_id(&self) -> u64 {
        self.chain_id
    }

    /// Create a fetcher with the given cancellation token and compute_block flag.
    fn create_fetcher(
        &self,
        cancel_token: CancellationToken,
        compute_block: bool,
    ) -> EvmBlockFetcher {
        EvmBlockFetcher::new(self.provider.clone())
            .with_chain_id(self.chain_id)
            .with_verify_block(compute_block)
            .with_verify_block_allow_skipping(self.compute_block_allow_skipping)
            .with_cancellation_token(cancel_token)
            .with_max_exponential_backoff_ms(self.max_exponential_backoff_ms)
    }

    /// Map a BlockFetchError to an EvmListenerError.
    ///
    /// VerificationFailed errors map to CouldNotComputeBlock,
    /// all other errors map to CouldNotFetchBlock.
    fn map_fetch_error(error: BlockFetchError) -> EvmListenerError {
        match error {
            BlockFetchError::VerificationFailed(_) => {
                EvmListenerError::CouldNotComputeBlock { source: error }
            }
            _ => EvmListenerError::CouldNotFetchBlock { source: error },
        }
    }

    /// Fetch a block by its number using the configured strategy.
    ///
    /// # Arguments
    /// * `block_number` - The block number to fetch
    /// * `cancel_token` - Cancellation token for this operation (share across batch for batch cancellation)
    /// * `compute_block` - Whether to verify block integrity (transaction root, receipt root, block hash)
    ///
    /// # Cancellation
    /// When `cancel_token.cancel()` is called, this operation will return `BlockFetchError::Cancelled`.
    /// All parallel operations sharing the same token will be cancelled together.
    pub async fn get_block_by_number(
        &self,
        block_number: u64,
        cancel_token: CancellationToken,
        compute_block: bool,
    ) -> Result<FetchedBlock, EvmListenerError> {
        let fetcher = self.create_fetcher(cancel_token, compute_block);

        let result = match self.fetcher_strategy {
            BlockFetcherStrategy::BlockReceipts => {
                fetcher
                    .fetch_block_with_block_receipts_by_number(block_number)
                    .await
            }
            BlockFetcherStrategy::BatchReceiptsFull => {
                fetcher
                    .fetch_block_with_batch_receipts_by_number(block_number)
                    .await
            }
            BlockFetcherStrategy::BatchReceiptsRange => {
                let batch_size = self.batch_receipts_size_range.unwrap_or(1);
                fetcher
                    .fetch_block_by_number_with_parallel_batched_receipts(block_number, batch_size)
                    .await
            }
            BlockFetcherStrategy::TransactionReceiptsParallel => {
                fetcher
                    .fetch_block_with_individual_receipts_by_number(block_number)
                    .await
            }
            BlockFetcherStrategy::TransactionReceiptsSequential => {
                fetcher
                    .fetch_block_with_sequential_receipts_by_number(block_number)
                    .await
            }
        };

        result.map_err(Self::map_fetch_error)
    }

    /// Fetch a block by its hash using the configured strategy.
    ///
    /// # Arguments
    /// * `hash` - The block hash to fetch
    /// * `cancel_token` - Cancellation token for this operation
    /// * `compute_block` - Whether to verify block integrity (transaction root, receipt root, block hash)
    ///
    /// # Cancellation
    /// When `cancel_token.cancel()` is called, this operation will return `BlockFetchError::Cancelled`.
    pub async fn get_block_by_hash(
        &self,
        hash: B256,
        cancel_token: CancellationToken,
        compute_block: bool,
    ) -> Result<FetchedBlock, EvmListenerError> {
        let fetcher = self.create_fetcher(cancel_token, compute_block);

        let result = match self.fetcher_strategy {
            BlockFetcherStrategy::BlockReceipts => {
                fetcher.fetch_block_with_block_receipts_by_hash(hash).await
            }
            BlockFetcherStrategy::BatchReceiptsFull => {
                fetcher.fetch_block_with_batch_receipts_by_hash(hash).await
            }
            BlockFetcherStrategy::BatchReceiptsRange => {
                let batch_size = self.batch_receipts_size_range.unwrap_or(1);
                fetcher
                    .fetch_block_by_hash_with_parallel_batched_receipts(hash, batch_size)
                    .await
            }
            BlockFetcherStrategy::TransactionReceiptsParallel => {
                fetcher
                    .fetch_block_with_individual_receipts_by_hash(hash)
                    .await
            }
            BlockFetcherStrategy::TransactionReceiptsSequential => {
                fetcher
                    .fetch_block_with_sequential_receipts_by_hash(hash)
                    .await
            }
        };

        result.map_err(Self::map_fetch_error)
    }

    /// Validates the block fetching strategy and initializes the database with a starting block.
    ///
    /// This function is called during service initialization and will panic on any failure.
    /// The rationale is that if initialization fails, the process cannot operate correctly
    /// and should crash to trigger a restart (crash-loop-backoff pattern).
    ///
    /// # Panics
    /// - If the RPC node is unreachable or returns an error
    /// - If the configured strategy is not compatible with the RPC node
    /// - If the database is unreachable or returns an error
    /// - If the starting block cannot be fetched or inserted
    pub async fn validate_strategy_and_init_block(&self, block_start_config: BlockStartConfig) {
        // Fetch latest block number from blockchain using SemEvmRpcProvider
        let latest_block_number = match self.provider.get_block_number().await {
            Ok(block_number) => block_number,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    "CRITICAL: Could not fetch latest block number from RPC"
                );
                panic!("Could not fetch latest block number from RPC: {}", e);
            }
        };

        tracing::info!(
            block_number = latest_block_number,
            "Fetched latest block number from blockchain"
        );

        // Resolve block_start_on_first_start config to a concrete block number.
        // "current" resolves to height - 1 for reorg safety at the first block.
        let block_start: u64 = match block_start_config {
            BlockStartConfig::Number(n) => n,
            BlockStartConfig::Current => {
                let resolved = latest_block_number.saturating_sub(1);
                tracing::info!(
                    latest_block_number,
                    resolved_block_start = resolved,
                    "Resolved block_start_on_first_start='current' to height - 1"
                );
                resolved
            }
        };

        // Validate strategy by attempting to fetch the latest block
        let cancel_token = CancellationToken::new();
        match self
            .get_block_by_number(latest_block_number, cancel_token, self.compute_block)
            .await
        {
            Ok(fetched_block) => {
                tracing::info!(
                    block_number = latest_block_number,
                    block_hash = %fetched_block.block.header.hash,
                    tx_count = fetched_block.transaction_count(),
                    strategy = ?self.fetcher_strategy,
                    "Strategy validation successful"
                );
            }
            Err(e) => {
                tracing::error!(
                    error = %e,
                    strategy = ?self.fetcher_strategy,
                    "CRITICAL: Strategy validation failed - binary should not start"
                );
                panic!(
                    "Strategy validation failed: {}. The configured strategy {:?} is not compatible with the RPC node.",
                    e, self.fetcher_strategy
                );
            }
        }

        // Check for existing canonical block in the database
        let latest_db_block = match self.repositories.blocks.get_latest_canonical_block().await {
            Ok(block) => block,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    "CRITICAL: Could not query database for latest canonical block"
                );
                panic!("Could not query database for latest canonical block: {}", e);
            }
        };

        if let Some(block) = latest_db_block {
            tracing::info!(
                block_number = block.block_number,
                block_hash = %block.block_hash,
                "Found existing canonical block in database, no initialization needed"
            );
            return;
        }

        tracing::info!(
            block_start = block_start,
            "No canonical block in database, initializing with block_start"
        );

        // Fetch the starting block from the blockchain
        let cancel_token = CancellationToken::new();
        let fetched_block = match self
            .get_block_by_number(block_start, cancel_token, self.compute_block)
            .await
        {
            Ok(block) => block,
            Err(e) => {
                tracing::error!(
                    error = %e,
                    block_start = block_start,
                    "CRITICAL: Could not fetch starting block from blockchain"
                );
                panic!(
                    "Could not fetch starting block {} from blockchain: {}",
                    block_start, e
                );
            }
        };

        // Convert FetchedBlock to NewDatabaseBlock and insert into database
        let new_db_block =
            NewDatabaseBlock::from_rpc_block(&fetched_block.block, BlockStatus::Canonical);

        if let Err(e) = self.repositories.blocks.insert_block(&new_db_block).await {
            if e.is_unique_violation() {
                tracing::info!(
                    block_number = new_db_block.block_number,
                    block_hash = %new_db_block.block_hash,
                    "Starting block already exists in database, skipping insert"
                );
            } else {
                tracing::error!(
                    error = %e,
                    block_number = new_db_block.block_number,
                    block_hash = %new_db_block.block_hash,
                    "CRITICAL: Could not insert starting block into database"
                );
                panic!("Could not insert starting block into database: {}", e);
            }
        } else {
            tracing::info!(
                block_number = new_db_block.block_number,
                block_hash = %new_db_block.block_hash,
                "Successfully initialized database with starting block"
            );
        }
    }

    /// Orchestrates one iteration of the V2 cursor algorithm.
    ///
    /// This is the main entry point for block fetching. It:
    /// 1. Reads the DB tip (latest canonical block)
    /// 2. Gets the current chain height from the RPC node
    /// 3. Calculates the range of blocks to fetch (capped by `batch_size`)
    /// 4. Spawns two concurrent tasks via `tokio::spawn`:
    ///    - **Producer** (`fetch_blocks_in_parallel`): fetches blocks via parallel RPC calls,
    ///      fills an `AsyncSlotBuffer` out-of-order
    ///    - **Consumer** (`cursor_processing`): reads slots sequentially, validates the
    ///      parent_hash chain, inserts validated blocks into the DB
    /// 5. Awaits both tasks and analyzes outcomes
    ///
    /// # Returns
    /// - `Ok(CursorResult::Complete)` — all blocks validated and inserted; sleeps `loop_delay_ms` before returning
    /// - `Ok(CursorResult::UpToDate)` — chain hasn't advanced, nothing to fetch
    /// - `Ok(CursorResult::ReorgDetected { block_number })` — reorg detected at this height (no sleep)
    /// - `Err(...)` — unrecoverable or transient error (no sleep, propagate fast for retry logic)
    ///
    /// # Cancellation
    /// A fresh `CancellationToken` is created per iteration. It is shared between the producer
    /// and consumer. Either side can cancel the other:
    /// - Cursor cancels on reorg detection or DB failure
    /// - Fetcher cancellation propagates to cursor via the shared token
    ///
    /// # Panics
    /// This function does not panic. Task panics are caught via `JoinError` and converted
    /// to `EvmListenerError::InvariantViolation`.
    pub async fn fetch_blocks_and_run_cursor(&self) -> Result<CursorResult, EvmListenerError> {
        metrics::counter!(
            "listener_cursor_iterations_total",
            "chain_id" => self.chain_id.to_string()
        )
        .increment(1);

        // We don't need to dedup the message, the flow lock + ack will handle the deduplication by itself.

        // Step 1: Get the latest canonical block from DB (the "tip" of our chain view).
        // validate_strategy_and_init_block guarantees at least one block exists.
        let db_block = self
            .repositories
            .blocks
            .get_latest_canonical_block()
            .await
            .map_err(|e| EvmListenerError::DatabaseError { source: e })?
            .ok_or_else(|| EvmListenerError::InvariantViolation {
                message: "No canonical block in database. \
                          validate_strategy_and_init_block must run before the cursor."
                    .to_string(),
            })?;

        let db_block_number = db_block.block_number;
        let db_block_hash = db_block.block_hash;

        metrics::gauge!(
            "listener_db_tip_block_number",
            "chain_id" => self.chain_id.to_string()
        )
        .set(db_block_number as f64);

        debug!(
            db_block_number = db_block_number,
            db_block_hash = %db_block_hash,
            "Retrieved DB tip for cursor iteration"
        );

        // Step 2: Get the current chain height from the RPC node
        let chain_height = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| EvmListenerError::ChainHeightError { source: e })?;

        metrics::gauge!(
            "listener_chain_height_block_number",
            "chain_id" => self.chain_id.to_string()
        )
        .set(chain_height as f64);

        // Step 3: If the chain hasn't advanced beyond our DB tip, there's nothing to do
        if chain_height <= db_block_number {
            debug!(
                chain_height = chain_height,
                db_block_number = db_block_number,
                "Chain has not advanced beyond DB tip"
            );
            tokio::time::sleep(Duration::from_millis(self.loop_delay_ms)).await;
            return Ok(CursorResult::UpToDate);
        }

        // Step 4: Calculate the inclusive range [range_start, range_end]
        // Example: db=100, batch=10, chain=500 -> start=101, end=110, length=10 (blocks 101..=110)
        // Example: db=100, batch=10, chain=103 -> start=101, end=103, length=3 (only 3 available)
        let range_start = db_block_number + 1;
        let range_end = std::cmp::min(chain_height, db_block_number + self.range_size as u64);
        let range_length = (range_end - range_start + 1) as usize; // +1 because both bounds are inclusive

        info!(
            range_start = range_start,
            range_end = range_end,
            range_length = range_length,
            chain_height = chain_height,
            "Starting cursor iteration"
        );

        // Step 5: Create shared state for producer-consumer coordination
        let range_start_time = Instant::now();
        let buffer = AsyncSlotBuffer::<FetchedBlock>::new(range_length);
        let cancel_token = CancellationToken::new();

        // Step 6: Spawn both tasks concurrently
        // - cursor_processing (consumer): reads slots sequentially, validates hash chain, inserts to DB
        // - fetch_blocks_in_parallel (producer): fetches blocks via parallel RPC calls, fills slots
        let cursor_handle = tokio::spawn(cursor_processing(
            self.clone(),
            buffer.clone(),
            cancel_token.clone(),
            db_block_hash,
            range_start,
            range_length,
        ));

        let fetcher_handle = tokio::spawn(fetch_blocks_in_parallel(
            self.clone(),
            buffer,
            cancel_token.clone(),
            range_start,
            range_length,
        ));

        // Step 7: Await both tasks to completion.
        // tokio::join! ensures neither task is abandoned even if the other completes/fails first.
        // On cancellation, in-flight HTTP requests may take up to 10s (RPC timeout) before
        // tasks finish. This is acceptable.
        let (cursor_join_result, fetcher_join_result) = tokio::join!(cursor_handle, fetcher_handle);

        metrics::histogram!(
            "listener_range_fetch_duration_seconds",
            "chain_id" => self.chain_id.to_string()
        )
        .record(range_start_time.elapsed().as_secs_f64());

        // Step 8: Unwrap JoinHandle results — a JoinError means the task panicked,
        // which is a critical bug (we never panic in our code).
        let cursor_outcome = cursor_join_result.map_err(|join_err| {
            cancel_token.cancel();
            error!(error = %join_err, "Cursor task panicked — this is a critical bug");
            EvmListenerError::InvariantViolation {
                message: format!("Cursor task panicked: {}", join_err),
            }
        })?;

        let fetcher_outcome = fetcher_join_result.map_err(|join_err| {
            cancel_token.cancel();
            error!(error = %join_err, "Fetcher task panicked — this is a critical bug");
            EvmListenerError::InvariantViolation {
                message: format!("Fetcher task panicked: {}", join_err),
            }
        })?;

        // NOTE: redo this, with clear error path and skips (Due to message broker mostly)
        // Step 9: Analyze outcomes — cursor takes priority since it's the authority
        // on what actually made it into the DB.
        // Arms are ordered: reorg first (no sleep), then success (sleep), then errors (no sleep).
        match (cursor_outcome, fetcher_outcome) {
            // === REORG PATH ===
            // Fetcher error (Cancelled) is expected here since cursor cancelled the token.
            // No sleep — reorg should be handled immediately.
            (
                Ok(CursorResult::ReorgDetected {
                    block_number,
                    block_hash,
                    parent_hash,
                }),
                _,
            ) => {
                info!(
                    block_number = block_number,
                    block_hash = %block_hash,
                    "Reorg detected"
                );

                metrics::counter!(
                    "listener_reorgs_total",
                    "chain_id" => self.chain_id.to_string()
                )
                .increment(1);

                Ok(CursorResult::ReorgDetected {
                    block_number,
                    block_hash,
                    parent_hash,
                })
            }

            // === SUCCESS PATH ===
            // All blocks validated and inserted. Sleep before returning to avoid hammering RPC.
            // Also handles the theoretically unreachable CursorResult::UpToDate from cursor_processing
            // (cursor_processing only returns Complete or ReorgDetected, but Rust requires exhaustive match).
            (Ok(cursor_result), Ok(())) => {
                info!(
                    range_start = range_start,
                    range_end = range_end,
                    "Cursor iteration complete — all blocks validated and inserted"
                );
                tokio::time::sleep(Duration::from_millis(self.loop_delay_ms)).await;
                Ok(cursor_result)
            }

            // === ERROR PATHS ===

            // Cursor was cancelled because fetcher failed — return the ROOT CAUSE (fetcher's error).
            // The cursor saw cancellation via tokio::select!, but the real problem is in the fetcher.
            (
                Err(EvmListenerError::CouldNotFetchBlock {
                    source: BlockFetchError::Cancelled,
                }),
                Err(fetcher_err),
            ) => {
                // TODO(retry): caller decides retry strategy:
                //   - Unrecoverable (UnsupportedMethod, Deserialization): escalate/alert, do NOT retry blindly
                //   - Recoverable (transport, timeout): retry the whole iteration.

                // TODO: Emit event for next loop iteration.
                error!(error = %fetcher_err, "Fetcher error caused cursor cancellation");
                Err(fetcher_err)
            }

            // Cursor had a real error (DB failure, invariant violation, etc.) — return it.
            // No sleep — propagate fast for retry logic.
            (Err(cursor_err), _) => {
                // TODO(retry): caller decides retry strategy:
                //   - DatabaseError: may retry after delay (DB might recover)
                //   - InvariantViolation: critical, needs investigation
                error!(error = %cursor_err, "Cursor error during iteration");
                Err(cursor_err)
            }

            // Cursor succeeded but fetcher errored — unexpected but cursor's result is authoritative.
            // This shouldn't happen: if cursor completed, all slots were filled successfully.
            (Ok(result), Err(fetcher_err)) => {
                warn!(
                    error = %fetcher_err,
                    "Fetcher errored despite cursor succeeding — unexpected"
                );
                Ok(result)
            }
        }
    }

    /// Walks backwards from the reorg point to reconstruct the canonical chain.
    ///
    /// # Algorithm
    ///
    /// ## Phase 1 — Walk + Publish (DB read-only)
    ///   1a. Fetch block N by `event.block_hash` from RPC. Publish block N's
    ///       events with `BlockFlow::Reorged`. Collect `NewDatabaseBlock` metadata.
    ///       (v1 never published block N's events — this closes the R2 gap.)
    ///   1b. Walk backwards from N-1: for each height, fetch by parent_hash,
    ///       publish events immediately (on-the-go), collect lightweight metadata.
    ///       The `FetchedBlock` is dropped after publishing — only `NewDatabaseBlock`
    ///       (72 bytes) is retained. DB is NEVER modified.
    ///       Stop when fork-point found, genesis reached, or indexing boundary +
    ///       finality_depth exhausted.
    ///
    /// ## Phase 2 — Commit (single DB transaction)
    ///   Reverse collected blocks to ascending order. Batch-upsert all blocks
    ///   via `batch_upsert_blocks_canonical`. On error: transaction rolled back,
    ///   DB untouched, broker retries the message → clean restart.
    ///
    /// ## Phase 3 — Resume
    ///   Publish `FETCH_NEW_BLOCKS` to resume the cursor.
    ///
    /// # Crash safety
    ///
    /// - Crash during Phase 1: DB untouched → retry re-walks from scratch,
    ///   re-publishes (at-least-once). No false fork-point possible.
    /// - Crash during Phase 2: transaction rolled back → DB untouched → same.
    /// - Crash after Phase 2 commit: DB correct → retry finds DB already updated,
    ///   walk terminates in ~1 block, re-publishes ~1 event (at-least-once).
    ///
    /// # Reorg depth definition
    ///
    /// `reorg_depth` = number of blocks replaced, INCLUDING block N.
    /// Example: reorg at height 100 with fork-point at 97 → depth = 3
    /// (blocks 100, 99, 98 replaced; block 97 is the common ancestor).
    pub async fn reorg_backtrack(
        &self,
        event: ReorgBacktrackEvent,
    ) -> Result<(), EvmListenerError> {
        let block_number = event.block_number;
        let chain_id_u64 = self.repositories.chain_id() as u64;

        // We don't need to dedup the message, the flow lock + ack will handle the deduplication by itself.

        let mut collected_blocks: Vec<NewDatabaseBlock> = Vec::new();

        info!(
            block_number,
            block_hash = %event.block_hash,
            parent_hash = %event.parent_hash,
            "reorg_backtrack_v2: starting (DB read-only during walk)"
        );

        // ══════════════════════════════════════════════════════════════
        // Phase 1a: Fetch block N by hash, publish its events.
        // Comes from the live flow, marked as live block.
        // ══════════════════════════════════════════════════════════════

        let cancel_token = CancellationToken::new();
        let block_n = self
            .get_block_by_hash(event.block_hash, cancel_token, self.compute_block)
            .await?;

        publisher::publish_block_events(
            &self.repositories,
            &block_n,
            chain_id_u64,
            // This block is coming from the live flow, and detect the reorg processing, but was issued from the live flow.
            BlockFlow::Live,
            &self.broker,
            &self.event_publisher,
            &self.publish_config,
        )
        .await
        .map_err(|source| EvmListenerError::PayloadBuildError { source })?;

        collected_blocks.push(NewDatabaseBlock::from_rpc_block(
            &block_n.block,
            BlockStatus::Canonical,
        ));

        info!(
            block_number,
            block_hash = %event.block_hash,
            "reorg_backtrack_v2: block N published, walking backwards"
        );

        // ══════════════════════════════════════════════════════════════
        // Phase 1b: Walk backwards from N-1.
        //
        // For each block: fetch by parent hash → publish events → collect
        // lightweight metadata (NewDatabaseBlock, 72 bytes). Then check
        // fork-point against DB canonical block at prev_height.
        //
        // DB is NEVER modified. If the process crashes here, the DB is
        // in its original state and retry starts from scratch.
        // ══════════════════════════════════════════════════════════════

        let mut current_block = block_n;
        let mut current_height = block_number;
        let mut steps_past_db: u64 = 0;

        loop {
            // Genesis guard — cannot compare parent hash below block 0.
            if current_height == 0 {
                warn!("reorg_backtrack_v2: reached genesis block, cannot walk further back");
                break;
            }

            let prev_height = current_height - 1;

            // Fork-point detection: read-only DB check.
            // Since the DB is NEVER modified during the walk, this comparison is
            // ALWAYS against the original state. No false positives possible.
            let db_prev_block = self
                .repositories
                .blocks
                .get_canonical_block_by_number(prev_height)
                .await
                .map_err(|e| EvmListenerError::DatabaseError { source: e })?;

            match db_prev_block {
                Some(prev_block)
                    if current_block.block.header.parent_hash == prev_block.block_hash =>
                {
                    // Fork-point found — DB block below matches our chain.
                    info!(
                        fork_point = prev_height,
                        reorg_depth = collected_blocks.len(),
                        "reorg_backtrack_v2: fork-point found"
                    );
                    break;
                }
                Some(prev_block) => {
                    // Mismatch — reorg goes deeper.
                    info!(
                        height = prev_height,
                        db_hash = %prev_block.block_hash,
                        new_parent = %current_block.block.header.parent_hash,
                        "reorg_backtrack_v2: parent mismatch, walking back"
                    );
                }
                None => {
                    // No DB block at previous height — indexing boundary.
                    steps_past_db += 1;
                    // We are passing db blocks only if we are under finality, with already collected blocks.
                    // Indeed passing collected_blocks will also considers the matching blocks into the database.
                    if collected_blocks.len() as u64 >= self.finality_depth {
                        warn!(
                            steps_past_db,
                            finality_depth = self.finality_depth,
                            prev_height,
                            "reorg_backtrack_v2: finality limit reached past DB boundary, stopping"
                        );
                        break;
                    }
                    info!(
                        prev_height,
                        steps_past_db,
                        finality_depth = self.finality_depth,
                        "reorg_backtrack_v2: no DB block at prev height, continuing"
                    );
                }
            }

            // Fetch parent block by hash from RPC.
            let cancel_token = CancellationToken::new();
            current_block = self
                .get_block_by_hash(
                    current_block.block.header.parent_hash,
                    cancel_token,
                    self.compute_block,
                )
                .await?;
            current_height = prev_height;

            // Publish events on-the-go. The FetchedBlock is dropped after this
            // loop iteration — only the NewDatabaseBlock (72 bytes) is retained.
            publisher::publish_block_events(
                &self.repositories,
                &current_block,
                chain_id_u64,
                BlockFlow::Reorged,
                &self.broker,
                &self.event_publisher,
                &self.publish_config,
            )
            .await
            .map_err(|source| EvmListenerError::PayloadBuildError { source })?;

            collected_blocks.push(NewDatabaseBlock::from_rpc_block(
                &current_block.block,
                BlockStatus::Canonical,
            ));

            info!(
                block_number = current_height,
                block_hash = %current_block.block.header.hash,
                blocks_collected = collected_blocks.len(),
                "reorg_backtrack_v2: published events, collected metadata"
            );
        }

        // ══════════════════════════════════════════════════════════════
        // Phase 2: Batch commit — single DB transaction.
        //
        // Reverse to ascending order (fork+1, ..., N-1, N) and upsert all
        // blocks atomically. If the transaction fails, the entire batch is
        // rolled back and the DB remains in its original state.
        //
        // All events have already been published (Phase 1). This satisfies
        // R4: publish-before-commit for every block.
        // ══════════════════════════════════════════════════════════════

        // collected_blocks is in descending order [N, N-1, ..., fork+1].
        // Reverse to ascending for conventional batch upsert ordering.
        collected_blocks.reverse();

        let reorg_depth = collected_blocks.len() as u64;
        info!(
            reorg_depth,
            "reorg_backtrack_v2: walk complete, committing batch to DB"
        );

        let upsert_results = self
            .repositories
            .blocks
            .batch_upsert_blocks_canonical(&collected_blocks)
            .await
            .map_err(|e| EvmListenerError::DatabaseError { source: e })?;

        let inserted_count = upsert_results
            .iter()
            .filter(|r| **r == UpsertResult::Inserted)
            .count();
        let updated_count = upsert_results
            .iter()
            .filter(|r| **r == UpsertResult::Updated)
            .count();
        let noop_count = upsert_results
            .iter()
            .filter(|r| **r == UpsertResult::NoOp)
            .count();
        let known_branch = updated_count > 0 || noop_count > 0;

        info!(
            block_number,
            reorg_depth,
            inserted_count,
            updated_count,
            noop_count,
            known_branch,
            "reorg_backtrack_v2: batch commit complete. {}",
            if known_branch {
                "Some blocks were from a previously known branch."
            } else {
                "All blocks were new to this node."
            }
        );

        // Phase 3 (publish FETCH_NEW_BLOCKS) moved to ReorgHandler — after lock release.

        Ok(())
    }

    /// Replay a historical block range for a single consumer.
    ///
    /// Catchup primitive: given a [`CatchupPayload`], fetch blocks
    /// `[block_start, block_end]` in parallel and publish them in order on
    /// the `catchup-event` queue for `consumer_id`.
    ///
    /// # Behavior
    /// - Clamps `block_end` to the current chain height (`eth_blockNumber`).
    /// - If `block_start` is above chain height → no-op `Ok(())`. The user
    ///   asked for blocks that don't exist yet; catchup is a bounded one-shot,
    ///   **not** a continuous tail. The caller can re-issue the request later.
    /// - No DB writes, no parent-hash validation, no reorg handling. Catchup
    ///   is designed for replay of blocks the user knows are historical
    ///   (typically below finality).
    /// - **Upper bound is the raw chain head (no finality margin).** Catchups
    ///   *within* the unfinalized window are permitted: the caller takes the
    ///   reorg risk on those blocks. The live cursor remains the source of
    ///   truth for the canonical view; catchup is purely for replay/recovery.
    /// - No advisory lock by design. Catchup is idempotent (at-least-once
    ///   delivery, downstream consumer dedupes by block_number + block_hash).
    ///   Two pods replaying the same range in parallel is wasteful but not
    ///   incorrect.
    ///
    /// # Parallelism
    /// Mirrors the live cursor exactly — same [`AsyncSlotBuffer`] +
    /// [`fetch_blocks_in_parallel`] producer + sequential consumer pattern.
    /// The producer fills slots out-of-order; the consumer publishes in order.
    pub async fn run_range_catchup(
        &self,
        payload: CatchupPayload,
    ) -> Result<(), EvmListenerError> {
        metrics::counter!(
            "listener_catchup_iterations_total",
            "chain_id" => self.chain_id.to_string()
        )
        .increment(1);

        // Step 1 — chain height.
        let chain_height = self
            .provider
            .get_block_number()
            .await
            .map_err(|e| EvmListenerError::ChainHeightError { source: e })?;

        // Step 2 — start above head: skip turn (the requested range doesn't
        // exist yet on chain). Bounded one-shot — no auto re-publish.
        if payload.block_start > chain_height {
            metrics::counter!(
                "listener_catchup_skipped_above_head_total",
                "chain_id" => self.chain_id.to_string()
            )
            .increment(1);
            info!(
                consumer_id = %payload.consumer_id,
                block_start = payload.block_start,
                block_end = payload.block_end,
                chain_height,
                "Catchup skipped: block_start above chain height"
            );
            return Ok(());
        }

        // Step 2b — clamp end down to head if the user asked beyond it.
        let range_end = std::cmp::min(payload.block_end, chain_height);
        let range_start = payload.block_start;
        let range_length = (range_end - range_start + 1) as usize;

        info!(
            consumer_id = %payload.consumer_id,
            range_start,
            range_end,
            range_length,
            chain_height,
            "Starting catchup range"
        );

        // Step 3 — shared producer/consumer state.
        let range_start_time = Instant::now();
        let buffer = AsyncSlotBuffer::<FetchedBlock>::new(range_length);
        let cancel_token = CancellationToken::new();

        // Step 4 — spawn the same producer used by the live cursor: pure fetch,
        // no DB or hash work — perfectly reusable.
        let fetcher_handle = tokio::spawn(fetch_blocks_in_parallel(
            self.clone(),
            buffer.clone(),
            cancel_token.clone(),
            range_start,
            range_length,
        ));

        // Step 5 — spawn the catchup consumer (in-order publish, no DB, no hashing).
        let catchup_handle = tokio::spawn(catchup_processing(
            self.clone(),
            buffer,
            cancel_token.clone(),
            range_start,
            range_length,
            payload.consumer_id.clone(),
        ));

        // Step 6 — join, classify like fetch_blocks_and_run_cursor.
        let (catchup_join, fetcher_join) = tokio::join!(catchup_handle, fetcher_handle);

        metrics::histogram!(
            "listener_catchup_range_duration_seconds",
            "chain_id" => self.chain_id.to_string()
        )
        .record(range_start_time.elapsed().as_secs_f64());

        let catchup_outcome = catchup_join.map_err(|join_err| {
            cancel_token.cancel();
            error!(error = %join_err, "Catchup consumer task panicked — this is a critical bug");
            EvmListenerError::InvariantViolation {
                message: format!("Catchup consumer panicked: {}", join_err),
            }
        })?;

        let fetcher_outcome = fetcher_join.map_err(|join_err| {
            cancel_token.cancel();
            error!(error = %join_err, "Catchup fetcher task panicked — this is a critical bug");
            EvmListenerError::InvariantViolation {
                message: format!("Catchup fetcher panicked: {}", join_err),
            }
        })?;

        // Same priority logic as the live path, simplified (no reorg branch):
        //  - both Ok → success
        //  - consumer cancelled by fetcher → return fetcher's root cause
        //  - consumer real error → return it
        //  - consumer Ok, fetcher Err → unexpected, log and treat as success
        //    (consumer is authoritative; if all slots were read, all blocks were published)
        match (catchup_outcome, fetcher_outcome) {
            (Ok(()), Ok(())) => {
                info!(
                    range_start,
                    range_end,
                    "Catchup range complete"
                );
                Ok(())
            }
            (
                Err(EvmListenerError::CouldNotFetchBlock {
                    source: BlockFetchError::Cancelled,
                }),
                Err(fetcher_err),
            ) => {
                error!(error = %fetcher_err, "Catchup fetcher error caused consumer cancellation");
                Err(fetcher_err)
            }
            (Err(consumer_err), _) => {
                error!(error = %consumer_err, "Catchup consumer error during iteration");
                Err(consumer_err)
            }
            (Ok(()), Err(fetcher_err)) => {
                warn!(
                    error = %fetcher_err,
                    "Catchup fetcher errored despite consumer succeeding — unexpected"
                );
                Ok(())
            }
        }
    }
}

/// Sequential block validator and DB inserter (the "consumer" in the producer-consumer pattern).
///
/// Reads blocks from the `AsyncSlotBuffer` in order (slot 0, 1, 2, ...), validates the
/// parent_hash chain against the previous block's hash, and inserts validated blocks into
/// the database as CANONICAL.
///
/// # Parameters
/// - `listener`: Cloned `EvmListener` (owns repositories for DB access). Passed by value
///   because this runs in `tokio::spawn` which requires `'static`.
/// - `buffer`: Shared slot buffer where the producer writes fetched blocks.
/// - `cancel_token`: Shared cancellation token. Cursor checks this before each slot read
///   (via `tokio::select!`) and cancels it on reorg detection or DB failure.
/// - `expected_parent_hash`: The hash of the DB tip block. The first fetched block's
///   `parent_hash` must match this value.
/// - `range_start`: The block number of slot 0 in the buffer.
/// - `range_length`: Total number of slots to process.
///
/// # Returns
/// - `Ok(CursorResult::Complete)` — all blocks validated and inserted
/// - `Ok(CursorResult::ReorgDetected { block_number })` — parent hash mismatch detected
/// - `Err(...)` — DB failure, buffer error, or cancellation from the fetcher side
///
/// # Cancellation Safety
/// `buffer.get()` is cancel-safe: if `tokio::select!` drops it while awaiting `Mutex::lock`,
/// the guard drops correctly. If dropped during `Notify::notified()`, the waiter is deregistered.
async fn cursor_processing(
    listener: EvmListener,
    buffer: AsyncSlotBuffer<FetchedBlock>,
    cancel_token: CancellationToken,
    expected_parent_hash: B256,
    range_start: u64,
    range_length: usize,
) -> Result<CursorResult, EvmListenerError> {
    let mut current_expected_hash = expected_parent_hash;

    for i in 0..range_length {
        let block_number = range_start + i as u64;

        // Wait for the block with cancellation guard.
        // buffer.get() blocks forever if the slot is never filled (e.g., fetcher cancelled),
        // so we race it against the cancellation token.
        // `biased` ensures cancellation is always checked first to prevent processing
        // stale data after cancellation is signaled.
        let fetched_block = tokio::select! {
            biased;
            _ = cancel_token.cancelled() => {
                return Err(EvmListenerError::CouldNotFetchBlock {
                    source: BlockFetchError::Cancelled,
                });
            }
            block_opt = buffer.get(i) => {
                block_opt.ok_or(EvmListenerError::SlotBufferError {
                    source: BufferError::IndexOutOfBounds,
                })?
            }
        };

        let block_hash = fetched_block.block.header.hash;
        let parent_hash = fetched_block.block.header.parent_hash;

        // Validate the parent hash chain: this block's parent must be the previous block's hash.
        // For slot 0, current_expected_hash is the DB tip's hash.
        // For subsequent slots, it's the hash of the block we just inserted.
        if parent_hash != current_expected_hash {
            // REORG DETECTED: the chain has diverged from our canonical view.
            // Cancel the fetcher to stop wasting RPC calls on a now-invalid range.
            cancel_token.cancel();
            warn!(
                block_number = block_number,
                expected_parent = %current_expected_hash,
                actual_parent = %parent_hash,
                block_hash = %block_hash,
                slot = i,
                "Reorg detected: parent hash mismatch"
            );
            return Ok(CursorResult::ReorgDetected {
                block_number,
                block_hash,
                parent_hash,
            });
        }

        // Hash chain is valid — publish events BEFORE inserting block.
        // Events MUST be delivered before the block is registered in DB.
        // If publish fails: block is NOT inserted, DB tip unchanged,
        // cursor retries this exact block on next iteration. Zero missed events.
        let chain_id_u64 = listener.repositories.chain_id() as u64;
        publisher::publish_block_events(
            &listener.repositories,
            &fetched_block,
            chain_id_u64,
            BlockFlow::Live,
            &listener.broker,
            &listener.event_publisher,
            &listener.publish_config,
        )
        .await
        .map_err(|source| {
            // Stop fetcher on publish failure — no point fetching more blocks
            cancel_token.cancel();
            EvmListenerError::PayloadBuildError { source }
        })?;

        // Safe to insert now — events have been delivered to all consumers.
        let new_db_block =
            NewDatabaseBlock::from_rpc_block(&fetched_block.block, BlockStatus::Canonical);

        listener
            .repositories
            .blocks
            .insert_block(&new_db_block)
            .await
            .map_err(|e| {
                // Stop fetcher on DB failure — no point fetching more blocks if we can't store them
                cancel_token.cancel();
                EvmListenerError::DatabaseError { source: e }
            })?;

        info!(
            block_number = block_number,
            block_hash = %block_hash,
            tx_count = fetched_block.transaction_count(),
            slot = i + 1,
            total = range_length,
            "Block validated and inserted as canonical"
        );

        // Update expected hash for next iteration: the NEXT block's parent must be THIS block's hash
        current_expected_hash = block_hash;
    }

    Ok(CursorResult::Complete)
}

/// Sequential publisher for the catchup pipeline (the "consumer" sibling of
/// [`cursor_processing`], stripped down for replay).
///
/// Reads blocks from the [`AsyncSlotBuffer`] in order (slot 0, 1, 2, …) and
/// publishes each one to `{consumer_id}.catchup-event` via
/// [`publisher::publish_catchup_block_events`]. **No** parent-hash validation,
/// **no** DB writes, **no** reorg branch — this is pure replay.
///
/// On publish failure, cancels the producer (no point fetching more blocks if
/// we can't deliver them) and propagates the error so the handler can retry
/// the entire range.
///
/// # Cancellation Safety
/// `buffer.get()` is cancel-safe; the `tokio::select! { biased; … }` checks
/// the cancel token first to avoid processing stale data after cancellation.
async fn catchup_processing(
    listener: EvmListener,
    buffer: AsyncSlotBuffer<FetchedBlock>,
    cancel_token: CancellationToken,
    range_start: u64,
    range_length: usize,
    consumer_id: String,
) -> Result<(), EvmListenerError> {
    let chain_id_u64 = listener.repositories.chain_id() as u64;

    for i in 0..range_length {
        let block_number = range_start + i as u64;

        let fetched_block = tokio::select! {
            biased;
            _ = cancel_token.cancelled() => {
                return Err(EvmListenerError::CouldNotFetchBlock {
                    source: BlockFetchError::Cancelled,
                });
            }
            block_opt = buffer.get(i) => {
                block_opt.ok_or(EvmListenerError::SlotBufferError {
                    source: BufferError::IndexOutOfBounds,
                })?
            }
        };

        // Publish to {consumer_id}.catchup-event. Errors stop the producer too.
        // BlockFlow::Catchup is hardcoded inside publish_catchup_block_events.
        publisher::publish_catchup_block_events(
            &listener.repositories,
            &fetched_block,
            chain_id_u64,
            &consumer_id,
            &listener.broker,
            &listener.event_publisher,
            &listener.publish_config,
        )
        .await
        .map_err(|source| {
            cancel_token.cancel();
            EvmListenerError::PayloadBuildError { source }
        })?;

        info!(
            consumer_id = %consumer_id,
            block_number = block_number,
            block_hash = %fetched_block.block.header.hash,
            tx_count = fetched_block.transaction_count(),
            slot = i + 1,
            total = range_length,
            "Catchup block published"
        );
    }

    Ok(())
}

/// Parallel block fetcher (the "producer" in the producer-consumer pattern).
///
/// Spawns one `tokio::spawn` task per block in the range. Each task fetches a block via RPC
/// (using the listener's configured strategy with infinite retry on recoverable errors) and
/// stores the result in the corresponding slot of the `AsyncSlotBuffer`.
///
/// # Parameters
/// - `listener`: Cloned `EvmListener` (owns provider, strategy, compute_block config).
///   Passed by value because this runs in `tokio::spawn`.
/// - `buffer`: Shared slot buffer. Each task writes to exactly one slot (index = position in range).
/// - `cancel_token`: Shared cancellation token. On error, this function cancels the token to
///   stop both the cursor and any remaining fetch tasks.
/// - `range_start`: The block number corresponding to slot 0.
/// - `range_length`: Total number of blocks to fetch.
///
/// # Error Handling
/// - `get_block_by_number` has infinite retry built-in for recoverable errors (transport, timeout).
///   Only unrecoverable errors (UnsupportedMethod, DeserializationError) or cancellation bubble up.
/// - On the first error from any task, `cancel_token` is cancelled and remaining tasks are drained.
/// - Task panics (JoinError) are treated as cancellation errors.
async fn fetch_blocks_in_parallel(
    listener: EvmListener,
    buffer: AsyncSlotBuffer<FetchedBlock>,
    cancel_token: CancellationToken,
    range_start: u64,
    range_length: usize,
) -> Result<(), EvmListenerError> {
    let compute_block = listener.compute_block;
    let mut join_set: JoinSet<Result<(), EvmListenerError>> = JoinSet::new();

    for i in 0..range_length {
        let block_number = range_start + i as u64;
        let listener = listener.clone();
        let buffer = buffer.clone();
        // Child token: cancelled when parent cancel_token is cancelled
        let child_token = cancel_token.child_token();

        join_set.spawn(async move {
            let fetch_start = Instant::now();
            let fetched_block = listener
                .get_block_by_number(block_number, child_token, compute_block)
                .await?;

            metrics::histogram!(
                "listener_block_fetch_duration_seconds",
                "chain_id" => listener.chain_id.to_string()
            )
            .record(fetch_start.elapsed().as_secs_f64());

            // Store the fetched block in the corresponding slot.
            // set_once ensures each slot is written exactly once — AlreadyFilled indicates a logic bug.
            buffer.set_once(i, fetched_block).await.map_err(|e| {
                error!(
                    slot = i,
                    block_number = block_number,
                    error = %e,
                    "Buffer slot already filled — this is a logic bug"
                );
                EvmListenerError::SlotBufferError { source: e }
            })
        });
    }

    // Drain JoinSet: propagate first error, cancel remaining tasks.
    // This follows the established pattern from evm_block_fetcher.rs.
    while let Some(result) = join_set.join_next().await {
        match result {
            // Task completed successfully — slot was filled
            Ok(Ok(())) => continue,

            // Task returned an error — cancel all remaining and propagate
            Ok(Err(e)) => {
                cancel_token.cancel();
                // Drain remaining tasks to avoid abandoned futures
                while join_set.join_next().await.is_some() {}
                return Err(e);
            }

            // Task panicked (JoinError) — cancel all remaining, treat as cancellation
            Err(join_err) => {
                cancel_token.cancel();
                while join_set.join_next().await.is_some() {}
                error!(error = %join_err, "Fetch task panicked");
                return Err(EvmListenerError::CouldNotFetchBlock {
                    source: BlockFetchError::Cancelled,
                });
            }
        }
    }

    Ok(())
}
