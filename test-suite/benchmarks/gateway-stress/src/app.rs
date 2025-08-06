use crate::{
    blockchain::{
        provider::{FillersWithoutNonceManagement, NonceManagedProvider},
        wallet::Wallet,
    },
    config::Config,
    decryption::{send_public_decryption, wait_for_response},
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
use std::time::SystemTime;
use tokio::{
    sync::mpsc,
    task::JoinSet,
    time::{Instant, interval},
};
use tracing::info;

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

    pub async fn public(&self) -> anyhow::Result<()> {
        let start = Instant::now();
        let mut interval = interval(self.config.tests_interval);
        let mut tasks = JoinSet::new();
        let (id_sender, id_receiver) = mpsc::unbounded_channel();

        info!("Subcribing to PublicDecryptionResponse events...");
        let response_filter = self
            .decryption_contract
            .PublicDecryptionResponse_filter()
            .watch()
            .await?;
        info!("Subcribed to PublicDecryptionResponse events!");

        tasks.spawn(wait_for_response(response_filter, id_receiver));
        loop {
            if start.elapsed() > self.config.tests_duration {
                break;
            }

            interval.tick().await;

            for index in 0..self.config.parallel_requests {
                tasks.spawn(send_public_decryption(
                    index,
                    self.decryption_contract.clone(),
                    self.config.ct_handles.clone(),
                    id_sender.clone(),
                ));
            }
        }
        drop(id_sender); // Dropping sender so `wait_for_responses` can exit properly

        tasks.join_all().await;
        Ok(())
    }

    pub async fn user(&self) -> anyhow::Result<()> {
        let contract_address = Address::default();
        let public_key =
            "2000000000000000a554e431f47ef7b1dd1b72a43432b06213a959953ec93785f2c699af9bc6f331";
        let user_addr = self.wallet.address();
        println!("user_addr: {user_addr}");
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
            println!("{receipt:?}")
        }
        Ok(())
    }
}
