//! Bounds the client connections of a replica, which Pingora 0.9 leaves unbounded: how many it
//! holds, how fast it accepts new ones, and how long a new one takes to send its first request
//! head.
//!
//! [`ClientConnections`] refuses connections before their TLS handshake, as the listener's
//! [`ConnectionFilter`]. [`ClientConnectionsApp`] counts the connections past their handshake,
//! bounds how many wait for their first request head at once, and closes one whose first request
//! head is late. It finds out that the head arrived through the connection's [`FirstHead`], which
//! [`FirstHeadCallback`] stores in the TLS digest.

use async_trait::async_trait;
use pingora::{
    apps::ServerApp,
    listeners::{ConnectionFilter, TlsAccept},
    protocols::{Stream, tls::TlsRef},
    proxy::Session,
    server::ShutdownWatch,
};
use pingora_limits::rate::Rate;
use prometheus::{
    IntCounter, IntCounterVec, IntGauge, register_int_counter, register_int_counter_vec,
    register_int_gauge,
};
use std::{
    any::Any,
    fmt,
    net::SocketAddr,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicBool, AtomicUsize, Ordering},
    },
    time::Duration,
};
use tokio::sync::{Notify, Semaphore};
use tracing::error;

static CLIENT_CONNECTIONS: LazyLock<IntGauge> = LazyLock::new(|| {
    register_int_gauge!(
        "coprocessor_proxy_client_connections",
        "Client connections open past their TLS handshake"
    )
    .unwrap()
});

static CONNECTIONS_REFUSED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_proxy_connections_refused_counter",
        "Client connections refused by the connection limits, by the limit that refused them",
        &["reason"]
    )
    .unwrap()
});

static HEAD_TIMEOUTS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_proxy_request_head_timeouts_counter",
        "Client connections closed because their first request head did not arrive in time"
    )
    .unwrap()
});

/// The limits on a replica's client connections, and the count of those past their handshake.
pub struct ClientConnections {
    max_connections: usize,
    max_new_connections_per_second: isize,
    first_head_timeout: Duration,
    /// The connections past their TLS handshake. Those still in it are not counted: Pingora does
    /// not tell when one fails. `max_new_connections_per_second` bounds them instead, since
    /// Pingora gives a handshake at most 60 s. The count sits here, next to the gauge, because
    /// the gauge is one per process and the tests run several proxies in one process.
    open: AtomicUsize,
    new_connections: Rate,
    /// A permit for each connection past its handshake that waits for its first head:
    /// `max_new_connections_per_second` × `first_head_timeout` of them. The rate alone does not
    /// bound these connections: a client can hold its handshakes back, up to Pingora's 60 s, and
    /// complete them together. Each can buffer up to 1 MiB of an unfinished head.
    first_heads: Semaphore,
    /// Set by the readiness self-check: the next loopback connection is accepted whatever the
    /// limits, so a replica at one of them still shows as ready.
    self_check: AtomicBool,
    /// Set when the self-check's connection is accepted: it also passes `first_heads` once past
    /// its handshake.
    self_check_first_head: AtomicBool,
}

impl ClientConnections {
    pub fn new(
        max_connections: usize,
        max_new_connections_per_second: u32,
        first_head_timeout: Duration,
    ) -> Self {
        let first_heads =
            u128::from(max_new_connections_per_second) * first_head_timeout.as_millis() / 1000;
        Self {
            max_connections,
            max_new_connections_per_second: max_new_connections_per_second as isize,
            first_head_timeout,
            open: AtomicUsize::new(0),
            new_connections: Rate::new(Duration::from_secs(1)),
            first_heads: Semaphore::new((first_heads as usize).max(1)),
            self_check: AtomicBool::new(false),
            self_check_first_head: AtomicBool::new(false),
        }
    }

    /// Lets the next loopback connection through the limits. The readiness self-check calls it
    /// just before it connects. Loopback peers are not exempted as a whole: a sidecar would
    /// bypass the limits.
    pub fn admit_self_check(&self) {
        self.self_check.store(true, Ordering::Relaxed);
    }

    /// Uses the self-check's pass of `flag` when `peer` is a loopback address.
    fn self_check_passes(flag: &AtomicBool, peer: Option<&SocketAddr>) -> bool {
        peer.is_some_and(|peer| peer.ip().is_loopback()) && flag.swap(false, Ordering::Relaxed)
    }

    fn open(&self) -> Open<'_> {
        self.open.fetch_add(1, Ordering::Relaxed);
        CLIENT_CONNECTIONS.inc();
        Open(self)
    }
}

/// `ConnectionFilter` requires it; `Rate` has none.
impl fmt::Debug for ClientConnections {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("ClientConnections")
    }
}

#[async_trait]
impl ConnectionFilter for ClientConnections {
    async fn should_accept(&self, peer: Option<&SocketAddr>) -> bool {
        if Self::self_check_passes(&self.self_check, peer) {
            self.self_check_first_head.store(true, Ordering::Relaxed);
            return true;
        }
        let reason = if self.open.load(Ordering::Relaxed) >= self.max_connections {
            "max_connections"
        } else if self.new_connections.observe(&(), 1) > self.max_new_connections_per_second {
            "new_connections_rate"
        } else {
            return true;
        };
        CONNECTIONS_REFUSED.with_label_values(&[reason]).inc();
        false
    }
}

/// One connection past its TLS handshake, counted until it drops.
struct Open<'a>(&'a ClientConnections);

impl Drop for Open<'_> {
    fn drop(&mut self) {
        self.0.open.fetch_sub(1, Ordering::Relaxed);
        CLIENT_CONNECTIONS.dec();
    }
}

/// Tells [`ClientConnectionsApp`] that the first request head of a connection arrived.
#[derive(Default)]
struct FirstHead(Notify);

/// A request head of the session's connection is parsed. Only the first call matters.
pub fn first_head_received(session: &Session) {
    let first_head = session
        .digest()
        .and_then(|digest| digest.ssl_digest.as_ref()?.extension.get::<FirstHead>());
    if let Some(first_head) = first_head {
        first_head.0.notify_one();
    }
}

/// Gives each connection its [`FirstHead`] when its TLS handshake completes.
pub struct FirstHeadCallback;

#[async_trait]
impl TlsAccept for FirstHeadCallback {
    async fn handshake_complete_callback(
        &self,
        _ssl: &TlsRef,
    ) -> Option<Arc<dyn Any + Send + Sync>> {
        Some(Arc::new(FirstHead::default()))
    }
}

/// Runs `inner` on each client connection, and closes one whose first request head is late.
///
/// Pingora 0.9 bounds each read of a request head (60 s), not the whole head, so a client can
/// hold a connection for days by sending a byte a minute. Remove [`FirstHead`],
/// [`FirstHeadCallback`] and the timer here once Pingora can bound the whole head by itself.
/// Later heads on a connection are not bounded: only an authenticated client keeps its
/// connection, and it can hold one with cheap requests anyway.
pub struct ClientConnectionsApp<A> {
    inner: Arc<A>,
    connections: Arc<ClientConnections>,
}

impl<A> ClientConnectionsApp<A> {
    pub fn new(inner: A, connections: Arc<ClientConnections>) -> Self {
        Self {
            inner: Arc::new(inner),
            connections,
        }
    }
}

#[async_trait]
impl<A: ServerApp + Send + Sync + 'static> ServerApp for ClientConnectionsApp<A> {
    async fn process_new(
        self: &Arc<Self>,
        stream: Stream,
        shutdown: &ShutdownWatch,
    ) -> Option<Stream> {
        let digest = stream.get_ssl_digest();
        let Some(first_head) = digest
            .as_deref()
            .and_then(|digest| digest.extension.get::<FirstHead>())
        else {
            // Only a listener set up without `FirstHeadCallback` gets here. Serving the
            // connection would leave its first head unbounded, so it is closed.
            error!("Client connection without a FirstHead: closing it");
            return None;
        };
        let connections = &self.connections;
        let peer = stream
            .get_socket_digest()
            .and_then(|digest| digest.peer_addr()?.as_inet().copied());
        // Over the cap, the connection is closed. The self-check's connection passes it, as it
        // passed the limits before its handshake.
        let first_head_permit = if ClientConnections::self_check_passes(
            &connections.self_check_first_head,
            peer.as_ref(),
        ) {
            None
        } else if let Ok(permit) = connections.first_heads.try_acquire() {
            Some(permit)
        } else {
            CONNECTIONS_REFUSED
                .with_label_values(&["first_heads"])
                .inc();
            return None;
        };
        let _open = connections.open();
        // Completes only when the first head is late. The permit is released either way.
        let first_head_late = async move {
            let arrived = first_head.0.notified();
            let on_time = tokio::time::timeout(connections.first_head_timeout, arrived)
                .await
                .is_ok();
            drop(first_head_permit);
            if on_time {
                std::future::pending::<()>().await;
            }
        };
        // Dropping the inner future closes the connection.
        tokio::select! {
            reused = self.inner.process_new(stream, shutdown) => reused,
            () = first_head_late => {
                HEAD_TIMEOUTS.inc();
                None
            }
        }
    }

    async fn cleanup(&self) {
        self.inner.cleanup().await;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn connections(max_connections: usize, max_new_per_second: u32) -> ClientConnections {
        ClientConnections::new(max_connections, max_new_per_second, Duration::from_secs(10))
    }

    #[tokio::test]
    async fn refuses_connections_while_as_many_as_the_limit_are_open() {
        let connections = connections(2, 1000);
        let first = connections.open();
        assert!(connections.should_accept(None).await);
        let _second = connections.open();
        assert!(!connections.should_accept(None).await);
        drop(first);
        assert!(connections.should_accept(None).await);
    }

    #[tokio::test]
    async fn refuses_new_connections_above_the_rate() {
        let connections = connections(1000, 3);
        let mut accepted = 0;
        for _ in 0..10 {
            accepted += usize::from(connections.should_accept(None).await);
        }
        // 3, or up to 6 when the loop runs across the end of a 1 s window.
        assert!((3..=6).contains(&accepted), "{accepted}");
    }

    #[tokio::test]
    async fn admits_one_loopback_connection_for_the_self_check() {
        let connections = connections(1, 1000);
        let _held = connections.open();
        let loopback: SocketAddr = "127.0.0.1:1".parse().unwrap();
        let remote: SocketAddr = "192.0.2.1:1".parse().unwrap();

        assert!(!connections.should_accept(Some(&loopback)).await);
        connections.admit_self_check();
        assert!(!connections.should_accept(Some(&remote)).await);
        assert!(connections.should_accept(Some(&loopback)).await);
        assert!(!connections.should_accept(Some(&loopback)).await);

        // Past its handshake, the self-check's connection passes the first-heads cap once.
        let first_head = &connections.self_check_first_head;
        assert!(!ClientConnections::self_check_passes(
            first_head,
            Some(&remote)
        ));
        assert!(ClientConnections::self_check_passes(
            first_head,
            Some(&loopback)
        ));
        assert!(!ClientConnections::self_check_passes(
            first_head,
            Some(&loopback)
        ));
    }

    #[test]
    fn lets_rate_times_head_timeout_connections_wait_for_their_first_head() {
        let first_heads = |per_second, timeout| {
            ClientConnections::new(1000, per_second, timeout)
                .first_heads
                .available_permits()
        };
        assert_eq!(first_heads(50, Duration::from_secs(2)), 100);
        assert_eq!(first_heads(5, Duration::from_millis(1500)), 7);
        assert_eq!(first_heads(1, Duration::from_millis(100)), 1);
    }
}
