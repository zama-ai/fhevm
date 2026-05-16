use crate::core::event::{UserDecryptPayload, UserDecryptRequest};
use crate::orchestrator::ContentHasher;
use sha2::{Digest, Sha256};

impl ContentHasher for UserDecryptRequest {
    /// Content hash used for dedup of in-flight requests.
    ///
    /// Bit-identical to the pre-refactor hash on both v2 dialects:
    /// - LegacyDirect: hashes contract_addresses / contracts_chain_id /
    ///   ct_handle_contract_pairs / extra_data / public_key / user_address
    ///   (in that order). Same as the old `UserDecryptRequest`.
    /// - LegacyDelegated: hashes contract_addresses / contracts_chain_id /
    ///   ct_handle_contract_pairs / extra_data / public_key /
    ///   delegator_address / delegate_address (in that order). Same as the
    ///   old `DelegatedUserDecryptRequest`.
    ///
    /// `signature` and `request_validity` are excluded — they're consumed
    /// on-chain prior to receiving a decryption-id and shouldn't gate dedup.
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

        match &self.payload {
            UserDecryptPayload::LegacyDirect { user_address } => {
                hasher.update(b"user_address:"); // 6
                hasher.update(user_address.as_slice());
            }
            UserDecryptPayload::LegacyDelegated {
                delegator_address,
                delegate_address,
            } => {
                hasher.update(b"delegator_address:"); // 6
                hasher.update(delegator_address.as_slice());

                hasher.update(b"delegate_address:"); // 7
                hasher.update(delegate_address.as_slice());
            }
        }

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::{HandleContractPair, RequestValidity, UserDecryptPayload};
    use alloy::primitives::{Address, Bytes, U256};

    fn sample_legacy_direct() -> UserDecryptRequest {
        UserDecryptRequest {
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
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
            payload: UserDecryptPayload::LegacyDirect {
                user_address: Address::from([2; 20]),
            },
        }
    }

    fn sample_legacy_delegated() -> UserDecryptRequest {
        UserDecryptRequest {
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
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
            payload: UserDecryptPayload::LegacyDelegated {
                delegator_address: Address::from([2; 20]),
                delegate_address: Address::from([3; 20]),
            },
        }
    }

    #[test]
    fn legacy_direct_content_hash_deterministic() {
        let request = sample_legacy_direct();
        assert_eq!(request.content_hash(), request.content_hash());
        assert_eq!(request.content_hash().len(), 32);
    }

    #[test]
    fn legacy_direct_content_hash_differs_on_chain_id() {
        let r1 = sample_legacy_direct();
        let mut r2 = r1.clone();
        r2.contracts_chain_id = 1338;
        assert_ne!(r1.content_hash(), r2.content_hash());
    }

    #[test]
    fn legacy_direct_excluded_fields_dont_affect_hash() {
        let r1 = sample_legacy_direct();
        let mut r2 = r1.clone();
        r2.signature = Bytes::from(vec![0xff; 4]);
        r2.request_validity = RequestValidity {
            start_timestamp: U256::from(999_999),
            duration_days: U256::from(99_999),
        };
        assert_eq!(r1.content_hash(), r2.content_hash());
    }

    #[test]
    fn legacy_delegated_content_hash_deterministic() {
        let request = sample_legacy_delegated();
        assert_eq!(request.content_hash(), request.content_hash());
        assert_eq!(request.content_hash().len(), 32);
    }

    #[test]
    fn legacy_delegated_content_hash_differs_on_chain_id() {
        let r1 = sample_legacy_delegated();
        let mut r2 = r1.clone();
        r2.contracts_chain_id = 1338;
        assert_ne!(r1.content_hash(), r2.content_hash());
    }

    #[test]
    fn legacy_delegated_excluded_fields_dont_affect_hash() {
        let r1 = sample_legacy_delegated();
        let mut r2 = r1.clone();
        r2.signature = Bytes::from(vec![0xff; 4]);
        r2.request_validity = RequestValidity {
            start_timestamp: U256::from(999_999),
            duration_days: U256::from(99_999),
        };
        assert_eq!(r1.content_hash(), r2.content_hash());
    }

    /// Direct and delegated payloads for the same handle set hash differently
    /// because each variant folds in dialect-specific addresses (user_address
    /// vs delegator+delegate). The byte-level layout matches the pre-refactor
    /// `UserDecryptRequest` / `DelegatedUserDecryptRequest` impls.
    #[test]
    fn legacy_direct_and_delegated_hash_differently() {
        assert_ne!(
            sample_legacy_direct().content_hash(),
            sample_legacy_delegated().content_hash()
        );
    }
}
