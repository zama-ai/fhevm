use std::net::TcpListener;

use anyhow::Context;
use ethereum_rpc_mock::{
    fhevm::FhevmMockWrapper, MockConfig, MockServer, MockServerHandle, Response, UsageLimit,
};
use fhevm_relayer::config::settings::{HostChainConfig, Settings, StorageConfig};
use fhevm_relayer::run_fhevm_relayer;
use fhevm_relayer::store::sql::client::PgClient;
use fhevm_relayer::tracing::init_tracing_once;

use alloy::primitives::{Address, Bytes};
use alloy::sol_types::{SolCall, SolValue};
use fhevm_host_bindings::acl::ACL;
use rand::{rng, RngExt};
use std::str::FromStr;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

use super::test_schema::TestSchema;

/// Per-test isolated setup with own ports, database, and mock servers
#[allow(dead_code)]
pub struct TestSetup {
    pub fhevm_mock: FhevmMockWrapper,
    pub host_server: MockServer,
    pub settings: Settings,
    pub http_port: u16,
    host_handle: MockServerHandle,
    gateway_handle: MockServerHandle,
    cancellation_token: CancellationToken,
    relayer_handle: JoinHandle<()>,
    test_schema: TestSchema,
}

impl TestSetup {
    /// Create test setup with fast readiness config (4 attempts × 250ms = ~1s total)
    /// This config is used in tests for readiness check timing out.
    #[allow(dead_code)]
    pub async fn new_with_fast_readiness() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path =
            create_readiness_config(&temp_config_dir, "fast_readiness.yaml", 4, 250)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with minimal readiness retries (2 attempts × 50ms = ~100ms total)
    /// Use when the test doesn't need many retries (e.g., contract errors that fail immediately).
    #[allow(dead_code)]
    pub async fn new_with_minimal_readiness() -> anyhow::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path =
            create_readiness_config(&temp_config_dir, "minimal_readiness.yaml", 2, 50)?;
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
        // Create isolated test schema first
        let test_schema = TestSchema::new().await?;
        tracing::info!(
            "Created isolated test schema: {}",
            test_schema.schema_name()
        );

        // Get free ports for mock servers (they don't support :0 yet)
        let host_port = get_free_port()?;
        let gateway_port = get_free_port()?;

        tracing::info!(
            "Setting up isolated test - mock servers on ports {} (host), {} (gateway)",
            host_port,
            gateway_port
        );

        // Create temporary config file from example
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = temp_config_dir.path().join("test_config.yaml");
        std::fs::copy("tests/relayer-test-config.yaml", &temp_config_path)
            .context("Failed to copy config file")?;

        // Configuration constants
        let decryption_addr: alloy::primitives::Address =
            "0xB8Ae44365c45A7C5256b14F607CaE23BC040c354"
                .parse()
                .expect("Invalid decryption address");
        let input_verification_addr: alloy::primitives::Address =
            "0xe61cff9c581c7c91aef682c2c10e8632864339ab"
                .parse()
                .expect("Invalid input verification address");

        // Create and start Host chain mock server
        tracing::debug!("Creating Host chain MockServer on port {}", host_port);
        let host_config = MockConfig {
            port: host_port,
            ..MockConfig::new()
        };
        let host_server = MockServer::new(host_config);
        let host_server_clone = host_server.clone();
        let host_handle = host_server
            .start()
            .await
            .context("Failed to start host mock server")?;

        // Create Gateway chain mock server
        tracing::debug!("Creating Gateway chain MockServer on port {}", gateway_port);
        let gateway_config = MockConfig {
            port: gateway_port,
            ..MockConfig::new()
        };
        let gateway_server = MockServer::new(gateway_config);

        // Configure FHEVM patterns BEFORE starting the server
        let fhevm_wrapper = FhevmMockWrapper::new(
            gateway_server.clone(),
            decryption_addr,
            input_verification_addr,
        );

        // Start Gateway chain mock server
        let gateway_handle = gateway_server
            .start()
            .await
            .context("Failed to start gateway mock server")?;

        // Create settings from config file (default or custom)
        let config_path_str = config_path.map(|p| p.to_string_lossy().to_string());
        let mut settings =
            Settings::new(config_path_str.clone()).expect("Failed to load configuration");

        // Initialize tracing once with settings
        init_tracing_once(&settings.log);

        // Keep test pools small to avoid exhausting CI Postgres.
        settings.storage.app_pool.max_connections = 2;
        settings.storage.app_pool.min_connections = 0;

        // Cron pool kept small — expiry worker is disabled by default
        // Minimum allowed value is 1 connection
        settings.storage.cron_pool.max_connections = 1;
        settings.storage.cron_pool.min_connections = 0;

        // Configure to use isolated test schema
        settings.storage.sql_database_url = test_schema.database_url();

        // Configure with dynamic ports (use :0 for automatic allocation for relayer HTTP/metrics)
        settings.http.endpoint = Some("0.0.0.0:0".to_string());
        settings.gateway.blockchain_rpc.http_url = format!("http://localhost:{}", gateway_port);
        settings.gateway.blockchain_rpc.read_http_url =
            format!("http://localhost:{}", gateway_port);
        settings.metrics.endpoint = "0.0.0.0:0".to_string();

        // Update listener pool URLs to use the mock server
        let ws_url = format!("ws://localhost:{}", gateway_port);
        for listener in &mut settings.gateway.listener_pool.listeners {
            listener.url = ws_url.clone();
        }

        // Wire host chain URLs to the host mock server
        for hc in &mut settings.host_chains {
            hc.url = format!("http://localhost:{}", host_port);
        }

        // Register default ACL allow-all pattern on host mock
        register_default_host_acl_allow_all(&host_server_clone, &settings.host_chains);

        // Start relayer service with isolated settings
        let cancellation_token = CancellationToken::new();
        let relayer_token = cancellation_token.clone();

        // Create a new settings instance for the relayer since Settings doesn't implement Clone
        let mut relayer_settings =
            Settings::new(config_path_str.clone()).expect("Failed to load configuration");
        relayer_settings.storage.app_pool.max_connections =
            settings.storage.app_pool.max_connections;
        relayer_settings.storage.cron_pool.max_connections =
            settings.storage.cron_pool.max_connections;
        relayer_settings.storage.app_pool.min_connections =
            settings.storage.app_pool.min_connections;
        relayer_settings.storage.cron_pool.min_connections =
            settings.storage.cron_pool.min_connections;
        relayer_settings.storage.sql_database_url = settings.storage.sql_database_url.clone();
        relayer_settings.http.endpoint = settings.http.endpoint.clone();
        relayer_settings.gateway.blockchain_rpc.http_url =
            settings.gateway.blockchain_rpc.http_url.clone();
        relayer_settings.gateway.blockchain_rpc.read_http_url =
            settings.gateway.blockchain_rpc.read_http_url.clone();
        relayer_settings.metrics.endpoint = settings.metrics.endpoint.clone();

        // Wire relayer host chain URLs to the host mock server
        for hc in &mut relayer_settings.host_chains {
            hc.url = format!("http://localhost:{}", host_port);
        }

        // Update relayer listener pool URLs to use the mock server
        for listener in &mut relayer_settings.gateway.listener_pool.listeners {
            listener.url = ws_url.clone();
        }

        // Create a channel to receive settings with actual ports
        let (settings_tx, settings_rx) = oneshot::channel::<Settings>();

        // Spawn relayer in background task - it will run until cancellation
        let relayer_handle = tokio::spawn(async move {
            match run_fhevm_relayer(relayer_settings, relayer_token, Some(settings_tx)).await {
                Ok(()) => tracing::debug!("Relayer service exited normally"),
                Err(e) => tracing::error!("Relayer service error: {}", e),
            }
        });

        // Wait to receive settings with actual ports (this confirms servers are ready)
        let updated_settings = settings_rx
            .await
            .context("Failed to receive settings from relayer")?;

        tracing::debug!("Relayer service started successfully with actual ports");

        // Extract actual HTTP port from the updated settings
        let http_port = updated_settings
            .http
            .endpoint
            .as_ref()
            .and_then(|endpoint| endpoint.rsplit(':').next())
            .and_then(|port| port.parse::<u16>().ok())
            .context("Failed to parse HTTP port from settings")?;

        tracing::info!(
            "Isolated test setup complete with actual ports - gateway: {}, http: {}, metrics: {}",
            gateway_port,
            updated_settings
                .http
                .endpoint
                .as_ref()
                .unwrap_or(&"none".to_string()),
            updated_settings.metrics.endpoint
        );

        // Update the settings with actual values
        settings = updated_settings;

        Ok(TestSetup {
            fhevm_mock: fhevm_wrapper,
            host_server: host_server_clone,
            settings,
            http_port,
            host_handle,
            gateway_handle,
            cancellation_token,
            relayer_handle,
            test_schema,
        })
    }

    #[allow(dead_code)]
    pub async fn shutdown(mut self) {
        self.cancellation_token.cancel();

        // Only wait for relayer - it has the DB connections
        if let Err(e) = self.relayer_handle.await {
            tracing::error!("Test relayer task failed: {}", e);
        }

        // Mock servers will shutdown when handles are dropped
        drop(self.host_handle);
        drop(self.gateway_handle);

        // Clean up test schema
        if let Err(e) = self.test_schema.cleanup().await {
            tracing::error!("Failed to cleanup test schema: {}", e);
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
fn create_readiness_config(
    temp_dir: &TempDir,
    filename: &str,
    max_attempts: u32,
    retry_interval_ms: u32,
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

/// Create a config file with low retry settings for tx_engine (2 attempts × 100ms)
/// This config is used in tests for max retries exceeded scenarios.
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

/// Get a free port by binding to port 0
/// This is needed for mock servers that don't support dynamic port allocation yet
#[allow(dead_code)]
fn get_free_port() -> anyhow::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0").context("Failed to bind to free port")?;
    let port = listener
        .local_addr()
        .context("Failed to get local address")?
        .port();
    Ok(port)
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
