//! Down migration: v0.8.8 → v0.8.7
//! Temporary binary for rollback scenarios only.
//!
//! Run BEFORE downgrading relayer binary to v0.8.7.

use relayer_migrate::{config::config, sql::client::PgClient};
use tracing::{Level, error, info};

const DOWN_SQL: &str = include_str!("../../down/v0.8.8_to_v0.8.7.sql");
const VERSION: &str = "v0.8.8 → v0.8.7";

#[tokio::main]
async fn main() -> Result<(), anyhow::Error> {
    tracing_subscriber::fmt()
        .with_env_filter(
            tracing_subscriber::EnvFilter::from_default_env().add_directive(Level::INFO.into()),
        )
        .init();

    info!("Starting down migration {}", VERSION);

    let db_url = &config().database_url;
    let max_attempts = config().max_attempts;

    info!("Connecting to database...");
    let client = PgClient::new(db_url.clone(), 1, max_attempts).await;
    let pool = client.get_pool();

    info!("Executing down migration SQL...");
    match sqlx::raw_sql(DOWN_SQL).execute(&pool).await {
        Ok(_) => {
            info!("Down migration {} completed successfully", VERSION);
            Ok(())
        }
        Err(e) => {
            error!("Down migration failed: {}", e);
            Err(e.into())
        }
    }
}
