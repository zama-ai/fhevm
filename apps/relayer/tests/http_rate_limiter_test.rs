mod common;

use crate::common::utils::TestSetup;
use alloy::primitives::{Address, Bytes, B256};
use futures::future::join_all;
use rand::{rng, Rng};
use serde_json::json;
use std::str::FromStr;
use tokio::time::{sleep, Duration};

mod constants {
    pub const EXTRA_DATA: &str = "0x00";
    pub const REQUEST_VALIDITY_START: &str = "1742450894";
    pub const REQUEST_VALIDITY_DAYS: &str = "10";
}

mod helpers {
    use super::*;
    use crate::common::utils;

    pub fn v1_user_decrypt_url(setup: &TestSetup) -> String {
        format!("http://localhost:{}/v1/user-decrypt", setup.http_port)
    }

    pub fn random_address() -> Address {
        utils::random_address()
    }

    pub fn random_handle() -> String {
        utils::random_handle()
    }

    pub fn random_signature() -> String {
        let mut rng = rng();
        (0..130)
            .map(|_| rng.random_range(0..16))
            .map(|digit| format!("{:x}", digit))
            .collect()
    }

    pub fn random_public_key() -> String {
        let mut rng = rng();
        (0..64)
            .map(|_| rng.random_range(0..16))
            .map(|digit| format!("{:x}", digit))
            .collect()
    }

    pub fn random_encrypted_bytes() -> Bytes {
        let mut rng = rng();
        let bytes: Vec<u8> = (0..32).map(|_| rng.random()).collect();
        Bytes::from(bytes)
    }

    pub fn create_user_decrypt_payload(
        chain_id: &str,
        contract_address: Address,
        user_address: Address,
    ) -> serde_json::Value {
        let handle = random_handle();
        json!({
            "handleContractPairs": [{
                "handle": handle,
                "contractAddress": format!("{:?}", contract_address)
            }],
            "requestValidity": {
                "startTimestamp": constants::REQUEST_VALIDITY_START,
                "durationDays": constants::REQUEST_VALIDITY_DAYS
            },
            "contractsChainId": chain_id,
            "contractAddresses": [format!("{:?}", contract_address)],
            "userAddress": format!("{:?}", user_address),
            "signature": random_signature(),
            "publicKey": random_public_key(),
            "extraData": constants::EXTRA_DATA
        })
    }

    pub fn extract_ciphertext_handles_from_user_payload(payload: &serde_json::Value) -> Vec<B256> {
        payload["handleContractPairs"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|pair| {
                pair["handle"].as_str().and_then(|s| {
                    let cleaned = s.strip_prefix("0x").unwrap_or(s);
                    B256::from_str(cleaned).ok()
                })
            })
            .collect()
    }
}

#[derive(Debug, Clone)]
struct Batch {
    request_count: u32,
    delay_between_requests_ms: u64,
    delay_after_batch_ms: u64,
}

impl Batch {
    fn new(request_count: u32, delay_between_requests_ms: u64, delay_after_batch_ms: u64) -> Self {
        Self {
            request_count,
            delay_between_requests_ms,
            delay_after_batch_ms,
        }
    }
}

#[derive(Debug, Clone)]
struct TestScenario {
    name: String,
    batches: Vec<Batch>,
    expected_results: Vec<BatchExpectation>,
}

#[derive(Debug, Clone)]
struct BatchExpectation {
    min_successful: u32,
    max_successful: u32,
    min_rate_limit_post_endpoints: u32,
    max_rate_limit_post_endpoints: u32,
    max_errors: u32,
}

impl TestScenario {
    fn new(name: &str, batches: Vec<Batch>, expected_results: Vec<BatchExpectation>) -> Self {
        Self {
            name: name.to_string(),
            batches,
            expected_results,
        }
    }
}

#[derive(Debug)]
struct BatchResult {
    successful_requests: u32,
    rate_limit_post_endpoints_requests: u32,
    error_requests: u32,
    retry_after_header_count: u32, // Count of 429 responses that had Retry-After header
}

async fn execute_batch(
    client: &reqwest::Client,
    url: &str,
    payload: &serde_json::Value,
    batch: &Batch,
) -> BatchResult {
    let mut all_tasks = Vec::new();

    for i in 0..batch.request_count {
        let client = client.clone();
        let url = url.to_string();
        let payload = payload.clone();

        let task = tokio::spawn(async move {
            let res = client
                .post(&url)
                .header("Content-Type", "application/json")
                .timeout(Duration::from_secs(10))
                .json(&payload)
                .send()
                .await;

            let (status, has_valid_retry_after_header) = match &res {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let has_valid_retry_after = if status == 429 {
                        // For 429 responses, check for Retry-After header with valid numeric value
                        response
                            .headers()
                            .get("retry-after")
                            .or_else(|| response.headers().get("Retry-After"))
                            .and_then(|header_val| header_val.to_str().ok())
                            .and_then(|header_str| header_str.parse::<u32>().ok())
                            .is_some()
                    } else {
                        true // Non-429 responses don't need Retry-After header
                    };
                    (status, has_valid_retry_after)
                }
                Err(_) => (0, false),
            };

            (i, status, res.is_ok(), has_valid_retry_after_header)
        });

        all_tasks.push(task);

        // Add delay between requests if specified
        if batch.delay_between_requests_ms > 0 && i < batch.request_count - 1 {
            sleep(Duration::from_millis(batch.delay_between_requests_ms)).await;
        }
    }

    // Wait for all requests in this batch to complete
    let results = join_all(all_tasks).await;

    // Analyze results for this batch
    let mut successful_requests = 0;
    let mut rate_limit_post_endpoints_requests = 0;
    let mut error_requests = 0;
    let mut retry_after_header_count = 0;

    for result in results {
        if let Ok((_request_id, status, _success, has_valid_retry_after)) = result {
            match status {
                200 => successful_requests += 1,
                429 => {
                    rate_limit_post_endpoints_requests += 1;
                    if has_valid_retry_after {
                        retry_after_header_count += 1;
                    }
                }
                _ => error_requests += 1,
            }
        } else {
            error_requests += 1;
        }
    }

    BatchResult {
        successful_requests,
        rate_limit_post_endpoints_requests,
        error_requests,
        retry_after_header_count,
    }
}

async fn execute_get_batch(client: &reqwest::Client, url: &str, batch: &Batch) -> BatchResult {
    let mut all_tasks = Vec::new();

    for i in 0..batch.request_count {
        let client = client.clone();
        let url = url.to_string();

        let task = tokio::spawn(async move {
            let res = client
                .get(&url)
                .timeout(Duration::from_secs(10))
                .send()
                .await;

            let (status, has_valid_retry_after_header) = match &res {
                Ok(response) => {
                    let status = response.status().as_u16();
                    let has_valid_retry_after = if status == 429 {
                        // For 429 responses, check for Retry-After header with valid numeric value
                        response
                            .headers()
                            .get("retry-after")
                            .or_else(|| response.headers().get("Retry-After"))
                            .and_then(|header_val| header_val.to_str().ok())
                            .and_then(|header_str| header_str.parse::<u32>().ok())
                            .is_some()
                    } else {
                        true // Non-429 responses don't need Retry-After header
                    };
                    (status, has_valid_retry_after)
                }
                Err(_) => (0, false),
            };

            (i, status, res.is_ok(), has_valid_retry_after_header)
        });

        all_tasks.push(task);

        // Add delay between requests if specified
        if batch.delay_between_requests_ms > 0 && i < batch.request_count - 1 {
            sleep(Duration::from_millis(batch.delay_between_requests_ms)).await;
        }
    }

    // Wait for all requests in this batch to complete
    let results = join_all(all_tasks).await;

    // Analyze results for this batch
    let mut successful_requests = 0;
    let mut rate_limit_post_endpoints_requests = 0;
    let mut error_requests = 0;
    let mut retry_after_header_count = 0;

    for result in results {
        if let Ok((_request_id, status, _success, has_valid_retry_after)) = result {
            match status {
                200 => successful_requests += 1,
                429 => {
                    rate_limit_post_endpoints_requests += 1;
                    if has_valid_retry_after {
                        retry_after_header_count += 1;
                    }
                }
                _ => error_requests += 1,
            }
        } else {
            error_requests += 1;
        }
    }

    BatchResult {
        successful_requests,
        rate_limit_post_endpoints_requests,
        error_requests,
        retry_after_header_count,
    }
}

async fn run_scenario(
    setup: &TestSetup,
    scenario: &TestScenario,
    payload: &serde_json::Value,
) -> Vec<BatchResult> {
    let client = reqwest::Client::new();
    let url = helpers::v1_user_decrypt_url(setup);
    let mut batch_results = Vec::new();

    println!("Running scenario: {}", scenario.name);

    for (batch_idx, batch) in scenario.batches.iter().enumerate() {
        println!(
            "  Batch {}: {} requests, {}ms delay between, {}ms delay after",
            batch_idx + 1,
            batch.request_count,
            batch.delay_between_requests_ms,
            batch.delay_after_batch_ms
        );

        let result = execute_batch(&client, &url, payload, batch).await;

        println!(
            "    Results: {} successful, {} rate limited ({} with valid Retry-After), {} errors",
            result.successful_requests,
            result.rate_limit_post_endpoints_requests,
            result.retry_after_header_count,
            result.error_requests
        );

        batch_results.push(result);

        // Add delay after batch if specified and not the last batch
        if batch.delay_after_batch_ms > 0 && batch_idx < scenario.batches.len() - 1 {
            sleep(Duration::from_millis(batch.delay_after_batch_ms)).await;
        }
    }

    batch_results
}

/// Helper function to create batch expectations based on rate limit config
/// This dynamically calculates expected results based on the actual rate limiter configuration,
/// eliminating the need to update test expectations when rate limits are changed.
fn create_batch_expectation(
    requests: u32,
    rate_limit_config: &crate::common::utils::TestSetup,
    tolerance_percent: f32,
) -> BatchExpectation {
    let burst_size = rate_limit_config
        .settings
        .http
        .rate_limit_post_endpoints
        .burst_size;
    let requests_per_second = rate_limit_config
        .settings
        .http
        .rate_limit_post_endpoints
        .requests_per_second;

    let tolerance = (requests as f32 * tolerance_percent / 100.0).ceil() as u32;

    if requests <= burst_size {
        // Within burst capacity - expect most/all to succeed
        BatchExpectation {
            min_successful: (requests as f32 * (1.0 - tolerance_percent / 100.0))
                .max(requests.saturating_sub(tolerance) as f32) as u32,
            max_successful: requests,
            min_rate_limit_post_endpoints: 0,
            max_rate_limit_post_endpoints: tolerance,
            max_errors: 2,
        }
    } else {
        // Exceeds burst capacity - expect rate limiting
        let expected_successful = burst_size + (requests_per_second / 10); // Allow some refill
        BatchExpectation {
            min_successful: expected_successful.saturating_sub(tolerance),
            max_successful: expected_successful + tolerance,
            min_rate_limit_post_endpoints: requests.saturating_sub(expected_successful + tolerance),
            max_rate_limit_post_endpoints: requests
                .saturating_sub(expected_successful.saturating_sub(tolerance)),
            max_errors: 2,
        }
    }
}

/// Helper function to create expectations for controlled rate testing
fn create_controlled_rate_expectation(
    requests: u32,
    delay_ms: u64,
    rate_limit_config: &crate::common::utils::TestSetup,
) -> BatchExpectation {
    let requests_per_second = rate_limit_config
        .settings
        .http
        .rate_limit_post_endpoints
        .requests_per_second;
    let actual_rps = if delay_ms > 0 {
        1000 / delay_ms
    } else {
        u64::MAX
    };

    if actual_rps <= requests_per_second as u64 {
        // Within rate limit - expect all to succeed
        BatchExpectation {
            min_successful: requests.saturating_sub(2), // Small tolerance for timing
            max_successful: requests,
            min_rate_limit_post_endpoints: 0,
            max_rate_limit_post_endpoints: 2,
            max_errors: 2,
        }
    } else {
        // Exceeds rate limit
        let expected_successful =
            (requests_per_second as f32 * (requests as f32 / actual_rps as f32)).ceil() as u32;
        BatchExpectation {
            min_successful: expected_successful.saturating_sub(2),
            max_successful: expected_successful + 2,
            min_rate_limit_post_endpoints: requests.saturating_sub(expected_successful + 2),
            max_rate_limit_post_endpoints: requests
                .saturating_sub(expected_successful.saturating_sub(2)),
            max_errors: 2,
        }
    }
}

fn validate_scenario_results(scenario: &TestScenario, results: &[BatchResult]) {
    assert_eq!(
        results.len(),
        scenario.expected_results.len(),
        "Scenario '{}': Number of batch results doesn't match expectations",
        scenario.name
    );

    for (batch_idx, (result, expectation)) in results
        .iter()
        .zip(scenario.expected_results.iter())
        .enumerate()
    {
        assert!(
            result.successful_requests >= expectation.min_successful
                && result.successful_requests <= expectation.max_successful,
            "Scenario '{}', Batch {}: Expected {}-{} successful requests, got {}",
            scenario.name,
            batch_idx + 1,
            expectation.min_successful,
            expectation.max_successful,
            result.successful_requests
        );

        assert!(
            result.rate_limit_post_endpoints_requests >= expectation.min_rate_limit_post_endpoints
                && result.rate_limit_post_endpoints_requests
                    <= expectation.max_rate_limit_post_endpoints,
            "Scenario '{}', Batch {}: Expected {}-{} rate limited requests, got {}",
            scenario.name,
            batch_idx + 1,
            expectation.min_rate_limit_post_endpoints,
            expectation.max_rate_limit_post_endpoints,
            result.rate_limit_post_endpoints_requests
        );

        assert!(
            result.error_requests <= expectation.max_errors,
            "Scenario '{}', Batch {}: Expected at most {} errors, got {}",
            scenario.name,
            batch_idx + 1,
            expectation.max_errors,
            result.error_requests
        );

        // Validate that ALL 429 responses have valid Retry-After headers
        assert_eq!(
            result.rate_limit_post_endpoints_requests, result.retry_after_header_count,
            "Scenario '{}', Batch {}: ALL 429 responses must have valid Retry-After header. Got {} 429s but only {} with valid headers",
            scenario.name,
            batch_idx + 1,
            result.rate_limit_post_endpoints_requests,
            result.retry_after_header_count
        );
    }
}

#[tokio::test]
async fn test_user_decrypt_rate_limit_post_endpoints_scenarios() {
    // Setup test environment
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Prepare test data
    let user_address = helpers::random_address();
    let contract_address = helpers::random_address();
    let payload = helpers::create_user_decrypt_payload(
        &setup.settings.gateway.blockchain_rpc.chain_id.to_string(),
        contract_address,
        user_address,
    );
    let handles = helpers::extract_ciphertext_handles_from_user_payload(&payload);
    let encrypted_bytes = helpers::random_encrypted_bytes();

    // Configure mock for all requests (generous limit to cover all scenarios)
    for _ in 0..500 {
        setup.fhevm_mock.on_user_decrypt_success(
            handles.clone(),
            user_address,
            encrypted_bytes.clone(),
            ethereum_rpc_mock::SubscriptionTarget::All,
        );
    }

    // Log the rate limiter configuration being used for testing
    let rate_config = &setup.settings.http.rate_limit_post_endpoints;
    println!(
        "Rate limiter config: {} requests/sec, {} burst size",
        rate_config.requests_per_second, rate_config.burst_size
    );

    // Define batches for testing (values relative to burst_size from config)
    let burst_size = rate_config.burst_size;
    let requests_per_second = rate_config.requests_per_second;

    let burst_batches = vec![
        Batch::new(burst_size, 0, 1000), // burst_size concurrent requests, 1s delay after
        Batch::new(burst_size, 0, 1000), // burst_size concurrent requests, 1s delay after
        Batch::new(burst_size * 2, 0, 1000), // 2x burst_size concurrent requests (should hit limit), 1s delay after
        Batch::new(burst_size, 0, 0),        // burst_size concurrent requests
    ];

    // Calculate delay for controlled rate (slightly under the requests_per_second limit)
    let controlled_rps = requests_per_second.saturating_sub(4); // Stay 4 RPS under limit for safety
    let delay_ms = if controlled_rps > 0 {
        1000u64 / controlled_rps as u64
    } else {
        100u64
    };
    let controlled_requests = controlled_rps * 3; // 3 seconds worth of requests

    let controlled_batches = vec![
        Batch::new(controlled_requests, delay_ms, 2000), // controlled requests at safe rate, 2s delay after
    ];

    // Create expectations based on rate limiter config (5% tolerance)
    let burst_expectations = vec![
        create_batch_expectation(burst_size, &setup, 5.0),
        create_batch_expectation(burst_size, &setup, 5.0),
        create_batch_expectation(burst_size * 2, &setup, 5.0),
        create_batch_expectation(burst_size, &setup, 5.0),
    ];

    let controlled_expectations = vec![create_controlled_rate_expectation(
        controlled_requests,
        delay_ms,
        &setup,
    )];

    // Define test scenarios
    let scenarios = vec![
        TestScenario::new("Burst Testing", burst_batches, burst_expectations),
        TestScenario::new(
            "Controlled Rate Testing",
            controlled_batches,
            controlled_expectations,
        ),
    ];

    // Run all scenarios
    for scenario in scenarios {
        // Wait a bit between scenarios to ensure clean state
        sleep(Duration::from_secs(3)).await;

        let results = run_scenario(&setup, &scenario, &payload).await;
        validate_scenario_results(&scenario, &results);

        println!("✓ Scenario '{}' completed successfully\n", scenario.name);
    }

    setup.shutdown().await;
}

async fn run_get_scenario(
    _setup: &TestSetup,
    scenario: &TestScenario,
    url: &str,
) -> Vec<BatchResult> {
    let client = reqwest::Client::new();
    let mut batch_results = Vec::new();

    println!("Running GET scenario: {}", scenario.name);

    for (batch_idx, batch) in scenario.batches.iter().enumerate() {
        println!(
            "  Batch {}: {} requests, {}ms delay between, {}ms delay after",
            batch_idx + 1,
            batch.request_count,
            batch.delay_between_requests_ms,
            batch.delay_after_batch_ms
        );

        let result = execute_get_batch(&client, url, batch).await;

        println!(
            "    Results: {} successful, {} rate limited ({} with valid Retry-After), {} errors",
            result.successful_requests,
            result.rate_limit_post_endpoints_requests,
            result.retry_after_header_count,
            result.error_requests
        );

        batch_results.push(result);

        // Add delay after batch if specified and not the last batch
        if batch.delay_after_batch_ms > 0 && batch_idx < scenario.batches.len() - 1 {
            sleep(Duration::from_millis(batch.delay_after_batch_ms)).await;
        }
    }

    batch_results
}

#[tokio::test]
async fn test_get_endpoints_burst_no_rate_limiting() {
    let setup = TestSetup::new().await.expect("Failed to create test setup");

    // Define a burst scenario with 100 concurrent requests (no delays)
    let burst_batch = Batch::new(100, 0, 0);
    let burst_expectation = BatchExpectation {
        min_successful: 98, // Allow for 2% tolerance
        max_successful: 100,
        min_rate_limit_post_endpoints: 0,
        max_rate_limit_post_endpoints: 0, // GET endpoints should NEVER be rate limited
        max_errors: 2,
    };

    let burst_scenario = TestScenario::new(
        "GET Endpoint Burst Test",
        vec![burst_batch],
        vec![burst_expectation],
    );

    let endpoints = vec![
        (
            "liveness",
            format!("http://localhost:{}/liveness", setup.http_port),
        ),
        (
            "version",
            format!("http://localhost:{}/version", setup.http_port),
        ),
        (
            "keyurl v1",
            format!("http://localhost:{}/v1/keyurl", setup.http_port),
        ),
        (
            "keyurl v2",
            format!("http://localhost:{}/v2/keyurl", setup.http_port),
        ),
    ];

    for (endpoint_name, url) in endpoints {
        println!("Testing {} endpoint", endpoint_name);

        let results = run_get_scenario(&setup, &burst_scenario, &url).await;
        validate_scenario_results(&burst_scenario, &results);

        println!("✓ {} endpoint burst test passed\n", endpoint_name);
    }

    setup.shutdown().await;
}
