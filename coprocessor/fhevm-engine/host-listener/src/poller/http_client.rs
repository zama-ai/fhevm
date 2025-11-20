use std::{
    fmt::{Debug, Display},
    future::Future,
    time::Duration,
};

use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::types::{Filter, Header, Log};
use alloy::transports::http::reqwest::Url;
use anyhow::{anyhow, Context, Result};
use tokio::time::sleep;
use tracing::warn;

use fhevm_engine_common::types::BlockchainProvider;

pub struct HttpChainClient {
    provider: BlockchainProvider,
    addresses: Vec<Address>,
    retry_interval: Duration,
    max_retries: u64,
}

impl HttpChainClient {
    pub fn new(
        rpc_url: &str,
        acl_address: Address,
        tfhe_address: Address,
        retry_interval: Duration,
        max_retries: u64,
    ) -> Result<Self> {
        let url = Url::parse(rpc_url)
            .context("Invalid rpc_url provided to poller HTTP client")?;
        let provider = ProviderBuilder::new().connect_http(url);

        let addresses = vec![acl_address, tfhe_address];

        Ok(Self {
            provider,
            addresses,
            retry_interval,
            max_retries,
        })
    }

    async fn retry_with_delay<T, F, Fut, E>(
        &self,
        label: &str,
        mut op: F,
    ) -> Result<(T, u64), RetryError<E>>
    where
        F: FnMut() -> Fut,
        Fut: Future<Output = Result<T, E>>,
        E: Display + Debug,
    {
        let mut retries = 0;
        loop {
            match op().await {
                Ok(value) => return Ok((value, retries)),
                Err(err) => {
                    if retries >= self.max_retries {
                        return Err(RetryError {
                            error: err,
                            retries,
                        });
                    }
                    retries += 1;
                    warn!(
                        label = label,
                        retries = retries,
                        error = %err,
                        "Retrying HTTP/RPC call"
                    );
                }
            }
            sleep(self.retry_interval).await;
        }
    }

    pub async fn chain_id(
        &self,
    ) -> Result<(u64, u64), RetryError<anyhow::Error>> {
        self.retry_with_delay("chain_id", || async {
            self.provider.get_chain_id().await.map_err(|e| anyhow!(e))
        })
        .await
    }

    pub async fn latest_block_number(
        &self,
    ) -> Result<(u64, u64), RetryError<anyhow::Error>> {
        self.retry_with_delay("latest_block_number", || async {
            self.provider
                .get_block_number()
                .await
                .map_err(|e| anyhow!(e))
        })
        .await
    }

    pub async fn logs_for_block(
        &self,
        block: u64,
    ) -> Result<(Vec<Log>, u64), RetryError<anyhow::Error>> {
        let filter = Self::build_filter(block, &self.addresses);
        self.retry_with_delay("logs_for_block", || async {
            self.provider
                .get_logs(&filter)
                .await
                .map_err(|e| anyhow!(e))
        })
        .await
    }

    pub async fn header_for_block(
        &self,
        block: u64,
    ) -> Result<(Header, u64), RetryError<anyhow::Error>> {
        let block_id = BlockId::number(block);
        self.retry_with_delay("header_for_block", || async {
            match self.provider.get_block(block_id).await {
                Ok(Some(block)) => Ok(block.header),
                Ok(None) => Err(anyhow!("Block {block} not found")),
                Err(err) => Err(anyhow!(err)),
            }
        })
        .await
    }

    pub(crate) fn build_filter(block: u64, addresses: &[Address]) -> Filter {
        let mut filter = Filter::new().from_block(block).to_block(block);
        if !addresses.is_empty() {
            filter = filter.address(addresses.to_vec());
        }
        filter
    }
}

#[derive(Debug)]
pub struct RetryError<E> {
    pub error: E,
    pub retries: u64,
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::Value;
    use std::sync::atomic::{AtomicUsize, Ordering};
    use std::sync::Arc;
    use std::time::Duration;

    fn test_client(
        retry_interval: Duration,
        max_retries: u64,
    ) -> HttpChainClient {
        HttpChainClient::new(
            "http://localhost:8545",
            Address::from([0u8; 20]),
            Address::from([1u8; 20]),
            retry_interval,
            max_retries,
        )
        .expect("failed to build HttpChainClient for tests")
    }

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

    #[tokio::test]
    async fn retry_with_delay_retries_then_succeeds() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();
        let client = test_client(Duration::from_millis(1), 5);

        let (value, retries) = client
            .retry_with_delay("test_retry", || {
                let attempts_clone = attempts_clone.clone();
                async move {
                    let current = attempts_clone.fetch_add(1, Ordering::SeqCst);
                    if current < 2 {
                        Err("temporary failure")
                    } else {
                        Ok(42)
                    }
                }
            })
            .await
            .unwrap();

        assert_eq!(value, 42);
        assert!(retries >= 2);
    }

    #[tokio::test]
    async fn retry_with_delay_stops_after_max() {
        let attempts = Arc::new(AtomicUsize::new(0));
        let attempts_clone = attempts.clone();
        let client = test_client(Duration::from_millis(1), 2);

        let err = client
            .retry_with_delay("test_retry_fail", || {
                let attempts_clone = attempts_clone.clone();
                async move {
                    attempts_clone.fetch_add(1, Ordering::SeqCst);
                    Err::<i32, _>("always fail")
                }
            })
            .await
            .unwrap_err();

        assert_eq!(err.retries, 2);
    }
}
