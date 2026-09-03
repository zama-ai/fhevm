use anyhow::Context;
use ethereum_rpc_mock::{
    ct_attestation::CtAttestationMock, fhevm::FhevmMockWrapper, MockConfig, MockServer,
    MockServerHandle, Response, UsageLimit,
};
use fhevm_relayer::config::settings::{
    GwCiphertextCheckConfig, HostChainConfig, KeyUrlConfig, Settings, StorageConfig,
};
use fhevm_relayer::run_fhevm_relayer;
use fhevm_relayer::store::sql::client::PgClient;
use fhevm_relayer::tracing::init_tracing_once;

use alloy::primitives::{hex, Address, Bytes, Log, B256, U256};
use alloy::signers::{local::PrivateKeySigner, SignerSync};
use alloy::sol_types::{SolCall, SolEvent, SolValue};
use fhevm_gateway_bindings::decryption::IDecryption::{
    RequestValiditySeconds, UserDecryptionRequestPayload,
};
use fhevm_host_bindings::acl::ACL;
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_host_bindings::ikms_generation::IKMSGeneration;
use rand::{rng, RngExt};
use std::str::FromStr;
use std::sync::Arc;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;
use user_decryption_signature::{compute_user_decrypt_digest, default_user_decrypt_domain};

use super::test_schema::TestSchema;

/// The config every integration test starts from.
// This module is compiled into each test binary separately, and only some of them name this
// constant, so the rest would see it as dead code under `-D warnings`.
#[allow(dead_code)]
pub const TEST_CONFIG_PATH: &str = "tests/relayer-test-config.yaml";

/// Gateway contract addresses the mocks and [`TEST_CONFIG_PATH`] agree on.
const TEST_INPUT_VERIFICATION_ADDRESS: &str = "0xe61cff9c581c7c91aef682c2c10e8632864339ab";

/// Must match `gateway.readiness_checker.gw_ciphertext_check.gateway_config_address` in
/// [`TEST_CONFIG_PATH`]: under `source: coprocessor_attestations` the relayer reads the Coprocessor
/// registry from it while starting up. `source: gateway_chain` carries no such key.
const TEST_GATEWAY_CONFIG_ADDRESS: &str = "0x576Ea67208b146E63C5255d0f90104E25e3e04c7";

/// Host-chain mock server, with every response the relayer needs to boot already registered.
///
/// Held by the test setup: dropping it shuts the listener down.
#[allow(dead_code)]
pub struct HostMock {
    pub port: u16,
    pub server: MockServer,
    handle: MockServerHandle,
}

impl HostMock {
    /// Start a host-chain mock on an OS-assigned port, reported as `self.port`, and register
    /// the responses every test relies on: ACL allow-all for the configured host chains, plus
    /// the `KMSGeneration` / `ProtocolConfig` getters the `/v2/keyurl` poller reads. `settings`
    /// is only read for contract addresses, so it can be an unwired copy of the test config.
    #[allow(dead_code)]
    pub async fn start(settings: &Settings) -> anyhow::Result<Self> {
        let config = MockConfig {
            port: 0,
            ..MockConfig::new()
        };
        let server = MockServer::new(config);
        let handle = server
            .clone()
            .start()
            .await
            .context("Failed to start host mock server")?;
        let port = handle.port();
        tracing::debug!("Started Host chain MockServer on port {}", port);

        register_default_host_acl_allow_all(&server, &settings.host_chains);

        let protocol_config_addr = Address::from_str(&settings.protocol_config.address)
            .expect("Invalid protocol_config address in test config");
        match &settings.keyurl {
            // The relayer gates startup on the `/v2/keyurl` poller's first successful poll, so
            // these have to answer before it boots.
            KeyUrlConfig::Chain {
                kms_generation_address,
                ..
            } => {
                let kms_generation_addr = Address::from_str(kms_generation_address)
                    .expect("Invalid kms_generation_address in test config");
                register_default_keyurl_poller_responses(
                    &server,
                    kms_generation_addr,
                    protocol_config_addr,
                );
            }
            KeyUrlConfig::Config { .. } => {
                register_missing_kms_context_getters(&server, protocol_config_addr);
            }
        }

        Ok(HostMock {
            port,
            server,
            handle,
        })
    }
}

/// Which ciphertext-readiness source a [`GatewayMock`] is wired for. Only the selected one gets
/// fixtures: wiring both would let a test pass on answers the relayer never asked for.
#[allow(dead_code)]
pub enum ReadinessSource<'a> {
    /// `Decryption.isPublicDecryptionReady` / `isUserDecryptionReady_1`. Deliberately left
    /// unarmed: patterns are matched first-registered-first, so a ready-by-default pattern would
    /// shadow every later `set_readiness_failure()`. Each test arms its own outcome.
    GatewayChain,
    /// Off-chain buckets, reached through the `GatewayConfig` Coprocessor registry.
    CoprocessorAttestations(&'a CtAttestationMock),
}

/// Gateway-chain mock server with the FHEVM patterns and the selected readiness source wired.
///
/// Held by the test setup: dropping it shuts the listener down.
#[allow(dead_code)]
pub struct GatewayMock {
    pub port: u16,
    pub fhevm: FhevmMockWrapper,
    handle: MockServerHandle,
}

impl GatewayMock {
    /// Start a gateway-chain mock on an OS-assigned port, reported as `self.port`. FHEVM patterns
    /// and `readiness`'s fixtures are registered before the server listens, so the relayer cannot
    /// observe a half-configured gateway.
    #[allow(dead_code)]
    pub async fn start(readiness: ReadinessSource<'_>) -> anyhow::Result<Self> {
        let config = MockConfig {
            port: 0,
            ..MockConfig::new()
        };
        let server = MockServer::new(config);

        let fhevm = FhevmMockWrapper::new(
            server.clone(),
            Address::from_str(TEST_DECRYPTION_ADDRESS).expect("Invalid decryption address"),
            Address::from_str(TEST_INPUT_VERIFICATION_ADDRESS)
                .expect("Invalid input verification address"),
            Address::from_str(TEST_GATEWAY_CONFIG_ADDRESS).expect("Invalid gateway config address"),
        );
        match readiness {
            // Deliberately unarmed — see the variant's doc.
            ReadinessSource::GatewayChain => {}
            ReadinessSource::CoprocessorAttestations(ct_attestation) => {
                fhevm.set_coprocessor_registry(ct_attestation)
            }
        }

        let handle = server
            .start()
            .await
            .context("Failed to start gateway mock server")?;
        let port = handle.port();
        tracing::debug!("Started Gateway chain MockServer on port {}", port);

        Ok(GatewayMock {
            port,
            fhevm,
            handle,
        })
    }
}

/// Point `settings` at the mock servers and the isolated test schema.
///
/// HTTP and metrics bind to port 0; the actually-bound addresses come back through
/// [`spawn_relayer`]. DB pools are kept small so parallel tests don't exhaust a CI Postgres.
#[allow(dead_code)]
pub fn wire_settings_to_mocks(
    settings: &mut Settings,
    host_port: u16,
    gateway_port: u16,
    database_url: String,
) {
    settings.storage.app_pool.max_connections = 2;
    settings.storage.app_pool.min_connections = 0;
    // Cron pool kept small — expiry worker is disabled by default.
    // Minimum allowed value is 1 connection.
    settings.storage.cron_pool.max_connections = 1;
    settings.storage.cron_pool.min_connections = 0;
    settings.storage.sql_database_url = database_url;

    settings.http.endpoint = Some("0.0.0.0:0".to_string());
    settings.metrics.endpoint = "0.0.0.0:0".to_string();

    settings.gateway.blockchain_rpc.http_url = format!("http://localhost:{}", gateway_port);
    settings.gateway.blockchain_rpc.read_http_url = format!("http://localhost:{}", gateway_port);
    let ws_url = format!("ws://localhost:{}", gateway_port);
    for listener in &mut settings.gateway.listener_pool.listeners {
        listener.url = ws_url.clone();
    }

    for hc in &mut settings.host_chains {
        hc.url = format!("http://localhost:{}", host_port);
    }
    settings.protocol_config.ethereum_http_rpc_url = format!("http://localhost:{}", host_port);
}

/// Spawn the relayer and wait for it to echo its settings back.
///
/// The echo is the startup handshake: receiving it means every startup gate passed, and the
/// returned settings carry the actually-bound HTTP / metrics addresses.
#[allow(dead_code)]
pub async fn spawn_relayer(
    settings: Settings,
    cancellation_token: CancellationToken,
) -> anyhow::Result<(JoinHandle<()>, Settings)> {
    let (settings_tx, settings_rx) = oneshot::channel::<Settings>();

    let relayer_handle = tokio::spawn(async move {
        match run_fhevm_relayer(settings, cancellation_token, Some(settings_tx)).await {
            Ok(()) => tracing::debug!("Relayer service exited normally"),
            Err(e) => tracing::error!("Relayer service error: {:#}", e),
        }
    });

    let updated_settings = settings_rx
        .await
        .context("Failed to receive settings from relayer")?;

    tracing::debug!("Relayer service started successfully with actual ports");

    Ok((relayer_handle, updated_settings))
}

/// Read the actually-bound HTTP port out of the settings echoed by [`spawn_relayer`].
#[allow(dead_code)]
pub fn http_port_of(settings: &Settings) -> anyhow::Result<u16> {
    settings
        .http
        .endpoint
        .as_ref()
        .and_then(|endpoint| endpoint.rsplit(':').next())
        .and_then(|port| port.parse::<u16>().ok())
        .context("Failed to parse HTTP port from settings")
}

/// Who is responsible for the Postgres schema a [`TestSetup`] runs against.
///
/// A [`TestSetup::join`]ed instance runs its relayer against the same schema as the peer it
/// joined (that is the point - see `join`'s doc comment), so it must never be the one to drop
/// it: only the original owner does, and only from its own `shutdown`.
enum SchemaHandle {
    Owned(TestSchema),
    /// A peer `TestSetup` owns the schema at this URL; nothing to clean up here.
    Shared {
        database_url: String,
    },
}

impl SchemaHandle {
    fn database_url(&self) -> String {
        match self {
            SchemaHandle::Owned(schema) => schema.database_url(),
            SchemaHandle::Shared { database_url } => database_url.clone(),
        }
    }
}

/// Per-test isolated setup with own ports, database, and mock servers
#[allow(dead_code)]
pub struct TestSetup {
    pub fhevm_mock: FhevmMockWrapper,
    /// Coprocessor buckets backing the off-chain readiness check, or `None` when the setup runs
    /// against `source: gateway_chain` and starts no buckets at all. Reach it through
    /// [`TestSetup::ct_attestation`].
    ct_attestation: Option<Arc<CtAttestationMock>>,
    pub host_server: MockServer,
    pub settings: Settings,
    pub http_port: u16,
    /// Ports of the mock servers backing [`Self::fhevm_mock`] / [`Self::host_server`], kept
    /// around (independent of who owns those mocks) so [`TestSetup::join`] can wire a second
    /// relayer at them without needing `host`/`gateway` to be `Some`.
    host_port: u16,
    gateway_port: u16,
    /// `None` for an instance built by [`TestSetup::join`]: it shares a peer's mock servers
    /// rather than starting its own, so there is nothing here for `shutdown` to drop.
    host: Option<HostMock>,
    gateway: Option<GatewayMock>,
    cancellation_token: CancellationToken,
    relayer_handle: JoinHandle<()>,
    test_schema: SchemaHandle,
}

impl TestSetup {
    /// The Coprocessor buckets backing the off-chain readiness check. Arm the per-test outcome on
    /// this (`serve_attestations`, `serve_nothing`, ...).
    ///
    /// Panics under `source: gateway_chain`, which starts no buckets.
    #[allow(dead_code)]
    pub fn ct_attestation(&self) -> &CtAttestationMock {
        self.ct_attestation.as_deref().expect(
            "this TestSetup runs against source: gateway_chain and starts no attestation buckets \
             — arm readiness through `fhevm_mock.set_readiness_*` instead",
        )
    }

    /// Create test setup with fast readiness config (4 attempts × 250ms = ~1s total)
    /// This config is used in tests for readiness check timing out.
    #[allow(dead_code)]
    pub async fn new_with_fast_readiness() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path =
            create_readiness_config(&temp_config_dir, "fast_readiness.yaml", 4, 250, None, None)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with minimal readiness retries (2 attempts × 50ms = ~100ms total)
    /// Use when the test doesn't need many retries (e.g., contract errors that fail immediately).
    #[allow(dead_code)]
    pub async fn new_with_minimal_readiness() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_readiness_config(
            &temp_config_dir,
            "minimal_readiness.yaml",
            2,
            50,
            None,
            None,
        )?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create a test setup whose overall request budget expires long before its retry attempts do.
    ///
    /// The retry counters are deliberately generous (100 × 50ms) while `request_timeout_ms` is
    /// tiny, so a test can prove the wall-clock budget is what ends the check. `head_timeout_ms` is
    /// compressed too: the budget only means something relative to how long one bucket probe may
    /// take, and `validate()` rejects a budget below a single probe's timeout.
    #[allow(dead_code)]
    pub async fn new_with_short_request_budget() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_readiness_config(
            &temp_config_dir,
            "short_request_budget.yaml",
            100,
            50,
            Some(50),
            Some(200),
        )?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create a test setup gated on the on-chain Gateway check (`source: gateway_chain`), armed
    /// through `fhevm_mock.set_readiness_*` rather than attestation buckets. Retries are minimal
    /// (2 × 50ms) so a failing check does not wait out a production-shaped budget.
    #[allow(dead_code)]
    pub async fn new_with_gateway_chain_readiness() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_gateway_chain_readiness_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with low retry config (2 attempts × 100ms)
    /// This config is used in tests for max retries exceeded scenarios.
    #[allow(dead_code)]
    pub async fn new_with_low_retries() -> anyhow::Result<Self> {
        // Create temp config with low retry settings
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_low_retry_config(&temp_config_dir)?;

        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create isolated test setup with free ports and temp database
    #[allow(dead_code)]
    pub async fn new() -> anyhow::Result<Self> {
        // Create a temp config based on the example config
        let temp_config_dir = tempfile::TempDir::new()?;
        let temp_config_path = create_default_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with specified number of listener instances (for redundancy testing)
    #[allow(dead_code)]
    pub async fn new_with_listeners(listener_count: usize) -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_listener_config(&temp_config_dir, listener_count)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with two host chains (chain_id 8009 and 9001) for cross-chain tests.
    #[allow(dead_code)]
    pub async fn new_with_multi_chain() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_multi_chain_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup that keeps serving for `wait` after its health check starts
    /// failing, instead of the zero the other setups use.
    #[allow(dead_code)]
    pub async fn new_with_lb_propagation_wait(wait: &str) -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_lb_propagation_config(&temp_config_dir, wait)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with admin endpoint enabled
    #[allow(dead_code)]
    pub async fn new_with_admin_endpoint() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_admin_endpoint_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create setup with optional custom config path
    #[allow(dead_code)]
    pub async fn new_with_config_path(
        config_path: Option<std::path::PathBuf>,
    ) -> anyhow::Result<Self> {
        Self::new_with_config_path_and_settings(config_path, |_| {}).await
    }

    /// Start a relayer with `mutate` applied to its settings *after* they have been wired to
    /// the mocks and the isolated schema, for config the other constructors do not expose - the
    /// dispatch-gate tests use it to pin `dispatcher_lock.key_override` onto a key the test
    /// itself holds. Same base config as [`TestSetup::new`].
    #[allow(dead_code)]
    pub async fn new_with_settings(mutate: impl FnOnce(&mut Settings)) -> anyhow::Result<Self> {
        let temp_config_dir = tempfile::TempDir::new()?;
        let temp_config_path = create_default_config(&temp_config_dir)?;
        Self::new_with_config_path_and_settings(Some(temp_config_path), mutate).await
    }

    async fn new_with_config_path_and_settings(
        config_path: Option<std::path::PathBuf>,
        mutate: impl FnOnce(&mut Settings),
    ) -> anyhow::Result<Self> {
        // Create isolated test schema first
        let test_schema = TestSchema::new().await?;
        tracing::info!(
            "Created isolated test schema: {}",
            test_schema.schema_name()
        );

        // Create settings from config file (default or custom)
        let config_path_str = config_path.map(|p| p.to_string_lossy().to_string());
        let mut settings =
            Settings::new(config_path_str.clone()).expect("Failed to load configuration");

        // Initialize tracing once with settings
        init_tracing_once(&settings.log);

        let host = HostMock::start(&settings).await?;

        let (ct_attestation, gateway) =
            match &settings.gateway.readiness_checker.gw_ciphertext_check {
                GwCiphertextCheckConfig::CoprocessorAttestations { .. } => {
                    // Attested by default — nearly every test needs an available ciphertext; tests
                    // wanting another outcome re-arm the buckets themselves.
                    let ct_attestation = Arc::new(CtAttestationMock::start().await);
                    ct_attestation.serve_attestations().await;
                    let gateway = GatewayMock::start(ReadinessSource::CoprocessorAttestations(
                        &ct_attestation,
                    ))
                    .await?;
                    (Some(ct_attestation), gateway)
                }
                // No ready-by-default counterpart here — see ReadinessSource::GatewayChain.
                GwCiphertextCheckConfig::GatewayChain { .. } => (
                    None,
                    GatewayMock::start(ReadinessSource::GatewayChain).await?,
                ),
            };
        let (host_port, gateway_port) = (host.port, gateway.port);

        tracing::info!(
            "Mock servers listening on ports {} (host), {} (gateway)",
            host_port,
            gateway_port
        );

        wire_settings_to_mocks(
            &mut settings,
            host_port,
            gateway_port,
            test_schema.database_url(),
        );
        mutate(&mut settings);

        // Start relayer service with isolated settings
        let cancellation_token = CancellationToken::new();
        let (relayer_handle, settings) =
            spawn_relayer(settings, cancellation_token.clone()).await?;

        let http_port = http_port_of(&settings)?;

        tracing::info!(
            "Isolated test setup complete with actual ports - gateway: {}, http: {}, metrics: {}",
            gateway_port,
            settings
                .http
                .endpoint
                .as_ref()
                .unwrap_or(&"none".to_string()),
            settings.metrics.endpoint
        );

        Ok(TestSetup {
            fhevm_mock: gateway.fhevm.clone(),
            ct_attestation,
            host_server: host.server.clone(),
            settings,
            http_port,
            host_port,
            gateway_port,
            host: Some(host),
            gateway: Some(gateway),
            cancellation_token,
            relayer_handle,
            test_schema: SchemaHandle::Owned(test_schema),
        })
    }

    /// Start a second relayer process against `existing`'s own schema and mock servers - two
    /// real relayer instances against one shared database schema, rather
    /// than the single-relayer-plus-faked-peer shape `dispatch_gate_test.rs` uses.
    ///
    /// Sharing the schema is what makes the two instances contend for the dispatcher lock for
    /// real: the lock key is derived from `current_schema()` (see
    /// `orchestrator::dispatcher_lock`), so two relayers pointed at the same schema resolve the
    /// same key on their own, with no need for `dispatcher_lock.key_override` - prefer that
    /// natural contention over the override for tests like these; the override stays available
    /// (`mutate`) for a test that needs the winner to be deterministic instead.
    ///
    /// Sharing the mock servers (rather than starting a second pair) is what makes the test's
    /// `fhevm_mock` / `host_server` expectations apply no matter which of the two pods ends up
    /// dispatching. Gets its own HTTP/metrics ports, same as any other constructor - the spawn
    /// path already binds port 0 and echoes the real address back. `ct_attestation` is an `Arc`
    /// clone of `existing`'s, since both instances' gateway mocks share one `GatewayConfig`
    /// registry pointing at the same attestation buckets.
    ///
    /// The returned instance does not own the schema or the mocks, so its `shutdown()` leaves
    /// them alone: dropping either while `existing` is still running would pull the database or
    /// the RPC mocks out from under it. Always shut a joined instance down *before* the one it
    /// joined.
    #[allow(dead_code)]
    pub async fn join(
        existing: &TestSetup,
        mutate: impl FnOnce(&mut Settings),
    ) -> anyhow::Result<Self> {
        let temp_config_dir = tempfile::TempDir::new()?;
        let temp_config_path = create_default_config(&temp_config_dir)?;
        let mut settings = Settings::new(Some(temp_config_path.to_string_lossy().to_string()))
            .expect("Failed to load configuration");

        init_tracing_once(&settings.log);

        wire_settings_to_mocks(
            &mut settings,
            existing.host_port,
            existing.gateway_port,
            existing.test_schema.database_url(),
        );
        mutate(&mut settings);

        let cancellation_token = CancellationToken::new();
        let (relayer_handle, settings) =
            spawn_relayer(settings, cancellation_token.clone()).await?;
        let http_port = http_port_of(&settings)?;

        tracing::info!(
            "Joined test setup complete, sharing schema and mocks with the peer - http: {}",
            settings
                .http
                .endpoint
                .as_ref()
                .unwrap_or(&"none".to_string())
        );

        Ok(TestSetup {
            fhevm_mock: existing.fhevm_mock.clone(),
            ct_attestation: existing.ct_attestation.clone(),
            host_server: existing.host_server.clone(),
            settings,
            http_port,
            host_port: existing.host_port,
            gateway_port: existing.gateway_port,
            host: None,
            gateway: None,
            cancellation_token,
            relayer_handle,
            test_schema: SchemaHandle::Shared {
                database_url: existing.test_schema.database_url(),
            },
        })
    }

    /// Start the shutdown sequence without waiting for it, so a test can observe the relayer
    /// while it is shutting down. `shutdown` still has to run afterwards to clean up.
    #[allow(dead_code)]
    pub fn begin_shutdown(&self) {
        self.cancellation_token.cancel();
    }

    #[allow(dead_code)]
    pub async fn shutdown(self) {
        self.cancellation_token.cancel();

        // Only wait for relayer - it has the DB connections
        if let Err(e) = self.relayer_handle.await {
            tracing::error!("Test relayer task failed: {}", e);
        }

        // Mock servers will shutdown when their handles are dropped - `None` for a joined
        // instance, which never started its own (see `TestSetup::join`).
        drop(self.host);
        drop(self.gateway);

        // Clean up the test schema, but only if this instance owns it - see `SchemaHandle`.
        if let SchemaHandle::Owned(mut schema) = self.test_schema {
            if let Err(e) = schema.cleanup().await {
                tracing::error!("Failed to cleanup test schema: {}", e);
            }
        }
    }
}

/// Create a default config file based on the example
fn create_default_config(temp_dir: &tempfile::TempDir) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("test_config.yaml");

    // Simply copy the example config without modifications
    std::fs::copy("tests/relayer-test-config.yaml", &temp_config_path)
        .context("Failed to copy example config")?;

    Ok(temp_config_path)
}

/// Create a config file with a second host chain entry for cross-chain tests.
///
/// Reads `local.yaml.example`, appends a host chain with `TEST_HOST_CHAIN_ID_2`
/// and `TEST_HOST_ACL_ADDRESS_2`, and writes to a temp file.
fn create_multi_chain_config(temp_dir: &TempDir) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("multi_chain.yaml");

    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Append second host chain entry
    if let Some(host_chains) = config.get_mut("host_chains") {
        if let Some(seq) = host_chains.as_sequence_mut() {
            let mut entry = serde_yaml::Mapping::new();
            entry.insert(
                serde_yaml::Value::String("chain_id".to_string()),
                serde_yaml::Value::Number(serde_yaml::Number::from(TEST_HOST_CHAIN_ID_2)),
            );
            entry.insert(
                serde_yaml::Value::String("url".to_string()),
                serde_yaml::Value::String("http://localhost:8545".to_string()),
            );
            entry.insert(
                serde_yaml::Value::String("acl_address".to_string()),
                serde_yaml::Value::String(TEST_HOST_ACL_ADDRESS_2.to_string()),
            );
            seq.push(serde_yaml::Value::Mapping(entry));
        }
    }

    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file with fast readiness settings (4 attempts × 250ms)
/// `head_timeout_ms` / `request_timeout_ms` are `None` for tests that only care about the retry
/// counters and want the base config's production-shaped timeouts. A test that needs the overall
/// request budget to expire during the test must set both, since the budget is only meaningful
/// relative to how long a single bucket probe is allowed to take.
fn create_readiness_config(
    temp_dir: &TempDir,
    filename: &str,
    max_attempts: u32,
    retry_interval_ms: u32,
    head_timeout_ms: Option<u64>,
    request_timeout_ms: Option<u64>,
) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join(filename);

    // Read the default config
    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Modify the readiness checker retry settings (both gw_ciphertext_check and host_acl_check)
    if let Some(gateway) = config.get_mut("gateway") {
        if let Some(readiness_checker) = gateway.get_mut("readiness_checker") {
            if let Some(gw_ciphertext_check) = readiness_checker.get_mut("gw_ciphertext_check") {
                if let Some(retry) = gw_ciphertext_check.get_mut("retry") {
                    retry["max_attempts"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(max_attempts));
                    retry["retry_interval_ms"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(retry_interval_ms));
                }
                if let Some(head_timeout_ms) = head_timeout_ms {
                    gw_ciphertext_check["head_timeout_ms"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(head_timeout_ms));
                }
                if let Some(request_timeout_ms) = request_timeout_ms {
                    gw_ciphertext_check["request_timeout_ms"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(request_timeout_ms));
                }
            }
            if let Some(host_acl_check) = readiness_checker.get_mut("host_acl_check") {
                if let Some(retry) = host_acl_check.get_mut("retry") {
                    retry["max_attempts"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(max_attempts));
                    retry["retry_interval_ms"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(retry_interval_ms));
                }
            }
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file gated on the on-chain Gateway check (`source: gateway_chain`).
///
/// Replaces the whole `gw_ciphertext_check` mapping rather than patching it: serde ignores the
/// attestation variant's leftover keys, so patching would test a config shape nobody deploys.
fn create_gateway_chain_readiness_config(temp_dir: &TempDir) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("gateway_chain_readiness.yaml");

    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    let mut retry = serde_yaml::Mapping::new();
    retry.insert(
        serde_yaml::Value::String("max_attempts".to_string()),
        serde_yaml::Value::Number(serde_yaml::Number::from(2)),
    );
    retry.insert(
        serde_yaml::Value::String("retry_interval_ms".to_string()),
        serde_yaml::Value::Number(serde_yaml::Number::from(50)),
    );

    let mut gw_ciphertext_check = serde_yaml::Mapping::new();
    gw_ciphertext_check.insert(
        serde_yaml::Value::String("source".to_string()),
        serde_yaml::Value::String("gateway_chain".to_string()),
    );
    gw_ciphertext_check.insert(
        serde_yaml::Value::String("retry".to_string()),
        serde_yaml::Value::Mapping(retry),
    );

    config["gateway"]["readiness_checker"]["gw_ciphertext_check"] =
        serde_yaml::Value::Mapping(gw_ciphertext_check);

    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;
    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file with low retry settings for tx_engine (2 attempts × 100ms)
/// This config is used in tests for max retries exceeded scenarios.
/// Copy the test config, setting how long shutdown keeps serving after `/healthz` fails.
fn create_lb_propagation_config(
    temp_dir: &TempDir,
    wait: &str,
) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("lb_propagation.yaml");

    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    config["shutdown"]["lb_propagation_wait"] = serde_yaml::Value::String(wait.to_string());

    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;
    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

fn create_low_retry_config(temp_dir: &TempDir) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("low_retry.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Modify the tx_engine retry settings to low values
    if let Some(gateway) = config.get_mut("gateway") {
        if let Some(tx_engine) = gateway.get_mut("tx_engine") {
            if let Some(retry) = tx_engine.get_mut("retry") {
                retry["max_attempts"] = serde_yaml::Value::Number(serde_yaml::Number::from(2));
                retry["retry_interval_ms"] =
                    serde_yaml::Value::Number(serde_yaml::Number::from(100));
            }
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file with admin endpoint enabled
fn create_admin_endpoint_config(temp_dir: &TempDir) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("admin_endpoint.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Enable admin endpoint
    if let Some(http) = config.get_mut("http") {
        http["enable_admin_endpoint"] = serde_yaml::Value::Bool(true);
    }

    // Serialize back to YAML and write to temp file
    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file with specified number of listener instances
fn create_listener_config(
    temp_dir: &TempDir,
    listener_count: usize,
) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("listener_config.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Modify the listeners array in listener_pool
    if let Some(gateway) = config.get_mut("gateway") {
        if let Some(listener_pool) = gateway.get_mut("listener_pool") {
            // Build the listeners array based on count
            let mut listeners = Vec::new();
            for _ in 0..listener_count {
                let mut listener = serde_yaml::Mapping::new();
                listener.insert(
                    serde_yaml::Value::String("type".to_string()),
                    serde_yaml::Value::String("subscription".to_string()),
                );
                listener.insert(
                    serde_yaml::Value::String("url".to_string()),
                    serde_yaml::Value::String("ws://localhost:8757".to_string()),
                );
                listeners.push(serde_yaml::Value::Mapping(listener));
            }
            listener_pool["listeners"] = serde_yaml::Value::Sequence(listeners);
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Create a config file with fast timeout settings for testing timeout behavior.
#[allow(dead_code)]
pub fn create_timeout_test_config(
    temp_dir: &TempDir,
    timeout_secs: u64,
    cron_interval_secs: u64,
) -> anyhow::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("timeout_test.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("tests/relayer-test-config.yaml")
        .context("Failed to read default config")?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value =
        serde_yaml::from_str(&config_content).context("Failed to parse YAML config")?;

    // Configure fast timeout settings for testing
    if let Some(storage) = config.get_mut("storage") {
        if let Some(cron) = storage.get_mut("cron") {
            cron["timeout_cron_interval"] =
                serde_yaml::Value::String(format!("{}s", cron_interval_secs));
            cron["public_decrypt_timeout"] =
                serde_yaml::Value::String(format!("{}s", timeout_secs));
            cron["user_decrypt_timeout"] = serde_yaml::Value::String(format!("{}s", timeout_secs));
            cron["input_proof_timeout"] = serde_yaml::Value::String(format!("{}s", timeout_secs));
            // Set startup delay to 0s for timeout tests - we want cron workers to start immediately
            cron["cron_startup_delay_after_recovery"] = serde_yaml::Value::String("0s".to_string());
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content =
        serde_yaml::to_string(&config).context("Failed to serialize modified config")?;

    std::fs::write(&temp_config_path, modified_content).context("Failed to write temp config")?;

    Ok(temp_config_path)
}

/// Generate a random Ethereum address for testing
#[allow(dead_code)]
pub fn random_address() -> Address {
    let mut rng = rng();
    let bytes: [u8; 20] = rng.random();
    Address::from(bytes)
}

/// Default host chain ID used in test config (local.yaml.example).
pub const TEST_HOST_CHAIN_ID: u64 = 8009;

/// Second host chain ID for cross-chain tests.
pub const TEST_HOST_CHAIN_ID_2: u64 = 9001;

/// ACL contract address for the second host chain (cross-chain tests).
pub const TEST_HOST_ACL_ADDRESS_2: &str = "0x2222222222222222222222222222222222222222";

/// Generate a random handle (64 hex characters) with a valid host chain ID.
///
/// Bytes 22..30 are set to the configured chain_id (big-endian) so that
/// `HostChainIdChecker::validate_handles` passes.
#[allow(dead_code)]
pub fn random_handle() -> String {
    random_handle_with_chain_id(TEST_HOST_CHAIN_ID)
}

/// Decryption contract address from the test config — the EIP-712 verifying contract the v3
/// signature pre-check uses.
pub const TEST_DECRYPTION_ADDRESS: &str = "0xB8Ae44365c45A7C5256b14F607CaE23BC040c354";

/// Fixed EOA used to sign v3 user-decryption requests in tests.
#[allow(dead_code)]
pub fn user_decrypt_test_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str("0x1111111111111111111111111111111111111111111111111111111111111111")
        .expect("valid test private key")
}

/// Sign a v3 unified user-decryption envelope in place: recompute the EIP-712 digest from the
/// envelope's `attestedPayload` fields and write a real EOA `signature`. The caller must have set
/// `attestedPayload.userAddress` to `signer`'s address. The domain chain id is read from the first
/// handle (as the relayer does); the verifying contract is [`TEST_DECRYPTION_ADDRESS`].
#[allow(dead_code)]
pub fn sign_v3_user_decrypt_envelope(payload: &mut serde_json::Value, signer: &PrivateKeySigner) {
    let p = &payload["attestedPayload"];

    let user_address = Address::from_str(p["userAddress"].as_str().unwrap()).unwrap();
    let public_key = Bytes::from_str(p["publicKey"].as_str().unwrap()).unwrap();
    let allowed_contracts: Vec<Address> = p["allowedContracts"]
        .as_array()
        .unwrap()
        .iter()
        .map(|a| Address::from_str(a.as_str().unwrap()).unwrap())
        .collect();
    let start = U256::from_str(p["requestValidity"]["startTimestamp"].as_str().unwrap()).unwrap();
    let duration =
        U256::from_str(p["requestValidity"]["durationSeconds"].as_str().unwrap()).unwrap();
    let extra_data = Bytes::from_str(p["extraData"].as_str().unwrap()).unwrap();

    // Chain id is encoded at bytes 22..30 of the first handle.
    let handle_hex = p["handles"][0]["ctHandle"].as_str().unwrap();
    let handle_bytes = hex::decode(handle_hex.strip_prefix("0x").unwrap_or(handle_hex)).unwrap();
    let chain_id = u64::from_be_bytes(handle_bytes[22..30].try_into().unwrap());

    let domain = default_user_decrypt_domain(
        chain_id,
        Address::from_str(TEST_DECRYPTION_ADDRESS).unwrap(),
    );
    let request = UserDecryptionRequestPayload {
        userAddress: user_address,
        publicKey: public_key,
        allowedContracts: allowed_contracts,
        requestValidity: RequestValiditySeconds {
            startTimestamp: start,
            durationSeconds: duration,
        },
        extraData: extra_data,
        signature: Bytes::new(),
    };
    let digest = compute_user_decrypt_digest(&request, &domain);
    let signature = signer.sign_hash_sync(&digest).unwrap();
    payload["signature"] =
        serde_json::Value::String(format!("0x{}", hex::encode(signature.as_bytes())));
}

/// Generate a random handle with a specific chain_id embedded at bytes 22..30.
#[allow(dead_code)]
pub fn random_handle_with_chain_id(chain_id: u64) -> String {
    let mut rng = rng();
    let mut bytes = [0u8; 32];
    for b in &mut bytes {
        *b = rng.random_range(0..=255);
    }
    // Embed chain_id at bytes 22..30 (big-endian)
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    format!("0x{}", hex::encode(bytes))
}

/// Setup test database connection
/// Note: Run `make migrate` before running tests that use SQL repositories
#[allow(dead_code)]
pub async fn setup_test_database(config: StorageConfig) -> anyhow::Result<PgClient> {
    let pg_client = PgClient::new(config).await?;
    Ok(pg_client)
}

/// Validates that a 202 response has a valid Retry-After header with numeric value
#[allow(dead_code)]
pub fn assert_retry_after_header_present(response: &reqwest::Response) {
    let retry_after_header = response
        .headers()
        .get("retry-after")
        .or_else(|| response.headers().get("Retry-After"))
        .and_then(|header_val| header_val.to_str().ok())
        .and_then(|header_str| header_str.parse::<u32>().ok());
    assert!(
        retry_after_header.is_some(),
        "202 response should have valid Retry-After header"
    );
}

/// Common helper for testing v2 API timeout behavior
/// Performs the full timeout test flow:
/// 1. POST request → Assert 202 "queued" with job_id
/// 2. Initial poll → Assert 202 "queued"
/// 3. Wait for timeout to occur
/// 4. Final poll → Assert 503 "failed" with error message
#[allow(dead_code)]
pub async fn test_v2_timeout_flow(
    post_url: String,
    get_url_fn: impl Fn(&str) -> String,
    payload: serde_json::Value,
    timeout_duration_secs: u64,
    cron_interval_secs: u64,
    initial_poll_delay_ms: u64,
) {
    let client = reqwest::Client::new();

    // Step 1: POST request - should return 202 with job_id
    let response = client
        .post(&post_url)
        .header("Content-Type", "application/json")
        .timeout(std::time::Duration::from_secs(10))
        .json(&payload)
        .send()
        .await
        .expect("Failed to send POST request");

    assert_eq!(
        response.status(),
        reqwest::StatusCode::ACCEPTED,
        "Expected 202 ACCEPTED from POST request"
    );
    let post_response: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse POST response");

    assert_eq!(
        post_response["status"], "queued",
        "Expected status 'queued', got response: {:?}",
        post_response
    );
    let job_id = post_response["result"]["jobId"]
        .as_str()
        .unwrap_or_else(|| {
            panic!(
                "jobId should be present in response. Full response: {:?}",
                post_response
            )
        });

    // Step 2: Poll status - should initially return 202 "queued"
    tokio::time::sleep(tokio::time::Duration::from_millis(initial_poll_delay_ms)).await;

    let response = client
        .get(get_url_fn(job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to GET status");

    let response_status = response.status();
    if response_status != reqwest::StatusCode::ACCEPTED {
        let error_body = response
            .text()
            .await
            .unwrap_or_else(|_| "Could not read response body".to_string());
        panic!(
            "Expected 202 for queued request, got {}. Response body: {}",
            response_status, error_body
        );
    }

    let status: serde_json::Value = response.json().await.expect("Failed to parse status");
    assert_eq!(status["status"], "queued");

    // Step 3: Wait for timeout to occur (timeout + cron interval + buffer)
    let wait_time =
        tokio::time::Duration::from_secs(timeout_duration_secs + cron_interval_secs + 5);
    tokio::time::sleep(wait_time).await;

    // Step 4: Poll status - should now return 503 "failed"
    let response = client
        .get(get_url_fn(job_id))
        .timeout(std::time::Duration::from_secs(10))
        .send()
        .await
        .expect("Failed to GET status after timeout");

    assert_eq!(
        response.status(),
        reqwest::StatusCode::SERVICE_UNAVAILABLE,
        "Expected 503 Gateway Timeout"
    );
    let status: serde_json::Value = response
        .json()
        .await
        .expect("Failed to parse timeout status");
    assert_eq!(status["status"], "failed");
    assert!(
        status["error"].is_object(),
        "Error details should be present for timeout"
    );
    let error_message = status["error"]["message"]
        .as_str()
        .expect("Error should have message field");
    assert!(
        error_message.contains("did not respond within the expected timeframe"),
        "Error message should indicate timeout"
    );
    assert_eq!(
        status["error"]["label"].as_str(),
        Some("response_timed_out"),
        "Expected label 'response_timed_out' for cron-based timeout"
    );
}

// ---------------------------------------------------------------------------
// Host ACL mock helpers
// ---------------------------------------------------------------------------

/// Extract the number of calls encoded in a multicall calldata.
///
/// ABI layout of `multicall(bytes[])`:
///   [0..4]   selector
///   [4..36]  offset to array data (0x20)
///   [36..68] array length
pub fn extract_multicall_count(input: &Bytes) -> usize {
    if input.len() < 68 {
        return 0;
    }
    // Array length is the uint256 at bytes 36..68; only the last 8 bytes matter for test counts.
    let len_bytes: [u8; 8] = input[60..68].try_into().unwrap_or([0u8; 8]);
    usize::try_from(u64::from_be_bytes(len_bytes)).unwrap_or(0)
}

/// Register default ACL multicall allow-all patterns on the host mock server.
///
/// Registers one pattern per expected call count so the response contains exactly
/// the right number of results. The predicate inspects the multicall calldata to
/// determine the count.
fn register_default_host_acl_allow_all(host_server: &MockServer, host_chains: &[HostChainConfig]) {
    // Cover all counts used by tests:
    //   1 — public decrypt (1 handle) and delegated user decrypt (1 pair)
    //   2 — user decrypt (1 pair → 2 isAllowed calls: user + contract)
    for count in 1..=2 {
        let response_bytes = host_acl_multicall_allow_response(count);

        for hc in host_chains {
            let acl_address =
                Address::from_str(&hc.acl_address).expect("Invalid ACL address in config");
            let multicall_selector = ACL::multicallCall::SELECTOR;

            host_server.on_call(
                move |params| {
                    params.to == acl_address
                        && params.input.len() >= 4
                        && params.input[0..4] == multicall_selector
                        && extract_multicall_count(&params.input) == count
                },
                Response::call_success(response_bytes.clone()),
                UsageLimit::Unlimited,
            );
        }
    }
}

/// Build ABI-encoded multicall response with `count` all-true boolean results.
///
/// Each result is an ABI-encoded `bool(true)` (32 bytes).
/// The outer encoding matches the `multicall(bytes[]) returns (bytes[])` return type.
#[allow(dead_code)]
pub fn host_acl_multicall_allow_response(count: usize) -> Bytes {
    let true_value = {
        let mut buf = vec![0u8; 32];
        buf[31] = 1;
        Bytes::from(buf)
    };

    let results: Vec<Bytes> = vec![true_value; count];
    Bytes::from(results.abi_encode_params())
}

/// Build ABI-encoded multicall response where specific indices are denied (false).
///
/// `denied` is a set of indices to mark as false; all others are true.
#[allow(dead_code)]
pub fn host_acl_multicall_deny_response(count: usize, denied: &[usize]) -> Bytes {
    let results: Vec<Bytes> = (0..count)
        .map(|i| {
            let mut buf = vec![0u8; 32];
            buf[31] = if denied.contains(&i) { 0 } else { 1 };
            Bytes::from(buf)
        })
        .collect();
    Bytes::from(results.abi_encode_params())
}

/// Register an ACL multicall pattern that denies all handles on the host mock.
///
/// Auto-detects the call count from the multicall calldata, so callers don't
/// need to know the per-request-type count (1 for public decrypt, 2 for user
/// decrypt, etc.).
#[allow(dead_code)]
pub fn register_host_acl_deny_all(host_server: &MockServer, acl_address: Address) {
    let multicall_selector = ACL::multicallCall::SELECTOR;

    host_server.on_call_dynamic(
        move |params| {
            params.to == acl_address
                && params.input.len() >= 4
                && params.input[0..4] == multicall_selector
        },
        move |params| {
            let count = extract_multicall_count(&params.input);
            let denied: Vec<usize> = (0..count).collect();
            Response::call_success(host_acl_multicall_deny_response(count, &denied))
        },
        UsageLimit::Unlimited,
    );
}

/// Register a count-aware ACL multicall allow-all pattern using `on_call_dynamic`.
///
/// Unlike `register_default_host_acl_allow_all` which only covers counts 1-2,
/// this handles any call count by inspecting the multicall calldata at runtime.
#[allow(dead_code)]
pub fn register_host_acl_allow_all_dynamic(host_server: &MockServer, acl_address: Address) {
    let multicall_selector = ACL::multicallCall::SELECTOR;

    host_server.on_call_dynamic(
        move |params| {
            params.to == acl_address
                && params.input.len() >= 4
                && params.input[0..4] == multicall_selector
        },
        move |params| {
            let count = extract_multicall_count(&params.input);
            Response::call_success(host_acl_multicall_allow_response(count))
        },
        UsageLimit::Unlimited,
    );
}

/// Register a count-aware ACL multicall pattern that denies specific indices.
///
/// `denied_indices` specifies which positions in the multicall response should
/// return false (denied). All other positions return true (allowed).
#[allow(dead_code)]
pub fn register_host_acl_partial_deny(
    host_server: &MockServer,
    acl_address: Address,
    denied_indices: Vec<usize>,
) {
    let multicall_selector = ACL::multicallCall::SELECTOR;

    host_server.on_call_dynamic(
        move |params| {
            params.to == acl_address
                && params.input.len() >= 4
                && params.input[0..4] == multicall_selector
        },
        move |params| {
            let count = extract_multicall_count(&params.input);
            Response::call_success(host_acl_multicall_deny_response(count, &denied_indices))
        },
        UsageLimit::Unlimited,
    );
}

/// Canned on-chain values the `/v2/keyurl` poller reads in tests. Exposed so test
/// assertions can compare the served `dataId` against them. `contextId` / `epochId`
/// are read by the poller (change detection) but not part of the served response.
#[allow(dead_code)]
pub const TEST_KEYURL_KEY_ID: u64 = 3;
#[allow(dead_code)]
pub const TEST_KEYURL_CRS_ID: u64 = 4;
#[allow(dead_code)]
pub const TEST_KEYURL_CONTEXT_ID: u64 = 1;
#[allow(dead_code)]
pub const TEST_KEYURL_EPOCH_ID: u64 = 1;
/// KMS node public-storage config carried by the seeded `NewKmsContext` event: the poller
/// reconstructs the served material URLs from these plus the hex-encoded key/CRS id.
#[allow(dead_code)]
pub const TEST_KEYURL_STORAGE_URL: &str = "http://minio:9000/kms-public";
#[allow(dead_code)]
pub const TEST_KEYURL_STORAGE_PREFIX: &str = "PUB-p1";

/// The full object URL the poller reconstructs for a given `segment` (`PublicKey` / `CRS`) and id:
/// `{storage_url}/{storage_prefix}/{segment}/{id_hex}` (id as 32-byte big-endian, lowercase hex).
#[allow(dead_code)]
pub fn test_keyurl_expected_url(segment: &str, id: U256) -> String {
    let id_hex = hex::encode(id.to_be_bytes::<32>());
    format!("{TEST_KEYURL_STORAGE_URL}/{TEST_KEYURL_STORAGE_PREFIX}/{segment}/{id_hex}")
}

/// The served `dataId` for a given on-chain id: `0x`-prefixed 32-byte big-endian, lowercase hex.
#[allow(dead_code)]
pub fn test_keyurl_expected_data_id(id: U256) -> String {
    format!("0x{}", hex::encode(id.to_be_bytes::<32>()))
}

/// Register a single `eth_call` response keyed by destination address and 4-byte selector.
fn register_call_response(
    host_server: &MockServer,
    to: Address,
    selector: [u8; 4],
    return_data: Vec<u8>,
) {
    let return_bytes = Bytes::from(return_data);
    host_server.on_call(
        move |params| params.to == to && params.input.len() >= 4 && params.input[0..4] == selector,
        Response::call_success(return_bytes.clone()),
        UsageLimit::Unlimited,
    );
}

/// Register canned `KMSGeneration` / `ProtocolConfig` getter responses on the host mock so the
/// `/v2/keyurl` poller's startup fetch succeeds. Without these the relayer would fail its startup
/// gate (the poller blocks startup until the first successful poll).
fn register_default_keyurl_poller_responses(
    host_server: &MockServer,
    kms_generation_address: Address,
    protocol_config_address: Address,
) {
    // getActiveKeyId() -> uint256
    register_call_response(
        host_server,
        kms_generation_address,
        IKMSGeneration::getActiveKeyIdCall::SELECTOR,
        U256::from(TEST_KEYURL_KEY_ID).abi_encode(),
    );
    // getActiveCrsId() -> uint256
    register_call_response(
        host_server,
        kms_generation_address,
        IKMSGeneration::getActiveCrsIdCall::SELECTOR,
        U256::from(TEST_KEYURL_CRS_ID).abi_encode(),
    );
    // getCurrentKmsContextAndEpoch() -> (uint256 contextId, uint256 epochId)
    register_call_response(
        host_server,
        protocol_config_address,
        IProtocolConfig::getCurrentKmsContextAndEpochCall::SELECTOR,
        (
            U256::from(TEST_KEYURL_CONTEXT_ID),
            U256::from(TEST_KEYURL_EPOCH_ID),
        )
            .abi_encode_params(),
    );
    // getKeyMaterials(uint256) -> (string[] urls, KeyDigest[] digests)
    // The digests array is empty, so its element type does not affect the ABI bytes (an empty
    // dynamic array encodes as just a length of 0); `Vec<Bytes>` stands in for `Vec<KeyDigest>`.
    let key_urls = vec![TEST_KEYURL_STORAGE_URL.to_string()];
    let empty_digests: Vec<Bytes> = Vec::new();
    register_call_response(
        host_server,
        kms_generation_address,
        IKMSGeneration::getKeyMaterialsCall::SELECTOR,
        (key_urls, empty_digests).abi_encode_params(),
    );
    // getCrsMaterials(uint256) -> (string[] urls, bytes digest). The returned URLs are only the
    // bucket base; the poller rebuilds the full object URLs from the KMS context nodes, so the
    // exact value here is irrelevant beyond the call succeeding.
    let crs_urls = vec![TEST_KEYURL_STORAGE_URL.to_string()];
    register_call_response(
        host_server,
        kms_generation_address,
        IKMSGeneration::getCrsMaterialsCall::SELECTOR,
        (crs_urls, Bytes::new()).abi_encode_params(),
    );
    // getKmsContextAnchor(uint256) -> (uint256 emissionBlockNumber, bytes32 contextInfoHash).
    // A non-zero block points the poller at the NewKmsContext log seeded below.
    register_call_response(
        host_server,
        protocol_config_address,
        IProtocolConfig::getKmsContextAnchorCall::SELECTOR,
        (U256::from(1u64), B256::ZERO).abi_encode_params(),
    );
    // Seed the NewKmsContext log the poller reads (via eth_getLogs at the anchor block) to recover
    // each KMS node's storage URL + prefix, which it needs to reconstruct the material object URLs.
    let event = IProtocolConfig::NewKmsContext {
        contextId: U256::from(TEST_KEYURL_CONTEXT_ID),
        previousContextId: U256::ZERO,
        kmsNodeParams: vec![IProtocolConfig::KmsNodeParams {
            txSenderAddress: Address::ZERO,
            signerAddress: Address::ZERO,
            ipAddress: String::new(),
            storageUrl: TEST_KEYURL_STORAGE_URL.to_string(),
            partyId: 1,
            mpcIdentity: String::new(),
            caCert: Bytes::new(),
            storagePrefix: TEST_KEYURL_STORAGE_PREFIX.to_string(),
        }],
        thresholds: IProtocolConfig::KmsThresholds {
            publicDecryption: U256::from(1u64),
            userDecryption: U256::from(1u64),
            kmsGen: U256::from(1u64),
            mpc: U256::from(1u64),
        },
        softwareVersion: String::new(),
        pcrValues: Vec::new(),
    };
    host_server.blockchain_state().add_log(Log {
        address: protocol_config_address,
        data: event.encode_log_data(),
    });
}

/// Wire the `ProtocolConfig` KMS-context getters to revert with a bare `0x`, reproducing a
/// protocol v0.13 deployment that does not implement them. Registered rather than left
/// unregistered so a relayer that wrongly polled fails as it does against the real deployment.
fn register_missing_kms_context_getters(
    host_server: &MockServer,
    protocol_config_address: Address,
) {
    for selector in [
        IProtocolConfig::getCurrentKmsContextAndEpochCall::SELECTOR,
        IProtocolConfig::getKmsContextAnchorCall::SELECTOR,
    ] {
        host_server.on_call(
            move |params| {
                params.to == protocol_config_address
                    && params.input.len() >= 4
                    && params.input[0..4] == selector
            },
            Response::revert("0x".to_string()),
            UsageLimit::Unlimited,
        );
    }
}

/// Register an ACL multicall pattern that returns an RPC error.
#[allow(dead_code)]
pub fn register_host_acl_rpc_error(host_server: &MockServer, acl_address: Address) {
    let multicall_selector = ACL::multicallCall::SELECTOR;

    host_server.on_call(
        move |params| {
            params.to == acl_address
                && params.input.len() >= 4
                && params.input[0..4] == multicall_selector
        },
        Response::error("RPC error: host chain node unavailable".to_string()),
        UsageLimit::Unlimited,
    );
}

/// Poll/heartbeat/sweep intervals fast enough that election and handover both resolve inside
/// the poll budgets the dispatcher tests allow themselves.
#[allow(dead_code)]
pub fn fast_timing(settings: &mut Settings) {
    settings.dispatcher_lock.poll_interval = std::time::Duration::from_millis(100);
    settings.dispatcher_lock.heartbeat_interval = std::time::Duration::from_millis(100);
    settings.sweep.interval = std::time::Duration::from_millis(100);
}

/// `(req_status, owner_epoch, attempts)` for one public-decrypt row - the three columns that
/// together say who drove a request and how it got there.
#[allow(dead_code)]
pub async fn row_state(pool: &sqlx::PgPool, ext_job_id: &str) -> (String, Option<i64>, i32) {
    use sqlx::Row;

    let row = sqlx::query(
        r#"
        SELECT req_status::text AS status, owner_epoch, attempts
        FROM public_decrypt_req
        WHERE ext_job_id = $1::uuid
        "#,
    )
    .bind(ext_job_id)
    .fetch_one(pool)
    .await
    .expect("Failed to read the request row");
    (
        row.get("status"),
        row.get("owner_epoch"),
        row.get("attempts"),
    )
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_yaml_parsing_approach() {
        // Test that our YAML parsing and modification approach works correctly
        let sample_config = r#"
gateway:
  readiness_checker:
    max_concurrency: 100
    gw_ciphertext_check:
      retry:
        max_attempts: 75
        retry_interval_ms: 3000
"#;

        // Parse YAML as a generic value
        let mut config: serde_yaml::Value =
            serde_yaml::from_str(sample_config).expect("Failed to parse YAML");

        // Modify the readiness checker gw_ciphertext_check retry settings
        if let Some(gateway) = config.get_mut("gateway") {
            if let Some(readiness_checker) = gateway.get_mut("readiness_checker") {
                if let Some(gw_ciphertext_check) = readiness_checker.get_mut("gw_ciphertext_check")
                {
                    if let Some(retry) = gw_ciphertext_check.get_mut("retry") {
                        retry["max_attempts"] =
                            serde_yaml::Value::Number(serde_yaml::Number::from(4));
                        retry["retry_interval_ms"] =
                            serde_yaml::Value::Number(serde_yaml::Number::from(250));
                    }
                }
            }
        }

        // Verify the changes
        let gateway = config.get("gateway").unwrap();
        let readiness_checker = gateway.get("readiness_checker").unwrap();
        let gw_ciphertext_check = readiness_checker.get("gw_ciphertext_check").unwrap();
        let retry = gw_ciphertext_check.get("retry").unwrap();
        let max_attempts = retry.get("max_attempts").unwrap().as_u64().unwrap();
        let retry_interval = retry.get("retry_interval_ms").unwrap().as_u64().unwrap();

        assert_eq!(max_attempts, 4);
        assert_eq!(retry_interval, 250);

        // Verify serialization works
        let serialized = serde_yaml::to_string(&config).expect("Failed to serialize YAML");
        assert!(serialized.contains("max_attempts: 4"));
        assert!(serialized.contains("retry_interval_ms: 250"));
    }
}
