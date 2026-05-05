use super::PgClient;
use sqlx::migrate::Migrator;
use std::path::Path;
use std::time::Duration;
use tracing::{error, info};

pub async fn run_migrations(client: &PgClient, max_attempts: u32) {
    let migrations_path = Path::new("./migrations");

    for attempt in 1..=max_attempts {
        info!(
            "Running database migrations (attempt {}/{})",
            attempt, max_attempts
        );

        let migrator = match Migrator::new(migrations_path).await {
            Ok(m) => m,
            Err(e) => {
                error!("Failed to load migrations: {}", e);
                if attempt < max_attempts {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                    continue;
                }
                panic!(
                    "Failed to load migrations after {} attempts: {}",
                    max_attempts, e
                );
            }
        };

        match migrator.run(client.pool()).await {
            Ok(_) => {
                info!("Database migrations completed successfully");
                return;
            }
            Err(e) => {
                error!("Migration attempt {} failed: {}", attempt, e);
                if attempt < max_attempts {
                    tokio::time::sleep(Duration::from_secs(2)).await;
                } else {
                    panic!(
                        "Database migrations failed after {} attempts: {}",
                        max_attempts, e
                    );
                }
            }
        }
    }
}
