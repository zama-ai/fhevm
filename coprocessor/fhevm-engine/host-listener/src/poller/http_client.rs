use std::time::Duration;

use alloy::eips::BlockId;
use alloy::primitives::{Address, B256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::{Filter, Header, Log};
use alloy::transports::http::reqwest::Url;
use alloy::transports::layers::RetryBackoffLayer;
use anyhow::{anyhow, Context, Result};

use crate::database::ingest::{validate_block_logs, BlockLogs};

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
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rpc_url: &str,
        acl_address: Address,
        tfhe_address: Address,
        kms_generation_address: Option<Address>,
        confidential_bridge_address: Option<Address>,
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

        let addresses = Self::monitored_addresses(
            acl_address,
            tfhe_address,
            kms_generation_address,
            confidential_bridge_address,
        );

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

    async fn logs_for_hash(&self, block_hash: B256) -> Result<Vec<Log>> {
        let filter = Self::build_filter(block_hash, &self.addresses);
        self.provider.get_logs(&filter).await.with_context(|| {
            format!("Failed to get logs for block {block_hash}")
        })
    }

    /// Read a coherent snapshot and verify that it is still canonical before
    /// advancing the finalized catchup cursor. A reorg retries the entire
    /// block on the next poll, rather than reusing a stale logs/header pair.
    pub async fn block_logs_for_number(
        &self,
        number: u64,
    ) -> Result<BlockLogs<Log>> {
        let header = self.header_for_block(number).await?;
        let logs = self.logs_for_hash(header.hash).await?;
        if header.number != number
            || logs.iter().any(|log| {
                log.block_hash != Some(header.hash)
                    || log.block_number != Some(number)
                    || log.removed
            })
        {
            return Err(anyhow!(
                "Logs do not match block {number} ({})",
                header.hash
            ));
        }
        let current = self.header_for_block(number).await?;
        if current.hash != header.hash {
            return Err(anyhow!(
                "Canonical block {number} changed while fetching logs"
            ));
        }
        let block_logs = BlockLogs {
            logs,
            summary: header.into(),
            catchup: true,
            finalized: true,
        };
        validate_block_logs(&block_logs)?;
        Ok(block_logs)
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

    fn build_filter(block_hash: B256, addresses: &[Address]) -> Filter {
        let mut filter = Filter::new().at_block_hash(block_hash);
        if !addresses.is_empty() {
            filter = filter.address(addresses.to_vec());
        }
        filter
    }

    fn monitored_addresses(
        acl_address: Address,
        tfhe_address: Address,
        kms_generation_address: Option<Address>,
        confidential_bridge_address: Option<Address>,
    ) -> Vec<Address> {
        let mut addresses = vec![acl_address, tfhe_address];
        if let Some(kms_generation_address) = kms_generation_address {
            addresses.push(kms_generation_address);
        }
        if let Some(confidential_bridge_address) = confidential_bridge_address {
            addresses.push(confidential_bridge_address);
        }
        addresses
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloy::rpc::types::Block;
    use alloy::transports::mock::Asserter;

    #[test]
    fn filter_builder_sets_addresses_and_block_hash() {
        let addr1 = Address::from([1u8; 20]);
        let addr2 = Address::from([2u8; 20]);

        let hash = B256::with_last_byte(42);
        let filter = HttpChainClient::build_filter(hash, &[addr1, addr2]);
        let serialized = serde_json::to_value(filter).unwrap();

        assert_eq!(
            serialized["blockHash"],
            serde_json::to_value(hash).unwrap()
        );
        assert!(serialized.get("fromBlock").is_none());
        assert!(serialized.get("toBlock").is_none());

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
        let filter =
            HttpChainClient::build_filter(B256::with_last_byte(1), &[]);
        let serialized = serde_json::to_value(filter).unwrap();
        assert!(serialized.get("address").is_none());
    }

    #[test]
    fn optional_kms_generation_address_is_not_monitored_when_absent() {
        let acl_address = Address::from([1u8; 20]);
        let tfhe_address = Address::from([2u8; 20]);
        let addresses = HttpChainClient::monitored_addresses(
            acl_address,
            tfhe_address,
            None,
            None,
        );

        let filter =
            HttpChainClient::build_filter(B256::with_last_byte(7), &addresses);
        let serialized = serde_json::to_value(filter).unwrap();
        let mut actual: Vec<Address> =
            serde_json::from_value(serialized.get("address").cloned().unwrap())
                .unwrap();
        actual.sort();

        let mut expected = vec![acl_address, tfhe_address];
        expected.sort();
        assert_eq!(actual, expected);
    }

    #[test]
    fn confidential_bridge_address_is_monitored_when_present() {
        let acl_address = Address::from([1u8; 20]);
        let tfhe_address = Address::from([2u8; 20]);
        let bridge_address = Address::from([3u8; 20]);
        let mut actual = HttpChainClient::monitored_addresses(
            acl_address,
            tfhe_address,
            None,
            Some(bridge_address),
        );
        actual.sort();

        let mut expected = vec![acl_address, tfhe_address, bridge_address];
        expected.sort();
        assert_eq!(actual, expected);
    }

    fn block(hash: u8) -> Block {
        let mut block: Block = Block::default();
        block.header.hash = B256::with_last_byte(hash);
        block.header.inner.number = 42;
        block
    }

    fn mocked_client(asserter: Asserter) -> HttpChainClient {
        HttpChainClient {
            provider: Box::new(
                ProviderBuilder::new().connect_mocked_client(asserter),
            ),
            addresses: vec![],
        }
    }

    #[tokio::test]
    async fn coherent_snapshot_retries_after_canonical_header_changes() {
        let asserter = Asserter::new();
        let old = block(1);
        let new = block(2);
        asserter.push_success(&old);
        asserter.push_success(&Vec::<Log>::new());
        asserter.push_success(&new);
        asserter.push_success(&new);
        asserter.push_success(&Vec::<Log>::new());
        asserter.push_success(&new);
        let client = mocked_client(asserter);

        assert!(client.block_logs_for_number(42).await.is_err());
        let retried = client.block_logs_for_number(42).await.unwrap();
        assert_eq!(retried.summary.hash, new.header.hash);
        assert!(retried.finalized);
    }

    #[tokio::test]
    async fn coherent_snapshot_rejects_wrong_hash_logs() {
        let asserter = Asserter::new();
        asserter.push_success(&block(1));
        let log: Log = Log {
            block_hash: Some(block(2).header.hash),
            block_number: Some(42),
            ..Default::default()
        };
        asserter.push_success(&vec![log]);
        let client = mocked_client(asserter);
        assert!(client.block_logs_for_number(42).await.is_err());
    }
}
