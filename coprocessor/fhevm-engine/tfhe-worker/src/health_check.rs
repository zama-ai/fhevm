use std::{
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
    time::Duration,
};

use fhevm_engine_common::database::connect_options_for_database_url;
use fhevm_engine_common::healthz_server::{
    default_get_version, HealthCheckService, HealthStatus, Version,
};
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};

const ACTIVITY_FRESHNESS: Duration = Duration::from_secs(10); // Not alive if tick is older
const CONNECTED_TICK_FRESHNESS: Duration = Duration::from_secs(5); // Need to check connection if tick is older

/// Represents the health status of the transaction sender service
#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub database_url: DatabaseURL,
    pub database_heartbeat: HeartBeat,
    pub activity_heartbeat: HeartBeat,
    in_flight_work: Arc<AtomicUsize>,
}

#[derive(Debug)]
pub struct InFlightWorkGuard {
    in_flight_work: Arc<AtomicUsize>,
}

impl Drop for InFlightWorkGuard {
    fn drop(&mut self) {
        self.in_flight_work.fetch_sub(1, Ordering::AcqRel);
    }
}

impl HealthCheck {
    pub fn new(database_url: DatabaseURL) -> Self {
        // A lazy pool is used to avoid blocking the main thread during initialization or bad database URL
        Self {
            database_url,
            database_heartbeat: HeartBeat::new(),
            activity_heartbeat: HeartBeat::new(),
            in_flight_work: Arc::new(AtomicUsize::new(0)),
        }
    }

    pub fn update_db_access(&self) {
        self.database_heartbeat.update();
    }

    pub fn update_activity(&self) {
        self.activity_heartbeat.update();
    }

    /// Keeps liveness positive while a worker batch is actively executing.
    /// Whole transactions can legitimately run longer than the heartbeat
    /// freshness window, so completion-only heartbeats are insufficient.
    pub fn begin_work(&self) -> InFlightWorkGuard {
        self.in_flight_work.fetch_add(1, Ordering::AcqRel);
        InFlightWorkGuard {
            in_flight_work: Arc::clone(&self.in_flight_work),
        }
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
            match connect_options_for_database_url(&self.database_url).await {
                Ok(connect_options) => {
                    let pool = sqlx::postgres::PgPoolOptions::new()
                        .acquire_timeout(Duration::from_secs(5))
                        .max_connections(1)
                        .connect_with(connect_options)
                        .await;
                    if let Ok(pool) = pool {
                        status.set_db_connected(&pool).await;
                    } else {
                        status.set_custom_check("database", false, true);
                    }
                }
                Err(_) => {
                    status.set_custom_check("database", false, true);
                }
            }
        };
        status
    }

    async fn is_alive(&self) -> bool {
        self.in_flight_work.load(Ordering::Acquire) > 0
            || self.activity_heartbeat.is_recent(&ACTIVITY_FRESHNESS)
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn stale_health_check() -> HealthCheck {
        HealthCheck {
            database_url: DatabaseURL::default(),
            database_heartbeat: HeartBeat::with_elapsed_secs(30),
            activity_heartbeat: HeartBeat::with_elapsed_secs(30),
            in_flight_work: Arc::new(AtomicUsize::new(0)),
        }
    }

    #[tokio::test]
    async fn in_flight_work_keeps_worker_alive_until_guard_drops() {
        let health_check = stale_health_check();
        assert!(!health_check.is_alive().await);

        let guard = health_check.begin_work();
        assert!(health_check.is_alive().await);

        drop(guard);
        assert!(!health_check.is_alive().await);
    }
}
