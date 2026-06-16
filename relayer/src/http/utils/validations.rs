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

/// Validates a host-chain account identity that may be either an EVM 0x-hex
/// 20-byte address or a Solana base58 32-byte Ed25519 public key (RFC-021).
///
/// EVM acceptance is byte-identical to `validate_blockchain_address`; a non-EVM
/// string is additionally accepted when it is a canonical Solana base58 address.
/// The `contract_chain_id` (carrying the Solana chain-type high bit) decides how
/// the accepted string is later interpreted in `InputProofRequest::try_from`.
pub fn validate_host_address(address: &str) -> Result<(), ValidationError> {
    if validate_blockchain_address(address).is_ok()
        || crate::http::utils::solana_address::is_solana_address(address)
    {
        return Ok(());
    }
    // Surface the EVM error shape, which is the common case for malformed input.
    validate_blockchain_address(address)
}

/// Validates a list of host-chain account identities (each EVM 0x-hex or RFC-021 Solana base58).
/// An empty list is permitted (RFC-021 Solana user-decryption carries none; the per-mode
/// requirement — EVM needs >=1 — is enforced downstream).
pub fn validate_host_addresses(addresses: &Vec<String>) -> Result<(), ValidationError> {
    for address in addresses {
        validate_host_address(address)?;
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

/// Validates a user-decryption request signature. 130 hex chars for an EVM EIP-712 (65-byte)
/// signature, or 128 hex chars for an RFC-021 Solana ed25519 (64-byte) `signMessage` signature.
/// Both are accepted at the HTTP layer (the chain id selects which is expected); the ed25519 case
/// is verified by the relayer before forwarding, and the EVM case on-chain by the gateway.
pub fn validate_user_decrypt_signature(sig: &str) -> Result<(), ValidationError> {
    // Surface hex-validity first so a malformed-hex signature reports "Invalid hex string"
    // regardless of length — matching the field-level contract every other hex field upholds.
    // A length-first check would mask the hex error for a wrong-length malformed signature.
    validate_no_0x_hex(sig)?;
    if sig.len() != 130 && sig.len() != 128 {
        return Err(ValidationError::new("validation_error").with_message(
            "signature must be 130 (EIP-712) or 128 (Solana ed25519) hex characters".into(),
        ));
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

/// Solana `extraData` version byte (`0x03`): a versioned blob carrying
/// {context_id, ed25519 identity, nonce, allowed ACL domain keys}. The relayer
/// treats it as opaque and forwards it verbatim; the canonical layout and its
/// parsing/verification live in the kms-connector `solana_extra_data` module.
const EXTRA_DATA_SOLANA_VERSION_BYTE: &str = "03";

/// Minimum hex length (including the `0x` prefix) of a Solana `0x03` blob:
/// 1 (version) + 32 (context_id) + 32 (identity) + 32 (nonce) + 4 (domain-key
/// count) = 101 bytes = 202 hex chars + 2 for `0x` = 204. A blob shorter than
/// this header cannot be a well-formed Solana request.
const EXTRA_DATA_SOLANA_MIN_HEX_LEN: usize = 2 + 101 * 2;

/// Validates the extraData field format for decryption requests.
///
/// Accepted formats:
/// - `"0x00"`: Legacy format (version 0)
/// - `"0x01" + 64 hex chars`: Versioned format (version 1: 1 byte version + 32 bytes payload)
/// - `"0x03" + …`: Solana ed25519 user-decrypt blob (version 3). Forwarded
///   opaquely; only the version byte, a minimum header length, and hex
///   well-formedness are checked here — the relayer never parses the tail.
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
        s if s.len() >= EXTRA_DATA_SOLANA_MIN_HEX_LEN
            && s.starts_with("0x")
            && &s[2..4] == EXTRA_DATA_SOLANA_VERSION_BYTE =>
        {
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

        // Validate contract address: EVM 0x-hex or RFC-021 Solana base58 (the chain id decides
        // interpretation downstream).
        validate_host_address(&pair.contract_address)?;
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

/// EVM unified EIP-712 attestation type: the `signature` is an EIP-712
/// signature verified on-chain by the gateway.
pub const V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1: &str = "eip712-unified-user-decrypt-v1";

/// Solana ed25519 attestation type: the `signature` is an ed25519 `signMessage`
/// blob over the Solana signing preimage (see the kms-connector
/// `solana_extra_data` module). The relayer does NOT verify it — it forwards
/// `signature` + the `0x03` `extraData` blob verbatim into the same gateway
/// V2 `userDecryptionRequest` calldata; each KMS party's connector verifies the
/// ed25519 signature off-chain.
pub const V3_ATTESTATION_TYPE_SOLANA_ED25519_V1: &str = "solana-ed25519-user-decrypt-v1";

/// Required `version` value in the EIP-712 payload.
pub const V3_PAYLOAD_VERSION: &str = "2.0";

/// Required `type` value in the EIP-712 payload.
pub const V3_PAYLOAD_TYPE: &str = "user_decryption";

/// v3 envelope: `attestationType` must match a supported scheme. Both the EVM
/// EIP-712 and the Solana ed25519 schemes route to the same gateway V2
/// `userDecryptionRequest` calldata; the relayer only forwards the opaque
/// `signature` + `extraData` and never verifies them.
pub fn validate_v3_attestation_type(value: &str) -> Result<(), ValidationError> {
    if value != V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1
        && value != V3_ATTESTATION_TYPE_SOLANA_ED25519_V1
    {
        return Err(ValidationError::new("validation_error").with_message(
            format!(
                "Unsupported attestationType; expected one of: [{}, {}]",
                V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1, V3_ATTESTATION_TYPE_SOLANA_ED25519_V1
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
mod tests {
    use super::*;

    #[test]
    fn v3_attestation_type_accepts_evm_and_solana() {
        assert!(validate_v3_attestation_type(V3_ATTESTATION_TYPE_EIP712_UNIFIED_V1).is_ok());
        assert!(validate_v3_attestation_type(V3_ATTESTATION_TYPE_SOLANA_ED25519_V1).is_ok());
        assert!(validate_v3_attestation_type("solana-ed25519-user-decrypt-v1").is_ok());
    }

    #[test]
    fn v3_attestation_type_rejects_unknown() {
        assert!(validate_v3_attestation_type("eip712-unified-user-decrypt-v2").is_err());
        assert!(validate_v3_attestation_type("solana-ed25519-user-decrypt-v2").is_err());
        assert!(validate_v3_attestation_type("").is_err());
        assert!(validate_v3_attestation_type("garbage").is_err());
    }

    #[test]
    fn user_decrypt_signature_accepts_evm_130_and_solana_128() {
        assert!(validate_user_decrypt_signature(&"a".repeat(130)).is_ok());
        assert!(validate_user_decrypt_signature(&"a".repeat(128)).is_ok());
    }

    #[test]
    fn user_decrypt_signature_reports_invalid_hex_regardless_of_length() {
        // A malformed-hex signature must surface "Invalid hex string" even when its length is not
        // 130/128 — the hex check runs before the length check. (Regression: a length-first check
        // masked this, so an invalid-hex signature reported only the length message.)
        let malformed = format!("{}g", "a".repeat(126)); // 127 chars, non-hex 'g'
        let err = validate_user_decrypt_signature(&malformed).unwrap_err();
        assert_eq!(
            err.message.as_deref(),
            Some(validation_messages::HEX_INVALID_STRING)
        );
    }

    #[test]
    fn user_decrypt_signature_reports_length_for_valid_hex_wrong_length() {
        // Valid hex, even length, but neither 130 nor 128 -> the length message.
        let err = validate_user_decrypt_signature(&"a".repeat(126)).unwrap_err();
        assert!(
            err.message
                .as_deref()
                .unwrap_or_default()
                .contains("130 (EIP-712) or 128"),
            "expected the 130/128 length message, got {:?}",
            err.message
        );
    }

    /// Builds a minimal well-formed Solana `0x03` extraData hex string with
    /// `domain_key_count` domain keys (matching the kms-connector layout:
    /// version + context_id + identity + nonce + count + keys).
    fn solana_extra_data_hex(domain_key_count: u32) -> String {
        let mut bytes = vec![0x03u8];
        bytes.extend_from_slice(&[0u8; 32]); // context_id
        bytes.extend_from_slice(&[7u8; 32]); // identity
        bytes.extend_from_slice(&[9u8; 32]); // nonce
        bytes.extend_from_slice(&domain_key_count.to_be_bytes());
        bytes.extend(std::iter::repeat_n(0xabu8, domain_key_count as usize * 32));
        format!("0x{}", hex::encode(bytes))
    }

    #[test]
    fn extra_data_accepts_solana_v3_blob() {
        // Header-only (zero domain keys) and with domain keys both pass.
        assert!(validate_extra_data_field_decryption(&solana_extra_data_hex(0)).is_ok());
        assert!(validate_extra_data_field_decryption(&solana_extra_data_hex(2)).is_ok());
    }

    #[test]
    fn extra_data_rejects_truncated_solana_v3_blob() {
        // A 0x03 prefix shorter than the minimum header is rejected.
        let short = format!("0x03{}", "00".repeat(10));
        assert!(validate_extra_data_field_decryption(&short).is_err());
    }

    #[test]
    fn extra_data_rejects_non_hex_solana_v3_blob() {
        // Right length, 0x03 version, but non-hex payload.
        let bad = format!("0x03{}", "zz".repeat(101));
        assert!(validate_extra_data_field_decryption(&bad).is_err());
    }

    #[test]
    fn extra_data_evm_paths_unchanged() {
        assert!(validate_extra_data_field_decryption("0x00").is_ok());
        let v1 = format!("0x01{}", "00".repeat(32));
        assert!(validate_extra_data_field_decryption(&v1).is_ok());
        // An unknown version byte (e.g. 0x02-as-decryption-extraData) still fails.
        let v2 = format!("0x02{}", "00".repeat(32));
        assert!(validate_extra_data_field_decryption(&v2).is_err());
    }
}
