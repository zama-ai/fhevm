use crate::{
    bench::{BenchAverageResult, BenchBurstResult, BenchRecordInput},
    blockchain::types::DecryptionType,
    cli::{RelayerBenchmarkArgs, RelayerTestArgs},
    config::Config,
    relayer::{public::public_decryption_burst, user::user_decryption_burst},
    utils::install_crypto_provider,
};
use alloy::{primitives::Address, transports::http::reqwest};
use anyhow::anyhow;
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::Arc, time::Instant};
use tokio::{task::JoinSet, time::interval};
use tracing::{Instrument, info};

pub struct RelayerTestManager {
    /// The client used to send request to the Relayer.
    relayer_client: reqwest::Client,

    /// The configuration of the test session.
    config: Config,

    /// The fhevm rust sdk used to compute the EIP712 for UserDecryptions.
    sdk: Arc<FhevmSdk>,
}

impl RelayerTestManager {
    /// Connects the tool to the Gateway.
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        install_crypto_provider();

        if config.relayer.is_none() {
            return Err(anyhow!("Missing [relayer] section in config file"));
        }
        let Some(blockchain_config) = config.blockchain.as_ref() else {
            return Err(anyhow!("Missing [blockchain] section in config file"));
        };

        let relayer_client = reqwest::ClientBuilder::new().build()?;
        info!("Successfully connected to the Relayer");

        let sdk = Arc::new(
            FhevmSdkBuilder::new()
                .with_gateway_chain_id(blockchain_config.gateway_chain_id)
                .with_decryption_contract(&blockchain_config.decryption_address.to_string())
                .with_acl_contract(&Address::ZERO.to_string())
                .with_input_verification_contract(&Address::ZERO.to_string())
                .with_host_chain_id(blockchain_config.host_chain_id)
                .build()?,
        );

        Ok(Self {
            relayer_client,
            config,
            sdk,
        })
    }

    /// Runs a decryption stress testing session via the Relayer.
    pub async fn stress_test(&self, args: RelayerTestArgs) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();

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

            let progress_bar = init_progress_bars(&self.config, &progress_tracker, burst_index)?;

            match args.decryption_type {
                DecryptionType::Public => burst_tasks.spawn(
                    public_decryption_burst(
                        burst_index,
                        self.config.clone(),
                        self.relayer_client.clone(),
                        progress_bar,
                    )
                    .in_current_span(),
                ),
                DecryptionType::User => burst_tasks.spawn(
                    user_decryption_burst(
                        burst_index,
                        self.config.clone(),
                        self.relayer_client.clone(),
                        Arc::clone(&self.sdk),
                        progress_bar,
                    )
                    .in_current_span(),
                ),
            };

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

    /// Runs a decryption benchmark session via the Relayer.
    pub async fn decryption_benchmark(&self, args: RelayerBenchmarkArgs) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();

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
            info!("Starting benchmark with parameters: {bench_record:?}");

            let results = self
                .perform_single_bench(&bench_record, &progress_tracker, &mut burst_index)
                .await?;

            if let Some(w) = &mut full_results_writer {
                for result in results.iter() {
                    w.serialize(result)?;
                }
            }
            let bench_result = BenchAverageResult::new(bench_record, results);
            average_results_writer.serialize(bench_result)?;
        }

        Ok(())
    }

    async fn perform_single_bench(
        &self,
        bench_record: &BenchRecordInput,
        progress_tracker: &MultiProgress,
        burst_index: &mut usize,
    ) -> anyhow::Result<Vec<BenchBurstResult>> {
        let mut config = self.config.clone();
        config.parallel_requests = bench_record.parallel_requests;

        let mut results = vec![];
        for _ in 0..bench_record.number_of_measures {
            let progress_bar = init_progress_bars(&config, progress_tracker, *burst_index)?;

            let burst_result = match bench_record.decryption_type {
                DecryptionType::Public => {
                    public_decryption_burst(
                        *burst_index,
                        self.config.clone(),
                        self.relayer_client.clone(),
                        progress_bar,
                    )
                    .await
                }
                DecryptionType::User => {
                    user_decryption_burst(
                        *burst_index,
                        self.config.clone(),
                        self.relayer_client.clone(),
                        Arc::clone(&self.sdk),
                        progress_bar,
                    )
                    .await
                }
            };

            if let Ok(burst_result) = burst_result {
                results.push(BenchBurstResult::new(
                    *burst_index,
                    bench_record.parallel_requests,
                    bench_record.decryption_type,
                    burst_result,
                ));
            }
            *burst_index += 1;
        }

        Ok(results)
    }
}

/// Create a progress bar to track a burst of requests sent to the Relayer.
pub fn init_progress_bars(
    config: &Config,
    progress_tracker: &MultiProgress,
    burst_index: usize,
) -> anyhow::Result<ProgressBar> {
    let style = ProgressStyle::with_template(
        "{prefix:32} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )?
    .progress_chars("##-");
    let prefix = format!("Burst #{burst_index}");

    let progress_bar = progress_tracker.add(
        ProgressBar::new(config.parallel_requests.into())
            .with_prefix(prefix.clone())
            .with_message("Handling decryption...")
            .with_style(style.clone()),
    );
    Ok(progress_bar)
}
