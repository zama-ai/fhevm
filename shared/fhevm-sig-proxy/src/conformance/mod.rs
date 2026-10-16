//! Black-box conformance toolbox and suite for the proxies built on this library.
//!
//! A proxy binary runs as a process in front of a recording [`Stub`] endpoint, over real TLS, and
//! is driven only from outside: requests, its health and metrics endpoints and its output.
//! [`conformance_tests!`](crate::conformance_tests) expands into one test per criterion every
//! deployment meets, and [`protocol_config_signer_tests!`](crate::protocol_config_signer_tests)
//! adds the criteria of a signer list read from `ProtocolConfig`. Each binary's package calls them
//! with its [`Profile`]; this library's own in-process tests use the same toolbox.
//!
//! Test-only: enabled by the `conformance` feature, from dev-dependencies.

mod process;
mod rpc;
mod stub;
pub mod suite;

pub use process::{
    ENDPOINT_HEALTHCHECK_FREQUENCY, ENDPOINT_RESPONSE_TIMEOUT, Launch, MetricsPort, Profile,
    ProxyProcess, SIGNER_REFRESH_INTERVAL, Setup, SignedRoute,
};
pub use rpc::{FakeRpc, KmsContext};
pub use stub::{Canned, Recorded, Stub};

use alloy::{primitives::Address, signers::local::PrivateKeySigner};
use std::{
    net::{SocketAddr, TcpListener as StdListener},
    path::{Path, PathBuf},
    sync::atomic::{AtomicU16, AtomicU32, Ordering},
    time::{Duration, Instant},
};

pub const CHAIN_ID: u64 = 1;
pub const PROTOCOL_CONFIG: Address = Address::repeat_byte(0xc0);
/// The proxy's own address, the audience of the headers it accepts.
pub const AUDIENCE: Address = Address::repeat_byte(0xa1);

/// The allowed sender.
pub fn sender() -> PrivateKeySigner {
    "0x0101010101010101010101010101010101010101010101010101010101010101"
        .parse()
        .unwrap()
}

/// A sender no deployment allows.
pub fn stranger() -> PrivateKeySigner {
    "0x0202020202020202020202020202020202020202020202020202020202020202"
        .parse()
        .unwrap()
}

/// An `Authorization` value signed by the allowed sender for `path` and `body`.
pub async fn authorization(path: &str, body: &[u8]) -> String {
    signed_by(
        &sender(),
        AUDIENCE,
        path,
        body,
        fhevm_sig_auth::unix_now() + 60,
    )
    .await
}

/// An `Authorization` value signed by `signer` for `audience`, `path` and `body`, valid until
/// `expires` (Unix seconds).
pub async fn signed_by(
    signer: &PrivateKeySigner,
    audience: Address,
    path: &str,
    body: &[u8],
    expires: u64,
) -> String {
    let domain = fhevm_sig_auth::domain(CHAIN_ID, PROTOCOL_CONFIG);
    fhevm_sig_auth::sign(
        signer,
        &domain,
        audience,
        &path.parse().unwrap(),
        body,
        expires,
    )
    .await
    .unwrap()
    .to_string()
}

/// A self-signed certificate in its own temporary directory, which also holds any other file a
/// launch needs.
pub struct TestTls {
    pub cert_path: PathBuf,
    pub key_path: PathBuf,
}

impl TestTls {
    pub fn generate() -> Self {
        static NEXT_DIR: AtomicU32 = AtomicU32::new(0);
        let certified = rcgen::generate_simple_self_signed(vec!["127.0.0.1".into()]).unwrap();
        let dir = std::env::temp_dir().join(format!(
            "fhevm-sig-proxy-{}-{}",
            std::process::id(),
            NEXT_DIR.fetch_add(1, Ordering::Relaxed)
        ));
        std::fs::create_dir_all(&dir).unwrap();
        let (cert_path, key_path) = (dir.join("tls.crt"), dir.join("tls.key"));
        std::fs::write(&cert_path, certified.cert.pem()).unwrap();
        std::fs::write(&key_path, certified.signing_key.serialize_pem()).unwrap();
        Self {
            cert_path,
            key_path,
        }
    }

    pub fn dir(&self) -> &Path {
        self.cert_path.parent().unwrap()
    }
}

impl Drop for TestTls {
    fn drop(&mut self) {
        std::fs::remove_dir_all(self.dir()).ok();
    }
}

/// A port nothing listens on, below the ephemeral ranges (Linux 32768, macOS 49152) so that no
/// client socket takes it before the proxy binds it. Each test process counts through its own
/// block of 250 ports.
pub fn free_port() -> u16 {
    static NEXT: AtomicU16 = AtomicU16::new(0);
    let block = 20_000 + (std::process::id() % 40) as u16 * 250;
    loop {
        let port = block + NEXT.fetch_add(1, Ordering::Relaxed) % 250;
        if StdListener::bind(("127.0.0.1", port)).is_ok() {
            return port;
        }
    }
}

/// A local address on a [`free_port`].
pub fn free_addr() -> SocketAddr {
    SocketAddr::from(([127, 0, 0, 1], free_port()))
}

pub async fn wait_for_listener(addr: SocketAddr) {
    let deadline = Instant::now() + Duration::from_secs(10);
    while tokio::net::TcpStream::connect(addr).await.is_err() {
        assert!(Instant::now() < deadline, "{addr} did not come up");
        tokio::time::sleep(Duration::from_millis(50)).await;
    }
}
