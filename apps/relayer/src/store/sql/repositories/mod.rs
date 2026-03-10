pub mod block_number_repo;
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
    orchestrator::Orchestrator,
    store::sql::repositories::{
        cron_task::{create_expiry_worker_future, create_timeout_worker_future},
        expiry_repo::ExpiryRepository,
        timeout_repo::TimeoutRepository,
    },
};
use block_number_repo::BlockNumberRepository;
use input_proof_repo::InputProofRepository;
use public_decrypt_repo::PublicDecryptRepository;
use std::{sync::Arc, time::Duration};
use user_decrypt_repo::UserDecryptRepository;

/// Centralized container for all SQL repositories.
///
/// Provides a single initialization point for all repositories from storage configuration,
/// reducing parameter passing and simplifying dependency management.
pub struct Repositories {
    pub input_proof: Arc<InputProofRepository>,
    pub public_decrypt: Arc<PublicDecryptRepository>,
    pub user_decrypt: Arc<UserDecryptRepository>,
    pub block_number: Arc<BlockNumberRepository>,
    pub timeout_repo: Arc<TimeoutRepository>,
    pub expiry_repo: Arc<ExpiryRepository>,

    // Internal fields for health checking
    pg_client: Arc<PgClient>,
    health_timeout: Duration,
}

impl Repositories {
    /// Create all repositories from storage configuration.
    pub async fn new(config: StorageConfig) -> anyhow::Result<Self> {
        let health_timeout = Duration::from_secs(config.sql_health_check_timeout_secs);
        let pg_client = Arc::new(PgClient::new(config.clone()).await?);

        Ok(Self {
            input_proof: Arc::new(InputProofRepository::new((*pg_client).clone())),
            public_decrypt: Arc::new(PublicDecryptRepository::new((*pg_client).clone())),
            user_decrypt: Arc::new(UserDecryptRepository::new((*pg_client).clone())),
            block_number: Arc::new(BlockNumberRepository::new((*pg_client).clone())),
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

    /// Register all background workers with the orchestrator for proper lifecycle management
    pub async fn register_background_workers(
        &self,
        orchestrator: &Arc<Orchestrator>,
        cron_config: crate::config::settings::CronConfig,
    ) -> anyhow::Result<()> {
        // Register timeout worker
        orchestrator
            .spawn_task_and_wait_ready(
                "timeout_worker",
                create_timeout_worker_future((*self.pg_client).clone(), cron_config.clone()),
                async { Ok(()) }, // Ready immediately
            )
            .await?;

        // Register expiry worker
        orchestrator
            .spawn_task_and_wait_ready(
                "expiry_worker",
                create_expiry_worker_future((*self.pg_client).clone(), cron_config.clone()),
                async { Ok(()) }, // Ready immediately
            )
            .await?;

        Ok(())
    }
}
