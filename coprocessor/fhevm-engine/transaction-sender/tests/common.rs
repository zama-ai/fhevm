#![cfg(test)]
#![allow(dead_code)]

use alloy::providers::{Provider, ProviderBuilder};
use alloy::signers::aws::AwsSigner;
use alloy::signers::Signer;
use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, U256},
    signers::local::PrivateKeySigner,
    sol,
    transports::http::reqwest::Url,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use test_harness::containers::{
    create_kms_client, create_kms_signing_key, start_local_kms, TestContainer, LOCAL_KMS_PORT,
};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio_util::sync::CancellationToken;
use tracing::Level;
use transaction_sender::{get_chain_id, make_abstract_signer, AbstractSigner, ConfigSettings};

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

sol!(
    #[sol(rpc)]
    CiphertextCommits,
    "artifacts/CiphertextCommits.sol/CiphertextCommits.json"
);

pub enum SignerType {
    PrivateKey,
    AwsKms,
}

pub fn is_coprocessor_config_error(err: &str) -> bool {
    err.starts_with("NotCoprocessorSigner(")
        || err.starts_with("NotCoprocessorTxSender(")
        || err.starts_with("CoprocessorSignerDoesNotMatchTxSender(")
}

pub struct TestEnvironment {
    pub signer: AbstractSigner,
    pub conf: ConfigSettings,
    pub cancel_token: CancellationToken,
    pub db_pool: Pool<Postgres>,
    pub contract_address: Address,
    pub user_address: Address,
    anvil: Option<AnvilInstance>,
    pub wallet: EthereumWallet,
    _db_instance: DBInstance,
    // Just keep the handle to destroy the container when it is dropped.
    _local_kms: Option<TestContainer>,
}

impl TestEnvironment {
    pub async fn new(signer_type: SignerType) -> anyhow::Result<Self> {
        let force_per_test_local_kms = false;
        Self::new_with_config(
            signer_type,
            ConfigSettings::default(),
            force_per_test_local_kms,
        )
        .await
    }

    pub async fn new_with_config(
        signer_type: SignerType,
        conf: ConfigSettings,
        force_per_test_local_kms: bool,
    ) -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let db_instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow::anyhow!("{e}"))?;
        let db_pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(db_instance.db_url())
            .await?;

        Self::truncate_tables(
            &db_pool,
            vec![
                "verify_proofs",
                "ciphertext_digest",
                "allowed_handles",
                "delegate_user_decrypt",
                "keys",
                "crs",
                "host_chains",
            ],
        )
        .await?;

        let anvil = Self::new_anvil()?;
        let chain_id =
            get_chain_id(anvil.ws_endpoint_url(), std::time::Duration::from_secs(1)).await;
        let abstract_signer;
        let local_kms;
        match signer_type {
            SignerType::PrivateKey => {
                local_kms = None;
                let mut signer = PrivateKeySigner::from_signing_key(anvil.keys()[0].clone().into());
                signer.set_chain_id(Some(chain_id));
                abstract_signer = make_abstract_signer(signer);
            }
            SignerType::AwsKms => {
                let host_port;
                if std::env::var("TEST_GLOBAL_LOCAL_KMS").unwrap_or("0".to_string()) == "1"
                    && !force_per_test_local_kms
                {
                    local_kms = None;
                    host_port = LOCAL_KMS_PORT;
                } else {
                    local_kms = Some(start_local_kms().await?);
                    host_port = local_kms.as_ref().unwrap().host_port;
                }

                let aws_kms_client = create_kms_client(host_port).await?;
                let key_id = create_kms_signing_key(&aws_kms_client).await?;
                let signer = AwsSigner::new(aws_kms_client, key_id, Some(chain_id)).await?;
                Self::fund_from_anvil(&anvil, signer.address()).await?;

                abstract_signer = make_abstract_signer(signer);
            }
        }
        let wallet = abstract_signer.clone().into();
        Ok(Self {
            signer: abstract_signer,
            conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            contract_address: PrivateKeySigner::random().address(),
            user_address: PrivateKeySigner::random().address(),
            anvil: Some(anvil),
            wallet,
            _db_instance: db_instance,
            _local_kms: local_kms,
        })
    }

    pub fn ws_endpoint_url(&self) -> Url {
        self.anvil.as_ref().unwrap().ws_endpoint_url()
    }

    pub async fn recreate_anvil(&mut self) -> anyhow::Result<()> {
        let port = self.anvil.as_ref().unwrap().port();
        if let Some(old) = self.anvil.take() {
            drop(old);
        }
        let anvil = Self::new_anvil_with_port(port)?;

        // Re-fund the signer address if it's not one of Anvil's pre-funded keys.
        // This is needed for KMS signers whose address is not built into Anvil.
        let signer_address = self.wallet.default_signer().address();
        if !anvil.addresses().contains(&signer_address) {
            Self::fund_from_anvil(&anvil, signer_address).await?;
        }

        self.anvil = Some(anvil);
        Ok(())
    }

    pub fn drop_anvil(&mut self) {
        if let Some(a) = self.anvil.take() {
            drop(a);
        }
    }

    pub async fn stop_local_kms(&mut self) {
        if let Some(a) = self._local_kms.take() {
            a.container.stop().await.unwrap();
        }
    }

    /// Fund an address using Anvil's anvil_setBalance cheat code.
    /// local-kms generates a random key on CreateKey, it does not support
    /// importing known secp256k1 key material. So the derived address is unknown to Anvil
    /// and must be funded explicitly.
    async fn fund_from_anvil(anvil: &AnvilInstance, recipient: Address) -> anyhow::Result<()> {
        let provider = ProviderBuilder::new().connect_http(anvil.endpoint_url());
        provider
            .raw_request::<_, ()>(
                "anvil_setBalance".into(),
                (recipient, U256::from(10_000_000_000_000_000_000u128)), // 10 ETH
            )
            .await?;
        Ok(())
    }

    fn new_anvil() -> anyhow::Result<AnvilInstance> {
        Ok(Anvil::new().block_time(1).try_spawn()?)
    }

    fn new_anvil_with_port(port: u16) -> anyhow::Result<AnvilInstance> {
        Ok(Anvil::new().block_time(1).port(port).try_spawn()?)
    }

    async fn truncate_tables(db_pool: &sqlx::PgPool, tables: Vec<&str>) -> Result<(), sqlx::Error> {
        for table in tables {
            let query = format!("TRUNCATE {}", table);
            sqlx::query(&query).execute(db_pool).await?;
        }
        Ok(())
    }
}
