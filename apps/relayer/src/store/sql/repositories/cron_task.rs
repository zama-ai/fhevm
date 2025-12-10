use crate::store::sql::{
    client::PgClient,
    repositories::{expiry_repo::ExpiryRepository, timeout_repo::TimeoutRepository},
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{error, info};

async fn run_timeout_worker_logic(pool: PgClient) {
    let repo = TimeoutRepository::new(pool);
    let mut interval = tokio::time::interval(Duration::from_secs(60));

    loop {
        interval.tick().await;
        match repo.time_out_stale_requests().await {
            Ok(0) => {}
            Ok(count) => {
                info!(
                    "Timeout Worker: Moved {} stale requests to timed_out",
                    count
                );
            }
            Err(e) => {
                error!("Timeout Worker Error (Retrying in next tick): {:?}", e);
            }
        }
    }
}

pub fn spawn_timeout_worker(pool: PgClient) {
    tokio::spawn(async move {
        loop {
            let pool_clone = pool.clone();

            info!("Starting Timeout Worker...");

            let handle = tokio::spawn(async move {
                run_timeout_worker_logic(pool_clone).await;
            });
            match handle.await {
                Ok(_) => {
                    error!(
                        "Timeout Worker stopped unexpectedly. Should never happen. Restarting..."
                    );
                }
                Err(e) => {
                    if e.is_panic() {
                        error!("CRITICAL: Timeout Worker PANICKED! Restarting in 5 seconds...");
                    } else {
                        error!("Timeout Worker task cancelled. Restarting...");
                    }
                }
            }
            sleep(Duration::from_secs(5)).await;
        }
    });
}

async fn run_expiry_worker_logic(pool: PgClient) {
    let repo = ExpiryRepository::new(pool);

    // Check every 6 hour (3600 seconds)
    let mut interval = tokio::time::interval(Duration::from_secs(21600));

    loop {
        interval.tick().await;

        match repo.purge_stale_data().await {
            Ok(0) => {}
            Ok(count) => {
                info!(
                    "Expiry Worker: Purged {} stale rows (requests/shares)",
                    count
                );
            }
            Err(e) => {
                error!("Expiry Worker Error (Retrying next hour): {:?}", e);
            }
        }
    }
}

pub fn spawn_expiry_worker(pool: PgClient) {
    tokio::spawn(async move {
        loop {
            let pool_clone = pool.clone();
            info!("Starting Expiry/Cleanup Worker...");

            let handle = tokio::spawn(async move {
                run_expiry_worker_logic(pool_clone).await;
            });

            match handle.await {
                Ok(_) => {
                    error!("Expiry Worker stopped unexpectedly. Restarting...");
                }
                Err(e) => {
                    if e.is_panic() {
                        error!("CRITICAL: Expiry Worker PANICKED! Restarting in 30 seconds...");
                    } else {
                        error!("Expiry Worker task cancelled. Restarting...");
                    }
                }
            }

            sleep(Duration::from_secs(30)).await;
        }
    });
}
