use crate::config::config::DatabaseConfig;
use sqlx::Postgres;
use sqlx::pool::PoolConnection;
use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::info;

pub struct PgClient {
    pool: PgPool,
}

impl PgClient {
    pub async fn new(config: &DatabaseConfig) -> Result<Self, sqlx::Error> {
        info!("Initializing database connection...");

        let pool = PgPoolOptions::new()
            .max_connections(config.pool.max_connections)
            .min_connections(config.pool.min_connections)
            .acquire_timeout(Duration::from_secs(config.pool.acquire_timeout_secs))
            .idle_timeout(Duration::from_secs(config.pool.idle_timeout_secs))
            .max_lifetime(Duration::from_secs(config.pool.max_lifetime_secs))
            .connect(&config.db_url)
            .await?;

        // Validate connection
        sqlx::query("SELECT 1").execute(&pool).await?;

        info!(
            "Database pool initialized: min={}, max={}",
            config.pool.min_connections, config.pool.max_connections
        );

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }

    pub async fn acquire(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.pool.acquire().await
    }

    pub async fn get_app_connection(&self) -> Result<PoolConnection<Postgres>, sqlx::Error> {
        self.pool.acquire().await
    }

    pub async fn close(&self) {
        self.pool.close().await;
    }
}
