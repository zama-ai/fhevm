//! Registry of the HTTP connections waiting for a decryption response.

use alloy::primitives::B256;
use std::{
    collections::HashMap,
    sync::{
        Arc, Mutex,
        atomic::{AtomicU64, Ordering},
    },
};
use tokio::sync::{OwnedSemaphorePermit, oneshot};

/// Registry mapping a `decryption_id` to the connections waiting for its response.
///
/// A single `decryption_id` can have multiple waiters (each with a unique token), in case
/// multiple HTTP clients submitted the same request.
#[derive(Debug, Default)]
pub struct Waiters {
    inner: Mutex<HashMap<B256, Vec<Waiter>>>,
    next_token: AtomicU64,
}

/// One connection waiting for a response.
#[derive(Debug)]
struct Waiter {
    /// Unique token per registration, so a dropped guard removes only its own entry.
    token: u64,
    /// The channel used to notify the decryption routes that the response is ready.
    wake: oneshot::Sender<()>,
}

impl Waiters {
    pub fn new() -> Self {
        Self::default()
    }

    /// Registers a waiter for `id`.
    ///
    /// The returned guard owns the in-flight `permit` and removes this waiter from the registry
    /// when dropped (client disconnect, error, panic).
    pub fn register(
        self: &Arc<Self>,
        id: B256,
        permit: OwnedSemaphorePermit,
    ) -> (WaiterGuard, oneshot::Receiver<()>) {
        let (tx, rx) = oneshot::channel();
        let token = self.next_token.fetch_add(1, Ordering::Relaxed);
        self.lock()
            .entry(id)
            .or_default()
            .push(Waiter { token, wake: tx });
        let guard = WaiterGuard {
            waiters: Arc::clone(self),
            id,
            token,
            _permit: permit,
        };
        (guard, rx)
    }

    /// Whether at least one connection waits for `id`.
    pub fn contains(&self, id: &B256) -> bool {
        self.lock().contains_key(id)
    }

    /// Wakes up every waiter of `id`.
    ///
    /// Returns whether anyone was waiting.
    pub fn wake(&self, id: &B256) -> bool {
        let Some(waiters) = self.lock().remove(id) else {
            return false;
        };
        for waiter in waiters {
            // A closed receiver only means the client already went away.
            let _ = waiter.wake.send(());
        }
        true
    }

    /// Drains the whole registry. Dropping the senders fails every waiter with `RecvError`.
    pub fn clear(&self) {
        let drained: Vec<Waiter> = self.lock().drain().flat_map(|(_, v)| v).collect();
        drop(drained);
    }

    pub fn len(&self) -> usize {
        self.lock().len()
    }

    pub fn is_empty(&self) -> bool {
        self.lock().is_empty()
    }

    fn remove_waiter(&self, id: &B256, token: u64) {
        let mut map = self.lock();
        if let Some(waiters) = map.get_mut(id) {
            waiters.retain(|w| w.token != token);
            if waiters.is_empty() {
                map.remove(id);
            }
        }
    }

    fn lock(&self) -> std::sync::MutexGuard<'_, HashMap<B256, Vec<Waiter>>> {
        // The map is only ever mutated under short critical sections that cannot panic, so a
        // poisoned lock still holds a consistent map.
        self.inner.lock().unwrap_or_else(|e| e.into_inner())
    }
}

/// Owns one registered waiter `(id, token)` and its in-flight permit.
///
/// The guard removes the waiter from the registry and releases its permit on drop.
#[derive(Debug)]
pub struct WaiterGuard {
    waiters: Arc<Waiters>,
    id: B256,
    token: u64,
    _permit: OwnedSemaphorePermit,
}

impl WaiterGuard {
    /// Re-registers this waiter after a wake-up that turned out to be stale.
    pub fn rearm(&self) -> oneshot::Receiver<()> {
        let (tx, rx) = oneshot::channel();
        self.waiters
            .lock()
            .entry(self.id)
            .or_default()
            .push(Waiter {
                token: self.token,
                wake: tx,
            });
        rx
    }
}

impl Drop for WaiterGuard {
    fn drop(&mut self) {
        self.waiters.remove_waiter(&self.id, self.token);
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
        let waiters = Arc::new(Waiters::new());
        let sem = Arc::new(Semaphore::new(2));
        let id = B256::repeat_byte(1);

        let (_g1, rx1) = waiters.register(id, permit(&sem).await);
        let (_g2, rx2) = waiters.register(id, permit(&sem).await);
        assert!(waiters.contains(&id));
        assert_eq!(sem.available_permits(), 0);

        assert!(waiters.wake(&id));
        rx1.await.unwrap();
        rx2.await.unwrap();
        assert!(!waiters.contains(&id));
        assert!(!waiters.wake(&id));
    }

    #[tokio::test]
    async fn dropped_guard_removes_only_its_waiter_and_releases_permit() {
        let waiters = Arc::new(Waiters::new());
        let sem = Arc::new(Semaphore::new(2));
        let id = B256::repeat_byte(2);

        let (g1, rx1) = waiters.register(id, permit(&sem).await);
        let (_g2, rx2) = waiters.register(id, permit(&sem).await);
        assert_eq!(sem.available_permits(), 0);

        drop(g1);
        assert_eq!(sem.available_permits(), 1);
        assert!(waiters.contains(&id));
        // The dropped waiter's receiver errors, the other one can still be woken.
        assert!(rx1.await.is_err());
        assert!(waiters.wake(&id));
        rx2.await.unwrap();
    }

    #[tokio::test]
    async fn dropping_last_guard_removes_the_entry() {
        let waiters = Arc::new(Waiters::new());
        let sem = Arc::new(Semaphore::new(1));
        let id = B256::repeat_byte(3);
        let (g, _rx) = waiters.register(id, permit(&sem).await);
        drop(g);
        assert!(!waiters.contains(&id));
        assert!(waiters.is_empty());
    }

    #[tokio::test]
    async fn rearmed_waiter_is_woken_again_and_removed_on_drop() {
        let waiters = Arc::new(Waiters::new());
        let sem = Arc::new(Semaphore::new(1));
        let id = B256::repeat_byte(4);
        let (g, rx) = waiters.register(id, permit(&sem).await);

        assert!(waiters.wake(&id));
        rx.await.unwrap();
        assert!(!waiters.contains(&id));

        let rx = g.rearm();
        assert!(waiters.contains(&id));
        assert!(waiters.wake(&id));
        rx.await.unwrap();

        let _rx = g.rearm();
        drop(g);
        assert!(!waiters.contains(&id));
        assert_eq!(sem.available_permits(), 1);
    }

    #[tokio::test]
    async fn clear_empties_the_map_and_fails_every_receiver() {
        let waiters = Arc::new(Waiters::new());
        let sem = Arc::new(Semaphore::new(3));
        let (_g1, rx1) = waiters.register(B256::repeat_byte(1), permit(&sem).await);
        let (_g2, rx2) = waiters.register(B256::repeat_byte(1), permit(&sem).await);
        let (_g3, rx3) = waiters.register(B256::repeat_byte(2), permit(&sem).await);

        waiters.clear();
        assert!(waiters.is_empty());
        for rx in [rx1, rx2, rx3] {
            assert!(rx.await.is_err());
        }
    }
}
