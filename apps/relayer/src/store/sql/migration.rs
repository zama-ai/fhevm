use anyhow::Result;
use std::time::Duration;
use tokio::time::sleep;
use tracing::error;

use crate::store::sql::client::PgClient;

pub async fn run_migrations(pool: &PgClient) -> Result<(), anyhow::Error> {
    loop {
        match sqlx::migrate!().run(&pool.get_pool()).await {
            Ok(_) => break,
            Err(err) => {
                error!("Migration failed: {}", err);
                sleep(Duration::from_secs(2)).await;
            }
        }
    }
    Ok(())
}
