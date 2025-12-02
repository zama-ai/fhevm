use std::time::Duration;

use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::{Filter, Header, Log};
use alloy::transports::http::reqwest::Url;
use alloy::transports::layers::RetryBackoffLayer;
use anyhow::{anyhow, Context, Result};

/// HTTP client with built-in retry via Alloy's RetryBackoffLayer.
///
/// Retries are handled automatically at the transport layer for:
/// - HTTP 429 (rate limit)
/// - HTTP 5xx (server errors)
/// - HTTP 408 (timeout)
/// - Connection errors
///
/// If retries are exhausted, the error propagates up and the caller
/// should handle it (e.g., exit for orchestrator restart).
pub struct HttpChainClient {
    /// Using `Box<dyn Provider>` to type-erase the complex nested provider type
    /// returned by `ProviderBuilder` with `RetryBackoffLayer`:
    /// - The concrete type is deeply nested and verbose
    /// - It would be fragile to Alloy version updates
    /// - We only need the `Provider` trait methods, not the concrete type
    provider: Box<dyn Provider<alloy::network::Ethereum> + Send + Sync>,
    addresses: Vec<Address>,
}

impl HttpChainClient {
    pub fn new(
        rpc_url: &str,
        acl_address: Address,
        tfhe_address: Address,
        retry_interval: Duration,
        max_retries: u32,
        compute_units_per_second: u64,
    ) -> Result<Self> {
        let url = Url::parse(rpc_url).context(
            "Invalid rpc_url provided to host listener poller HTTP client",
        )?;

        // RetryBackoffLayer handles retries automatically at the transport level.
        // Parameters:
        // - max_retries: maximum retry attempts
        // - initial_backoff_ms: starting backoff duration
        // - compute_units_per_second: rate limiting budget (high value = no throttling)
        let backoff_ms = retry_interval.as_millis() as u64;
        let retry_layer = RetryBackoffLayer::new(
            max_retries,
            backoff_ms,
            compute_units_per_second,
        );

        let client = RpcClient::builder().layer(retry_layer).http(url);
        let provider = ProviderBuilder::new().connect_client(client);

        let addresses = vec![acl_address, tfhe_address];

        Ok(Self {
            provider: Box::new(provider),
            addresses,
        })
    }

    pub async fn chain_id(&self) -> Result<u64> {
        self.provider
            .get_chain_id()
            .await
            .context("Failed to get chain ID")
    }

    pub async fn latest_block_number(&self) -> Result<u64> {
        self.provider
            .get_block_number()
            .await
            .context("Failed to get latest block number")
    }

    pub async fn logs_for_block(&self, block: u64) -> Result<Vec<Log>> {
        let filter = Self::build_filter(block, &self.addresses);
        self.provider
            .get_logs(&filter)
            .await
            .with_context(|| format!("Failed to get logs for block {}", block))
    }

    pub async fn header_for_block(&self, block_number: u64) -> Result<Header> {
        let block_id = BlockId::number(block_number);
        let block =
            self.provider.get_block(block_id).await.with_context(|| {
                format!("Failed to get header for block {}", block_number)
            })?;
        match block {
            Some(block) => Ok(block.header),
            None => Err(anyhow!("Block {} not found", block_number)),
        }
    }

    fn build_filter(block: u64, addresses: &[Address]) -> Filter {
        let mut filter = Filter::new().from_block(block).to_block(block);
        if !addresses.is_empty() {
            filter = filter.address(addresses.to_vec());
        }
        filter
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;

    #[test]
    fn filter_builder_sets_addresses_and_block_bounds() {
        let addr1 = Address::from([1u8; 20]);
        let addr2 = Address::from([2u8; 20]);

        let filter = HttpChainClient::build_filter(42, &[addr1, addr2]);
        let serialized = serde_json::to_value(filter).unwrap();

        let from_block = serialized
            .get("fromBlock")
            .and_then(Value::as_str)
            .expect("fromBlock missing");
        let to_block = serialized
            .get("toBlock")
            .and_then(Value::as_str)
            .expect("toBlock missing");

        let from_block_num =
            u64::from_str_radix(from_block.trim_start_matches("0x"), 16)
                .unwrap();
        let to_block_num =
            u64::from_str_radix(to_block.trim_start_matches("0x"), 16).unwrap();
        assert_eq!(from_block_num, 42);
        assert_eq!(to_block_num, 42);

        let mut addresses: Vec<Address> =
            serde_json::from_value(serialized.get("address").cloned().unwrap())
                .unwrap();
        addresses.sort();
        let mut expected = vec![addr1, addr2];
        expected.sort();
        assert_eq!(addresses, expected);
    }

    #[test]
    fn filter_builder_skips_addresses_when_empty() {
        let filter = HttpChainClient::build_filter(1, &[]);
        let serialized = serde_json::to_value(filter).unwrap();
        assert!(serialized.get("address").is_none());
    }
}
