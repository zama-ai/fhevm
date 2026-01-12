use crate::{
    config::settings::CronConfig,
    store::sql::{
        client::PgClient,
        repositories::{expiry_repo::ExpiryRepository, timeout_repo::TimeoutRepository},
    },
};
use futures::FutureExt;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info};

async fn run_timeout_worker_logic(pool: PgClient, cron_config: CronConfig) {
    let repo = TimeoutRepository::new(pool, cron_config.clone());
    let mut interval = tokio::time::interval(cron_config.timeout_cron_interval);

    debug!(
        interval_secs = cron_config.timeout_cron_interval.as_secs(),
        public_decrypt_timeout_secs = cron_config.public_decrypt_timeout.as_secs(),
        user_decrypt_timeout_secs = cron_config.user_decrypt_timeout.as_secs(),
        input_proof_timeout_secs = cron_config.input_proof_timeout.as_secs(),
        "Timeout worker initialized"
    );

    loop {
        interval.tick().await;
        debug!("Timeout worker tick - checking for stale requests");

        match repo.time_out_stale_requests().await {
            Ok(0) => {
                debug!("Timeout worker tick complete - nothing timed out");
            }
            Ok(count) => {
                info!(
                    timed_out_count = count,
                    "Timeout Worker: Moved stale requests to timed_out"
                );
            }
            Err(e) => {
                error!(error = ?e, "Timeout Worker Error (Retrying in next tick)");
            }
        }
    }
}

pub async fn create_timeout_worker_future(pool: PgClient, cron_config: CronConfig) {
    loop {
        let pool_clone = pool.clone();
        let cron_config_clone = cron_config.clone();

        info!("Starting Timeout Worker...");

        let result = std::panic::AssertUnwindSafe(async {
            run_timeout_worker_logic(pool_clone, cron_config_clone).await;
        })
        .catch_unwind()
        .await;

        match result {
            Ok(_) => {
                error!("Timeout Worker stopped unexpectedly. Restarting in 5 seconds...");
            }
            Err(_) => {
                error!("CRITICAL: Timeout Worker PANICKED! Restarting in 5 seconds...");
            }
        }
        sleep(Duration::from_secs(5)).await;
    }
}

async fn run_expiry_worker_logic(pool: PgClient, cron_config: CronConfig) {
    let repo = ExpiryRepository::new(pool, cron_config.clone());

    let mut interval = tokio::time::interval(cron_config.expiry_cron_interval);

    debug!(
        interval_secs = cron_config.expiry_cron_interval.as_secs(),
        public_decrypt_expiry_secs = cron_config.public_decrypt_expiry.as_secs(),
        user_decrypt_expiry_secs = cron_config.user_decrypt_expiry.as_secs(),
        input_proof_expiry_secs = cron_config.input_proof_expiry.as_secs(),
        "Expiry worker initialized"
    );

    // Skip the first immediate tick to avoid competing with timeout worker at startup
    interval.tick().await;

    loop {
        interval.tick().await;
        debug!("Expiry worker tick - checking for stale data");

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
    }
}

pub async fn create_expiry_worker_future(pool: PgClient, cron_config: CronConfig) {
    loop {
        let pool_clone = pool.clone();
        info!("Starting Expiry/Cleanup Worker...");

        let result = std::panic::AssertUnwindSafe(async {
            run_expiry_worker_logic(pool_clone, cron_config.clone()).await;
        })
        .catch_unwind()
        .await;

        match result {
            Ok(_) => {
                error!("Expiry Worker stopped unexpectedly. Restarting in 30 seconds...");
            }
            Err(_) => {
                error!("CRITICAL: Expiry Worker PANICKED! Restarting in 30 seconds...");
            }
        }

        sleep(Duration::from_secs(30)).await;
    }
}
