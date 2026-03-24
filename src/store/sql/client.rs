use crate::config::settings::{SqlPoolConfig, StorageConfig};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tracing::info;

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
        let conn = self.app_pool.acquire().await;
        conn
    }

    pub async fn get_cron_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        let conn = self.cron_pool.acquire().await;
        conn
    }

    /// Gracefully close both pools, waiting for connections to drain.
    pub async fn close(&self) {
        self.app_pool.close().await;
        self.cron_pool.close().await;
    }
}
