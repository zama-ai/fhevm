//! Base58 Solana address validation and decoding for the relayer.

use solana_pubkey::{ParsePubkeyError, Pubkey, PUBKEY_BYTES};
use std::str::FromStr;
use validator::ValidationError;

pub const SOLANA_ADDRESS_LEN: usize = PUBKEY_BYTES;

pub mod solana_address_messages {
    pub const MUST_NOT_BE_EMPTY: &str = "Solana address must not be empty";
    pub const INVALID_BASE58_CHARACTER: &str =
        "Solana address contains a non-base58 character (allowed: 1-9, A-Z, a-z excluding 0OIl)";
    pub const MUST_DECODE_TO_32_BYTES: &str =
        "Solana address must base58-decode to exactly 32 bytes (Ed25519 public key)";
}

/// Validates a base58-encoded 32-byte address, including off-curve PDAs.
pub fn validate_solana_address(address: &str) -> Result<(), ValidationError> {
    decode_solana_address(address).map(|_| ())
}

/// Returns whether `address` is a valid Solana base58 address.
pub fn is_solana_address(address: &str) -> bool {
    validate_solana_address(address).is_ok()
}

/// Decodes a base58 address without requiring it to be an on-curve public key.
pub fn decode_solana_address(address: &str) -> Result<[u8; SOLANA_ADDRESS_LEN], ValidationError> {
    if address.is_empty() {
        return Err(ValidationError::new("validation_error")
            .with_message(solana_address_messages::MUST_NOT_BE_EMPTY.into()));
    }

    Pubkey::from_str(address)
        .map(|pubkey| pubkey.to_bytes())
        .map_err(|error| {
            let message = match error {
                ParsePubkeyError::Invalid => solana_address_messages::INVALID_BASE58_CHARACTER,
                ParsePubkeyError::WrongSize => solana_address_messages::MUST_DECODE_TO_32_BYTES,
            };
            ValidationError::new("validation_error").with_message(message.into())
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    // Well-known Solana addresses.
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
    fn solana_address_accepts_off_curve_pdas() {
        let program = Pubkey::from_str(TOKEN_PROGRAM).unwrap();
        let (pda, _) = Pubkey::find_program_address(&[b"address-validation"], &program);
        assert!(!pda.is_on_curve());
        assert_eq!(
            decode_solana_address(&pda.to_string()).unwrap(),
            pda.to_bytes()
        );
    }

    #[test]
    fn solana_address_rejects_oversized_strings() {
        // The library bounds input length before checking characters.
        for address in ["1".repeat(45), "0".repeat(45)] {
            let error = decode_solana_address(&address).unwrap_err();
            assert_eq!(
                error.message.as_deref(),
                Some(solana_address_messages::MUST_DECODE_TO_32_BYTES)
            );
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
        for bad in [
            "0invalidaddress",
            "OIl0",
            "not+base58",
            "é",
            " 11111111111111111111111111111111",
        ] {
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
        // Valid base58 with the wrong decoded width.
        for address in [
            "abc",
            "1111111111111111111111111111111",
            "111111111111111111111111111111111",
        ] {
            let error = validate_solana_address(address).unwrap_err();
            assert_eq!(
                error.message.as_deref(),
                Some(solana_address_messages::MUST_DECODE_TO_32_BYTES)
            );
        }
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
        let decoded = decode_solana_address(SYSTEM_PROGRAM).expect("valid address");
        assert_eq!(decoded, [0u8; SOLANA_ADDRESS_LEN]);
        let mut bytes = [0u8; SOLANA_ADDRESS_LEN];
        bytes[SOLANA_ADDRESS_LEN - 1] = 1;
        let address = Pubkey::new_from_array(bytes).to_string();
        assert_eq!(address, "11111111111111111111111111111112");
        assert_eq!(decode_solana_address(&address).unwrap(), bytes);
    }
}
