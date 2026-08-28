use std::sync::Arc;

use async_trait::async_trait;
use broker::{AckDecision, Handler, HandlerError, Message, Publisher};
use tracing::{error, info, warn};

use primitives::event::{CatchupPayload, FilterCommand, ReorgBacktrackEvent};
use primitives::routing;
use primitives::utils::checksum_optional_address;

use crate::store::FlowLock;
use crate::store::models::FilterType as DbFilterType;

use super::cleaner::{Cleaner, CleanerError};
use super::evm_listener::{CursorResult, EvmListener, EvmListenerError};
use super::filters::{FilterError, Filters};
use crate::metrics::error_kind_label;

/// Classify an [`EvmListenerError`] as transient (infrastructure) or permanent (logic bug).
///
/// Explicit match arms — no wildcard — so that adding a new `EvmListenerError`
/// variant forces a conscious classification decision at compile time.
fn classify(err: EvmListenerError, chain_id: u64) -> HandlerError {
    let chain_id_str = chain_id.to_string();
    let kind = error_kind_label(&err);

    match &err {
        EvmListenerError::CouldNotFetchBlock { .. }
        | EvmListenerError::CouldNotComputeBlock { .. }
        | EvmListenerError::DatabaseError { .. }
        | EvmListenerError::ChainHeightError { .. }
        | EvmListenerError::SlotBufferError { .. }
        | EvmListenerError::BrokerPublishError { .. }
        | EvmListenerError::MessageProcessingError { .. }
        | EvmListenerError::PayloadBuildError { .. } => {
            metrics::counter!(
                "listener_transient_errors_total",
                "chain_id" => chain_id_str,
                "error_kind" => kind,
            )
            .increment(1);
            HandlerError::transient(err)
        }
        EvmListenerError::InvariantViolation { .. } => {
            metrics::counter!(
                "listener_permanent_errors_total",
                "chain_id" => chain_id_str,
                "error_kind" => kind,
            )
            .increment(1);
            HandlerError::permanent(err)
        }
    }
}

/// Classify a [`FilterError`] as transient or permanent.
fn classify_filter(err: FilterError) -> HandlerError {
    match &err {
        FilterError::DatabaseError { .. } => HandlerError::transient(err),
    }
}

// ── CleanerHandler ──────────────────────────────────────────────────────

/// Classify a [`CleanerError`] as transient or permanent.
fn classify_cleaner(err: CleanerError) -> HandlerError {
    match &err {
        CleanerError::BrokerPublishError { .. } | CleanerError::AdvisoryLockError { .. } => {
            HandlerError::transient(err)
        }
    }
}

/// Manual [`Handler`] impl for the clean-blocks consumer.
///
/// Ignores the message payload (the message is just a wake-up signal) and
/// calls [`Cleaner::run`]. DB errors are caught and skipped internally;
/// only lock-acquire and broker publish failures bubble up as transient errors.
/// Acquires a PostgreSQL advisory lock (cleaner-specific key, per chain_id)
/// before processing. If the lock is held by another pod, the message is
/// Acked (not requeued): a lock holder is already running the loop and will
/// republish the next iteration itself.
/// This provides HPA-safe mutual exclusion for the cleaner flow and prevents
/// redelivered duplicates from multiplying the self-perpetuating clean loop.
#[derive(Clone)]
pub struct CleanerHandler {
    cleaner: Arc<Cleaner>,
    flow_lock: FlowLock,
    publisher: Publisher,
}

impl CleanerHandler {
    pub fn new(cleaner: Arc<Cleaner>, flow_lock: FlowLock, publisher: Publisher) -> Self {
        Self {
            cleaner,
            flow_lock,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for CleanerHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        // Step 1: Try to acquire the distributed lock (non-blocking).
        let guard = match self.flow_lock.try_acquire().await {
            Ok(Some(guard)) => guard,
            Ok(None) => {
                warn!(
                    "Cleaner: advisory lock held by another processor, Acking and skipping this process, mostly duplicate."
                );
                return Ok(AckDecision::Ack);
            }
            Err(e) => {
                return Err(classify_cleaner(CleanerError::AdvisoryLockError {
                    message: format!("Failed to acquire advisory lock: {e}"),
                }));
            }
        };

        // Step 2: Process under lock. The lock spans the cron sleep inside
        // `run()` on purpose: a redelivered duplicate arriving meanwhile must
        // be skipped, or it would start a second self-perpetuating loop.
        let reschedule = self.cleaner.run().await;

        // Step 3: Release lock BEFORE publishing (eliminates race with other handlers).
        if let Err(unlock_err) = guard.release().await {
            warn!(error = %unlock_err, "Failed to explicitly release advisory lock");
        }

        // Step 4: Publish next iteration AFTER lock release, then Ack.
        if reschedule {
            self.publisher
                .publish(routing::CLEAN_BLOCKS, &serde_json::Value::Null)
                .await
                .map_err(|e| {
                    error!(error = %e, "Cleaner: failed to publish next iteration");
                    classify_cleaner(CleanerError::BrokerPublishError {
                        message: format!("Broker publish failed: {e}"),
                    })
                })?;
        }
        Ok(AckDecision::Ack)
    }
}

// ── FinalCleanerHandler ─────────────────────────────────────────────────

/// Manual [`Handler`] impl for the clean-final-blocks consumer.
///
/// Ignores the message payload (the message is just a wake-up signal) and
/// calls [`Cleaner::run_final`]. DB errors are caught and skipped internally;
/// only lock-acquire and broker publish failures bubble up as transient errors.
/// Acquires a PostgreSQL advisory lock (final-cleaner-specific key, per
/// chain_id) before processing. If the lock is held by another pod, the
/// message is Acked (not requeued): a lock holder is already running the loop
/// and will republish the next iteration itself.
/// This provides HPA-safe mutual exclusion for the final-cleaner flow and
/// prevents redelivered duplicates from multiplying the self-perpetuating
/// clean loop.
#[derive(Clone)]
pub struct FinalCleanerHandler {
    cleaner: Arc<Cleaner>,
    flow_lock: FlowLock,
    publisher: Publisher,
}

impl FinalCleanerHandler {
    pub fn new(cleaner: Arc<Cleaner>, flow_lock: FlowLock, publisher: Publisher) -> Self {
        Self {
            cleaner,
            flow_lock,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for FinalCleanerHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        // Step 1: Try to acquire the distributed lock (non-blocking).
        let guard = match self.flow_lock.try_acquire().await {
            Ok(Some(guard)) => guard,
            Ok(None) => {
                warn!(
                    "FinalCleaner: advisory lock held by another processor, Acking and skipping this process, mostly duplicate."
                );
                return Ok(AckDecision::Ack);
            }
            Err(e) => {
                return Err(classify_cleaner(CleanerError::AdvisoryLockError {
                    message: format!("Failed to acquire advisory lock: {e}"),
                }));
            }
        };

        // Step 2: Process under lock. The lock spans the cron sleep inside
        // `run_final()` on purpose: a redelivered duplicate arriving meanwhile
        // must be skipped, or it would start a second self-perpetuating loop.
        let reschedule = self.cleaner.run_final().await;

        // Step 3: Release lock BEFORE publishing (eliminates race with other handlers).
        if let Err(unlock_err) = guard.release().await {
            warn!(error = %unlock_err, "Failed to explicitly release advisory lock");
        }

        // Step 4: Publish next iteration AFTER lock release, then Ack.
        if reschedule {
            self.publisher
                .publish(routing::CLEAN_FINAL_BLOCKS, &serde_json::Value::Null)
                .await
                .map_err(|e| {
                    error!(error = %e, "FinalCleaner: failed to publish next iteration");
                    classify_cleaner(CleanerError::BrokerPublishError {
                        message: format!("Broker publish failed: {e}"),
                    })
                })?;
        }
        Ok(AckDecision::Ack)
    }
}

// ── FetchHandler ─────────────────────────────────────────────────────────

/// Manual [`Handler`] impl for the fetch-new-blocks consumer.
///
/// Ignores the message payload (the message is just a wake-up signal) and
/// calls [`EvmListener::fetch_blocks_and_run_cursor`]. Errors are routed
/// through [`classify`] so that infrastructure failures (DB, RPC) produce
/// `HandlerError::Transient` — enabling the circuit breaker.
/// Acquires a PostgreSQL advisory lock (per chain_id) before processing.
/// If the lock is held by another pod, the message is Acked (not requeued).
/// Avoids infinite message requeuing over message duplication.
/// This provides HPA-safe mutual exclusion for the fetch flow.
#[derive(Clone)]
pub struct FetchHandler {
    listener: Arc<EvmListener>,
    flow_lock: FlowLock,
    publisher: Publisher,
}

impl FetchHandler {
    pub fn new(listener: Arc<EvmListener>, flow_lock: FlowLock, publisher: Publisher) -> Self {
        Self {
            listener,
            flow_lock,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for FetchHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        // Step 1: Try to acquire the distributed lock (non-blocking).
        let guard = match self.flow_lock.try_acquire().await {
            Ok(Some(guard)) => guard,
            Ok(None) => {
                warn!(
                    "Fetch: advisory lock held by another processor, Acking and skipping this process, mostly duplicate."
                );
                return Ok(AckDecision::Ack);
            }
            Err(e) => {
                return Err(HandlerError::transient(
                    EvmListenerError::MessageProcessingError {
                        message: format!("Failed to acquire advisory lock: {e}"),
                    },
                ));
            }
        };

        // Step 2: Process under lock.
        let result = self.listener.fetch_blocks_and_run_cursor().await;

        // Step 3: Release lock BEFORE publishing (eliminates race with other handlers).
        if let Err(unlock_err) = guard.release().await {
            warn!(error = %unlock_err, "Failed to explicitly release advisory lock");
        }

        // Step 4: Publish continuation message AFTER lock release, then Ack.
        match result {
            Ok(CursorResult::ReorgDetected {
                block_number,
                block_hash,
                parent_hash,
            }) => {
                let event = ReorgBacktrackEvent {
                    block_number,
                    block_hash,
                    parent_hash,
                };
                self.publisher
                    .publish(routing::BACKTRACK_REORG, &event)
                    .await
                    .map_err(|e| {
                        error!(error = %e, "Failed to publish backtrack event");
                        HandlerError::transient(EvmListenerError::BrokerPublishError {
                            message: format!("Broker publish failed: {e}"),
                        })
                    })?;
                info!(
                    block_number = block_number,
                    block_hash = %block_hash,
                    "Backtrack event published"
                );
                Ok(AckDecision::Ack)
            }
            Ok(_) => {
                // Complete or UpToDate — schedule next fetch iteration.
                self.publisher
                    .publish(routing::FETCH_NEW_BLOCKS, &serde_json::Value::Null)
                    .await
                    .map_err(|e| {
                        error!(error = %e, "Failed to publish fetch trigger");
                        HandlerError::transient(EvmListenerError::BrokerPublishError {
                            message: format!("Broker publish failed: {e}"),
                        })
                    })?;
                Ok(AckDecision::Ack)
            }
            Err(e) => Err(classify(e, self.listener.chain_id())),
        }
    }
}

// ── FinalityHandler ──────────────────────────────────────────────────────

/// Manual [`Handler`] impl for the fetch-final-block consumer.
///
/// Ignores the message payload (the message is just a wake-up signal) and
/// calls [`EvmListener::fetch_final_blocks`]. Errors are routed through
/// [`classify`] so that infrastructure failures (DB, RPC) produce
/// `HandlerError::Transient` — enabling the circuit breaker.
/// Acquires a PostgreSQL advisory lock (finality-specific key, per chain_id)
/// before processing. If the lock is held by another pod, the message is
/// Acked (not requeued). Avoids infinite message requeuing over message
/// duplication. This provides HPA-safe mutual exclusion for the finality
/// flow, fully independent from the fetch/reorg cursor lock: a stall of the
/// finality flow never impacts the live flow, and vice versa.
/// When the finality flow is inactive, the message is Acked without
/// re-triggering, so a stale seeded loop terminates deliberately.
#[derive(Clone)]
pub struct FinalityHandler {
    listener: Arc<EvmListener>,
    flow_lock: FlowLock,
    publisher: Publisher,
}

impl FinalityHandler {
    pub fn new(listener: Arc<EvmListener>, flow_lock: FlowLock, publisher: Publisher) -> Self {
        Self {
            listener,
            flow_lock,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for FinalityHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        // Step 0: Inactive flow — skip and end the loop deliberately.
        if !self.listener.finality_active() {
            info!("Finality: inactive — skipping and not re-triggering");
            return Ok(AckDecision::Ack);
        }

        // Step 1: Try to acquire the distributed lock (non-blocking).
        let guard = match self.flow_lock.try_acquire().await {
            Ok(Some(guard)) => guard,
            Ok(None) => {
                warn!(
                    "Finality: advisory lock held by another processor, Acking and skipping this process, mostly duplicate."
                );
                return Ok(AckDecision::Ack);
            }
            Err(e) => {
                return Err(HandlerError::transient(
                    EvmListenerError::MessageProcessingError {
                        message: format!("Failed to acquire advisory lock: {e}"),
                    },
                ));
            }
        };

        // Step 2: Process under lock.
        let result = self.listener.fetch_final_blocks().await;

        // Step 3: Release lock BEFORE publishing (eliminates race with other handlers).
        if let Err(unlock_err) = guard.release().await {
            warn!(error = %unlock_err, "Failed to explicitly release advisory lock");
        }

        // Step 4: Publish continuation message AFTER lock release, then Ack.
        match result {
            Ok(()) => {
                // Up-to-date or complete — schedule next finality iteration.
                self.publisher
                    .publish(routing::FETCH_FINAL_BLOCK, &serde_json::Value::Null)
                    .await
                    .map_err(|e| {
                        error!(error = %e, "Failed to publish finality trigger");
                        HandlerError::transient(EvmListenerError::BrokerPublishError {
                            message: format!("Broker publish failed: {e}"),
                        })
                    })?;
                Ok(AckDecision::Ack)
            }
            Err(e) => Err(classify(e, self.listener.chain_id())),
        }
    }
}

// ── ReorgHandlerV2 ──────────────────────────────────────────────────────

/// Handler for the backtrack-reorg consumer using the state-atomic v2 algorithm.
///
/// Identical wiring to [`ReorgHandler`] but calls [`EvmListener::reorg_backtrack_v2`].
/// Errors go through [`classify`] unchanged — the handler preserves all existing
/// error semantics (transient for infra, permanent for invariants).
///
/// Acquires a PostgreSQL advisory lock (per chain_id) before processing.
/// Shares the same lock key as [`FetchHandler`], guaranteeing fetch and
/// reorg never run in parallel for the same chain.
#[derive(Clone)]
pub struct ReorgHandler {
    listener: Arc<EvmListener>,
    flow_lock: FlowLock,
    publisher: Publisher,
}

impl ReorgHandler {
    pub fn new(listener: Arc<EvmListener>, flow_lock: FlowLock, publisher: Publisher) -> Self {
        Self {
            listener,
            flow_lock,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for ReorgHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        // Deserialize before lock — dead-letter garbage early.
        let event: ReorgBacktrackEvent = serde_json::from_slice(&msg.payload)?;

        // Step 1: Try to acquire the distributed lock (non-blocking).
        let guard = match self.flow_lock.try_acquire().await {
            Ok(Some(guard)) => guard,
            Ok(None) => {
                warn!("Reorg: advisory lock held by another processor, Acking, mostly duplicate.");
                return Ok(AckDecision::Ack);
            }
            Err(e) => {
                return Err(HandlerError::transient(
                    EvmListenerError::MessageProcessingError {
                        message: format!("Failed to acquire advisory lock: {e}"),
                    },
                ));
            }
        };

        // Step 2: Process under lock.
        let result = self.listener.reorg_backtrack(event).await;

        // Step 3: Release lock BEFORE publishing (eliminates race with other handlers).
        if let Err(unlock_err) = guard.release().await {
            warn!(error = %unlock_err, "Failed to explicitly release advisory lock");
        }

        // Step 4: Publish cursor resume AFTER lock release, then Ack.
        match result {
            Ok(()) => {
                self.publisher
                    .publish(routing::FETCH_NEW_BLOCKS, &serde_json::Value::Null)
                    .await
                    .map_err(|e| {
                        error!(error = %e, "Failed to publish fetch trigger after reorg backtrack");
                        HandlerError::transient(EvmListenerError::BrokerPublishError {
                            message: format!("Broker publish failed: {e}"),
                        })
                    })?;
                Ok(AckDecision::Ack)
            }
            Err(e) => Err(classify(e, self.listener.chain_id())),
        }
    }
}

// ── WatchHandler ────────────────────────────────────────────────────────

/// Handler for the control.watch consumer.
///
/// Deserializes `msg.payload` into [`FilterCommand`], validates and checksums
/// it, then calls [`Filters::add_filter`]. Deserialization and validation
/// errors are dead-lettered immediately (deterministic, will never succeed on
/// retry). Database errors are transient via [`classify_filter`].
#[derive(Clone)]
pub struct WatchHandler {
    filters: Arc<Filters>,
}

impl WatchHandler {
    pub fn new(filters: Arc<Filters>) -> Self {
        Self { filters }
    }
}

#[async_trait]
impl Handler for WatchHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut event: FilterCommand = match serde_json::from_slice(&msg.payload) {
            Ok(e) => e,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering watch FilterCommand: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = event.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering watch FilterCommand: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        let from = checksum_optional_address(&event.from);
        let to = checksum_optional_address(&event.to);
        let log_address = checksum_optional_address(&event.log_address);
        // Missing filter_type means a legacy (or default) command: Live watcher.
        let filter_type: DbFilterType = event.filter_type.unwrap_or_default().into();

        self.filters
            .add_filter(
                &event.consumer_id,
                from.as_deref(),
                to.as_deref(),
                log_address.as_deref(),
                filter_type,
            )
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify_filter)
    }
}

// ── UnwatchHandler ──────────────────────────────────────────────────────

/// Handler for the control.unwatch consumer.
///
/// Deserializes `msg.payload` into [`FilterCommand`], validates and checksums
/// it, then calls [`Filters::remove_filter`]. Deserialization and validation
/// errors are dead-lettered immediately (deterministic, will never succeed on
/// retry). Database errors are transient via [`classify_filter`].
#[derive(Clone)]
pub struct UnwatchHandler {
    filters: Arc<Filters>,
}

impl UnwatchHandler {
    pub fn new(filters: Arc<Filters>) -> Self {
        Self { filters }
    }
}

#[async_trait]
impl Handler for UnwatchHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut event: FilterCommand = match serde_json::from_slice(&msg.payload) {
            Ok(e) => e,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering unwatch FilterCommand: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = event.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering unwatch FilterCommand: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        let from = checksum_optional_address(&event.from);
        let to = checksum_optional_address(&event.to);
        let log_address = checksum_optional_address(&event.log_address);
        // Missing filter_type means a legacy (or default) command: Live watcher.
        let filter_type: DbFilterType = event.filter_type.unwrap_or_default().into();

        self.filters
            .remove_filter(
                &event.consumer_id,
                from.as_deref(),
                to.as_deref(),
                log_address.as_deref(),
                filter_type,
            )
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify_filter)
    }
}

// ── CatchupHandler ──────────────────────────────────────────────────────

/// Handler for the `catchup` consumer (the **orchestrator**).
///
/// Deserializes `msg.payload` into [`CatchupPayload`], validates it (trims
/// `consumer_id`, enforces `block_start <= block_end`), asks the listener
/// to compute bounded sub-payloads, then publishes each sub-payload to
/// `routing::RANGE_CATCHUP` itself. The listener is the source of truth for
/// the orchestrator logic (chain height fetch, skip-above-head, clamp,
/// split); the broker boundary lives here in the handler.
///
/// Deserialization or validation failures are dead-lettered immediately —
/// they are deterministic and will never succeed on retry. Orchestrator
/// errors (RPC head fetch) route through the same [`classify`] path as the
/// live cursor. Broker publish failures map to
/// `HandlerError::transient(EvmListenerError::BrokerPublishError { … })` —
/// the broker retries the orchestrator message; already-published sub-ranges
/// will be re-published on retry, downstream dedupes by
/// (block_number, block_hash).
///
/// No advisory lock by design.
#[derive(Clone)]
pub struct CatchupHandler {
    listener: Arc<EvmListener>,
    publisher: Publisher,
}

impl CatchupHandler {
    pub fn new(listener: Arc<EvmListener>, publisher: Publisher) -> Self {
        Self {
            listener,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for CatchupHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut payload: CatchupPayload = match serde_json::from_slice(&msg.payload) {
            Ok(p) => p,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering CatchupPayload: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = payload.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering CatchupPayload: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        // Compute the sub-ranges (chain height fetch + skip + clamp + split
        // live in EvmListener::dispatch_catchup_range).
        let subranges = self
            .listener
            .dispatch_catchup_range(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        // Publish each sub-range to range-catchup. Bubble any broker error
        // out as transient so the broker retries the orchestrator message.
        for sub in &subranges {
            self.publisher
                .publish(routing::RANGE_CATCHUP, sub)
                .await
                .map_err(|e| {
                    error!(
                        consumer_id = %sub.consumer_id,
                        block_start = sub.block_start,
                        block_end = sub.block_end,
                        error = %e,
                        "Failed to publish catchup sub-range",
                    );
                    HandlerError::transient(EvmListenerError::BrokerPublishError {
                        message: format!(
                            "Failed to publish catchup sub-range [{}, {}]: {}",
                            sub.block_start, sub.block_end, e
                        ),
                    })
                })?;
        }

        // Increment fan-out counter only after the full loop succeeded — same
        // semantics as the previous `dispatch_catchup_range` had internally.
        if !subranges.is_empty() {
            metrics::counter!(
                "listener_catchup_subranges_total",
                "chain_id" => self.listener.chain_id().to_string()
            )
            .increment(subranges.len() as u64);
        }

        Ok(AckDecision::Ack)
    }
}

// ── FinalCatchupHandler ─────────────────────────────────────────────────

/// Handler for the `final-catchup` consumer (the finality **orchestrator**).
///
/// Mirror of [`CatchupHandler`] for FINAL watchers: deserializes
/// `msg.payload` into [`CatchupPayload`], validates it, asks the listener to
/// compute bounded sub-payloads clamped to the **finalized head**, then
/// publishes each sub-payload to `routing::RANGE_FINAL_CATCHUP` itself.
///
/// Deserialization or validation failures are dead-lettered immediately —
/// they are deterministic and will never succeed on retry. Orchestrator
/// errors (final head fetch) route through the same [`classify`] path as the
/// live cursor. Broker publish failures map to
/// `HandlerError::transient(EvmListenerError::BrokerPublishError { … })` —
/// the broker retries the orchestrator message; already-published sub-ranges
/// will be re-published on retry, downstream dedupes by
/// (block_number, block_hash).
///
/// Requests are dropped (Acked) when the finality flow is inactive.
/// No advisory lock by design.
#[derive(Clone)]
pub struct FinalCatchupHandler {
    listener: Arc<EvmListener>,
    publisher: Publisher,
}

impl FinalCatchupHandler {
    pub fn new(listener: Arc<EvmListener>, publisher: Publisher) -> Self {
        Self {
            listener,
            publisher,
        }
    }
}

#[async_trait]
impl Handler for FinalCatchupHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut payload: CatchupPayload = match serde_json::from_slice(&msg.payload) {
            Ok(p) => p,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering final catchup CatchupPayload: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = payload.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering final catchup CatchupPayload: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        // Inactive finality flow — drop the request deliberately (after
        // validation, so the log identifies what is being discarded).
        if !self.listener.finality_active() {
            warn!(
                consumer_id = %payload.consumer_id,
                block_start = payload.block_start,
                block_end = payload.block_end,
                "FinalCatchup: finality flow inactive — dropping final catchup request"
            );
            return Ok(AckDecision::Ack);
        }

        // Compute the sub-ranges (final height fetch + skip + clamp + split
        // live in EvmListener::dispatch_final_catchup_range).
        let subranges = self
            .listener
            .dispatch_final_catchup_range(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        // Publish each sub-range to range-final-catchup. Bubble any broker
        // error out as transient so the broker retries the orchestrator message.
        for sub in &subranges {
            self.publisher
                .publish(routing::RANGE_FINAL_CATCHUP, sub)
                .await
                .map_err(|e| {
                    error!(
                        consumer_id = %sub.consumer_id,
                        block_start = sub.block_start,
                        block_end = sub.block_end,
                        error = %e,
                        "Failed to publish final catchup sub-range",
                    );
                    HandlerError::transient(EvmListenerError::BrokerPublishError {
                        message: format!(
                            "Failed to publish final catchup sub-range [{}, {}]: {}",
                            sub.block_start, sub.block_end, e
                        ),
                    })
                })?;
        }

        // Increment fan-out counter only after the full loop succeeded — same
        // semantics as the live catchup orchestrator.
        if !subranges.is_empty() {
            metrics::counter!(
                "listener_catchup_subranges_total",
                "chain_id" => self.listener.chain_id().to_string()
            )
            .increment(subranges.len() as u64);
        }

        Ok(AckDecision::Ack)
    }
}

// ── RangeFinalCatchupHandler ────────────────────────────────────────────

/// Handler for the `range-final-catchup` consumer (the finality **fetcher**).
///
/// Consumes bounded sub-payloads produced by [`FinalCatchupHandler`] and runs
/// [`EvmListener::run_final_range_catchup`] for each: parallel fetch +
/// in-order publish on `{consumer_id}.final-catchup-event`.
///
/// Defensively re-validates the payload — sub-payloads cross the broker
/// boundary, and the broker is the trust boundary. Errors classified through
/// the same [`classify`] path as the live cursor. Sub-ranges are dropped
/// (Acked) when the finality flow is inactive — a sub-range enqueued before
/// the flag flipped must not run.
#[derive(Clone)]
pub struct RangeFinalCatchupHandler {
    listener: Arc<EvmListener>,
}

impl RangeFinalCatchupHandler {
    pub fn new(listener: Arc<EvmListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl Handler for RangeFinalCatchupHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut payload: CatchupPayload = match serde_json::from_slice(&msg.payload) {
            Ok(p) => p,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering range-final-catchup CatchupPayload: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = payload.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering range-final-catchup CatchupPayload: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        // Inactive finality flow — drop the sub-range deliberately (after
        // validation, so the log identifies what is being discarded). A
        // sub-range enqueued before the flag flipped must not run.
        if !self.listener.finality_active() {
            warn!(
                consumer_id = %payload.consumer_id,
                block_start = payload.block_start,
                block_end = payload.block_end,
                "RangeFinalCatchup: finality flow inactive — dropping final catchup sub-range"
            );
            return Ok(AckDecision::Ack);
        }

        self.listener
            .run_final_range_catchup(payload)
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| classify(e, self.listener.chain_id()))
    }
}

// ── RangeCatchupHandler ─────────────────────────────────────────────────

/// Handler for the `range-catchup` consumer (the **fetcher**).
///
/// Consumes bounded sub-payloads produced by [`CatchupHandler`] and runs
/// [`EvmListener::run_range_catchup`] for each: parallel fetch +
/// in-order publish on `{consumer_id}.catchup-event`.
///
/// Defensively re-validates the payload — sub-payloads cross the broker
/// boundary, and the broker is the trust boundary. Errors classified through
/// the same [`classify`] path as the live cursor.
#[derive(Clone)]
pub struct RangeCatchupHandler {
    listener: Arc<EvmListener>,
}

impl RangeCatchupHandler {
    pub fn new(listener: Arc<EvmListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl Handler for RangeCatchupHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut payload: CatchupPayload = match serde_json::from_slice(&msg.payload) {
            Ok(p) => p,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering range-catchup CatchupPayload: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = payload.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering range-catchup CatchupPayload: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        self.listener
            .run_range_catchup(payload)
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| classify(e, self.listener.chain_id()))
    }
}
