use std::net::TcpListener;
use std::time::Duration;

use ethereum_rpc_mock::{fhevm::FhevmMockWrapper, MockConfig, MockServer, MockServerHandle};
use fhevm_relayer::config::settings::Settings;
use fhevm_relayer::run_fhevm_relayer;

use tempfile::TempDir;
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
    _temp_db: TempDir,
    _cancellation_token: CancellationToken,
}

impl TestSetup {
    /// Create isolated test setup with free ports and temp database
    #[allow(dead_code)]
    pub async fn new() -> eyre::Result<Self> {
        // Initialize tracing once
        init_tracing_once();

        // Get free ports
        let host_port = get_free_port()?;
        let gateway_port = get_free_port()?;
        let http_port = get_free_port()?;

        tracing::info!(
            "Setting up isolated test on ports {} (host), {} (gateway), {} (http)",
            host_port,
            gateway_port,
            http_port
        );

        // Create temporary database directory
        let temp_db = TempDir::new()?;
        let temp_db_path = temp_db.path().to_string_lossy().to_string();

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

        // Create isolated settings
        let mut settings = Settings::new(None).expect("Failed to load default configuration");

        // Configure with isolated ports and database
        settings.db_path_rocksdb = temp_db_path;
        settings.http_endpoint = Some(format!("0.0.0.0:{}", http_port));
        settings.networks.fhevm.http_url = format!("http://localhost:{}", host_port);
        settings.networks.fhevm.ws_url = format!("ws://localhost:{}", host_port);
        settings.networks.gateway.http_url = format!("http://localhost:{}", gateway_port);
        settings.networks.gateway.ws_url = format!("ws://localhost:{}", gateway_port);

        // Enable mock mode
        settings.transaction.retry.mock_mode = true;

        // Start relayer service with isolated settings
        let cancellation_token = CancellationToken::new();
        let relayer_token = cancellation_token.clone();

        // Create a new settings instance for the relayer since Settings doesn't implement Clone
        let mut relayer_settings =
            Settings::new(None).expect("Failed to load default configuration");
        relayer_settings.db_path_rocksdb = settings.db_path_rocksdb.clone();
        relayer_settings.http_endpoint = settings.http_endpoint.clone();
        relayer_settings.networks.fhevm.http_url = settings.networks.fhevm.http_url.clone();
        relayer_settings.networks.fhevm.ws_url = settings.networks.fhevm.ws_url.clone();
        relayer_settings.networks.gateway.http_url = settings.networks.gateway.http_url.clone();
        relayer_settings.networks.gateway.ws_url = settings.networks.gateway.ws_url.clone();
        relayer_settings.transaction.retry.mock_mode = settings.transaction.retry.mock_mode;

        tokio::spawn(async move {
            tracing::debug!("Starting isolated relayer service...");
            match run_fhevm_relayer(relayer_settings, relayer_token).await {
                Ok(()) => tracing::debug!("Relayer service exited normally"),
                Err(e) => tracing::error!("Relayer service error: {}", e),
            }
        });

        // Give time for servers to be fully ready
        tokio::time::sleep(Duration::from_millis(500)).await;

        tracing::info!(
            "Isolated test setup complete on ports {} (host), {} (gateway), {} (http)",
            host_port,
            gateway_port,
            http_port
        );

        Ok(TestSetup {
            fhevm_mock: fhevm_wrapper,
            settings,
            http_port,
            _host_handle: host_handle,
            _gateway_handle: gateway_handle,
            _temp_db: temp_db,
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

/// Get a free port by binding to port 0
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

/// Initialize tracing once for all tests
#[allow(dead_code)]
fn init_tracing_once() {
    use std::sync::Once;
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        let subscriber = tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::try_from_default_env()
                    .unwrap_or_else(|_| "info,eth_json_rpc_mock=info,fhevm_relayer=info".into()),
            )
            .with_target(true)
            .with_thread_ids(true)
            .with_file(true)
            .with_line_number(true);

        let _ = subscriber.try_init();
    });
}
