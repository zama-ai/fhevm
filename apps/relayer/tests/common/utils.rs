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
use tokio_util::sync::CancellationToken;

/// Per-test isolated setup with own ports, database, and mock servers
pub struct TestSetup {
    #[allow(dead_code)]
    pub fhevm_mock: FhevmMockWrapper,
    #[allow(dead_code)]
    pub settings: Settings,
    #[allow(dead_code)]
    pub http_port: u16,
    _host_handle: MockServerHandle,
    _gateway_handle: MockServerHandle,
    _cancellation_token: CancellationToken,
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

    /// Create isolated test setup with free ports and temp database
    #[allow(dead_code)]
    pub async fn new() -> eyre::Result<Self> {
        // Create a temp config based on the example config
        let temp_config_dir = tempfile::TempDir::new()?;
        let temp_config_path = create_default_config(&temp_config_dir)?;
        Self::new_with_config_path(Some(temp_config_path)).await
    }

    /// Create setup with optional custom config path
    async fn new_with_config_path(config_path: Option<std::path::PathBuf>) -> eyre::Result<Self> {
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
        relayer_settings.http.endpoint = settings.http.endpoint.clone();
        relayer_settings.gateway.blockchain_rpc.http_url =
            settings.gateway.blockchain_rpc.http_url.clone();
        relayer_settings.gateway.blockchain_rpc.ws_url =
            settings.gateway.blockchain_rpc.ws_url.clone();
        relayer_settings.metrics.endpoint = settings.metrics.endpoint.clone();

        // Create a channel to receive settings with actual ports
        let (settings_tx, settings_rx) = oneshot::channel::<Settings>();

        // Spawn relayer in background task - it will run until cancellation
        tokio::spawn(async move {
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
            _host_handle: host_handle,
            _gateway_handle: gateway_handle,
            _cancellation_token: cancellation_token,
        })
    }
}

impl Drop for TestSetup {
    fn drop(&mut self) {
        tracing::debug!("Cleaning up isolated test setup");
        self._cancellation_token.cancel();
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
pub async fn setup_test_database(config: StorageConfig) -> eyre::Result<PgClient> {
    let pg_client = PgClient::new(config).await;
    Ok(pg_client)
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
