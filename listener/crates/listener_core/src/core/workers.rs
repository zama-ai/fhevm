use std::sync::Arc;

use async_trait::async_trait;
use broker::{AckDecision, Handler, HandlerError, Message, Publisher};
use tracing::{debug, error, info, warn};
use uuid::Uuid;

use primitives::event::{CancelCatchupPayload, CatchupPayload, FilterCommand, ReorgBacktrackEvent};
use primitives::routing;
use primitives::utils::checksum_optional_address;

use crate::store::FlowLock;
use crate::store::models::{
    CancelOutcome, CatchupFlow, CatchupStatus, CoverageMerge, FilterType as DbFilterType,
    RequestAdmission,
};

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

/// What the guard learned about the request a sub-range belongs to.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum RequestLookup {
    /// The payload carries no `catchup_id` — it was published by an
    /// orchestrator predating the field. Normal during a rolling deploy.
    Untracked,
    /// The store returned a row.
    Found(CatchupStatus),
    /// No row: swept after retention, or never written.
    Missing,
}

/// Whether a sub-range should still be fetched.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum SubrangeVerdict {
    /// Fetch it.
    Run,
    /// Fetch it, but warn — we could not confirm the request still exists.
    RunUnconfirmed,
    /// Drop it: the request reached a terminal state.
    Discard(CatchupStatus),
}

/// Map what the store said onto what the fetcher does.
///
/// Both "keep going" cases are deliberate, and the asymmetry is the point:
/// running a sub-range nobody wants is wasted RPC, while skipping one that is
/// wanted is a silent data gap. Every uncertain case therefore resolves to
/// *run*. Only a row we positively read as terminal stops the fetch.
fn subrange_verdict(lookup: RequestLookup) -> SubrangeVerdict {
    match lookup {
        RequestLookup::Untracked => SubrangeVerdict::Run,
        RequestLookup::Found(CatchupStatus::Active) => SubrangeVerdict::Run,
        RequestLookup::Found(status) => SubrangeVerdict::Discard(status),
        RequestLookup::Missing => SubrangeVerdict::RunUnconfirmed,
    }
}

/// Whether this delivery of a catchup request should publish sub-ranges.
///
/// Only a freshly admitted request fans out. [`RequestAdmission::AlreadyFannedOut`]
/// is the duplicate-request no-op: the same `catchup_id` arriving twice must
/// publish nothing the second time, because a re-fan-out of a large request is
/// hundreds of thousands of messages that every fetcher then has to drain.
/// [`RequestAdmission::AlreadyTerminal`] covers a request the consumer already
/// cancelled, possibly before this delivery was even processed.
fn should_fan_out(admission: &RequestAdmission) -> bool {
    matches!(admission, RequestAdmission::FanOut)
}

/// Which counter a cancel outcome increments, if any.
///
/// [`CancelOutcome::AlreadyTerminal`] returns `None` on purpose — re-cancelling
/// is legitimately idempotent and not worth a metric. [`CancelOutcome::NotOwned`]
/// must never be silent: it is an operator reaching for the brake and missing,
/// and without a signal the only symptom is that nothing happens.
fn cancel_metric(outcome: &CancelOutcome) -> Option<&'static str> {
    match outcome {
        CancelOutcome::Cancelled => Some("listener_catchup_cancelled_total"),
        CancelOutcome::AlreadyTerminal => None,
        CancelOutcome::NotOwned { .. } => Some("listener_catchup_cancel_rejected_total"),
    }
}

/// Decide whether a catchup sub-range should still be fetched.
///
/// Runs before any RPC call, so a cancelled request costs one indexed read per
/// outstanding sub-range instead of a full parallel fetch.
///
/// The lookup takes the primary key only. Adding a `consumer_id` predicate
/// here would turn an ownership mismatch into [`RequestLookup::Missing`],
/// which resolves to *run* — inverting the guard instead of tightening it.
/// Ownership is enforced on the cancel path, where the id comes from outside.
async fn catchup_subrange_is_live(
    listener: &EvmListener,
    payload: &CatchupPayload,
    flow: CatchupFlow,
) -> Result<bool, HandlerError> {
    let lookup = match payload.catchup_id {
        None => RequestLookup::Untracked,
        Some(catchup_id) => {
            let status = listener
                .catchup_request_status(catchup_id)
                .await
                .map_err(|e| classify(e, listener.chain_id()))?;
            match status {
                Some(status) => RequestLookup::Found(status),
                None => RequestLookup::Missing,
            }
        }
    };

    match subrange_verdict(lookup) {
        SubrangeVerdict::Run => Ok(true),
        SubrangeVerdict::RunUnconfirmed => {
            warn!(
                catchup_id = ?payload.catchup_id,
                consumer_id = %payload.consumer_id,
                block_start = payload.block_start,
                block_end = payload.block_end,
                "Catchup sub-range references an unknown request — running it anyway",
            );
            Ok(true)
        }
        SubrangeVerdict::Discard(status) => {
            metrics::counter!(
                "listener_catchup_subrange_discarded_total",
                "chain_id" => listener.chain_id().to_string(),
                "flow" => flow.metric_label(),
            )
            .increment(1);
            info!(
                catchup_id = ?payload.catchup_id,
                consumer_id = %payload.consumer_id,
                block_start = payload.block_start,
                block_end = payload.block_end,
                ?status,
                "Discarding catchup sub-range: request is no longer active",
            );
            Ok(false)
        }
    }
}

/// Credit a published sub-range to its request's coverage set (D16).
///
/// Runs **after** the publishes, never before. Merging first would mark a
/// request complete for blocks that a crash then prevents from being emitted;
/// merging after means a crash simply replays the sub-range, and the union
/// absorbs the repeat.
///
/// A merge failure is transient on purpose. Acking here would lose the
/// coverage permanently and strand the request `ACTIVE` forever, whereas a
/// replay costs one redundant fetch of an already-idempotent sub-range.
async fn record_subrange_coverage(
    listener: &EvmListener,
    catchup_id: Option<Uuid>,
    block_start: u64,
    block_end: u64,
    flow: CatchupFlow,
) -> Result<(), HandlerError> {
    // Untracked sub-range from a consumer predating `catchup_id`: nothing to
    // credit, and no row to complete.
    let Some(catchup_id) = catchup_id else {
        return Ok(());
    };

    let merge = listener
        .record_catchup_coverage(catchup_id, block_start, block_end)
        .await
        .map_err(|e| classify(e, listener.chain_id()))?;

    match merge {
        CoverageMerge::Completed => {
            metrics::counter!(
                "listener_catchup_completed_total",
                "chain_id" => listener.chain_id().to_string(),
                "flow" => flow.metric_label(),
            )
            .increment(1);
            info!(
                %catchup_id,
                block_start,
                block_end,
                ?flow,
                "Catchup request complete: every fanned-out block has been published",
            );
        }
        CoverageMerge::Progressed => {
            debug!(
                %catchup_id,
                block_start,
                block_end,
                "Recorded catchup sub-range coverage",
            );
        }
        // Cancelled mid-flight, already completed, or a replay after
        // completion. The blocks went out either way, so this is not an error.
        CoverageMerge::NotActive => {
            debug!(
                %catchup_id,
                block_start,
                block_end,
                "Published catchup sub-range for a request that is no longer active",
            );
        }
    }

    Ok(())
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
        let request = payload.clone();
        let subranges = self
            .listener
            .dispatch_catchup_range(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        // Record the request before publishing anything, then let the stored
        // row decide whether this delivery fans out at all.
        let admission = self
            .listener
            .admit_catchup_request(&request, CatchupFlow::Catchup, &subranges)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        if !should_fan_out(&admission) {
            info!(
                consumer_id = %request.consumer_id,
                catchup_id = ?request.catchup_id,
                ?admission,
                "Catchup orchestrator: request already recorded, publishing nothing"
            );
            return Ok(AckDecision::Ack);
        }

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
                "chain_id" => self.listener.chain_id().to_string(),
                "flow" => CatchupFlow::Catchup.metric_label()
            )
            .increment(subranges.len() as u64);
        }

        // The fanout is complete only now. A NULL fanned_out_at left by a crash
        // makes a redelivery republish the range — waste, never loss.
        self.listener
            .mark_catchup_fanned_out(request.catchup_id)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

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
        let request = payload.clone();
        let subranges = self
            .listener
            .dispatch_final_catchup_range(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        // Record the request before publishing anything, then let the stored
        // row decide whether this delivery fans out at all.
        let admission = self
            .listener
            .admit_catchup_request(&request, CatchupFlow::FinalCatchup, &subranges)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        if !should_fan_out(&admission) {
            info!(
                consumer_id = %request.consumer_id,
                catchup_id = ?request.catchup_id,
                ?admission,
                "Final catchup orchestrator: request already recorded, publishing nothing"
            );
            return Ok(AckDecision::Ack);
        }

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
                "chain_id" => self.listener.chain_id().to_string(),
                "flow" => CatchupFlow::FinalCatchup.metric_label()
            )
            .increment(subranges.len() as u64);
        }

        // The fanout is complete only now. A NULL fanned_out_at left by a crash
        // makes a redelivery republish the range — waste, never loss.
        self.listener
            .mark_catchup_fanned_out(request.catchup_id)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

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

        if !catchup_subrange_is_live(&self.listener, &payload, CatchupFlow::FinalCatchup).await? {
            return Ok(AckDecision::Ack);
        }

        // Captured before the run consumes the payload.
        let (catchup_id, block_start, block_end) =
            (payload.catchup_id, payload.block_start, payload.block_end);

        self.listener
            .run_final_range_catchup(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        record_subrange_coverage(
            &self.listener,
            catchup_id,
            block_start,
            block_end,
            CatchupFlow::FinalCatchup,
        )
        .await?;

        Ok(AckDecision::Ack)
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

        if !catchup_subrange_is_live(&self.listener, &payload, CatchupFlow::Catchup).await? {
            return Ok(AckDecision::Ack);
        }

        // Captured before the run consumes the payload.
        let (catchup_id, block_start, block_end) =
            (payload.catchup_id, payload.block_start, payload.block_end);

        self.listener
            .run_range_catchup(payload)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        record_subrange_coverage(
            &self.listener,
            catchup_id,
            block_start,
            block_end,
            CatchupFlow::Catchup,
        )
        .await?;

        Ok(AckDecision::Ack)
    }
}

// ── CancelCatchupHandler ────────────────────────────────────────────────

/// Handler for the `cancel-catchup` and `cancel-final-catchup` consumers.
///
/// One implementation parameterised by [`CatchupFlow`], unlike the
/// orchestrator/fetcher pairs above: the two flows differ only in which value
/// lands in a tombstone row, so a mirrored copy would be two chances to drift
/// and no extra expressiveness.
///
/// Retiring a request is a single UPDATE. Nothing is fetched, nothing is
/// published, and no sub-range is chased down — outstanding sub-ranges
/// discover the flipped row themselves at [`catchup_subrange_is_live`].
///
/// **Every outcome Acks.** A cancel that is rejected or redundant is not a
/// transient failure: classifying it as one would retry it forever and walk
/// the circuit breaker toward tripping catchup for the whole pod.
#[derive(Clone)]
pub struct CancelCatchupHandler {
    listener: Arc<EvmListener>,
    flow: CatchupFlow,
}

impl CancelCatchupHandler {
    pub fn new(listener: Arc<EvmListener>, flow: CatchupFlow) -> Self {
        Self { listener, flow }
    }
}

#[async_trait]
impl Handler for CancelCatchupHandler {
    async fn call(&self, msg: &Message) -> Result<AckDecision, HandlerError> {
        let mut payload: CancelCatchupPayload = match serde_json::from_slice(&msg.payload) {
            Ok(p) => p,
            Err(err) => {
                error!(
                    %err,
                    msg_id = %msg.metadata.id,
                    topic = %msg.metadata.topic,
                    payload_len = msg.payload.len(),
                    "Dead-lettering CancelCatchupPayload: deserialization failed",
                );
                return Ok(AckDecision::Dead);
            }
        };

        if let Err(err) = payload.validate() {
            error!(
                %err,
                msg_id = %msg.metadata.id,
                topic = %msg.metadata.topic,
                "Dead-lettering CancelCatchupPayload: validation failed",
            );
            return Ok(AckDecision::Dead);
        }

        let outcome = self
            .listener
            .cancel_catchup_request(payload.catchup_id, &payload.consumer_id, self.flow)
            .await
            .map_err(|e| classify(e, self.listener.chain_id()))?;

        if let Some(counter) = cancel_metric(&outcome) {
            metrics::counter!(
                counter,
                "chain_id" => self.listener.chain_id().to_string(),
                "flow" => self.flow.metric_label(),
            )
            .increment(1);
        }

        match outcome {
            CancelOutcome::Cancelled => info!(
                catchup_id = %payload.catchup_id,
                consumer_id = %payload.consumer_id,
                flow = self.flow.metric_label(),
                "Catchup request cancelled",
            ),
            // Re-cancelling is legitimately idempotent — nothing to say.
            CancelOutcome::AlreadyTerminal => {}
            // Loud on purpose. This is an operator reaching for the brake and
            // missing: the request is still running, and without this line the
            // only symptom is that nothing happens.
            CancelOutcome::NotOwned { stored } => error!(
                catchup_id = %payload.catchup_id,
                requested_by = %payload.consumer_id,
                owned_by = %stored,
                flow = self.flow.metric_label(),
                "Rejected catchup cancel: request belongs to a different consumer",
            ),
        }

        Ok(AckDecision::Ack)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // ── Fetcher guard ───────────────────────────────────────────────────

    #[test]
    fn an_active_request_runs_its_sub_ranges() {
        assert_eq!(
            subrange_verdict(RequestLookup::Found(CatchupStatus::Active)),
            SubrangeVerdict::Run,
        );
    }

    #[test]
    fn a_cancelled_request_discards_its_sub_ranges() {
        assert_eq!(
            subrange_verdict(RequestLookup::Found(CatchupStatus::Cancelled)),
            SubrangeVerdict::Discard(CatchupStatus::Cancelled),
        );
    }

    #[test]
    fn a_skipped_request_discards_its_sub_ranges() {
        assert_eq!(
            subrange_verdict(RequestLookup::Found(CatchupStatus::Skipped)),
            SubrangeVerdict::Discard(CatchupStatus::Skipped),
        );
    }

    /// Rolling deploy: an orchestrator predating `catchup_id` published this
    /// sub-range. Discarding it would drop blocks during every upgrade.
    #[test]
    fn a_payload_without_a_catchup_id_still_runs() {
        assert_eq!(
            subrange_verdict(RequestLookup::Untracked),
            SubrangeVerdict::Run,
        );
    }

    /// The inversion guard. A missing row means the request was swept or never
    /// written — never that it was cancelled. Mapping this to `Discard` would
    /// turn a retention sweep into a silent data gap, which is also what adding
    /// a `consumer_id` predicate to the lookup would do.
    #[test]
    fn a_missing_request_row_still_runs() {
        let verdict = subrange_verdict(RequestLookup::Missing);
        assert_eq!(verdict, SubrangeVerdict::RunUnconfirmed);
        assert!(
            !matches!(verdict, SubrangeVerdict::Discard(_)),
            "an unknown request must never stop a fetch",
        );
    }

    // ── Orchestrator admission ──────────────────────────────────────────

    #[test]
    fn a_freshly_admitted_request_fans_out() {
        assert!(should_fan_out(&RequestAdmission::FanOut));
    }

    /// D13. This is the whole point of `fanned_out_at`: the second delivery of
    /// the same `catchup_id` publishes nothing.
    #[test]
    fn a_duplicate_request_does_not_fan_out_again() {
        assert!(!should_fan_out(&RequestAdmission::AlreadyFannedOut));
    }

    #[test]
    fn a_retired_request_does_not_fan_out() {
        for status in [CatchupStatus::Cancelled, CatchupStatus::Skipped] {
            assert!(
                !should_fan_out(&RequestAdmission::AlreadyTerminal(status)),
                "{status:?} must not fan out",
            );
        }
    }

    // ── Cancel reporting ────────────────────────────────────────────────

    #[test]
    fn a_successful_cancel_is_counted() {
        assert_eq!(
            cancel_metric(&CancelOutcome::Cancelled),
            Some("listener_catchup_cancelled_total"),
        );
    }

    /// Re-cancelling is idempotent by design, so it is not an event.
    #[test]
    fn re_cancelling_is_not_counted() {
        assert_eq!(cancel_metric(&CancelOutcome::AlreadyTerminal), None);
    }

    /// The failure this mechanism exists to surface: an operator cancels an id
    /// owned by someone else, and without a signal the only symptom is that the
    /// runaway catchup keeps running.
    #[test]
    fn a_cancel_for_another_consumer_is_counted_separately() {
        let outcome = CancelOutcome::NotOwned {
            stored: "host-listener".to_string(),
        };
        assert_eq!(
            cancel_metric(&outcome),
            Some("listener_catchup_cancel_rejected_total"),
        );
        assert_ne!(
            cancel_metric(&outcome),
            cancel_metric(&CancelOutcome::Cancelled),
            "a rejected cancel must not look like a successful one",
        );
    }
}
