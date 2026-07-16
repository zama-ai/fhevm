//! Live ingest heartbeat and recovery/halt classification for readiness.
//!
//! Readiness must never persist `ready=true`. This process-local state is one
//! input, combined with DB reachability and integrity status on each probe.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, Mutex};
use std::time::{Duration, Instant};

use solana_proof_store::RunnerError;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum IngestTerminal {
    Cancelled,
    RecoveryRequired { reason: String },
    IntegrityHalted { reason: String },
    SourceFailed { reason: String },
    StoreFailed { reason: String },
}

#[derive(Debug)]
struct Inner {
    last_progress_at: Option<Instant>,
    last_slot: Option<u64>,
    terminal: Option<IngestTerminal>,
}

#[derive(Debug)]
pub struct IngestHealth {
    writer_running: AtomicBool,
    inner: Mutex<Inner>,
}

impl IngestHealth {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            writer_running: AtomicBool::new(false),
            inner: Mutex::new(Inner {
                last_progress_at: None,
                last_slot: None,
                terminal: None,
            }),
        })
    }

    pub fn mark_started(&self) {
        self.writer_running.store(true, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.terminal = None;
        // Do not advance last_progress_at here: Ready requires a real apply
        // (mark_progress). Start alone must not look like ingest progress.
    }

    pub fn mark_progress(&self, slot: u64) {
        let mut inner = self.inner.lock().expect("ingest health lock");
        inner.last_progress_at = Some(Instant::now());
        inner.last_slot = Some(slot);
    }

    pub fn mark_finished(&self, result: Result<(), RunnerError>) {
        self.writer_running.store(false, Ordering::SeqCst);
        let mut inner = self.inner.lock().expect("ingest health lock");
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

    pub fn writer_running(&self) -> bool {
        self.writer_running.load(Ordering::SeqCst)
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

    pub fn silence_exceeded(&self, max_silence: Duration) -> bool {
        let inner = self.inner.lock().expect("ingest health lock");
        match inner.last_progress_at {
            Some(at) => at.elapsed() > max_silence,
            None => true,
        }
    }
}
