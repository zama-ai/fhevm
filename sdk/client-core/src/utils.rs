use crate::ClientCoreError;
use crate::Result;
use alloy::primitives::{Address, Bytes};
use std::str::FromStr;
use tracing::debug;

/// Validate that an address is not the zero address.
pub fn validate_address(addr: &Address) -> Result<()> {
    if addr.is_zero() {
        return Err(ClientCoreError::InvalidParams(
            "Zero address is not allowed".to_string(),
        ));
    }
    Ok(())
}

/// Parse and validate an Ethereum address from a string.
pub fn validate_address_from_str(addr_str: &str) -> Result<Address> {
    if addr_str.trim().is_empty() {
        return Err(ClientCoreError::InvalidParams(
            "Address string cannot be empty".to_string(),
        ));
    }
    debug!("Parsing address: {}", addr_str);

    let address = Address::from_str(addr_str.trim()).map_err(|e| {
        ClientCoreError::InvalidParams(format!("Invalid address format '{addr_str}': {e}"))
    })?;

    debug!("Parsed address: {}", address);
    validate_address(&address)?;

    Ok(address)
}

/// Parse a hex string (with or without 0x prefix) into bytes.
pub fn parse_hex_string(hex_str: &str, field_name: &str) -> Result<Bytes> {
    let cleaned = hex_str.strip_prefix("0x").unwrap_or(hex_str);

    let bytes = hex::decode(cleaned).map_err(|e| {
        ClientCoreError::InvalidParams(format!("Invalid hex string for {field_name}: {e}"))
    })?;

    Ok(Bytes::from(bytes))
}

/// Convert a chain ID to a standardized 32-byte big-endian representation.
///
/// This matches the Node.js implementation:
/// `fromHexString(chainId.toString(16).padStart(64, '0'))`
pub fn chain_id_to_bytes(chain_id: u64) -> [u8; 32] {
    let mut buffer = [0u8; 32];
    buffer[24..32].copy_from_slice(&chain_id.to_be_bytes());
    buffer
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_chain_id_to_bytes_different_sizes() {
        // Small chain ID (1 - Ethereum Mainnet)
        let bytes = chain_id_to_bytes(1);
        assert_eq!(
            hex::encode(bytes),
            "0000000000000000000000000000000000000000000000000000000000000001"
        );

        // Medium chain ID (42161 - Arbitrum One)
        let bytes = chain_id_to_bytes(42161);
        assert_eq!(
            hex::encode(bytes),
            "000000000000000000000000000000000000000000000000000000000000a4b1"
        );

        // Large chain ID (4294967295)
        let bytes = chain_id_to_bytes(4294967295);
        assert_eq!(
            hex::encode(bytes),
            "00000000000000000000000000000000000000000000000000000000ffffffff"
        );

        // Max u64
        let bytes = chain_id_to_bytes(u64::MAX);
        assert_eq!(
            hex::encode(bytes),
            "000000000000000000000000000000000000000000000000ffffffffffffffff"
        );
    }

    #[test]
    fn test_validate_address_rejects_zero() {
        assert!(validate_address(&Address::ZERO).is_err());
    }

    #[test]
    fn test_validate_address_from_str_rejects_empty() {
        assert!(validate_address_from_str("").is_err());
        assert!(validate_address_from_str("  ").is_err());
    }

    #[test]
    fn test_parse_hex_string() {
        let result = parse_hex_string("0xabcd", "test").unwrap();
        assert_eq!(result.as_ref(), &[0xab, 0xcd]);

        let result = parse_hex_string("abcd", "test").unwrap();
        assert_eq!(result.as_ref(), &[0xab, 0xcd]);

        assert!(parse_hex_string("xyz", "test").is_err());
    }
}
