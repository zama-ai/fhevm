mod common;

#[cfg(test)]
mod tests {
    use crate::common::utils::TestSetup;
    use serde_json::json;
    use std::str::FromStr;

    use alloy::primitives::B256;
    use ethereum_rpc_mock::fhevm::FhevmMockWrapper;
    use rand::{rng, Rng};

    #[tokio::test]
    async fn test_public_decrypt_url_endpoint() {
        let setup = TestSetup::new().await.expect("Failed to create test setup");
        let payload_1 = random_payload_for_public_decrypt();
        let payload_2 = random_payload_for_public_decrypt();
        let payload_3 = random_payload_for_public_decrypt();

        tokio::join!(
            test_single_request(&setup, payload_1),
            test_sequential_requests(&setup, payload_2),
            test_parallel_requests(&setup, payload_3),
        );
    }

    async fn test_single_request(setup: &TestSetup, payload: serde_json::Value) {
        let handles = extract_ciphertext_handles_from_public_payload(&payload);
        setup
            .fhevm_mock
            .setup_for_public_decrypt_success_response(handles);

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);
        let (res, response_time) = post_public_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("Simple public decrypt request took: {:?}", response_time);
    }

    async fn test_sequential_requests(setup: &TestSetup, payload: serde_json::Value) {
        let handles = extract_ciphertext_handles_from_public_payload(&payload);
        setup
            .fhevm_mock
            .setup_for_public_decrypt_success_response(handles.clone());

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);
        let (res, response_time) = post_public_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First public decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            setup
                .fhevm_mock
                .setup_for_public_decrypt_success_response(handles.clone());
            let (res, elapsed) = post_public_decrypt(&client, &base_url, &payload, 1).await;
            response_times_micros.push(elapsed.as_micros());
            assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
            println!(
                "Sequential public decrypt request {} took: {:?}",
                i + 1,
                elapsed
            );
        }
    }

    async fn test_parallel_requests(setup: &TestSetup, payload: serde_json::Value) {
        let number_of_queries = 5;
        let mut response_times_micros = Vec::new();
        let base_url = format!("http://localhost:{}", setup.http_port);

        // Send the first request and wait for it to complete
        let first_request = {
            let payload_clone = payload.clone();
            let url_clone = base_url.clone();
            let handles_clone = extract_ciphertext_handles_from_public_payload(&payload);
            let fhevm_mock_clone = setup.fhevm_mock.clone();
            tokio::spawn(async move {
                fhevm_mock_clone.setup_for_public_decrypt_success_response(handles_clone);
                let client = reqwest::Client::new();
                let (res, response_time) =
                    post_public_decrypt(&client, &url_clone, &payload_clone, 10).await;
                assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
                println!(
                    "Parallel public decrypt request 1 (payload) took: {:?}",
                    response_time
                );
                response_time
            })
        };

        // Send the remaining requests in parallel
        let mut remaining_set = tokio::task::JoinSet::new();
        for i in 1..number_of_queries {
            let payload_clone = payload.clone();
            let url_clone = base_url.clone();
            let handles_clone = extract_ciphertext_handles_from_public_payload(&payload);
            let fhevm_mock_clone = setup.fhevm_mock.clone();
            remaining_set.spawn(async move {
                fhevm_mock_clone.setup_for_public_decrypt_success_response(handles_clone);
                let client = reqwest::Client::new();
                let (res, response_time) =
                    post_public_decrypt(&client, &url_clone, &payload_clone, 10).await;
                assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
                println!(
                    "Parallel public decrypt request {} (payload) took: {:?}",
                    i + 1,
                    response_time
                );
                response_time
            });
        }

        // Collect the first request result
        let first_elapsed = first_request.await.unwrap();
        let first_elapsed = first_elapsed.as_micros();

        // Collect the remaining request results
        while let Some(res) = remaining_set.join_next().await {
            let elapsed = res.unwrap();
            response_times_micros.push(elapsed.as_micros());
        }

        response_times_micros.sort();

        // Print for debug
        println!("First timing: {}μs", first_elapsed);
        println!("Following timings (+mock): {:?}μs", response_times_micros);
    }

    /// Extract ciphertext handles from public decrypt payload
    pub fn extract_ciphertext_handles_from_public_payload(
        payload: &serde_json::Value,
    ) -> Vec<B256> {
        payload["ciphertextHandles"]
            .as_array()
            .unwrap_or(&vec![])
            .iter()
            .filter_map(|handle| {
                handle.as_str().and_then(|s| {
                    let cleaned = s.strip_prefix("0x").unwrap_or(s);
                    B256::from_str(cleaned).ok()
                })
            })
            .collect()
    }

    /// Generate a random ciphertext handle
    pub fn random_handle() -> String {
        let mut rng = rng();
        (0..64)
            .map(|_| rng.random_range(0..16))
            .map(|digit| format!("{:x}", digit))
            .collect()
    }

    /// Generate a random payload for public decryption
    pub fn random_payload_for_public_decrypt() -> serde_json::Value {
        let random_handle = random_handle();
        json!({"ciphertextHandles": [random_handle], "extraData": "0x00"})
    }

    /// Make HTTP POST request to public decrypt endpoint
    pub async fn post_public_decrypt(
        client: &reqwest::Client,
        base_url: &str,
        payload: &serde_json::Value,
        timeout_secs: u64,
    ) -> (reqwest::Response, std::time::Duration) {
        let start = tokio::time::Instant::now();
        let res = client
            .post(format!("{}/v1/public-decrypt", base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(timeout_secs))
            .json(payload)
            .send()
            .await
            .unwrap();
        let elapsed = start.elapsed();

        // Print error message if status is not 200 OK
        let status = res.status();
        if status != 200 {
            let error_text = res
                .text()
                .await
                .unwrap_or_else(|_| "Unable to read error response".to_string());
            tracing::info!(
                "POST /v1/public-decrypt failed with status {}: {}",
                status,
                error_text
            );
            panic!(
                "POST /v1/public-decrypt failed with status {}: {}",
                status, error_text
            );
        }

        (res, elapsed)
    }

    /// FHEVM mock setup extensions for isolated testing
    pub trait FhevmMockSetup {
        /// Setup FHEVM mock for successful public decryption
        fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>);
    }

    impl FhevmMockSetup for FhevmMockWrapper {
        fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>) {
            let plaintext_values = vec![42u64; ciphertext_handles.len()];
            self.on_public_decrypt_success(ciphertext_handles, plaintext_values);
        }
    }
}
