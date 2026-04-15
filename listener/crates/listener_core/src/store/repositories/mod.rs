pub mod block_repo;
pub mod filter_repo;
use crate::store::client::PgClient;
pub use block_repo::BlockRepository;
pub use filter_repo::FilterRepository;
use std::sync::Arc;

/// Centralized container for all SQL repositories.
///
/// Provides a single initialization point for all repositories from database config,
/// reducing parameter passing and simplifying dependency management.
#[derive(Clone)]
pub struct Repositories {
    pub blocks: BlockRepository,
    pub filters: FilterRepository,
    chain_id: i64,
}

impl Repositories {
    /// Create all repositories from database configuration.
    pub fn new(client: Arc<PgClient>, chain_id: i64) -> Self {
        Self {
            blocks: BlockRepository::new(client.clone(), chain_id),
            filters: FilterRepository::new(client, chain_id),
            chain_id,
        }
    }

    pub fn chain_id(&self) -> i64 {
        self.chain_id
    }
}
