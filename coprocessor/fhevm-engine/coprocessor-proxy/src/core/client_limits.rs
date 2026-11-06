//! Bounds the time a client takes to send a request head, which Pingora 0.9 leaves unbounded.
//!
//! [`ClientLimits`] is shared by two users:
//! - [`LimitedApp`] gives each connection a deadline for its first request head once TLS
//!   completes, and closes the connection when a head misses its deadline;
//! - the proxy's hooks clear a connection's deadline once a head is parsed
//!   ([`ClientLimits::head_received`]) and set the next one once the response is sent
//!   ([`ClientLimits::response_sent`]).

use crate::core::CLIENT_IDLE_TIMEOUT;
use async_trait::async_trait;
use pingora::{
    apps::ServerApp,
    protocols::{SocketDigest, Stream},
    server::ShutdownWatch,
};
use prometheus::{IntCounter, IntGauge, register_int_counter, register_int_gauge};
use std::{
    collections::HashMap,
    sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError},
    time::Duration,
};
use tokio::{sync::watch, time::Instant};
use tracing::warn;

/// The time a client has, after a response, to send the next request head on the connection.
/// Later than Pingora's keep-alive ([`CLIENT_IDLE_TIMEOUT`]), which starts once the response is
/// finished: Pingora closes an idle connection itself, and this deadline only closes one whose
/// next head, or previous body, is still arriving.
pub const NEXT_HEAD_TIMEOUT: Duration = Duration::from_secs(CLIENT_IDLE_TIMEOUT.as_secs() + 5);

/// The shortest time between two warnings about connections closed at a head deadline: a flood
/// closes thousands a minute.
const CLOSED_CONNECTION_WARN_INTERVAL: Duration = Duration::from_secs(10);

static HEAD_TIMEOUTS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_proxy_request_head_timeouts_counter",
        "Client connections closed at a request head deadline: no complete head arrived in time, \
         or the previous request's body was still arriving"
    )
    .unwrap()
});

static CLIENT_CONNECTIONS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_proxy_client_connections",
        "Client connections open past their TLS handshake"
    )
    .unwrap()
});

/// Identifies an open client connection: the address of its [`SocketDigest`], which Pingora
/// shares between the connection's stream and its HTTP session. [`LimitedApp`] holds the digest
/// for the whole connection, so no other open connection has the same address. The client's
/// ip:port does not identify it: a client that resets a connection can open the next one from the
/// same port while the proxy still runs the first.
pub type ConnectionId = usize;

pub fn connection_id(digest: &Arc<SocketDigest>) -> ConnectionId {
    Arc::as_ptr(digest) as ConnectionId
}

/// A request head deadline, or `None` while a request is being answered.
type HeadDeadline = Option<Instant>;

/// Connections closed since the last warning about them, and when it was logged.
#[derive(Default)]
struct UnloggedCloses {
    count: u64,
    warned_at: Option<Instant>,
}

impl UnloggedCloses {
    /// Counts a closed connection, and returns the count to warn about, at most once per
    /// [`CLOSED_CONNECTION_WARN_INTERVAL`].
    fn record(&mut self, now: Instant) -> Option<u64> {
        self.count += 1;
        if self
            .warned_at
            .is_some_and(|at| now - at < CLOSED_CONNECTION_WARN_INTERVAL)
        {
            return None;
        }
        self.warned_at = Some(now);
        Some(std::mem::take(&mut self.count))
    }
}

pub struct ClientLimits {
    /// The time a client has, after its TLS handshake, to send its first request head.
    head_timeout: Duration,
    /// The time a client has, after a response, to send the next request head on the connection.
    next_head_timeout: Duration,
    deadlines: Mutex<HashMap<ConnectionId, watch::Sender<HeadDeadline>>>,
    unlogged_head_timeouts: Mutex<UnloggedCloses>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl ClientLimits {
    pub fn new(head_timeout: Duration) -> Self {
        Self {
            head_timeout,
            next_head_timeout: NEXT_HEAD_TIMEOUT,
            deadlines: Mutex::default(),
            unlogged_head_timeouts: Mutex::default(),
        }
    }

    /// The head of the connection's current request is parsed: no deadline until its response.
    pub fn head_received(&self, connection: ConnectionId) {
        self.set_deadline(connection, None);
    }

    /// The connection's response is sent: the next request head is due within
    /// [`NEXT_HEAD_TIMEOUT`].
    pub fn response_sent(&self, connection: ConnectionId) {
        self.set_deadline(connection, Some(Instant::now() + self.next_head_timeout));
    }

    fn set_deadline(&self, connection: ConnectionId, deadline: HeadDeadline) {
        if let Some(sender) = lock(&self.deadlines).get(&connection) {
            sender.send_replace(deadline);
        }
    }

    /// Counts a connection past its TLS handshake until the returned guard drops, with the first
    /// request head due within the head timeout.
    fn track(&self, connection: ConnectionId) -> Tracked<'_> {
        CLIENT_CONNECTIONS.inc();
        let (sender, deadline) = watch::channel(Some(Instant::now() + self.head_timeout));
        lock(&self.deadlines).insert(connection, sender);
        Tracked {
            limits: self,
            connection,
            deadline,
        }
    }
}

/// A connection counted by [`ClientLimits::track`].
struct Tracked<'a> {
    limits: &'a ClientLimits,
    connection: ConnectionId,
    deadline: watch::Receiver<HeadDeadline>,
}

impl Tracked<'_> {
    /// Completes when a request head deadline passes before the head arrives.
    async fn head_overdue(&mut self) {
        loop {
            let deadline = *self.deadline.borrow_and_update();
            let changed = match deadline {
                Some(at) => tokio::select! {
                    () = tokio::time::sleep_until(at) => return,
                    changed = self.deadline.changed() => changed,
                },
                None => self.deadline.changed().await,
            };
            // Without its sender no hook can reach the connection: close it.
            if changed.is_err() {
                return;
            }
        }
    }
}

impl Drop for Tracked<'_> {
    fn drop(&mut self) {
        lock(&self.limits.deadlines).remove(&self.connection);
        CLIENT_CONNECTIONS.dec();
    }
}

/// Runs `inner` on each client connection, counted by [`ClientLimits`] and closed when a request
/// head misses its deadline.
pub struct LimitedApp<A> {
    inner: Arc<A>,
    limits: Arc<ClientLimits>,
}

impl<A> LimitedApp<A> {
    pub fn new(inner: A, limits: Arc<ClientLimits>) -> Self {
        Self {
            inner: Arc::new(inner),
            limits,
        }
    }
}

#[async_trait]
impl<A: ServerApp + Send + Sync + 'static> ServerApp for LimitedApp<A> {
    async fn process_new(
        self: &Arc<Self>,
        stream: Stream,
        shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        // Held until the connection ends, which keeps its `ConnectionId` unique.
        let Some(digest) = stream.get_socket_digest() else {
            return self.inner.process_new(stream, shutdown).await;
        };
        let mut tracked = self.limits.track(connection_id(&digest));
        // Dropping the inner future closes the connection.
        let reused = tokio::select! {
            reused = self.inner.process_new(stream, shutdown) => reused,
            () = tracked.head_overdue() => {
                HEAD_TIMEOUTS.inc();
                let unlogged = lock(&self.limits.unlogged_head_timeouts).record(Instant::now());
                if let Some(count) = unlogged {
                    warn!(count, "Client connections closed at a request head deadline");
                }
                None
            }
        };
        drop(tracked);
        drop(digest);
        reused
    }

    async fn cleanup(&self) {
        self.inner.cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const ID: ConnectionId = 1;

    async fn overdue(tracked: &mut Tracked<'_>, within: Duration) -> bool {
        tokio::time::timeout(within, tracked.head_overdue())
            .await
            .is_ok()
    }

    #[tokio::test(start_paused = true)]
    async fn the_first_head_is_due_within_the_head_timeout() {
        let limits = ClientLimits::new(Duration::from_secs(10));
        let mut tracked = limits.track(ID);
        assert!(!overdue(&mut tracked, Duration::from_millis(9_900)).await);
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_being_answered_has_no_deadline_and_the_next_head_is_due_after_pingoras_keep_alive()
     {
        let limits = ClientLimits::new(Duration::from_secs(1));
        let mut tracked = limits.track(ID);
        limits.head_received(ID);
        // prf-service can hold a request for minutes.
        assert!(!overdue(&mut tracked, Duration::from_secs(300)).await);
        limits.response_sent(ID);
        assert!(!overdue(&mut tracked, NEXT_HEAD_TIMEOUT - Duration::from_millis(100)).await);
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn two_connections_keep_their_own_deadlines() {
        let limits = ClientLimits::new(Duration::from_secs(1));
        let mut first = limits.track(1);
        let mut second = limits.track(2);
        limits.head_received(1);
        // The first connection's hook does not clear the second's deadline.
        assert!(overdue(&mut second, Duration::from_millis(1_100)).await);
        assert!(!overdue(&mut first, Duration::from_secs(10)).await);
        drop(second);
        // The second connection's end leaves the first's deadline in place.
        limits.response_sent(1);
        assert!(!overdue(&mut first, NEXT_HEAD_TIMEOUT - Duration::from_millis(100)).await);
        assert!(overdue(&mut first, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_connection_whose_deadline_is_gone_is_overdue() {
        let limits = ClientLimits::new(Duration::from_secs(10));
        let mut tracked = limits.track(ID);
        limits.head_received(ID);
        limits.deadlines.lock().unwrap().clear();
        assert!(overdue(&mut tracked, Duration::from_millis(1)).await);
    }

    #[test]
    fn closed_connections_leave_no_deadline_behind() {
        let limits = ClientLimits::new(Duration::from_secs(10));
        for connection in 0..1000 {
            drop(limits.track(connection));
        }
        assert!(limits.deadlines.lock().unwrap().is_empty());
    }

    #[test]
    fn closed_connections_are_warned_about_at_most_once_per_interval_with_the_count_since() {
        let mut closes = UnloggedCloses::default();
        let start = Instant::now();
        assert_eq!(closes.record(start), Some(1));
        for _ in 0..3 {
            assert_eq!(closes.record(start + Duration::from_secs(1)), None);
        }
        assert_eq!(
            closes.record(start + CLOSED_CONNECTION_WARN_INTERVAL),
            Some(4)
        );
    }
}
