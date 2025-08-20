use crate::{FhevmError, Result};
use kms_grpc::kms::v1::{UserDecryptionResponse, UserDecryptionResponsePayload};
use serde::Deserialize;
use tracing::debug;

/// Hex representation from JSON
#[derive(Debug, Deserialize)]
struct UserDecryptionResponseHex {
    pub signature: String,
    pub payload: Option<String>,
}

/// Container for the JSON response
#[derive(Debug, Deserialize)]
struct JsonResponse {
    pub response: Vec<UserDecryptionResponseHex>,
}

/// Converter for user decryption responses
pub struct UserDecryptionDeserializer;

impl UserDecryptionDeserializer {
    /// Convert JSON string to Vec<UserDecryptionResponse>
    ///
    /// This function handles the deserialization flow:
    /// 1. Parse JSON to hex intermediate type
    /// 2. Decode hex strings to bytes
    /// 3. Deserialize payload bytes using bincode
    ///
    /// # Arguments
    ///
    /// * `json_str` - JSON string from relayer/gateway
    ///
    /// # Returns
    ///
    /// Vec<UserDecryptionResponse> ready for KMS processing
    pub fn json_to_responses(json_str: &str) -> Result<Vec<UserDecryptionResponse>> {
        debug!("Parsing user decryption JSON response");

        // Step 1: Parse JSON to hex intermediate type
        let hex_responses: JsonResponse = serde_json::from_str(json_str)
            .map_err(|e| FhevmError::DecryptionError(format!("JSON parse error: {e}")))?;

        debug!(
            "Found {} responses to process",
            hex_responses.response.len()
        );

        // Step 2: Convert hex responses to final type
        let mut responses = Vec::new();

        for (i, hex_resp) in hex_responses.response.iter().enumerate() {
            debug!("Processing response {}", i);

            // Decode external signature
            let external_signature = decode_hex_field(&hex_resp.signature, "signature")?;
            debug!("Decoded signature: {} bytes", external_signature.len());

            // Decode and deserialize payload
            let payload = if let Some(hex_payload) = &hex_resp.payload {
                let payload_bytes = decode_hex_field(hex_payload, "payload")?;
                debug!("Decoded payload: {} bytes", payload_bytes.len());

                if !payload_bytes.is_empty() {
                    Some(deserialize_payload(&payload_bytes)?)
                } else {
                    debug!("Empty payload bytes, skipping deserialization");
                    None
                }
            } else {
                debug!("No payload present in response {}", i);
                None
            };

            responses.push(UserDecryptionResponse {
                signature: vec![], // No ECDSA signature in wasm use case
                external_signature,
                payload,
                extra_data: vec![], // Extra data not used for now
            });
        }

        debug!(
            "Successfully parsed {} user decryption responses",
            responses.len()
        );
        Ok(responses)
    }
}

/// Decode hex string with 0x prefix handling
fn decode_hex_field(hex_str: &str, field_name: &str) -> Result<Vec<u8>> {
    let cleaned = hex_str.trim_start_matches("0x");

    hex::decode(cleaned)
        .map_err(|e| FhevmError::DecryptionError(format!("Invalid {field_name} hex: {e}")))
}

/// Deserialize payload bytes to UserDecryptionResponsePayload
fn deserialize_payload(buf: &[u8]) -> Result<UserDecryptionResponsePayload> {
    debug!("Deserializing payload of {} bytes", buf.len());

    bc2wrap::deserialize(buf)
        .map_err(|e| FhevmError::DecryptionError(format!("Bincode deserialize error: {e}")))
}

#[cfg(test)]
mod tests {
    use super::*;

    fn create_test_json_response() -> String {
        r#"{
            "response": [
                {
                    "payload": "29000000000000002100000000000000029395c8ff9ca2d768dd40bf9fb6dfc834874487da26218ee57a929228b807ff2b20000000000000002a92056afa790a38b17237730d08ef686c04a2e0dac55aec05b97f26c79a95a50100000000000000020000001501000000000000c5000000000000003b62b10c9abb1f9c4caef03543917fa093758c0b6eb22444293172d287415966f72a4bb1c352aacf7c0003652653aefedb05871dbf068643e8f57baa56a631b579ea0d062921c178e9a73ca341d8a983687a84cd1690af7f4679642a5e3209f8d902c9fcde4a18d8c2dc5bd06d30cdae4ae26c838d35199db8497d454fa4dfc6ec43315254b901d4262fb07f0a039b9523ea0aa658ea583ed29fe54ce22d9fa361502be74746c993e814e6685e7ba723cfcd7b590fa394efbd9068156dfc17d9d3c8c5fa7f1800000000000000717abaaaeb83db7e49cac2168789d3184de51040f7205936200000000000000031604bdf7defdf92477633d530e37899aa12b94dcf132fb6d717aad48b8b625d2000000000000000f2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200010000000100000000000000",
                    "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }"#.to_string()
    }

    #[test]
    fn test_json_to_responses() {
        let json_response = create_test_json_response();
        let responses = UserDecryptionDeserializer::json_to_responses(&json_response).unwrap();

        assert_eq!(responses.len(), 1);
        assert_eq!(responses[0].signature.len(), 0); // Empty signature (matching JS)
        assert_eq!(responses[0].external_signature.len(), 65); // 130 hex chars = 65 bytes
        assert!(responses[0].payload.is_some());

        if let Some(payload) = &responses[0].payload {
            println!("✅ Payload deserialized successfully!");
            println!(
                "   Verification key: {} bytes",
                payload.verification_key.len()
            );
            println!("   Digest: {} bytes", payload.digest.len());
            println!("   Party ID: {}", payload.party_id);
            println!("   Degree: {}", payload.degree);
            println!(
                "   Signcrypted ciphertexts: {}",
                payload.signcrypted_ciphertexts.len()
            );
        }
    }

    #[test]
    fn test_with_0x_prefix() {
        let json_response = r#"{
            "response": [
                {
                    "payload": "0x29000000000000002100000000000000029395c8ff9ca2d768dd40bf9fb6dfc834874487da26218ee57a929228b807ff2b20000000000000002a92056afa790a38b17237730d08ef686c04a2e0dac55aec05b97f26c79a95a50100000000000000020000001501000000000000c5000000000000003b62b10c9abb1f9c4caef03543917fa093758c0b6eb22444293172d287415966f72a4bb1c352aacf7c0003652653aefedb05871dbf068643e8f57baa56a631b579ea0d062921c178e9a73ca341d8a983687a84cd1690af7f4679642a5e3209f8d902c9fcde4a18d8c2dc5bd06d30cdae4ae26c838d35199db8497d454fa4dfc6ec43315254b901d4262fb07f0a039b9523ea0aa658ea583ed29fe54ce22d9fa361502be74746c993e814e6685e7ba723cfcd7b590fa394efbd9068156dfc17d9d3c8c5fa7f1800000000000000717abaaaeb83db7e49cac2168789d3184de51040f7205936200000000000000031604bdf7defdf92477633d530e37899aa12b94dcf132fb6d717aad48b8b625d2000000000000000f2eac20e8f2385a14094f424c3adb8ee0a713bfcbbff00000000000030390200010000000100000000000000",
                    "signature": "0x70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }"#;

        let responses = UserDecryptionDeserializer::json_to_responses(json_response).unwrap();
        assert_eq!(responses.len(), 1);
        assert!(responses[0].payload.is_some());
    }

    #[test]
    fn test_empty_payload() {
        let json_response = r#"{
            "response": [
                {
                    "payload": "",
                    "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }"#;

        let responses = UserDecryptionDeserializer::json_to_responses(json_response).unwrap();
        assert_eq!(responses.len(), 1);
        assert!(responses[0].payload.is_none()); // Should be None for empty payload
    }

    #[test]
    fn test_no_payload_field() {
        let json_response = r#"{
            "response": [
                {
                    "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }"#;

        let responses = UserDecryptionDeserializer::json_to_responses(json_response).unwrap();
        assert_eq!(responses.len(), 1);
        assert!(responses[0].payload.is_none());
    }

    #[test]
    fn test_invalid_json() {
        let invalid_json = r#"{ "response": [ invalid }"#;
        let result = UserDecryptionDeserializer::json_to_responses(invalid_json);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("JSON parse error"));
    }

    #[test]
    fn test_invalid_hex() {
        let json_response = r#"{
            "response": [
                {
                    "payload": "invalid_hex",
                    "signature": "70ec9d960d08632518ba9591f028edeb3c55345c35f0b383596a13e8a7d773582af5f1f2c1897b73d05333d39ab8c9d5bef64e7c14bc636d4a176c30ba3842ee1b"
                }
            ]
        }"#;

        let result = UserDecryptionDeserializer::json_to_responses(json_response);
        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("Invalid payload hex")
        );
    }

    #[test]
    fn test_decode_hex_field() {
        // Test without 0x prefix
        let result = decode_hex_field("abcd", "test").unwrap();
        assert_eq!(result, vec![0xab, 0xcd]);

        // Test with 0x prefix
        let result = decode_hex_field("0xabcd", "test").unwrap();
        assert_eq!(result, vec![0xab, 0xcd]);

        // Test invalid hex
        let result = decode_hex_field("xyz", "test");
        assert!(result.is_err());
    }
}
