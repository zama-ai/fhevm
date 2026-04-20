use std::sync::Arc;
use thiserror::Error;
use tokio::sync::{Mutex, Notify};

/// A thread-safe, generic buffer for async slotting.
/// Optimized for: Parallel random-order insertion -> Sequential ordered reading.
#[derive(Clone)]
pub struct AsyncSlotBuffer<T> {
    slots: Arc<Vec<Slot<T>>>,
}

struct Slot<T> {
    // The data container. Mutex ensures visibility across threads.
    value: Mutex<Option<T>>,
    // The signaling mechanism for the cursor.
    notify: Notify,
}

#[derive(Error, Debug, PartialEq)]
pub enum BufferError {
    #[error("Index out of bounds")]
    IndexOutOfBounds,

    #[error("Index already filled")]
    AlreadyFilled,
}

impl<T: Clone + Send + 'static> AsyncSlotBuffer<T> {
    /// Initialize with fixed size. All slots start empty.
    pub fn new(size: usize) -> Self {
        let mut slots = Vec::with_capacity(size);
        for _ in 0..size {
            slots.push(Slot {
                value: Mutex::new(None),
                notify: Notify::new(),
            });
        }
        Self {
            slots: Arc::new(slots),
        }
    }

    /// PRODUCER (Strict): Sets the value ONLY if the slot is empty.
    /// Returns `Err(AlreadyFilled)` if data exists.
    pub async fn set_once(&self, index: usize, item: T) -> Result<(), BufferError> {
        let slot = self.slots.get(index).ok_or(BufferError::IndexOutOfBounds)?;

        {
            let mut guard = slot.value.lock().await;
            if guard.is_some() {
                return Err(BufferError::AlreadyFilled);
            }
            *guard = Some(item);
        }

        // Notify any waiting cursors
        slot.notify.notify_waiters();
        Ok(())
    }

    /// PRODUCER (Overwrite): Sets the value, replacing any existing data.
    /// Could be useful if a better version of a block is fetched or for correction.
    pub async fn set(&self, index: usize, item: T) -> Result<(), BufferError> {
        let slot = self.slots.get(index).ok_or(BufferError::IndexOutOfBounds)?;

        {
            let mut guard = slot.value.lock().await;
            // We unconditionally overwrite the data
            *guard = Some(item);
        }

        // We notify waiters.
        slot.notify.notify_waiters();
        Ok(())
    }

    /// CONSUMER: Gets the value at index.
    /// If empty: WAITS (Sleeps) until filled.
    /// If full: Returns immediately.
    /// Returns None only if index is out of bounds.
    pub async fn get(&self, index: usize) -> Option<T> {
        let slot = self.slots.get(index)?;

        // The Loop pattern ensures we don't miss notifications
        loop {
            let wait_for_fill = slot.notify.notified();
            {
                let guard = slot.value.lock().await;
                if let Some(val) = &*guard {
                    return Some(val.clone());
                }
            }
            wait_for_fill.await;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;
    use tokio::time::sleep;

    // Mock structure
    #[derive(Clone, Debug, PartialEq)]
    struct MockBlock {
        height: u64,
        hash: String,
        parent_hash: String,
    }

    impl MockBlock {
        fn new(height: u64, seed: &str, parent_seed: &str) -> Self {
            Self {
                height,
                hash: format!("hash_{}", seed),
                parent_hash: format!("hash_{}", parent_seed),
            }
        }
    }

    // SCENARIO 1: Simple Sequential Fill & Check
    #[tokio::test]
    async fn test_sequential_fill_and_read() {
        let buffer = AsyncSlotBuffer::<MockBlock>::new(3);
        let b0 = MockBlock::new(0, "0", "gen");
        let b1 = MockBlock::new(1, "1", "0");
        let b2 = MockBlock::new(2, "2", "1");

        buffer.set_once(1, b1.clone()).await.unwrap();
        buffer.set_once(2, b2.clone()).await.unwrap();
        buffer.set_once(0, b0.clone()).await.unwrap();

        let mut prev_hash = "hash_gen".to_string();
        for i in 0..3 {
            let block = buffer.get(i).await.expect("Bounds check");
            assert_eq!(block.parent_hash, prev_hash);
            prev_hash = block.hash;
        }
    }

    // SCENARIO 2: Parallel Producers (Random Order)
    #[tokio::test]
    async fn test_parallel_fill_random_latency() {
        let size = 10;
        let buffer = AsyncSlotBuffer::<MockBlock>::new(size);
        let mut previous_known_hash = "hash_old".to_string();

        for i in 0..size {
            let buf_clone = buffer.clone();
            let prev_seed = if i == 0 {
                "old".to_string()
            } else {
                format!("{}", i - 1)
            };

            tokio::spawn(async move {
                let delay = if i == 0 { 50 } else { 5 };
                sleep(Duration::from_millis(delay)).await;
                let block = MockBlock::new(i as u64, &i.to_string(), &prev_seed);
                buf_clone.set_once(i, block).await.unwrap();
            });
        }

        for i in 0..size {
            let block = buffer.get(i).await.unwrap();
            assert_eq!(block.parent_hash, previous_known_hash);
            previous_known_hash = block.hash;
        }
    }

    // SCENARIO 3: Bounds & strict Set
    #[tokio::test]
    async fn test_bounds_and_strict_set() {
        let buffer = AsyncSlotBuffer::<i32>::new(1);

        // Fill once
        assert_eq!(buffer.set_once(0, 100).await, Ok(()));

        // Fill again (Should Fail)
        assert_eq!(
            buffer.set_once(0, 200).await,
            Err(BufferError::AlreadyFilled)
        );

        // Check value is still original
        assert_eq!(buffer.get(0).await, Some(100));

        // Bounds
        assert_eq!(
            buffer.set_once(5, 10).await,
            Err(BufferError::IndexOutOfBounds)
        );
        assert_eq!(buffer.set(5, 10).await, Err(BufferError::IndexOutOfBounds));
    }

    // SCENARIO 4: Overwrite / Replace capability
    #[tokio::test]
    async fn test_overwrite_scenario() {
        let buffer = AsyncSlotBuffer::<String>::new(1);

        // 1. Set initial value
        buffer
            .set_once(0, "Original Data".to_string())
            .await
            .unwrap();

        // Verify
        let val = buffer.get(0).await.unwrap();
        assert_eq!(val, "Original Data");

        // 2. Replace value
        buffer
            .set(0, "New Corrected Data".to_string())
            .await
            .unwrap();

        // Verify the data inside is updated
        let val_new = buffer.get(0).await.unwrap();
        assert_eq!(val_new, "New Corrected Data");
    }

    // SCENARIO 5: External Predecessor Handover (The "Chain Tip" Scenario)
    // This tests using a data point OUTSIDE the struct to validate element 0.
    #[tokio::test]
    async fn test_external_predecessor_handover() {
        // 1. Context: We have a "Tip" from the database or previous iteration
        let chain_tip = MockBlock::new(99, "99", "98");
        let tip_hash = chain_tip.hash.clone(); // "hash_99"

        // 2. Setup Buffer for the NEXT range (100-102)
        let buffer = AsyncSlotBuffer::<MockBlock>::new(3);

        // 3. Fill the buffer (mimic parallel fetch)
        // Note: Block 100's parent MUST match the chain_tip hash ("hash_99")
        let b100 = MockBlock::new(100, "100", "99");
        let b101 = MockBlock::new(101, "101", "100");
        let b102 = MockBlock::new(102, "102", "101");

        let producer_buf = buffer.clone();
        tokio::spawn(async move {
            producer_buf.set_once(0, b100).await.unwrap();
            producer_buf.set_once(1, b101).await.unwrap();
            producer_buf.set_once(2, b102).await.unwrap();
        });

        // 4. Cursor Logic
        // Initialize the comparator with the EXTERNAL data
        let mut last_valid_hash = tip_hash;

        for i in 0..3 {
            let current_block = buffer.get(i).await.expect("Block exists");

            println!(
                "Checking Block {}: Expect Parent {} == {}",
                current_block.height, current_block.parent_hash, last_valid_hash
            );

            // This assertion works for Index 0 because `last_valid_hash`
            // was seeded with the external `chain_tip`
            assert_eq!(current_block.parent_hash, last_valid_hash);

            // Update for next loop
            last_valid_hash = current_block.hash.clone();
        }

        // Ensure we reached the end of this batch
        assert_eq!(last_valid_hash, "hash_102");
    }
}
