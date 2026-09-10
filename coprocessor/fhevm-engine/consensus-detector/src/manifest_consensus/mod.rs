//! Generation-scoped manifest publication and peer verification.
//!
//! This module is deliberately isolated from the upgrade state-hash detector.
//! Database access that crosses the active GCS schema boundary belongs under
//! the private `storage` module.

use std::collections::BTreeMap;
use std::sync::{Arc, RwLock};
use std::time::Duration;
use thiserror::Error;

use aws_sdk_s3::Client;
use sqlx::PgPool;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use tracing::error;
use tracing::info;

#[cfg(test)]
use block_manifest::LEGACY_CONSENSUS_EPOCH;
use fhevm_engine_common::versioning::{
    reconcile_stack_mode, run_stack_version_listener, StackMode,
};

pub(crate) mod db_error;
pub(crate) mod lineage;
pub(crate) mod manifest_archive;
pub(crate) mod publication;
pub(crate) mod storage;
pub(crate) mod verification;

pub use publication::block_discovery::{
    parse_publication_cadence_override, publication_cadence_overrides,
};

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

    #[error("S3 object not found: {0}")]
    S3ObjectNotFound(String),

    #[error("peer manifest exceeds the size limit: {0}")]
    PeerManifestTooLarge(String),

    #[error("internal error: {0}")]
    InternalError(String),

    /// Live descriptors no longer match an unpublished seal. The seal is
    /// discarded and the block is resealed on a later pass.
    #[error(
        "sealed block content is stale for chain {host_chain_id} block {block_number}: {reason}"
    )]
    StaleBlockSeal {
        host_chain_id: i64,
        block_number: i64,
        reason: String,
    },

    /// A missing in-generation parent has producer inventory and was inserted
    /// unsealed, or is already tracked without a digest. Publication waits so
    /// `lock_next` can seal it as ordinary work. The child's unpublished seal
    /// is kept and this does not consume the child's publication retry budget.
    #[error(
        "publication is waiting for unsealed predecessor on chain {host_chain_id} block {block_number}"
    )]
    PredecessorUnsealed {
        host_chain_id: i64,
        block_number: i64,
    },
}

impl From<ExecutionError> for fhevm_engine_common::pg_pool::ServiceError {
    fn from(error: ExecutionError) -> Self {
        match error {
            ExecutionError::DbError(error) => Self::Database(error),
            error => Self::InternalError(error.to_string()),
        }
    }
}

/// Publication and verification policy owned by consensus-detector.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Config {
    /// Publisher tick: discover host blocks, then seal and publish already
    /// tracked work. Must be greater than zero.
    pub discovery_interval: Duration,
    pub publication_retry_delay: Duration,
    pub publication_retry_count: u32,
    pub verification_delay: Duration,
    pub verification_retry_delay: Duration,
    pub verification_retry_count: u32,
    /// Wall-clock stall with no newly computed handle before missing
    /// ciphertext may be sealed as `is_uncomputed`.
    pub incomplete_block_timeout: Duration,
    /// Host-chain lag, in due manifests (publication periods), before missing
    /// ciphertext may be sealed as `is_uncomputed`.
    pub incomplete_manifest_max_lag: u32,
    /// Insert-time cadence overlays (`chain_id → K`). Unlisted chains use the
    /// built-in table (Ethereum mainnet/Sepolia/Hoodi 5, Polygon/Amoy and
    /// unknown 30). Existing rows keep the cadence stored at insert.
    pub publication_cadence_overrides: BTreeMap<i64, i64>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            discovery_interval: Duration::from_secs(10),
            publication_retry_delay: Duration::from_secs(60),
            publication_retry_count: 30,
            verification_delay: Duration::from_secs(5 * 60),
            verification_retry_delay: Duration::from_secs(60),
            verification_retry_count: 5,
            incomplete_block_timeout: Duration::from_secs(5 * 60),
            incomplete_manifest_max_lag: 3,
            publication_cadence_overrides: BTreeMap::new(),
        }
    }
}

impl Config {
    pub(crate) fn incomplete_seal_lag_blocks(&self, publication_cadence: i64) -> Option<i64> {
        let periods = i64::from(self.incomplete_manifest_max_lag);
        if publication_cadence <= 0 || periods <= 0 {
            return None;
        }
        publication_cadence.checked_mul(periods)
    }

    pub(crate) fn incomplete_block_timeout_secs(&self) -> Option<i64> {
        if self.incomplete_block_timeout.is_zero() {
            return None;
        }
        i64::try_from(self.incomplete_block_timeout.as_secs())
            .ok()
            .map(|secs| secs.max(1))
    }
}

/// Runtime gate shared by publication and verification.
///
/// Blue works immediately. Green stays parked until `DryRunStarted`, parks
/// again after rollback, and continues as the live stack after cutover. The
/// retired Blue stack is also fenced once the live stack version changes.
pub(crate) struct ManifestWorkGate {
    mode: Arc<StackMode>,
    active_generation: Arc<RwLock<Option<String>>>,
}

impl ManifestWorkGate {
    fn new(gcs_mode: bool, active_generation: Option<String>) -> Arc<Self> {
        Arc::new(Self {
            mode: StackMode::new(gcs_mode),
            active_generation: Arc::new(RwLock::new(active_generation)),
        })
    }

    /// Returns whether work pinned to `generation` may still make progress.
    ///
    /// Comparing the generation, rather than merely checking that some
    /// generation is active, fences an in-flight loop when Green is rapidly
    /// reactivated for a later upgrade window.
    pub(crate) fn work_enabled_for(&self, generation: &str) -> bool {
        !self.mode.is_paused() && self.pinned_generation().as_deref() == Some(generation)
    }

    pub(crate) fn pinned_generation(&self) -> Option<String> {
        if self.mode.is_paused() {
            return None;
        }
        self.active_generation
            .read()
            .unwrap_or_else(|poisoned| poisoned.into_inner())
            .clone()
    }

    pub(crate) fn pin_generation(&self, generation: Option<String>) {
        *self
            .active_generation
            .write()
            .unwrap_or_else(|poisoned| poisoned.into_inner()) = generation;
    }

    #[cfg(test)]
    pub(crate) fn always_enabled() -> Arc<Self> {
        Self::new(false, Some(LEGACY_CONSENSUS_EPOCH.to_owned()))
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
        tracing::warn!("Manifest verification disabled by --my-bucket=none");
        return Ok(());
    }

    let work_gate = ManifestWorkGate::new(config.gcs_mode, None);
    // `resolve_gcs_mode` classifies both the current Blue stack and an old,
    // restarted Blue binary as non-GCS. Reconcile against the durable live
    // stack version before either manifest worker starts so the latter is
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
        work_gate.pin_generation(Some(generation.clone()));
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
                            *watcher_state
                                .write()
                                .unwrap_or_else(|poisoned| poisoned.into_inner()) = None;
                            error!(%error, "manifest GCS activation watcher failed; restarting");
                            tokio::time::sleep(Duration::from_secs(5)).await;
                        }
                    }
                }
            }
        });
        info!("Green manifest publication and verification parked until DryRunStarted");
    }

    if let Some(interval_secs) = config.gauge_update_interval_secs {
        let period = Duration::from_secs(interval_secs.into());
        publication::metrics::spawn_publication_gauge_updates(period, pool.clone());
        verification::metrics::spawn_verification_gauge_updates(period, pool.clone());
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

    let handle = verification::peer_downloader::spawn_peer_manifest_downloader(
        pool,
        cancel.child_token(),
        client,
        work_gate,
    );
    supervise("peer manifest verifier", handle, cancel);

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
        let gate = ManifestWorkGate::new(true, None);
        assert!(!gate.work_enabled_for("7"));

        gate.pin_generation(Some("7".to_owned()));
        assert!(gate.work_enabled_for("7"));

        gate.pin_generation(None);
        assert!(!gate.work_enabled_for("7"));
    }

    #[test]
    fn rapid_green_reactivation_cannot_reuse_the_previous_generation() {
        let gate = ManifestWorkGate::new(true, None);

        gate.pin_generation(Some("7".to_owned()));
        gate.pin_generation(None);
        // No manifest poll observes the parked state before the next window.
        gate.pin_generation(Some("8".to_owned()));

        assert_eq!(gate.pinned_generation().as_deref(), Some("8"));
        assert!(!gate.work_enabled_for("7"));
        assert!(gate.work_enabled_for("8"));
    }

    #[test]
    fn blue_manifest_work_is_enabled_without_an_upgrade_window() {
        assert!(
            ManifestWorkGate::new(false, Some(LEGACY_CONSENSUS_EPOCH.to_owned()))
                .work_enabled_for(LEGACY_CONSENSUS_EPOCH)
        );
    }
}
