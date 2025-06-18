use alloy::{
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, FixedBytes},
    providers::{ProviderBuilder, WsConnect},
};
use connector_utils::conn::GatewayProvider;
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{self, DecryptionInstance},
    gatewayconfig::GatewayConfig::{self, GatewayConfigInstance},
    kmsmanagement::KmsManagement::{self, KmsManagementInstance},
};
use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, GenericImage, ImageExt, core::WaitFor, runners::AsyncRunner};

pub const DECRYPTION_MOCK_ADDRESS: Address = Address(FixedBytes([
    184, 174, 68, 54, 92, 69, 167, 197, 37, 107, 20, 246, 7, 202, 226, 59, 192, 64, 195, 84,
]));
pub const GATEWAY_CONFIG_MOCK_ADDRESS: Address = Address(FixedBytes([
    159, 167, 153, 249, 90, 114, 37, 140, 4, 21, 223, 237, 216, 207, 118, 210, 97, 60, 117, 15,
]));
pub const KMS_MANAGEMENT_MOCK_ADDRESS: Address = Address(FixedBytes([
    200, 27, 227, 169, 24, 21, 210, 212, 9, 109, 174, 8, 26, 113, 22, 201, 250, 123, 223, 8,
]));

pub const TEST_MNEMONIC: &str =
    "coyote sketch defense hover finger envelope celery urge panther venue verb cheese";

pub async fn test_instance_with_db_only() -> anyhow::Result<TestInstance> {
    let (db_container, db) = setup_test_db_instance().await?;
    let test_instance = TestInstance::new(db_container, db);
    Ok(test_instance)
}

pub async fn test_instance_with_db_and_gw() -> anyhow::Result<TestInstance> {
    let anvil = setup_anvil_gateway().await?;
    let provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;
    let gw_instance = GatewayInstance::new(anvil, provider);

    let test_instance = test_instance_with_db_only().await?;
    Ok(test_instance.with_gateway(gw_instance))
}

async fn setup_anvil_gateway() -> anyhow::Result<AnvilInstance> {
    let chain_id = rand::random::<u32>();
    let anvil = Anvil::new()
        .mnemonic(TEST_MNEMONIC)
        .block_time(1)
        .chain_id(chain_id as u64)
        .try_spawn()?;
    println!("Anvil started...");

    let _deploy_mock_container =
        GenericImage::new("ghcr.io/zama-ai/fhevm/gateway-contracts", "v0.7.0-rc0")
            .with_wait_for(WaitFor::message_on_stdout("Mock contract deployment done!"))
            .with_env_var("HARDHAT_NETWORK", "staging")
            .with_env_var("RPC_URL", anvil.endpoint_url().as_str())
            .with_env_var("CHAIN_ID_GATEWAY", format!("{chain_id}"))
            .with_env_var("MNEMONIC", TEST_MNEMONIC)
            .with_env_var(
                "DEPLOYER_ADDRESS",
                "0xCf28E90D4A6dB23c34E1881aEF5fd9fF2e478634",
            ) // accounts[1]
            .with_env_var(
                "DEPLOYER_PRIVATE_KEY",
                "0xe746bc71f6bee141a954e6a49bc9384d334e393a7ea1e70b50241cb2e78e9e4c",
            ) // accounts[1]
            .with_env_var(
                "PAUSER_ADDRESS",
                "0xfCefe53c7012a075b8a711df391100d9c431c468",
            )
            .with_network("host")
            .with_cmd(["npx hardhat task:deployGatewayMockContracts"])
            .start()
            .await?;
    println!("Mock contract successfully deployed on Anvil!");

    Ok(anvil)
}

async fn setup_test_db_instance() -> anyhow::Result<(ContainerAsync<GenericImage>, Pool<Postgres>)>
{
    let container = GenericImage::new("postgres", "17.5")
        .with_wait_for(WaitFor::message_on_stderr(
            "database system is ready to accept connections",
        ))
        .with_env_var("POSTGRES_USER", "postgres")
        .with_env_var("POSTGRES_PASSWORD", "postgres")
        .start()
        .await?;
    println!("Postgres started...");

    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(5432).await?;
    let admin_db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/postgres");
    let db_url = format!("postgresql://postgres:postgres@{cont_host}:{cont_port}/kms-connector");

    println!("Creating KMS Connector db...");
    let admin_pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(1)
        .connect(&admin_db_url)
        .await?;
    sqlx::query("CREATE DATABASE \"kms-connector\";")
        .execute(&admin_pool)
        .await?;
    println!("KMS Connector DB url: {db_url}");
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await?;

    println!("Running migrations...");
    sqlx::migrate!("../connector-db/migrations")
        .run(&pool)
        .await?;
    println!("KMS Connector DB ready!");

    Ok((container, pool))
}

pub struct TestInstance {
    pub _db_container: ContainerAsync<GenericImage>,
    pub db: Pool<Postgres>,
    pub gateway_instance: Option<GatewayInstance>,
}

impl TestInstance {
    /// `TestInstance` with database only.
    pub fn new(db_container: ContainerAsync<GenericImage>, db: Pool<Postgres>) -> Self {
        Self {
            _db_container: db_container,
            db,
            gateway_instance: None,
        }
    }

    pub fn with_gateway(mut self, gateway_instance: GatewayInstance) -> Self {
        self.gateway_instance = Some(gateway_instance);
        self
    }

    pub fn anvil(&self) -> &AnvilInstance {
        &self.gateway().anvil
    }

    pub fn provider(&self) -> &GatewayProvider {
        &self.gateway().provider
    }

    pub fn decryption_contract(&self) -> &DecryptionInstance<(), GatewayProvider> {
        &self.gateway().decryption_contract
    }

    pub fn gateway_config_contract(&self) -> &GatewayConfigInstance<(), GatewayProvider> {
        &self.gateway().gateway_config_contract
    }

    pub fn kms_management_contract(&self) -> &KmsManagementInstance<(), GatewayProvider> {
        &self.gateway().kms_management_contract
    }

    fn gateway(&self) -> &GatewayInstance {
        &self
            .gateway_instance
            .as_ref()
            .expect("GatewayInstance has not been setup")
    }
}

pub struct GatewayInstance {
    pub anvil: AnvilInstance,
    pub provider: GatewayProvider,
    pub decryption_contract: DecryptionInstance<(), GatewayProvider>,
    pub gateway_config_contract: GatewayConfigInstance<(), GatewayProvider>,
    pub kms_management_contract: KmsManagementInstance<(), GatewayProvider>,
}

impl GatewayInstance {
    pub fn new(anvil: AnvilInstance, provider: GatewayProvider) -> Self {
        let decryption_contract = Decryption::new(DECRYPTION_MOCK_ADDRESS, provider.clone());
        let gateway_config_contract =
            GatewayConfig::new(GATEWAY_CONFIG_MOCK_ADDRESS, provider.clone());
        let kms_management_contract =
            KmsManagement::new(KMS_MANAGEMENT_MOCK_ADDRESS, provider.clone());

        GatewayInstance {
            anvil,
            provider,
            decryption_contract,
            gateway_config_contract,
            kms_management_contract,
        }
    }
}
