use alloy::{primitives::Address, signers::local::PrivateKeySigner, sol};
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use tokio_util::sync::CancellationToken;
use tracing::Level;
use transaction_sender::ConfigSettings;

sol!(
    #[sol(rpc)]
    ZKPoKManager,
    "artifacts/ZKPoKManager.sol/ZKPoKManager.json"
);

sol!(
    #[sol(rpc)]
    CiphertextStorage,
    "artifacts/CiphertextStorage.sol/CiphertextStorage.json"
);

pub struct TestEnvironment {
    pub signer: PrivateKeySigner,
    pub conf: ConfigSettings,
    pub cancel_token: CancellationToken,
    pub db_pool: Pool<Postgres>,
    pub contract_address: Address,
    pub user_address: Address,
}

impl TestEnvironment {
    pub async fn new() -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let conf = ConfigSettings::default();

        let db_pool = PgPoolOptions::new()
            .max_connections(1)
            .connect(&conf.database_url)
            .await?;

        // Delete all proofs from the database.
        sqlx::query!("TRUNCATE verify_proofs",)
            .execute(&db_pool)
            .await?;

        Ok(Self {
            signer: PrivateKeySigner::random(),
            conf: conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            contract_address: PrivateKeySigner::random().address(),
            user_address: PrivateKeySigner::random().address(),
        })
    }
}
