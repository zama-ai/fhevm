#![cfg(test)]
#![allow(dead_code)]

use alloy::signers::aws::AwsSigner;
use alloy::signers::Signer;
use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::Address,
    signers::local::PrivateKeySigner,
    sol,
    transports::http::reqwest::Url,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use test_harness::localstack::{
    create_aws_aws_kms_client, create_localstack_kms_signing_key, start_localstack,
    LocalstackContainer, LOCALSTACK_PORT,
};
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

sol!(
    #[sol(rpc)]
    MultichainACL,
    "artifacts/MultichainACL.sol/MultichainACL.json"
);

pub enum SignerType {
    PrivateKey,
    AwsKms,
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
    // Just keep the handle to destroy the container when it is dropped.
    _localstack: Option<LocalstackContainer>,
}

impl TestEnvironment {
    pub async fn new(signer_type: SignerType) -> anyhow::Result<Self> {
        let force_per_test_localstack = false;
        Self::new_with_config(
            signer_type,
            ConfigSettings::default(),
            force_per_test_localstack,
        )
        .await
    }

    pub async fn new_with_config(
        signer_type: SignerType,
        conf: ConfigSettings,
        force_per_test_localstack: bool,
    ) -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(conf.database_url.as_str())
            .await?;

        Self::truncate_tables(
            &db_pool,
            vec!["verify_proofs", "ciphertext_digest", "allowed_handles"],
        )
        .await?;

        let anvil = Self::new_anvil()?;
        let chain_id =
            get_chain_id(anvil.ws_endpoint_url(), std::time::Duration::from_secs(1)).await;
        let abstract_signer;
        let localstack;
        match signer_type {
            SignerType::PrivateKey => {
                localstack = None;
                let mut signer = PrivateKeySigner::from_signing_key(anvil.keys()[0].clone().into());
                signer.set_chain_id(Some(chain_id));
                abstract_signer = make_abstract_signer(signer);
            }
            SignerType::AwsKms => {
                let host_port;
                if std::env::var("TEST_GLOBAL_LOCALSTACK").unwrap_or("0".to_string()) == "1"
                    && !force_per_test_localstack
                {
                    localstack = None;
                    host_port = LOCALSTACK_PORT;
                } else {
                    localstack = Some(start_localstack().await?);
                    host_port = localstack.as_ref().unwrap().host_port;
                }

                let aws_kms_client = create_aws_aws_kms_client(host_port).await?;
                let key_id =
                    create_localstack_kms_signing_key(&aws_kms_client, &anvil.keys()[0].to_bytes())
                        .await?;
                let signer = AwsSigner::new(aws_kms_client, key_id, Some(chain_id)).await?;
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
            _localstack: localstack,
        })
    }

    pub fn ws_endpoint_url(&self) -> Url {
        self.anvil.as_ref().unwrap().ws_endpoint_url()
    }

    pub fn recreate_anvil(&mut self) -> anyhow::Result<()> {
        if let Some(old) = self.anvil.take() {
            drop(old);
        }
        self.anvil = Some(Self::new_anvil()?);
        Ok(())
    }

    pub fn drop_anvil(&mut self) {
        if let Some(a) = self.anvil.take() {
            drop(a);
        }
    }

    pub async fn stop_localstack(&mut self) {
        if let Some(a) = self._localstack.take() {
            a.container.stop().await.unwrap();
        }
    }

    fn new_anvil() -> anyhow::Result<AnvilInstance> {
        Ok(Anvil::new().port(13389_u16).try_spawn()?)
    }

    async fn truncate_tables(db_pool: &sqlx::PgPool, tables: Vec<&str>) -> Result<(), sqlx::Error> {
        for table in tables {
            let query = format!("TRUNCATE {}", table);
            sqlx::query(&query).execute(db_pool).await?;
        }
        Ok(())
    }
}
