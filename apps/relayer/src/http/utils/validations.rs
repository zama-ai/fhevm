use crate::http::endpoints::common::types::{HandleContractPairJson, RequestValidityJson};
use alloy::primitives::U256;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::str::FromStr;
use std::time::{SystemTime, UNIX_EPOCH};
use validator::ValidationError;

// Generic validation error messages (reusable across fields)
pub mod validation_messages {
    pub const GENERIC_REQUIRED_BUT_MISSING: &str = "Required but missing";

    pub const NUMBER_DECIMAL_OR_HEX: &str = "Must be decimal number or 0x hex string";

    pub const HEX_MUST_START_WITH_0X: &str = "Must start with 0x";
    pub const HEX_MUST_NOT_START_WITH_0X: &str = "Must not start with 0x";
    pub const HEX_INVALID_CHARACTERS: &str = "Contains invalid hex characters";
    pub const HEX_INVALID_STRING: &str = "Invalid hex string";

    // Generic length validation messages
    pub const LENGTH_MUST_BE_42_CHARACTERS: &str = "Must be 42 characters long"; // Keep for backward compatibility
    pub const LENGTH_MUST_BE_64_CHARACTERS: &str = "Must be 64 characters long"; // Keep for backward compatibility
    pub const LENGTH_MUST_BE_130_CHARACTERS: &str = "Must be 130 characters long";

    // Generic collection validation messages
    pub const MUST_NOT_BE_EMPTY: &str = "Must not be empty";

    pub const INVALID_EXTRA_DATA_FORMAT: &str =
        "Must be 0x00 or versioned format: version byte followed by payload (e.g. 0x01 + bytes)";
    pub const TIMESTAMP_MUST_NOT_BE_IN_FUTURE: &str = "Timestamp must not be in the future";
}

pub fn de_string_or_number<'de, D: Deserializer<'de>>(deserializer: D) -> Result<String, D::Error> {
    Ok(match Value::deserialize(deserializer)? {
        Value::String(s) => s,
        Value::Number(num) => format!("{num}"),
        _ => return Err(de::Error::custom("wrong type")),
    })
}

// Custom validation function for a standard Ethereum-style blockchain address.
// It must start with "0x", be 42 characters long, and contain hex characters.
pub fn validate_blockchain_address(address: &str) -> Result<(), ValidationError> {
    if !address.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_START_WITH_0X.into()));
    }
    if address.len() != 42 {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::LENGTH_MUST_BE_42_CHARACTERS.into()));
    }
    // The `hex` crate robustly checks if the string slice (after "0x") is valid hex.
    if hex::decode(&address[2..]).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_CHARACTERS.into()));
    }
    Ok(())
}

pub fn validate_blockchain_addresses(addresses: &Vec<String>) -> Result<(), ValidationError> {
    for address in addresses {
        validate_blockchain_address(address)?;
    }
    Ok(())
}

pub fn validate_no_0x_hex(hex_str: &str) -> Result<(), ValidationError> {
    if hex_str.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_NOT_START_WITH_0X.into()));
    };

    if hex::decode(hex_str).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_STRING.into()));
    }
    Ok(())
}

pub fn validate_0x_hex(hex_str: &str) -> Result<(), ValidationError> {
    if !hex_str.starts_with("0x") {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_MUST_START_WITH_0X.into()));
    };

    if hex::decode(&hex_str[2..]).is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::HEX_INVALID_STRING.into()));
    }
    Ok(())
}

pub fn validate_0x_hexs(hex_strs: &Vec<String>) -> Result<(), ValidationError> {
    for hex_str in hex_strs {
        validate_0x_hex(hex_str)?;
    }
    Ok(())
}

/// Validates the extraData field format for decryption requests.
///
/// Accepted formats:
/// - `"0x00"`: Legacy format (version 0)
/// - `"0x01" + 64 hex chars`: Versioned format (version 1: 1 byte version + 32 bytes payload)
pub fn validate_extra_data_field_decryption(extra_data: &str) -> Result<(), ValidationError> {
    match extra_data {
        "0x00" => Ok(()),
        s if s.len() == 68 && s.starts_with("0x01") => {
            // Version 1: [0x01 | contextId(32B)] = 33 bytes = 66 hex chars + "0x" prefix = 68 chars
            hex::decode(&s[2..]).map_err(|_| {
                ValidationError::new("validation_error")
                    .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into())
            })?;
            Ok(())
        }
        _ => Err(ValidationError::new("validation_error")
            .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into())),
    }
}

/// Validates the extraData field for input proof requests.
///
/// Only accepts `"0x00"` for now. Versioned extraData for input proofs will come in a future release.
pub fn validate_extra_data_field_input_proof(extra_data: &str) -> Result<(), ValidationError> {
    if extra_data != "0x00" {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into()));
    }
    Ok(())
}

pub fn validate_u32_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u32>().is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Value must be a valid u32 number".into()));
    }
    Ok(())
}

pub fn validate_u64_string(value: &str) -> Result<(), ValidationError> {
    if value.parse::<u64>().is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message("Value must be a valid u64 number".into()));
    }
    Ok(())
}

pub fn validate_timestamp(value: &str) -> Result<(), ValidationError> {
    let u256_value = U256::from_str(value).map_err(|_| {
        ValidationError::new("validation_error")
            .with_message("Value must be a valid U256 number".into())
    })?;

    // U256 to u64 conversion is truncating. It's safe for timestamps for the foreseeable future.
    let timestamp = u256_value.to::<u64>();

    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            ValidationError::new("internal_server_error")
                .with_message("System time is before UNIX epoch.".into())
        })?
        .as_secs();

    if timestamp > now {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::TIMESTAMP_MUST_NOT_BE_IN_FUTURE.into()));
    }

    Ok(())
}

pub fn validate_chain_id_string(value: &str) -> Result<(), ValidationError> {
    // Match the logic in parse_chain_id() function
    let result = if let Some(stripped) = value.strip_prefix("0x") {
        // Parse as hex if it starts with 0x
        u64::from_str_radix(stripped, 16)
    } else {
        // Parse as decimal otherwise
        value.parse::<u64>()
    };

    if result.is_err() {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::NUMBER_DECIMAL_OR_HEX.into()));
    }
    Ok(())
}

pub fn validate_handle_contract_pairs(
    pairs: &Vec<HandleContractPairJson>,
) -> Result<(), ValidationError> {
    for pair in pairs {
        validate_0x_hex(&pair.handle)?;

        // Validate handle length
        if pair.handle.len() != 66 {
            return Err(ValidationError::new("validation_error")
                .with_message(validation_messages::LENGTH_MUST_BE_64_CHARACTERS.into()));
        }

        // Validate contract address
        validate_blockchain_address(&pair.contract_address)?;
    }
    Ok(())
}

pub fn validate_request_validity(
    request_validity: &RequestValidityJson,
) -> Result<(), ValidationError> {
    validate_timestamp(&request_validity.start_timestamp)?;
    validate_u32_string(&request_validity.duration_days)?;
    Ok(())
}
