use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

use crate::sql::client::PgClient;

pub async fn run_migrations(pool: &PgClient, max_attempts: u32) -> Result<(), anyhow::Error> {
    let mut attempt: u32 = 0;
    loop {
        match sqlx::migrate!("./migrations").run(&pool.get_pool()).await {
            Ok(_) => break,
            Err(err) => {
                error!("Migration failed: {}", err);
                if attempt == max_attempts {
                    error!("Could not migrate after all attempts, aborting: {}", err);
                    panic!("CRITICAL: Could not apply migration properly");
                }
                sleep(Duration::from_secs(2)).await;
                attempt += 1;
            }
        }
    }
    Ok(())
}
