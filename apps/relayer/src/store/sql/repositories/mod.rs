pub mod block_number_repo;
pub mod health;
pub mod input_proof_repo;
pub mod public_decrypt_repo;
pub mod timeout_repo;
pub mod user_decrypt_repo;
pub mod utils;

use super::client::PgClient;
use crate::config::settings::StorageConfig;
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

    // Internal fields for health checking
    pg_client: Arc<PgClient>,
    health_timeout: Duration,
}

impl Repositories {
    /// Create all repositories from storage configuration.
    pub async fn new(config: StorageConfig) -> Self {
        let health_timeout = Duration::from_secs(config.sql_health_check_timeout_secs);
        let pg_client = Arc::new(PgClient::new(config).await);

        Self {
            input_proof: Arc::new(InputProofRepository::new((*pg_client).clone())),
            public_decrypt: Arc::new(PublicDecryptRepository::new((*pg_client).clone())),
            user_decrypt: Arc::new(UserDecryptRepository::new((*pg_client).clone())),
            block_number: Arc::new(BlockNumberRepository::new((*pg_client).clone())),
            pg_client,
            health_timeout,
        }
    }
}
