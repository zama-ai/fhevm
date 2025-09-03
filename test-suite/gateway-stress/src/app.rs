use crate::{
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    config::Config,
    decryption::{
        EVENT_LISTENER_POLLING, init_public_decryption_response_listener,
        init_user_decryption_response_listener, public_decryption_burst, user_decryption_burst,
    },
};
use alloy::{
    network::EthereumWallet,
    primitives::Address,
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{FillProvider, JoinFill, WalletFiller},
    },
};
use anyhow::anyhow;
use fhevm_gateway_bindings::decryption::Decryption::{self, DecryptionInstance};
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::sync::{Arc, Once};
use tokio::{
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::info;

/// The provider used to interact with the Gateway.
type AppProvider = NonceManagedProvider<
    FillProvider<
        JoinFill<JoinFill<Identity, FillersWithoutNonceManagement>, WalletFiller<EthereumWallet>>,
        RootProvider,
    >,
>;

/// A struct used to perform the load/stress testing of the Gateway.
pub struct App {
    /// The decryption contract instance.
    decryption_contract: DecryptionInstance<AppProvider>,

    /// The configuration of the test session.
    config: Config,

    /// The wallet used to send the requests to the Gateway.
    wallet: Wallet,

    /// The fhevm rust sdk used to compute the EIP712 for UserDecryptions.
    sdk: Arc<FhevmSdk>,
}

impl App {
    /// Connects the tool to the Gateway.
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        INSTALL_CRYPTO_PROVIDER_ONCE.call_once(|| {
            rustls::crypto::aws_lc_rs::default_provider()
                .install_default()
                .map_err(|e| anyhow!("Failed to install AWS-LC crypto provider: {e:?}"))
                .unwrap()
        });

        let wallet = Wallet::from_config(&config).await?;
        let gateway_url = &config.gateway_url;
        let provider = NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .filler(FillersWithoutNonceManagement::default())
                .wallet(wallet.clone())
                .connect_ws(WsConnect::new(gateway_url))
                .await
                .map_err(|e| anyhow!("Failed to connect to Gateway at {gateway_url}: {e}"))?,
            wallet.address(),
        );
        info!("Successfully connected to the Gateway");
        let decryption_contract = Decryption::new(config.decryption_address, provider);

        let sdk = Arc::new(
            FhevmSdkBuilder::new()
                .with_gateway_chain_id(config.gateway_chain_id)
                .with_decryption_contract(&config.decryption_address.to_string())
                .with_acl_contract(&Address::ZERO.to_string())
                .with_input_verification_contract(&Address::ZERO.to_string())
                .with_host_chain_id(config.host_chain_id)
                .build()?,
        );

        Ok(Self {
            decryption_contract,
            config,
            wallet,
            sdk,
        })
    }

    /// Performs the public decryption stress test.
    pub async fn public_decryption_stress_test(&self) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let response_listener =
            init_public_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listener to be ready

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
                self.init_progress_bars(&progress_tracker, burst_index)?;

            burst_tasks.spawn(public_decryption_burst(
                burst_index,
                self.config.clone(),
                self.decryption_contract.clone(),
                response_listener.clone(),
                requests_pb,
                responses_pb,
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

    /// Performs the user decryption stress test.
    pub async fn user_decryption_stress_test(&self) -> anyhow::Result<()> {
        let progress_tracker = MultiProgress::new();
        let response_listener =
            init_user_decryption_response_listener(self.decryption_contract.clone()).await?;
        tokio::time::sleep(EVENT_LISTENER_POLLING).await; // Sleep for listener to be ready

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
                self.init_progress_bars(&progress_tracker, burst_index)?;

            burst_tasks.spawn(user_decryption_burst(
                burst_index,
                self.config.clone(),
                self.decryption_contract.clone(),
                Arc::clone(&self.sdk),
                self.wallet.address(),
                response_listener.clone(),
                requests_pb,
                responses_pb,
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

    /// Create progress bars to track a burst of requests sent to the Gateway.
    ///
    /// - One is used to track when requests are received by the Gateway (tx receipt was received).
    /// - The other is used to track when responses are received by the Gateway (response event was
    ///   catched).
    fn init_progress_bars(
        &self,
        progress_tracker: &MultiProgress,
        burst_index: usize,
    ) -> anyhow::Result<(ProgressBar, ProgressBar)> {
        let style = ProgressStyle::with_template(
            "{prefix:32} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}",
        )?
        .progress_chars("##-");
        let prefix = format!("Burst #{burst_index}");

        let requests_pb = progress_tracker.add(
            ProgressBar::new(self.config.parallel_requests.into())
                .with_prefix(prefix.clone())
                .with_message("Sending requests...")
                .with_style(style.clone()),
        );
        let responses_pb = progress_tracker.insert_after(
            &requests_pb,
            ProgressBar::new(self.config.parallel_requests.into())
                .with_prefix(prefix)
                .with_message("Waiting responses...")
                .with_style(style),
        );

        Ok((requests_pb, responses_pb))
    }
}

static INSTALL_CRYPTO_PROVIDER_ONCE: Once = Once::new();
