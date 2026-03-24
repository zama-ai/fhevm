//! Flow step enums for structured logging
//!
//! This module provides two categories of steps:
//!
//! ## L1: Request Steps (INFO/WARN/ERROR)
//! Track job progress through the system. Always include `int_job_id`.
//! - `PublicDecryptStep`, `UserDecryptStep`, `InputProofStep`
//! - `KeyUrlStep`, `ListenerStep`
//!
//! When multiple relayers are operated against the same contracts, gateway
//! events may belong to other relayers. Pre-correlation observation logs use
//! DEBUG; post-correlation milestones
//! remain INFO. Retry/drop paths for unmatched events also use DEBUG.
//!
//! ## L2: Worker Steps (DEBUG/WARN)
//! Track infrastructure operations. May or may not have `int_job_id`.
//! - `WorkerStep`, `ThrottlerStep`, `TxEngineStep`

use std::fmt;

/// Public decrypt flow steps (L1)
///
/// Flow: Request → Dedup → Readiness queue → Readiness check → TX queue → TX → Gateway event → Response
#[derive(Debug, Clone, Copy)]
pub enum PublicDecryptStep {
    // Happy path milestones (INFO)
    ReqReceived,
    DedupHit,
    Queued,
    ReadinessQueued,
    ReadinessCheckPassed,
    TxQueued,
    TxConfirmed,
    GwEventReceived,
    RespSent,

    // Degraded path (WARN for local races, DEBUG for foreign events)
    Bounced,
    GwEventRetrying, // Gateway event arrived before gw_reference_id stored
}

impl fmt::Display for PublicDecryptStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReqReceived => "req_received",
            Self::DedupHit => "dedup_hit",
            Self::Queued => "queued",
            Self::ReadinessQueued => "readiness_queued",
            Self::ReadinessCheckPassed => "readiness_check_passed",
            Self::TxQueued => "tx_queued",
            Self::TxConfirmed => "tx_confirmed",
            Self::GwEventReceived => "gw_event_received",
            Self::RespSent => "resp_sent",
            Self::Bounced => "bounced",
            Self::GwEventRetrying => "gw_event_retrying",
        };
        write!(f, "{}", s)
    }
}

/// User decrypt flow steps (L1)
///
/// Flow: Request → Dedup → Readiness queue → Readiness check → TX queue → TX → Share events → Threshold → Response
#[derive(Debug, Clone, Copy)]
pub enum UserDecryptStep {
    // Happy path milestones (INFO)
    ReqReceived,
    DedupHit,
    Queued,
    ReadinessQueued,
    ReadinessCheckPassed,
    TxQueued,
    TxConfirmed,
    ShareReceived,
    ThresholdReached,
    RespSent,

    // Degraded path (WARN for local races, DEBUG for foreign events)
    Bounced,
    LateShareReceived,
    GwEventRetrying, // Gateway event arrived before gw_reference_id stored
}

impl fmt::Display for UserDecryptStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReqReceived => "req_received",
            Self::DedupHit => "dedup_hit",
            Self::Queued => "queued",
            Self::ReadinessQueued => "readiness_queued",
            Self::ReadinessCheckPassed => "readiness_check_passed",
            Self::TxQueued => "tx_queued",
            Self::TxConfirmed => "tx_confirmed",
            Self::ShareReceived => "share_received",
            Self::ThresholdReached => "threshold_reached",
            Self::RespSent => "resp_sent",
            Self::Bounced => "bounced",
            Self::LateShareReceived => "late_share_received",
            Self::GwEventRetrying => "gw_event_retrying",
        };
        write!(f, "{}", s)
    }
}

/// Input proof flow steps (L1)
///
/// Flow: Request → Dedup → TX queue → TX → Gateway event (accept/reject) → Response
#[derive(Debug, Clone, Copy)]
pub enum InputProofStep {
    // Happy path milestones (INFO)
    ReqReceived,
    DedupHit,
    Queued,
    TxQueued,
    TxConfirmed,
    GwEventReceived,
    ProofAccepted,
    ProofRejected,
    RespSent,

    // Degraded path (WARN for local races, DEBUG for foreign events)
    Bounced,
    GwEventRetrying, // Gateway event arrived before gw_reference_id stored
}

impl fmt::Display for InputProofStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ReqReceived => "req_received",
            Self::DedupHit => "dedup_hit",
            Self::Queued => "queued",
            Self::TxQueued => "tx_queued",
            Self::TxConfirmed => "tx_confirmed",
            Self::GwEventReceived => "gw_event_received",
            Self::ProofAccepted => "proof_accepted",
            Self::ProofRejected => "proof_rejected",
            Self::RespSent => "resp_sent",
            Self::Bounced => "bounced",
            Self::GwEventRetrying => "gw_event_retrying",
        };
        write!(f, "{}", s)
    }
}

/// Gateway listener steps (L1, blockchain event ingestion)
#[derive(Debug, Clone, Copy)]
pub enum ListenerStep {
    // Lifecycle
    ListenerStarted,
    ProviderConnected,
    SubscriptionActive,

    // Event processing (DEBUG — high-volume when multiple relayers share contracts)
    EventReceived,
    EventDuplicate,
    BlockProgressUpdated,

    // Degraded path (WARN)
    ProviderRetrying,
    SubscriptionDropped,
    BlockUpdateFailed,
}

impl fmt::Display for ListenerStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::ListenerStarted => "listener_started",
            Self::ProviderConnected => "provider_connected",
            Self::SubscriptionActive => "subscription_active",
            Self::EventReceived => "event_received",
            Self::EventDuplicate => "event_duplicate",
            Self::BlockProgressUpdated => "block_progress_updated",
            Self::ProviderRetrying => "provider_retrying",
            Self::SubscriptionDropped => "subscription_dropped",
            Self::BlockUpdateFailed => "block_update_failed",
        };
        write!(f, "{}", s)
    }
}

/// Background worker steps (L2)
///
/// Normal operations are DEBUG, problems are WARN.
#[derive(Debug, Clone, Copy)]
pub enum WorkerStep {
    // Normal ops (DEBUG)
    WorkerStarted,
    TickCompleted,
    RowsProcessed,

    // Problems (WARN)
    WorkerPanicked,
    WorkerRestarting,
}

impl fmt::Display for WorkerStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::WorkerStarted => "worker_started",
            Self::TickCompleted => "tick_completed",
            Self::RowsProcessed => "rows_processed",
            Self::WorkerPanicked => "worker_panicked",
            Self::WorkerRestarting => "worker_restarting",
        };
        write!(f, "{}", s)
    }
}

/// Throttler/rate-limiter steps (L2)
///
/// Normal operations are DEBUG, problems are WARN.
#[derive(Debug, Clone, Copy)]
pub enum ThrottlerStep {
    // Normal ops (DEBUG)
    TaskEnqueued,
    TaskDequeued,

    // Problems (WARN)
    QueueFull,
    QueueClosed,
}

impl fmt::Display for ThrottlerStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TaskEnqueued => "task_enqueued",
            Self::TaskDequeued => "task_dequeued",
            Self::QueueFull => "queue_full",
            Self::QueueClosed => "queue_closed",
        };
        write!(f, "{}", s)
    }
}

/// Transaction engine steps (L2)
///
/// Documents the tx lifecycle. Normal ops are DEBUG, retries are WARN, failures are ERROR.
#[derive(Debug, Clone, Copy)]
pub enum TxEngineStep {
    // Happy path (DEBUG/INFO)
    TxPrepared,    // Gas estimated, request built
    NonceAcquired, // Nonce obtained
    TxSending,     // Submitting to RPC
    TxSent,        // Receipt received, success

    // Degraded path (WARN)
    TxRetrying, // Recoverable error, retrying

    // Terminal failure (ERROR)
    TxFailed, // Unrecoverable, giving up
}

impl fmt::Display for TxEngineStep {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::TxPrepared => "tx_prepared",
            Self::NonceAcquired => "nonce_acquired",
            Self::TxSending => "tx_sending",
            Self::TxSent => "tx_sent",
            Self::TxRetrying => "tx_retrying",
            Self::TxFailed => "tx_failed",
        };
        write!(f, "{}", s)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_step_display() {
        assert_eq!(
            format!("{}", PublicDecryptStep::ReqReceived),
            "req_received"
        );
        assert_eq!(
            format!("{}", UserDecryptStep::ThresholdReached),
            "threshold_reached"
        );
        assert_eq!(
            format!("{}", InputProofStep::ProofAccepted),
            "proof_accepted"
        );
    }
}
