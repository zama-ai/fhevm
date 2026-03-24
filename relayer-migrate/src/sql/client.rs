use sqlx::{PgPool, postgres::PgPoolOptions};
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

#[derive(Debug, Clone)]
pub struct PgClient {
    pool: PgPool,
}

impl PgClient {
    pub async fn new(db_url: String, max_connections: u32, max_attempts: u32) -> Self {
        let mut attemtps: u32 = 0;
        let pool = loop {
            match PgPoolOptions::new()
                .max_connections(max_connections)
                .connect(&db_url)
                .await
            {
                Ok(pool) => break pool,
                Err(err) => {
                    error!("Failed to connect to database: {}", err);
                    if attemtps == max_attempts {
                        error!(
                            "Could not connect to the database after all attempts, aborting:: {}",
                            err
                        );
                        panic!("Couldn't create a connection to the database");
                    }
                    sleep(Duration::from_secs(2)).await;
                    attemtps += 1;
                }
            }
        };

        PgClient { pool }
    }

    pub fn get_pool(&self) -> PgPool {
        self.pool.clone()
    }
}
