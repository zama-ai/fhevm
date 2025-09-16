use std::time::Duration;

use fhevm_engine_common::healthz_server::{
    default_get_version, HealthCheckService, HealthStatus, Version,
};
use fhevm_engine_common::utils::HeartBeat;

const ACTIVITY_FRESHNESS: Duration = Duration::from_secs(10); // Not alive if tick is older
const CONNECTED_TICK_FRESHNESS: Duration = Duration::from_secs(5); // Need to check connection if tick is older

/// Represents the health status of the transaction sender service
#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub database_url: String,
    pub database_heartbeat: HeartBeat,
    pub activity_heartbeat: HeartBeat,
}

impl HealthCheck {
    pub fn new(database_url: String) -> Self {
        // A lazy pool is used to avoid blocking the main thread during initialization or bad database URL
        Self {
            database_url,
            database_heartbeat: HeartBeat::new(),
            activity_heartbeat: HeartBeat::new(),
        }
    }

    pub fn update_db_access(&self) {
        self.database_heartbeat.update();
    }

    pub fn update_activity(&self) {
        self.activity_heartbeat.update();
    }
}

impl HealthCheckService for HealthCheck {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        // service inner loop
        let check_alive = self.is_alive().await;
        status.set_custom_check("alive", check_alive, false);
        if self.database_heartbeat.is_recent(&CONNECTED_TICK_FRESHNESS) {
            status.set_custom_check("database", true, true);
        } else {
            let pool = sqlx::postgres::PgPoolOptions::new()
                .acquire_timeout(Duration::from_secs(5))
                .max_connections(1)
                .connect(&self.database_url);
            if let Ok(pool) = pool.await {
                status.set_db_connected(&pool).await;
            } else {
                status.set_custom_check("database", false, true);
            }
        };
        status
    }

    async fn is_alive(&self) -> bool {
        self.activity_heartbeat.is_recent(&ACTIVITY_FRESHNESS)
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}
