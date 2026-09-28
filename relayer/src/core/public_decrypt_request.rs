use crate::core::event::PublicDecryptRequest;
use crate::orchestrator::ContentHasher;
use sha2::{Digest, Sha256};

impl ContentHasher for PublicDecryptRequest {
    /// TODO: Consider canonical ordering for list items.
    fn content_hash(&self) -> [u8; 32] {
        let mut hasher = Sha256::new();

        hasher.update(b"ct_handles:"); // 1
        for handle in &self.ct_handles {
            hasher.update(handle);
        }

        hasher.update(b"extra_data:"); // 2
        hasher.update(&self.extra_data);

        // Only Solana requests carry stores, so EVM hashes are unchanged. The same handles
        // named with a different store are a different request.
        if !self.encrypted_stores.is_empty() {
            hasher.update(b"encrypted_stores:"); // 3
            for store in &self.encrypted_stores {
                hasher.update(store);
            }
        }

        hasher.finalize().into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::Bytes;

    #[test]
    fn test_public_decrypt_request_content_hash_deterministic() {
        let request = PublicDecryptRequest {
            ct_handles: vec![[1; 32], [2; 32]],
            extra_data: Bytes::from(vec![0xaa, 0xbb]),
            encrypted_stores: Vec::new(),
        };

        let id1 = request.content_hash();
        let id2 = request.content_hash();

        assert_eq!(id1, id2, "Same request should produce same content hash");
        assert_eq!(id1.len(), 32, "Content hash should be 32 bytes");
    }

    #[test]
    fn test_public_decrypt_request_content_hash_different_for_different_requests() {
        let request1 = PublicDecryptRequest {
            ct_handles: vec![[1; 32]],
            extra_data: Bytes::from(vec![0xaa]),
            encrypted_stores: Vec::new(),
        };

        let request2 = PublicDecryptRequest {
            ct_handles: vec![[2; 32]],
            extra_data: Bytes::from(vec![0xaa]),
            encrypted_stores: Vec::new(),
        };

        let id1 = request1.content_hash();
        let id2 = request2.content_hash();

        assert_ne!(
            id1, id2,
            "Different requests should produce different content hashes"
        );
    }

    #[test]
    fn the_same_handles_with_another_store_are_another_request() {
        let request = |store: u8| PublicDecryptRequest {
            ct_handles: vec![[1; 32]],
            extra_data: Bytes::from(vec![0x00]),
            encrypted_stores: vec![[store; 32]],
        };

        assert_ne!(request(0xaa).content_hash(), request(0xbb).content_hash());
    }

    #[test]
    fn an_evm_request_hashes_as_before_stores_existed() {
        let request = PublicDecryptRequest {
            ct_handles: vec![[1; 32]],
            extra_data: Bytes::from(vec![0x00]),
            encrypted_stores: Vec::new(),
        };
        let mut hasher = Sha256::new();
        hasher.update(b"ct_handles:");
        hasher.update([1; 32]);
        hasher.update(b"extra_data:");
        hasher.update([0x00]);
        let before: [u8; 32] = hasher.finalize().into();

        assert_eq!(request.content_hash(), before);
    }
}
