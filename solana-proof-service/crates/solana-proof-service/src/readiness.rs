//! Derived live readiness for proof serving.
//!
//! Never stores `ready=true`. Combines database reachability, persisted
//! integrity / history completeness, and process-local Yellowstone link /
//! writer / recovery state on each probe.
//!
//! Readiness is the bootstrap / ingest gate. Per-request proof trust is
//! peak-equality against confirmed chain state (see `proof`), not this probe.

use std::sync::Arc;
use std::time::Duration;

use async_trait::async_trait;
use serde::Serialize;
use solana_proof_store::{IntegrityStatus, SqlProofStore, StoreError};
use utoipa::ToSchema;

use crate::ingest_health::{IngestHealth, IngestTerminal, SourceLinkState};

/// Hard ceiling for readiness DB reachability + integrity queries.
pub const READINESS_DB_TIMEOUT: Duration = Duration::from_secs(2);

/// DB reachability + integrity inputs for [`evaluate_readiness`].
#[async_trait]
pub trait ReadinessQueryable: Send + Sync {
    async fn database_reachable(&self) -> bool;
    async fn integrity_status(&self) -> Result<IntegrityStatus, StoreError>;
}

#[async_trait]
impl ReadinessQueryable for SqlProofStore {
    async fn database_reachable(&self) -> bool {
        sqlx::query_scalar::<_, i32>("SELECT 1")
            .fetch_one(self.pool())
            .await
            .is_ok()
    }

    async fn integrity_status(&self) -> Result<IntegrityStatus, StoreError> {
        SqlProofStore::integrity_status(self).await
    }
}

/// Bounded readiness classification labels (also used as metric labels).
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, ToSchema)]
#[serde(rename_all = "snake_case")]
pub enum ReadinessClass {
    Ready,
    DatabaseUnavailable,
    WriterMissing,
    SourceLagging,
    HistoryIncomplete,
    RecoveryRequired,
    IntegrityHalted,
}

impl ReadinessClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ready => "ready",
            Self::DatabaseUnavailable => "database_unavailable",
            Self::WriterMissing => "writer_missing",
            Self::SourceLagging => "source_lagging",
            Self::HistoryIncomplete => "history_incomplete",
            Self::RecoveryRequired => "recovery_required",
            Self::IntegrityHalted => "integrity_halted",
        }
    }

    pub fn is_ready(self) -> bool {
        matches!(self, Self::Ready)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, ToSchema)]
pub struct ReadinessReport {
    pub ready: bool,
    pub status: ReadinessClass,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub checkpoint_slot: Option<u64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub last_ingest_slot: Option<u64>,
}

/// Pure classification from already-fetched inputs (unit-testable without infra).
pub fn classify_readiness(
    db_ok: bool,
    integrity: Option<&IntegrityStatus>,
    ingest: &IngestHealth,
) -> ReadinessReport {
    if !db_ok {
        return report(
            ReadinessClass::DatabaseUnavailable,
            Some("database reachability check failed".to_owned()),
            None,
            ingest.last_slot(),
        );
    }

    let Some(integrity) = integrity else {
        return report(
            ReadinessClass::DatabaseUnavailable,
            Some("integrity status unavailable".to_owned()),
            None,
            ingest.last_slot(),
        );
    };

    let checkpoint_slot = integrity.checkpoint.as_ref().map(|c| c.slot);

    if integrity.integrity_halted {
        return report(
            ReadinessClass::IntegrityHalted,
            integrity.integrity_halt_reason.clone(),
            checkpoint_slot,
            ingest.last_slot(),
        );
    }

    if let Some(terminal) = ingest.terminal() {
        match terminal {
            IngestTerminal::RecoveryRequired { reason } => {
                return report(
                    ReadinessClass::RecoveryRequired,
                    Some(reason),
                    checkpoint_slot,
                    ingest.last_slot(),
                );
            }
            IngestTerminal::IntegrityHalted { reason } => {
                return report(
                    ReadinessClass::IntegrityHalted,
                    Some(reason),
                    checkpoint_slot,
                    ingest.last_slot(),
                );
            }
            IngestTerminal::Cancelled
            | IngestTerminal::SourceFailed { .. }
            | IngestTerminal::StoreFailed { .. }
            | IngestTerminal::Crashed { .. } => {
                return report(
                    ReadinessClass::WriterMissing,
                    Some(format!("ingest writer stopped: {terminal:?}")),
                    checkpoint_slot,
                    ingest.last_slot(),
                );
            }
        }
    }

    if !integrity.history_complete {
        return report(
            ReadinessClass::HistoryIncomplete,
            Some(
                "history_complete=false until bounded recovery proves continuity from start"
                    .to_owned(),
            ),
            checkpoint_slot,
            ingest.last_slot(),
        );
    }

    if !ingest.writer_running() {
        return report(
            ReadinessClass::WriterMissing,
            Some("ingest writer is not running".to_owned()),
            checkpoint_slot,
            ingest.last_slot(),
        );
    }

    match ingest.source_link() {
        SourceLinkState::Connected => report(
            ReadinessClass::Ready,
            None,
            checkpoint_slot,
            ingest.last_slot(),
        ),
        SourceLinkState::Connecting => report(
            ReadinessClass::SourceLagging,
            Some(
                "waiting for first applied or replayed block to prove Yellowstone continuity"
                    .to_owned(),
            ),
            checkpoint_slot,
            ingest.last_slot(),
        ),
        SourceLinkState::Disconnected => report(
            ReadinessClass::SourceLagging,
            Some("yellowstone stream disconnected; reconnecting".to_owned()),
            checkpoint_slot,
            ingest.last_slot(),
        ),
        SourceLinkState::Idle => report(
            ReadinessClass::WriterMissing,
            Some("ingest writer source link is idle".to_owned()),
            checkpoint_slot,
            ingest.last_slot(),
        ),
    }
}

fn report(
    status: ReadinessClass,
    reason: Option<String>,
    checkpoint_slot: Option<u64>,
    last_ingest_slot: Option<u64>,
) -> ReadinessReport {
    ReadinessReport {
        ready: status.is_ready(),
        status,
        reason,
        checkpoint_slot,
        last_ingest_slot,
    }
}

/// Live readiness probe against the store + ingest health.
pub async fn evaluate_readiness<S: ReadinessQueryable>(
    store: &S,
    ingest: &Arc<IngestHealth>,
) -> ReadinessReport {
    match tokio::time::timeout(READINESS_DB_TIMEOUT, evaluate_readiness_db(store, ingest)).await {
        Ok(report) => report,
        Err(_) => {
            tracing::warn!(
                timeout_secs = READINESS_DB_TIMEOUT.as_secs(),
                "readiness database probe timed out"
            );
            classify_readiness(false, None, ingest)
        }
    }
}

async fn evaluate_readiness_db<S: ReadinessQueryable>(
    store: &S,
    ingest: &Arc<IngestHealth>,
) -> ReadinessReport {
    if !store.database_reachable().await {
        return classify_readiness(false, None, ingest);
    }

    match store.integrity_status().await {
        Ok(status) => classify_readiness(true, Some(&status), ingest),
        Err(_) => classify_readiness(false, None, ingest),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use solana_proof_source::BlockCheckpoint;

    fn status(history_complete: bool, halted: bool) -> IntegrityStatus {
        IntegrityStatus {
            history_complete,
            history_start: Some(BlockCheckpoint {
                slot: 1,
                block_hash: [1u8; 32],
            }),
            checkpoint: Some(BlockCheckpoint {
                slot: 10,
                block_hash: [2u8; 32],
            }),
            integrity_halted: halted,
            integrity_halt_reason: halted.then(|| "conflict".to_owned()),
        }
    }

    #[test]
    fn database_unavailable_wins() {
        let ingest = IngestHealth::new();
        let report = classify_readiness(false, None, &ingest);
        assert_eq!(report.status, ReadinessClass::DatabaseUnavailable);
        assert!(!report.ready);
    }

    #[test]
    fn integrity_halt_before_history() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        let report = classify_readiness(true, Some(&status(false, true)), &ingest);
        assert_eq!(report.status, ReadinessClass::IntegrityHalted);
    }

    #[test]
    fn recovery_required_from_ingest_terminal() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        ingest.mark_finished(Err(solana_proof_store::RunnerError::RecoveryRequired(
            "gap".into(),
        )));
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::RecoveryRequired);
        assert_eq!(report.reason.as_deref(), Some("gap"));
    }

    #[test]
    fn history_incomplete_until_recovery_seam() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        let report = classify_readiness(true, Some(&status(false, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::HistoryIncomplete);
    }

    #[test]
    fn writer_missing_when_task_exits() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        ingest.mark_finished(Ok(()));
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::WriterMissing);
    }

    #[test]
    fn connecting_before_progress_is_source_lagging() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::SourceLagging);
        assert!(!report.ready);
    }

    #[test]
    fn ready_only_after_first_apply_or_replay_progress() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        let before = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(before.status, ReadinessClass::SourceLagging);

        ingest.mark_progress(42);
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::Ready);
        assert!(report.ready);
        assert_eq!(report.last_ingest_slot, Some(42));
    }

    #[test]
    fn ready_when_history_complete_progress_and_writer_live() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        ingest.mark_progress(42);
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::Ready);
        assert!(report.ready);
        assert_eq!(report.last_ingest_slot, Some(42));
    }

    #[test]
    fn disconnected_is_source_lagging() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        ingest.mark_progress(7);
        ingest.mark_disconnected();
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::SourceLagging);
        assert!(!report.ready);
    }

    #[test]
    fn crashed_writer_is_missing() {
        let ingest = IngestHealth::new();
        ingest.mark_started();
        ingest.mark_progress(1);
        ingest.mark_crashed("panic");
        let report = classify_readiness(true, Some(&status(true, false)), &ingest);
        assert_eq!(report.status, ReadinessClass::WriterMissing);
        assert!(!report.ready);
    }

    struct HangingStore;

    #[async_trait]
    impl ReadinessQueryable for HangingStore {
        async fn database_reachable(&self) -> bool {
            std::future::pending::<()>().await;
            true
        }

        async fn integrity_status(&self) -> Result<IntegrityStatus, StoreError> {
            unreachable!("should time out before integrity")
        }
    }

    #[tokio::test]
    async fn readiness_db_timeout_reports_database_unavailable() {
        // Temporarily shrink the timeout via the public constant path: we call
        // the inner evaluator through evaluate_readiness which uses the const.
        // Use a hanging store and assert the outer timeout fires within a few seconds.
        let ingest = IngestHealth::new();
        let started = std::time::Instant::now();
        let report = evaluate_readiness(&HangingStore, &ingest).await;
        assert!(started.elapsed() < Duration::from_secs(5));
        assert_eq!(report.status, ReadinessClass::DatabaseUnavailable);
        assert!(!report.ready);
    }
}
