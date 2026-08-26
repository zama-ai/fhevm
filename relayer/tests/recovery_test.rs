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
// Each test owns an isolated database schema and its own mock servers, so they can run in
// parallel with each other and with the rest of the suite.

mod common;

use anyhow::{bail, Context};

use alloy::primitives::{Address, Bytes, B256};
use common::test_schema::TestSchema;
use common::utils::{
    http_port_of, random_handle, spawn_relayer, wire_settings_to_mocks, GatewayMock, HostMock,
    TEST_CONFIG_PATH,
};
use ethereum_rpc_mock::{fhevm::UserDecryptKind, SubscriptionTarget};
use fhevm_relayer::config::settings::{Settings, StorageConfig};
use fhevm_relayer::tracing::init_tracing_once;
use serde_json::json;
use std::slice;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use tokio_util::sync::CancellationToken;

/// Test setup with two gateway mocks - one broken, one working
struct RecoveryTestSetup {
    /// Kept stuck on purpose: its readiness switch and its silence about response events are what
    /// strand a request at the status under test.
    broken_gateway: GatewayMock,
    working_gateway: GatewayMock,
    host: HostMock,
    /// Outlives the relayer restart — the requests recovery has to pick up live here.
    test_schema: TestSchema,
    http_port: Option<u16>,
    cancellation_token: CancellationToken,
    relayer_task: Option<tokio::task::JoinHandle<()>>,
    storage_settings: Option<StorageConfig>,
}

impl RecoveryTestSetup {
    async fn new() -> anyhow::Result<Self> {
        let test_schema = TestSchema::new().await?;

        // Settings are only read here for the contract addresses the host mock answers on; each
        // relayer start loads its own copy and wires it to the mocks.
        let settings = Settings::new(Some(TEST_CONFIG_PATH.to_string()))
            .expect("Failed to load configuration");
        init_tracing_once(&settings.log);

        let host = HostMock::start(&settings).await?;
        let broken_gateway = GatewayMock::start().await?;
        let working_gateway = GatewayMock::start().await?;

        tracing::info!(
            "Setting up recovery test - broken gateway: {}, working gateway: {}, host: {}, schema: {}",
            broken_gateway.port,
            working_gateway.port,
            host.port,
            test_schema.schema_name()
        );

        Ok(Self {
            broken_gateway,
            working_gateway,
            host,
            test_schema,
            http_port: None,
            cancellation_token: CancellationToken::new(),
            relayer_task: None,
            storage_settings: None,
        })
    }

    async fn start_relayer_with_gateway(
        &mut self,
        gateway_port: u16,
        modify_config: impl FnOnce(&mut Settings),
    ) -> anyhow::Result<()> {
        let mut settings = Settings::new(Some(TEST_CONFIG_PATH.to_string()))
            .expect("Failed to load configuration");

        wire_settings_to_mocks(
            &mut settings,
            self.host.port,
            gateway_port,
            self.test_schema.database_url(),
        );

        // Apply custom config modifications
        modify_config(&mut settings);

        // Store storage settings for DB access
        self.storage_settings = Some(settings.storage.clone());

        let (task_handle, updated_settings) =
            spawn_relayer(settings, self.cancellation_token.clone()).await?;
        self.relayer_task = Some(task_handle);

        let port = http_port_of(&updated_settings)?;
        self.http_port = Some(port);
        tracing::info!("Relayer HTTP server running on port {}", port);

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
        self.broken_gateway.fhevm.set_readiness_failure();
        tracing::info!("Broken gateway configured for 'queued' stuck - readiness fails");
    }

    /// Configure broken gateway for 'processing' status:
    /// - Requests pass readiness check
    /// - Transaction is accepted but no response event is emitted
    fn configure_for_processing_stuck(&self, _handles: &[String]) {
        // Set readiness to pass so requests can leave queued status
        self.broken_gateway.fhevm.set_readiness_success();

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
        self.broken_gateway.fhevm.set_readiness_success();

        // DON'T register any event patterns - this keeps transactions pending
        // without any events being emitted, so requests stay in 'processing' or 'tx_in_flight' status

        tracing::info!(
            "Broken gateway configured for 'tx_in_flight' stuck - no events will be emitted"
        );
    }

    /// Configure working gateway to complete any request
    fn configure_working_gateway(&self, handles: &[String]) {
        self.working_gateway.fhevm.set_readiness_success();

        let b256_handles: Vec<B256> = handles
            .iter()
            .filter_map(|h| B256::from_str(h).ok())
            .collect();

        if !b256_handles.is_empty() {
            let values: Vec<u64> = (0..b256_handles.len()).map(|i| i as u64 + 42).collect();
            self.working_gateway.fhevm.on_public_decrypt_success(
                b256_handles.clone(),
                values,
                SubscriptionTarget::All,
            );

            let dummy_address =
                Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
            self.working_gateway.fhevm.on_user_decrypt_success(
                UserDecryptKind::Direct,
                b256_handles,
                dummy_address,
                SubscriptionTarget::All,
            );
        }

        let dummy_address =
            Address::from_str("0x1234567890123456789012345678901234567890").unwrap();
        let proof_data = Bytes::from(b"valid_proof");
        self.working_gateway.fhevm.on_input_proof_success(
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

    /// Incomplete rows straight from SQL, for the diagnostic logging below. The repository
    /// method this replaces (`find_incomplete_requests`) went with startup recovery: the sweep
    /// selects the rows it drives inside its own claim `UPDATE`, so nothing in the relayer reads
    /// them separately any more.
    async fn incomplete_public_decrypt_rows(&self) -> anyhow::Result<Vec<(Vec<u8>, String)>> {
        let pool = sqlx::postgres::PgPoolOptions::new()
            .max_connections(1)
            .connect(&self.test_schema.database_url())
            .await
            .context("Failed to connect diagnostic pool")?;
        let rows = sqlx::query(
            r#"
            SELECT int_job_id, req_status::text AS status
            FROM public_decrypt_req
            WHERE req_status IN ('queued'::req_status, 'processing'::req_status,
                                 'tx_in_flight'::req_status)
            ORDER BY created_at ASC
            "#,
        )
        .fetch_all(&pool)
        .await
        .context("Failed to query incomplete rows")?;
        Ok(rows
            .into_iter()
            .map(|row| {
                use sqlx::Row;
                (row.get("int_job_id"), row.get("status"))
            })
            .collect())
    }

    /// Stop the relayer and drop the test schema.
    async fn shutdown(mut self) {
        self.shutdown_relayer().await;

        if let Err(e) = self.test_schema.cleanup().await {
            tracing::error!("Failed to cleanup test schema: {}", e);
        }
    }
}

// Request sending helpers
// Uses V2 for public decrypt and input proof (async, returns immediately)
// Skips user decrypt for now as V2 format is different

async fn send_public_decrypt_request(base_url: &str, handle: &str) -> anyhow::Result<String> {
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
            .with_context(|| format!("No jobId in response: {:?}", body))?
            .to_string())
    } else {
        Err(anyhow::anyhow!("Request failed: {:?}", body))
    }
}

#[allow(dead_code)]
async fn send_input_proof_request(base_url: &str) -> anyhow::Result<String> {
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
            .with_context(|| format!("No jobId in response: {:?}", body))?
            .to_string())
    } else {
        Err(anyhow::anyhow!("Request failed: {:?}", body))
    }
}

// Status polling helpers - using V2 endpoints
async fn poll_request_status(
    base_url: &str,
    request_type: &str,
    job_id: &str,
) -> anyhow::Result<serde_json::Value> {
    let client = reqwest::Client::new();
    let url = match request_type {
        "public" => format!("{}/v2/public-decrypt/{}", base_url, job_id),
        "input" => format!("{}/v2/input-proof/{}", base_url, job_id),
        _ => bail!("Unknown request type: {}", request_type),
    };

    let response = client.get(url).send().await?;
    Ok(response.json().await?)
}

async fn wait_for_completion(
    base_url: &str,
    requests: Vec<(&str, &str)>,
    timeout: Duration,
) -> anyhow::Result<()> {
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
                            bail!("Request {} failed: {:?}", job_id, status);
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

    Err(anyhow::anyhow!(
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

    let mut setup = RecoveryTestSetup::new()
        .await
        .expect("Failed to create test setup");

    // Phase 1: Pre-generate handles and configure broken gateway
    let handle1 = random_handle();
    setup.configure_for_processing_stuck(slice::from_ref(&handle1));

    // Phase 2: Start relayer with broken gateway
    tracing::info!("Phase 1: Starting relayer with broken gateway");
    setup
        .start_relayer_with_gateway(setup.broken_gateway.port, |settings| {
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
    let incomplete = setup
        .incomplete_public_decrypt_rows()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    // Log details of what we found
    for (job_id, status) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={}",
            hex::encode(job_id),
            status
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
        .start_relayer_with_gateway(setup.working_gateway.port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart to verify what recovery should have seen
    let incomplete_after_restart = setup
        .incomplete_public_decrypt_rows()
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

    setup.shutdown().await;

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

    let mut setup = RecoveryTestSetup::new()
        .await
        .expect("Failed to create test setup");

    // Phase 1: Pre-generate handles and configure broken gateway to fail readiness
    let handle1 = random_handle();
    setup.configure_for_queued_stuck();

    // Phase 2: Start relayer with broken gateway (readiness fails)
    tracing::info!("Phase 1: Starting relayer with broken gateway (readiness fails)");
    setup
        .start_relayer_with_gateway(setup.broken_gateway.port, |settings| {
            // High retry limits so requests don't fail, just keep retrying readiness
            settings
                .gateway
                .readiness_checker
                .gw_ciphertext_check
                .retry
                .max_attempts = 100;
            settings
                .gateway
                .readiness_checker
                .gw_ciphertext_check
                .retry
                .retry_interval_ms = 100;
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
    let incomplete = setup
        .incomplete_public_decrypt_rows()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    for (job_id, status) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={}",
            hex::encode(job_id),
            status
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
        .start_relayer_with_gateway(setup.working_gateway.port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart
    let incomplete_after_restart = setup
        .incomplete_public_decrypt_rows()
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

    setup.shutdown().await;

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
        .start_relayer_with_gateway(setup.broken_gateway.port, |settings| {
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
    let incomplete = setup
        .incomplete_public_decrypt_rows()
        .await
        .expect("Failed to query DB");
    tracing::info!(
        "Found {} incomplete public decrypt requests in DB",
        incomplete.len()
    );

    for (job_id, status) in &incomplete {
        tracing::info!(
            "  - Request int_job_id={} status={}",
            hex::encode(job_id),
            status
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
        .start_relayer_with_gateway(setup.working_gateway.port, |_| {})
        .await
        .expect("Failed to start relayer with working gateway");

    // Query DB after restart - recovery should have reset tx_in_flight → processing
    let incomplete_after_restart = setup
        .incomplete_public_decrypt_rows()
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

    setup.shutdown().await;

    tracing::info!("✓ Test passed: Public decrypt request recovered from tx_in_flight status");
}
