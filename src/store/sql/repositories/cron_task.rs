use crate::{
    config::settings::CronConfig,
    logging::WorkerStep,
    store::sql::{
        client::PgClient,
        repositories::{expiry_repo::ExpiryRepository, timeout_repo::TimeoutRepository},
    },
};
use futures::FutureExt;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{debug, error, info, warn};

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
                debug!(
                    step = %WorkerStep::TickCompleted,
                    worker = "timeout",
                    "Tick complete, no requests timed out"
                );
            }
            Ok(count) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "timeout",
                    rows = count,
                    "Moved stale requests to timed_out"
                );
            }
            Err(e) => {
                error!(error = ?e, worker = "timeout", "Database error, retrying next tick");
            }
        }
    }
}

pub async fn create_timeout_worker_future(pool: PgClient, cron_config: CronConfig) {
    loop {
        let pool_clone = pool.clone();
        let cron_config_clone = cron_config.clone();

        info!(
            step = %WorkerStep::WorkerStarted,
            worker = "timeout",
            "Worker started"
        );

        let result = std::panic::AssertUnwindSafe(async {
            run_timeout_worker_logic(pool_clone, cron_config_clone).await;
        })
        .catch_unwind()
        .await;

        match result {
            Ok(_) => {
                warn!(
                    step = %WorkerStep::WorkerRestarting,
                    worker = "timeout",
                    delay_secs = 5,
                    "Worker stopped unexpectedly, restarting"
                );
            }
            Err(_) => {
                error!(
                    step = %WorkerStep::WorkerPanicked,
                    worker = "timeout",
                    delay_secs = 5,
                    "Worker panicked, restarting"
                );
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
                debug!(
                    step = %WorkerStep::TickCompleted,
                    worker = "expiry",
                    "Tick complete, no requests expired"
                );
            }
            Ok(count) => {
                info!(
                    step = %WorkerStep::RowsProcessed,
                    worker = "expiry",
                    rows = count,
                    "Purged stale rows"
                );
            }
            Err(e) => {
                error!(error = ?e, worker = "expiry", "Database error, retrying next interval");
            }
        }
    }
}

pub async fn create_expiry_worker_future(pool: PgClient, cron_config: CronConfig) {
    loop {
        let pool_clone = pool.clone();

        info!(
            step = %WorkerStep::WorkerStarted,
            worker = "expiry",
            "Worker started"
        );

        let result = std::panic::AssertUnwindSafe(async {
            run_expiry_worker_logic(pool_clone, cron_config.clone()).await;
        })
        .catch_unwind()
        .await;

        match result {
            Ok(_) => {
                warn!(
                    step = %WorkerStep::WorkerRestarting,
                    worker = "expiry",
                    delay_secs = 30,
                    "Worker stopped unexpectedly, restarting"
                );
            }
            Err(_) => {
                error!(
                    step = %WorkerStep::WorkerPanicked,
                    worker = "expiry",
                    delay_secs = 30,
                    "Worker panicked, restarting"
                );
            }
        }

        sleep(Duration::from_secs(30)).await;
    }
}
