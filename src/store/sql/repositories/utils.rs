use sha2::{Digest, Sha256};

/// Computes PostgreSQL advisory lock ID from input bytes using SHA256.
///
/// Uses the first 8 bytes of SHA256 hash as a signed 64-bit integer for
/// pg_advisory_xact_lock(). Provides full i64 range with uniform distribution.
///
/// Note: In the extremely rare case of hash collisions (different inputs producing
/// the same lock ID), it only causes requests to share the same advisory lock,
/// serializing their execution. This doesn't affect correctness - just reduces
/// concurrency for those specific colliding requests.
pub fn compute_advisory_lock_id(input: &[u8]) -> i64 {
    let hash = Sha256::digest(input);
    let first_8_bytes: [u8; 8] = hash[..8].try_into().unwrap();
    i64::from_be_bytes(first_8_bytes) // Direct interpretation - full range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_advisory_lock_id_deterministic() {
        let input = b"test_input";
        let id1 = compute_advisory_lock_id(input);
        let id2 = compute_advisory_lock_id(input);
        assert_eq!(id1, id2, "Advisory lock ID should be deterministic");
    }

    #[test]
    fn test_advisory_lock_id_different_inputs() {
        let input1 = b"input1";
        let input2 = b"input2";
        let id1 = compute_advisory_lock_id(input1);
        let id2 = compute_advisory_lock_id(input2);
        assert_ne!(
            id1, id2,
            "Different inputs should produce different lock IDs"
        );
    }

    #[test]
    fn test_advisory_lock_id_full_range() {
        // Test that we can get both positive and negative values
        let mut has_positive = false;
        let mut has_negative = false;

        for i in 0..1000 {
            let input = format!("test_{}", i);
            let lock_id = compute_advisory_lock_id(input.as_bytes());
            if lock_id > 0 {
                has_positive = true;
            }
            if lock_id < 0 {
                has_negative = true;
            }
        }

        assert!(has_positive, "Should generate positive lock IDs");
        assert!(has_negative, "Should generate negative lock IDs");
    }
}
