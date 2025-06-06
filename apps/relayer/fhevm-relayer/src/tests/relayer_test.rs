// These tests aren't launched in CI because the CI currently doesn't run a relayer
#[cfg(not(feature = "ci"))]
#[cfg(test)]
mod tests {

    use reqwest;
    use serde_json::json;

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
                "ciphertextWithInputVerification": "aaaaaaaaaaaa"
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 500);
        if let Ok(ok_text) = res_text {
            assert_eq!(ok_text, "{\"message\":\"REQUEST FAILED RESPONSE\"}");
        }
    }

    #[tokio::test]
    async fn test_input_url_endpoint() {
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
                "ciphertextWithInputVerification": "abcdef"
            }))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
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
                            "ciphertextWithInputVerification": "abcdef"
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
            let (result, index) = res.unwrap();
            let result = result.unwrap();
            assert_eq!(result.status(), 200);
            println!("{} request is ok", index);
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
                "ciphertextWithInputVerification": ""
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{:?}, {}", res_text, status_code);
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
                "ciphertextWithInputVerification": "abcdef"
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        // TODO: Should fail
        let check_incorrect_chain_id = false;
        if check_incorrect_chain_id {
            assert_eq!(status_code, 400, "{:?}, {}", res_text, status_code);
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
                "ciphertextWithInputVerification": "abcdef"
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{:?}, {}", res_text, status_code);
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
                "ciphertextWithInputVerification": "abcdef"
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{:?}, {}", res_text, status_code);
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
                "ciphertextWithInputVerification": "abcdefabcdefs"
            }))
            .send()
            .await
            .unwrap();

        let status_code = res.status();
        let res_text = res.text().await;
        assert_eq!(status_code, 400, "{:?}, {}", res_text, status_code);
        if let Ok(ok_text) = res_text {
            assert_eq!(
                ok_text,
                "{\"message\":\"Error decoding ciphertextWithInputVerification: Odd number of digits\"}"
            )
        }
    }

    #[tokio::test]
    async fn test_user_decrypt_url_endpoint() {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/user-decrypt")
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(10))
            .json(&json!({"handleContractPairs":[{"handle":"bf9b45c007d626278570aa9622a9c8646f1bfd4e25a5401bd576d15e05320000","contractAddress":"0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"}],"requestValidity":{"startTimestamp":"1742450894","durationDays":"10"},"contractsChainId":"123456","contractAddresses":["0x59AAd6Dc3C909aeED1916937cC310fBfBB118c8C"],"userAddress":"0xa5e1defb98EFe38EBb2D958CEe052410247F4c80","signature":"f77ca89b541ca80645dfa2822a95354142b73d078429083569d9ec97e23868282a11bc8f2addeac311edbb0d6b4e2763ae1f8e69702f2ddb89ff952dded2c2d61c","publicKey":"2000000000000000127eae823019dbba103069c7d2ee53b16de8a29057911dfd8ba82c25abfb071a"}))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
    }

    #[tokio::test]
    async fn test_public_decrypt_url_endpoint() {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/public-decrypt")
            .header("Content-Type", "application/json")
            .timeout(std::time::Duration::from_secs(5))
            .json(&json!({"ciphertextHandles": ["0x5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff00000000000030390500"]}))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
    }
}
