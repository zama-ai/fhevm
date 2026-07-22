//! Live ingest writer + Yellowstone link state for readiness.
//!
//! Readiness must never persist `ready=true`. This process-local state is one
//! input, combined with DB reachability and integrity status on each probe.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};

use solana_proof_store::RunnerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestTerminal {
    Cancelled,
    RecoveryRequired {
        reason: String,
    },
    IntegrityHalted {
        reason: String,
    },
    SourceFailed {
        reason: String,
    },
    StoreFailed {
        reason: String,
    },
    /// Writer task panicked or missed an orderly `mark_finished`.
    Crashed {
        reason: String,
    },
}

/// Yellowstone link lifecycle for readiness.
///
/// `Connected` means at least one block was Applied or AlreadyApplied on the
/// live stream (subscription + durable cursor continuity proven). A bare gRPC
/// subscribe is still `Connecting`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SourceLinkState {
    /// Writer task has not started.
    Idle,
    /// Writer is running but has not yet proven cursor continuity via progress,
    /// or is reconnecting after a disconnect before the next apply/replay.
    Connecting,
    /// At least one Applied / AlreadyApplied observed; filtered-stream idle is OK.
    Connected,
    /// Stream dropped after a prior Connected; reconnect in progress.
    Disconnected,
}

#[derive(Debug)]
struct Inner {
    last_slot: Option<u64>,
    terminal: Option<IngestTerminal>,
    source_link: SourceLinkState,
}

#[derive(Debug)]
pub struct IngestHealth {
    writer_running: AtomicBool,
    recovery_in_progress: AtomicBool,
    inner: Mutex<Inner>,
}

impl IngestHealth {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            writer_running: AtomicBool::new(false),
            recovery_in_progress: AtomicBool::new(false),
            inner: Mutex::new(Inner {
                last_slot: None,
                terminal: None,
                source_link: SourceLinkState::Idle,
            }),
        })
    }

    /// Writer task entered the ingest loop (not yet continuity-proven).
    pub fn mark_started(&self) {
        self.writer_running.store(true, Ordering::SeqCst);
        self.recovery_in_progress.store(false, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.terminal = None;
        inner.source_link = SourceLinkState::Connecting;
    }

    /// Gates readiness to `recovery_required` while bounded RPC recovery runs.
    pub fn set_recovery_in_progress(&self, active: bool) {
        self.recovery_in_progress.store(active, Ordering::SeqCst);
    }

    pub fn recovery_in_progress(&self) -> bool {
        self.recovery_in_progress.load(Ordering::SeqCst)
    }

    /// Live stream dropped; reconnect/backoff is in progress.
    pub fn mark_disconnected(&self) {
        let mut inner = self.inner.lock().expect("ingest health lock");
        if matches!(inner.source_link, SourceLinkState::Connected) {
            inner.source_link = SourceLinkState::Disconnected;
        } else {
            inner.source_link = SourceLinkState::Connecting;
        }
    }

    /// Applied or exact-replay no-op: subscription + cursor continuity proven.
    pub fn mark_progress(&self, slot: u64) {
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.last_slot = Some(slot);
        inner.source_link = SourceLinkState::Connected;
    }

    pub fn mark_finished(&self, result: Result<(), RunnerError>) {
        self.writer_running.store(false, Ordering::SeqCst);
        self.recovery_in_progress.store(false, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.source_link = SourceLinkState::Idle;
        inner.terminal = Some(match result {
            Ok(()) => IngestTerminal::Cancelled,
            Err(RunnerError::RecoveryRequired(reason)) => {
                IngestTerminal::RecoveryRequired { reason }
            }
            Err(RunnerError::IntegrityHalted(reason)) => IngestTerminal::IntegrityHalted { reason },
            Err(RunnerError::Source(err)) => IngestTerminal::SourceFailed {
                reason: err.to_string(),
            },
            Err(RunnerError::Store(err)) => IngestTerminal::StoreFailed {
                reason: err.to_string(),
            },
        });
    }

    /// Unexpected writer exit (panic / missed finish / shutdown deadline).
    pub fn mark_crashed(&self, reason: impl Into<String>) {
        self.writer_running.store(false, Ordering::SeqCst);
        self.recovery_in_progress.store(false, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.source_link = SourceLinkState::Idle;
        inner.terminal = Some(IngestTerminal::Crashed {
            reason: reason.into(),
        });
    }

    pub fn writer_running(&self) -> bool {
        self.writer_running.load(Ordering::SeqCst)
    }

    pub fn source_link(&self) -> SourceLinkState {
        self.inner.lock().expect("ingest health lock").source_link
    }

    pub fn terminal(&self) -> Option<IngestTerminal> {
        self.inner
            .lock()
            .expect("ingest health lock")
            .terminal
            .clone()
    }

    pub fn last_slot(&self) -> Option<u64> {
        self.inner.lock().expect("ingest health lock").last_slot
    }
}
