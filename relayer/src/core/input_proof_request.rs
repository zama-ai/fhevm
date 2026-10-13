use crate::core::event::{HostIdentities, InputProofRequest};
use crate::orchestrator::ContentHasher;
use sha2::{Digest, Sha256};

impl ContentHasher for InputProofRequest {
    /// Computes SHA-256 hash from all request fields.
    /// All fields are semantically meaningful and included in the hash.
    ///
    /// This enables content-based deduplication: identical requests produce
    /// identical hashes, allowing the system to detect and handle duplicates.
    fn content_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(b"contract_chain_id:");
        hasher.update(self.contract_chain_id.to_be_bytes());

        match &self.identities {
            HostIdentities::Evm {
                contract_address,
                user_address,
            } => {
                hasher.update(b"contract_address:");
                hasher.update(contract_address.as_slice());

                hasher.update(b"user_address:");
                hasher.update(user_address.as_slice());
            }
            HostIdentities::Solana {
                solana_contract_address,
                solana_user_address,
            } => {
                hasher.update(b"solana_contract_address:");
                hasher.update(solana_contract_address.as_slice());

                hasher.update(b"solana_user_address:");
                hasher.update(solana_user_address.as_slice());
            }
        }

        hasher.update(b"ciphertext_with_zk_proof:");
        hasher.update(&self.ciphetext_with_zk_proof);

        hasher.update(b"extra_data:");
        hasher.update(&self.extra_data);

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Address, Bytes, FixedBytes};

    /// The hash an EVM request has always had, so a request stored before Solana identities
    /// existed still deduplicates against a new one.
    #[test]
    fn an_evm_request_hashes_as_before_solana_identities_existed() {
        let request = InputProofRequest::new(
            1337,
            Address::from([1; 20]),
            Address::from([2; 20]),
            Bytes::from(vec![0xaa]),
            Bytes::from(vec![0x00]),
        );
        assert_eq!(
            hex::encode(request.content_hash()),
            "89b04c8e13f48c3225fa4cee9520af1e81610111deb3f61542b860d2837d0f04"
        );
    }

    #[test]
    fn each_solana_identity_affects_the_hash() {
        let solana = |contract: u8, user: u8| InputProofRequest {
            contract_chain_id: zama_solana_acl::host_chain::solana_host_chain_id(12345),
            identities: HostIdentities::Solana {
                solana_contract_address: FixedBytes::from([contract; 32]),
                solana_user_address: FixedBytes::from([user; 32]),
            },
            ciphetext_with_zk_proof: Bytes::from(vec![0xaa]),
            extra_data: Bytes::from(vec![0x00]),
        };
        let base = solana(1, 2).content_hash();
        assert_ne!(base, solana(9, 2).content_hash());
        assert_ne!(base, solana(1, 9).content_hash());
    }

    #[test]
    fn test_input_proof_request_content_hash_deterministic() {
        let request = InputProofRequest::new(
            1337,
            Address::from([1; 20]),
            Address::from([2; 20]),
            Bytes::from(vec![0xaa, 0xbb, 0xcc]),
            Bytes::from(vec![0x00]),
        );

        let hash1 = request.content_hash();
        let hash2 = request.content_hash();

        assert_eq!(hash1, hash2, "Same request should produce same hash");
        assert_eq!(hash1.len(), 32, "Hash should be 32 bytes");
    }

    #[test]
    fn test_input_proof_request_content_hash_different_for_different_requests() {
        let request1 = InputProofRequest::new(
            1337,
            Address::from([1; 20]),
            Address::from([2; 20]),
            Bytes::from(vec![0xaa, 0xbb]),
            Bytes::from(vec![0x00]),
        );

        let mut request2 = request1.clone();
        request2.contract_chain_id = 1338;

        let hash1 = request1.content_hash();
        let hash2 = request2.content_hash();

        assert_ne!(
            hash1, hash2,
            "Different requests should have different hashes"
        );
    }

    #[test]
    fn test_input_proof_request_each_field_affects_hash() {
        let base_request = InputProofRequest::new(
            1337,
            Address::from([1; 20]),
            Address::from([2; 20]),
            Bytes::from(vec![0xaa]),
            Bytes::from(vec![0x00]),
        );
        let base_hash = base_request.content_hash();

        // Test contract_chain_id
        let mut modified = base_request.clone();
        modified.contract_chain_id = 9999;
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "contract_chain_id should affect hash"
        );

        // Test contract_address
        let mut modified = base_request.clone();
        modified.identities = HostIdentities::Evm {
            contract_address: Address::from([99; 20]),
            user_address: Address::from([2; 20]),
        };
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "contract_address should affect hash"
        );

        // Test user_address
        let mut modified = base_request.clone();
        modified.identities = HostIdentities::Evm {
            contract_address: Address::from([1; 20]),
            user_address: Address::from([99; 20]),
        };
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "user_address should affect hash"
        );

        // Test ciphetext_with_zk_proof
        let mut modified = base_request.clone();
        modified.ciphetext_with_zk_proof = Bytes::from(vec![0xbb]);
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "ciphetext_with_zk_proof should affect hash"
        );

        // Test extra_data
        let mut modified = base_request.clone();
        modified.extra_data = Bytes::from(vec![0xff]);
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "extra_data should affect hash"
        );
    }

    #[test]
    fn test_input_proof_request_hash_format() {
        let request = InputProofRequest::new(
            1337,
            Address::from([1; 20]),
            Address::from([2; 20]),
            Bytes::from(vec![0xaa]),
            Bytes::from(vec![0x00]),
        );

        let hash = request.content_hash();

        // Verify it's 32 bytes
        assert_eq!(hash.len(), 32);

        // Verify it's not all zeros (would indicate problem with hashing)
        assert_ne!(hash, [0u8; 32]);
    }
}
