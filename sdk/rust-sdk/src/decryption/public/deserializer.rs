use super::types::DecryptedResults;
use crate::utils::parse_hex_string;
use crate::{FhevmError, Result};
use alloy::primitives::U256;

// Constants for ABI decoding
const DUMMY_REQUEST_ID_SIZE: usize = 32;
const WORD_SIZE: usize = 32;
const ADDRESS_BYTE_OFFSET: usize = 12;
const BOOL_VALUE_OFFSET: usize = 31;

// Type discriminants mapping
const TYPE_BOOL: u8 = 0;
const TYPE_UINT8: u8 = 2;
const TYPE_UINT16: u8 = 3;
const TYPE_UINT32: u8 = 4;
const TYPE_UINT64: u8 = 5;
const TYPE_UINT128: u8 = 6;
const TYPE_ADDRESS: u8 = 7;
const TYPE_UINT256: u8 = 8;
const TYPE_BYTES64: u8 = 9;
const TYPE_BYTES128: u8 = 10;
const TYPE_BYTES256: u8 = 11;

/// Deserialize decrypted result using ABI decoding
pub fn deserialize_decrypted_result(
    handles: &[String],
    decrypted_result: &str,
) -> Result<DecryptedResults> {
    // Extract types from handles
    let types_list = extract_types_from_handles(handles)?;

    // Decode the hex string
    let decrypted_bytes = parse_hex_string(decrypted_result, "decrypted result")?;

    // Check minimum size based on number of handles
    // Each value needs 32 bytes in the decrypted result
    let required_bytes = handles.len() * WORD_SIZE;
    if decrypted_bytes.len() < required_bytes {
        return Err(FhevmError::DecryptionError(format!(
            "Insufficient decrypted data: need at least {} bytes for {} values, got {}",
            required_bytes,
            handles.len(),
            decrypted_bytes.len()
        )));
    }

    // Create the restored encoded data
    let restored_encoded = create_restored_encoded(&decrypted_bytes);

    // Decode based on number of handles
    if handles.len() == 1 {
        decode_single_value(handles, &types_list, &restored_encoded)
    } else {
        decode_multiple_values(handles, &types_list, &restored_encoded)
    }
}

fn extract_types_from_handles(handles: &[String]) -> Result<Vec<u8>> {
    handles
        .iter()
        .map(|handle| extract_type_from_handle(handle))
        .collect()
}

fn extract_type_from_handle(handle: &str) -> Result<u8> {
    // Remove 0x prefix if present
    let handle = if handle.starts_with("0x") || handle.starts_with("0X") {
        &handle[2..]
    } else {
        handle
    };

    // Check minimum length (at least 4 hex chars needed)
    if handle.len() < 4 {
        return Err(FhevmError::DecryptionError(format!(
            "Handle too short to extract type (need at least 4 hex chars, got {})",
            handle.len()
        )));
    }

    // Get the type discriminant from the handle (3rd and 4th last hex chars)
    let hex_pair = &handle[handle.len() - 4..handle.len() - 2];
    u8::from_str_radix(hex_pair, 16).map_err(|e| {
        FhevmError::DecryptionError(format!("Invalid handle type hex '{}': {}", hex_pair, e))
    })
}

fn create_restored_encoded(decrypted_bytes: &[u8]) -> Vec<u8> {
    let mut restored_encoded =
        Vec::with_capacity(DUMMY_REQUEST_ID_SIZE + decrypted_bytes.len() + DUMMY_REQUEST_ID_SIZE);

    restored_encoded.extend_from_slice(&[0u8; DUMMY_REQUEST_ID_SIZE]); // dummy requestID
    restored_encoded.extend_from_slice(decrypted_bytes);
    restored_encoded.extend_from_slice(&[0u8; DUMMY_REQUEST_ID_SIZE]); // dummy empty bytes[] length

    restored_encoded
}

fn decode_single_value(
    handles: &[String],
    types_list: &[u8],
    restored_encoded: &[u8],
) -> Result<DecryptedResults> {
    let mut results = DecryptedResults::new();

    // Check if we have enough data (dummy request ID + at least one word)
    let required_size = DUMMY_REQUEST_ID_SIZE + WORD_SIZE;
    if restored_encoded.len() < required_size {
        return Err(FhevmError::DecryptionError(format!(
            "Insufficient data for single value decoding: need {} bytes, got {}",
            required_size,
            restored_encoded.len()
        )));
    }

    let handle = &handles[0];
    let type_disc = types_list[0];
    let value_bytes = &restored_encoded[DUMMY_REQUEST_ID_SIZE..DUMMY_REQUEST_ID_SIZE + WORD_SIZE];

    let value = decode_value_by_type(type_disc, value_bytes)?;
    results.insert(handle.clone(), value);

    Ok(results)
}

fn decode_multiple_values(
    handles: &[String],
    types_list: &[u8],
    restored_encoded: &[u8],
) -> Result<DecryptedResults> {
    let mut results = DecryptedResults::new();
    let mut offset = DUMMY_REQUEST_ID_SIZE; // Skip dummy requestID

    for (i, handle) in handles.iter().enumerate() {
        if offset + WORD_SIZE > restored_encoded.len() {
            return Err(FhevmError::DecryptionError(format!(
                "Not enough data for handle {} (need {} bytes, have {})",
                i,
                offset + WORD_SIZE,
                restored_encoded.len()
            )));
        }

        let type_disc = types_list[i];
        let value_bytes = &restored_encoded[offset..offset + WORD_SIZE];

        let value = decode_value_by_type(type_disc, value_bytes)?;
        results.insert(handle.clone(), value);

        offset += WORD_SIZE;
    }

    Ok(results)
}

fn decode_value_by_type(type_disc: u8, value_bytes: &[u8]) -> Result<serde_json::Value> {
    match type_disc {
        TYPE_BOOL => decode_bool(value_bytes),
        TYPE_UINT8 | TYPE_UINT16 | TYPE_UINT32 | TYPE_UINT64 | TYPE_UINT128 | TYPE_UINT256 => {
            decode_numeric(value_bytes)
        }
        TYPE_ADDRESS => decode_address(value_bytes),
        TYPE_BYTES64 | TYPE_BYTES128 | TYPE_BYTES256 => decode_bytes(value_bytes),
        _ => Err(FhevmError::DecryptionError(format!(
            "Unknown type discriminant: {}",
            type_disc
        ))),
    }
}

fn decode_bool(value_bytes: &[u8]) -> Result<serde_json::Value> {
    if value_bytes.len() != WORD_SIZE {
        return Err(FhevmError::DecryptionError(
            "Invalid value bytes length for bool".to_string(),
        ));
    }

    let is_true = value_bytes[BOOL_VALUE_OFFSET] == 1;
    Ok(serde_json::json!(is_true))
}

fn decode_numeric(value_bytes: &[u8]) -> Result<serde_json::Value> {
    let value = U256::from_be_slice(value_bytes);
    Ok(serde_json::json!(value.to_string()))
}

fn decode_address(value_bytes: &[u8]) -> Result<serde_json::Value> {
    if value_bytes.len() != WORD_SIZE {
        return Err(FhevmError::DecryptionError(
            "Invalid value bytes length for address".to_string(),
        ));
    }

    // Address is the last 20 bytes of the 32-byte slot
    let addr_bytes = &value_bytes[ADDRESS_BYTE_OFFSET..WORD_SIZE];
    let addr = format!("0x{}", hex::encode(addr_bytes));
    Ok(serde_json::json!(addr))
}

fn decode_bytes(value_bytes: &[u8]) -> Result<serde_json::Value> {
    // For static layout, just return hex of the slot
    Ok(serde_json::json!(format!("0x{}", hex::encode(value_bytes))))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_type_from_handle() {
        // Test handle ending with type 04 (uint32) - with 0x prefix
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, TYPE_UINT32);

        // Test handle ending with type 00 (bool) - without 0x prefix
        let handle = "f94fd2cead277005511f811497a185db1b81598f2aff00000000000030390000";
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, TYPE_BOOL);

        // Test handle ending with type 07 (address) - with 0X prefix (uppercase)
        let handle = "0Xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390700";
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, TYPE_ADDRESS);
    }

    #[test]
    fn test_extract_type_with_various_formats() {
        // Test minimum valid handle (4 hex chars) without prefix
        let handle = "0400"; // type 04
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, TYPE_UINT32);

        // Test minimum valid handle with prefix
        let handle = "0x0500"; // type 05
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, TYPE_UINT64);

        // Test edge case: exactly 4 chars with 0x prefix (should fail)
        let handle = "0x04";
        let result = extract_type_from_handle(handle);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }

    #[test]
    fn test_decode_bool() {
        // Test true
        let mut value_bytes = [0u8; 32];
        value_bytes[31] = 1;
        let result = decode_bool(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!(true));

        // Test false
        let value_bytes = [0u8; 32];
        let result = decode_bool(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!(false));
    }

    #[test]
    fn test_decode_numeric() {
        // Test value 242
        let mut value_bytes = [0u8; 32];
        value_bytes[31] = 242;
        let result = decode_numeric(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!("242"));

        // Test larger value
        let mut value_bytes = [0u8; 32];
        value_bytes[30] = 1;
        value_bytes[31] = 0;
        let result = decode_numeric(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!("256"));
    }

    #[test]
    fn test_decode_address() {
        let mut value_bytes = [0u8; 32];
        // Set address bytes (last 20 bytes)
        for i in 12..32 {
            value_bytes[i] = 0xab;
        }

        let result = decode_address(&value_bytes).unwrap();
        assert_eq!(
            result,
            serde_json::json!("0xabababababababababababababababababababab")
        );
    }

    #[test]
    fn test_deserialize_single_uint32() {
        let handles =
            vec!["0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()];

        // Decrypted result for value 242
        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060";

        let results = deserialize_decrypted_result(&handles, decrypted_result).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[&handles[0]], serde_json::json!("242"));
    }

    #[test]
    fn test_deserialize_multiple_values() {
        let handles = vec![
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00000".to_string(), // bool
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb00400".to_string(), // uint32
        ];

        // Create test data: true (1) and 100
        let mut decrypted_bytes = Vec::new();

        // Bool value (true)
        let mut bool_bytes = [0u8; 32];
        bool_bytes[31] = 1;
        decrypted_bytes.extend_from_slice(&bool_bytes);

        // Uint32 value (100)
        let mut uint_bytes = [0u8; 32];
        uint_bytes[31] = 100;
        decrypted_bytes.extend_from_slice(&uint_bytes);

        let decrypted_result = format!("0x{}", hex::encode(&decrypted_bytes));

        let results = deserialize_decrypted_result(&handles, &decrypted_result).unwrap();

        assert_eq!(results.len(), 2);
        assert_eq!(results[&handles[0]], serde_json::json!(true));
        assert_eq!(results[&handles[1]], serde_json::json!("100"));
    }

    #[test]
    fn test_handle_too_short_error() {
        let result = extract_type_from_handle("0x123");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }

    #[test]
    fn test_insufficient_data_error() {
        let handles =
            vec!["0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00400".to_string()];

        // The minimum valid decrypted result for 1 value should be 32 bytes
        // But let's provide only 31 bytes (62 hex chars)
        let decrypted_result = "0x".to_owned() + &"00".repeat(31);

        let result = deserialize_decrypted_result(&handles, &decrypted_result);
        assert!(
            result.is_err(),
            "Expected error for insufficient data, but got: {:?}",
            result
        );

        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Insufficient decrypted data"),
                "Error should mention insufficient data, got: {}",
                error_msg
            );
            // Verify it mentions the specific byte counts
            assert!(error_msg.contains("32 bytes") && error_msg.contains("31"));
        }
    }

    #[test]
    fn test_empty_decrypted_result() {
        let handles =
            vec!["0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00400".to_string()];
        let decrypted_result = "0x"; // Empty hex (0 bytes after 0x)

        let result = deserialize_decrypted_result(&handles, decrypted_result);
        assert!(result.is_err(), "Expected error for empty decrypted result");

        if let Err(e) = result {
            let error_msg = e.to_string();
            // Empty "0x" results in 0 bytes, so we get insufficient data error
            assert!(
                error_msg.contains("Insufficient decrypted data") && error_msg.contains("0"),
                "Error should mention 0 bytes, got: {}",
                error_msg
            );
        }
    }

    #[test]
    fn test_invalid_hex_error() {
        let handles =
            vec!["0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00400".to_string()];
        let decrypted_result = "0xGGGG"; // Invalid hex characters

        let result = deserialize_decrypted_result(&handles, decrypted_result);
        assert!(result.is_err(), "Expected error for invalid hex");

        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Invalid hex"),
                "Error should mention invalid hex, got: {}",
                error_msg
            );
        }
    }

    #[test]
    fn test_partial_data_for_multiple_values() {
        let handles = vec![
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00000".to_string(), // bool
            "0xbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb00400".to_string(), // uint32
            "0xcccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc00700".to_string(), // address
        ];

        // For 3 values, we need at least 3 * 32 = 96 bytes
        // Let's provide only 64 bytes (enough for 2 values)
        let decrypted_result = "0x".to_owned() + &"00".repeat(64);

        let result = deserialize_decrypted_result(&handles, &decrypted_result);
        assert!(result.is_err(), "Expected error for partial data");

        if let Err(e) = result {
            let error_msg = e.to_string();
            assert!(
                error_msg.contains("Insufficient decrypted data"),
                "Error should mention insufficient data, got: {}",
                error_msg
            );
            // Verify it mentions needing 96 bytes but got 64
            assert!(error_msg.contains("96 bytes") && error_msg.contains("64"));
        }
    }

    #[test]
    fn test_minimum_valid_data() {
        // Test that exactly 32 bytes works for a single value
        let handles =
            vec!["0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa00400".to_string()];

        // Provide exactly 32 bytes (64 hex chars) - should work
        let decrypted_result = "0x".to_owned() + &"00".repeat(32);

        let result = deserialize_decrypted_result(&handles, &decrypted_result);
        assert!(
            result.is_ok(),
            "32 bytes should be sufficient for single value"
        );

        let values = result.unwrap();
        assert_eq!(values.len(), 1);
        assert_eq!(values[&handles[0]], serde_json::json!("0"));
    }

    #[test]
    fn test_real_world_decrypted_value() {
        // Test with a real-world example from the JS tests
        let handles =
            vec!["0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()];

        // This is the actual format returned by the gateway (without the dummy bytes)
        // Just the value 242 as a uint256
        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000f2";

        let result = deserialize_decrypted_result(&handles, decrypted_result);
        assert!(result.is_ok(), "Should decode real-world value");

        let values = result.unwrap();
        assert_eq!(values[&handles[0]], serde_json::json!("242"));
    }
}
