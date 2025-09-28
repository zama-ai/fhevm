use crate::core::event::UserDecryptResponse;
use alloy::primitives::{Bytes, U256};
use anyhow::{anyhow, Result};
use dashmap::DashMap;
use tracing::{debug, info, warn};

/// Individual user decryption response share from gateway
#[derive(Debug, Clone)]
pub struct UserDecryptionResponseShare {
    pub decryption_id: U256,
    pub index_share: U256,
    pub user_decrypted_share: Bytes,
    pub signature: Bytes,
    pub extra_data: Bytes,
}

/// Internal structure for collecting individual shares before assembly
#[derive(Debug, Clone)]
struct DecryptionResponseCollection {
    pub shares: DashMap<U256, UserDecryptionResponseShare>, // index_share -> share
    pub expected_count: usize,
    pub consensus_reached: bool,
}

impl DecryptionResponseCollection {
    fn new(expected_count: usize) -> Self {
        Self {
            shares: DashMap::new(),
            expected_count,
            consensus_reached: false,
        }
    }

    fn is_complete(&self) -> bool {
        self.shares.len() >= self.expected_count && self.consensus_reached
    }

    fn assemble_final_response(&self) -> Option<UserDecryptResponse> {
        if !self.is_complete() {
            return None;
        }

        // Sort shares by index_share to maintain order
        let mut shares_vec: Vec<_> = self
            .shares
            .iter()
            .map(|entry| entry.value().clone())
            .collect();
        shares_vec.sort_by_key(|share| share.index_share);

        if shares_vec.is_empty() {
            warn!("No shares found when assembling final response");
            return None;
        }

        let first_share = &shares_vec[0];
        let decryption_id = first_share.decryption_id;

        // Extract reencrypted_shares and signatures in order
        let reencrypted_shares: Vec<Bytes> = shares_vec
            .iter()
            .map(|share| share.user_decrypted_share.clone())
            .collect();

        let signatures: Vec<Bytes> = shares_vec
            .iter()
            .map(|share| share.signature.clone())
            .collect();

        // Use extra_data from first share (should be consistent across all shares)
        let extra_data = first_share.extra_data.clone();

        Some(UserDecryptResponse {
            gateway_request_id: decryption_id,
            reencrypted_shares,
            signatures,
            extra_data,
        })
    }
}

/// Store for collecting and assembling multi-response user decryption responses
///
/// This store handles the new gateway user decryption flow where responses arrive as:
/// - N individual UserDecryptionResponse events (one per share)
/// - 1 UserDecryptionResponseThresholdReached consensus event
///
/// Thread-safety is achieved through fine-grained DashMap locking per decryption_id.
pub struct UserDecryptResponseStore {
    /// Per-decryption collections for assembling individual shares
    collections: DashMap<U256, DecryptionResponseCollection>,
    /// Expected number of shares per decryption request
    expected_share_count: usize,
}

impl UserDecryptResponseStore {
    pub fn new(expected_share_count: usize) -> Self {
        Self {
            collections: DashMap::new(),
            expected_share_count,
        }
    }

    /// Add an individual response share to the collection
    ///
    /// Returns Ok(None) if more shares or consensus are needed
    /// Returns Ok(Some(response)) if the final response is ready
    /// Returns Err if there's an error processing the share
    pub fn add_response(
        &self,
        share: UserDecryptionResponseShare,
    ) -> Result<Option<UserDecryptResponse>> {
        let decryption_id = share.decryption_id;
        let index_share = share.index_share;

        debug!(
            decryption_id = %decryption_id,
            index_share = %index_share,
            "Adding individual response share"
        );

        // Get or create collection for this decryption_id
        let collection = self
            .collections
            .entry(decryption_id)
            .or_insert_with(|| DecryptionResponseCollection::new(self.expected_share_count));

        // Check for duplicate shares
        if collection.shares.contains_key(&index_share) {
            warn!(
                decryption_id = %decryption_id,
                index_share = %index_share,
                "Received duplicate share for decryption"
            );
            return Err(anyhow!(
                "Duplicate share received for index {}",
                index_share
            ));
        }

        // Add the share
        collection.shares.insert(index_share, share);

        info!(
            decryption_id = %decryption_id,
            current_shares = collection.shares.len(),
            expected_shares = collection.expected_count,
            consensus_reached = collection.consensus_reached,
            "Added response share"
        );

        // Check if we can assemble the final response
        if collection.is_complete() {
            debug!(
                decryption_id = %decryption_id,
                "All conditions met, assembling final response"
            );
            return Ok(collection.assemble_final_response());
        }

        Ok(None)
    }

    /// Mark consensus as reached for a decryption_id
    ///
    /// Returns Ok(None) if more shares are needed
    /// Returns Ok(Some(response)) if the final response is ready
    /// Returns Err if there's an error processing the consensus
    pub fn mark_consensus(&self, decryption_id: U256) -> Result<Option<UserDecryptResponse>> {
        debug!(
            decryption_id = %decryption_id,
            "Marking consensus reached"
        );

        // Get or create collection for this decryption_id
        let mut collection = self
            .collections
            .entry(decryption_id)
            .or_insert_with(|| DecryptionResponseCollection::new(self.expected_share_count));

        collection.consensus_reached = true;

        info!(
            decryption_id = %decryption_id,
            current_shares = collection.shares.len(),
            expected_shares = collection.expected_count,
            "Consensus reached"
        );

        // Check for missing shares at consensus time
        if collection.shares.len() < collection.expected_count {
            warn!(
                decryption_id = %decryption_id,
                current_shares = collection.shares.len(),
                expected_shares = collection.expected_count,
                "Consensus reached but missing shares"
            );
        }

        // Check if we can assemble the final response
        if collection.is_complete() {
            debug!(
                decryption_id = %decryption_id,
                "All conditions met after consensus, assembling final response"
            );
            return Ok(collection.assemble_final_response());
        }

        Ok(None)
    }

    /// Clean up completed or failed decryption collections
    ///
    /// Should be called after successfully dispatching events and persisting to cache
    pub fn cleanup(&self, decryption_id: U256) {
        if let Some((_, collection)) = self.collections.remove(&decryption_id) {
            debug!(
                decryption_id = %decryption_id,
                shares_collected = collection.shares.len(),
                "Cleaned up decryption collection"
            );
        }
    }

    /// Get current status for debugging purposes
    pub fn get_status(&self, decryption_id: U256) -> Option<(usize, usize, bool)> {
        self.collections.get(&decryption_id).map(|collection| {
            (
                collection.shares.len(),
                collection.expected_count,
                collection.consensus_reached,
            )
        })
    }

    /// Get count of active collections (for monitoring purposes)
    pub fn active_collections_count(&self) -> usize {
        self.collections.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::{Bytes, U256};

    fn create_test_share(
        decryption_id: U256,
        index_share: U256,
        share_data: Vec<u8>,
    ) -> UserDecryptionResponseShare {
        UserDecryptionResponseShare {
            decryption_id,
            index_share,
            user_decrypted_share: Bytes::from(share_data),
            signature: Bytes::from(vec![0xaa, 0xbb, 0xcc]),
            extra_data: Bytes::from(vec![0x00]),
        }
    }

    #[test]
    fn test_single_share_no_consensus() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        let share = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);
        let result = store.add_response(share).unwrap();

        assert!(
            result.is_none(),
            "Should not return response with only one share"
        );

        let (current, expected, consensus) = store.get_status(decryption_id).unwrap();
        assert_eq!(current, 1);
        assert_eq!(expected, 2);
        assert!(!consensus);
    }

    #[test]
    fn test_all_shares_no_consensus() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        let share1 = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);
        let share2 = create_test_share(decryption_id, U256::from(1), vec![0x03, 0x04]);

        assert!(store.add_response(share1).unwrap().is_none());
        assert!(store.add_response(share2).unwrap().is_none());

        let (current, expected, consensus) = store.get_status(decryption_id).unwrap();
        assert_eq!(current, 2);
        assert_eq!(expected, 2);
        assert!(!consensus);
    }

    #[test]
    fn test_consensus_no_shares() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        let result = store.mark_consensus(decryption_id).unwrap();
        assert!(
            result.is_none(),
            "Should not return response without shares"
        );

        let (current, expected, consensus) = store.get_status(decryption_id).unwrap();
        assert_eq!(current, 0);
        assert_eq!(expected, 2);
        assert!(consensus);
    }

    #[test]
    fn test_complete_flow_shares_first() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        // Add shares first
        let share1 = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);
        let share2 = create_test_share(decryption_id, U256::from(1), vec![0x03, 0x04]);

        assert!(store.add_response(share1).unwrap().is_none());
        assert!(store.add_response(share2).unwrap().is_none());

        // Add consensus - should trigger final response
        let result = store.mark_consensus(decryption_id).unwrap();
        assert!(result.is_some(), "Should return final response");

        let response = result.unwrap();
        assert_eq!(response.gateway_request_id, decryption_id);
        assert_eq!(response.reencrypted_shares.len(), 2);
        assert_eq!(response.signatures.len(), 2);

        // Verify ordering by index_share
        assert_eq!(
            response.reencrypted_shares[0],
            Bytes::from(&[0x01, 0x02][..])
        );
        assert_eq!(
            response.reencrypted_shares[1],
            Bytes::from(&[0x03, 0x04][..])
        );
    }

    #[test]
    fn test_complete_flow_consensus_first() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        // Mark consensus first
        assert!(store.mark_consensus(decryption_id).unwrap().is_none());

        // Add shares
        let share1 = create_test_share(decryption_id, U256::from(1), vec![0x03, 0x04]);
        let share2 = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);

        assert!(store.add_response(share1).unwrap().is_none());
        let result = store.add_response(share2).unwrap();

        assert!(result.is_some(), "Should return final response");

        let response = result.unwrap();
        assert_eq!(response.gateway_request_id, decryption_id);
        assert_eq!(response.reencrypted_shares.len(), 2);

        // Verify ordering is correct (sorted by index_share)
        assert_eq!(
            response.reencrypted_shares[0],
            Bytes::from(&[0x01, 0x02][..])
        );
        assert_eq!(
            response.reencrypted_shares[1],
            Bytes::from(&[0x03, 0x04][..])
        );
    }

    #[test]
    fn test_duplicate_share_error() {
        let store = UserDecryptResponseStore::new(2);
        let decryption_id = U256::from(1);

        let share1 = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);
        let share1_duplicate = create_test_share(decryption_id, U256::from(0), vec![0x05, 0x06]);

        assert!(store.add_response(share1).unwrap().is_none());
        let result = store.add_response(share1_duplicate);

        assert!(result.is_err(), "Should error on duplicate share");
        assert!(result.unwrap_err().to_string().contains("Duplicate share"));
    }

    #[test]
    fn test_cleanup() {
        let store = UserDecryptResponseStore::new(1);
        let decryption_id = U256::from(1);

        let share = create_test_share(decryption_id, U256::from(0), vec![0x01, 0x02]);
        store.add_response(share).unwrap();

        assert_eq!(store.active_collections_count(), 1);

        store.cleanup(decryption_id);
        assert_eq!(store.active_collections_count(), 0);
        assert!(store.get_status(decryption_id).is_none());
    }
}
