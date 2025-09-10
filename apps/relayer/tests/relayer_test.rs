mod utils;

#[cfg(test)]
mod tests {

    use reqwest;
    use serde_json::json;

    use aws_credential_types::Credentials;
    use fhevm_relayer::{
        gateway_processors_mock::handler::PARTIAL_MOCKED_PROCESSING_TIME,
        sqs::sqs_listener::{send_message_to_sqs_queue, wait_for_response_with_id},
    };

    #[tokio::test]
    async fn test_input_url_sqs_endpoint() {
        let inbound_queue =
            "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/relayer-queue"
                .to_string();
        let outbound_queue = "http://sqs.eu-central-1.localhost.localstack.cloud:4566/000000000000/orchestrator-queue".to_string();
        let aws_access_key_id = "test";
        let aws_secret_access_key = "test";
        let aws_region = "eu-central-1";
        let aws_endpoint_url = "http://sqs.eu-central-1.localhost.localstack.cloud:4566";
        let credentials = Credentials::from_keys(aws_access_key_id, aws_secret_access_key, None);

        let config = aws_config::defaults(aws_config::BehaviorVersion::latest())
            .region(aws_region)
            .endpoint_url(aws_endpoint_url)
            .credentials_provider(credentials)
            .load()
            .await;
        let sqs_client = aws_sdk_sqs::Client::new(&config);

        let request_id = uuid::Uuid::new_v4();
        let message = &json!({
            "payload":{
            "contractChainId": 123456,
            "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
            "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
            "ciphertextWithInputVerification": "abcdef",
            "requestId": request_id.to_string(),
                "extraData": "0x00"
            },
            "type": "relayer:input-registration:input-registration-request",
        });

        // Post message
        match send_message_to_sqs_queue(&sqs_client, &inbound_queue, &message).await {
            Ok(_) => println!("success sending response back to sqs: {outbound_queue}"),
            Err(error) => {
                panic!("Couldn't send request to sqs: {outbound_queue} with error: {error:?}");
            }
        };

        let timeout = tokio::time::Duration::from_secs(6);
        let response = tokio::time::timeout(
            timeout,
            wait_for_response_with_id(&sqs_client, request_id, outbound_queue),
        )
        .await;
        match response {
            Err(_) => {
                panic!("Relayer didn't respond through SQS in less than {timeout:?}");
            }
            Ok(value) => match value {
                Err(error) => {
                    panic!("Relayer didn't respond correctly {error:?}");
                }
                Ok(sub_value) => {
                    matches!(
                        sub_value,
                        fhevm_relayer::sqs::sqs_listener::ResponseJson::InputProofResponse { .. }
                    );
                }
            },
        }
    }

    // TODO: split in multiple tests
    #[tokio::test]
    async fn test_input_url_endpoint_on_chain_rejection() {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
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
            assert_eq!(
                ok_text,
                "{\"message\":\"Transaction rejected: \\\"Rejected\\\"\"}"
            );
        }
    }

    #[tokio::test]
    async fn test_input_url_http_endpoint() {
        let before_time = tokio::time::Instant::now();
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
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
        // NOTE: this duration is a bit biased by the fact that other queries might launched in
        // parallel due to the parallelization of the tests
        let single_query_duration = after_time - before_time;
        println!(
            "Took {}s to process 1 input flow.",
            single_query_duration.as_secs()
        );

        // Re-activate once counter issue is fixed on gateway-contracts
        let before_time = tokio::time::Instant::now();
        let mut set = tokio::task::JoinSet::new();
        let number_of_queries = 10;
        for i in 1..(number_of_queries + 1) {
            set.spawn(async move {
                let client = reqwest::Client::new();
                (
                    client
                        .post("http://localhost:3000/v1/input-proof")
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
        // We add a totally arbitrary 20% margin for concurrent requests
        assert!(single_query_duration.mul_f64(1.2) > multi_query_duration);

        println!(
            "Took {}s to process {} input flow.",
            number_of_queries,
            multi_query_duration.as_secs()
        );

        // Empty ct-proof
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
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

        // Incorrect chain-id
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({
                "contractChainId": "0",
                "contractAddress": "0xcEc0e9723bF28D2A2C867108cC4C3A38a011d4D1",
                "userAddress": "0xa5e1defb98EFe38EBb2D958CEe052410247F4c80",
                "ciphertextWithInputVerification": "abcdef",
                "extraData": "0x00"
            }))
            .send()
            .await
            .map_err(|e| format!("Error submitting request with incorrect chain-id: {e}"))
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        let check_incorrect_chain_id = false;
        if check_incorrect_chain_id {
            assert_eq!(status_code, 400, "{res_text:?}, {status_code}");
            if let Ok(ok_text) = res_text {
                assert_eq!(
                    ok_text,
                    "{\"message\":\"Input Verification cannot be empty.\"}"
                )
            }
        }

        // Incorrect contract address
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
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
            .post("http://localhost:3000/v1/input-proof")
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
            .post("http://localhost:3000/v1/input-proof")
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
        tokio::join!(
            test_user_single_request(helpers::random_payload_for_user_decrypt()),
            test_user_sequential_requests(helpers::random_payload_for_user_decrypt()),
            // TODO: test_user_parallel_requests(helpers::random_payload_for_user_decrypt()),
        );
    }

    async fn test_user_single_request(payload: serde_json::Value) {
        let client = reqwest::Client::new();
        let (res, _response_time) = helpers::post_user_decrypt(&client, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("Single user decrypt request completed.");
    }

    async fn test_user_sequential_requests(payload: serde_json::Value) {
        let client = reqwest::Client::new();
        let (res, response_time) = helpers::post_user_decrypt(&client, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First public decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            let (res, response_time) = helpers::post_user_decrypt(&client, &payload, 1).await;
            assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
            response_times_micros.push(response_time.as_micros());
            println!(
                "Sequential user decrypt request {} completed in {:?}.",
                i + 1,
                response_time
            );
        }
        assert!(
            response_times_micros.iter().all(|&x| x < 1_000_000),
            "All sequential requests should take less than 1 second"
        );
    }

    #[tokio::test]
    async fn test_public_decrypt_url_endpoint() {
        let payload_1 = helpers::random_payload_for_public_decrypt();
        let payload_2 = helpers::random_payload_for_public_decrypt();
        let payload_3 = helpers::random_payload_for_public_decrypt();

        tokio::join!(
            test_single_request(payload_1),
            test_sequential_requests(payload_2),
            test_parallel_requests(payload_3),
        );
    }

    async fn test_single_request(payload: serde_json::Value) {
        let client = reqwest::Client::new();
        let (res, response_time) = helpers::post_public_decrypt(&client, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("Simple public decrypt request took: {:?}", response_time);
    }

    async fn test_sequential_requests(payload: serde_json::Value) {
        let client = reqwest::Client::new();
        let (res, response_time) = helpers::post_public_decrypt(&client, &payload, 10).await;
        assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
        println!("First public decrypt request took: {:?}", response_time);

        let mut response_times_micros = Vec::new();
        for i in 0..3 {
            let (res, elapsed) = helpers::post_public_decrypt(&client, &payload, 1).await;
            response_times_micros.push(elapsed.as_micros());
            assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
            println!(
                "Sequential public decrypt request {} took: {:?}",
                i + 1,
                elapsed
            );
        }
        assert!(
            response_times_micros.iter().all(|x| *x < 1_000_000),
            "All subsequent requests should take less than 1s"
        );
    }

    async fn test_parallel_requests(payload: serde_json::Value) {
        let number_of_queries = 5;
        let mut response_times_micros = Vec::new();

        // Send the first request and wait for it to complete
        let first_request = {
            let payload_clone = payload.clone();
            tokio::spawn(async move {
                let client = reqwest::Client::new();
                let (res, response_time) =
                    helpers::post_public_decrypt(&client, &payload_clone, 10).await;
                assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
                println!(
                    "Parallel public decrypt request 1 (payload) took: {:?}",
                    response_time
                );
                response_time
            })
        };

        tokio::time::sleep(PARTIAL_MOCKED_PROCESSING_TIME).await;

        // Send the remaining requests in parallel
        let mut remaining_set = tokio::task::JoinSet::new();
        for i in 1..number_of_queries {
            remaining_set.spawn({
                let payload_clone = payload.clone();
                async move {
                    let client = reqwest::Client::new();
                    let (res, response_time) =
                        helpers::post_public_decrypt(&client, &payload_clone, 10).await;
                    assert_eq!(res.status(), 200, "{}", res.text().await.unwrap());
                    println!(
                        "Parallel public decrypt request {} (payload) took: {:?}",
                        i + 1,
                        response_time
                    );
                    response_time
                }
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

        // Add PARTIAL_MOCKED_PROCESSING_TIME to each following value
        let response_times_micros: Vec<_> = response_times_micros
            .iter()
            .map(|&x| x + PARTIAL_MOCKED_PROCESSING_TIME.as_micros())
            .collect();

        println!("ONE: {response_times_micros:?}");

        // Assert delta between each following and first is 250ms (250_000 micros)
        for (i, &val) in response_times_micros.iter().enumerate() {
            let delta = (val as i128 - first_elapsed as i128).abs();
            assert!(
                delta < 250_000,
                "Request {}: Delta between (following + mock) and first is not ~250ms, got {}μs",
                i + 2,
                delta
            );
        }

        // Print for debug
        println!("First timing: {}μs", first_elapsed);
        println!("Following timings (+mock): {:?}μs", response_times_micros);
    }

    mod helpers {
        use rand::{rng, Rng};
        use serde_json::json;
        pub fn random_handle() -> String {
            let mut rng = rng();
            (0..64)
                .map(|_| rng.random_range(0..16))
                .map(|digit| format!("{:x}", digit))
                .collect()
        }

        pub fn random_payload_for_public_decrypt() -> serde_json::Value {
            let random_handle = random_handle();
            json!({"ciphertextHandles": [random_handle], "extraData": "0x00"})
        }

        pub fn random_payload_for_user_decrypt() -> serde_json::Value {
            let random_handle = random_handle();
            json!({"handleContractPairs":[{"handle":random_handle,"contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a", "extraData": "0x00"})
        }

        pub async fn post_public_decrypt(
            client: &reqwest::Client,
            payload: &serde_json::Value,
            timeout_secs: u64,
        ) -> (reqwest::Response, std::time::Duration) {
            let start = tokio::time::Instant::now();
            let res = client
                .post("http://localhost:3000/v1/public-decrypt")
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(timeout_secs))
                .json(payload)
                .send()
                .await
                .unwrap();
            let elapsed = start.elapsed();
            (res, elapsed)
        }

        pub async fn post_user_decrypt(
            client: &reqwest::Client,
            payload: &serde_json::Value,
            timeout_secs: u64,
        ) -> (reqwest::Response, std::time::Duration) {
            let start = tokio::time::Instant::now();
            let res = client
                .post("http://localhost:3000/v1/user-decrypt")
                .header("Content-Type", "application/json")
                .timeout(std::time::Duration::from_secs(timeout_secs))
                .json(payload)
                .send()
                .await
                .unwrap();
            let elapsed = start.elapsed();
            (res, elapsed)
        }
    }
}
