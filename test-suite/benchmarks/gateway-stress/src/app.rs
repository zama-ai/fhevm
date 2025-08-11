use crate::{
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    config::Config,
    decryption::{init_public_decryption_response_listener, public_decryption_burst},
};
use alloy::{
    hex,
    network::EthereumWallet,
    primitives::U256,
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{FillProvider, JoinFill, WalletFiller},
    },
};
use fhevm_gateway_rust_bindings::decryption::{
    Decryption::{self, CtHandleContractPair, DecryptionInstance},
    IDecryption::RequestValidity,
};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::time::SystemTime;
use tokio::{
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::{debug, info};

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
    decryption_contract: DecryptionInstance<(), AppProvider>,

    /// The configuration of the test session.
    config: Config,

    /// The wallet used to send the requests to the Gateway.
    wallet: Wallet,
}

impl App {
    /// Connects the tool to the Gateway.
    pub async fn connect(config: Config) -> anyhow::Result<Self> {
        let wallet = Wallet::from_config(&config).await?;
        let provider = NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .filler(FillersWithoutNonceManagement::default())
                .wallet(wallet.clone())
                .on_ws(WsConnect::new(&config.gateway_url))
                .await?,
            wallet.address(),
        );
        let decryption_contract = Decryption::new(config.decryption_address, provider);

        Ok(Self {
            decryption_contract,
            config,
            wallet,
        })
    }

    /// Performs the public decryption stress test.
    pub async fn public_decryption_stress_test(&self) -> anyhow::Result<()> {
        let session_start = Instant::now();
        let mut interval = interval(self.config.tests_interval);

        let mut burst_tasks = JoinSet::new();
        let mut burst_index = 1;
        let progress_tracker = MultiProgress::new();
        let response_listener =
            init_public_decryption_response_listener(self.decryption_contract.clone()).await?;

        loop {
            interval.tick().await;

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
    pub async fn user(&self) -> anyhow::Result<()> {
        let public_key =
            "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
        let user_addr = self.wallet.address();
        let signature = "1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef1234567890abcdef12";
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();
        for handle in &self.config.ct_handles {
            let decryption_call = self
                .decryption_contract
                .userDecryptionRequest(
                    vec![CtHandleContractPair {
                        ctHandle: *handle,
                        contractAddress: self.config.allowed_contract,
                    }],
                    RequestValidity {
                        startTimestamp: U256::from(timestamp),
                        durationDays: U256::ONE,
                    },
                    U256::ZERO, // host chainId
                    vec![self.config.allowed_contract],
                    user_addr,
                    hex::decode(public_key)?.into(),
                    hex::decode(signature)?.into(),
                )
                .send()
                .await?;

            let receipt = decryption_call.get_receipt().await?;
            debug!("{receipt:?}")
        }
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
