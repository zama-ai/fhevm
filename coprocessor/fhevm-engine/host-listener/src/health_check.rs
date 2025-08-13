use std::sync::Arc;
use std::time::Duration;

use tokio::sync::RwLock;

use fhevm_engine_common::healthz_server::{
    default_get_version, HealthCheckService, HealthStatus, Version,
};
use fhevm_engine_common::types::BlockchainProvider;

const IS_ALIVE_TICK_FRESHNESS: Duration = Duration::from_secs(20); // Not alive if tick is older
const CONNECTED_TICK_FRESHNESS: Duration = Duration::from_secs(5); // Need to check connection if tick is older

/// Represents the health status of the host-listener service
#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub blockchain_timeout_tick: Tick,
    pub blockchain_tick: Tick,
    pub blockchain_provider: Arc<RwLock<Option<BlockchainProvider>>>,
    pub database_pool: Arc<RwLock<sqlx::Pool<sqlx::Postgres>>>,
    pub database_tick: Tick,
}

#[derive(Clone, Debug)]
pub struct Tick(Arc<RwLock<std::time::Instant>>);

impl Tick {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(std::time::Instant::now())))
    }

    pub async fn update(&self) {
        *self.0.write().await = std::time::Instant::now();
    }

    pub async fn is_recent(&self, freshness: &Duration) -> bool {
        &self.0.read().await.elapsed() < freshness
    }
}

impl Default for Tick {
    fn default() -> Self {
        Self::new()
    }
}

impl HealthCheckService for HealthCheck {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        // service inner loop
        let check_alive = self.is_alive().await;
        status.set_custom_check("alive", check_alive, false);
        // blockchain
        if self
            .blockchain_tick
            .is_recent(&CONNECTED_TICK_FRESHNESS)
            .await
        {
            status.set_custom_check("blockchain_provider", true, true);
        } else if let Some(provider) =
            (*self.blockchain_provider.read().await).clone()
        {
            // cloned to ensure the service is not blocked during the IO
            status.set_blockchain_connected(&provider).await;
        } else {
            // the provider is being replaced, let's make it visible
            status.set_custom_check("blockchain_provider", false, true);
        };
        // database
        if self
            .database_tick
            .is_recent(&CONNECTED_TICK_FRESHNESS)
            .await
        {
            status.set_custom_check("database", true, true);
        } else {
            // cloned to ensure the service is not blocked during the IO
            let pool = self.database_pool.read().await.clone();
            status.set_db_connected(&pool).await;
        };
        status
    }

    async fn is_alive(&self) -> bool {
        self.blockchain_tick
            .is_recent(&IS_ALIVE_TICK_FRESHNESS)
            .await
            || self
                .blockchain_timeout_tick
                .is_recent(&IS_ALIVE_TICK_FRESHNESS)
                .await
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}
