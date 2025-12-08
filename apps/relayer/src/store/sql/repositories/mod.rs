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
    store::sql::repositories::{
        cron_task::{spawn_expiry_worker, spawn_timeout_worker},
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
    pub async fn new(config: StorageConfig) -> Self {
        let health_timeout = Duration::from_secs(config.sql_health_check_timeout_secs);
        let pg_client = Arc::new(PgClient::new(config).await);

        pg_client.spawn_db_pool_monitor();

        Self {
            input_proof: Arc::new(InputProofRepository::new((*pg_client).clone())),
            public_decrypt: Arc::new(PublicDecryptRepository::new((*pg_client).clone())),
            user_decrypt: Arc::new(UserDecryptRepository::new((*pg_client).clone())),
            block_number: Arc::new(BlockNumberRepository::new((*pg_client).clone())),
            timeout_repo: Arc::new(TimeoutRepository::new((*pg_client).clone())),
            expiry_repo: Arc::new(ExpiryRepository::new((*pg_client).clone())),
            pg_client,
            health_timeout,
        }
    }

    pub fn start_background_workers(
        &self,
        timeout_cron_interval_secs: Duration,
        expiry_cron_interval_secs: Duration,
    ) {
        // We use the internal pg_client to spawn workers
        spawn_timeout_worker((*self.pg_client).clone(), timeout_cron_interval_secs);
        spawn_expiry_worker((*self.pg_client).clone(), expiry_cron_interval_secs);
    }
}
