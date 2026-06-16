//! Base58 Solana address validation for the relayer (RFC-021 host chains).
//!
//! On EVM hosts an account address is a 20-byte value rendered as a 0x-prefixed
//! hex string (`validate_blockchain_address` in `validations.rs`). RFC-021 widens
//! the canonical host-chain address to 32 bytes so EVM and non-EVM hosts share one
//! width; on the Solana side a host-chain address is a 32-byte Ed25519 public key
//! rendered in Bitcoin/Solana base58 (no `0x` prefix, no `0OIl` characters).
//!
//! This mirrors the shape and semantics of the EVM string validator: it accepts
//! the canonical text encoding and rejects anything that is not exactly 32 bytes
//! once decoded, returning a `validator::ValidationError` with a descriptive
//! message. Decoding is done with a small self-contained base58 decoder so the
//! relayer does not take on a new dependency for one config field.

use validator::ValidationError;

/// Byte width of a Solana Ed25519 public key — the canonical RFC-021 host-chain
/// address width on the Solana side.
pub const SOLANA_ADDRESS_LEN: usize = 32;

/// The Bitcoin/Solana base58 alphabet, in value order (index = digit value).
/// Excludes `0`, `O`, `I`, and `l` to avoid visually ambiguous characters.
const BASE58_ALPHABET: &[u8; 58] = b"123456789ABCDEFGHJKLMNPQRSTUVWXYZabcdefghijkmnopqrstuvwxyz";

pub mod solana_address_messages {
    pub const MUST_NOT_BE_EMPTY: &str = "Solana address must not be empty";
    pub const INVALID_BASE58_CHARACTER: &str =
        "Solana address contains a non-base58 character (allowed: 1-9, A-Z, a-z excluding 0OIl)";
    pub const MUST_DECODE_TO_32_BYTES: &str =
        "Solana address must base58-decode to exactly 32 bytes (Ed25519 public key)";
}

/// Maps a base58 character to its digit value, or `None` if it is not in the
/// alphabet.
fn base58_digit_value(byte: u8) -> Option<u8> {
    BASE58_ALPHABET
        .iter()
        .position(|alphabet_byte| *alphabet_byte == byte)
        .map(|position| position as u8)
}

/// Decodes a base58 string into its raw bytes, preserving leading-zero bytes
/// (each leading `1` in base58 maps to one leading `0x00` byte). Returns the
/// offending character on the first non-base58 byte.
fn base58_decode(input: &str) -> Result<Vec<u8>, u8> {
    // Count the leading '1's: in base58 each one encodes a leading zero byte.
    let leading_ones = input.bytes().take_while(|byte| *byte == b'1').count();

    // big-endian byte accumulator built by repeated "multiply by 58, add digit".
    let mut bytes: Vec<u8> = Vec::with_capacity(input.len());
    for character in input.bytes() {
        let mut carry = base58_digit_value(character).ok_or(character)? as u32;
        for byte in bytes.iter_mut() {
            carry += (*byte as u32) * 58;
            *byte = (carry & 0xff) as u8;
            carry >>= 8;
        }
        while carry > 0 {
            bytes.push((carry & 0xff) as u8);
            carry >>= 8;
        }
    }

    // `bytes` holds the little-endian digits; reverse to big-endian and prepend
    // the leading zero bytes that the leading '1's stand for.
    let mut decoded = vec![0u8; leading_ones];
    decoded.extend(bytes.into_iter().rev());
    Ok(decoded)
}

/// Validates that `address` is a canonical base58-encoded 32-byte Solana
/// public key, mirroring `validate_blockchain_address` for the EVM 0x-hex form.
///
/// Accepts the bare base58 text (no `0x` prefix). Rejects empty input, any
/// non-base58 character, and anything that does not decode to exactly 32 bytes.
pub fn validate_solana_address(address: &str) -> Result<(), ValidationError> {
    if address.is_empty() {
        return Err(ValidationError::new("validation_error")
            .with_message(solana_address_messages::MUST_NOT_BE_EMPTY.into()));
    }

    let decoded = base58_decode(address).map_err(|_| {
        ValidationError::new("validation_error")
            .with_message(solana_address_messages::INVALID_BASE58_CHARACTER.into())
    })?;

    if decoded.len() != SOLANA_ADDRESS_LEN {
        return Err(ValidationError::new("validation_error")
            .with_message(solana_address_messages::MUST_DECODE_TO_32_BYTES.into()));
    }

    Ok(())
}

/// Returns whether `address` is a valid canonical Solana base58 address.
pub fn is_solana_address(address: &str) -> bool {
    validate_solana_address(address).is_ok()
}

/// Decodes a canonical base58 Solana address into its 32 raw bytes, applying the
/// same acceptance rules as `validate_solana_address`. This is the byte-yielding
/// counterpart used when a 32-byte host identity must be threaded into a request
/// (e.g. the Solana input-proof attestation), where the EVM 20-byte `Address`
/// cannot hold the full Ed25519 public key.
pub fn decode_solana_address(address: &str) -> Result<[u8; 32], ValidationError> {
    validate_solana_address(address)?;
    let decoded = base58_decode(address).map_err(|_| {
        ValidationError::new("validation_error")
            .with_message(solana_address_messages::INVALID_BASE58_CHARACTER.into())
    })?;
    let mut out = [0u8; SOLANA_ADDRESS_LEN];
    out.copy_from_slice(&decoded);
    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;

    // Well-known Solana addresses (32-byte Ed25519 pubkeys in base58).
    const SYSTEM_PROGRAM: &str = "11111111111111111111111111111111";
    const TOKEN_PROGRAM: &str = "TokenkegQfeZyiNwAJbNbGKPFXCWuBvf9Ss623VQ5DA";
    const WRAPPED_SOL_MINT: &str = "So11111111111111111111111111111111111111112";

    #[test]
    fn solana_address_accepts_canonical_pubkeys() {
        for address in [SYSTEM_PROGRAM, TOKEN_PROGRAM, WRAPPED_SOL_MINT] {
            assert!(
                validate_solana_address(address).is_ok(),
                "expected {address} to be a valid Solana address"
            );
            assert!(is_solana_address(address));
        }
    }

    #[test]
    fn solana_address_rejects_empty_string() {
        let error = validate_solana_address("").unwrap_err();
        assert_eq!(
            error.message.as_deref(),
            Some(solana_address_messages::MUST_NOT_BE_EMPTY)
        );
    }

    #[test]
    fn solana_address_rejects_non_base58_characters() {
        // '0', 'O', 'I', 'l' are excluded from the base58 alphabet, as is '+'.
        for bad in ["0invalidaddress", "OIl0", "not+base58"] {
            let error = validate_solana_address(bad).unwrap_err();
            assert_eq!(
                error.message.as_deref(),
                Some(solana_address_messages::INVALID_BASE58_CHARACTER),
                "expected {bad} to be rejected as non-base58"
            );
        }
    }

    #[test]
    fn solana_address_rejects_wrong_byte_length() {
        // Valid base58 but decodes to fewer than 32 bytes.
        let too_short = "abc";
        let error = validate_solana_address(too_short).unwrap_err();
        assert_eq!(
            error.message.as_deref(),
            Some(solana_address_messages::MUST_DECODE_TO_32_BYTES)
        );
    }

    #[test]
    fn solana_address_rejects_evm_0x_hex_form() {
        // An EVM-style 0x-prefixed address is not valid Solana base58: '0' and
        // 'x' handling aside, it must not be accepted by the Solana validator.
        let evm = "0x0123456789abcdef0123456789abcdef01234567";
        assert!(validate_solana_address(evm).is_err());
    }

    #[test]
    fn solana_address_base58_decode_roundtrip_preserves_leading_zeroes() {
        // The all-'1' system program id decodes to 32 zero bytes.
        let decoded = base58_decode(SYSTEM_PROGRAM).expect("valid base58");
        assert_eq!(decoded, vec![0u8; SOLANA_ADDRESS_LEN]);
    }
}
