use crate::{
    conn::WalletProvider,
    tests::setup::{
        CustomTestWriter, DbInstance, KmsInstance, S3Instance, blockchain::BlockchainInstance,
    },
};
use alloy::transports::http::reqwest::Url;
use fhevm_gateway_bindings::{
    decryption::Decryption::DecryptionInstance,
    gateway_config::GatewayConfig::GatewayConfigInstance,
};
use fhevm_host_bindings::{
    kms_generation::KMSGeneration::KMSGenerationInstance,
    protocol_config::ProtocolConfig::ProtocolConfigInstance,
};
use sqlx::{Pool, Postgres};
use std::time::Duration;
use testcontainers::{ContainerAsync, GenericImage};
use tokio::sync::mpsc::{self, UnboundedReceiver};
use tracing_subscriber::EnvFilter;

/// The integration test environment.
pub struct TestInstance {
    /// Use to enable tracing during tests.
    _tracing_default_guard: Option<tracing::subscriber::DefaultGuard>,
    db: Option<DbInstance>,
    blockchain: Option<BlockchainInstance>,
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
        self.db
            .as_ref()
            .expect("DB is not setup")
            .db_container
            .as_ref()
            .expect(
                "DB container is not available when running against an external Postgres server",
            )
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

    pub fn provider(&self) -> &WalletProvider {
        &self.blockchain().provider
    }

    pub fn anvil_container(&self) -> &ContainerAsync<GenericImage> {
        &self.blockchain().anvil
    }

    pub fn decryption_contract(&self) -> &DecryptionInstance<WalletProvider> {
        &self.blockchain().decryption_contract
    }

    pub fn gateway_config_contract(&self) -> &GatewayConfigInstance<WalletProvider> {
        &self.blockchain().gateway_config_contract
    }

    pub fn kms_generation_contract(&self) -> &KMSGenerationInstance<WalletProvider> {
        &self.blockchain().kms_generation_contract
    }

    pub fn protocol_config_contract(&self) -> &ProtocolConfigInstance<WalletProvider> {
        &self.blockchain().protocol_config_contract
    }

    fn blockchain(&self) -> &BlockchainInstance {
        self.blockchain
            .as_ref()
            .expect("BlockchainInstance has not been setup")
    }

    pub fn s3_url(&self) -> &str {
        &self.s3.as_ref().expect("S3 has not been setup").url
    }

    pub fn anvil_block_time(&self) -> Duration {
        self.blockchain().anvil_block_time()
    }

    pub fn anvil_http_endpoint(&self) -> Url {
        self.blockchain().anvil_http_endpoint()
    }

    pub fn kms_container(&self) -> &ContainerAsync<GenericImage> {
        &self.kms.as_ref().expect("KMS has not been setup").container
    }
}

pub struct TestInstanceBuilder {
    _tracing_default_guard: Option<tracing::subscriber::DefaultGuard>,
    db: Option<DbInstance>,
    blockchain: Option<BlockchainInstance>,
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
            blockchain: None,
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

    pub fn with_blockchain(mut self, blockchain_instance: BlockchainInstance) -> Self {
        self.blockchain = Some(blockchain_instance);
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
            blockchain: self.blockchain,
            s3: self.s3,
            kms: self.kms,
            log_rx: self.log_rx,
        }
    }

    /// Test setup with a DB only.
    ///
    /// Uses an already-running, shared Postgres server (one fresh database per test) rather than
    /// spinning up a container, which is much faster for the many DB-only tests.
    pub async fn db_setup() -> anyhow::Result<TestInstance> {
        let builder = TestInstanceBuilder::default();
        let db = DbInstance::setup_external().await?;
        Ok(builder.with_db(db).build())
    }

    /// Test setup with a DB and Anvil blockchain.
    pub async fn db_bc_setup() -> anyhow::Result<TestInstance> {
        let builder = TestInstanceBuilder::default();
        let db = DbInstance::setup_external().await?;
        let blockchain = BlockchainInstance::setup().await?;
        Ok(builder.with_db(db).with_blockchain(blockchain).build())
    }
}
