use crate::{
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    config::Config,
    decryption::{
        init_public_decryption_response_listener, public_decryption_burst,
        user::{DURATION_DAYS, EXTRA_DATA, RAND_PUBLIC_KEY, generate_eip712},
    },
};
use alloy::{
    hex,
    network::EthereumWallet,
    primitives::{Address, U256},
    providers::{
        Identity, ProviderBuilder, RootProvider, WsConnect,
        fillers::{FillProvider, JoinFill, WalletFiller},
    },
};
use fhevm_gateway_rust_bindings::decryption::{
    Decryption::{self, CtHandleContractPair, DecryptionInstance},
    IDecryption::{ContractsInfo, RequestValidity},
};
use gateway_sdk::{FhevmSdk, FhevmSdkBuilder};
use indicatif::{MultiProgress, ProgressBar, ProgressStyle};
use std::{sync::Arc, time::SystemTime};
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
        let wallet = Wallet::from_config(&config).await?;
        let provider = NonceManagedProvider::new(
            ProviderBuilder::new()
                .disable_recommended_fillers()
                .filler(FillersWithoutNonceManagement::default())
                .wallet(wallet.clone())
                .connect_ws(WsConnect::new(&config.gateway_url))
                .await?,
            wallet.address(),
        );
        let decryption_contract = Decryption::new(config.decryption_address, provider);

        let sdk = Arc::new(
            FhevmSdkBuilder::new()
                .with_gateway_chain_id(config.host_chain_id)
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
        let user_addr = self.wallet.address();
        let timestamp = SystemTime::now()
            .duration_since(SystemTime::UNIX_EPOCH)?
            .as_secs();

        let sdk = Arc::clone(&self.sdk);
        let eip712 = generate_eip712(sdk, &self.config, timestamp)?;

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
                        durationDays: U256::from(DURATION_DAYS),
                    },
                    ContractsInfo {
                        chainId: U256::from(self.config.host_chain_id),
                        addresses: vec![self.config.allowed_contract],
                    },
                    user_addr,
                    hex::decode(RAND_PUBLIC_KEY)?.into(),
                    eip712.signature.clone().unwrap(),
                    EXTRA_DATA.into(),
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
