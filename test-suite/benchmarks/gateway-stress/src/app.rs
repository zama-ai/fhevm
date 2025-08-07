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
    primitives::{Address, U256},
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

type AppProvider = NonceManagedProvider<
    FillProvider<
        JoinFill<JoinFill<Identity, FillersWithoutNonceManagement>, WalletFiller<EthereumWallet>>,
        RootProvider,
    >,
>;

pub struct App {
    decryption_contract: DecryptionInstance<(), AppProvider>,
    config: Config,
    wallet: Wallet,
}

impl App {
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
            (self.config.parallel_requests * (burst_index - 1) as u32) as f64 / elapsed as f64
        );
        Ok(())
    }

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

    pub async fn user(&self) -> anyhow::Result<()> {
        let contract_address = Address::default();
        let public_key =
            "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
        let user_addr = self.wallet.address();
        debug!("user_addr: {user_addr}");
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
                        contractAddress: contract_address,
                    }],
                    RequestValidity {
                        startTimestamp: U256::from(timestamp),
                        durationDays: U256::ONE,
                    },
                    U256::ZERO, // host chainId
                    vec![contract_address],
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
}
