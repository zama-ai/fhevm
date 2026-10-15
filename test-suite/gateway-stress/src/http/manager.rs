use crate::{
    bench::{BenchAverageResult, BenchBurstResult, BenchRecordInput},
    cli::{HttpBenchmarkArgs, HttpTestArgs},
    config::Config,
    decryption::{BurstResult, types::DecryptionType},
    http::{
        HttpConnector, HttpRequestBuilder,
        types::{HttpDecryptionRequest, RequestOutcome},
    },
};
use anyhow::anyhow;
use std::collections::BTreeMap;
use tokio::{
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::{Instrument, debug, error, info};

pub struct HttpTestManager {
    config: Config,
    connectors: Vec<HttpConnector>,
    request_builder: HttpRequestBuilder,
}

impl HttpTestManager {
    pub async fn connect(config: Config) -> anyhow::Result<HttpTestManager> {
        let http_config = config
            .http
            .as_ref()
            .ok_or_else(|| anyhow!("Missing [http] section in config file"))?;
        if http_config.urls.is_empty() {
            return Err(anyhow!(
                "`[http].urls` must list at least one connector URL"
            ));
        }

        info!(
            "Configuring HTTP clients for {} connectors...",
            http_config.urls.len()
        );
        let connectors = (0..http_config.urls.len())
            .map(|i| HttpConnector::connect(http_config, i))
            .collect::<anyhow::Result<Vec<_>>>()?;

        let request_builder = HttpRequestBuilder::new(
            config.id_counter_start,
            config.user_ct.clone(),
            config.public_ct.clone(),
            config.allowed_contract,
            config.blockchain.clone(),
        );
        let manager = HttpTestManager {
            config,
            connectors,
            request_builder,
        };

        manager.health_check().await?;
        info!("All connectors health check were successful!");
        Ok(manager)
    }

    pub async fn health_check(&self) -> anyhow::Result<()> {
        let mut health_results = vec![];
        for connector in &self.connectors {
            health_results.push(connector.health_check().await);
        }
        if health_results.iter().any(anyhow::Result::is_err) {
            return Err(anyhow!("Health check failed: {health_results:?}"));
        }
        Ok(())
    }

    /// Runs a decryption stress testing session via the KMS Connectors' HTTP endpoints.
    pub async fn stress_test(mut self, args: HttpTestArgs) -> anyhow::Result<()> {
        let session_start = Instant::now();
        let mut interval = interval(self.config.tests_interval);
        let mut burst_tasks = JoinSet::new();
        let mut burst_index = 1;
        let mut handles_decrypted = 0_usize;
        loop {
            if !self.config.sequential {
                interval.tick().await;
            }

            if session_start.elapsed() > self.config.tests_duration {
                break;
            }

            let requests = self
                .request_builder
                .build_requests(args.decryption_type, self.config.parallel_requests)
                .await?;
            burst_tasks.spawn(handle_burst(burst_index, self.connectors.clone(), requests));

            burst_index += 1;

            if self.config.sequential
                && let Some(Ok(Ok(result))) = burst_tasks.join_next().await
            {
                handles_decrypted += result.handles_decrypted;
            }
        }

        for result in burst_tasks.join_all().await.into_iter().flatten() {
            handles_decrypted += result.handles_decrypted;
        }

        let elapsed = session_start.elapsed().as_secs_f64();
        let throughput = handles_decrypted as f64 / elapsed;
        info!(
            "Handled all burst in {elapsed:.2}s. Decrypted {handles_decrypted} handles. \
             Throughput: {throughput:.2} tps"
        );
        Ok(())
    }

    /// Runs a decryption benchmark session via the KMS Connectors' HTTP endpoints.
    pub async fn decryption_benchmark(mut self, args: HttpBenchmarkArgs) -> anyhow::Result<()> {
        let mut csv_reader = csv::ReaderBuilder::new()
            .delimiter(b';')
            .comment(Some(b'#'))
            .from_path(args.input)?;
        let mut average_results_writer = csv::WriterBuilder::new()
            .delimiter(b';')
            .from_path(&args.output)?;
        let mut full_results_writer = if let Some(path) = args.results {
            Some(csv::WriterBuilder::new().delimiter(b';').from_path(path)?)
        } else {
            None
        };

        let mut burst_index = 1;
        for csv_row in csv_reader.deserialize::<BenchRecordInput>() {
            let bench_record = csv_row.map_err(|e| anyhow!("Invalid row: {e}"))?;
            ensure_http_supported(bench_record.decryption_type)?;
            info!("Starting benchmark with parameters: {bench_record:?}");

            let results = self
                .perform_single_bench(&bench_record, &mut burst_index)
                .await?;

            if let Some(w) = &mut full_results_writer {
                for result in results.iter() {
                    w.serialize(result)?;
                }
                w.flush()?;
            }
            let bench_result = BenchAverageResult::new(bench_record, results);
            average_results_writer.serialize(bench_result)?;
            average_results_writer.flush()?;
        }
        Ok(())
    }

    async fn perform_single_bench(
        &mut self,
        bench_record: &BenchRecordInput,
        burst_index: &mut usize,
    ) -> anyhow::Result<Vec<BenchBurstResult>> {
        let mut results = vec![];
        for _ in 0..bench_record.number_of_measures {
            let requests = self
                .request_builder
                .build_requests(bench_record.decryption_type, bench_record.parallel_requests)
                .await?;

            let burst_result = handle_burst(*burst_index, self.connectors.clone(), requests).await;

            if let Ok(burst_result) = burst_result {
                results.push(BenchBurstResult::new(
                    *burst_index,
                    bench_record.parallel_requests,
                    bench_record.decryption_type,
                    burst_result.into(),
                ));
            }
            *burst_index += 1;
        }

        Ok(results)
    }
}

/// Rejects decryption types that can't be driven through the HTTP path.
pub fn ensure_http_supported(decryption_type: DecryptionType) -> anyhow::Result<()> {
    if matches!(decryption_type, DecryptionType::User) {
        return Err(anyhow!(
            "Legacy `user` decryption is not supported over the `http`/`bench-http` path (the KMS \
             connector HTTP interface only serves `public` and RFC-016 `user-v2` decryptions). \
             Use `-t user-v2` or the `gw`/`bench-gw` path instead."
        ));
    }
    Ok(())
}

/// A [`BurstResult`] with the number of handles the burst decrypted across all parties.
pub struct HttpBurstResult {
    pub latency: f64,
    pub throughput: f64,
    pub handles_decrypted: usize,
}

impl From<HttpBurstResult> for BurstResult {
    fn from(r: HttpBurstResult) -> Self {
        BurstResult {
            latency: r.latency,
            throughput: r.throughput,
        }
    }
}

/// Sends the burst to every party concurrently and waits for all answers.
///
/// Like the DB path, the burst latency is the slowest party's latency, i.e. the time between the
/// first request being sent and the last `200` being received, and the burst throughput is the
/// slowest party's. A burst fails if any party answers any request with a non-200 status.
#[tracing::instrument(skip(connectors, requests))]
async fn handle_burst(
    burst_index: usize,
    connectors: Vec<HttpConnector>,
    requests: Vec<HttpDecryptionRequest>,
) -> anyhow::Result<HttpBurstResult> {
    let Some(first) = requests.first() else {
        return Err(anyhow!("Empty request burst"));
    };
    info!("Starting requests burst ({})...", first.type_str());

    let mut party_tasks = JoinSet::new();
    for connector in connectors {
        let requests = requests.clone();
        party_tasks.spawn(send_burst_to_party(connector, requests).in_current_span());
    }

    let mut is_error = false;
    let mut latency = 0_f64;
    let mut throughput = f64::MAX;
    let mut handles_decrypted = 0_usize;
    for result in party_tasks.join_all().await {
        match result {
            Err(e) => {
                error!("One of the connector failed to handle the burst: {e}");
                is_error = true;
            }
            Ok(res) => {
                latency = latency.max(res.latency);
                throughput = throughput.min(res.throughput);
                handles_decrypted = res.handles_decrypted;
            }
        }
    }
    if is_error {
        return Err(anyhow!(
            "At least one connector failed to handle the burst."
        ));
    }

    info!(
        latency = latency,
        throughput = throughput,
        "Burst successfully processed by all connectors!",
    );
    Ok(HttpBurstResult {
        latency,
        throughput,
        handles_decrypted,
    })
}

/// Sends every request of the burst to one party concurrently.
async fn send_burst_to_party(
    connector: HttpConnector,
    requests: Vec<HttpDecryptionRequest>,
) -> anyhow::Result<HttpBurstResult> {
    let burst_start = Instant::now();
    let mut tasks = JoinSet::new();
    for request in requests {
        let connector = connector.clone();
        tasks.spawn(async move { connector.send(&request).await }.in_current_span());
    }

    let mut outcomes: Vec<RequestOutcome> = Vec::new();
    // Failed requests, grouped by HTTP status (`"transport"` when no response was received).
    let mut failures: BTreeMap<String, usize> = BTreeMap::new();
    for result in tasks.join_all().await {
        match result {
            Ok(outcome) if outcome.is_success() => outcomes.push(outcome),
            Ok(outcome) => {
                *failures.entry(outcome.http_status.to_string()).or_default() += 1;
                debug!(
                    connector = %connector,
                    decryption_id = %outcome.decryption_id,
                    "Request failed with status {}: {:?}",
                    outcome.http_status,
                    outcome.error
                );
            }
            Err(e) => {
                *failures.entry("transport".to_string()).or_default() += 1;
                debug!(connector = %connector, "Request failed: {e}");
            }
        }
    }
    // The burst ends when the last answer of this party has been received.
    let latency = burst_start.elapsed().as_secs_f64();

    if !failures.is_empty() {
        let total: usize = failures.values().sum();
        let breakdown = failures
            .iter()
            .map(|(status, count)| format!("{count} x {status}"))
            .collect::<Vec<_>>()
            .join(", ");
        return Err(anyhow!(
            "{connector}: {total} request(s) of the burst failed ({breakdown}), {} succeeded",
            outcomes.len()
        ));
    }

    let handles_decrypted: usize = outcomes.iter().map(|o| o.handle_count).sum();
    let result = HttpBurstResult {
        latency,
        throughput: handles_decrypted as f64 / latency,
        handles_decrypted,
    };
    debug!(
        connector = %connector,
        latency = result.latency,
        throughput = result.throughput,
        max_request_latency = outcomes
            .iter()
            .map(|o| o.elapsed.as_secs_f64())
            .fold(0_f64, f64::max),
        "Burst successfully processed!"
    );
    Ok(result)
}
