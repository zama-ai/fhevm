//! Sharding utilities for request filtering
//!
//! This module provides functionality to determine which shard should handle
//! a given request based on request ID and shard configuration.

use crate::gw_adapters::events::KmsCoreEvent;
use tracing::debug;

/// Determines if this shard should handle the given request ID using modulo-based sharding
///
/// # Arguments
/// * `request_id` - The request ID to check
/// * `shard_id` - The ID of this shard (0-based)
/// * `total_shards` - Total number of shards
///
/// # Returns
/// `true` if this shard should handle the request, `false` otherwise
pub fn should_handle_request(request_id: u64, shard_id: u32, total_shards: u32) -> bool {
    // Use a simple hash to ensure better distribution even if request IDs have patterns
    let hash = request_id.wrapping_mul(0x9e3779b97f4a7c15_u64);
    let assigned_shard = (hash % total_shards as u64) as u32;

    debug!(
        "Request {} -> hash {} -> shard {} (we are shard {})",
        request_id, hash, assigned_shard, shard_id
    );

    assigned_shard == shard_id
}

/// Extract request ID from different event types
///
/// # Arguments
/// * `event` - The KmsCoreEvent to extract request ID from
///
/// # Returns
/// `Some(request_id)` if the event contains a request ID, `None` otherwise
pub fn extract_request_id(event: &KmsCoreEvent) -> Option<u64> {
    match event {
        KmsCoreEvent::PublicDecryptionRequest(req) => {
            // Convert U256 to u64 by taking the lower 64 bits
            Some(req.decryptionId.to::<u64>())
        }
        KmsCoreEvent::UserDecryptionRequest(req) => {
            // Convert U256 to u64 by taking the lower 64 bits
            Some(req.decryptionId.to::<u64>())
        }
        KmsCoreEvent::KeygenRequest(req) => {
            // Use preKeyId for keygen requests
            Some(req.preKeyId.to::<u64>())
        }
        KmsCoreEvent::CrsgenRequest(req) => {
            // Use crsgenRequestId for crsgen requests
            Some(req.crsgenRequestId.to::<u64>())
        }
        KmsCoreEvent::KskgenRequest(req) => {
            // Use preKskId for kskgen requests
            Some(req.preKskId.to::<u64>())
        }
        // Response events and other events don't need sharding
        _ => None,
    }
}

/// Check if sharding is enabled based on configuration
///
/// # Arguments
/// * `shard_id` - Optional shard ID from configuration
/// * `total_shards` - Optional total shards from configuration
///
/// # Returns
/// `Some((shard_id, total_shards))` if sharding is enabled, `None` otherwise
pub fn get_shard_config(shard_id: Option<u32>, total_shards: Option<u32>) -> Option<(u32, u32)> {
    match (shard_id, total_shards) {
        (Some(shard_id), Some(total_shards)) => Some((shard_id, total_shards)),
        _ => None,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use fhevm_gateway_rust_bindings::decryption::Decryption;

    #[test]
    fn test_should_handle_request_distribution() {
        // Test that requests are distributed reasonably evenly across shards
        let total_shards = 3;
        let mut shard_counts = vec![0; total_shards as usize];

        // Test 300 request IDs
        for request_id in 0..300 {
            for shard_id in 0..total_shards {
                if should_handle_request(request_id, shard_id, total_shards) {
                    shard_counts[shard_id as usize] += 1;
                }
            }
        }

        // Verify total requests processed
        let total_processed: usize = shard_counts.iter().sum();
        assert_eq!(total_processed, 300, "Total requests should equal input");

        // Each shard should handle roughly 100 requests (allow ±10% variance for hash distribution)
        let expected = 100;
        let tolerance = 10; // 10% tolerance
        for (shard_id, count) in shard_counts.iter().enumerate() {
            assert!(
                *count >= expected - tolerance && *count <= expected + tolerance,
                "Shard {shard_id} handled {count} requests, expected {expected}±{tolerance}"
            );
        }

        println!("Distribution: {shard_counts:?}");
    }

    #[test]
    fn test_should_handle_request_deterministic() {
        // Same request ID should always go to same shard
        let request_id = 12345;
        let total_shards = 5;

        let mut assigned_shard = None;
        for shard_id in 0..total_shards {
            if should_handle_request(request_id, shard_id, total_shards) {
                assert!(
                    assigned_shard.is_none(),
                    "Request assigned to multiple shards"
                );
                assigned_shard = Some(shard_id);
            }
        }

        assert!(
            assigned_shard.is_some(),
            "Request not assigned to any shard"
        );

        // Should be consistent across multiple calls
        let first_assigned = assigned_shard.unwrap();
        for _ in 0..10 {
            assert!(should_handle_request(
                request_id,
                first_assigned,
                total_shards
            ));
            for other_shard in 0..total_shards {
                if other_shard != first_assigned {
                    assert!(!should_handle_request(
                        request_id,
                        other_shard,
                        total_shards
                    ));
                }
            }
        }
    }

    #[test]
    fn test_extract_request_id() {
        use alloy::primitives::U256;

        // Test PublicDecryptionRequest
        let public_req = Decryption::PublicDecryptionRequest {
            decryptionId: U256::from(123),
            ..Default::default()
        };
        let event = KmsCoreEvent::PublicDecryptionRequest(public_req);
        assert_eq!(extract_request_id(&event), Some(123));

        // Test UserDecryptionRequest
        let user_req = Decryption::UserDecryptionRequest {
            decryptionId: U256::from(456),
            ..Default::default()
        };
        let event = KmsCoreEvent::UserDecryptionRequest(user_req);
        assert_eq!(extract_request_id(&event), Some(456));

        // Test response events (should return None)
        let response = Decryption::PublicDecryptionResponse {
            decryptionId: U256::from(789),
            ..Default::default()
        };
        let event = KmsCoreEvent::PublicDecryptionResponse(response);
        assert_eq!(extract_request_id(&event), None);
    }

    #[test]
    fn test_get_shard_config() {
        // Both values provided
        assert_eq!(get_shard_config(Some(1), Some(3)), Some((1, 3)));

        // Only one value provided
        assert_eq!(get_shard_config(Some(1), None), None);
        assert_eq!(get_shard_config(None, Some(3)), None);

        // No values provided
        assert_eq!(get_shard_config(None, None), None);
    }
}
