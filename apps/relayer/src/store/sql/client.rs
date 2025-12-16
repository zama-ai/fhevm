use crate::{
    config::settings::{SqlPoolConfig, StorageConfig},
    metrics,
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

#[derive(Debug, Clone)]
pub struct PgClient {
    app_pool: PgPool,
    cron_pool: PgPool,
}

impl PgClient {
    pub async fn new(config: StorageConfig) -> anyhow::Result<Self> {
        let app_pool = Self::create_pool(&config.sql_database_url, &config.app_pool, "app").await?;
        let cron_pool =
            Self::create_pool(&config.sql_database_url, &config.cron_pool, "cron").await?;

        Ok(PgClient {
            app_pool,
            cron_pool,
        })
    }

    async fn create_pool(
        database_url: &str,
        pool_config: &SqlPoolConfig,
        pool_type: &str,
    ) -> anyhow::Result<PgPool> {
        let pool = PgPoolOptions::new()
            .max_connections(pool_config.max_connections)
            .acquire_timeout(Duration::from_secs(pool_config.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(pool_config.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(pool_config.max_lifetime_secs))
            .test_before_acquire(true)
            .min_connections(pool_config.min_connections)
            .connect(database_url)
            .await
            .map_err(|e| {
                anyhow::anyhow!("Failed to connect to {} database pool: {}", pool_type, e)
            })?;

        let mut connections = Vec::new();
        for i in 0..pool_config.min_connections {
            match pool.acquire().await {
                Ok(conn) => connections.push(conn),
                Err(e) => {
                    return Err(anyhow::anyhow!(
                        "Failed to acquire minimum connection {}/{} for {} pool: {}",
                        i + 1,
                        pool_config.min_connections,
                        pool_type,
                        e
                    ));
                }
            }
        }

        drop(connections);

        info!(
            "Successfully validated {} pool with {} min connections (max: {})",
            pool_type, pool_config.min_connections, pool_config.max_connections
        );

        Ok(pool)
    }

    pub fn get_app_pool(&self) -> PgPool {
        self.app_pool.clone()
    }

    pub fn get_cron_pool(&self) -> PgPool {
        self.cron_pool.clone()
    }

    pub async fn get_app_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        let start = std::time::Instant::now();
        let conn = self.app_pool.acquire().await;
        metrics::observe_pool_wait(start.elapsed());
        conn
    }

    pub async fn get_cron_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        let start = std::time::Instant::now();
        let conn = self.cron_pool.acquire().await;
        metrics::observe_pool_wait(start.elapsed());
        conn
    }

    pub fn spawn_db_pool_monitor(&self) {
        let app_pool = self.app_pool.clone();
        let cron_pool = self.cron_pool.clone();
        tokio::spawn(async move {
            loop {
                let app_pool_clone = app_pool.clone();
                let cron_pool_clone = cron_pool.clone();
                let join_handle = tokio::spawn(async move {
                    monitor_pool_loop(app_pool_clone, cron_pool_clone).await;
                });

                match join_handle.await {
                    Ok(_) => {
                        error!("DB Pool Monitor task exited unexpectedly (clean exit). Restarting in 2s...");
                    }
                    Err(e) => {
                        if e.is_panic() {
                            error!("DB Pool Monitor task PANICKED. Restarting in 2s...");
                        } else {
                            error!(
                                "DB Pool Monitor task cancelled/failed: {}. Restarting in 2s...",
                                e
                            );
                        }
                    }
                }
                // 3. Backoff to prevent hot loops in case of persistent startup crashes
                sleep(Duration::from_secs(2)).await;
            }
        });
    }
}

async fn monitor_pool_loop(app_pool: PgPool, cron_pool: PgPool) {
    info!("Starting DB Pool Monitor loop for both app and cron pools");
    loop {
        // Monitor app pool
        let app_size = app_pool.size();
        let app_idle = app_pool.num_idle();
        let app_active = app_size.saturating_sub(app_idle as u32);

        // Monitor cron pool
        let cron_size = cron_pool.size();
        let cron_idle = cron_pool.num_idle();
        let cron_active = cron_size.saturating_sub(cron_idle as u32);

        // For now, aggregate stats for backward compatibility
        // In the future, we could extend metrics to track pools separately
        let total_active = app_active + cron_active;
        let total_idle = app_idle + cron_idle;

        // Update the Prometheus Gauges
        metrics::sql::update_pool_stats(total_active, total_idle as u32);

        sleep(Duration::from_secs(5)).await;
    }
}
