use super::types::DecryptedResults;
use crate::encryption::primitives::EncryptionType;
use crate::utils::parse_hex_string;
use crate::{ClientCoreError, Result};
use alloy::primitives::U256;

/// Padding to match on-chain ABI encoding layout.
const DUMMY_REQUEST_ID_SIZE: usize = 32;
const WORD_SIZE: usize = 32;
const ADDRESS_BYTE_OFFSET: usize = 12;
const BOOL_VALUE_OFFSET: usize = 31;

/// Deserialize decrypted result using ABI decoding.
pub fn deserialize_decrypted_result(
    handles: &[String],
    decrypted_result: &str,
) -> Result<DecryptedResults> {
    let types_list = extract_types_from_handles(handles)?;
    let decrypted_bytes = parse_hex_string(decrypted_result, "decrypted result")?;

    let required_bytes = handles.len() * WORD_SIZE;
    if decrypted_bytes.len() < required_bytes {
        return Err(ClientCoreError::DecryptionError(format!(
            "Insufficient decrypted data: need at least {} bytes for {} values, got {}",
            required_bytes,
            handles.len(),
            decrypted_bytes.len()
        )));
    }

    let restored_encoded = create_restored_encoded(&decrypted_bytes);

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
    let handle = handle
        .strip_prefix("0x")
        .or_else(|| handle.strip_prefix("0X"))
        .unwrap_or(handle);

    let hex_pair = handle
        .get(handle.len().saturating_sub(4)..handle.len().saturating_sub(2))
        .ok_or_else(|| {
            ClientCoreError::DecryptionError(format!(
                "Handle too short to extract type (need at least 4 hex chars, got {})",
                handle.len()
            ))
        })?;

    if hex_pair.len() != 2 {
        return Err(ClientCoreError::DecryptionError(format!(
            "Handle too short to extract type (need at least 4 hex chars, got {})",
            handle.len()
        )));
    }

    u8::from_str_radix(hex_pair, 16).map_err(|e| {
        ClientCoreError::DecryptionError(format!("Invalid handle type hex '{hex_pair}': {e}"))
    })
}

fn create_restored_encoded(decrypted_bytes: &[u8]) -> Vec<u8> {
    let mut restored_encoded =
        Vec::with_capacity(DUMMY_REQUEST_ID_SIZE + decrypted_bytes.len() + DUMMY_REQUEST_ID_SIZE);

    restored_encoded.extend_from_slice(&[0u8; DUMMY_REQUEST_ID_SIZE]);
    restored_encoded.extend_from_slice(decrypted_bytes);
    restored_encoded.extend_from_slice(&[0u8; DUMMY_REQUEST_ID_SIZE]);

    restored_encoded
}

fn decode_single_value(
    handles: &[String],
    types_list: &[u8],
    restored_encoded: &[u8],
) -> Result<DecryptedResults> {
    let mut results = DecryptedResults::new();

    let required_size = DUMMY_REQUEST_ID_SIZE + WORD_SIZE;
    if restored_encoded.len() < required_size {
        return Err(ClientCoreError::DecryptionError(format!(
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
    let mut offset = DUMMY_REQUEST_ID_SIZE;

    for (i, handle) in handles.iter().enumerate() {
        if offset + WORD_SIZE > restored_encoded.len() {
            return Err(ClientCoreError::DecryptionError(format!(
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
    let enc_type = EncryptionType::from_discriminant(type_disc).map_err(|_| {
        ClientCoreError::DecryptionError(format!("Unknown type discriminant: {type_disc}"))
    })?;

    match enc_type {
        EncryptionType::Bit1 => decode_bool(value_bytes),
        EncryptionType::Bit160 => decode_address(value_bytes),
        _ => decode_numeric(value_bytes, &enc_type),
    }
}

fn decode_bool(value_bytes: &[u8]) -> Result<serde_json::Value> {
    if value_bytes.len() != WORD_SIZE {
        return Err(ClientCoreError::DecryptionError(
            "Invalid value bytes length for bool".to_string(),
        ));
    }

    match value_bytes[BOOL_VALUE_OFFSET] {
        0 => Ok(serde_json::json!(false)),
        1 => Ok(serde_json::json!(true)),
        v => Err(ClientCoreError::DecryptionError(format!(
            "Invalid boolean value: expected 0 or 1, got {v}"
        ))),
    }
}

fn decode_numeric(value_bytes: &[u8], enc_type: &EncryptionType) -> Result<serde_json::Value> {
    let value = U256::from_be_slice(value_bytes);

    let max_value = match enc_type {
        EncryptionType::Bit8 => Some(U256::from(u8::MAX)),
        EncryptionType::Bit16 => Some(U256::from(u16::MAX)),
        EncryptionType::Bit32 => Some(U256::from(u32::MAX)),
        EncryptionType::Bit64 => Some(U256::from(u64::MAX)),
        EncryptionType::Bit128 => Some(U256::from(u128::MAX)),
        _ => None, // Bit256 uses the full range
    };

    if let Some(max) = max_value
        && value > max
    {
        return Err(ClientCoreError::DecryptionError(format!(
            "Value {} exceeds maximum for {:?} (max {})",
            value, enc_type, max
        )));
    }

    Ok(serde_json::json!(value.to_string()))
}

fn decode_address(value_bytes: &[u8]) -> Result<serde_json::Value> {
    if value_bytes.len() != WORD_SIZE {
        return Err(ClientCoreError::DecryptionError(
            "Invalid value bytes length for address".to_string(),
        ));
    }

    let addr_bytes = &value_bytes[ADDRESS_BYTE_OFFSET..WORD_SIZE];
    let addr = format!("0x{}", hex::encode(addr_bytes));
    Ok(serde_json::json!(addr))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_type_from_handle() {
        let handle = "0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400";
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, EncryptionType::Bit32.discriminant());

        let handle = "f94fd2cead277005511f811497a185db1b81598f2aff00000000000030390000";
        let type_disc = extract_type_from_handle(handle).unwrap();
        assert_eq!(type_disc, EncryptionType::Bit1.discriminant());
    }

    #[test]
    fn test_decode_bool() {
        let mut value_bytes = [0u8; 32];
        value_bytes[31] = 1;
        let result = decode_bool(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!(true));

        let value_bytes = [0u8; 32];
        let result = decode_bool(&value_bytes).unwrap();
        assert_eq!(result, serde_json::json!(false));
    }

    #[test]
    fn test_decode_bool_rejects_invalid_values() {
        let mut value_bytes = [0u8; 32];
        value_bytes[31] = 2;
        let result = decode_bool(&value_bytes);
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Invalid boolean"));
    }

    #[test]
    fn test_decode_numeric() {
        let mut value_bytes = [0u8; 32];
        value_bytes[31] = 242;
        let result = decode_numeric(&value_bytes, &EncryptionType::Bit32).unwrap();
        assert_eq!(result, serde_json::json!("242"));
    }

    #[test]
    fn test_deserialize_single_uint32() {
        let handles =
            vec!["0xf94fd2cead277005511f811497a185db1b81598f2aff00000000000030390400".to_string()];

        let decrypted_result = "0x00000000000000000000000000000000000000000000000000000000000000f20000000000000000000000000000000000000000000000000000000000000060";

        let results = deserialize_decrypted_result(&handles, decrypted_result).unwrap();
        assert_eq!(results.len(), 1);
        assert_eq!(results[&handles[0]], serde_json::json!("242"));
    }

    #[test]
    fn test_handle_too_short_error() {
        let result = extract_type_from_handle("0x123");
        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("too short"));
    }
}
