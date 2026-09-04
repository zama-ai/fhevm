pub mod chain_cursor_repo;
pub mod cron_task;
pub mod expiry_repo;
pub mod health;
pub mod input_proof_repo;
pub mod public_decrypt_repo;
pub mod timeout_repo;
pub mod user_decrypt_repo;
pub mod utils;

use super::client::PgClient;
use crate::{
    config::settings::StorageConfig,
    orchestrator::{DispatchGate, DispatcherLock, Orchestrator},
    store::sql::repositories::{
        cron_task::{create_expiry_worker_future, create_timeout_worker_future},
        expiry_repo::ExpiryRepository,
        timeout_repo::TimeoutRepository,
    },
};
use chain_cursor_repo::ChainCursorRepository;
use input_proof_repo::InputProofRepository;
use public_decrypt_repo::PublicDecryptRepository;
use std::{sync::Arc, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::warn;
use user_decrypt_repo::UserDecryptRepository;

/// Centralized container for all SQL repositories.
///
/// Provides a single initialization point for all repositories from storage configuration,
/// reducing parameter passing and simplifying dependency management.
pub struct Repositories {
    pub input_proof: Arc<InputProofRepository>,
    pub public_decrypt: Arc<PublicDecryptRepository>,
    pub user_decrypt: Arc<UserDecryptRepository>,
    pub chain_cursor: Arc<ChainCursorRepository>,
    pub timeout_repo: Arc<TimeoutRepository>,
    pub expiry_repo: Arc<ExpiryRepository>,

    // Internal fields for health checking
    pg_client: Arc<PgClient>,
    health_timeout: Duration,
}

impl Repositories {
    /// Create all repositories from storage configuration. `dispatcher_lock` is the epoch
    /// source every request-row and chain-cursor write fences against - see the doc block on
    /// `public_decrypt_repo` for the rationale. Cheap to pass by value: `DispatcherLock`
    /// clones share one dedicated connection and state.
    pub async fn new(
        config: StorageConfig,
        dispatcher_lock: DispatcherLock,
    ) -> anyhow::Result<Self> {
        let health_timeout = Duration::from_secs(config.sql_health_check_timeout_secs);
        let pg_client = Arc::new(PgClient::new(config.clone()).await?);

        Ok(Self {
            input_proof: Arc::new(InputProofRepository::new(
                (*pg_client).clone(),
                dispatcher_lock.clone(),
            )),
            public_decrypt: Arc::new(PublicDecryptRepository::new(
                (*pg_client).clone(),
                dispatcher_lock.clone(),
            )),
            user_decrypt: Arc::new(UserDecryptRepository::new(
                (*pg_client).clone(),
                dispatcher_lock.clone(),
            )),
            chain_cursor: Arc::new(ChainCursorRepository::new(
                (*pg_client).clone(),
                dispatcher_lock,
            )),
            timeout_repo: Arc::new(TimeoutRepository::new(
                (*pg_client).clone(),
                config.cron.clone(),
            )),
            expiry_repo: Arc::new(ExpiryRepository::new(
                (*pg_client).clone(),
                config.cron.clone(),
            )),
            pg_client,
            health_timeout,
        })
    }

    /// Gracefully close underlying database pools.
    pub async fn close_pools(&self) {
        self.pg_client.close().await;
    }

    /// Register background workers with the orchestrator for proper lifecycle management.
    /// The timeout worker always starts; the expiry worker only starts when enabled.
    /// `shutdown` stops the workers and their panic-restart supervisor loops.
    pub async fn register_background_workers(
        &self,
        orchestrator: &Arc<Orchestrator>,
        cron_config: crate::config::settings::CronConfig,
        gate: DispatchGate,
        shutdown: CancellationToken,
    ) -> anyhow::Result<()> {
        orchestrator
            .spawn_task_and_wait_ready(
                "timeout_worker",
                create_timeout_worker_future(
                    (*self.pg_client).clone(),
                    cron_config.clone(),
                    gate.clone(),
                    shutdown.clone(),
                ),
                async { Ok(()) }, // Ready immediately
            )
            .await?;

        if cron_config.expiry_enabled {
            warn!(
                public_decrypt_expiry = ?cron_config.public_decrypt_expiry,
                user_decrypt_expiry = ?cron_config.user_decrypt_expiry,
                input_proof_expiry = ?cron_config.input_proof_expiry,
                "Expiry worker enabled — will DELETE rows older than configured retention windows"
            );

            orchestrator
                .spawn_task_and_wait_ready(
                    "expiry_worker",
                    create_expiry_worker_future(
                        (*self.pg_client).clone(),
                        cron_config.clone(),
                        gate,
                        shutdown,
                    ),
                    async { Ok(()) }, // Ready immediately
                )
                .await?;
        }

        Ok(())
    }
}
