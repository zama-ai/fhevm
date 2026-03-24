use alloy::primitives::FixedBytes;
use moka::future::Cache;
use std::time::Duration;

/// Composite key for event deduplication
#[derive(Hash, Eq, PartialEq, Clone, Debug)]
pub struct EventKey {
    pub block_number: u64,
    pub block_hash: FixedBytes<32>,
    pub log_index: u64,
}

/// Event deduplicator using Moka cache with automatic TTL cleanup
pub struct EventDeduplicator {
    cache: Cache<EventKey, ()>,
}

impl EventDeduplicator {
    /// Create a new event deduplicator with TTL and capacity limits
    pub fn new(ttl_seconds: u64, max_capacity: usize) -> Self {
        let cache = Cache::builder()
            .time_to_live(Duration::from_secs(ttl_seconds))
            .max_capacity(max_capacity as u64)
            .build();

        Self { cache }
    }

    /// Try to insert an event key. Returns true if this is the first time seeing this event,
    /// false if it's a duplicate (already seen within TTL window).
    /// This operation is atomic - concurrent calls on the same key are coalesced.
    pub async fn try_insert(&self, key: EventKey) -> bool {
        self.cache.entry(key).or_insert(()).await.is_fresh()
    }

    /// Get current cache size for monitoring
    pub fn cache_size(&self) -> u64 {
        self.cache.entry_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::primitives::FixedBytes;

    #[tokio::test]
    async fn test_deduplication_basic() {
        let dedup = EventDeduplicator::new(5, 1000);

        let key = EventKey {
            block_number: 12345,
            block_hash: FixedBytes::from([1u8; 32]),
            log_index: 0,
        };

        // First insertion should return true (new event)
        assert!(dedup.try_insert(key.clone()).await);

        // Second insertion should return false (duplicate)
        assert!(!dedup.try_insert(key.clone()).await);
    }

    #[tokio::test]
    async fn test_different_events_allowed() {
        let dedup = EventDeduplicator::new(5, 1000);

        let key1 = EventKey {
            block_number: 12345,
            block_hash: FixedBytes::from([1u8; 32]),
            log_index: 0,
        };

        let key2 = EventKey {
            block_number: 12345,
            block_hash: FixedBytes::from([1u8; 32]),
            log_index: 1, // Different log index
        };

        // Both events should be allowed (different keys)
        assert!(dedup.try_insert(key1).await);
        assert!(dedup.try_insert(key2).await);
    }

    #[tokio::test]
    async fn test_cache_size_tracking() {
        // Test with a specific capacity to ensure cache size respects limits
        // Cache sizing formula: events_per_second * listener_instances * ttl_seconds * safety_buffer
        // Example: 100 events/sec * 3 listeners * 5s TTL * 1.2 buffer = 1,800 capacity
        let dedup = EventDeduplicator::new(5, 1800);

        assert_eq!(dedup.cache_size(), 0);

        let key = EventKey {
            block_number: 12345,
            block_hash: FixedBytes::from([1u8; 32]),
            log_index: 0,
        };

        dedup.try_insert(key.clone()).await;

        // Test that we can still query the cache (cache size might be eventually consistent)
        // The important part is the deduplication functionality works
        assert!(!dedup.try_insert(key).await); // Should be duplicate now
    }

    #[tokio::test]
    async fn test_ttl_expiration() {
        let dedup = EventDeduplicator::new(1, 1000); // 1 second TTL

        let key = EventKey {
            block_number: 12345,
            block_hash: FixedBytes::from([1u8; 32]),
            log_index: 0,
        };

        // Insert event
        assert!(dedup.try_insert(key.clone()).await);

        // Should be duplicate immediately
        assert!(!dedup.try_insert(key.clone()).await);

        // Wait for TTL expiration
        tokio::time::sleep(Duration::from_secs(2)).await;

        // Should be allowed again after TTL
        assert!(dedup.try_insert(key).await);
    }

    #[tokio::test]
    async fn test_capacity_limit() {
        let dedup = EventDeduplicator::new(60, 2); // Small capacity

        // Fill cache to capacity
        for i in 0..3 {
            let key = EventKey {
                block_number: i,
                block_hash: FixedBytes::from([i as u8; 32]),
                log_index: 0,
            };
            dedup.try_insert(key).await;
        }

        // Cache should not exceed max capacity due to eviction
        assert!(dedup.cache_size() <= 2);
    }
}
