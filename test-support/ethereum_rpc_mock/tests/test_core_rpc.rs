//! Core JSON-RPC method tests for the eth-json-rpc-mock-v2 server
//!
//! Tests fundamental Ethereum JSON-RPC methods with focus on mock server behavior:
//! - Chain information queries (chain ID, block number, gas price)
//! - Account state queries (balance, transaction count, code)
//! - Storage access and block queries
//!
//! All tests verify specific mock server responses and run on both HTTP and WebSocket.

use alloy::primitives::U256;
use ethereum_rpc_mock::test_utils::{generate_test_addresses, test_with_both_transports};

/// Test chain ID returns fixed value from mock server
#[tokio::test]
async fn test_chain_id_mock_behavior() {
    test_with_both_transports("test_chain_id", |provider, _mode| async move {
        let chain_id = provider.get_chain_id().await?;
        assert_eq!(chain_id, 1337, "Mock server should return chain ID 1337");
        Ok(())
    })
    .await
    .unwrap();
}

/// Test gas price returns expected fixed value (20 gwei)
#[tokio::test]
async fn test_gas_price_fixed_value() {
    test_with_both_transports("test_gas_price", |provider, _mode| async move {
        let gas_price = provider.get_gas_price().await?;
        assert_eq!(
            gas_price, 20_000_000_000_u128,
            "Mock server should return fixed gas price of 20 gwei"
        );
        Ok(())
    })
    .await
    .unwrap();
}

/// Test balance queries return zero for all addresses (mock behavior)
#[tokio::test]
async fn test_balance_queries_return_zero() {
    test_with_both_transports("test_get_balance", |provider, _mode| async move {
        let addresses = generate_test_addresses(5);

        for address in addresses {
            let balance = provider.get_balance(address).await?;
            assert_eq!(
                balance,
                U256::ZERO,
                "Mock server should return zero balance for address {}",
                address
            );
        }

        Ok(())
    })
    .await
    .unwrap();
}
