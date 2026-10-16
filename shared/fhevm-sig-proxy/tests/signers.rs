//! The signers read from `ProtocolConfig`, observed from outside: through a fake Ethereum RPC.

mod common;

use common::TestProxy;
use fhevm_sig_proxy::{
    SignerSource,
    conformance::{FakeRpc, KmsContext, sender},
};
use http::{Method, StatusCode};
use std::time::{Duration, Instant};

const REFRESH_INTERVAL: Duration = Duration::from_millis(100);

async fn proxy_reading(rpc: &FakeRpc) -> TestProxy {
    let rpc_url = rpc.url.clone();
    TestProxy::start(move |config| {
        config.signers = SignerSource::ProtocolConfig {
            rpc_url,
            refresh_interval: REFRESH_INTERVAL,
        }
    })
    .await
}

fn the_sender_active() -> Vec<KmsContext> {
    vec![KmsContext {
        active: true,
        tx_senders: vec![sender().address()],
    }]
}

async fn wait_until_loaded(t: &TestProxy) {
    // A few refreshes: a failed read is retried within the refresh interval.
    let deadline = Instant::now() + REFRESH_INTERVAL * 10;
    while !t.state.signers_loaded() {
        assert!(Instant::now() < deadline, "signers never loaded");
        tokio::time::sleep(Duration::from_millis(20)).await;
    }
}

async fn status(t: &TestProxy) -> StatusCode {
    t.signed(Method::GET, "/items/1", b"").await.status()
}

#[tokio::test]
async fn requests_are_refused_and_not_ready_until_the_first_read() {
    let rpc = FakeRpc::start(the_sender_active()).await;
    rpc.set_failing(true);
    let t = proxy_reading(&rpc).await;

    tokio::time::sleep(REFRESH_INTERVAL * 3).await;
    assert!(!t.state.signers_loaded());
    assert_eq!(status(&t).await, StatusCode::UNAUTHORIZED);
    assert!(t.b.heads().is_empty());

    rpc.set_failing(false);
    wait_until_loaded(&t).await;
    assert_eq!(status(&t).await, StatusCode::OK);
}

#[tokio::test]
async fn a_signer_removed_from_protocol_config_is_refused_after_a_refresh() {
    let rpc = FakeRpc::start(the_sender_active()).await;
    let t = proxy_reading(&rpc).await;
    wait_until_loaded(&t).await;
    assert_eq!(status(&t).await, StatusCode::OK);

    // The sender's context is destroyed, and a new one without it is Active.
    rpc.set_contexts(vec![
        KmsContext {
            active: false,
            tx_senders: vec![sender().address()],
        },
        KmsContext {
            active: true,
            tx_senders: vec![],
        },
    ]);
    tokio::time::sleep(REFRESH_INTERVAL * 3).await;

    let refused = t.signed(Method::GET, "/items/1", b"").await;
    assert_eq!(refused.status(), StatusCode::UNAUTHORIZED);
    assert!(
        refused
            .text()
            .await
            .unwrap()
            .starts_with("Auth(UnknownSigner(")
    );
}

#[tokio::test]
async fn an_rpc_failure_keeps_the_last_signers() {
    let rpc = FakeRpc::start(the_sender_active()).await;
    let t = proxy_reading(&rpc).await;
    wait_until_loaded(&t).await;

    rpc.set_failing(true);
    tokio::time::sleep(REFRESH_INTERVAL * 3).await;

    assert!(t.state.signers_loaded());
    assert_eq!(status(&t).await, StatusCode::OK);
}
