#[cfg(not(feature = "ci"))]
#[cfg(test)]
mod tests {

    use reqwest;
    use serde_json::json;

    #[tokio::test]
    async fn test_input_url_endpoint() {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/input-proof")
            .header("Content-Type", "application/json")
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
    }

    #[tokio::test]
    async fn test_user_decrypt_url_endpoint() {
        let client = reqwest::Client::new();
        let res = client
            .post("http://localhost:3000/v1/user-decrypt")
            .header("Content-Type", "application/json")
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
            .json(&json!({"ciphertextHandles": ["0x5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff00000000000030390500"]}))
            .send()
            .await
            .unwrap();
        assert_eq!(res.status(), 200);
    }
}
