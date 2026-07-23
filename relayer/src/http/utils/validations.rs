use crate::http::endpoints::common::types::{
    HandleContractPairJson, HandleEntryJson, RequestValidityJson, RequestValiditySecondsJson,
};
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
        "Must be 0x00, or a versioned format: 0x01 + 32-byte contextId (0x07-tagged first byte), or 0x02 + 32-byte contextId (0x07-tagged) + 32-byte epochId (0x08-tagged)";
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

/// Validates an empty string or a valid `0x`-prefixed hex string.
pub fn validate_0x_hex_allow_empty(hex_str: &str) -> Result<(), ValidationError> {
    if hex_str.is_empty() {
        return Ok(());
    }
    validate_0x_hex(hex_str)
}

/// OpenAPI schema for the decryption `extraData` request field.
///
/// Single source of truth for the user-facing description: every request type
/// carrying the field references this via `#[schema(schema_with = ...)]`.
/// The accepted formats are specified in [`validate_extra_data_field_decryption`].
#[allow(deprecated)]
pub fn extra_data_decryption_schema() -> utoipa::openapi::schema::Object {
    utoipa::openapi::schema::ObjectBuilder::new()
        .schema_type(utoipa::openapi::schema::SchemaType::Type(
            utoipa::openapi::schema::Type::String,
        ))
        .description(Some(
            "Extra data forwarded verbatim to the gateway contract. Accepts `\"0x00\"`, version `0x01` \
             (`0x01` + 32-byte contextId), or version `0x02` (`0x02` + 32-byte contextId + 32-byte epochId). \
             contextId must be 0x07-tagged and epochId must be 0x08-tagged (first byte of each).",
        ))
        // Deprecated `example` (singular) matches what `#[schema(example = ...)]`
        // emits for every other field; `examples` would render differently in the spec.
        .example(Some("0x00".into()))
        .build()
}

/// Validates the extraData field format for decryption requests.
///
/// Each version has a fixed size; trailing bytes are rejected. New fields are
/// introduced by bumping the version byte, not by appending to an existing one.
/// - `"0x00"`: Legacy format (version 0).
/// - `"0x01" + 64 hex chars`: Version 1 — `[version(1B) | contextId(32B)]` = 33 bytes
///   (66 hex chars + `"0x"` prefix = 68 chars). The contextId must be 0x07-tagged
///   (its first byte must be `0x07`).
/// - `"0x02" + 128 hex chars`: Version 2 — `[version(1B) | contextId(32B) | epochId(32B)]`
///   = 65 bytes (130 hex chars + `"0x"` prefix = 132 chars). The contextId must be
///   0x07-tagged and the epochId must be 0x08-tagged (first byte of each, respectively).
///
/// The contextId and epochId are opaque to the Relayer: they are not interpreted beyond
/// their type tag, and the bytes are propagated verbatim to the Gateway.
pub fn validate_extra_data_field_decryption(extra_data: &str) -> Result<(), ValidationError> {
    const CONTEXT_ID_TAG: u8 = 0x07;
    const EPOCH_ID_TAG: u8 = 0x08;

    match extra_data {
        "0x00" => Ok(()),
        s if s.len() == 68 && s.starts_with("0x01") => {
            // Version 1: [0x01 | contextId(32B)] = 33 bytes = 66 hex chars + "0x" prefix = 68 chars
            let bytes = decode_versioned_extra_data(s)?;
            validate_id_tag(bytes[1], CONTEXT_ID_TAG)
        }
        s if s.len() == 132 && s.starts_with("0x02") => {
            // Version 2: [0x02 | contextId(32B) | epochId(32B)] = 65 bytes
            // = 130 hex chars + "0x" prefix = 132 chars.
            let bytes = decode_versioned_extra_data(s)?;
            validate_id_tag(bytes[1], CONTEXT_ID_TAG)?;
            validate_id_tag(bytes[33], EPOCH_ID_TAG)
        }
        _ => Err(ValidationError::new("validation_error")
            .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into())),
    }
}

/// Ensures the hex payload of a versioned extraData string decodes cleanly.
fn decode_versioned_extra_data(extra_data: &str) -> Result<Vec<u8>, ValidationError> {
    hex::decode(&extra_data[2..]).map_err(|_| {
        ValidationError::new("validation_error")
            .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into())
    })
}

/// Checks that an id's type tag (the first byte of the 32-byte id) matches the expected one.
fn validate_id_tag(tag: u8, expected_tag: u8) -> Result<(), ValidationError> {
    if tag != expected_tag {
        return Err(ValidationError::new("validation_error")
            .with_message(validation_messages::INVALID_EXTRA_DATA_FORMAT.into()));
    }
    Ok(())
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

// ---------------------------------------------------------------------------
// v3 (unified EIP-712 user-decryption) validators
// ---------------------------------------------------------------------------

/// The single attestation-type value supported by the current v3 endpoint.
/// Future signature schemes (e.g. Solana ed25519) plug in by widening this
/// allowlist.
pub const V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1: &str = "eip712-unified-user-decrypt-v1";

/// Required `version` value in the EIP-712 payload.
pub const V3_PAYLOAD_VERSION: &str = "2.0";

/// Required `type` value in the EIP-712 payload.
pub const V3_PAYLOAD_TYPE: &str = "user_decryption";

/// v3 envelope: `attestationType` must match a supported scheme.
pub fn validate_v3_attestation_type(value: &str) -> Result<(), ValidationError> {
    if value != V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1 {
        return Err(ValidationError::new("validation_error").with_message(
            format!(
                "Unsupported attestationType; expected one of: [{}]",
                V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1
            )
            .into(),
        ));
    }
    Ok(())
}

/// v3 payload: `version` must be `"2.0"`.
pub fn validate_v3_version(value: &str) -> Result<(), ValidationError> {
    if value != V3_PAYLOAD_VERSION {
        return Err(ValidationError::new("validation_error").with_message(
            format!("Unsupported version; expected \"{}\"", V3_PAYLOAD_VERSION).into(),
        ));
    }
    Ok(())
}

/// v3 payload: `type` must be `"user_decryption"`.
pub fn validate_v3_payload_type(value: &str) -> Result<(), ValidationError> {
    if value != V3_PAYLOAD_TYPE {
        return Err(ValidationError::new("validation_error").with_message(
            format!("Unsupported payload type; expected \"{}\"", V3_PAYLOAD_TYPE).into(),
        ));
    }
    Ok(())
}

/// Validates each `HandleEntryJson`: the ctHandle is a 0x-prefixed 64-hex
/// string, and both `contractAddress` and `ownerAddress` are valid Ethereum
/// addresses. The list must be non-empty (enforced by `length(min = 1)`).
pub fn validate_handle_entries(entries: &Vec<HandleEntryJson>) -> Result<(), ValidationError> {
    for entry in entries {
        validate_0x_hex(&entry.ct_handle)?;
        if entry.ct_handle.len() != 66 {
            return Err(ValidationError::new("validation_error")
                .with_message(validation_messages::LENGTH_MUST_BE_64_CHARACTERS.into()));
        }
        validate_blockchain_address(&entry.contract_address)?;
        validate_blockchain_address(&entry.owner_address)?;
    }
    Ok(())
}

/// Like `validate_blockchain_addresses` but `allowedContracts` may be
/// empty (permissive mode is part of the unified EIP-712 spec).
pub fn validate_blockchain_addresses_allow_empty(
    addresses: &Vec<String>,
) -> Result<(), ValidationError> {
    for address in addresses {
        validate_blockchain_address(address)?;
    }
    Ok(())
}

/// Validates the v3 request-validity window: `startTimestamp` not in the
/// future, `durationSeconds` parses as a u64, and the resulting window
/// ends in the future (`start_timestamp + duration_seconds > now`).
pub fn validate_request_validity_seconds(
    request_validity: &RequestValiditySecondsJson,
) -> Result<(), ValidationError> {
    validate_timestamp(&request_validity.start_timestamp)?;
    validate_u64_string(&request_validity.duration_seconds)?;

    let start = request_validity
        .start_timestamp
        .parse::<u64>()
        .map_err(|_| {
            ValidationError::new("validation_error")
                .with_message("startTimestamp must be a valid u64 number".into())
        })?;
    let duration = request_validity
        .duration_seconds
        .parse::<u64>()
        .map_err(|_| {
            ValidationError::new("validation_error")
                .with_message("durationSeconds must be a valid u64 number".into())
        })?;
    let now = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map_err(|_| {
            ValidationError::new("internal_server_error")
                .with_message("System time is before UNIX epoch.".into())
        })?
        .as_secs();

    let end = start.saturating_add(duration);
    if end <= now {
        return Err(ValidationError::new("validation_error")
            .with_message("requestValidity window has already expired".into()));
    }

    Ok(())
}

#[cfg(test)]
mod extra_data_tests {
    use super::{validate_extra_data_field_decryption, validate_extra_data_field_input_proof};

    // 32-byte contextId / epochId payloads (64 hex chars each), tagged with their
    // required first byte: 0x07 for contextId, 0x08 for epochId.
    const CONTEXT_ID_HEX: &str = "07000000000000000000000000000000000000000000000000000000000000a1";
    const EPOCH_ID_HEX: &str = "08000000000000000000000000000000000000000000000000000000000000b2";
    // Untagged variants (first byte 0x00) used to test tag rejection.
    const UNTAGGED_CONTEXT_ID_HEX: &str =
        "00000000000000000000000000000000000000000000000000000000000000a1";
    const UNTAGGED_EPOCH_ID_HEX: &str =
        "00000000000000000000000000000000000000000000000000000000000000b2";

    #[test]
    fn accepts_legacy_version_0x00() {
        assert!(validate_extra_data_field_decryption("0x00").is_ok());
    }

    #[test]
    fn accepts_version_0x01_with_context_id() {
        // [0x01 | contextId(32B)] = 33 bytes = 68 chars (backward compatibility).
        let extra_data = format!("0x01{CONTEXT_ID_HEX}");
        assert_eq!(extra_data.len(), 68);
        assert!(validate_extra_data_field_decryption(&extra_data).is_ok());
    }

    #[test]
    fn accepts_version_0x02_with_context_id_and_epoch_id() {
        // [0x02 | contextId(32B) | epochId(32B)] = 65 bytes = 132 chars.
        let extra_data = format!("0x02{CONTEXT_ID_HEX}{EPOCH_ID_HEX}");
        assert_eq!(extra_data.len(), 132);
        assert!(validate_extra_data_field_decryption(&extra_data).is_ok());
    }

    #[test]
    fn rejects_version_0x01_with_untagged_context_id() {
        let extra_data = format!("0x01{UNTAGGED_CONTEXT_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_version_0x02_with_untagged_context_id() {
        let extra_data = format!("0x02{UNTAGGED_CONTEXT_ID_HEX}{EPOCH_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_version_0x02_with_untagged_epoch_id() {
        let extra_data = format!("0x02{CONTEXT_ID_HEX}{UNTAGGED_EPOCH_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_version_0x02_missing_epoch_id() {
        // [0x02 | contextId(32B)] = 33 bytes, shorter than the fixed 65-byte size.
        let extra_data = format!("0x02{CONTEXT_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_version_0x02_with_trailing_bytes() {
        // v0x02 is exactly 65 bytes; any extra bytes past the epochId are rejected.
        let extra_data = format!("0x02{CONTEXT_ID_HEX}{EPOCH_ID_HEX}ff");
        assert_eq!(extra_data.len(), 134);
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_version_0x01_with_trailing_bytes() {
        // v0x01 is exactly 33 bytes; an appended epochId is not a valid v0x01 payload.
        let extra_data = format!("0x01{CONTEXT_ID_HEX}{EPOCH_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_unsupported_version_byte() {
        let extra_data = format!("0x03{CONTEXT_ID_HEX}{EPOCH_ID_HEX}");
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_bare_version_byte() {
        assert!(validate_extra_data_field_decryption("0x01").is_err());
        assert!(validate_extra_data_field_decryption("0x02").is_err());
    }

    #[test]
    fn rejects_non_hex_payload() {
        // Correct length and version prefix, but a non-hex character in the payload.
        let extra_data = format!("0x02{}", "z".repeat(128));
        assert_eq!(extra_data.len(), 132);
        assert!(validate_extra_data_field_decryption(&extra_data).is_err());
    }

    #[test]
    fn rejects_empty_and_garbage() {
        assert!(validate_extra_data_field_decryption("").is_err());
        assert!(validate_extra_data_field_decryption("invalid").is_err());
    }

    #[test]
    fn input_proof_accepts_only_0x00() {
        assert!(validate_extra_data_field_input_proof("0x00").is_ok());
        // Versioned extraData for input proofs is not supported yet.
        let v2 = format!("0x02{CONTEXT_ID_HEX}{EPOCH_ID_HEX}");
        assert!(validate_extra_data_field_input_proof(&v2).is_err());
    }
}
