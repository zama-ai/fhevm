use crate::{
    config::KmsWallet,
    conn::WalletGatewayProvider,
    tests::setup::{
        CHAIN_ID, DEPLOYER_PRIVATE_KEY,
        db::setup_test_db_instance,
        gw::{GatewayInstance, setup_anvil_gateway},
    },
};
use alloy::{
    node_bindings::AnvilInstance,
    primitives::ChainId,
    providers::{ProviderBuilder, WsConnect},
};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::DecryptionInstance,
    gatewayconfig::GatewayConfig::GatewayConfigInstance,
    kmsmanagement::KmsManagement::KmsManagementInstance,
};
use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, GenericImage};

pub async fn test_instance_with_db_only() -> anyhow::Result<TestInstance> {
    let (db_container, db) = setup_test_db_instance().await?;
    let test_instance = TestInstance::new(db_container, db);
    Ok(test_instance)
}

pub async fn test_instance_with_db_and_gw() -> anyhow::Result<TestInstance> {
    let anvil = setup_anvil_gateway().await?;
    let wallet = KmsWallet::from_private_key_str(
        DEPLOYER_PRIVATE_KEY,
        Some(ChainId::from(*CHAIN_ID as u64)),
    )?;

    let provider = ProviderBuilder::new()
        .wallet(wallet)
        .on_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;
    let gw_instance = GatewayInstance::new(anvil, provider);

    let test_instance = test_instance_with_db_only().await?;
    Ok(test_instance.with_gateway(gw_instance))
}

/// The integration test environment.
pub struct TestInstance {
    /// Use to enable tracing during tests.
    pub _tracing_default_guard: Option<tracing::subscriber::DefaultGuard>,
    /// Use to keep the database container running during the tests.
    pub _db_container: ContainerAsync<GenericImage>,
    pub db: Pool<Postgres>,
    pub gateway_instance: Option<GatewayInstance>,
}

impl TestInstance {
    /// `TestInstance` with database only.
    pub fn new(db_container: ContainerAsync<GenericImage>, db: Pool<Postgres>) -> Self {
        // Initialize tracing for this test
        let subscriber = tracing_subscriber::fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .finish();

        Self {
            _tracing_default_guard: Some(tracing::subscriber::set_default(subscriber)),
            _db_container: db_container,
            db,
            gateway_instance: None,
        }
    }

    /// Adds a `GatewayInstance` to the test environment.
    pub fn with_gateway(mut self, gateway_instance: GatewayInstance) -> Self {
        self.gateway_instance = Some(gateway_instance);
        self
    }

    pub fn anvil(&self) -> &AnvilInstance {
        &self.gateway().anvil
    }

    pub fn provider(&self) -> &WalletGatewayProvider {
        &self.gateway().provider
    }

    pub fn decryption_contract(&self) -> &DecryptionInstance<(), WalletGatewayProvider> {
        &self.gateway().decryption_contract
    }

    pub fn gateway_config_contract(&self) -> &GatewayConfigInstance<(), WalletGatewayProvider> {
        &self.gateway().gateway_config_contract
    }

    pub fn kms_management_contract(&self) -> &KmsManagementInstance<(), WalletGatewayProvider> {
        &self.gateway().kms_management_contract
    }

    pub fn disable_tracing(&mut self) {
        self._tracing_default_guard = None;
    }

    fn gateway(&self) -> &GatewayInstance {
        self.gateway_instance
            .as_ref()
            .expect("GatewayInstance has not been setup")
    }
}
