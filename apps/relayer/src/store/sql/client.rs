use crate::config::settings::StorageConfig;
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[derive(Debug, Clone)]
pub struct PgClient {
    pool: PgPool,
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

        PgClient { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
