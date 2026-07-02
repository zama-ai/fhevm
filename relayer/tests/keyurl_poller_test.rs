//! Unit-style tests for the `/v2/keyurl` host-chain poller.
//!
//! These drive a real `KeyUrlPoller` against the `ethereum_rpc_mock` HTTP mock (no database),
//! exercising the binding calls and ABI decoding end-to-end:
//! - the initial fetch maps the on-chain ids/context into the served response, and
//! - a simulated id change is detected by the poll loop and pushed to the watch channel.

#![cfg(feature = "integration-tests")]

use std::net::TcpListener;
use std::str::FromStr;
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use std::time::Duration;

use alloy::primitives::{Address, Bytes, U256};
use alloy::sol_types::{SolCall, SolValue};
use ethereum_rpc_mock::{MockConfig, MockServer, Response, UsageLimit};
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_host_bindings::ikms_generation::IKMSGeneration;
use fhevm_relayer::config::settings::{KeyUrlConfig, ProtocolConfigSettings, RetrySettings};
use fhevm_relayer::host::KeyUrlPoller;
use tokio::sync::watch;

const CONTRACT_ADDR: &str = "0x1234567890123456789012345678901234567890";
const KEY_URL: &str = "https://example.com/PublicKey/0400";
const CRS_URL: &str = "https://example.com/CRS/0500";

fn get_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

fn make_config(port: u16, poll_interval_ms: u64) -> (ProtocolConfigSettings, KeyUrlConfig) {
    let protocol_config = ProtocolConfigSettings {
        ethereum_http_rpc_url: format!("http://localhost:{}", port),
        address: CONTRACT_ADDR.to_string(),
        retry: RetrySettings {
            max_attempts: 3,
            retry_interval_ms: 50,
        },
    };
    let keyurl = KeyUrlConfig {
        kms_generation_address: CONTRACT_ADDR.to_string(),
        poll_interval_ms,
    };
    (protocol_config, keyurl)
}

fn addr() -> Address {
    Address::from_str(CONTRACT_ADDR).unwrap()
}

fn key_materials_bytes() -> Bytes {
    // (string[] urls, KeyDigest[] digests) — empty digest array (element type irrelevant when empty)
    let urls = vec![KEY_URL.to_string()];
    let empty: Vec<Bytes> = Vec::new();
    Bytes::from((urls, empty).abi_encode_params())
}

fn crs_materials_bytes() -> Bytes {
    // (string[] urls, bytes digest)
    let urls = vec![CRS_URL.to_string()];
    Bytes::from((urls, Bytes::new()).abi_encode_params())
}

/// Register the static getters (CRS id, context/epoch, materials). The active key id is
/// registered separately so individual tests can make it static or dynamic.
fn register_static_getters(server: &MockServer) {
    let contract = addr();

    server.on_call(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IKMSGeneration::getActiveCrsIdCall::SELECTOR
        },
        Response::call_success(Bytes::from(U256::from(4u64).abi_encode())),
        UsageLimit::Unlimited,
    );
    server.on_call(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IProtocolConfig::getCurrentKmsContextAndEpochCall::SELECTOR
        },
        Response::call_success(Bytes::from(
            (U256::from(1u64), U256::from(2u64)).abi_encode_params(),
        )),
        UsageLimit::Unlimited,
    );
    server.on_call(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IKMSGeneration::getKeyMaterialsCall::SELECTOR
        },
        Response::call_success(key_materials_bytes()),
        UsageLimit::Unlimited,
    );
    server.on_call(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IKMSGeneration::getCrsMaterialsCall::SELECTOR
        },
        Response::call_success(crs_materials_bytes()),
        UsageLimit::Unlimited,
    );
}

#[tokio::test]
async fn initialize_maps_chain_state_to_response() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    let contract = addr();
    mock.on_call(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IKMSGeneration::getActiveKeyIdCall::SELECTOR
        },
        Response::call_success(Bytes::from(U256::from(3u64).abi_encode())),
        UsageLimit::Unlimited,
    );
    register_static_getters(&mock);
    let _handle = mock.start().await.unwrap();

    let (protocol_config, keyurl) = make_config(port, 12_000);
    let mut poller = KeyUrlPoller::new(&protocol_config, &keyurl).unwrap();
    let response = poller
        .initialize()
        .await
        .expect("initialize should succeed");

    // dataId carries the real on-chain getActiveKeyId / getActiveCrsId (decimal string).
    assert_eq!(
        response.response.fhe_key_info[0].fhe_public_key.data_id,
        "3"
    );
    assert_eq!(response.response.crs["2048"].data_id, "4");
    // urls come from getKeyMaterials / getCrsMaterials.
    assert_eq!(
        response.response.fhe_key_info[0].fhe_public_key.urls,
        vec![KEY_URL.to_string()]
    );
    assert_eq!(
        response.response.crs["2048"].urls,
        vec![CRS_URL.to_string()]
    );
    // contextId / epochId from getCurrentKmsContextAndEpoch.
    assert_eq!(response.response.context_id, "1");
    assert_eq!(response.response.epoch_id, "2");
}

#[tokio::test]
async fn run_pushes_updated_value_on_id_change() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    // Active key id is served from a shared atomic so the test can rotate it mid-run.
    let active_key_id = Arc::new(AtomicU64::new(3));
    let active_key_id_for_mock = active_key_id.clone();
    let contract = addr();
    mock.on_call_dynamic(
        move |p| {
            p.to == contract
                && p.input.len() >= 4
                && p.input[0..4] == IKMSGeneration::getActiveKeyIdCall::SELECTOR
        },
        move |_params| {
            let id = active_key_id_for_mock.load(Ordering::SeqCst);
            Response::call_success(Bytes::from(U256::from(id).abi_encode()))
        },
        UsageLimit::Unlimited,
    );
    register_static_getters(&mock);
    let _handle = mock.start().await.unwrap();

    // Seed via the startup fetch (reads key id 3), then run the loop on a short interval.
    let (protocol_config, keyurl) = make_config(port, 100);
    let mut poller = KeyUrlPoller::new(&protocol_config, &keyurl).unwrap();
    let initial = poller
        .initialize()
        .await
        .expect("initialize should succeed");
    assert_eq!(initial.response.fhe_key_info[0].fhe_public_key.data_id, "3");

    let (tx, mut rx) = watch::channel(initial);
    let run_handle = tokio::spawn(async move { poller.run(tx).await });

    // Rotate the active key id on-chain; the poller should detect it and push the new value.
    active_key_id.store(7, Ordering::SeqCst);

    tokio::time::timeout(Duration::from_secs(3), rx.changed())
        .await
        .expect("watch should update within timeout after id change")
        .expect("watch sender should stay alive");

    assert_eq!(
        rx.borrow().response.fhe_key_info[0].fhe_public_key.data_id,
        "7"
    );

    run_handle.abort();
}
