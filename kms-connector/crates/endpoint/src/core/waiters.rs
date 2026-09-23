//! Registry of the HTTP connections waiting for a decryption response.

use alloy::primitives::B256;
use std::{
    collections::HashMap,
    ops::DerefMut,
    sync::{Arc, Mutex},
};
use tokio::sync::{OwnedSemaphorePermit, watch};

/// Registry mapping a `decryption_id` to the connections waiting for its response.
///
/// A single `decryption_id` can have multiple waiters, in case multiple HTTP clients submitted the
/// same request.
#[derive(Debug, Default)]
pub struct WaiterRegistry {
    inner: Mutex<HashMap<B256, watch::Sender<()>>>,
}

impl WaiterRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a waiter for `id`, which takes ownership of the in-flight `permit`.
    pub fn register(self: &Arc<Self>, id: B256, permit: OwnedSemaphorePermit) -> Waiter {
        let receiver = self
            .lock()
            .entry(id)
            .or_insert_with(|| watch::channel(()).0)
            .subscribe();
        Waiter {
            waiters: Arc::clone(self),
            id,
            receiver: Some(receiver),
            _permit: permit,
        }
    }

    /// Whether at least one connection waits for `id`.
    pub fn contains(&self, id: &B256) -> bool {
        self.lock().contains_key(id)
    }

    /// Wakes up every waiter of `id`.
    ///
    /// Returns whether anyone was waiting.
    pub fn wake(&self, id: &B256) -> bool {
        match self.lock().get(id) {
            Some(sender) => sender.send(()).is_ok(),
            None => false,
        }
    }

    /// Empties the registry. Dropping the senders fails every waiter with `RecvError`.
    pub fn clear(&self) {
        // Bind the old map so it is dropped after the lock guard is released.
        let _old = std::mem::take(self.lock().deref_mut());
    }

    /// Number of `decryption_id`s with at least one waiter.
    pub fn len(&self) -> usize {
        self.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }

    /// Locks the registry, recovering from a poisoned lock.
    ///
    /// Poisoning is only a signal that a thread panicked while holding the lock, and it
    /// requires a decision on whether the protected data is still consistent. Here it is: every
    /// critical section is a single `HashMap` operation that either completes or never starts, so
    /// a poisoned lock still holds a consistent map. Recovery is required rather than optional,
    /// because this is called from [`Waiter::drop`], which can run while a task is already
    /// unwinding after a panic. A second panic there would abort the whole process.
    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<B256, watch::Sender<()>>> {
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// One connection waiting for a response: its subscription and its in-flight permit.
///
/// Dropping it (client disconnect, error, panic) unsubscribes and releases the permit. A wake-up
/// happening between two `wait` calls is observed by the second one.
#[derive(Debug)]
pub struct Waiter {
    waiters: Arc<WaiterRegistry>,
    id: B256,
    // Only `None` while dropping.
    receiver: Option<watch::Receiver<()>>,
    _permit: OwnedSemaphorePermit,
}

impl Waiter {
    /// Blocks until the next `wake` for this `Waiter`.
    pub async fn wait(&mut self) -> Result<(), watch::error::RecvError> {
        self.receiver
            .as_mut()
            .expect("receiver is only taken on drop")
            .changed()
            .await
    }
}

impl Drop for Waiter {
    fn drop(&mut self) {
        let mut map = self.waiters.lock();
        // Unsubscribe under the lock so a concurrent `register` cannot race the cleanup below.
        drop(self.receiver.take());
        if map
            .get(&self.id)
            .is_some_and(|sender| sender.receiver_count() == 0)
        {
            map.remove(&self.id);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use tokio::sync::Semaphore;

    async fn permit(sem: &Arc<Semaphore>) -> OwnedSemaphorePermit {
        Arc::clone(sem).acquire_owned().await.unwrap()
    }

    #[tokio::test]
    async fn two_waiters_on_one_id_are_both_woken() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(2));
        let id = B256::repeat_byte(1);

        let mut w1 = waiters.register(id, permit(&sem).await);
        let mut w2 = waiters.register(id, permit(&sem).await);
        assert!(waiters.contains(&id));
        assert_eq!(sem.available_permits(), 0);

        assert!(waiters.wake(&id));
        w1.wait().await.unwrap();
        w2.wait().await.unwrap();
        // WaiterRegistry stays registered after a wake-up.
        assert!(waiters.contains(&id));
        assert!(waiters.wake(&id));
    }

    /// A waiter that is between two `wait()` calls must not miss a wake-up.
    #[tokio::test]
    async fn wake_between_two_waits_is_not_lost() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(1));
        let id = B256::repeat_byte(2);
        let mut w = waiters.register(id, permit(&sem).await);

        assert!(waiters.wake(&id));
        w.wait().await.unwrap();

        // `w` is not awaiting `wait()` while this wake-up happens.
        assert!(waiters.wake(&id));
        // Its next `wait()` must observe it instead of hanging.
        tokio::time::timeout(std::time::Duration::from_secs(1), w.wait())
            .await
            .expect("wake-up was lost")
            .unwrap();
    }

    #[tokio::test]
    async fn dropped_waiter_only_unsubscribes_itself_and_releases_its_permit() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(2));
        let id = B256::repeat_byte(3);

        let w1 = waiters.register(id, permit(&sem).await);
        let mut w2 = waiters.register(id, permit(&sem).await);
        assert_eq!(sem.available_permits(), 0);

        drop(w1);
        assert_eq!(sem.available_permits(), 1);
        assert!(waiters.contains(&id));
        // The other one can still be woken.
        assert!(waiters.wake(&id));
        w2.wait().await.unwrap();
    }

    #[tokio::test]
    async fn dropping_last_waiter_removes_the_entry() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(1));
        let id = B256::repeat_byte(4);
        let w = waiters.register(id, permit(&sem).await);
        drop(w);
        assert!(!waiters.contains(&id));
        assert!(waiters.is_empty());
        assert!(!waiters.wake(&id));
        assert_eq!(sem.available_permits(), 1);
    }

    /// After `clear`, a waiter of the old generation must not remove the entry a new waiter of the
    /// same id created in the meantime.
    #[tokio::test]
    async fn dropping_cleared_waiter_keeps_new_entry_of_same_id() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(2));
        let id = B256::repeat_byte(5);
        let mut old = waiters.register(id, permit(&sem).await);

        waiters.clear();
        assert!(old.wait().await.is_err());
        let mut new = waiters.register(id, permit(&sem).await);

        drop(old);
        assert!(waiters.contains(&id));
        assert!(waiters.wake(&id));
        new.wait().await.unwrap();
    }

    #[tokio::test]
    async fn clear_empties_the_map_and_fails_every_waiter() {
        let waiters = Arc::new(WaiterRegistry::new());
        let sem = Arc::new(Semaphore::new(3));
        let mut w1 = waiters.register(B256::repeat_byte(1), permit(&sem).await);
        let mut w2 = waiters.register(B256::repeat_byte(1), permit(&sem).await);
        let mut w3 = waiters.register(B256::repeat_byte(2), permit(&sem).await);

        waiters.clear();
        assert!(waiters.is_empty());
        for w in [&mut w1, &mut w2, &mut w3] {
            assert!(w.wait().await.is_err());
        }
    }
}
