//! Preallocated buffers for downloaded ciphertexts.
//!
//! One memory region is allocated at startup and split into fixed-size buffers. Every ciphertext body is written into
//! one of them, so ciphertext memory is bounded by the region's size. When all buffers are in use, new requests wait
//! for enough buffers to be released (a request reserves all the buffers it needs in one go).

use crate::core::Config;
use anyhow::anyhow;
use bytes::Bytes;
use std::{
    num::NonZeroUsize,
    sync::{Arc, Mutex, PoisonError},
};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};
use tracing::info;

pub struct CiphertextPool {
    capacity: usize,
    /// Buffers not held by a `Slot`.
    buffers: Mutex<Vec<&'static mut [u8]>>,
    /// One permit per buffer. Waiting for permits is how requests wait for buffers.
    permits: Arc<Semaphore>,
}

impl CiphertextPool {
    /// Builds the pool described by `config`, rejecting one too small for the largest request.
    pub fn from_config(config: &Config) -> anyhow::Result<Arc<Self>> {
        if config.ciphertext_pool_slots < config.max_handles_per_decryption {
            return Err(anyhow!(
                "ciphertext_pool_slots ({}) must be at least max_handles_per_decryption ({})",
                config.ciphertext_pool_slots,
                config.max_handles_per_decryption
            ));
        }
        let pool = Self::new(config.ciphertext_pool_slots, config.s3_max_ciphertext_size);
        info!(
            "Allocated {} ciphertext buffers of {} bytes",
            config.ciphertext_pool_slots, config.s3_max_ciphertext_size
        );
        Ok(pool)
    }

    /// Allocates one region of `slots × slot_size` bytes and splits it into buffers.
    ///
    /// The region is leaked: it lives as long as the process, and leaking it is what lets each
    /// buffer be a plain `&'static mut [u8]`. Its pages are written immediately so the memory is
    /// committed now (a fresh zeroed allocation may only get pages on first write), and a pool
    /// that doesn't fit fails at startup. `black_box` keeps the compiler from dropping the writes.
    pub(super) fn new(slots: NonZeroUsize, slot_size: NonZeroUsize) -> Arc<Self> {
        let len = slots
            .get()
            .checked_mul(slot_size.get())
            .expect("pool size overflows usize");
        // Leaking tells the compiler that this ref is never freed, so this ref is valid for the whole lifetime of the process.
        let region: &'static mut [u8] = Box::leak(vec![0u8; len].into_boxed_slice());
        // Write one byte to each memory page, forcing the kernel to back the memory with real memory.
        // 4kb is the smallest pages size on the targets we run on; it's the safe lower bound.
        for byte in region.iter_mut().step_by(4096) {
            *byte = std::hint::black_box(0);
        }
        // The region need not start on a page boundary, so its last bytes can sit on a page the
        // loop skipped.
        region[len - 1] = std::hint::black_box(0);

        Arc::new(Self {
            capacity: slots.get(),
            buffers: Mutex::new(region.chunks_mut(slot_size.get()).collect()),
            permits: Arc::new(Semaphore::new(slots.get())),
        })
    }

    /// Number of buffers not handed out.
    #[cfg(test)]
    pub(super) fn available(&self) -> usize {
        self.permits.available_permits()
    }

    /// Takes `needed` buffers at once, waiting until they are all free. Waiters are served in
    /// order (tokio's `Semaphore` is fair), so large requests aren't starved by small ones.
    pub async fn reserve(self: &Arc<Self>, needed: usize) -> anyhow::Result<Vec<Slot>> {
        let too_small = || {
            anyhow!(
                "request needs {needed} ciphertext buffers, but the pool only has {}",
                self.capacity
            )
        };
        if needed > self.capacity {
            return Err(too_small());
        }
        let mut permits = Arc::clone(&self.permits)
            .acquire_many_owned(u32::try_from(needed).map_err(|_| too_small())?)
            .await
            .expect("pool semaphore is never closed");

        let mut buffers = self.buffers.lock().unwrap_or_else(PoisonError::into_inner);
        Ok((0..needed)
            .map(|_| Slot {
                buffer: buffers
                    .pop()
                    .expect("permits acquired above; a permit always has a buffer"),
                len: 0,
                pool: Arc::clone(self),
                _permit: permits.split(1).expect("one permit per reserved buffer"),
            })
            .collect())
    }
}

/// A buffer taken from the pool. It goes back to the pool when the slot is dropped.
///
/// Slots are filled during download, then turned into an immutable, ref counted `Bytes`. When the last clone of the
/// `Bytes` is dropped, the slot's `drop` impl runs and returns the memory to the pool. The rest of the pipeline only ever
/// sees a `Bytes` and doesn't need to know about the pool.
pub struct Slot {
    buffer: &'static mut [u8],
    len: usize,
    pool: Arc<CiphertextPool>,
    // Each slot holds its own permit, split off the request's reservation, so its buffer goes
    // back to the pool when its own ciphertext is dropped rather than when the whole request is.
    _permit: OwnedSemaphorePermit,
}

impl Slot {
    /// The whole buffer, for a download to write into.
    pub fn buffer_mut(&mut self) -> &mut [u8] {
        self.buffer
    }

    /// Turns the first `len` bytes into `Bytes` without copying them. The buffer returns to the
    /// pool when the last clone is dropped.
    pub fn into_bytes(mut self, len: usize) -> Bytes {
        assert!(len <= self.buffer.len(), "body longer than its buffer");
        self.len = len;
        Bytes::from_owner(self)
    }
}

impl AsRef<[u8]> for Slot {
    fn as_ref(&self) -> &[u8] {
        &self.buffer[..self.len]
    }
}

impl Drop for Slot {
    fn drop(&mut self) {
        let buffer = std::mem::take(&mut self.buffer);
        // A poisoned lock still holds a valid list: push and pop either complete or don't run.
        // Panicking here instead would abort the process if the slot is dropped while unwinding.
        let mut buffers = self
            .pool
            .buffers
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        buffers.push(buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    fn pool(slots: usize) -> Arc<CiphertextPool> {
        let slot_size = 16.try_into().unwrap();
        let slots = slots.try_into().unwrap();
        CiphertextPool::new(slots, slot_size)
    }

    fn filled(mut slot: Slot, body: &[u8]) -> Bytes {
        slot.buffer_mut()[..body.len()].copy_from_slice(body);
        slot.into_bytes(body.len())
    }

    #[tokio::test]
    async fn a_request_larger_than_the_pool_is_refused() {
        let pool = pool(2);
        assert!(pool.reserve(3).await.is_err());
        assert_eq!(pool.available(), 2);
    }

    #[tokio::test]
    async fn the_buffer_returns_when_the_last_bytes_clone_drops() {
        let pool = pool(1);
        let slot = pool.reserve(1).await.unwrap().pop().unwrap();
        let bytes = filled(slot, b"ct");
        let clone = bytes.clone();

        drop(bytes);
        assert_eq!(pool.available(), 0);
        drop(clone);
        assert_eq!(pool.available(), 1);
    }

    #[tokio::test]
    async fn slots_are_released_one_by_one() {
        let pool = pool(3);
        let mut slots = pool.reserve(3).await.unwrap();
        slots.truncate(1);
        assert_eq!(pool.available(), 2);

        drop(slots);
        assert_eq!(pool.available(), 3);
    }

    #[tokio::test]
    async fn a_reservation_waits_until_all_its_buffers_are_free() {
        let pool = pool(3);
        let first = pool.reserve(2).await.unwrap();

        let second = tokio::spawn({
            let pool = Arc::clone(&pool);
            async move { pool.reserve(2).await.map(drop) }
        });
        tokio::time::sleep(Duration::from_millis(50)).await;
        assert!(!second.is_finished(), "only one buffer is free");

        drop(first);
        tokio::time::timeout(Duration::from_secs(1), second)
            .await
            .unwrap()
            .unwrap()
            .unwrap();
    }

    #[tokio::test]
    async fn a_reused_buffer_exposes_only_the_new_body() {
        let pool = pool(1);
        let slot = pool.reserve(1).await.unwrap().pop().unwrap();
        drop(filled(slot, b"previous body"));

        let slot = pool.reserve(1).await.unwrap().pop().unwrap();
        assert_eq!(&filled(slot, b"new")[..], b"new");
    }
}
