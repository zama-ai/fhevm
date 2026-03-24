use uuid::Uuid;

/// Generates unique, time-ordered request IDs that are safe to use concurrently.
pub fn new_internal_request_id() -> Uuid {
    Uuid::now_v7()
}

/// Generates random external reference IDs for client-facing operations.
pub fn new_external_reference_id() -> Uuid {
    Uuid::new_v4()
}

/// Trait for generating deterministic content hashes using SHA-256 hashing.
/// Used for content-based deduplication of requests with identical payloads.
/// Implementations should process fields in a consistent order to ensure
/// deterministic hash generation across different instances.
pub trait ContentHasher {
    /// Computes a deterministic SHA-256 hash of the implementing type's content.
    fn content_hash(&self) -> [u8; 32];
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::future;
    use std::collections::HashSet;
    use tokio::time::{sleep, Duration};

    #[test]
    fn test_sequential_uniqueness() {
        let request_ids: Vec<Uuid> = (0..100).map(|_| new_internal_request_id()).collect();
        assert_eq!(
            request_ids.iter().collect::<HashSet<_>>().len(),
            request_ids.len()
        );
    }

    #[tokio::test]
    async fn test_concurrent_uniqueness() {
        let tasks: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async move {
                    (0..100)
                        .map(|_| new_internal_request_id())
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        let all_request_ids: Vec<Uuid> = future::join_all(tasks)
            .await
            .into_iter()
            .flat_map(|result| result.unwrap())
            .collect();

        // All 10,000 request IDs should be unique
        assert_eq!(all_request_ids.len(), 10_000);
        assert_eq!(all_request_ids.iter().collect::<HashSet<_>>().len(), 10_000);
    }

    #[test]
    fn test_sequential_ids_sort_correctly() {
        let request_ids: Vec<Uuid> = (0..100).map(|_| new_internal_request_id()).collect();
        let id_strings: Vec<String> = request_ids.iter().map(|id| id.to_string()).collect();
        let mut sorted = id_strings.clone();
        sorted.sort();
        assert_eq!(id_strings, sorted);
    }

    #[tokio::test]
    async fn test_delayed_ids_sort_correctly() {
        let mut request_ids = Vec::new();
        for _ in 0..10 {
            request_ids.push(new_internal_request_id());
            sleep(Duration::from_millis(2)).await;
        }

        let id_strings: Vec<String> = request_ids.iter().map(|id| id.to_string()).collect();
        let mut sorted = id_strings.clone();
        sorted.sort();
        assert_eq!(id_strings, sorted);
    }

    #[test]
    fn test_external_reference_uniqueness() {
        let reference_ids: Vec<Uuid> = (0..1000).map(|_| new_external_reference_id()).collect();
        assert_eq!(
            reference_ids.iter().collect::<HashSet<_>>().len(),
            reference_ids.len()
        );
    }

    #[tokio::test]
    async fn test_external_reference_concurrent_uniqueness() {
        let tasks: Vec<_> = (0..100)
            .map(|_| {
                tokio::spawn(async move {
                    (0..100)
                        .map(|_| new_external_reference_id())
                        .collect::<Vec<_>>()
                })
            })
            .collect();

        let all_reference_ids: Vec<Uuid> = future::join_all(tasks)
            .await
            .into_iter()
            .flat_map(|result| result.unwrap())
            .collect();

        // All 10,000 reference IDs should be unique
        assert_eq!(all_reference_ids.len(), 10_000);
        assert_eq!(
            all_reference_ids.iter().collect::<HashSet<_>>().len(),
            10_000
        );
    }

    #[test]
    fn test_external_reference_randomness() {
        let reference_ids: Vec<Uuid> = (0..100).map(|_| new_external_reference_id()).collect();
        let id_strings: Vec<String> = reference_ids.iter().map(|id| id.to_string()).collect();

        // UUID v4 should not be ordered - sorted order should differ from generated order
        let mut sorted = id_strings.clone();
        sorted.sort();
        assert_ne!(id_strings, sorted, "UUID v4 should be random, not ordered");
    }

    #[test]
    fn test_external_reference_version() {
        let reference_id = new_external_reference_id();
        // UUID v4 should have version bits set to 0b0100 (4) in the most significant 4 bits of octet 6
        let bytes = reference_id.as_bytes();
        let version = (bytes[6] & 0xF0) >> 4;
        assert_eq!(version, 4, "Should be UUID version 4");
    }

    #[test]
    fn test_external_reference_variant() {
        let reference_id = new_external_reference_id();
        // RFC 4122 variant should have bits 10xx in the most significant bits of octet 8
        let bytes = reference_id.as_bytes();
        let variant_bits = (bytes[8] & 0xC0) >> 6;
        assert_eq!(variant_bits, 0b10, "Should have RFC 4122 variant bits");
    }

    #[test]
    fn test_external_reference_distribution() {
        // Test that generated UUIDs have good distribution in their random hex characters
        // Note: UUID v4 has fixed version (4) and variant bits, so we exclude those positions
        let reference_ids: Vec<Uuid> = (0..1000).map(|_| new_external_reference_id()).collect();

        // Extract only the random hex characters (exclude version and variant positions)
        let random_hex_chars: Vec<char> = reference_ids
            .iter()
            .flat_map(|uuid| {
                let uuid_str = uuid.to_string();
                let chars: Vec<char> = uuid_str.chars().filter(|c| *c != '-').collect();
                // UUID format: xxxxxxxx-xxxx-4xxx-yxxx-xxxxxxxxxxxx (32 hex chars total)
                // Position 12 is version (always 4), position 16 is variant (8,9,a,b)
                // Extract all positions except version at index 12
                let mut random_chars = Vec::new();
                for (i, &c) in chars.iter().enumerate() {
                    if i != 12 {
                        // Skip version position
                        random_chars.push(c);
                    }
                }
                random_chars
            })
            .collect();

        // Count frequency of each hex digit in random positions
        let mut counts = [0; 16];
        for c in random_hex_chars.iter() {
            if let Some(digit) = c.to_digit(16) {
                counts[digit as usize] += 1;
            }
        }

        // With 1000 UUIDs * 31 random hex chars = 31000 total random chars
        // Each digit should appear roughly 31000/16 = 1937.5 times
        // Allow reasonable variance for random distribution
        for (digit, count) in counts.iter().enumerate() {
            assert!(
                *count > 1400 && *count < 2500,
                "Hex digit {:x} appears {} times in random positions, expected ~1937 (range 1400-2500)",
                digit,
                count
            );
        }

        // Separately verify that version digit '4' appears exactly 1000 times (once per UUID)
        let version_fours = reference_ids
            .iter()
            .map(|uuid| {
                let chars: Vec<char> = uuid.to_string().chars().filter(|c| *c != '-').collect();
                chars[12] // Version position
            })
            .filter(|&c| c == '4')
            .count();

        assert_eq!(
            version_fours, 1000,
            "Version digit '4' should appear exactly once per UUID"
        );
    }
}
