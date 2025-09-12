mod helpers;
mod utils;

#[cfg(test)]
mod tests {
    use crate::helpers::fhevm::{self, FhevmMockSetup};
    use crate::utils::TestSetup;
    use alloy::primitives::{Address, Bytes};
    use serde_json::json;
    use std::{str::FromStr, time::Duration};

    #[tokio::test]
    async fn test_input_url_endpoint_on_chain_rejection() {
        // Create isolated test setup with own ports and database
        let setup = TestSetup::new().await.expect("Failed to create test setup");

        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();
        let ciphertext_data = alloy::primitives::Bytes::from_str("0xaaaaaaaaaaaa").unwrap();

        // Configure FHEVM mock for this specific test
        setup
            .fhevm_mock
            .setup_for_input_proof_success_reject(user_address, ciphertext_data);

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
            test_user_single_request(&setup, fhevm::random_payload_for_user_decrypt()),
            test_user_sequential_requests(&setup, fhevm::random_payload_for_user_decrypt()),
        );
    }

    async fn test_user_single_request(setup: &TestSetup, payload: serde_json::Value) {
        let handles = fhevm::extract_ciphertext_handles_from_user_payload(&payload);
        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();

        setup
            .fhevm_mock
            .setup_for_user_decrypt(user_address, handles);

        let client = reqwest::Client::new();
        let (res, _response_time) = fhevm::post_user_decrypt(
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
        let handles = fhevm::extract_ciphertext_handles_from_user_payload(&payload);
        let user_address = Address::from_str("0xa5e1defb98EFe38EBb2D958CEe052410247F4c80").unwrap();

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);

        setup
            .fhevm_mock
            .setup_for_user_decrypt(user_address, handles.clone());
        let (res, response_time) = fhevm::post_user_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First user decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            setup
                .fhevm_mock
                .setup_for_user_decrypt(user_address, handles.clone());
            let (res, response_time) =
                fhevm::post_user_decrypt(&client, &base_url, &payload, 1).await;
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
        let payload_1 = fhevm::random_payload_for_public_decrypt();
        let payload_2 = fhevm::random_payload_for_public_decrypt();
        let payload_3 = fhevm::random_payload_for_public_decrypt();

        tokio::join!(
            test_single_request(&setup, payload_1),
            test_sequential_requests(&setup, payload_2),
            test_parallel_requests(&setup, payload_3),
        );
    }

    async fn test_single_request(setup: &TestSetup, payload: serde_json::Value) {
        let handles = fhevm::extract_ciphertext_handles_from_public_payload(&payload);
        setup.fhevm_mock.setup_for_public_decrypt(handles);

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);
        let (res, response_time) =
            fhevm::post_public_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("Simple public decrypt request took: {:?}", response_time);
    }

    async fn test_sequential_requests(setup: &TestSetup, payload: serde_json::Value) {
        let handles = fhevm::extract_ciphertext_handles_from_public_payload(&payload);
        setup.fhevm_mock.setup_for_public_decrypt(handles.clone());

        let client = reqwest::Client::new();
        let base_url = format!("http://localhost:{}", setup.http_port);
        let (res, response_time) =
            fhevm::post_public_decrypt(&client, &base_url, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First public decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            setup.fhevm_mock.setup_for_public_decrypt(handles.clone());
            let (res, elapsed) = fhevm::post_public_decrypt(&client, &base_url, &payload, 1).await;
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
            let handles_clone = fhevm::extract_ciphertext_handles_from_public_payload(&payload);
            let fhevm_mock_clone = setup.fhevm_mock.clone();
            tokio::spawn(async move {
                fhevm_mock_clone.setup_for_public_decrypt(handles_clone);
                let client = reqwest::Client::new();
                let (res, response_time) =
                    fhevm::post_public_decrypt(&client, &url_clone, &payload_clone, 10).await;
                assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
                println!(
                    "Parallel public decrypt request 1 (payload) took: {:?}",
                    response_time
                );
                response_time
            })
        };

        tokio::time::sleep(Duration::from_micros(100)).await;

        // Send the remaining requests in parallel
        let mut remaining_set = tokio::task::JoinSet::new();
        for i in 1..number_of_queries {
            let payload_clone = payload.clone();
            let url_clone = base_url.clone();
            let handles_clone = fhevm::extract_ciphertext_handles_from_public_payload(&payload);
            let fhevm_mock_clone = setup.fhevm_mock.clone();
            remaining_set.spawn(async move {
                fhevm_mock_clone.setup_for_public_decrypt(handles_clone);
                let client = reqwest::Client::new();
                let (res, response_time) =
                    fhevm::post_public_decrypt(&client, &url_clone, &payload_clone, 10).await;
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
}
