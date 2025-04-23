use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::Address,
    signers::local::PrivateKeySigner,
    sol,
};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::Level;
use transaction_sender::ConfigSettings;

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
    MultichainAcl,
    "artifacts/MultichainAcl.sol/MultichainAcl.json"
);

pub struct TestEnvironment {
    pub signer: PrivateKeySigner,
    pub conf: ConfigSettings,
    pub cancel_token: CancellationToken,
    pub db_pool: Pool<Postgres>,
    #[allow(dead_code)]
    pub contract_address: Address,
    #[allow(dead_code)]
    pub user_address: Address,
    #[allow(dead_code)]
    pub anvil: AnvilInstance,
    #[allow(dead_code)]
    pub wallet: EthereumWallet,
}

impl TestEnvironment {
    pub async fn new() -> anyhow::Result<Self> {
        Self::new_with_config(ConfigSettings::default()).await
    }

    pub async fn new_with_config(conf: ConfigSettings) -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&conf.database_url)
            .await?;

        truncate_tables(
            &db_pool,
            vec!["verify_proofs", "ciphertext_digest", "allowed_handles"],
        )
        .await?;

        let anvil = Anvil::new().try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.clone().into();
        Ok(Self {
            signer,
            conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            contract_address: PrivateKeySigner::random().address(),
            user_address: PrivateKeySigner::random().address(),
            anvil,
            wallet,
        })
    }
}

async fn truncate_tables(db_pool: &sqlx::PgPool, tables: Vec<&str>) -> Result<(), sqlx::Error> {
    for table in tables {
        let query = format!("TRUNCATE {}", table);
        sqlx::query(&query).execute(db_pool).await?;
    }
    Ok(())
}
