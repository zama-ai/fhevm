#![cfg(feature = "integration-tests")]

use alloy::primitives::{Address, Bytes, U256};
use alloy::sol_types::SolCall;
use alloy::sol_types::SolValue;
use ethereum_rpc_mock::{MockConfig, MockServer, Response, UsageLimit};
use fhevm_host_bindings::i_protocol_config::IProtocolConfig;
use fhevm_relayer::config::settings::{ProtocolConfigSettings, RetrySettings};
use fhevm_relayer::host::threshold_resolver::ThresholdResolver;
use std::net::TcpListener;
use std::str::FromStr;

fn get_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

const PROTOCOL_CONFIG_ADDR: &str = "0x1234567890123456789012345678901234567890";

fn threshold_selector() -> [u8; 4] {
    IProtocolConfig::getUserDecryptionThresholdForContextCall::SELECTOR
}

fn abi_encode_u256(value: u64) -> Bytes {
    Bytes::from(U256::from(value).abi_encode())
}

fn make_config(port: u16) -> ProtocolConfigSettings {
    ProtocolConfigSettings {
        ethereum_http_rpc_url: format!("http://localhost:{}", port),
        address: PROTOCOL_CONFIG_ADDR.to_string(),
        retry: RetrySettings {
            max_attempts: 3,
            retry_interval_ms: 50,
        },
    }
}

fn register_threshold_response(server: &MockServer, threshold: u64, usage: UsageLimit) {
    let protocol_config_addr = Address::from_str(PROTOCOL_CONFIG_ADDR).unwrap();
    let selector = threshold_selector();
    let response_bytes = abi_encode_u256(threshold);

    server.on_call(
        move |params| {
            params.to == protocol_config_addr
                && params.input.len() >= 4
                && params.input[0..4] == selector
        },
        Response::call_success(response_bytes),
        usage,
    );
}

#[tokio::test]
async fn context_zero_returns_preseeded_default() {
    let port = get_free_port();
    let config = make_config(port);

    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    // Context ID 0 returns the pre-seeded static default — no RPC needed
    assert_eq!(resolver.resolve(U256::ZERO).await.unwrap(), 9u32);
}

#[tokio::test]
async fn fetches_threshold_from_contract() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    register_threshold_response(&mock, 5, UsageLimit::Unlimited);

    let _handle = mock.start().await.unwrap();
    let config = make_config(port);
    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    // Non-zero context_id fetches from mock
    let threshold = resolver.resolve(U256::from(42)).await.unwrap();
    assert_eq!(threshold, 5u32);
}

#[tokio::test]
async fn caches_threshold_permanently() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    // Only allow one successful call — second call would fail if not cached
    register_threshold_response(&mock, 7, UsageLimit::Once);

    let _handle = mock.start().await.unwrap();
    let config = make_config(port);
    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    // First call fetches
    assert_eq!(resolver.resolve(U256::from(10)).await.unwrap(), 7);

    // Second call hits cache — mock would reject a second call (UsageLimit::Once)
    assert_eq!(resolver.resolve(U256::from(10)).await.unwrap(), 7);
}

#[tokio::test]
async fn retry_succeeds_after_initial_failure() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    // First call fails, subsequent calls succeed
    let protocol_config_addr = Address::from_str(PROTOCOL_CONFIG_ADDR).unwrap();
    let selector = threshold_selector();

    mock.on_call(
        move |params| {
            params.to == protocol_config_addr
                && params.input.len() >= 4
                && params.input[0..4] == selector
        },
        Response::Error("node unavailable".to_string()),
        UsageLimit::Once,
    );
    register_threshold_response(&mock, 3, UsageLimit::Unlimited);

    let _handle = mock.start().await.unwrap();
    let config = make_config(port);
    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    // First attempt fails, retry succeeds
    let threshold = resolver.resolve(U256::from(77)).await.unwrap();
    assert_eq!(threshold, 3u32);
}

#[tokio::test]
async fn all_retries_exhausted_returns_error() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    // All calls fail
    let protocol_config_addr = Address::from_str(PROTOCOL_CONFIG_ADDR).unwrap();
    let selector = threshold_selector();

    mock.on_call(
        move |params| {
            params.to == protocol_config_addr
                && params.input.len() >= 4
                && params.input[0..4] == selector
        },
        Response::Error("permanent failure".to_string()),
        UsageLimit::Unlimited,
    );

    let _handle = mock.start().await.unwrap();
    let config = make_config(port);
    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    // All 3 retries fail → error
    let result = resolver.resolve(U256::from(99)).await;
    assert!(
        result.is_err(),
        "Should return error after exhausting retries"
    );
}

#[tokio::test]
async fn failed_fetch_is_not_cached() {
    let port = get_free_port();
    let mock = MockServer::new(MockConfig {
        port,
        ..MockConfig::new()
    });

    let protocol_config_addr = Address::from_str(PROTOCOL_CONFIG_ADDR).unwrap();
    let selector = threshold_selector();

    // First: all retries fail (3 errors consumed)
    for _ in 0..3 {
        mock.on_call(
            move |params| {
                params.to == protocol_config_addr
                    && params.input.len() >= 4
                    && params.input[0..4] == selector
            },
            Response::Error("temporary failure".to_string()),
            UsageLimit::Once,
        );
    }

    // Then: success for subsequent calls
    register_threshold_response(&mock, 11, UsageLimit::Unlimited);

    let _handle = mock.start().await.unwrap();
    let config = make_config(port);
    let resolver = ThresholdResolver::new(&config, 9u32, 100).await.unwrap();

    let context_id = U256::from(55);

    // First resolve: all retries fail → error
    let result = resolver.resolve(context_id).await;
    assert!(result.is_err());

    // Second resolve: retries again (not cached) → succeeds
    let threshold = resolver.resolve(context_id).await.unwrap();
    assert_eq!(threshold, 11u32);
}
