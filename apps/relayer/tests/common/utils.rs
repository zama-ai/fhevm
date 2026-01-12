use std::net::TcpListener;

use ethereum_rpc_mock::{fhevm::FhevmMockWrapper, MockConfig, MockServer, MockServerHandle};
use fhevm_relayer::config::settings::{Settings, StorageConfig};
use fhevm_relayer::run_fhevm_relayer;
use fhevm_relayer::store::sql::client::PgClient;
use fhevm_relayer::tracing::init_tracing_once;

use alloy::primitives::Address;
use rand::{rng, Rng};
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::task::JoinHandle;
use tokio_util::sync::CancellationToken;

/// Per-test isolated setup with own ports, database, and mock servers
#[allow(dead_code)]
pub struct TestSetup {
    pub fhevm_mock: FhevmMockWrapper,
    pub settings: Settings,
    pub http_port: u16,
    host_handle: MockServerHandle,
    gateway_handle: MockServerHandle,
    cancellation_token: CancellationToken,
    relayer_handle: JoinHandle<()>,
}

impl TestSetup {
    /// Create test setup with fast readiness config (4 attempts × 250ms = ~1s total)
    /// This config is used in tests for readiness check timing out.
    #[allow(dead_code)]
    pub async fn new_with_fast_readiness() -> eyre::Result<Self> {
        // Create temp config with modified readiness settings
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_fast_readiness_config(&temp_config_dir)?;

        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with low retry config (2 attempts × 100ms)
    /// This config is used in tests for max retries exceeded scenarios.
    #[allow(dead_code)]
    pub async fn new_with_low_retries() -> eyre::Result<Self> {
        // Create temp config with low retry settings
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_low_retry_config(&temp_config_dir)?;

        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create isolated test setup with free ports and temp database
    #[allow(dead_code)]
    pub async fn new() -> eyre::Result<Self> {
        // Create a temp config based on the example config
        let temp_config_dir = tempfile::TempDir::new()?;
        let temp_config_path = create_default_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with specified number of listener instances (for redundancy testing)
    #[allow(dead_code)]
    pub async fn new_with_listeners(listener_count: usize) -> eyre::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_listener_config(&temp_config_dir, listener_count)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create test setup with admin endpoint enabled
    #[allow(dead_code)]
    pub async fn new_with_admin_endpoint() -> eyre::Result<Self> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = create_admin_endpoint_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create setup with optional custom config path
    #[allow(dead_code)]
    pub async fn new_with_config_path(
        config_path: Option<std::path::PathBuf>,
    ) -> eyre::Result<Self> {
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
        std::fs::copy("config/local.yaml.example", &temp_config_path)
            .map_err(|e| eyre::eyre!("Failed to copy config file: {}", e))?;

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
        let host_handle = host_server
            .start()
            .await
            .map_err(|e| eyre::eyre!("Failed to start host mock server: {}", e))?;

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
            .map_err(|e| eyre::eyre!("Failed to start gateway mock server: {}", e))?;

        // Create settings from config file (default or custom)
        let config_path_str = config_path.map(|p| p.to_string_lossy().to_string());
        let mut settings =
            Settings::new(config_path_str.clone()).expect("Failed to load configuration");

        // Initialize tracing once with settings
        init_tracing_once(&settings.log);

        // Keep test pools small to avoid exhausting CI Postgres.
        settings.storage.app_pool.max_connections = 2;
        settings.storage.cron_pool.max_connections = 3;
        settings.storage.app_pool.min_connections = 0;
        settings.storage.cron_pool.min_connections = 0;

        // Configure with dynamic ports (use :0 for automatic allocation for relayer HTTP/metrics)
        settings.http.endpoint = Some("0.0.0.0:0".to_string());
        settings.gateway.blockchain_rpc.http_url = format!("http://localhost:{}", gateway_port);
        settings.gateway.blockchain_rpc.ws_url = format!("ws://localhost:{}", gateway_port);
        settings.metrics.endpoint = "0.0.0.0:0".to_string();

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
        relayer_settings.http.endpoint = settings.http.endpoint.clone();
        relayer_settings.gateway.blockchain_rpc.http_url =
            settings.gateway.blockchain_rpc.http_url.clone();
        relayer_settings.gateway.blockchain_rpc.ws_url =
            settings.gateway.blockchain_rpc.ws_url.clone();
        relayer_settings.metrics.endpoint = settings.metrics.endpoint.clone();

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
            .map_err(|_| eyre::eyre!("Failed to receive settings from relayer"))?;

        tracing::debug!("Relayer service started successfully with actual ports");

        // Extract actual HTTP port from the updated settings
        let http_port = updated_settings
            .http
            .endpoint
            .as_ref()
            .and_then(|endpoint| endpoint.rsplit(':').next())
            .and_then(|port| port.parse::<u16>().ok())
            .ok_or_else(|| eyre::eyre!("Failed to parse HTTP port from settings"))?;

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
            settings,
            http_port,
            host_handle,
            gateway_handle,
            cancellation_token,
            relayer_handle,
        })
    }

    #[allow(dead_code)]
    pub async fn shutdown(self) {
        self.cancellation_token.cancel();

        // Only wait for relayer - it has the DB connections
        if let Err(e) = self.relayer_handle.await {
            tracing::error!("Test relayer task failed: {}", e);
        }

        // Mock servers will shutdown when handles are dropped
        drop(self.host_handle);
        drop(self.gateway_handle);
    }
}

/// Create a default config file based on the example
fn create_default_config(temp_dir: &tempfile::TempDir) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("test_config.yaml");

    // Simply copy the example config without modifications
    std::fs::copy("config/local.yaml.example", &temp_config_path)
        .map_err(|e| eyre::eyre!("Failed to copy example config: {}", e))?;

    Ok(temp_config_path)
}

/// Create a config file with fast readiness settings (4 attempts × 250ms)
fn create_fast_readiness_config(temp_dir: &TempDir) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("fast_readiness.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("config/local.yaml.example")
        .map_err(|e| eyre::eyre!("Failed to read default config: {}", e))?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse YAML config: {}", e))?;

    // Modify the readiness checker retry settings
    if let Some(gateway) = config.get_mut("gateway") {
        if let Some(readiness_checker) = gateway.get_mut("readiness_checker") {
            if let Some(retry) = readiness_checker.get_mut("retry") {
                retry["max_attempts"] = serde_yaml::Value::Number(serde_yaml::Number::from(4));
                retry["retry_interval_ms"] =
                    serde_yaml::Value::Number(serde_yaml::Number::from(250));
            }
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content = serde_yaml::to_string(&config)
        .map_err(|e| eyre::eyre!("Failed to serialize modified config: {}", e))?;

    std::fs::write(&temp_config_path, modified_content)
        .map_err(|e| eyre::eyre!("Failed to write temp config: {}", e))?;

    Ok(temp_config_path)
}

/// Create a config file with low retry settings for tx_engine (2 attempts × 100ms)
/// This config is used in tests for max retries exceeded scenarios.
fn create_low_retry_config(temp_dir: &TempDir) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("low_retry.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("config/local.yaml.example")
        .map_err(|e| eyre::eyre!("Failed to read default config: {}", e))?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse YAML config: {}", e))?;

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
    let modified_content = serde_yaml::to_string(&config)
        .map_err(|e| eyre::eyre!("Failed to serialize modified config: {}", e))?;

    std::fs::write(&temp_config_path, modified_content)
        .map_err(|e| eyre::eyre!("Failed to write temp config: {}", e))?;

    Ok(temp_config_path)
}

/// Create a config file with admin endpoint enabled
fn create_admin_endpoint_config(temp_dir: &TempDir) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("admin_endpoint.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("config/local.yaml.example")
        .map_err(|e| eyre::eyre!("Failed to read default config: {}", e))?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse YAML config: {}", e))?;

    // Enable admin endpoint
    if let Some(http) = config.get_mut("http") {
        http["enable_admin_endpoint"] = serde_yaml::Value::Bool(true);
    }

    // Serialize back to YAML and write to temp file
    let modified_content = serde_yaml::to_string(&config)
        .map_err(|e| eyre::eyre!("Failed to serialize modified config: {}", e))?;

    std::fs::write(&temp_config_path, modified_content)
        .map_err(|e| eyre::eyre!("Failed to write temp config: {}", e))?;

    Ok(temp_config_path)
}

/// Create a config file with specified number of listener instances
fn create_listener_config(
    temp_dir: &TempDir,
    listener_count: usize,
) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("listener_config.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("config/local.yaml.example")
        .map_err(|e| eyre::eyre!("Failed to read default config: {}", e))?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse YAML config: {}", e))?;

    // Modify the listener instances count
    if let Some(gateway) = config.get_mut("gateway") {
        if let Some(listener) = gateway.get_mut("listener") {
            listener["listener_instances"] =
                serde_yaml::Value::Number(serde_yaml::Number::from(listener_count));
        }
    }

    // Serialize back to YAML and write to temp file
    let modified_content = serde_yaml::to_string(&config)
        .map_err(|e| eyre::eyre!("Failed to serialize modified config: {}", e))?;

    std::fs::write(&temp_config_path, modified_content)
        .map_err(|e| eyre::eyre!("Failed to write temp config: {}", e))?;

    Ok(temp_config_path)
}

/// Create a config file with fast timeout settings for testing timeout behavior
/// Sets test_mock to false to enable background workers including timeout worker
#[allow(dead_code)]
pub fn create_timeout_test_config(
    temp_dir: &TempDir,
    timeout_secs: u64,
    cron_interval_secs: u64,
) -> eyre::Result<std::path::PathBuf> {
    let temp_config_path = temp_dir.path().join("timeout_test.yaml");

    // Read the default config
    let config_content = std::fs::read_to_string("config/local.yaml.example")
        .map_err(|e| eyre::eyre!("Failed to read default config: {}", e))?;

    // Parse YAML as a generic value
    let mut config: serde_yaml::Value = serde_yaml::from_str(&config_content)
        .map_err(|e| eyre::eyre!("Failed to parse YAML config: {}", e))?;

    // Set test_mock to false to enable background workers
    if let Some(global) = config.get_mut("global") {
        global["test_mock"] = serde_yaml::Value::Bool(false);
    }

    // Modify the timeout settings
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
    let modified_content = serde_yaml::to_string(&config)
        .map_err(|e| eyre::eyre!("Failed to serialize modified config: {}", e))?;

    std::fs::write(&temp_config_path, modified_content)
        .map_err(|e| eyre::eyre!("Failed to write temp config: {}", e))?;

    Ok(temp_config_path)
}

/// Get a free port by binding to port 0
/// This is needed for mock servers that don't support dynamic port allocation yet
#[allow(dead_code)]
fn get_free_port() -> eyre::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")
        .map_err(|e| eyre::eyre!("Failed to bind to free port: {}", e))?;
    let port = listener
        .local_addr()
        .map_err(|e| eyre::eyre!("Failed to get local address: {}", e))?
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

/// Generate a random handle (64 hex characters) for testing
#[allow(dead_code)]
pub fn random_handle() -> String {
    let mut rng = rng();
    let hex: String = (0..64)
        .map(|_| rng.random_range(0..16))
        .map(|digit| format!("{:x}", digit))
        .collect();
    format!("0x{}", hex)
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
    retry:
      max_attempts: 75
      retry_interval_ms: 3000
"#;

        // Parse YAML as a generic value
        let mut config: serde_yaml::Value =
            serde_yaml::from_str(sample_config).expect("Failed to parse YAML");

        // Modify the readiness checker retry settings
        if let Some(gateway) = config.get_mut("gateway") {
            if let Some(readiness_checker) = gateway.get_mut("readiness_checker") {
                if let Some(retry) = readiness_checker.get_mut("retry") {
                    retry["max_attempts"] = serde_yaml::Value::Number(serde_yaml::Number::from(4));
                    retry["retry_interval_ms"] =
                        serde_yaml::Value::Number(serde_yaml::Number::from(250));
                }
            }
        }

        // Verify the changes
        let gateway = config.get("gateway").unwrap();
        let readiness_checker = gateway.get("readiness_checker").unwrap();
        let retry = readiness_checker.get("retry").unwrap();
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
