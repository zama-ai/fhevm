use std::future::Future;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::{Arc, LazyLock};
use std::time::Duration;

use fhevm_engine_common::utils::HeartBeat;
use tokio::sync::RwLock;

use fhevm_engine_common::healthz_server::{
    default_get_version, HealthCheckService, HealthStatus, Version,
};
use fhevm_engine_common::types::BlockchainProvider;

static RECOVERY_PAUSED: LazyLock<prometheus::IntGauge> = LazyLock::new(|| {
    prometheus::register_int_gauge!(
        "host_listener_recovery_paused",
        "Listener paused for chain history recovery; requires audited recovery and restart"
    )
    .unwrap()
});

const IS_ALIVE_TICK_FRESHNESS: Duration = Duration::from_secs(20); // Not alive if tick is older
const CONNECTED_TICK_FRESHNESS: Duration = Duration::from_secs(5); // Need to check connection if tick is older

/// Represents the health status of the host-listener service
#[derive(Clone, Debug)]
pub struct HealthCheck {
    pub blockchain_timeout_tick: HeartBeat,
    pub blockchain_tick: HeartBeat,
    pub blockchain_provider: Arc<RwLock<Option<BlockchainProvider>>>,
    pub database_pool: Arc<RwLock<sqlx::Pool<sqlx::Postgres>>>,
    pub database_tick: HeartBeat,
    /// Latched until restart: clearing a durable halt must not resume cached work.
    pub recovery_paused: Arc<AtomicBool>,
}

impl HealthCheck {
    /// Observe containment before startup, while idle, and after a failed write.
    /// Drop ingestion before pausing so its pending transactions are rolled back.
    /// Recovery can replace cursors/history: only a fresh process may resume.
    pub(crate) async fn run_with_containment(
        &self,
        cancel: &tokio_util::sync::CancellationToken,
        work: impl Future<Output = anyhow::Result<()>>,
    ) -> anyhow::Result<()> {
        RECOVERY_PAUSED.set(0);
        let pool = self.database_pool.read().await.clone();
        let result =
            if fhevm_engine_common::chain_recovery::is_halted(&pool).await? {
                drop(work);
                None
            } else {
                tokio::pin!(work);
                let observe = async {
                    loop {
                        tokio::time::sleep(Duration::from_secs(5)).await;
                        let pool = self.database_pool.read().await.clone();
                        if fhevm_engine_common::chain_recovery::is_halted(&pool)
                            .await?
                        {
                            return Ok::<_, sqlx::Error>(());
                        }
                    }
                };
                // Keep polling ingestion while an observation waits for a DB
                // connection: ingestion may itself need to release that connection.
                tokio::select! {
                    biased;
                    _ = cancel.cancelled() => return Ok(()),
                    observed = observe => { observed?; None }
                    result = &mut work => Some(result),
                }
            };
        if let Some(result) = result {
            // Preserve ordinary errors (including malformed events). A write
            // may discover/record a halt between the periodic observations.
            let pool = self.database_pool.read().await.clone();
            if !fhevm_engine_common::chain_recovery::is_halted(&pool)
                .await
                .unwrap_or(false)
            {
                return result;
            }
        }
        self.recovery_paused.store(true, Ordering::Relaxed);
        RECOVERY_PAUSED.set(1);
        tracing::error!(
            event = "chain_recovery_paused",
            "Host listener paused by chain history containment; audited recovery and restart required"
        );
        let mut interval = tokio::time::interval(Duration::from_secs(5));
        loop {
            tokio::select! {
                _ = cancel.cancelled() => return Ok(()),
                _ = interval.tick() => self.blockchain_timeout_tick.update(),
            }
        }
    }
}

impl HealthCheckService for HealthCheck {
    async fn health_check(&self) -> HealthStatus {
        let mut status = HealthStatus::default();
        let pool = self.database_pool.read().await.clone();
        let recovery_ready = !self.recovery_paused.load(Ordering::Relaxed)
            && matches!(
                fhevm_engine_common::chain_recovery::readiness_blocked(&pool)
                    .await,
                Ok(false)
            );
        status.set_custom_check("chain_recovery", recovery_ready, true);
        // service inner loop
        let check_alive = self.is_alive().await;
        status.set_custom_check("alive", check_alive, false);
        // blockchain
        if self.blockchain_tick.is_recent(&CONNECTED_TICK_FRESHNESS) {
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
        if self.database_tick.is_recent(&CONNECTED_TICK_FRESHNESS) {
            status.set_custom_check("database", true, true);
        } else {
            // cloned to ensure the service is not blocked during the IO
            let pool = self.database_pool.read().await.clone();
            status.set_db_connected(&pool).await;
        };
        status
    }

    async fn is_alive(&self) -> bool {
        self.blockchain_tick.is_recent(&IS_ALIVE_TICK_FRESHNESS)
            || self
                .blockchain_timeout_tick
                .is_recent(&IS_ALIVE_TICK_FRESHNESS)
    }

    fn get_version(&self) -> Version {
        default_get_version()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Helper to build a HealthCheck without real DB/provider connections.
    fn build_test_health_check(
        blockchain_timeout_tick: HeartBeat,
        blockchain_tick: HeartBeat,
        database_tick: HeartBeat,
    ) -> HealthCheck {
        let db_url = "postgres://test:test@localhost:5432/test";
        let pool = sqlx::postgres::PgPoolOptions::new()
            .connect_lazy(db_url)
            .unwrap();
        HealthCheck {
            blockchain_timeout_tick,
            blockchain_tick,
            blockchain_provider: Arc::new(RwLock::new(None)),
            database_pool: Arc::new(RwLock::new(pool)),
            database_tick,
            recovery_paused: Default::default(),
        }
    }

    fn stale_tick() -> HeartBeat {
        HeartBeat::with_elapsed_secs(30)
    }

    #[tokio::test]
    async fn not_alive_when_all_ticks_stale() {
        let health_check =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        assert!(!health_check.is_alive().await);
    }

    #[tokio::test]
    async fn is_alive_after_blockchain_tick_update() {
        let health_check =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        assert!(!health_check.is_alive().await);
        health_check.blockchain_tick.update();
        assert!(health_check.is_alive().await);
    }

    #[tokio::test]
    async fn is_alive_after_timeout_tick_update() {
        let health_check =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        assert!(!health_check.is_alive().await);
        health_check.blockchain_timeout_tick.update();
        assert!(health_check.is_alive().await);
    }

    #[tokio::test]
    async fn not_alive_after_only_database_tick_update() {
        let health_check =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        assert!(!health_check.is_alive().await);
        health_check.database_tick.update();
        assert!(!health_check.is_alive().await);
    }

    #[tokio::test]
    async fn is_alive_after_all_ticks_update() {
        let health_check =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        assert!(!health_check.is_alive().await);
        health_check.blockchain_tick.update();
        health_check.blockchain_timeout_tick.update();
        health_check.database_tick.update();
        assert!(health_check.is_alive().await);
    }
    async fn record_test_halt(pool: &sqlx::PgPool) -> anyhow::Result<()> {
        use fhevm_engine_common::chain_recovery::{
            record_halt, HistoryMismatch, ListenerSource,
        };
        record_halt(
            pool,
            HistoryMismatch {
                source: ListenerSource::Host,
                gcs_mode: false,
                chain_id: 42,
                block_number: 65,
                expected_hash: Some(&[1_u8; 32]),
                observed_hash: Some(&[2_u8; 32]),
                reason: "test committed history conflict",
            },
        )
        .await?;
        Ok(())
    }

    #[derive(Clone, Copy)]
    enum HaltTiming {
        Startup,
        Idle,
        FailedWrite,
    }

    async fn assert_containment_pause(
        timing: HaltTiming,
    ) -> anyhow::Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let pool = sqlx::PgPool::connect(instance.db_url()).await?;
        let mut health =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        health.database_pool = Arc::new(RwLock::new(pool.clone()));
        if matches!(timing, HaltTiming::Startup) {
            record_test_halt(&pool).await?;
        }
        let cancel = tokio_util::sync::CancellationToken::new();
        let started = tokio::sync::Notify::new();
        let work = async {
            let mut tx = pool.begin().await?;
            sqlx::query("INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES (42, 65)")
                .execute(&mut *tx).await?;
            started.notify_one();
            if matches!(timing, HaltTiming::FailedWrite) {
                tx.rollback().await?;
                record_test_halt(&pool).await?;
                return Err(sqlx::Error::Protocol(
                    "committed history conflict".into(),
                )
                .into());
            }
            // Model an idle RPC/stream with unacknowledged, uncommitted work.
            std::future::pending::<()>().await;
            tx.commit().await?;
            Ok(())
        };
        let listener = health.run_with_containment(&cancel, work);
        tokio::pin!(listener);
        if matches!(timing, HaltTiming::Idle) {
            tokio::select! {
                result = &mut listener => panic!("listener exited: {result:?}"),
                result = tokio::time::timeout(Duration::from_secs(5), started.notified()) => result?,
            }
            record_test_halt(&pool).await?;
        }
        tokio::select! {
            result = &mut listener => panic!("contained listener exited: {result:?}"),
            result = tokio::time::timeout(Duration::from_secs(10), async {
                while !health.recovery_paused.load(Ordering::Relaxed) {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }) => result?,
        }
        assert!(health.is_alive().await);
        health.blockchain_tick.update();
        health.database_tick.update();
        assert!(!health.health_check().await.is_healthy());
        let cursor: Option<i64> = sqlx::query_scalar("SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = 42").fetch_optional(&pool).await?;
        assert!(
            cursor.is_none(),
            "uncommitted ingestion must be dropped before pausing"
        );
        // Even after an audited DB recovery, stale cached work must not resume.
        sqlx::query("DELETE FROM public.chain_recovery_halts")
            .execute(&pool)
            .await?;
        health.blockchain_tick.update();
        health.database_tick.update();
        assert!(!health.health_check().await.is_healthy());
        assert!(tokio::time::timeout(
            Duration::from_millis(100),
            &mut listener
        )
        .await
        .is_err());
        cancel.cancel();
        tokio::time::timeout(Duration::from_secs(1), listener).await??;
        Ok(())
    }

    #[tokio::test]
    async fn containment_before_startup_keeps_liveness_and_blocks_readiness(
    ) -> anyhow::Result<()> {
        assert_containment_pause(HaltTiming::Startup).await
    }

    #[tokio::test]
    async fn containment_while_idle_drops_pending_work_and_latches_pause(
    ) -> anyhow::Result<()> {
        assert_containment_pause(HaltTiming::Idle).await
    }

    #[tokio::test]
    async fn containment_from_failed_write_pauses_instead_of_exiting(
    ) -> anyhow::Result<()> {
        assert_containment_pause(HaltTiming::FailedWrite).await
    }

    #[tokio::test]
    async fn protocol_error_without_containment_remains_fatal(
    ) -> anyhow::Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let pool = sqlx::PgPool::connect(instance.db_url()).await?;
        let mut health =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        health.database_pool = Arc::new(RwLock::new(pool));
        let cancel = tokio_util::sync::CancellationToken::new();
        let err = health
            .run_with_containment(&cancel, async {
                Err(sqlx::Error::Protocol("malformed executor event".into())
                    .into())
            })
            .await
            .unwrap_err();
        assert!(matches!(
            err.downcast_ref::<sqlx::Error>(),
            Some(sqlx::Error::Protocol(_))
        ));
        assert!(!health.recovery_paused.load(Ordering::Relaxed));
        Ok(())
    }
    #[tokio::test]
    async fn recovery_observation_does_not_block_work_releasing_a_connection(
    ) -> anyhow::Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow::anyhow!("{err}"))?;
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(instance.db_url())
            .await?;
        let mut health =
            build_test_health_check(stale_tick(), stale_tick(), stale_tick());
        health.database_pool = Arc::new(RwLock::new(pool.clone()));
        let cancel = tokio_util::sync::CancellationToken::new();
        tokio::time::timeout(
            Duration::from_secs(10),
            health.run_with_containment(&cancel, async {
                let tx = pool.begin().await?;
                // The five-second observer must yield while waiting for our sole connection.
                tokio::time::sleep(Duration::from_secs(6)).await;
                tx.rollback().await?;
                Ok(())
            }),
        )
        .await??;
        assert!(!health.recovery_paused.load(Ordering::Relaxed));
        Ok(())
    }
}
