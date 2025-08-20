use std::time::Duration;

use crate::{
    conn::WalletGatewayProvider,
    tests::setup::{CustomTestWriter, DbInstance, KmsInstance, S3Instance, gw::GatewayInstance},
};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::DecryptionInstance,
    gateway_config::GatewayConfig::GatewayConfigInstance,
    kms_management::KmsManagement::KmsManagementInstance,
};
use sqlx::{Pool, Postgres};
use testcontainers::{ContainerAsync, GenericImage};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tracing_subscriber::EnvFilter;

/// The integration test environment.
pub struct TestInstance {
    /// Use to enable tracing during tests.
    _tracing_default_guard: Option<tracing::subscriber::DefaultGuard>,
    db: Option<DbInstance>,
    gateway: Option<GatewayInstance>,
    s3: Option<S3Instance>,
    kms: Option<KmsInstance>,

    /// Receiver channel to read/check log printed via the `tracing` crate.
    log_rx: UnboundedReceiver<Vec<u8>>,
}

fn logs_contain(logs: &[u8], bytes: &[u8]) -> bool {
    logs.windows(bytes.len()).any(|b| b == bytes)
}

impl TestInstance {
    pub fn builder() -> TestInstanceBuilder {
        TestInstanceBuilder::default()
    }

    /// Consumes the logs of the `log_rx` channel until it finds the expected one.
    pub async fn wait_for_log(&mut self, log: &str) {
        let mut logs_received = Vec::<u8>::new();
        while !logs_contain(&logs_received, log.as_bytes()) {
            let next_log = self.log_rx.recv().await.expect("log channel closed");
            let mut split_log = next_log.split(|b| *b == b'\n');
            let mut line_end = split_log.next().unwrap().to_vec();
            logs_received.append(&mut line_end);

            if logs_contain(&logs_received, log.as_bytes()) {
                break;
            }

            if let Some(next_line) = split_log.next() {
                logs_received = next_line.to_vec();
            }
        }
    }

    pub fn db(&self) -> &Pool<Postgres> {
        &self.db.as_ref().expect("DB is not setup").db
    }

    pub fn db_container(&self) -> &ContainerAsync<GenericImage> {
        &self.db.as_ref().expect("DB is not setup").db_container
    }

    pub fn db_url(&self) -> &str {
        &self.db.as_ref().expect("DB is not setup").url
    }

    pub fn kms_url(&self) -> &str {
        &self
            .kms
            .as_ref()
            .expect("KmsInstance has not been setup")
            .url
    }

    pub fn provider(&self) -> &WalletGatewayProvider {
        &self.gateway().provider
    }

    pub fn anvil_container(&self) -> &ContainerAsync<GenericImage> {
        &self.gateway().anvil
    }

    pub fn decryption_contract(&self) -> &DecryptionInstance<WalletGatewayProvider> {
        &self.gateway().decryption_contract
    }

    pub fn gateway_config_contract(&self) -> &GatewayConfigInstance<WalletGatewayProvider> {
        &self.gateway().gateway_config_contract
    }

    pub fn kms_management_contract(&self) -> &KmsManagementInstance<WalletGatewayProvider> {
        &self.gateway().kms_management_contract
    }

    fn gateway(&self) -> &GatewayInstance {
        self.gateway
            .as_ref()
            .expect("GatewayInstance has not been setup")
    }

    pub fn s3_url(&self) -> &str {
        &self.s3.as_ref().expect("S3 has not been setup").url
    }

    pub fn anvil_block_time(&self) -> Duration {
        self.gateway().anvil_block_time()
    }

    pub fn anvil_ws_endpoint(&self) -> String {
        self.gateway().anvil_ws_endpoint()
    }

    pub fn kms_container(&self) -> &ContainerAsync<GenericImage> {
        &self.kms.as_ref().expect("KMS has not been setup").container
    }
}

pub struct TestInstanceBuilder {
    _tracing_default_guard: Option<tracing::subscriber::DefaultGuard>,
    db: Option<DbInstance>,
    gateway: Option<GatewayInstance>,
    s3: Option<S3Instance>,
    kms: Option<KmsInstance>,
    log_rx: UnboundedReceiver<Vec<u8>>,
}

impl Default for TestInstanceBuilder {
    fn default() -> Self {
        let (log_tx, log_rx) = mpsc::unbounded_channel();

        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(
                EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")),
            )
            .with_writer(CustomTestWriter::new(log_tx))
            .finish();

        Self {
            _tracing_default_guard: Some(tracing::subscriber::set_default(subscriber)),
            db: None,
            gateway: None,
            s3: None,
            kms: None,
            log_rx,
        }
    }
}

impl TestInstanceBuilder {
    pub fn with_db(mut self, db_instance: DbInstance) -> Self {
        self.db = Some(db_instance);
        self
    }

    pub fn with_gateway(mut self, gateway_instance: GatewayInstance) -> Self {
        self.gateway = Some(gateway_instance);
        self
    }

    pub fn with_s3(mut self, s3_instance: S3Instance) -> Self {
        self.s3 = Some(s3_instance);
        self
    }

    pub fn with_kms(mut self, kms_instance: KmsInstance) -> Self {
        self.kms = Some(kms_instance);
        self
    }

    pub fn with_tracing(mut self, tracing: Option<tracing::subscriber::DefaultGuard>) -> Self {
        self._tracing_default_guard = tracing;
        self
    }

    pub fn build(self) -> TestInstance {
        TestInstance {
            _tracing_default_guard: self._tracing_default_guard,
            db: self.db,
            gateway: self.gateway,
            s3: self.s3,
            kms: self.kms,
            log_rx: self.log_rx,
        }
    }

    /// Test setup with a DB only.
    pub async fn db_setup() -> anyhow::Result<TestInstance> {
        let builder = TestInstanceBuilder::default();
        let db = DbInstance::setup().await?;
        Ok(builder.with_db(db).build())
    }

    /// Test setup with a DB and Anvil Gateway.
    pub async fn db_gw_setup() -> anyhow::Result<TestInstance> {
        let builder = TestInstanceBuilder::default();
        let db = DbInstance::setup().await?;
        let gateway = GatewayInstance::setup().await?;
        Ok(builder.with_db(db).with_gateway(gateway).build())
    }

    /// Full test setup.
    pub async fn full() -> anyhow::Result<TestInstance> {
        let s3_instance = S3Instance::setup().await?;
        let kms_instance = KmsInstance::setup(&s3_instance.url).await?;
        let test_instance_builder = TestInstanceBuilder::default()
            .with_db(DbInstance::setup().await?)
            .with_gateway(GatewayInstance::setup().await?)
            .with_s3(s3_instance)
            .with_kms(kms_instance);
        Ok(test_instance_builder.build())
    }
}
