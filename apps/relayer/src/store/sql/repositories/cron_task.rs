use crate::store::sql::{
    client::PgClient,
    repositories::{expiry_repo::ExpiryRepository, timeout_repo::TimeoutRepository},
};
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info};

async fn run_timeout_worker_logic(pool: PgClient, timeout_cron_interval_secs: Duration) {
    let repo = TimeoutRepository::new(pool);
    let mut interval = tokio::time::interval(timeout_cron_interval_secs);

    loop {
        match repo.time_out_stale_requests().await {
            Ok(0) => {
                debug!("Nothing timed out for gateway events.");
            }
            Ok(count) => {
                info!(
                    "Timeout Worker: Moved {} stale requests to timed_out.",
                    count
                );
            }
            Err(e) => {
                error!("Timeout Worker Error (Retrying in next tick): {:?}", e);
            }
        }
        interval.tick().await;
    }
}

pub fn spawn_timeout_worker(pool: PgClient, timeout_cron_interval_secs: Duration) {
    tokio::spawn(async move {
        loop {
            let pool_clone = pool.clone();

            info!("Starting Timeout Worker...");

            let handle = tokio::spawn(async move {
                run_timeout_worker_logic(pool_clone, timeout_cron_interval_secs).await;
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

async fn run_expiry_worker_logic(pool: PgClient, expiry_cron_intervals_secs: Duration) {
    let repo = ExpiryRepository::new(pool);

    // Check every 5 minutes (60 * 5 seconds)
    let mut interval = tokio::time::interval(expiry_cron_intervals_secs);

    loop {
        match repo.purge_stale_data().await {
            Ok(0) => {
                debug!("No request expired.");
            }
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
        interval.tick().await;
    }
}

pub fn spawn_expiry_worker(pool: PgClient, expiry_cron_intervals_secs: Duration) {
    tokio::spawn(async move {
        loop {
            let pool_clone = pool.clone();
            info!("Starting Expiry/Cleanup Worker...");

            let handle = tokio::spawn(async move {
                run_expiry_worker_logic(pool_clone, expiry_cron_intervals_secs).await;
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
