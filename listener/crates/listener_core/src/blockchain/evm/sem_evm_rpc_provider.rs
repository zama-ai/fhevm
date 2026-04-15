use std::sync::Arc;
use std::time::Duration;

use alloy::network::AnyRpcBlock;
use alloy::network::AnyTransactionReceipt;
use alloy::primitives::B256;
use alloy::providers::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller,
};
use alloy::providers::{Identity, Provider, ProviderBuilder, RootProvider};
use alloy::rpc::client::RpcClient;
use alloy::transports::http::Client;
// use alloy::rpc::types::{Block, Transaction, TransactionReceipt};
use alloy::transports::http::Http;
use serde::de::DeserializeOwned;
use serde_json::{Value, json};
use thiserror::Error;
use tokio::sync::{Semaphore, SemaphorePermit};
use tracing::{error, instrument, trace, warn};
use url::Url;

#[derive(Error, Debug)]
pub enum RpcProviderError {
    #[error("Invalid RPC URL: {0}")]
    UrlParseError(#[from] url::ParseError),

    #[error("Semaphore closed")]
    SemaphoreClosed,

    #[error("RPC Deserialization Error in method {method}: {details}")]
    DeserializationError { method: String, details: String },

    #[error("RPC UnsupportedMethod Error in method {method}: {details}")]
    UnsupportedMethod { method: String, details: String },

    #[error("RPC Batch Error: {0}")]
    BatchError(String),

    #[error("RPC Batch Unsupported by node")]
    BatchUnsupported,

    #[error("RPC Rate Limited: {0}")]
    RateLimited(String),

    #[error("RPC Transport Error: {0}")]
    TransportError(String),

    #[error("JSON Serialization Error: {0}")]
    SerdeError(#[from] serde_json::Error),

    #[error("Node returned null for expected data (Block/Receipt not found)")]
    NotFound,
}

type ProviderForSemRpc = FillProvider<
    JoinFill<
        Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

#[derive(Clone)]
pub struct SemEvmRpcProvider {
    /// RootProvider in Alloy 1.x takes exactly one generic: <N: Network>
    provider: ProviderForSemRpc,
    /// Semaphore to limit concurrent requests to the RPC node
    semaphore: Arc<Semaphore>,
    /// RPC URL for batch requests (reqwest needs the raw URL)
    rpc_url: String,
    /// Shared HTTP client for batch requests, built with the same production policy
    // batch_client: reqwest::Client,
    batch_client: Client,
}

impl SemEvmRpcProvider {
    /// Create a new provider with a concurrency limit and a tuned HTTP client.
    pub fn new(rpc_url: String, max_concurrent_requests: usize) -> Result<Self, RpcProviderError> {
        let url = Url::parse(&rpc_url)?;

        // Build a tuned Alloy HTTP client for heavy-duty indexing
        // Slow providers (30 s timeout)
        // Docker/Kubernetes stale connections (10 s idle eviction + 15 s keepalive)
        // Pod rollouts and scaling events (short idle timeout forces connection recycling)
        // Network policy blackholes (TCP keepalive detects silent drops)
        // Load balancer idle timeout alignment (10 s < any cloud LB timeout)
        let http_client = alloy::transports::http::Client::builder()
            .timeout(Duration::from_secs(25)) // Must accommodate slow providers with high response time
            .connect_timeout(Duration::from_secs(3)) // fast failure if the host is unreachable
            .pool_idle_timeout(Duration::from_secs(10)) // aggressive eviction of unused request.
            .pool_max_idle_per_host(max_concurrent_requests) // Match the semaphore size setting
            .tcp_keepalive(Duration::from_secs(15)) // OS-level dead connection detection. (the pool with self heal within 10 (idle timeout) - 15 sec)
            .build()
            .map_err(|e| RpcProviderError::TransportError(e.to_string()))?;

        // 1. Create the Http transport with Alloy's client
        let transport = Http::with_client(http_client.clone(), url);

        // 2. Create the RpcClient (The low-level engine)
        // RpcClient does not take generic arguments in this version.
        let rpc_client = RpcClient::new(transport, true);

        // 3. Build the Provider using the RpcClient
        let provider = ProviderBuilder::new().connect_client(rpc_client);

        // Build a dedicated reqwest client for batch requests with the same production policy
        let batch_client = http_client.clone();

        Ok(Self {
            provider,
            semaphore: Arc::new(Semaphore::new(max_concurrent_requests)),
            rpc_url,
            batch_client,
        })
    }

    /// Internal helper to acquire permit and execute a raw JSON-RPC request.
    /// This ensures every single call respects the concurrency limit.
    async fn raw_request<T: DeserializeOwned + Send + Sync + alloy_json_rpc::RpcRecv>(
        &self,
        method: &str,
        params: Value,
    ) -> Result<T, RpcProviderError> {
        let _permit: SemaphorePermit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| RpcProviderError::SemaphoreClosed)?;

        trace!(method = method, "Acquired semaphore, sending request");

        let client = self.provider.root().client();

        let result: T = client
            .request(method.to_string(), params)
            .await
            .map_err(|e| {
                let err_msg = e.to_string();

                if err_msg.contains("deserialization error")
                    || err_msg.contains("data did not match")
                {
                    error!(
                        method = method,
                        error = %err_msg,
                        "CRITICAL: rpc deserialization failure."
                    );
                    // This is where you would increment your custom metric
                    // metrics::RPC_DESER_ERRORS.with_label_values(&[method]).inc();
                    // Note: Raise alarm.

                    return RpcProviderError::DeserializationError {
                        method: method.to_string(),
                        details: err_msg,
                    };
                }
                if err_msg.contains("does not exist")
                    || err_msg.contains("is not whitelisted")
                    || err_msg.contains("is unsupported")
                {
                    // Note: Raise an alarm with a metric.
                    error!(
                        method = method,
                        error = %err_msg,
                        "CRITICAL: method is not supported."
                    );

                    return RpcProviderError::UnsupportedMethod {
                        method: method.to_string(),
                        details: err_msg,
                    };
                }
                if err_msg.contains("rate limit")
                    || err_msg.contains("too many requests")
                    || err_msg.contains("429")
                {
                    warn!(method = method, error = %err_msg, "Rate limited by RPC provider");
                    return RpcProviderError::RateLimited(err_msg);
                }

                RpcProviderError::TransportError(err_msg)
            })?;

        Ok(result)
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_block_number(&self) -> Result<u64, RpcProviderError> {
        let hex: String = self.raw_request("eth_blockNumber", json!([])).await?;

        u64::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| RpcProviderError::SerdeError(serde::ser::Error::custom(e)))
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_chain_id(&self) -> Result<u64, RpcProviderError> {
        let hex: String = self.raw_request("eth_chainId", json!([])).await?;

        u64::from_str_radix(hex.trim_start_matches("0x"), 16)
            .map_err(|e| RpcProviderError::SerdeError(serde::ser::Error::custom(e)))
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_block_by_number(&self, number: u64) -> Result<AnyRpcBlock, RpcProviderError> {
        let hex_number = format!("0x{:x}", number);
        self.get_block_by_number_hex(hex_number).await
    }

    pub async fn get_block_by_number_hex(
        &self,
        number_hex: String,
    ) -> Result<AnyRpcBlock, RpcProviderError> {
        let block: Option<AnyRpcBlock> = self
            .raw_request("eth_getBlockByNumber", json!([number_hex, true]))
            .await?;

        block.ok_or(RpcProviderError::NotFound)
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_block_by_hash(&self, hash: String) -> Result<AnyRpcBlock, RpcProviderError> {
        let block: Option<AnyRpcBlock> = self
            .raw_request("eth_getBlockByHash", json!([hash, true]))
            .await?;

        block.ok_or(RpcProviderError::NotFound)
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_transaction_receipt(
        &self,
        tx_hash: String,
    ) -> Result<AnyTransactionReceipt, RpcProviderError> {
        let receipt: Option<AnyTransactionReceipt> = self
            .raw_request("eth_getTransactionReceipt", json!([tx_hash]))
            .await?;

        receipt.ok_or(RpcProviderError::NotFound)
    }

    #[instrument(skip(self), level = "debug")]
    pub async fn get_block_receipts(
        &self,
        number: u64,
    ) -> Result<Vec<AnyTransactionReceipt>, RpcProviderError> {
        let hex_number = format!("0x{:x}", number);
        self.get_block_receipts_hex(hex_number).await
    }

    pub async fn get_block_receipts_hex(
        &self,
        number_hex: String,
    ) -> Result<Vec<AnyTransactionReceipt>, RpcProviderError> {
        let receipts: Option<Vec<AnyTransactionReceipt>> = self
            .raw_request("eth_getBlockReceipts", json!([number_hex]))
            .await?;

        receipts.ok_or(RpcProviderError::NotFound)
    }

    /// Fetch multiple transaction receipts in a single JSON-RPC batch request.
    /// More efficient than individual calls when eth_getBlockReceipts is unavailable.
    #[instrument(skip(self, tx_hashes), level = "debug")]
    pub async fn get_transaction_receipts_batch(
        &self,
        tx_hashes: &[B256],
    ) -> Result<Vec<AnyTransactionReceipt>, RpcProviderError> {
        if tx_hashes.is_empty() {
            return Ok(vec![]);
        }

        let _permit = self
            .semaphore
            .acquire()
            .await
            .map_err(|_| RpcProviderError::SemaphoreClosed)?;

        trace!(count = tx_hashes.len(), "Sending batch receipt request");

        // Build batch request array
        let batch_request: Vec<Value> = tx_hashes
            .iter()
            .enumerate()
            .map(|(id, hash)| {
                json!({
                    "jsonrpc": "2.0",
                    "id": id,
                    "method": "eth_getTransactionReceipt",
                    "params": [format!("{:?}", hash)]
                })
            })
            .collect();

        // Send batch request via the shared, production-tuned HTTP client
        let response = self
            .batch_client
            .post(&self.rpc_url)
            .header("Content-Type", "application/json")
            .json(&batch_request)
            .send()
            .await
            .map_err(|e| RpcProviderError::BatchError(format!("HTTP request failed: {}", e)))?;

        if response.status() == reqwest::StatusCode::TOO_MANY_REQUESTS {
            return Err(RpcProviderError::RateLimited(
                "HTTP 429: rate limited".to_string(),
            ));
        }

        if !response.status().is_success() {
            return Err(RpcProviderError::BatchError(format!(
                "HTTP error: {}",
                response.status()
            )));
        }

        let response_text = response
            .text()
            .await
            .map_err(|e| RpcProviderError::BatchError(format!("Failed to read response: {}", e)))?;

        // Parse the batch response
        let batch_response: Vec<Value> = serde_json::from_str(&response_text).map_err(|e| {
            // Check if the node doesn't support batch requests (returns single error object)
            if response_text.contains("error") && !response_text.starts_with('[') {
                return RpcProviderError::BatchUnsupported;
            }
            RpcProviderError::BatchError(format!("Failed to parse batch response: {}", e))
        })?;

        // Sort by ID to maintain order
        let mut sorted_responses: Vec<(usize, Value)> = batch_response
            .into_iter()
            .filter_map(|v| {
                let id = v.get("id")?.as_u64()? as usize;
                Some((id, v))
            })
            .collect();
        sorted_responses.sort_by_key(|(id, _)| *id);

        // Extract and deserialize results
        let mut receipts = Vec::with_capacity(tx_hashes.len());
        for (id, response) in sorted_responses {
            // Check for error in individual response
            if let Some(err) = response.get("error") {
                let err_msg = err
                    .get("message")
                    .and_then(|m| m.as_str())
                    .unwrap_or("Unknown error");
                return Err(RpcProviderError::BatchError(format!(
                    "Error for tx {}: {}",
                    id, err_msg
                )));
            }

            let result = response.get("result").ok_or_else(|| {
                RpcProviderError::BatchError(format!("Missing result for tx {}", id))
            })?;

            if result.is_null() {
                return Err(RpcProviderError::NotFound);
            }

            let receipt: AnyTransactionReceipt =
                serde_json::from_value(result.clone()).map_err(|e| {
                    RpcProviderError::DeserializationError {
                        method: "eth_getTransactionReceipt (batch)".to_string(),
                        details: format!("Failed to deserialize receipt {}: {}", id, e),
                    }
                })?;

            receipts.push(receipt);
        }

        Ok(receipts)
    }
}

/// Extract transaction hashes from an AnyRpcBlock.
/// Convenience utility for batch operations.
pub fn extract_tx_hashes(block: &AnyRpcBlock) -> Vec<B256> {
    block.transactions.hashes().collect()
}

#[cfg(test)]
#[path = "./tests/sem_evm_rpc_provider_tests.rs"]
mod tests;
