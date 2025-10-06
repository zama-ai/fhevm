use crate::{
    bench::{BenchAverageResult, BenchBurstResult, BenchRecordInput},
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    cli::{GwBenchmarkArgs, GwTestArgs},
    config::Config,
    decryption::{
        EVENT_LISTENER_POLLING, init_public_decryption_response_listener,
        init_user_decryption_response_listener, public::PublicDecryptThresholdEvent,
        public_decryption_burst, types::DecryptionType, user::UserDecryptThresholdEvent,
        user_decryption_burst,
    },
};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        Identity, ProviderBuilder, RootProvider,
        fillers::{ChainIdFiller, FillProvider, JoinFill, WalletFiller},
    },
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
use futures::Stream;
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Once};
use tokio::{
    sync::Mutex,
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::{Instrument, info};

/// The provider used to interact with the Gateway.
type AppProvider = NonceManagedProvider<
    FillProvider<
        JoinFill<
            JoinFill<JoinFill<Identity, ChainIdFiller>, FillersWithoutNonceManagement>,
            WalletFiller<EthereumWallet>,
        >,
        RootProvider,
    >,
>;

/// A struct used to perform the load/stress testing of the Gateway.
pub struct GatewayTestManager {
    /// The decryption contract instance.
    decryption_contract: DecryptionInstance<AppProvider>,

    /// The configuration of the test session.
    config: Config,

    /// The wallet used to send the requests to the Gateway.
    wallet: Wallet,

    /// The fhevm rust sdk used to compute the EIP712 for UserDecryptions.
    sdk: Arc<FhevmSdk>,
}

impl GatewayTestManager {
    /// Connects the tool to the Gateway.
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
                .unwrap()
        });

        let Some(blockchain_config) = config.blockchain.as_ref() else {
            return Err(anyhow!("Missing [blockchain] section in config file"));
        };

        let wallet = Wallet::from_config(blockchain_config).await?;
        let provider = NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .with_chain_id(blockchain_config.gateway_chain_id)
                .filler(FillersWithoutNonceManagement::default())
                .wallet(wallet.clone())
                .connect_http(blockchain_config.gateway_url.parse()?),
            wallet.address(),
        );
        info!("Successfully connected to the Gateway");
        let decryption_contract = Decryption::new(blockchain_config.decryption_address, provider);

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
            decryption_contract,
            config,
            wallet,
            sdk,
        })
    }

    /// Runs a decryption stress testing session via the Gateway chain.
    pub async fn stress_test(&self, args: GwTestArgs) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let pub_response_listener =
            init_public_decryption_response_listener(self.decryption_contract.clone()).await?;
        let user_response_listener =
            init_user_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listeners to be ready

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

            let (requests_pb, responses_pb) =
                init_progress_bars(&self.config, &progress_tracker, burst_index)?;

            match args.decryption_type {
                DecryptionType::Public => burst_tasks.spawn(
                    public_decryption_burst(
                        burst_index,
                        self.config.clone(),
                        self.decryption_contract.clone(),
                        pub_response_listener.clone(),
                        requests_pb,
                        responses_pb,
                    )
                    .in_current_span(),
                ),
                DecryptionType::User => burst_tasks.spawn(user_decryption_burst(
                    burst_index,
                    self.config.clone(),
                    self.decryption_contract.clone(),
                    Arc::clone(&self.sdk),
                    self.wallet.address(),
                    user_response_listener.clone(),
                    requests_pb,
                    responses_pb,
                )),
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

    /// Runs a decryption benchmark session via the Gateway chain.
    pub async fn decryption_benchmark(&self, args: GwBenchmarkArgs) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let pub_response_listener =
            init_public_decryption_response_listener(self.decryption_contract.clone()).await?;
        let user_response_listener =
            init_user_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listeners to be ready

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
                .perform_single_bench(
                    &bench_record,
                    &progress_tracker,
                    &mut burst_index,
                    pub_response_listener.clone(),
                    user_response_listener.clone(),
                )
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

    async fn perform_single_bench<PS, US>(
        &self,
        bench_record: &BenchRecordInput,
        progress_tracker: &MultiProgress,
        burst_index: &mut usize,
        pub_response_listener: Arc<Mutex<PS>>,
        user_response_listener: Arc<Mutex<US>>,
    ) -> anyhow::Result<Vec<BenchBurstResult>>
    where
        PS: Stream<Item = PublicDecryptThresholdEvent> + Unpin + Send + 'static,
        US: Stream<Item = UserDecryptThresholdEvent> + Unpin + Send + 'static,
    {
        let mut config = self.config.clone();
        config.parallel_requests = bench_record.parallel_requests;

        let mut results = vec![];
        for _ in 0..bench_record.number_of_measures {
            let (requests_pb, responses_pb) =
                init_progress_bars(&config, progress_tracker, *burst_index)?;

            let burst_result = match bench_record.decryption_type {
                DecryptionType::Public => {
                    public_decryption_burst(
                        *burst_index,
                        config.clone(),
                        self.decryption_contract.clone(),
                        pub_response_listener.clone(),
                        requests_pb,
                        responses_pb,
                    )
                    .await
                }
                DecryptionType::User => {
                    user_decryption_burst(
                        *burst_index,
                        config.clone(),
                        self.decryption_contract.clone(),
                        Arc::clone(&self.sdk),
                        self.wallet.address(),
                        user_response_listener.clone(),
                        requests_pb,
                        responses_pb,
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

/// Create progress bars to track a burst of requests sent to the Gateway.
///
/// - One is used to track when requests are received by the Gateway (tx receipt was received).
/// - The other is used to track when responses are received by the Gateway (response event was
///   catched).
fn init_progress_bars(
    config: &Config,
    progress_tracker: &MultiProgress,
    burst_index: usize,
) -> anyhow::Result<(ProgressBar, ProgressBar)> {
    let style = ProgressStyle::with_template(
        "{prefix:32} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
    )?
    .progress_chars("##-");
    let prefix = format!("Burst #{burst_index}");

    let requests_pb = progress_tracker.add(
        ProgressBar::new(config.parallel_requests.into())
            .with_prefix(prefix.clone())
            .with_message("Sending requests...")
            .with_style(style.clone()),
    );
    let responses_pb = progress_tracker.insert_after(
        &requests_pb,
        ProgressBar::new(config.parallel_requests.into())
            .with_prefix(prefix)
            .with_message("Waiting responses...")
            .with_style(style),
    );

    Ok((requests_pb, responses_pb))
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
