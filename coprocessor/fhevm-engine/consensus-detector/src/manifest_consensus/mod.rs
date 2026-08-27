//! Generation-scoped manifest publication.
//!
//! This module is deliberately isolated from the upgrade state-hash detector.
//! Database access that crosses the active GCS schema boundary belongs under
//! [`storage`].

use std::sync::{
    atomic::{AtomicI64, Ordering},
    Arc,
};
use std::time::Duration;
use thiserror::Error;

use aws_sdk_s3::Client;
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::error;
use tracing::info;

use fhevm_engine_common::{
    gcs_activation::GCS_NOT_ACTIVATED,
    versioning::{reconcile_stack_mode, run_stack_version_listener, StackMode},
};

pub(crate) mod lineage;
pub(crate) mod manifest_archive;
pub(crate) mod publication;
pub(crate) mod storage;

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub(crate) enum ExecutionError {
    #[error("database error: {0}")]
    DbError(#[from] sqlx::Error),

    #[error("serialization error: {0}")]
    SerializationError(String),

    #[error("deserialization error: {0}")]
    DeserializationError(String),

    #[error("S3 transient error: {0}")]
    S3TransientError(String),

    #[error("internal error: {0}")]
    InternalError(String),
}

impl From<ExecutionError> for fhevm_engine_common::pg_pool::ServiceError {
    fn from(error: ExecutionError) -> Self {
        match error {
            ExecutionError::DbError(error) => Self::Database(error),
            error => Self::InternalError(error.to_string()),
        }
    }
}

/// Publication policy owned by consensus-detector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    pub publication_retry_delay: Duration,
    pub publication_retry_count: u32,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            publication_retry_delay: Duration::from_secs(60),
            publication_retry_count: 30,
        }
    }
}

/// Runtime gate for manifest publication.
///
/// Blue works immediately. Green stays parked until `DryRunStarted`, parks
/// again after rollback, and continues as the live stack after cutover. The
/// retired Blue stack is also fenced once the live stack version changes.
pub(crate) struct ManifestWorkGate {
    mode: Arc<StackMode>,
    active_generation: Arc<AtomicI64>,
}

impl ManifestWorkGate {
    fn new(gcs_mode: bool, active_generation: i64) -> Arc<Self> {
        Arc::new(Self {
            mode: StackMode::new(gcs_mode),
            active_generation: Arc::new(AtomicI64::new(active_generation)),
        })
    }

    /// Returns whether work pinned to `generation` may still make progress.
    ///
    /// Comparing the generation, rather than merely checking that some
    /// generation is active, fences an in-flight loop when Green is rapidly
    /// reactivated for a later upgrade window.
    pub(crate) fn work_enabled_for(&self, generation: i64) -> bool {
        generation != GCS_NOT_ACTIVATED
            && !self.mode.is_paused()
            && self.active_generation.load(Ordering::SeqCst) == generation
    }

    pub(crate) fn pinned_generation(&self) -> Option<i64> {
        if self.mode.is_paused() {
            return None;
        }
        let generation = self.active_generation.load(Ordering::SeqCst);
        (generation != GCS_NOT_ACTIVATED).then_some(generation)
    }

    #[cfg(test)]
    pub(crate) fn always_enabled() -> Arc<Self> {
        Self::new(false, 0)
    }
}

pub(crate) async fn start(
    config: &crate::Config,
    pool: PgPool,
    client: Arc<Client>,
    cancel: CancellationToken,
) -> Result<(), ExecutionError> {
    if config.my_bucket.is_none() {
        tracing::warn!("Manifest publication disabled by --my-bucket=none");
        return Ok(());
    }

    let work_gate = ManifestWorkGate::new(config.gcs_mode, GCS_NOT_ACTIVATED);
    // `resolve_gcs_mode` classifies both the current Blue stack and an old,
    // restarted Blue binary as non-GCS. Reconcile against the durable live
    // stack version before the manifest worker starts so the latter is
    // parked even though it could not have received the cutover notification.
    reconcile_stack_mode(&pool, &work_gate.mode)
        .await
        .map_err(|error| {
            ExecutionError::InternalError(format!(
                "failed to reconcile manifest stack version at startup: {error}"
            ))
        })?;
    if !config.gcs_mode && !work_gate.mode.is_paused() {
        let generation = storage::active::load_validated_generation(&pool).await?;
        work_gate
            .active_generation
            .store(generation, Ordering::SeqCst);
        info!(generation, "Pinned active manifest generation at startup");
    }
    {
        let listener_pool = pool.clone();
        let listener_mode = Arc::clone(&work_gate.mode);
        let listener_cancel = cancel.child_token();
        tokio::spawn(async move {
            if let Err(error) =
                run_stack_version_listener(listener_pool, listener_mode, listener_cancel).await
            {
                error!(%error, "manifest stack-version listener exited");
            }
        });
    }
    if config.gcs_mode {
        let watcher_pool = pool.clone();
        let watcher_state = Arc::clone(&work_gate.active_generation);
        let watcher_cancel = cancel.child_token();
        tokio::spawn(async move {
            loop {
                tokio::select! {
                    _ = watcher_cancel.cancelled() => return,
                    result = storage::active::run_gcs_active_generation_watcher(
                        &watcher_pool,
                        &watcher_state,
                    ) => {
                        if let Err(error) = result {
                            watcher_state.store(GCS_NOT_ACTIVATED, Ordering::SeqCst);
                            error!(%error, "manifest GCS activation watcher failed; restarting");
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        });
        info!("Green manifest publication parked until DryRunStarted");
    }

    if let Some(interval_secs) = config.gauge_update_interval_secs {
        let period = Duration::from_secs(interval_secs.into());
        publication::metrics::spawn_publication_gauge_updates(
            period,
            pool.clone(),
            Arc::clone(&work_gate),
        );
    }

    let signer = config.manifest_signer.clone().ok_or_else(|| {
        ExecutionError::InternalError("manifest signer is not configured".to_owned())
    })?;
    let handle = publication::publisher::spawn_manifest_publisher(
        pool.clone(),
        cancel.child_token(),
        config.clone(),
        Arc::clone(&client),
        signer,
        Arc::clone(&work_gate),
    );
    supervise("manifest publisher", handle, cancel.clone());

    Ok(())
}

fn supervise(
    task: &'static str,
    handle: JoinHandle<Result<(), ExecutionError>>,
    parent_cancel: CancellationToken,
) {
    tokio::spawn(async move {
        match handle.await {
            Ok(Ok(())) if parent_cancel.is_cancelled() => {}
            Ok(Ok(())) => error!(task, "manifest-consensus task stopped unexpectedly"),
            Ok(Err(error)) => error!(task, %error, "manifest-consensus task failed"),
            Err(error) => error!(task, %error, "manifest-consensus task panicked"),
        }
        parent_cancel.cancel();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn green_manifest_work_waits_for_activation_and_repauses_on_rollback() {
        let gate = ManifestWorkGate::new(true, GCS_NOT_ACTIVATED);
        assert!(!gate.work_enabled_for(7));

        gate.active_generation.store(7, Ordering::SeqCst);
        assert!(gate.work_enabled_for(7));

        gate.active_generation
            .store(GCS_NOT_ACTIVATED, Ordering::SeqCst);
        assert!(!gate.work_enabled_for(7));
    }

    #[test]
    fn rapid_green_reactivation_cannot_reuse_the_previous_generation() {
        let gate = ManifestWorkGate::new(true, GCS_NOT_ACTIVATED);

        gate.active_generation.store(7, Ordering::SeqCst);
        gate.active_generation
            .store(GCS_NOT_ACTIVATED, Ordering::SeqCst);
        // No manifest poll observes the parked state before the next window.
        gate.active_generation.store(8, Ordering::SeqCst);

        assert_eq!(gate.pinned_generation(), Some(8));
        assert!(!gate.work_enabled_for(7));
        assert!(gate.work_enabled_for(8));
    }

    #[test]
    fn blue_manifest_work_is_enabled_without_an_upgrade_window() {
        assert!(ManifestWorkGate::new(false, 0).work_enabled_for(0));
    }
}
