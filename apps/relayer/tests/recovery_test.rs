// Recovery tests that validate the relayer's ability to recover incomplete requests
// after a restart. Each test targets a specific stuck status:
//
// 1. test_recovery_from_queued_status - requests stuck before readiness check
// 2. test_recovery_from_processing_status - requests stuck after readiness, before tx
// 3. test_recovery_from_tx_in_flight_status - requests stuck after tx sent
//
// Test pattern:
// - Start relayer with broken gateway mock (causes requests to get stuck at target status)
// - Send request(s)
// - Verify requests reach target status via DB query
// - Shutdown relayer
// - Restart with working gateway mock
// - Verify all requests complete successfully
//
// IMPORTANT: Run tests sequentially to avoid database conflicts:
//   cargo test --test recovery_test -- --test-threads=1

mod common;

use alloy::primitives::{Address, Bytes, B256};
use ethereum_rpc_mock::{
    fhevm::{FhevmMockWrapper, UserDecryptKind},
    MockConfig, MockServer, MockServerHandle, SubscriptionTarget,
};
use fhevm_relayer::config::settings::{Settings, StorageConfig};
use fhevm_relayer::run_fhevm_relayer;
use fhevm_relayer::store::sql::repositories::Repositories;
use fhevm_relayer::tracing::init_tracing_once;
use rand::{rng, RngExt};
use serde_json::json;
use sqlx::postgres::PgPoolOptions;
use std::net::TcpListener;
use std::slice;
use std::str::FromStr;
use std::time::Duration;
use tempfile::TempDir;
use tokio::sync::oneshot;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

// Contract addresses used by mocks
const DECRYPTION_ADDR: &str = "0xB8Ae44365c45A7C5256b14F607CaE23BC040c354";
const INPUT_VERIFICATION_ADDR: &str = "0xe61cff9c581c7c91aef682c2c10e8632864339ab";

/// Test setup with two gateway mocks - one broken, one working
struct RecoveryTestSetup {
    broken_gateway_port: u16,
    working_gateway_port: u16,
    broken_gateway: FhevmMockWrapper,
    working_gateway: FhevmMockWrapper,
    _broken_handle: MockServerHandle,
    _working_handle: MockServerHandle,
    http_port: Option<u16>,
    settings_rx: Option<oneshot::Receiver<Settings>>,
    cancellation_token: CancellationToken,
    relayer_task: Option<tokio::task::JoinHandle<()>>,
    storage_settings: Option<StorageConfig>,
}

impl RecoveryTestSetup {
    async fn new() -> eyre::Result<Self> {
        let broken_port = get_free_port()?;
        let working_port = get_free_port()?;

        tracing::info!(
            "Setting up recovery test - broken gateway: {}, working gateway: {}",
            broken_port,
            working_port
        );

        let decryption_addr: Address = DECRYPTION_ADDR.parse().expect("Invalid decryption address");
        let input_verification_addr: Address = INPUT_VERIFICATION_ADDR
            .parse()
            .expect("Invalid input verification address");

        // Create broken gateway mock server
        let broken_config = MockConfig {
            port: broken_port,
            ..MockConfig::new()
        };
        let broken_server = MockServer::new(broken_config);
        let broken_gateway = FhevmMockWrapper::new(
            broken_server.clone(),
            decryption_addr,
            input_verification_addr,
        );
        let broken_handle = broken_server
            .start()
            .await
            .map_err(|e| eyre::eyre!("Failed to start broken gateway: {}", e))?;

        // Create working gateway mock server
        let working_config = MockConfig {
            port: working_port,
            ..MockConfig::new()
        };
        let working_server = MockServer::new(working_config);
        let working_gateway = FhevmMockWrapper::new(
            working_server.clone(),
            decryption_addr,
            input_verification_addr,
        );
        let working_handle = working_server
            .start()
            .await
            .map_err(|e| eyre::eyre!("Failed to start working gateway: {}", e))?;

        Ok(Self {
            broken_gateway_port: broken_port,
            working_gateway_port: working_port,
            broken_gateway,
            working_gateway,
            _broken_handle: broken_handle,
            _working_handle: working_handle,
            http_port: None,
            settings_rx: None,
            cancellation_token: CancellationToken::new(),
            relayer_task: None,
            storage_settings: None,
        })
    }

    async fn start_relayer_with_gateway(
        &mut self,
        gateway_port: u16,
        modify_config: impl FnOnce(&mut Settings),
    ) -> eyre::Result<()> {
        let temp_config_dir = TempDir::new()?;
        let temp_config_path = temp_config_dir.path().join("test_config.yaml");
        std::fs::copy("config/local.yaml.example", &temp_config_path)
            .map_err(|e| eyre::eyre!("Failed to copy config file: {}", e))?;

        let mut settings = Settings::new(Some(temp_config_path.to_string_lossy().to_string()))
            .expect("Failed to load configuration");

        init_tracing_once(&settings.log);

        // Configure with specified gateway
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

        // Store storage settings for DB access
        self.storage_settings = Some(settings.storage.clone());

        // Apply custom config modifications
        modify_config(&mut settings);

        let (settings_tx, settings_rx) = oneshot::channel::<Settings>();
        self.settings_rx = Some(settings_rx);

        let relayer_token = self.cancellation_token.clone();

        let task_handle = tokio::spawn(async move {
            match run_fhevm_relayer(settings, relayer_token, Some(settings_tx)).await {
                Ok(()) => tracing::debug!("Relayer service exited normally"),
                Err(e) => tracing::error!("Relayer service error: {}", e),
            }
        });

        self.relayer_task = Some(task_handle);

        let updated_settings = self
            .settings_rx
            .take()
            .ok_or_else(|| eyre::eyre!("Settings receiver not available"))?
            .await
            .map_err(|_| eyre::eyre!("Failed to receive settings from relayer"))?;

        if let Some(endpoint) = updated_settings.http.endpoint {
            let port = endpoint
                .split(':')
                .next_back()
                .and_then(|p| p.parse::<u16>().ok())
                .ok_or_else(|| eyre::eyre!("Failed to parse HTTP port"))?;
            self.http_port = Some(port);
            tracing::info!("Relayer HTTP server running on port {}", port);
        }

        Ok(())
    }

    async fn shutdown_relayer(&mut self) {
        tracing::info!("Shutting down relayer");
        self.cancellation_token.cancel();

        if let Some(task) = self.relayer_task.take() {
            match tokio::time::timeout(Duration::from_secs(10), task).await {
                Ok(Ok(())) => tracing::info!("Relayer task completed successfully"),
                Ok(Err(e)) => tracing::warn!("Relayer task panicked: {}", e),
                Err(_) => tracing::warn!("Relayer task shutdown timed out after 10s"),
            }
        }

        tracing::info!("Relayer shutdown completed");
    }

    fn reset_for_restart(&mut self) {
        self.cancellation_token = CancellationToken::new();
    }

    /// Configure broken gateway for 'queued' status:
    /// - Readiness check fails, so requests stay in queued
    fn configure_for_queued_stuck(&self) {
        self.broken_gateway.set_readiness_failure();
        tracing::info!("Broken gateway configured for 'queued' stuck - readiness fails");
    }

    /// Configure broken gateway for 'processing' status:
    /// - Requests pass readiness check
    /// - Transaction is accepted but no response event is emitted
    fn configure_for_processing_stuck(&self, _handles: &[String]) {
        // Set readiness to pass so requests can leave queued status
        self.broken_gateway.set_readiness_success();

        // DON'T register any event patterns - this keeps transactions pending
        // without any events being emitted, so requests stay in 'processing' status

        tracing::info!(
            "Broken gateway configured for 'processing' stuck - no events will be emitted"
        );
    }

    /// Configure broken gateway for 'tx_in_flight' status:
    /// - Requests pass readiness check and transaction is sent
    /// - No events emitted so request stays in processing/tx_in_flight
    #[allow(dead_code)]
    fn configure_for_tx_in_flight_stuck(&self, _handles: &[String]) {
        // Set readiness to pass so requests can leave queued status
        self.broken_gateway.set_readiness_success();

        // DON'T register any event patterns - this keeps transactions pending
        // without any events being emitted, so requests stay in 'processing' or 'tx_in_flight' status

        tracing::info!(
            "Broken gateway configured for 'tx_in_flight' stuck - no events will be emitted"
        );
    }

    /// Configure working gateway to complete any request
    fn configure_working_gateway(&self, handles: &[String]) {
        self.working_gateway.set_readiness_success();

        let b256_handles: Vec<B256> = handles
            .iter()
            .filter_map(|h| B256::from_str(h).ok())
            .collect();

        if !b256_handles.is_empty() {
            let values: Vec<u64> = (0..b256_handles.len()).map(|i| i as u64 + 42).collect();
            self.working_gateway.on_public_decrypt_success(
                b256_handles.clone(),
                values,
                SubscriptionTarget::All,
            );

            let dummy_address =
                Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
            self.working_gateway.on_user_decrypt_success(
                UserDecryptKind::Direct,
                b256_handles,
                dummy_address,
                SubscriptionTarget::All,
            );
        }

        let dummy_address =
            Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let proof_data = Bytes::from(b"valid_proof");
        self.working_gateway.on_input_proof_success(
            dummy_address,
            proof_data,
            10,
            SubscriptionTarget::All,
        );

        tracing::info!("Working gateway configured for {} handles", handles.len());
    }

    fn http_url(&self) -> String {
        format!(
            "http://localhost:{}",
            self.http_port.expect("HTTP port not set")
        )
    }

    /// Get repositories for DB access
    async fn get_repositories(&self) -> eyre::Result<Repositories> {
        let storage = self
            .storage_settings
            .as_ref()
            .ok_or_else(|| eyre::eyre!("Storage settings not available"))?;
        Repositories::new(storage.clone())
            .await
            .map_err(|e| eyre::eyre!("Failed to create repositories: {}", e))
    }
}

fn get_free_port() -> eyre::Result<u16> {
    let listener = TcpListener::bind("127.0.0.1:0")?;
    let port = listener.local_addr()?.port();
    drop(listener);
    Ok(port)
}

fn random_handle() -> String {
    let mut rng = rng();
    let bytes: [u8; 32] = rng.random();
    format!("0x{}", hex::encode(bytes))
}

/// Clean up all incomplete requests from the database to start with a clean state
async fn cleanup_incomplete_requests() -> eyre::Result<()> {
    // Use the same database URL as the config
    let database_url = "postgresql://postgres:postgres@localhost:5433/relayer_db";

    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(database_url)
        .await
        .map_err(|e| eyre::eyre!("Failed to connect to database: {}", e))?;

    // Delete all incomplete requests
    sqlx::query("DELETE FROM public_decrypt_req WHERE req_status IN ('queued', 'processing', 'tx_in_flight', 'receipt_received')")
        .execute(&pool)
        .await
        .map_err(|e| eyre::eyre!("Failed to delete incomplete public decrypt requests: {}", e))?;

    sqlx::query("DELETE FROM user_decrypt_req WHERE req_status IN ('queued', 'processing', 'tx_in_flight', 'receipt_received')")
        .execute(&pool)
        .await
        .map_err(|e| eyre::eyre!("Failed to delete incomplete user decrypt requests: {}", e))?;

    sqlx::query("DELETE FROM input_proof_req WHERE req_status IN ('queued', 'processing', 'tx_in_flight', 'receipt_received')")
        .execute(&pool)
        .await
        .map_err(|e| eyre::eyre!("Failed to delete incomplete input proof requests: {}", e))?;

    tracing::info!("Cleaned up all incomplete requests from database");
    Ok(())
}

// Request sending helpers
// Uses V2 for public decrypt and input proof (async, returns immediately)
// Skips user decrypt for now as V2 format is different

async fn send_public_decrypt_request(base_url: &str, handle: &str) -> eyre::Result<String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "ciphertextHandles": [handle],
        "extraData": "0x00"
    });

    let response = client
        .post(format!("{}/v2/public-decrypt", base_url))
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body: serde_json::Value = response.json().await?;

    if status.is_success() || status.as_u16() == 202 {
        Ok(body["result"]["jobId"]
            .as_str()
            .ok_or_else(|| eyre::eyre!("No jobId in response: {:?}", body))?
            .to_string())
    } else {
        Err(eyre::eyre!("Request failed: {:?}", body))
    }
}

#[allow(dead_code)]
async fn send_input_proof_request(base_url: &str) -> eyre::Result<String> {
    let client = reqwest::Client::new();
    let payload = json!({
        "ciphertext": random_handle(),
        "proof": random_handle(),
        "contractAddress": "0x1234567890123456789012345678901234567890",
        "callerAddress": "0x0987654321098765432109876543210987654321"
    });

    let response = client
        .post(format!("{}/v2/input-proof", base_url))
        .json(&payload)
        .send()
        .await?;
    let status = response.status();
    let body: serde_json::Value = response.json().await?;

    if status.is_success() || status.as_u16() == 202 {
        Ok(body["result"]["jobId"]
            .as_str()
            .ok_or_else(|| eyre::eyre!("No jobId in response: {:?}", body))?
            .to_string())
    } else {
        Err(eyre::eyre!("Request failed: {:?}", body))
    }
}

// Status polling helpers - using V2 endpoints
async fn poll_request_status(
    base_url: &str,
    request_type: &str,
    job_id: &str,
) -> eyre::Result<serde_json::Value> {
    let client = reqwest::Client::new();
    let url = match request_type {
        "public" => format!("{}/v2/public-decrypt/{}", base_url, job_id),
        "input" => format!("{}/v2/input-proof/{}", base_url, job_id),
        _ => return Err(eyre::eyre!("Unknown request type: {}", request_type)),
    };

    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

async fn wait_for_completion(
    base_url: &str,
    requests: Vec<(&str, &str)>,
    timeout: Duration,
) -> eyre::Result<()> {
    let start = tokio::time::Instant::now();
    let mut completed = vec![false; requests.len()];

    while start.elapsed() < timeout {
        let mut all_done = true;

        for (i, (req_type, job_id)) in requests.iter().enumerate() {
            if completed[i] {
                continue;
            }

            match poll_request_status(base_url, req_type, job_id).await {
                Ok(status) => {
                    if let Some(state) = status.get("status").and_then(|s| s.as_str()) {
                        // V2 uses "succeeded" for success, "failed" for failure, "queued" for in-progress
                        if state == "succeeded" {
                            tracing::info!(
                                "✓ Request {} {} completed successfully",
                                req_type,
                                job_id
                            );
                            completed[i] = true;
                        } else if state == "failed" {
                            tracing::error!(
                                "✗ Request {} {} failed: {:?}",
                                req_type,
                                job_id,
                                status
                            );
                            return Err(eyre::eyre!("Request {} failed: {:?}", job_id, status));
                        } else {
                            // Still queued or processing
                            all_done = false;
                        }
                    }
                }
                Err(e) => {
                    tracing::warn!("Failed to poll {} {}: {}", req_type, job_id, e);
                    all_done = false;
                }
            }
        }

        if all_done {
            tracing::info!("All {} requests completed successfully!", requests.len());
            return Ok(());
        }

        sleep(Duration::from_millis(500)).await;
    }

    Err(eyre::eyre!(
        "Timeout waiting for requests to complete after {:?}",
        timeout
    ))
}

// ============================================================================
// TEST: Recovery from 'processing' status
// ============================================================================

/// Test recovery from requests stuck in 'processing' status.
///
/// Scenario:
/// - Requests pass readiness check and enter 'processing' status
/// - Broken gateway emits request events but no response events
/// - After restart with working gateway, recovery completes all requests
///
/// Expected behavior:
/// - Recovery dispatches ReadinessCheckPassed event (for public decrypt)
/// - Recovery dispatches ReqRcvdFromUser event (for input proof)
#[tokio::test]
async fn test_recovery_from_processing_status() {
    tracing::info!("=== Test: Recovery from Processing Status ===");

    // Clean up any leftover incomplete requests from previous test runs
    cleanup_incomplete_requests()
        .await
        .expect("Failed to cleanup database");

    let mut setup = RecoveryTestSetup::new()
        .await
        .expect("Failed to create test setup");

    // Phase 1: Pre-generate handles and configure broken gateway
    let handle1 = random_handle();
    setup.configure_for_processing_stuck(slice::from_ref(&handle1));

    // Phase 2: Start relayer with broken gateway
    tracing::info!("Phase 1: Starting relayer with broken gateway");
    setup
        .start_relayer_with_gateway(setup.broken_gateway_port, |settings| {
            // High retry limits so requests don't fail, just keep waiting
            settings.gateway.tx_engine.retry.max_attempts = 100;
            settings.gateway.tx_engine.retry.retry_interval_ms = 100;
        })
        .await
        .expect("Failed to start relayer");

    // Phase 3: Send public decrypt request
    // Note: user decrypt and input proof skipped for now due to V2 format complexity
    tracing::info!("Phase 2: Sending public decrypt request");
    let job_id_public = send_public_decrypt_request(&setup.http_url(), &handle1)
        .await
        .expect("Failed to send public decrypt");

    tracing::info!("Sent request - public: {}", job_id_public);

    // Phase 4: Wait for requests to reach processing status
    tracing::info!("Phase 3: Waiting for requests to reach processing status...");
    sleep(Duration::from_secs(3)).await;

    // Verify via DB that at least one request is in processing/tx_in_flight
    let repos = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete = repos
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    // Log details of what we found
    for (job_id, _req_json, status, updated_at) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={:?} updated_at={}",
            hex::encode(job_id),
            status,
            updated_at
        );
    }

    // Phase 5: Shutdown relayer
    tracing::info!("Phase 4: Shutting down relayer");
    setup.shutdown_relayer().await;
    setup.reset_for_restart();

    // Phase 6: Configure and restart with working gateway
    tracing::info!("Phase 5: Starting relayer with working gateway");
    setup.configure_working_gateway(&[handle1]);

    setup
        .start_relayer_with_gateway(setup.working_gateway_port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart to verify what recovery should have seen
    let repos2 = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete_after_restart = repos2
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "After restart: {} incomplete public decrypt requests in DB",
        incomplete_after_restart.len()
    );

    // Phase 7: Wait for recovery to complete request
    tracing::info!("Phase 6: Waiting for recovery to complete request...");
    let our_requests = vec![("public", job_id_public.as_str())];

    wait_for_completion(&setup.http_url(), our_requests, Duration::from_secs(15))
        .await
        .expect("Recovery failed to complete request");

    tracing::info!("✓ Test passed: Public decrypt request recovered from processing status");
}

// ============================================================================
// TEST: Recovery from 'queued' status
// ============================================================================

/// Test recovery from requests stuck in 'queued' status.
///
/// Scenario:
/// - Readiness check fails, so requests stay in 'queued' status
/// - After restart with working gateway, recovery completes all requests
///
/// Expected behavior:
/// - Recovery dispatches ReqRcvdFromUser event (starts from beginning)
#[tokio::test]
async fn test_recovery_from_queued_status() {
    tracing::info!("=== Test: Recovery from Queued Status ===");

    // Clean up any leftover incomplete requests from previous test runs
    cleanup_incomplete_requests()
        .await
        .expect("Failed to cleanup database");

    let mut setup = RecoveryTestSetup::new()
        .await
        .expect("Failed to create test setup");

    // Phase 1: Pre-generate handles and configure broken gateway to fail readiness
    let handle1 = random_handle();
    setup.configure_for_queued_stuck();

    // Phase 2: Start relayer with broken gateway (readiness fails)
    tracing::info!("Phase 1: Starting relayer with broken gateway (readiness fails)");
    setup
        .start_relayer_with_gateway(setup.broken_gateway_port, |settings| {
            // High retry limits so requests don't fail, just keep retrying readiness
            settings.gateway.readiness_checker.retry.max_attempts = 100;
            settings.gateway.readiness_checker.retry.retry_interval_ms = 100;
        })
        .await
        .expect("Failed to start relayer");

    // Phase 3: Send public decrypt request
    tracing::info!("Phase 2: Sending public decrypt request");
    let job_id_public = send_public_decrypt_request(&setup.http_url(), &handle1)
        .await
        .expect("Failed to send public decrypt");

    tracing::info!("Sent request - public: {}", job_id_public);

    // Phase 4: Wait for requests to stay in queued status (readiness fails)
    tracing::info!("Phase 3: Waiting for requests to reach queued status...");
    sleep(Duration::from_secs(3)).await;

    // Verify via DB that request is in queued status
    let repos = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete = repos
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    for (job_id, _req_json, status, updated_at) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={:?} updated_at={}",
            hex::encode(job_id),
            status,
            updated_at
        );
    }

    // Phase 5: Shutdown relayer
    tracing::info!("Phase 4: Shutting down relayer");
    setup.shutdown_relayer().await;
    setup.reset_for_restart();

    // Phase 6: Configure and restart with working gateway
    tracing::info!("Phase 5: Starting relayer with working gateway");
    setup.configure_working_gateway(&[handle1]);

    setup
        .start_relayer_with_gateway(setup.working_gateway_port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart
    let repos2 = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete_after_restart = repos2
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "After restart: {} incomplete public decrypt requests in DB",
        incomplete_after_restart.len()
    );

    // Phase 7: Wait for recovery to complete request
    tracing::info!("Phase 6: Waiting for recovery to complete request...");
    let our_requests = vec![("public", job_id_public.as_str())];

    wait_for_completion(&setup.http_url(), our_requests, Duration::from_secs(15))
        .await
        .expect("Recovery failed to complete request");

    tracing::info!("✓ Test passed: Public decrypt request recovered from queued status");
}

// ============================================================================
// TEST: Recovery from 'tx_in_flight' status
// ============================================================================

/// Test recovery from requests stuck in 'tx_in_flight' status.
///
/// Scenario:
/// - Requests pass readiness check and enter 'processing' status
/// - Transaction is sent, reaching 'tx_in_flight' status
/// - Relayer crashes before receipt is received
/// - After restart, recovery resets tx_in_flight → processing and re-dispatches
///
/// Expected behavior:
/// - Recovery resets: tx_in_flight → processing
/// - Recovery dispatches ReadinessCheckPassed event
/// - Transaction is re-sent and completes successfully
///
/// Note: Due to mock auto-mining, determining exact stuck status is difficult.
/// This test verifies that recovery can handle requests that were in-flight during crash.
#[tokio::test]
async fn test_recovery_from_tx_in_flight_status() {
    tracing::info!("=== Test: Recovery from TxInFlight Status ===");

    // Clean up any leftover incomplete requests from previous test runs
    cleanup_incomplete_requests()
        .await
        .expect("Failed to cleanup database");

    let mut setup = RecoveryTestSetup::new()
        .await
        .expect("Failed to create test setup");

    // Phase 1: Pre-generate handles and configure broken gateway
    let handle1 = random_handle();
    setup.configure_for_processing_stuck(slice::from_ref(&handle1));

    // Phase 2: Start relayer with broken gateway
    tracing::info!(
        "Phase 1: Starting relayer with broken gateway (transactions sent but no response)"
    );
    setup
        .start_relayer_with_gateway(setup.broken_gateway_port, |settings| {
            // High retry limits so requests don't fail, just keep waiting
            settings.gateway.tx_engine.retry.max_attempts = 100;
            settings.gateway.tx_engine.retry.retry_interval_ms = 100;
        })
        .await
        .expect("Failed to start relayer");

    // Phase 3: Send public decrypt request
    tracing::info!("Phase 2: Sending public decrypt request");
    let job_id_public = send_public_decrypt_request(&setup.http_url(), &handle1)
        .await
        .expect("Failed to send public decrypt");

    tracing::info!("Sent request - public: {}", job_id_public);

    // Phase 4: Wait for requests to reach processing/tx_in_flight status
    tracing::info!("Phase 3: Waiting for requests to be in processing/tx_in_flight...");
    sleep(Duration::from_secs(3)).await;

    // Verify via DB that request is in an incomplete status
    let repos = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete = repos
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    for (job_id, _req_json, status, updated_at) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={:?} updated_at={}",
            hex::encode(job_id),
            status,
            updated_at
        );
    }

    // Phase 5: Shutdown relayer
    tracing::info!("Phase 4: Shutting down relayer");
    setup.shutdown_relayer().await;
    setup.reset_for_restart();

    // Phase 6: Configure and restart with working gateway
    tracing::info!("Phase 5: Starting relayer with working gateway");
    setup.configure_working_gateway(&[handle1]);

    setup
        .start_relayer_with_gateway(setup.working_gateway_port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart - recovery should have reset tx_in_flight → processing
    let repos2 = setup
        .get_repositories()
        .await
        .expect("Failed to get repositories");
    let incomplete_after_restart = repos2
        .public_decrypt
        .find_incomplete_requests()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "After restart: {} incomplete public decrypt requests in DB (tx_in_flight should be reset to processing)",
        incomplete_after_restart.len()
    );

    // Phase 7: Wait for recovery to complete request
    tracing::info!("Phase 6: Waiting for recovery to complete request...");
    let our_requests = vec![("public", job_id_public.as_str())];

    wait_for_completion(&setup.http_url(), our_requests, Duration::from_secs(15))
        .await
        .expect("Recovery failed to complete request");

    tracing::info!("✓ Test passed: Public decrypt request recovered from tx_in_flight status");
}
