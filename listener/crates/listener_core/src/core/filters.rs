use thiserror::Error;
use tracing::info;

use crate::store::SqlError;
use crate::store::models::Filter;
use crate::store::repositories::Repositories;

#[derive(Error, Debug)]
pub enum FilterError {
    #[error("Database error: {source}")]
    DatabaseError {
        #[source]
        source: SqlError,
    },
}

/// Manages filter lifecycle for a specific chain.
/// Wrap in `Arc` for sharing between handlers and evm_listener.
#[derive(Clone)]
pub struct Filters {
    repositories: Repositories,
    chain_id: u64,
}

impl Filters {
    pub fn new(repositories: Repositories, chain_id: u64) -> Self {
        Self {
            repositories,
            chain_id,
        }
    }

    /// Add a filter.
    pub async fn add_filter(
        &self,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
    ) -> Result<Option<Filter>, FilterError> {
        info!(
            chain_id = self.chain_id,
            consumer_id, from, to, log_address, "Adding filter"
        );
        self.repositories
            .filters
            .add_filter(consumer_id, from, to, log_address)
            .await
            .map_err(|source| FilterError::DatabaseError { source })
    }

    /// Remove a filter.
    pub async fn remove_filter(
        &self,
        consumer_id: &str,
        from: Option<&str>,
        to: Option<&str>,
        log_address: Option<&str>,
    ) -> Result<Option<Filter>, FilterError> {
        info!(
            chain_id = self.chain_id,
            consumer_id, from, to, log_address, "Removing filter"
        );
        self.repositories
            .filters
            .remove_filter(consumer_id, from, to, log_address)
            .await
            .map_err(|source| FilterError::DatabaseError { source })
    }
}
