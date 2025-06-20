use std::time::{SystemTime, UNIX_EPOCH};
use std::sync::Arc;

use tokio::sync::RwLock;

use fhevm_engine_common::healthz_server::{HealthCheckService, HealthStatus, Version};
use fhevm_engine_common::types::BlockchainProvider;

/// Represents the health status of the transaction sender service
#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub blockchain_timeout_tick: Tick,
    pub blockchain_tick: Tick,
    pub blockchain_provider: Arc<RwLock<Option<BlockchainProvider>>>,
    pub database_pool: Arc<RwLock<sqlx::Pool<sqlx::Postgres>>>,
    pub database_tick: Tick,
}

#[derive(Clone, Debug)]
pub struct Tick(Arc<RwLock<u64>>);

impl Tick {
    pub fn new() -> Self {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let now = if let Ok(now) = now {
            now.as_secs()
        } else {
            0
        };
        Self(Arc::new(RwLock::new(now)))
    }

    pub async fn update(&self) {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let Ok(now) = now else {
            return;
        };
        *self.0.write().await = now.as_secs();
    }

    pub async fn is_recent(&self, seconds: u64) -> bool {
        let now = SystemTime::now().duration_since(UNIX_EPOCH);
        let Ok(now) = now else {
            return false;
        };
        let tick = self.0.read().await;
        now.as_secs() - *tick < seconds
    }
}


impl HealthCheckService for HealthCheck {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        // service inner loop
        let check_alive = self.is_alive().await;
        status.checks.insert("alive", check_alive);
        if self.blockchain_tick.is_recent(5).await {
            status.checks.insert("blockchain_provider", true);
        } else if let Some(provider) = self.blockchain_provider.read().await.as_ref() {
            status.set_blockchain_connected(provider).await;
        } else {
            status.checks.insert("blockchain_provider", false);
        };
        if self.database_tick.is_recent(30).await {
            status.checks.insert("database", true);
        } else {
            // clone to ensure the service is not blocked if overwriting a new pool
            let pool = self.database_pool.read().await.clone();
            status.set_db_connected(&pool).await;
        };
        status
    }

    async fn is_alive(&self) -> bool {
        self.blockchain_tick.is_recent(20).await || self.blockchain_timeout_tick.is_recent(20).await
    }

    fn get_version(&self) -> Version {
        Version {
            name: env!("CARGO_PKG_NAME"),
            version: env!("CARGO_PKG_VERSION"),
            build: option_env!("BUILD_ID").unwrap_or("unknown"),
        }
    }
}
