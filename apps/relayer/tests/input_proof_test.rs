mod common;

#[cfg(test)]
mod tests {
    use crate::common::utils::TestSetup;
    use serde_json::json;
    use std::str::FromStr;

    use alloy::primitives::{Address, Bytes};
    use ethereum_rpc_mock::fhevm::FhevmMockWrapper;

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

    /// FHEVM mock setup extensions for isolated testing
    pub trait FhevmMockSetup {
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
