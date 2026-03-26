use crate::core::event::{DelegatedUserDecryptRequest, UserDecryptRequest};
use crate::orchestrator::ContentHasher;
use sha2::{Digest, Sha256};

impl ContentHasher for UserDecryptRequest {
    /// TODO: Consider canonical ordering for list items.
    fn content_hash(&self) -> [u8; 32] {
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

        hasher.update(b"user_address:"); // 6
        hasher.update(self.user_address.as_slice());

        // NOTE: signature and request_validity are excluded from content hash
        // because these are only used on-chain prior to receiving a decryption-id.

        hasher.finalize().into()
    }
}

impl ContentHasher for DelegatedUserDecryptRequest {
    /// TODO: Consider canonical ordering for list items.
    fn content_hash(&self) -> [u8; 32] {
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

        hasher.update(b"delegator_address:"); // 6
        hasher.update(self.delegator_address.as_slice());

        hasher.update(b"delegate_address:"); // 7
        hasher.update(self.delegate_address.as_slice());

        // NOTE: signature, startTimestamp and durationDays are excluded from content hash
        // because these are only used on-chain prior to receiving a decryption-id.

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::{HandleContractPair, RequestValidity};
    use alloy::primitives::{Address, Bytes, U256};

    #[test]
    fn test_user_decrypt_request_content_hash_deterministic() {
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

        let id1 = request.content_hash();
        let id2 = request.content_hash();

        assert_eq!(id1, id2, "Same request should produce same content hash");
        assert_eq!(id1.len(), 32, "Content hash should be 32 bytes");
    }

    #[test]
    fn test_user_decrypt_request_content_hash_different_for_different_requests() {
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

        let id1 = request1.content_hash();
        let id2 = request2.content_hash();

        assert_ne!(
            id1, id2,
            "Different requests should produce different content hashes"
        );
    }

    #[test]
    fn test_user_decrypt_request_excluded_fields_dont_affect_hash() {
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

        // Change excluded fields - these should NOT affect the content hash
        request2.signature = Bytes::from(vec![0xff, 0xff, 0xff, 0xff]);
        request2.request_validity = RequestValidity {
            start_timestamp: U256::from(999999),
            duration_days: U256::from(99999),
        };

        let hash1 = request1.content_hash();
        let hash2 = request2.content_hash();

        assert_eq!(
            hash1, hash2,
            "Changing signature and request_validity should NOT affect content hash"
        );

        // Change included field - this SHOULD affect the content hash
        request2.contracts_chain_id = 9999;
        let hash3 = request2.content_hash();

        assert_ne!(
            hash1, hash3,
            "Changing contracts_chain_id should affect content hash"
        );
    }

    #[test]
    fn test_delegated_user_decrypt_request_content_hash_deterministic() {
        let request = DelegatedUserDecryptRequest {
            ct_handle_contract_pairs: vec![HandleContractPair {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
            }],
            start_timestamp: U256::from(1000),
            duration_days: U256::from(30),
            contracts_chain_id: 1337,
            contract_addresses: vec![Address::from([1; 20])],
            delegator_address: Address::from([2; 20]),
            delegate_address: Address::from([3; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let id1 = request.content_hash();
        let id2 = request.content_hash();

        assert_eq!(id1, id2, "Same request should produce same content hash");
        assert_eq!(id1.len(), 32, "Content hash should be 32 bytes");
    }

    #[test]
    fn test_delegated_user_decrypt_request_content_hash_different_for_different_requests() {
        let request1 = DelegatedUserDecryptRequest {
            ct_handle_contract_pairs: vec![HandleContractPair {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
            }],
            start_timestamp: U256::from(1000),
            duration_days: U256::from(30),
            contracts_chain_id: 1337,
            contract_addresses: vec![Address::from([1; 20])],
            delegator_address: Address::from([2; 20]),
            delegate_address: Address::from([3; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let mut request2 = request1.clone();
        request2.contracts_chain_id = 1338; // Different chain ID

        let id1 = request1.content_hash();
        let id2 = request2.content_hash();

        assert_ne!(
            id1, id2,
            "Different requests should produce different content hashes"
        );
    }

    #[test]
    fn test_delegated_user_decrypt_request_excluded_fields_dont_affect_hash() {
        let request1 = DelegatedUserDecryptRequest {
            ct_handle_contract_pairs: vec![HandleContractPair {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
            }],
            start_timestamp: U256::from(1000),
            duration_days: U256::from(30),
            contracts_chain_id: 1337,
            contract_addresses: vec![Address::from([1; 20])],
            delegator_address: Address::from([2; 20]),
            delegate_address: Address::from([3; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let mut request2 = request1.clone();

        // Change excluded fields - these should NOT affect the content hash
        request2.signature = Bytes::from(vec![0xff, 0xff, 0xff, 0xff]);
        request2.start_timestamp = U256::from(999999);
        request2.duration_days = U256::from(99999);

        let hash1 = request1.content_hash();
        let hash2 = request2.content_hash();

        assert_eq!(
            hash1, hash2,
            "Changing signature and request_validity should NOT affect content hash"
        );

        // Change included field - this SHOULD affect the content hash
        request2.contracts_chain_id = 9999;
        let hash3 = request2.content_hash();

        assert_ne!(
            hash1, hash3,
            "Changing contracts_chain_id should affect content hash"
        );
    }
}
