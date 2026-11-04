//! Bounds what one client can hold of the proxy, which Pingora 0.9 leaves unbounded: the time a
//! client takes to send a request head, and the connections one client IP opens.
//!
//! [`ClientLimits`] is shared by three users:
//! - its [`ConnectionFilter`] refuses a connection before TLS when the client IP holds
//!   [`ClientLimits::max_connections_per_ip`] connections in total, or
//!   [`ClientLimits::max_pending_handshakes_per_ip`] connections in their TLS handshake, and
//!   records each connection it accepts as pending;
//! - [`LimitedApp`] moves each connection from pending to open once TLS completes, and closes it
//!   when a request head misses its deadline;
//! - the proxy's hooks clear a connection's deadline once a head is parsed
//!   ([`ClientLimits::head_received`]) and set the next one once the response is sent
//!   ([`ClientLimits::response_sent`]).
//!
//! Pingora calls nothing when a handshake fails, so a pending entry that never completes expires
//! after [`PENDING_HANDSHAKE_EXPIRY`], when its handshake has ended either way.

use crate::core::CLIENT_IDLE_TIMEOUT;
use async_trait::async_trait;
use pingora::{
    apps::ServerApp,
    listeners::ConnectionFilter,
    protocols::{SocketDigest, Stream},
    server::ShutdownWatch,
};
use prometheus::{
    IntCounter, IntCounterVec, IntGauge, register_int_counter, register_int_counter_vec,
    register_int_gauge,
};
use std::{
    collections::HashMap,
    fmt,
    net::{IpAddr, SocketAddr},
    sync::{Arc, LazyLock, Mutex, MutexGuard, PoisonError},
    time::Duration,
};
use tokio::{sync::watch, time::Instant};

/// The time Pingora 0.9 gives a TLS handshake, fixed in `Service::run_endpoint`
/// (pingora-core `services/listening.rs`).
const PINGORA_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(60);

/// When a pending entry expires. Pingora starts its handshake timer only when the connection's
/// task runs, after the filter accepted it, so an entry outlives that timer by a margin that
/// covers the delay under load.
const PENDING_HANDSHAKE_EXPIRY: Duration =
    Duration::from_secs(PINGORA_HANDSHAKE_TIMEOUT.as_secs() + 5);

/// The time a client has, after a response, to send the next request head on the connection.
/// Later than Pingora's keep-alive ([`CLIENT_IDLE_TIMEOUT`]), which starts once the response is
/// finished: Pingora closes an idle connection itself, and this deadline only closes one whose
/// next head, or previous body, is still arriving.
pub const NEXT_HEAD_TIMEOUT: Duration = Duration::from_secs(CLIENT_IDLE_TIMEOUT.as_secs() + 5);

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

/// Connections accepted and still in their TLS handshake, with the time each was accepted.
struct PendingHandshakes {
    by_ip: HashMap<IpAddr, HashMap<SocketAddr, Instant>>,
    /// When entries of every IP were last expired. Between sweeps, an IP's entries expire when it
    /// connects.
    swept_at: Instant,
}

#[derive(Clone, Copy)]
enum Refusal {
    OpenConnections,
    PendingHandshakes,
}

pub struct ClientLimits {
    /// The time a client has, after its TLS handshake, to send its first request head.
    head_timeout: Duration,
    /// The time a client has, after a response, to send the next request head on the connection.
    next_head_timeout: Duration,
    /// Connections in their TLS handshake and past it. 0: no limit.
    max_connections_per_ip: usize,
    /// 0: no limit.
    max_pending_handshakes_per_ip: usize,
    /// Locked before `open` when both are needed.
    pending: Mutex<PendingHandshakes>,
    open: Mutex<HashMap<IpAddr, usize>>,
    deadlines: Mutex<HashMap<ConnectionId, watch::Sender<HeadDeadline>>>,
}

fn lock<T>(mutex: &Mutex<T>) -> MutexGuard<'_, T> {
    mutex.lock().unwrap_or_else(PoisonError::into_inner)
}

impl ClientLimits {
    pub fn new(
        head_timeout: Duration,
        max_connections_per_ip: usize,
        max_pending_handshakes_per_ip: usize,
    ) -> Self {
        Self {
            head_timeout,
            next_head_timeout: NEXT_HEAD_TIMEOUT,
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

    /// Records an accepted connection as pending, or refuses it when its IP is at a limit.
    fn admit(&self, peer: SocketAddr) -> Result<(), Refusal> {
        if self.max_connections_per_ip == 0 && self.max_pending_handshakes_per_ip == 0 {
            return Ok(());
        }
        let now = Instant::now();
        let is_live = |accepted_at: &Instant| now - *accepted_at < PENDING_HANDSHAKE_EXPIRY;
        let ip = canonical_ip(peer);
        let mut pending = lock(&self.pending);
        if now - pending.swept_at >= PENDING_HANDSHAKE_EXPIRY {
            pending.by_ip.retain(|_, peers| {
                peers.retain(|_, accepted_at| is_live(accepted_at));
                !peers.is_empty()
            });
            pending.swept_at = now;
        }
        let peers = pending.by_ip.entry(ip).or_default();
        peers.retain(|_, accepted_at| is_live(accepted_at));
        let open = lock(&self.open).get(&ip).copied().unwrap_or(0);
        let refusal = if self.max_connections_per_ip > 0
            && open + peers.len() >= self.max_connections_per_ip
        {
            Some(Refusal::OpenConnections)
        } else if self.max_pending_handshakes_per_ip > 0
            && peers.len() >= self.max_pending_handshakes_per_ip
        {
            Some(Refusal::PendingHandshakes)
        } else {
            None
        };
        match refusal {
            Some(refusal) => {
                if peers.is_empty() {
                    pending.by_ip.remove(&ip);
                }
                Err(refusal)
            }
            None => {
                // A new connection from the same address replaces an entry whose handshake ended.
                peers.insert(peer, now);
                Ok(())
            }
        }
    }

    /// Counts a connection past its TLS handshake until the returned guard drops, with the first
    /// request head due within the head timeout.
    fn track(&self, connection: ConnectionId, peer: SocketAddr) -> Tracked<'_> {
        let ip = canonical_ip(peer);
        {
            // One step under both locks, so the filter never sees the connection in neither count.
            let mut pending = lock(&self.pending);
            if let Some(peers) = pending.by_ip.get_mut(&ip) {
                peers.remove(&peer);
                if peers.is_empty() {
                    pending.by_ip.remove(&ip);
                }
            }
            *lock(&self.open).entry(ip).or_default() += 1;
        }
        CLIENT_CONNECTIONS.inc();
        let (sender, deadline) = watch::channel(Some(Instant::now() + self.head_timeout));
        lock(&self.deadlines).insert(connection, sender);
        Tracked {
            limits: self,
            connection,
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
        match self.admit(peer) {
            Ok(()) => true,
            Err(refusal) => {
                let reason = match refusal {
                    Refusal::OpenConnections => "open_connections",
                    Refusal::PendingHandshakes => "pending_handshakes",
                };
                CONNECTIONS_REFUSED.with_label_values(&[reason]).inc();
                false
            }
        }
    }
}

/// An IPv4 client reaching a dual-stack listener appears as an IPv4-mapped IPv6 address.
fn canonical_ip(peer: SocketAddr) -> IpAddr {
    peer.ip().to_canonical()
}

/// A connection counted by [`ClientLimits::track`].
struct Tracked<'a> {
    limits: &'a ClientLimits,
    connection: ConnectionId,
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
        let mut open = lock(&self.limits.open);
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
        // Held until the connection ends, which keeps its `ConnectionId` unique.
        let Some(digest) = stream.get_socket_digest() else {
            return self.inner.process_new(stream, shutdown).await;
        };
        let Some(peer) = digest.peer_addr().and_then(|addr| addr.as_inet()).copied() else {
            return self.inner.process_new(stream, shutdown).await;
        };
        let mut tracked = self.limits.track(connection_id(&digest), peer);
        // Dropping the inner future closes the connection.
        let reused = tokio::select! {
            reused = self.inner.process_new(stream, shutdown) => reused,
            () = tracked.head_overdue() => {
                HEAD_TIMEOUTS.inc();
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

    const A: SocketAddr = SocketAddr::new(IpAddr::V4(std::net::Ipv4Addr::new(10, 0, 0, 1)), 1000);

    fn peer(ip: [u8; 4], port: u16) -> SocketAddr {
        SocketAddr::from((ip, port))
    }

    /// A connection id derived from the peer, for tests with one connection per address.
    fn id(peer: SocketAddr) -> ConnectionId {
        peer.port() as ConnectionId
    }

    fn track(limits: &ClientLimits, peer: SocketAddr) -> Tracked<'_> {
        limits.track(id(peer), peer)
    }

    async fn overdue(tracked: &mut Tracked<'_>, within: Duration) -> bool {
        tokio::time::timeout(within, tracked.head_overdue())
            .await
            .is_ok()
    }

    #[tokio::test(start_paused = true)]
    async fn the_first_head_is_due_within_the_head_timeout() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 0);
        let mut tracked = track(&limits, A);
        assert!(!overdue(&mut tracked, Duration::from_millis(9_900)).await);
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_request_being_answered_has_no_deadline_and_the_next_head_is_due_after_pingoras_keep_alive()
     {
        let limits = ClientLimits::new(Duration::from_secs(1), 0, 0);
        let mut tracked = track(&limits, A);
        limits.head_received(id(A));
        // prf-service can hold a request for minutes.
        assert!(!overdue(&mut tracked, Duration::from_secs(300)).await);
        limits.response_sent(id(A));
        assert!(!overdue(&mut tracked, NEXT_HEAD_TIMEOUT - Duration::from_millis(100)).await);
        assert!(overdue(&mut tracked, Duration::from_millis(200)).await);
    }

    #[tokio::test(start_paused = true)]
    async fn two_connections_from_one_address_keep_their_own_deadlines() {
        let limits = ClientLimits::new(Duration::from_secs(1), 0, 0);
        let mut first = limits.track(1, A);
        let mut second = limits.track(2, A);
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
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 0);
        let mut tracked = track(&limits, A);
        limits.head_received(id(A));
        limits.deadlines.lock().unwrap().clear();
        assert!(overdue(&mut tracked, Duration::from_millis(1)).await);
    }

    #[tokio::test]
    async fn an_ip_at_its_open_connection_limit_is_refused_while_another_ip_is_served() {
        let limits = ClientLimits::new(Duration::from_secs(10), 2, 0);
        let first = track(&limits, peer([10, 0, 0, 1], 1));
        let _second = track(&limits, peer([10, 0, 0, 1], 2));
        assert!(!limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 2], 1))).await);
        drop(first);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
    }

    #[tokio::test]
    async fn pending_handshakes_count_toward_the_open_connection_limit() {
        let limits = ClientLimits::new(Duration::from_secs(10), 3, 10);
        let ip = |port| peer([10, 0, 0, 1], port);
        assert!(limits.should_accept(Some(&ip(1))).await);
        assert!(limits.should_accept(Some(&ip(2))).await);
        let _first = track(&limits, ip(1));
        assert!(limits.should_accept(Some(&ip(3))).await);
        // 1 open and 2 pending: completing the handshakes cannot take the IP past 3.
        assert!(!limits.should_accept(Some(&ip(4))).await);
        let _second = track(&limits, ip(2));
        let third = track(&limits, ip(3));
        assert!(!limits.should_accept(Some(&ip(4))).await);
        drop(third);
        assert!(limits.should_accept(Some(&ip(4))).await);
    }

    #[tokio::test]
    async fn an_ipv4_mapped_address_counts_as_its_ipv4_address() {
        let limits = ClientLimits::new(Duration::from_secs(10), 1, 0);
        let _tracked = track(&limits, peer([10, 0, 0, 1], 1));
        let mapped = SocketAddr::new("::ffff:10.0.0.1".parse().unwrap(), 2);
        assert!(!limits.should_accept(Some(&mapped)).await);
    }

    #[tokio::test]
    async fn closed_connections_leave_no_count_behind() {
        let limits = ClientLimits::new(Duration::from_secs(10), 1, 0);
        for port in 0..1000 {
            drop(track(&limits, peer([10, 0, 0, 1], port)));
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
        let _tracked = track(&limits, peer([10, 0, 0, 1], 1));
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 3))).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_completed_handshake_frees_only_its_own_pending_entry() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 2);
        let slow = |port| peer([10, 0, 0, 1], port);
        let fast = peer([10, 0, 0, 1], 100);
        assert!(limits.should_accept(Some(&slow(1))).await);
        assert!(limits.should_accept(Some(&fast)).await);
        drop(track(&limits, fast));
        assert!(limits.should_accept(Some(&slow(2))).await);
        // Slow handshakes 1 and 2 hold both slots: the fast completion freed only its own.
        assert!(!limits.should_accept(Some(&slow(3))).await);

        // A handshake completing after its entry expired frees no other connection's slot.
        let late = peer([10, 0, 0, 1], 200);
        tokio::time::advance(PENDING_HANDSHAKE_EXPIRY).await;
        assert!(limits.should_accept(Some(&late)).await);
        tokio::time::advance(PENDING_HANDSHAKE_EXPIRY).await;
        // These accepts expire the late entry and fill both slots.
        assert!(limits.should_accept(Some(&slow(4))).await);
        assert!(limits.should_accept(Some(&slow(5))).await);
        drop(track(&limits, late));
        assert!(!limits.should_accept(Some(&slow(6))).await);
    }

    #[tokio::test(start_paused = true)]
    async fn a_pending_entry_outlives_the_pingora_handshake_timeout() {
        let limits = ClientLimits::new(Duration::from_secs(10), 0, 1);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], 1))).await);
        assert!(limits.should_accept(Some(&peer([10, 0, 0, 2], 1))).await);
        // Still held when Pingora's own timer, started later, may not have fired yet.
        tokio::time::advance(PINGORA_HANDSHAKE_TIMEOUT).await;
        assert!(!limits.should_accept(Some(&peer([10, 0, 0, 1], 2))).await);
        tokio::time::advance(
            PENDING_HANDSHAKE_EXPIRY - PINGORA_HANDSHAKE_TIMEOUT - Duration::from_millis(1),
        )
        .await;
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
            .map(|port| track(&limits, peer([10, 0, 0, 1], port)))
            .collect();
        for port in 0..1000 {
            assert!(limits.should_accept(Some(&peer([10, 0, 0, 1], port))).await);
        }
        assert!(limits.pending.lock().unwrap().by_ip.is_empty());
    }
}
