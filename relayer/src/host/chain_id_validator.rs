use std::collections::HashSet;

use crate::core::event::HandleContractPair;
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

    /// Validate that all U256 handles in HandleContractPair have a supported chain ID.
    /// Returns `Err(unsupported_chain_id)` on the first mismatch.
    pub fn validate_u256_handles(&self, pairs: &[HandleContractPair]) -> Result<(), u64> {
        for pair in pairs {
            let chain_id = extract_chain_id_from_u256(&pair.ct_handle);
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
    use alloy::primitives::{Address, U256};

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
        let handle_bytes = make_handle_with_chain_id(8009);
        let pairs = vec![HandleContractPair {
            ct_handle: U256::from_be_bytes(handle_bytes),
            contract_address: Some(Address::ZERO),
            contract_id: None,
        }];
        assert!(validator.validate_u256_handles(&pairs).is_ok());
    }

    #[test]
    fn test_validate_u256_handles_unsupported() {
        let validator = HostChainIdChecker::new(vec![8009]);
        let handle_bytes = make_handle_with_chain_id(5555);
        let pairs = vec![HandleContractPair {
            ct_handle: U256::from_be_bytes(handle_bytes),
            contract_address: Some(Address::ZERO),
            contract_id: None,
        }];
        assert_eq!(validator.validate_u256_handles(&pairs), Err(5555));
    }
}
