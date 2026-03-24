/// Serializes a single CT handle (32-byte array) as hex string with 0x prefix.
pub fn serialize_ct_handle_as_hex(handle: &[u8; 32]) -> String {
    format!("0x{}", hex::encode(handle))
}

/// Serializes Vec<[u8; 32]> CT handles as Vec<String> hex strings for serde.
pub fn serialize_ct_handles_as_hex<S>(
    handles: &[[u8; 32]],
    serializer: S,
) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    use serde::Serialize;
    let hex_handles: Vec<String> = handles.iter().map(serialize_ct_handle_as_hex).collect();
    hex_handles.serialize(serializer)
}

/// Deserializes Vec<String> hex strings back to Vec<[u8; 32]> CT handles for serde.
pub fn deserialize_ct_handles_from_hex<'de, D>(deserializer: D) -> Result<Vec<[u8; 32]>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    let hex_handles: Vec<String> = Vec::deserialize(deserializer)?;

    hex_handles
        .into_iter()
        .map(|hex_str| {
            // Remove 0x prefix if present
            let hex_str = hex_str.strip_prefix("0x").unwrap_or(&hex_str);

            // Decode hex to bytes
            let bytes = hex::decode(hex_str)
                .map_err(|e| serde::de::Error::custom(format!("Invalid hex string: {}", e)))?;

            // Convert to [u8; 32]
            if bytes.len() != 32 {
                return Err(serde::de::Error::custom(format!(
                    "Expected 32 bytes, got {}",
                    bytes.len()
                )));
            }

            let mut arr = [0u8; 32];
            arr.copy_from_slice(&bytes);
            Ok(arr)
        })
        .collect()
}

pub fn serialize_vec_as_hex(vec: &Vec<u8>) -> String {
    hex::encode(vec)
}

#[cfg(test)]
mod tests {
    use crate::core::event::PublicDecryptRequest;
    use alloy::primitives::Bytes;

    #[test]
    fn test_ct_handles_hex_serialization() {
        let request = PublicDecryptRequest {
            ct_handles: vec![
                [
                    1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22,
                    23, 24, 25, 26, 27, 28, 29, 30, 31, 32,
                ],
                [
                    255, 254, 253, 252, 251, 250, 249, 248, 247, 246, 245, 244, 243, 242, 241, 240,
                    239, 238, 237, 236, 235, 234, 233, 232, 231, 230, 229, 228, 227, 226, 225, 224,
                ],
            ],
            extra_data: Bytes::from("test"),
        };

        let json = serde_json::to_string(&request).unwrap();
        println!("CT handles JSON: {}", json);

        // Verify it contains hex strings with 0x prefix instead of byte arrays
        assert!(json.contains("0x0102030405060708090a0b0c0d0e0f101112131415161718191a1b1c1d1e1f20"));
        assert!(json.contains("0xfffefdfcfbfaf9f8f7f6f5f4f3f2f1f0efeeedecebeae9e8e7e6e5e4e3e2e1e0"));

        // Verify it doesn't contain raw byte arrays like [1,2,3,...]
        assert!(!json.contains("[1,2,3"));
        assert!(!json.contains("[255,254,253"));
    }
}
