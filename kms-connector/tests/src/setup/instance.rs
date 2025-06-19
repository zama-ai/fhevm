use crate::setup::{
    db::setup_test_db_instance,
    gw::{GatewayInstance, setup_anvil_gateway},
};
use alloy::{
    node_bindings::AnvilInstance,
    providers::{ProviderBuilder, WsConnect},
};
use connector_utils::conn::GatewayProvider;
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
    let provider = ProviderBuilder::new()
        .on_ws(WsConnect::new(anvil.ws_endpoint_url()))
        .await?;
    let gw_instance = GatewayInstance::new(anvil, provider);

    let test_instance = test_instance_with_db_only().await?;
    Ok(test_instance.with_gateway(gw_instance))
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
