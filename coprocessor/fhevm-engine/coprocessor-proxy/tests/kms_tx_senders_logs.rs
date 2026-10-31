//! The log output of a failed `ProtocolConfig` read, in a test binary of its own: the subscriber
//! capturing it is global.

use alloy::{primitives::Address, providers::ProviderBuilder};
use coprocessor_proxy::kms_tx_senders::Follower;
use fhevm_sig_auth::RefreshableSigners;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};

#[derive(Clone, Default)]
struct Logs(Arc<Mutex<Vec<u8>>>);

impl std::io::Write for Logs {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.0.lock().unwrap().extend_from_slice(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[tokio::test]
async fn a_failed_read_logs_no_rpc_url() {
    let logs = Logs::default();
    let writer = logs.clone();
    tracing_subscriber::fmt()
        .with_writer(move || writer.clone())
        .with_ansi(false)
        .init();
    // Nothing listens on port 1: the connection is refused, and the transport error quotes the
    // URL with its keys.
    let url = "http://127.0.0.1:1/v2/SECRET-PATH-KEY?apikey=SECRET-QUERY-KEY";
    let provider = ProviderBuilder::new().connect_http(url.parse().unwrap());
    let interval = Duration::from_millis(20);
    let follower = Follower::new(
        provider,
        Address::ZERO,
        RefreshableSigners::default(),
        interval,
    );

    follower.run(tokio::time::sleep(interval * 5)).await;

    let logs = String::from_utf8(logs.0.lock().unwrap().clone()).unwrap();
    assert!(logs.contains("KMS tx-sender read failed"), "{logs}");
    for secret in ["SECRET", "apikey", "127.0.0.1:1"] {
        assert!(!logs.contains(secret), "{secret} in {logs}");
    }
}
