use crate::config::settings::StorageConfig;
use crate::http::HealthCheck;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[derive(Debug, Clone)]
pub struct PgClient {
    pool: PgPool,
    health_timeout: Duration,
}

impl PgClient {
    pub async fn new(config: StorageConfig) -> Self {
        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(config.sql_max_connections)
                .connect(&config.sql_database_url)
                .await
            {
                Ok(pool) => break pool,
                Err(err) => {
                    error!("Failed to connect to database: {}", err);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        };

        PgClient {
            pool,
            health_timeout: Duration::from_secs(config.sql_health_check_timeout_secs),
        }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

#[async_trait::async_trait]
impl HealthCheck for PgClient {
    async fn check(&self) -> anyhow::Result<()> {
        match tokio::time::timeout(
            self.health_timeout,
            sqlx::query("SELECT 1").execute(&self.pool),
        )
        .await
        {
            Err(_) => Err(anyhow::anyhow!(
                "Database health check timed out after {:?}",
                self.health_timeout
            )),
            Ok(Err(e)) => Err(anyhow::anyhow!("Database health check failed: {}", e)),
            Ok(Ok(_)) => Ok(()),
        }
    }
}
