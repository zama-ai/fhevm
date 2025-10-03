use crate::{
    cli::DbTestArgs,
    config::Config,
    db::{DbConnector, RequestBuilder, ResponseTracker},
    decryption::types::DecryptionRequest,
};
use anyhow::anyhow;
use std::sync::Arc;
use tokio::{
    sync::{Mutex, mpsc},
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::{Instrument, error, info};

pub struct DatabaseTestManager {
    config: Config,
    db_connectors: Vec<DbConnector>,
    request_builder: RequestBuilder,
    response_trackers: Vec<Arc<Mutex<ResponseTracker>>>,
}

impl DatabaseTestManager {
    pub async fn connect(config: Config) -> anyhow::Result<DatabaseTestManager> {
        let db_config = config
            .database
            .as_ref()
            .ok_or_else(|| anyhow!("Missing [database] section in config file"))?;

        info!(
            "Connecting to {} connectors' database...",
            db_config.urls.len()
        );

        let mut db_connectors = vec![];
        let mut response_trackers = vec![];
        for i in 0..db_config.urls.len() {
            let (request_sender, request_receiver) = mpsc::unbounded_channel();
            let connector = DbConnector::connect(db_config, i, request_sender).await?;
            let tracker = ResponseTracker::new(
                connector.name.clone(),
                request_receiver,
                connector.db_pool.clone(),
            );
            db_connectors.push(connector);
            response_trackers.push(Arc::new(Mutex::new(tracker)));
        }

        let request_builder = RequestBuilder::new(
            config.user_ct_handles.clone(),
            config.public_ct_handles.clone(),
            db_config.ct_digest,
            db_config.key_id,
            db_config.copro_tx_sender_addr,
        );
        let manager = DatabaseTestManager {
            config,
            db_connectors,
            request_builder,
            response_trackers,
        };

        manager.health_check().await?;
        info!("All databases health check were successful!");

        Ok(manager)
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        let mut health_results = vec![];
        for db_connector in &self.db_connectors {
            health_results.push(db_connector.health_check().await);
        }
        if health_results.iter().any(anyhow::Result::is_err) {
            return Err(anyhow!("Health check failed: {health_results:?}"));
        }
        Ok(())
    }

    pub async fn clear_databases(&self) -> anyhow::Result<()> {
        info!("Clearing database tables before test...");

        let mut clear_results = vec![];
        for db_connector in &self.db_connectors {
            clear_results.push(db_connector.clear_tables().await);
        }

        if clear_results.iter().any(anyhow::Result::is_err) {
            return Err(anyhow!("Health check failed: {clear_results:?}"));
        }

        info!("All databases tables were cleared successfully!");
        Ok(())
    }

    pub async fn stress_test(mut self, args: DbTestArgs) -> anyhow::Result<()> {
        if args.clear_db {
            self.clear_databases().await?;
        }

        let session_start = Instant::now();
        let mut interval = interval(self.config.tests_interval);
        let mut burst_tasks = JoinSet::new();
        let mut burst_index = 1;
        loop {
            if !self.config.sequential {
                interval.tick().await;
            }

            if session_start.elapsed() > self.config.tests_duration {
                break;
            }

            let requests = self
                .request_builder
                .build_requests(args.decryption_type, self.config.parallel_requests)?;
            burst_tasks.spawn(handle_burst(
                burst_index,
                self.db_connectors.clone(),
                requests,
                self.response_trackers.clone(),
            ));

            burst_index += 1;

            if self.config.sequential {
                burst_tasks.join_next().await;
            }
        }

        burst_tasks.join_all().await;
        let elapsed = session_start.elapsed().as_secs_f64();
        info!(
            "Handled all burst in {:.2}s. Throughput: {:.2} tps",
            elapsed,
            (self.config.parallel_requests * (burst_index - 1) as u32) as f64 / elapsed
        );
        Ok(())
    }
}

#[tracing::instrument(skip(db_connectors, requests, response_trackers))]
async fn handle_burst(
    burst_index: usize,
    db_connectors: Vec<DbConnector>,
    requests: Vec<DecryptionRequest>,
    response_trackers: Vec<Arc<Mutex<ResponseTracker>>>,
) {
    info!(
        "Starting requests burst ({})...",
        requests.first().map(|r| r.type_str()).unwrap()
    );

    insert_requests_in_all_connectors(db_connectors, requests).await;
    let mut wait_responses_tasks = JoinSet::new();
    for tracker in response_trackers {
        wait_responses_tasks.spawn(
            async move { tracker.lock().await.wait_responses_of_next_burst().await }
                .in_current_span(),
        );
    }

    let results = wait_responses_tasks.join_all().await;
    let mut is_error = false;
    for res in results.iter() {
        if let Err(e) = res {
            is_error = true;
            error!("One of the connector failed to handled the burst: {e}");
        }
    }
    if is_error {
        return;
    }

    let mut latency = 0_f64;
    let mut throughput = f64::MAX;
    for res in results.into_iter().map(|r| r.unwrap()) {
        latency = latency.max(res.latency);
        throughput = throughput.min(res.throughput);
    }

    info!(
        latency = latency,
        throughput = throughput,
        "Burst sucessfully processed by all connectors!",
    );
}

async fn insert_requests_in_all_connectors(
    db_connectors: Vec<DbConnector>,
    requests: Vec<DecryptionRequest>,
) {
    let mut requests_insertion_tasks = JoinSet::new();
    for connector in db_connectors {
        let cloned_requests = requests.clone();
        requests_insertion_tasks.spawn(async move {
            if let Err(e) = connector.insert_requests(cloned_requests).await {
                error!(connector_name = connector.name, "{e}");
            }
        });
    }
    requests_insertion_tasks.join_all().await;
}
