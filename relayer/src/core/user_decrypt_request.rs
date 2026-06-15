use crate::core::event::UserDecryptRequest;
use crate::orchestrator::ContentHasher;
use sha2::{Digest, Sha256};

impl ContentHasher for UserDecryptRequest {
    /// Content hash used for dedup of in-flight requests.
    ///
    /// Bit-identical to the pre-refactor hash for each legacy EIP-712
    /// attestation type:
    /// - LegacyDirect: contract_addresses / contracts_chain_id /
    ///   ct_handle_contract_pairs / extra_data / public_key /
    ///   user_address (in that order). Same as the original
    ///   `UserDecryptRequest` impl.
    /// - LegacyDelegated: contract_addresses / contracts_chain_id /
    ///   ct_handle_contract_pairs / extra_data / public_key /
    ///   delegator_address / delegate_address. Same as the original
    ///   `DelegatedUserDecryptRequest` impl.
    /// - Eip712UnifiedV1: a `variant:eip712_unified_v1` prefix plus the
    ///   variant-specific fields so unified-EIP-712 requests for the
    ///   same handle set produce distinct dedup keys from legacy ones.
    ///
    /// `signature` and the validity-window fields are excluded — they're
    /// consumed on-chain prior to receiving a decryption-id and shouldn't
    /// gate dedup.
    ///
    /// TODO: Consider canonical ordering for list items.
    fn content_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        match self {
            UserDecryptRequest::LegacyDirect {
                ct_handle_contract_pairs,
                contracts_chain_id,
                contract_addresses,
                user_address,
                request_validity: _,
                signature: _,
                public_key,
                extra_data,
            } => {
                hasher.update(b"contract_addresses:"); // 1
                for address in contract_addresses {
                    hasher.update(address.as_slice());
                }

                hasher.update(b"contracts_chain_id:"); // 2
                hasher.update(contracts_chain_id.to_be_bytes());

                hasher.update(b"ct_handle_contract_pairs:"); // 3
                for pair in ct_handle_contract_pairs {
                    hasher.update(pair.ct_handle.to_be_bytes::<32>());
                    hasher.update(pair.contract_address.as_slice());
                }

                hasher.update(b"extra_data:"); // 4
                hasher.update(extra_data);

                hasher.update(b"public_key:"); // 5
                hasher.update(public_key);

                hasher.update(b"user_address:"); // 6
                hasher.update(user_address.as_slice());
            }
            UserDecryptRequest::LegacyDelegated {
                ct_handle_contract_pairs,
                contracts_chain_id,
                contract_addresses,
                delegator_address,
                delegate_address,
                request_validity: _,
                signature: _,
                public_key,
                extra_data,
            } => {
                hasher.update(b"contract_addresses:"); // 1
                for address in contract_addresses {
                    hasher.update(address.as_slice());
                }

                hasher.update(b"contracts_chain_id:"); // 2
                hasher.update(contracts_chain_id.to_be_bytes());

                hasher.update(b"ct_handle_contract_pairs:"); // 3
                for pair in ct_handle_contract_pairs {
                    hasher.update(pair.ct_handle.to_be_bytes::<32>());
                    hasher.update(pair.contract_address.as_slice());
                }

                hasher.update(b"extra_data:"); // 4
                hasher.update(extra_data);

                hasher.update(b"public_key:"); // 5
                hasher.update(public_key);

                hasher.update(b"delegator_address:"); // 6
                hasher.update(delegator_address.as_slice());

                hasher.update(b"delegate_address:"); // 7
                hasher.update(delegate_address.as_slice());
            }
            UserDecryptRequest::Eip712UnifiedV1 {
                handles,
                user_address,
                allowed_contracts,
                request_validity: _,
                signature: _,
                public_key,
                extra_data,
            } => {
                // Variant tag prevents v3 unified hashes from colliding
                // with v2 legacy hashes that share the same user_address +
                // handles.
                hasher.update(b"variant:eip712_unified_v1:");

                hasher.update(b"allowed_contracts:");
                for address in allowed_contracts {
                    hasher.update(address.as_slice());
                }

                hasher.update(b"handles:");
                for h in handles {
                    hasher.update(h.ct_handle.to_be_bytes::<32>());
                    hasher.update(h.contract_address.as_slice());
                    hasher.update(h.owner_address.as_slice());
                }

                hasher.update(b"extra_data:");
                hasher.update(extra_data);

                hasher.update(b"public_key:");
                hasher.update(public_key);

                hasher.update(b"user_address:");
                hasher.update(user_address.as_slice());
            }
            UserDecryptRequest::SolanaUnifiedV1 {
                handles,
                user_identity,
                allowed_acl_domain_keys,
                request_validity: _,
                nonce,
                signature: _,
                public_key,
                extra_data,
            } => {
                // Distinct variant tag so Solana hashes never collide with EVM unified hashes.
                hasher.update(b"variant:solana_unified_v1:");

                hasher.update(b"allowed_acl_domain_keys:");
                for key in allowed_acl_domain_keys {
                    hasher.update(key.as_slice());
                }

                hasher.update(b"handles:");
                for h in handles {
                    hasher.update(h.ct_handle.to_be_bytes::<32>());
                    hasher.update(h.contract_address.as_slice());
                    hasher.update(h.owner_address.as_slice());
                }

                hasher.update(b"extra_data:");
                hasher.update(extra_data);

                hasher.update(b"public_key:");
                hasher.update(public_key);

                hasher.update(b"user_identity:");
                hasher.update(user_identity.as_slice());

                hasher.update(b"nonce:");
                hasher.update(nonce.as_slice());
            }
        }

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event::{
        HandleContractPair, HandleEntry, RequestValidity, RequestValiditySeconds,
    };
    use alloy::primitives::{Address, Bytes, U256};

    fn sample_legacy_direct() -> UserDecryptRequest {
        UserDecryptRequest::LegacyDirect {
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
        }
    }

    fn sample_legacy_delegated() -> UserDecryptRequest {
        UserDecryptRequest::LegacyDelegated {
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
            delegator_address: Address::from([2; 20]),
            delegate_address: Address::from([3; 20]),
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
        }
    }

    fn sample_eip712_unified() -> UserDecryptRequest {
        UserDecryptRequest::Eip712UnifiedV1 {
            handles: vec![HandleEntry {
                ct_handle: U256::from(123),
                contract_address: Address::from([1; 20]),
                owner_address: Address::from([2; 20]),
            }],
            user_address: Address::from([2; 20]),
            allowed_contracts: vec![Address::from([1; 20])],
            request_validity: RequestValiditySeconds {
                start_timestamp: U256::from(1000),
                duration_seconds: U256::from(604_800),
            },
            signature: Bytes::from(vec![0xde, 0xad, 0xbe, 0xef]),
            public_key: Bytes::from(vec![0xab, 0xcd]),
            extra_data: Bytes::from(vec![0x00]),
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
        if let UserDecryptRequest::LegacyDirect {
            contracts_chain_id, ..
        } = &mut r2
        {
            *contracts_chain_id = 1338;
        }
        assert_ne!(r1.content_hash(), r2.content_hash());
    }

    #[test]
    fn legacy_direct_excluded_fields_dont_affect_hash() {
        let r1 = sample_legacy_direct();
        let mut r2 = r1.clone();
        if let UserDecryptRequest::LegacyDirect {
            signature,
            request_validity,
            ..
        } = &mut r2
        {
            *signature = Bytes::from(vec![0xff; 4]);
            *request_validity = RequestValidity {
                start_timestamp: U256::from(999_999),
                duration_days: U256::from(99_999),
            };
        }
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
        if let UserDecryptRequest::LegacyDelegated {
            contracts_chain_id, ..
        } = &mut r2
        {
            *contracts_chain_id = 1338;
        }
        assert_ne!(r1.content_hash(), r2.content_hash());
    }

    #[test]
    fn legacy_delegated_excluded_fields_dont_affect_hash() {
        let r1 = sample_legacy_delegated();
        let mut r2 = r1.clone();
        if let UserDecryptRequest::LegacyDelegated {
            signature,
            request_validity,
            ..
        } = &mut r2
        {
            *signature = Bytes::from(vec![0xff; 4]);
            *request_validity = RequestValidity {
                start_timestamp: U256::from(999_999),
                duration_days: U256::from(99_999),
            };
        }
        assert_eq!(r1.content_hash(), r2.content_hash());
    }

    /// The three attestation formats produce distinct hashes for inputs
    /// that would otherwise look similar — the format tag (implicit for
    /// legacy via field-name labels, explicit for unified via the
    /// `variant:eip712_unified_v1:` prefix) ensures dedup never collides
    /// across formats.
    #[test]
    fn attestation_formats_hash_differently() {
        let direct = sample_legacy_direct().content_hash();
        let delegated = sample_legacy_delegated().content_hash();
        let unified = sample_eip712_unified().content_hash();
        assert_ne!(direct, delegated);
        assert_ne!(direct, unified);
        assert_ne!(delegated, unified);
    }
}
