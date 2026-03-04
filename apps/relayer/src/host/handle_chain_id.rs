use alloy::primitives::U256;

/// Extract chain ID from bytes 22..30 of a 32-byte handle (big-endian u64).
pub fn extract_chain_id_from_handle(handle: &[u8; 32]) -> u64 {
    let bytes: [u8; 8] = handle[22..30].try_into().expect("slice is exactly 8 bytes");
    u64::from_be_bytes(bytes)
}

/// Extract chain ID from a U256 handle by converting to big-endian bytes first.
pub fn extract_chain_id_from_u256(handle: &U256) -> u64 {
    let bytes: [u8; 32] = handle.to_be_bytes();
    extract_chain_id_from_handle(&bytes)
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_handle_with_chain_id(chain_id: u64) -> [u8; 32] {
        let mut handle = [0u8; 32];
        handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
        handle
    }

    #[test]
    fn test_extract_chain_id_from_handle() {
        let handle = make_handle_with_chain_id(8009);
        assert_eq!(extract_chain_id_from_handle(&handle), 8009);
    }

    #[test]
    fn test_extract_chain_id_from_u256() {
        let handle_bytes = make_handle_with_chain_id(12345);
        let handle_u256 = U256::from_be_bytes(handle_bytes);
        assert_eq!(extract_chain_id_from_u256(&handle_u256), 12345);
    }
}
