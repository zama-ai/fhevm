use crate::core::event::UserDecryptRequest;
use crate::orchestrator::IndexerIdGenerator;
use sha2::{Digest, Sha256};

impl IndexerIdGenerator for UserDecryptRequest {
    /// TODO: Consider canonical ordering for list items.
    fn compute_indexer_id(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(b"contract_addresses:"); // 1
        for address in &self.contract_addresses {
            hasher.update(address.as_slice());
        }

        hasher.update(b"contracts_chain_id:"); // 2
        hasher.update(self.contracts_chain_id.to_be_bytes());

        hasher.update(b"ct_handle_contract_pairs:"); // 3
        for pair in &self.ct_handle_contract_pairs {
            hasher.update(pair.ct_handle.to_be_bytes::<32>());
            hasher.update(pair.contract_address.as_slice());
        }

        hasher.update(b"extra_data:"); // 4
        hasher.update(&self.extra_data);

        hasher.update(b"public_key:"); // 5
        hasher.update(&self.public_key);

        hasher.update(b"request_validity:"); // 6
        hasher.update(b"duration_days:");
        hasher.update(self.request_validity.duration_days.to_be_bytes::<32>());
        hasher.update(b"start_timestamp:");
        hasher.update(self.request_validity.start_timestamp.to_be_bytes::<32>());

        hasher.update(b"signature:"); // 7
        hasher.update(&self.signature);

        hasher.update(b"user_address:"); // 8
        hasher.update(self.user_address.as_slice());

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::{HandleContractPair, RequestValidity};
    use alloy::primitives::{Address, Bytes, U256};

    #[test]
    fn test_user_decrypt_request_indexer_id_deterministic() {
        let request = UserDecryptRequest {
            ct_handle_contract_pairs: vec![HandleContractPair {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
            }],
            request_validity: RequestValidity {
                start_timestamp: U256::from(1000),
                duration_days: U256::from(30),
            },
            contracts_chain_id: 1337,
            contract_addresses: vec![Address::from([1; 20])],
            user_address: Address::from([2; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let id1 = request.compute_indexer_id();
        let id2 = request.compute_indexer_id();

        assert_eq!(id1, id2, "Same request should produce same indexer ID");
        assert_eq!(id1.len(), 32, "Indexer ID should be 32 bytes");
    }

    #[test]
    fn test_user_decrypt_request_indexer_id_different_for_different_requests() {
        let request1 = UserDecryptRequest {
            ct_handle_contract_pairs: vec![HandleContractPair {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
            }],
            request_validity: RequestValidity {
                start_timestamp: U256::from(1000),
                duration_days: U256::from(30),
            },
            contracts_chain_id: 1337,
            contract_addresses: vec![Address::from([1; 20])],
            user_address: Address::from([2; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let mut request2 = request1.clone();
        request2.contracts_chain_id = 1338; // Different chain ID

        let id1 = request1.compute_indexer_id();
        let id2 = request2.compute_indexer_id();

        assert_ne!(
            id1, id2,
            "Different requests should produce different indexer IDs"
        );
    }
}
