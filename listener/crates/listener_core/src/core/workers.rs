use std::sync::Arc;

use async_trait::async_trait;
use broker::{AckDecision, Handler, HandlerError, Message};
use tracing::error;

use primitives::event::{FilterCommand, ReorgBacktrackEvent};
use primitives::utils::checksum_optional_address;

use super::cleaner::{Cleaner, CleanerError};
use super::evm_listener::{EvmListener, EvmListenerError};
use super::filters::{FilterError, Filters};

/// Classify an [`EvmListenerError`] as transient (infrastructure) or permanent (logic bug).
///
/// Explicit match arms — no wildcard — so that adding a new `EvmListenerError`
/// variant forces a conscious classification decision at compile time.
fn classify(err: EvmListenerError) -> HandlerError {
    match &err {
        EvmListenerError::CouldNotFetchBlock { .. }
        | EvmListenerError::CouldNotComputeBlock { .. }
        | EvmListenerError::DatabaseError { .. }
        | EvmListenerError::ChainHeightError { .. }
        | EvmListenerError::SlotBufferError { .. }
        | EvmListenerError::BrokerPublishError { .. }
        | EvmListenerError::MessageProcessingError { .. }
        | EvmListenerError::PayloadBuildError { .. } => HandlerError::transient(err),
        EvmListenerError::InvariantViolation { .. } => HandlerError::permanent(err),
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
#[derive(Clone)]
pub struct FetchHandler {
    listener: Arc<EvmListener>,
}

impl FetchHandler {
    pub fn new(listener: Arc<EvmListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl Handler for FetchHandler {
    async fn call(&self, _msg: &Message) -> Result<AckDecision, HandlerError> {
        self.listener
            .fetch_blocks_and_run_cursor()
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify)
    }
}

// ── ReorgHandlerV2 ──────────────────────────────────────────────────────

/// Handler for the backtrack-reorg consumer using the state-atomic v2 algorithm.
///
/// Identical wiring to [`ReorgHandler`] but calls [`EvmListener::reorg_backtrack_v2`].
/// Errors go through [`classify`] unchanged — the handler preserves all existing
/// error semantics (transient for infra, permanent for invariants).
///
/// To switch from v1 to v2, replace `ReorgHandler::new(...)` with
/// `ReorgHandlerV2::new(...)` in `main.rs`. Both handlers accept the same
/// `ReorgBacktrackEvent` payload.
#[derive(Clone)]
pub struct ReorgHandler {
    listener: Arc<EvmListener>,
}

impl ReorgHandler {
    pub fn new(listener: Arc<EvmListener>) -> Self {
        Self { listener }
    }
}

#[async_trait]
impl Handler for ReorgHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let event: ReorgBacktrackEvent = serde_json::from_slice(&msg.payload)?;
        self.listener
            .reorg_backtrack(event)
            .await
            .map(|_| AckDecision::Ack)
            .map_err(classify)
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
