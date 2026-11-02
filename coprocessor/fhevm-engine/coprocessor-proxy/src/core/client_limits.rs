//! Bounds what one client can hold of the proxy, which Pingora 0.9 leaves unbounded: the time a
//! client takes to send a request head, and the connections one client IP opens.
//!
//! [`ClientLimits`] is shared by three users:
//! - its [`ConnectionFilter`] refuses a connection before TLS when the client IP holds
//!   [`ClientLimits::max_connections_per_ip`] connections past TLS, or
//!   [`ClientLimits::max_pending_handshakes_per_ip`] connections in their TLS handshake, and
//!   records each connection it accepts as pending;
//! - [`LimitedApp`] moves each connection from pending to counted once TLS completes, and closes
//!   it when a request head misses its deadline;
//! - the proxy's hooks clear a connection's deadline once a head is parsed
//!   ([`ClientLimits::head_received`]) and set the next one once the response is sent
//!   ([`ClientLimits::response_sent`]).
//!
//! Pingora calls nothing when a handshake fails, so a pending entry that never completes expires
//! after [`PINGORA_HANDSHAKE_TIMEOUT`], when its handshake has ended either way.

use crate::core::CLIENT_IDLE_TIMEOUT;
use async_trait::async_trait;
use pingora::{
    apps::ServerApp, listeners::ConnectionFilter, protocols::Stream, server::ShutdownWatch,
};
use prometheus::{
    IntCounter, IntCounterVec, IntGauge, register_int_counter, register_int_counter_vec,
    register_int_gauge,
};
use std::{
    collections::HashMap,
    fmt,
    net::{IpAddr, SocketAddr},
    sync::{Arc, LazyLock, Mutex, PoisonError},
    time::Duration,
};
use tokio::{sync::watch, time::Instant};

/// The time Pingora 0.9 gives a TLS handshake, fixed in `Service::run_endpoint`
/// (pingora-core `services/listening.rs`).
const PINGORA_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(60);

static CONNECTIONS_REFUSED: LazyLock<IntCounterVec> = LazyLock::new(|| {
    register_int_counter_vec!(
        "coprocessor_proxy_connections_refused_counter",
        "Client connections refused before TLS, by the per-IP limit they hit",
        &["reason"]
    )
    .unwrap()
});

static HEAD_TIMEOUTS: LazyLock<IntCounter> = LazyLock::new(|| {
    register_int_counter!(
        "coprocessor_proxy_request_head_timeouts_counter",
        "Client connections closed because a request head did not arrive in time"
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

/// A request head deadline, or `None` while a request is being answered.
type HeadDeadline = Option<Instant>;

/// Connections accepted and still in their TLS handshake, with the time each was accepted.
struct PendingHandshakes {
    by_ip: HashMap<IpAddr, HashMap<SocketAddr, Instant>>,
    /// When entries of every IP were last expired. Between sweeps, an IP's entries expire when it
    /// connects.
    swept_at: Instant,
}

pub struct ClientLimits {
    /// The time a client has, after its TLS handshake, to send its first request head.
    head_timeout: Duration,
    /// The time a client has, after a response, to send the next request head on the connection.
    next_head_timeout: Duration,
    /// 0: no limit.
    max_connections_per_ip: usize,
    /// 0: no limit, and no connection is recorded as pending.
    max_pending_handshakes_per_ip: usize,
    pending: Mutex<PendingHandshakes>,
    open: Mutex<HashMap<IpAddr, usize>>,
    deadlines: Mutex<HashMap<SocketAddr, watch::Sender<HeadDeadline>>>,
}

impl ClientLimits {
    pub fn new(
        head_timeout: Duration,
        max_connections_per_ip: usize,
        max_pending_handshakes_per_ip: usize,
    ) -> Self {
        Self {
            head_timeout,
            next_head_timeout: CLIENT_IDLE_TIMEOUT,
            max_connections_per_ip,
            max_pending_handshakes_per_ip,
            pending: Mutex::new(PendingHandshakes {
                by_ip: HashMap::new(),
                swept_at: Instant::now(),
            }),
            open: Mutex::default(),
            deadlines: Mutex::default(),
        }
    }

    /// The head of the connection's current request is parsed: no deadline until its response.
    pub fn head_received(&self, peer: SocketAddr) {
        self.set_deadline(peer, None);
    }

    /// The connection's response is sent: the next request head is due within the idle timeout.
    pub fn response_sent(&self, peer: SocketAddr) {
        self.set_deadline(peer, Some(Instant::now() + self.next_head_timeout));
    }

    fn set_deadline(&self, peer: SocketAddr, deadline: HeadDeadline) {
        let deadlines = self
            .deadlines
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(sender) = deadlines.get(&peer) {
            sender.send_replace(deadline);
        }
    }

    fn open_connections(&self, ip: IpAddr) -> usize {
        let open = self.open.lock().unwrap_or_else(PoisonError::into_inner);
        open.get(&ip).copied().unwrap_or(0)
    }

    /// Records an accepted connection as pending, or refuses it when its IP holds
    /// `max_pending_handshakes_per_ip` pending connections.
    fn start_handshake(&self, peer: SocketAddr) -> bool {
        if self.max_pending_handshakes_per_ip == 0 {
            return true;
        }
        let now = Instant::now();
        let is_live = |accepted_at: &Instant| now - *accepted_at < PINGORA_HANDSHAKE_TIMEOUT;
        let mut pending = self.pending.lock().unwrap_or_else(PoisonError::into_inner);
        if now - pending.swept_at >= PINGORA_HANDSHAKE_TIMEOUT {
            pending.by_ip.retain(|_, peers| {
                peers.retain(|_, accepted_at| is_live(accepted_at));
                !peers.is_empty()
            });
            pending.swept_at = now;
        }
        let peers = pending.by_ip.entry(canonical_ip(peer)).or_default();
        peers.retain(|_, accepted_at| is_live(accepted_at));
        if peers.len() >= self.max_pending_handshakes_per_ip {
            return false;
        }
        // A new connection from the same address replaces an entry whose handshake ended.
        peers.insert(peer, now);
        true
    }

    fn handshake_completed(&self, peer: SocketAddr) {
        let mut pending = self.pending.lock().unwrap_or_else(PoisonError::into_inner);
        let ip = canonical_ip(peer);
        if let Some(peers) = pending.by_ip.get_mut(&ip) {
            peers.remove(&peer);
            if peers.is_empty() {
                pending.by_ip.remove(&ip);
            }
        }
    }

    /// Counts a connection past its TLS handshake until the returned guard drops, with the first
    /// request head due within the head timeout.
    fn track(&self, peer: SocketAddr) -> Tracked<'_> {
        self.handshake_completed(peer);
        let ip = canonical_ip(peer);
        *self
            .open
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .entry(ip)
            .or_default() += 1;
        CLIENT_CONNECTIONS.inc();
        let (sender, deadline) = watch::channel(Some(Instant::now() + self.head_timeout));
        self.deadlines
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .insert(peer, sender);
        Tracked {
            limits: self,
            peer,
            ip,
            deadline,
        }
    }
}

impl fmt::Debug for ClientLimits {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ClientLimits")
            .field("head_timeout", &self.head_timeout)
            .field("max_connections_per_ip", &self.max_connections_per_ip)
            .field(
                "max_pending_handshakes_per_ip",
                &self.max_pending_handshakes_per_ip,
            )
            .finish_non_exhaustive()
    }
}

#[async_trait]
impl ConnectionFilter for ClientLimits {
    async fn should_accept(&self, addr: Option<&SocketAddr>) -> bool {
        let Some(&peer) = addr else {
            return true;
        };
        if self.max_connections_per_ip > 0
            && self.open_connections(canonical_ip(peer)) >= self.max_connections_per_ip
        {
            CONNECTIONS_REFUSED
                .with_label_values(&["open_connections"])
                .inc();
            return false;
        }
        if !self.start_handshake(peer) {
            CONNECTIONS_REFUSED
                .with_label_values(&["pending_handshakes"])
                .inc();
            return false;
        }
        true
    }
}

/// An IPv4 client reaching a dual-stack listener appears as an IPv4-mapped IPv6 address.
fn canonical_ip(peer: SocketAddr) -> IpAddr {
    peer.ip().to_canonical()
}

/// A connection counted by [`ClientLimits::track`].
struct Tracked<'a> {
    limits: &'a ClientLimits,
    peer: SocketAddr,
    ip: IpAddr,
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
            // The sender lives in `ClientLimits::deadlines` as long as `self`.
            debug_assert!(changed.is_ok());
        }
    }
}

impl Drop for Tracked<'_> {
    fn drop(&mut self) {
        self.limits
            .deadlines
            .lock()
            .unwrap_or_else(PoisonError::into_inner)
            .remove(&self.peer);
        let mut open = self
            .limits
            .open
            .lock()
            .unwrap_or_else(PoisonError::into_inner);
        if let Some(count) = open.get_mut(&self.ip) {
            *count -= 1;
            if *count == 0 {
                open.remove(&self.ip);
            }
        }
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
        let peer = stream
            .get_socket_digest()
            .and_then(|digest| digest.peer_addr().and_then(|addr| addr.as_inet()).copied());
        let Some(peer) = peer else {
            return self.inner.process_new(stream, shutdown).await;
        };
        let mut tracked = self.limits.track(peer);
        // Dropping the inner future closes the connection.
        tokio::select! {
            reused = self.inner.process_new(stream, shutdown) => reused,
            () = tracked.head_overdue() => {
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

    const A: SocketAddr = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1)), 1000);

    fn peer(ip: [u8; 4], port: u16) -> SocketAddr {
        SocketAddr::from((ip, port))
    }

    async fn overdue(tracked: &mut Tracked<'_>, within: Duration) -> bool {
        tokio::time::timeout(within, tracked.head_overdue())
            .await
            .is_ok()
    }

    #[tokio::test(start_paused = true)]
    async fn the_first_head_is_due_within_the_head_timeout() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 0);
        let mut tracked = limits.track(A);
        assert!(!overdue(&mut tracked, Duration::from_millis(9_900)).await);
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_being_answered_has_no_deadline_and_the_next_head_is_due_after_the_idle_timeout()
     {
        let limits = ClientLimits::new(Duration::from_secs(1), 0, 0);
        let mut tracked = limits.track(A);
        limits.head_received(A);
        // prf-service can hold a request for minutes.
        assert!(!overdue(&mut tracked, Duration::from_secs(300)).await);
        limits.response_sent(A);
        assert!(
            !overdue(
                &mut tracked,
                CLIENT_IDLE_TIMEOUT - Duration::from_millis(100)
            )
            .await
        );
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test]
    async fn an_ip_at_its_open_connection_limit_is_refused_while_another_ip_is_served() {
        let limits = ClientLimits::new(Duration::from_secs(10), 2, 0);
        let first = limits.track(peer([10, 0, 0, 1], 1));
        let _second = limits.track(peer([10, 0, 0, 1], 2));
        assert!(!limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 2], 1))).await);
        drop(first);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
    }

    #[tokio::test]
    async fn an_ipv4_mapped_address_counts_as_its_ipv4_address() {
        let limits = ClientLimits::new(Duration::from_secs(10), 1, 0);
        let _tracked = limits.track(peer([10, 0, 0, 1], 1));
        let mapped = SocketAddr::new("::ffff:10.0.0.1".parse().unwrap(), 2);
        assert!(!limits.should_accept(Some(&mapped)).await);
    }

    #[tokio::test]
    async fn closed_connections_leave_no_count_behind() {
        let limits = ClientLimits::new(Duration::from_secs(10), 1, 0);
        for port in 0..1000 {
            drop(limits.track(peer([10, 0, 0, 1], port)));
        }
        assert!(limits.open.lock().unwrap().is_empty());
        assert!(limits.deadlines.lock().unwrap().is_empty());
        assert!(limits.should_accept(Some(&A)).await);
    }

    #[tokio::test]
    async fn an_ip_at_its_pending_handshake_limit_is_refused_while_another_ip_is_served() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 2);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 1))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 2))).await);
        assert!(!limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 2], 1))).await);
        let _tracked = limits.track(peer([10, 0, 0, 1], 1));
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_completed_handshake_frees_only_its_own_pending_entry() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 2);
        let slow = |port| peer([10, 0, 0, 1], port);
        let fast = peer([10, 0, 0, 1], 100);
        assert!(limits.should_accept(Some(&slow(1))).await);
        assert!(limits.should_accept(Some(&fast)).await);
        drop(limits.track(fast));
        assert!(limits.should_accept(Some(&slow(2))).await);
        // Slow handshakes 1 and 2 hold both slots: the fast completion freed only its own.
        assert!(!limits.should_accept(Some(&slow(3))).await);

        // A handshake completing after its entry expired frees no other connection's slot.
        let late = peer([10, 0, 0, 1], 200);
        tokio::time::advance(PINGORA_HANDSHAKE_TIMEOUT).await;
        assert!(limits.should_accept(Some(&late)).await);
        tokio::time::advance(PINGORA_HANDSHAKE_TIMEOUT).await;
        // These accepts expire the late entry and fill both slots.
        assert!(limits.should_accept(Some(&slow(4))).await);
        assert!(limits.should_accept(Some(&slow(5))).await);
        drop(limits.track(late));
        assert!(!limits.should_accept(Some(&slow(6))).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_pending_entry_expires_with_the_pingora_handshake_timeout() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 1);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 1))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 2], 1))).await);
        tokio::time::advance(PINGORA_HANDSHAKE_TIMEOUT - Duration::from_millis(1)).await;
        assert!(!limits.should_accept(Some(&peer([10, 0, 0, 1], 2))).await);
        tokio::time::advance(Duration::from_millis(1)).await;
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 2))).await);
        // The sweep also expired the entry of the IP that never came back.
        let pending = limits.pending.lock().unwrap();
        assert!(!pending.by_ip.contains_key(&IpAddr::from([10, 0, 0, 2])));
    }

    #[tokio::test]
    async fn zero_disables_both_per_ip_limits() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 0);
        let _tracked: Vec<_> = (0..100)
            .map(|port| limits.track(peer([10, 0, 0, 1], port)))
            .collect();
        for port in 0..1000 {
            assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], port))).await);
        }
        assert!(limits.pending.lock().unwrap().by_ip.is_empty());
    }
}
