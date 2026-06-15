use std::collections::HashSet;

use alloy::primitives::U256;

use crate::host::handle_chain_id::{extract_chain_id_from_handle, extract_chain_id_from_u256};

pub struct HostChainIdChecker {
    supported_chain_ids: HashSet<u64>,
}

impl HostChainIdChecker {
    pub fn new(chain_ids: Vec<u64>) -> Self {
        Self {
            supported_chain_ids: chain_ids.into_iter().collect(),
        }
    }

    /// Validate that all handles have a supported chain ID.
    /// Returns `Err(unsupported_chain_id)` on the first mismatch.
    pub fn validate_handles(&self, handles: &[[u8; 32]]) -> Result<(), u64> {
        for handle in handles {
            let chain_id = extract_chain_id_from_handle(handle);
            if !self.supported_chain_ids.contains(&chain_id) {
                return Err(chain_id);
            }
        }
        Ok(())
    }

    /// Validate that every supplied U256 handle encodes a supported chain
    /// ID. Returns `Err(unsupported_chain_id)` on the first mismatch.
    ///
    /// Takes an iterator over `&U256` so call sites can pass handles from
    /// any container shape (e.g. `Vec<HandleContractPair>` via
    /// `iter().map(|p| &p.ct_handle)`, or `Vec<HandleEntry>` via the
    /// equivalent map) without allocating an intermediate `Vec`.
    pub fn validate_u256_handles<'a, I>(&self, handles: I) -> Result<(), u64>
    where
        I: IntoIterator<Item = &'a U256>,
    {
        for handle in handles {
            let chain_id = extract_chain_id_from_u256(handle);
            if !self.supported_chain_ids.contains(&chain_id) {
                return Err(chain_id);
            }
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::U256;

    fn make_handle_with_chain_id(chain_id: u64) -> [u8; 32] {
        let mut handle = [0u8; 32];
        handle[22..30].copy_from_slice(&chain_id.to_be_bytes());
        handle
    }

    #[test]
    fn test_validate_handles_all_valid() {
        let validator = HostChainIdChecker::new(vec![8009, 9000]);
        let handles = vec![
            make_handle_with_chain_id(8009),
            make_handle_with_chain_id(9000),
        ];
        assert!(validator.validate_handles(&handles).is_ok());
    }

    #[test]
    fn test_validate_handles_unsupported() {
        let validator = HostChainIdChecker::new(vec![8009]);
        let handles = vec![
            make_handle_with_chain_id(8009),
            make_handle_with_chain_id(9999),
        ];
        assert_eq!(validator.validate_handles(&handles), Err(9999));
    }

    #[test]
    fn test_validate_handles_empty() {
        let validator = HostChainIdChecker::new(vec![8009]);
        assert!(validator.validate_handles(&[]).is_ok());
    }

    #[test]
    fn test_validate_u256_handles_all_valid() {
        let validator = HostChainIdChecker::new(vec![8009]);
        let handle = U256::from_be_bytes(make_handle_with_chain_id(8009));
        assert!(validator.validate_u256_handles([&handle]).is_ok());
    }

    #[test]
    fn test_validate_u256_handles_unsupported() {
        let validator = HostChainIdChecker::new(vec![8009]);
        let handle = U256::from_be_bytes(make_handle_with_chain_id(5555));
        assert_eq!(validator.validate_u256_handles([&handle]), Err(5555));
    }
}
