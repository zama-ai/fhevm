mod common;

#[cfg(test)]
mod tests {
    use crate::common::utils::TestSetup;
    use serde_json::json;
    use std::str::FromStr;

    use alloy::primitives::{Address, Bytes, B256};
    use ethereum_rpc_mock::fhevm::FhevmMockWrapper;
    use rand::{rng, Rng};

    #[tokio::test]
    async fn test_input_url_endpoint_on_chain_rejection() {
        // Create isolated test setup with own ports and database
        let setup = TestSetup::new().await.expect("Failed to create test setup");

        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();
        let ciphertext_data = alloy::primitives::Bytes::from_str("0xaaaaaaaaaaaa").unwrap();

        // Configure FHEVM mock for this specific test
        setup
            .fhevm_mock
            .setup_for_input_proof_reject_response(user_address, ciphertext_data);

        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "http://localhost:{}/v1/input-proof",
                setup.http_port
            ))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "aaaaaaaaaaaa",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Input proof that was supposed to be rejected failed.: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400);
        if let Ok(ok_text) = res_text {
            assert!(
                ok_text.contains("Transaction rejected") && ok_text.contains("Rejected"),
                "Expected rejection error, got: {}",
                ok_text
            );
        }
    }

    #[tokio::test]
    async fn test_input_url_http_endpoint() {
        let setup = TestSetup::new().await.expect("Failed to create test setup");
        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();
        let ciphertext_data = Bytes::from_str("0xabcdef").unwrap();

        setup
            .fhevm_mock
            .setup_for_input_proof_success_response(user_address, ciphertext_data);

        let before_time = tokio::time::Instant::now();
        let client = reqwest::Client::new();
        let res = client
            .post(format!(
                "http://localhost:{}/v1/input-proof",
                setup.http_port
            ))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "abcdef",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Input proof that was supposed to be accepted failed.: {e}"))
            .unwrap();
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        let after_time = tokio::time::Instant::now();
        let single_query_duration = after_time - before_time;
        println!(
            "Took {}s to process 1 input flow.",
            single_query_duration.as_secs()
        );

        // Multiple requests test
        let before_time = tokio::time::Instant::now();
        let mut set = tokio::task::JoinSet::new();
        let number_of_queries = 10;
        let base_url = format!("http://localhost:{}", setup.http_port);

        for i in 1..(number_of_queries + 1) {
            let url = base_url.clone();
            set.spawn(async move {
                let client = reqwest::Client::new();
                (
                    client
                        .post(format!("{}/v1/input-proof", url))
                        .header("Content-Type", "application/json")
                        .timeout(std::time::Duration::from_secs(5))
                        .json(&json!({
                                    "contractChainId": "123456",
                                    "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
                                    "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                                    "ciphertextWithInputVerification": "abcdef",
                        "extraData": "0x00"
                                }))
                        .send()
                        .await,
                    i,
                )
            });
        }

        let send_time = tokio::time::Instant::now();
        println!(
            "Took {}s to send {} input flow requests.",
            number_of_queries,
            (after_time - send_time).as_secs()
        );

        while let Some(res) = set.join_next().await {
            let (result, index) = res
                .map_err(|e| {
                    format!("Error in one of {number_of_queries} input-proof requests: {e}")
                })
                .unwrap();
            let result = result.unwrap();
            assert_eq!(result.status(), 200, "{}", result.text().await.unwrap());
            println!("{index} request is ok");
        }

        let after_time = tokio::time::Instant::now();
        let multi_query_duration = after_time - before_time;
        println!(
            "Took {}s to process {} input flow.",
            number_of_queries,
            multi_query_duration.as_secs()
        );

        // Test cases for various error conditions
        test_input_error_cases(&setup).await;
    }

    async fn test_input_error_cases(setup: &TestSetup) {
        let base_url = format!("http://localhost:{}", setup.http_port);

        // Empty ct-proof
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/v1/input-proof", base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Error submitting request with empty proof: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
        if let Ok(ok_text) = res_text {
            assert_eq!(
                ok_text,
                "{\"message\":\"Input Verification cannot be empty.\"}"
            )
        }

        // Incorrect contract address
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/v1/input-proof", base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xfds",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "abcdef",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Error submitting request with incorrect contract address: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
        if let Ok(ok_text) = res_text {
            assert_eq!(
                ok_text,
                "{\"message\":\"Error parsing contractAddress: OddLength\"}"
            )
        }

        // Incorrect user address
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/v1/input-proof", base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "userAddress": "0xfds",
                "ciphertextWithInputVerification": "abcdef",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Error submitting request with incorrect user address: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
        if let Ok(ok_text) = res_text {
            assert_eq!(
                ok_text,
                "{\"message\":\"Error parsing userAddress: OddLength\"}"
            )
        }

        // Incorrect ct-proof
        let client = reqwest::Client::new();
        let res = client
            .post(format!("{}/v1/input-proof", base_url))
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "123456",
                "contractAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "abcdefabcdefs",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Error submitting request with incorrect proof: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
        if let Ok(ok_text) = res_text {
            assert_eq!(
                ok_text,
                "{\"message\":\"Error decoding ciphertextWithInputVerification: Odd number of digits\"}"
            )
        }
    }

    #[tokio::test]
    async fn test_user_decrypt_url_endpoint() {
        let setup = TestSetup::new().await.expect("Failed to create test setup");

        tokio::join!(
            test_user_single_request(&setup, random_payload_for_user_decrypt()),
            test_user_sequential_requests(&setup, random_payload_for_user_decrypt()),
        );
    }

    async fn test_user_single_request(setup: &TestSetup, payload: serde_json::Value) {
        let handles = extract_ciphertext_handles_from_user_payload(&payload);
        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();

        setup
            .fhevm_mock
            .setup_for_user_decrypt_success_response(user_address, handles);

        let client = reqwest::Client::new();
        let (res, _response_time) = post_user_decrypt(
            &client,
            &format!("http://localhost:{}", setup.http_port),
            &payload,
            10,
        )
        .await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("Single user decrypt request completed.");
    }

    async fn test_user_sequential_requests(setup: &TestSetup, payload: serde_json::Value) {
        let handles = extract_ciphertext_handles_from_user_payload(&payload);
        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);

        setup
            .fhevm_mock
            .setup_for_user_decrypt_success_response(user_address, handles.clone());
        let (res, response_time) = post_user_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First user decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            setup
                .fhevm_mock
                .setup_for_user_decrypt_success_response(user_address, handles.clone());
            let (res, response_time) = post_user_decrypt(&client, &base_url, &payload, 1).await;
            assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
            response_times_micros.push(response_time.as_micros());
            println!(
                "Sequential user decrypt request {} completed in {:?}.",
                i + 1,
                response_time
            );
        }
    }

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

    /// Extract ciphertext handles from user decrypt payload
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

    /// Generate a random payload for user decryption
    pub fn random_payload_for_user_decrypt() -> serde_json::Value {
        let random_handle = random_handle();
        json!({"handleContractPairs":[{"handle":random_handle,"contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a","extraData":"0x00"})
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

    /// Make HTTP POST request to user decrypt endpoint
    pub async fn post_user_decrypt(
        client: &reqwest::Client,
        base_url: &str,
        payload: &serde_json::Value,
        timeout_secs: u64,
    ) -> (reqwest::Response, std::time::Duration) {
        let start = tokio::time::Instant::now();
        let res = client
            .post(format!("{}/v1/user-decrypt", base_url))
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
                "POST /v1/user-decrypt failed with status {}: {}",
                status,
                error_text
            );
            panic!(
                "POST /v1/user-decrypt failed with status {}: {}",
                status, error_text
            );
        }

        (res, elapsed)
    }

    /// FHEVM mock setup extensions for isolated testing
    pub trait FhevmMockSetup {
        /// Setup FHEVM mock for successful public decryption
        fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>);

        /// Setup FHEVM mock for successful user decryption
        fn setup_for_user_decrypt_success_response(
            &self,
            user_address: Address,
            ciphertext_handles: Vec<B256>,
        );

        /// Setup FHEVM mock for successful input proof verification
        fn setup_for_input_proof_success_response(
            &self,
            user_address: Address,
            ciphertext_data: Bytes,
        );

        /// Setup FHEVM mock for input proof rejection
        fn setup_for_input_proof_reject_response(
            &self,
            user_address: Address,
            ciphertext_data: Bytes,
        );
    }

    impl FhevmMockSetup for FhevmMockWrapper {
        fn setup_for_public_decrypt_success_response(&self, ciphertext_handles: Vec<B256>) {
            let plaintext_values = vec![42u64; ciphertext_handles.len()];
            self.on_public_decrypt_success(ciphertext_handles, plaintext_values);
        }

        fn setup_for_user_decrypt_success_response(
            &self,
            user_address: Address,
            ciphertext_handles: Vec<B256>,
        ) {
            let encrypted_bytes = Bytes::from(vec![42u8; 32]);
            self.on_user_decrypt_success(ciphertext_handles, user_address, encrypted_bytes);
        }

        fn setup_for_input_proof_success_response(
            &self,
            user_address: Address,
            ciphertext_data: Bytes,
        ) {
            self.on_input_proof_success(user_address, ciphertext_data);
        }

        fn setup_for_input_proof_reject_response(
            &self,
            user_address: Address,
            ciphertext_data: Bytes,
        ) {
            self.on_input_proof_error(user_address, ciphertext_data);
        }
    }
}
