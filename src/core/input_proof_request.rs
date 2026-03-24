use crate::core::event::InputProofRequest;
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

        hasher.update(b"contract_address:");
        hasher.update(self.contract_address.as_slice());

        hasher.update(b"user_address:");
        hasher.update(self.user_address.as_slice());

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
    use alloy::primitives::{Address, Bytes};

    #[test]
    fn test_input_proof_request_content_hash_deterministic() {
        let request = InputProofRequest {
            contract_chain_id: 1337,
            contract_address: Address::from([1; 20]),
            user_address: Address::from([2; 20]),
            ciphetext_with_zk_proof: Bytes::from(vec![0xaa, 0xbb, 0xcc]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let hash1 = request.content_hash();
        let hash2 = request.content_hash();

        assert_eq!(hash1, hash2, "Same request should produce same hash");
        assert_eq!(hash1.len(), 32, "Hash should be 32 bytes");
    }

    #[test]
    fn test_input_proof_request_content_hash_different_for_different_requests() {
        let request1 = InputProofRequest {
            contract_chain_id: 1337,
            contract_address: Address::from([1; 20]),
            user_address: Address::from([2; 20]),
            ciphetext_with_zk_proof: Bytes::from(vec![0xaa, 0xbb]),
            extra_data: Bytes::from(vec![0x00]),
        };

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
        let base_request = InputProofRequest {
            contract_chain_id: 1337,
            contract_address: Address::from([1; 20]),
            user_address: Address::from([2; 20]),
            ciphetext_with_zk_proof: Bytes::from(vec![0xaa]),
            extra_data: Bytes::from(vec![0x00]),
        };
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
        modified.contract_address = Address::from([99; 20]);
        assert_ne!(
            base_hash,
            modified.content_hash(),
            "contract_address should affect hash"
        );

        // Test user_address
        let mut modified = base_request.clone();
        modified.user_address = Address::from([99; 20]);
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
        let request = InputProofRequest {
            contract_chain_id: 1337,
            contract_address: Address::from([1; 20]),
            user_address: Address::from([2; 20]),
            ciphetext_with_zk_proof: Bytes::from(vec![0xaa]),
            extra_data: Bytes::from(vec![0x00]),
        };

        let hash = request.content_hash();

        // Verify it's 32 bytes
        assert_eq!(hash.len(), 32);

        // Verify it's not all zeros (would indicate problem with hashing)
        assert_ne!(hash, [0u8; 32]);
    }
}
