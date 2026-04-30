use std::sync::Arc;

use async_trait::async_trait;
use broker::{AckDecision, Handler, HandlerError, Message, Publisher};
use tracing::{error, info, warn};

use primitives::event::{CatchupPayload, FilterCommand, ReorgBacktrackEvent};
use primitives::routing;
use primitives::utils::checksum_optional_address;

use crate::store::FlowLock;

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
        CleanerError::BrokerPublishError { .. } => HandlerError::transient(err),
    }
}

/// Manual [`Handler`] impl for the clean-blocks consumer.
///
/// Ignores the message payload (the message is just a wake-up signal) and
/// calls [`Cleaner::run`]. DB errors are caught and skipped internally;
/// only broker publish failures bubble up as transient errors.
#[derive(Clone)]
pub struct CleanerHandler {
    cleaner: Arc<Cleaner>,
}

impl CleanerHandler {
    pub fn new(cleaner: Arc<Cleaner>) -> Self {
        Self { cleaner }
    }
}

#[async_trait]
impl Handler for CleanerHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        self.cleaner
            .run()
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify_cleaner)
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

        self.filters
            .add_filter(
                &event.consumer_id,
                from.as_deref(),
                to.as_deref(),
                log_address.as_deref(),
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

        self.filters
            .remove_filter(
                &event.consumer_id,
                from.as_deref(),
                to.as_deref(),
                log_address.as_deref(),
            )
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify_filter)
    }
}

// ── CatchupHandler ──────────────────────────────────────────────────────

/// Handler for the `catchup` consumer.
///
/// Deserializes `msg.payload` into [`CatchupPayload`], validates it (which
/// trims `consumer_id` and enforces the inclusive range cap
/// `CATCHUP_MAX_RANGE`), then calls [`EvmListener::run_range_catchup`].
///
/// Deserialization or validation failures are dead-lettered immediately —
/// they are deterministic and will never succeed on retry. Replay errors
/// route through the same [`classify`] path as the live cursor so that
/// infrastructure failures (RPC, broker, slot buffer) are transient and trip
/// the circuit breaker, while invariant violations are permanent.
///
/// One-shot: no continuation publish (catchup is bounded per message; the
/// producer chunks if the requested range exceeds `CATCHUP_MAX_RANGE`). No
/// advisory lock (catchup is idempotent at the downstream consumer level).
#[derive(Clone)]
pub struct CatchupHandler {
    listener: Arc<EvmListener>,
}

impl CatchupHandler {
    pub fn new(listener: Arc<EvmListener>) -> Self {
        Self { listener }
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

        self.listener
            .run_range_catchup(payload)
            .await
            .map(|_| AckDecision::Ack)
            .map_err(|e| classify(e, self.listener.chain_id()))
    }
}
