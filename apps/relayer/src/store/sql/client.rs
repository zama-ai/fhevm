use crate::{config::settings::StorageConfig, metrics};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

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

    pub fn spawn_db_pool_monitor(&self) {
        let pool = self.pool.clone();
        tokio::spawn(async move {
            loop {
                let pool_clone = pool.clone();
                let join_handle = tokio::spawn(async move {
                    monitor_pool_loop(pool_clone).await;
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

    /// Use this instead of &self.pool if you want to strictly measure wait time.
    pub async fn get_connection(
        &self,
    ) -> Result<sqlx::pool::PoolConnection<sqlx::Postgres>, sqlx::Error> {
        let start = std::time::Instant::now();
        let conn = self.pool.acquire().await;
        metrics::observe_pool_wait(start.elapsed());
        conn
    }
}

async fn monitor_pool_loop(pool: PgPool) {
    info!("Starting DB Pool Monitor loop");
    loop {
        let size = pool.size();
        let idle = pool.num_idle();
        let active = size.saturating_sub(idle as u32);

        // Update the Prometheus Gauges
        metrics::sql::update_pool_stats(active, idle as u32);

        sleep(Duration::from_secs(5)).await;
    }
}
