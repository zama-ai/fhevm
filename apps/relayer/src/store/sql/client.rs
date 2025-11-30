use crate::http::HealthCheck;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[derive(Debug, Clone)]
pub struct PgClient {
    pool: PgPool,
}

impl PgClient {
    pub async fn new(db_url: String, max_connections: u32) -> Self {
        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(max_connections)
                .connect(&db_url)
                .await
            {
                Ok(pool) => break pool,
                Err(err) => {
                    error!("Failed to connect to database: {}", err);
                    sleep(Duration::from_secs(2)).await;
                }
            }
        };

        PgClient { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}

#[async_trait::async_trait]
impl HealthCheck for PgClient {
    async fn check(&self) -> anyhow::Result<()> {
        sqlx::query("SELECT 1")
            .execute(&self.pool)
            .await
            .map_err(|e| anyhow::anyhow!("Database health check failed: {}", e))?;
        Ok(())
    }
}
