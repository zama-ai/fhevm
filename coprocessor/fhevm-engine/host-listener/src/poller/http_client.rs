use std::sync::Mutex;
use std::time::Duration;

use alloy::eips::BlockId;
use alloy::primitives::{Address, B256};
use alloy::providers::{Provider, ProviderBuilder};
use alloy::rpc::client::RpcClient;
use alloy::rpc::types::{
    Filter, Header, Log, TransactionInput, TransactionRequest,
};
use alloy::sol_types::SolCall;
use alloy::transports::http::reqwest::{Client, Url};
use alloy::transports::layers::RetryBackoffLayer;
use anyhow::{anyhow, Context, Result};

use crate::cmd::ExecutorVersionRequest;
use crate::database::event_operands::{
    ExecutorVersion, ExecutorVersionSchedule,
};
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
    executor_version_cursor: Mutex<Option<(u64, B256, ExecutorVersion)>>,
}

impl HttpChainClient {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        rpc_url: &str,
        acl_address: Address,
        tfhe_address: Address,
        kms_generation_address: Option<Address>,
        protocol_config_address: Option<Address>,
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

        // A connected peer can accept a request and never send a response.
        // Reqwest defaults to no deadline; without one the retry layer never
        // sees an error and the poller cannot advance its durable cursor.
        let http = Client::builder()
            .connect_timeout(Duration::from_secs(3))
            .timeout(Duration::from_secs(30))
            .build()
            .context("Failed to build bounded host polling HTTP client")?;
        let client = RpcClient::builder()
            .layer(retry_layer)
            .http_with_client(http, url);
        let provider = ProviderBuilder::new().connect_client(client);

        let addresses = Self::monitored_addresses(
            acl_address,
            tfhe_address,
            kms_generation_address,
            protocol_config_address,
            confidential_bridge_address,
        );

        Ok(Self {
            provider: Box::new(provider),
            addresses,
            executor_version_cursor: Mutex::new(None),
        })
    }

    async fn query_executor_version(
        &self,
        address: Address,
        at_block_hash: B256,
    ) -> Option<ExecutorVersion> {
        let request = TransactionRequest::default().to(address).input(
            TransactionInput::both(
                crate::contracts::TfheContract::getVersionCall {}
                    .abi_encode()
                    .into(),
            ),
        );
        match tokio::time::timeout(
            Duration::from_secs(30),
            self.provider
                .call(request)
                .block(BlockId::hash_canonical(at_block_hash)),
        )
        .await
        {
            Ok(Ok(value)) => crate::contracts::TfheContract::getVersionCall::abi_decode_returns(
                &value,
            )
            .ok()
            .and_then(|value| ExecutorVersion::parse(&value)),
            Ok(Err(error)) => {
                tracing::debug!(
                    %address,
                    %at_block_hash,
                    %error,
                    "Cannot resolve FHEVMExecutor version at canonical block"
                );
                None
            }
            Err(_) => {
                tracing::debug!(
                    %address,
                    %at_block_hash,
                    "Timed out resolving FHEVMExecutor version at canonical block"
                );
                None
            }
        }
    }

    pub(crate) async fn executor_version_schedule(
        &self,
        request: ExecutorVersionRequest,
    ) -> ExecutorVersionSchedule {
        let Some(tfhe_proxy) = request.tfhe_proxy else {
            if let Ok(mut cursor) = self.executor_version_cursor.lock() {
                *cursor = None;
            }
            return ExecutorVersionSchedule::default();
        };

        let cached_parent = self
            .executor_version_cursor
            .lock()
            .ok()
            .and_then(|cursor| *cursor)
            .filter(|(number, hash, _)| {
                *hash == request.parent_hash
                    && number.checked_add(1) == Some(request.block_number)
            });
        let mut current = cached_parent
            .map(|(_, _, version)| version)
            .or_else(|| request.persisted_parent_version.flatten());
        if current.is_none() {
            current = self
                .query_executor_version(tfhe_proxy, request.parent_hash)
                .await;
        }
        let mut schedule = ExecutorVersionSchedule::new(current);
        for (log_index, implementation) in request.upgrades {
            current = self
                .query_executor_version(implementation, request.block_hash)
                .await;
            schedule.add_transition(log_index, current);
        }

        if let Some(version) = current {
            if let Ok(mut cursor) = self.executor_version_cursor.lock() {
                *cursor =
                    Some((request.block_number, request.block_hash, version));
            }
        } else if let Ok(mut cursor) = self.executor_version_cursor.lock() {
            *cursor = None;
        }
        schedule
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

    pub async fn finalized_block_number(&self) -> Result<u64> {
        self.provider
            .get_block(BlockId::finalized())
            .await
            .context("Failed to get finalized block")?
            .map(|block| block.header.number)
            .ok_or_else(|| anyhow!("Finalized block not found"))
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
        protocol_config_address: Option<Address>,
        confidential_bridge_address: Option<Address>,
    ) -> Vec<Address> {
        let mut addresses = vec![acl_address, tfhe_address];
        if let Some(kms_generation_address) = kms_generation_address {
            addresses.push(kms_generation_address);
        }
        if let Some(protocol_config_address) = protocol_config_address {
            addresses.push(protocol_config_address);
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
    use crate::cmd::block_history::BlockSummary;
    use crate::cmd::ExecutorVersionRequest;
    use crate::database::event_operands::ExecutorVersion;
    use crate::database::ingest::BlockLogs;
    use alloy::primitives::{Bytes, Log as PrimitiveLog};
    use alloy::rpc::types::Block;
    use alloy::sol_types::{SolCall, SolEvent};
    use alloy::transports::mock::Asserter;
    use serde_json::Value;

    #[tokio::test]
    async fn silent_peer_times_out_and_same_client_recovers() {
        use tokio::io::{AsyncReadExt, AsyncWriteExt};
        let listener =
            tokio::net::TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let (received, observed) = tokio::sync::oneshot::channel();
        let server = tokio::spawn(async move {
            let (mut silent, _) = listener.accept().await.unwrap();
            let mut bytes = [0; 4096];
            assert!(silent.read(&mut bytes).await.unwrap() > 0);
            received.send(()).unwrap();
            // Keep this socket open without sending even response headers.
            let (mut healthy, _) = listener.accept().await.unwrap();
            let mut request = Vec::new();
            loop {
                let n = healthy.read(&mut bytes).await.unwrap();
                assert!(n > 0);
                request.extend_from_slice(&bytes[..n]);
                if let Some(boundary) =
                    request.windows(4).position(|s| s == b"\r\n\r\n")
                {
                    if let Ok(value) = serde_json::from_slice::<Value>(
                        &request[boundary + 4..],
                    ) {
                        let body = serde_json::json!({"jsonrpc":"2.0", "id":value["id"], "result":"0x3039"}).to_string();
                        let reply = format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body);
                        healthy.write_all(reply.as_bytes()).await.unwrap();
                        break;
                    }
                }
            }
            drop(silent);
        });
        let client = HttpChainClient::new(
            &url,
            Address::ZERO,
            Address::ZERO,
            None,
            None,
            None,
            Duration::from_millis(1),
            0,
            1_000_000,
        )
        .unwrap();
        let outcome = tokio::time::timeout(Duration::from_secs(40), async {
            let first = client.chain_id();
            let (first, _) = tokio::join!(first, observed);
            assert!(
                first.is_err(),
                "a silent peer must return a transport failure"
            );
            assert_eq!(client.chain_id().await.unwrap(), 12345);
        })
        .await;
        server.abort();
        let _ = server.await;
        outcome.expect("the real production request deadline must release a silent connection");
    }

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
    fn protocol_config_address_is_monitored_when_present() {
        let acl_address = Address::from([1u8; 20]);
        let tfhe_address = Address::from([2u8; 20]);
        let protocol_config_address = Address::from([4u8; 20]);
        let mut actual = HttpChainClient::monitored_addresses(
            acl_address,
            tfhe_address,
            None,
            Some(protocol_config_address),
            None,
        );
        actual.sort();

        let mut expected =
            vec![acl_address, tfhe_address, protocol_config_address];
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
            executor_version_cursor: Mutex::new(None),
        }
    }

    fn encoded_version(version: &str) -> Bytes {
        crate::contracts::TfheContract::getVersionCall::abi_encode_returns(
            &version.to_owned(),
        )
        .into()
    }

    fn upgraded_log(
        proxy: Address,
        implementation: Address,
        block_number: u64,
        block_hash: B256,
        log_index: u64,
    ) -> Log {
        Log {
            inner: PrimitiveLog {
                address: proxy,
                data: crate::contracts::TfheContract::Upgraded {
                    implementation,
                }
                .encode_log_data(),
            },
            block_number: Some(block_number),
            block_hash: Some(block_hash),
            block_timestamp: Some(1),
            transaction_hash: Some(B256::with_last_byte(1)),
            transaction_index: Some(0),
            log_index: Some(log_index),
            removed: false,
        }
    }

    fn version_block(
        number: u64,
        hash: B256,
        parent_hash: B256,
        logs: Vec<Log>,
    ) -> BlockLogs<Log> {
        BlockLogs {
            logs,
            summary: BlockSummary {
                number,
                hash,
                parent_hash,
                timestamp: 1,
            },
            catchup: false,
            finalized: true,
        }
    }

    #[tokio::test]
    async fn version_schedule_bootstraps_and_applies_upgrades_after_log_index()
    {
        let asserter = Asserter::new();
        asserter.push_success(&encoded_version("FHEVMExecutor v0.5.0"));
        asserter.push_success(&encoded_version("FHEVMExecutor v0.6.0"));
        asserter.push_success(&encoded_version("FHEVMExecutor v0.7.0"));
        let client = mocked_client(asserter);
        let proxy = Address::from([1; 20]);
        let v06 = Address::from([6; 20]);
        let v07 = Address::from([7; 20]);
        let hash = B256::with_last_byte(0x64);
        let block = version_block(
            100,
            hash,
            B256::with_last_byte(0x63),
            vec![
                upgraded_log(proxy, v07, 100, hash, 9),
                upgraded_log(proxy, v06, 100, hash, 4),
            ],
        );
        let request = ExecutorVersionRequest::from_block(&block, Some(proxy));
        assert_eq!(request.upgrades, vec![(4, v06), (9, v07)]);

        let schedule = client.executor_version_schedule(request).await;
        assert_eq!(
            schedule.version_for_log_index(3),
            Some(ExecutorVersion::new(0, 5, 0))
        );
        assert_eq!(
            schedule.version_for_log_index(4),
            Some(ExecutorVersion::new(0, 5, 0))
        );
        assert_eq!(
            schedule.version_for_log_index(5),
            Some(ExecutorVersion::new(0, 6, 0))
        );
        assert_eq!(
            schedule.version_for_log_index(9),
            Some(ExecutorVersion::new(0, 6, 0))
        );
        assert_eq!(
            schedule.version_for_log_index(10),
            Some(ExecutorVersion::new(0, 7, 0))
        );
    }

    #[tokio::test]
    async fn version_resolution_handles_unknown_restart_and_orphaned_upgrade() {
        let seed = 0x70_u64;
        let address_byte = (seed % 251 + 1) as u8;
        let block_number = 100 + seed % 1_000_000;
        let asserter = Asserter::new();
        asserter.push_success(&encoded_version("FHEVMExecutor v0.5.0"));
        asserter.push_success(&encoded_version("FHEVMExecutor v0.6.0"));
        asserter.push_success(&encoded_version("FHEVMExecutor v0.5.0"));
        let client = mocked_client(asserter);
        let proxy = Address::from([address_byte; 20]);
        let parent_hash = B256::with_last_byte(address_byte);
        let original_hash = B256::with_last_byte(address_byte.wrapping_add(1));
        let replacement_hash =
            B256::with_last_byte(address_byte.wrapping_add(2));
        let implementation = Address::from([address_byte.wrapping_add(6); 20]);

        let original = version_block(
            block_number,
            original_hash,
            parent_hash,
            vec![upgraded_log(proxy, implementation, 100, original_hash, 2)],
        );
        let original = client
            .executor_version_schedule(ExecutorVersionRequest::from_block(
                &original,
                Some(proxy),
            ))
            .await;
        assert_eq!(
            original.ending_version(),
            Some(ExecutorVersion::new(0, 6, 0))
        );

        // A canonical replacement for the same height omits the orphaned
        // upgrade; the cached version from the old block must not carry over.
        let replacement =
            version_block(block_number, replacement_hash, parent_hash, vec![]);
        let replacement = client
            .executor_version_schedule(ExecutorVersionRequest::from_block(
                &replacement,
                Some(proxy),
            ))
            .await;
        assert_eq!(
            replacement.ending_version(),
            Some(ExecutorVersion::new(0, 5, 0))
        );

        // A restarted client uses the hash-matched persisted parent version.
        let restarted = mocked_client(Asserter::new());
        let mut request = ExecutorVersionRequest::from_block(
            &version_block(
                block_number + 1,
                B256::with_last_byte(address_byte.wrapping_add(3)),
                replacement_hash,
                vec![],
            ),
            Some(proxy),
        );
        request.persisted_parent_version =
            Some(Some(ExecutorVersion::new(0, 5, 0)));
        let replayed = restarted.executor_version_schedule(request).await;
        assert_eq!(
            replayed.version_for_log_index(0),
            Some(ExecutorVersion::new(0, 5, 0))
        );

        // Unknown contract strings remain permissive at bootstrap and after an
        // upgrade instead of applying a guessed newer policy.
        let unknown_responses = Asserter::new();
        unknown_responses
            .push_success(&encoded_version("unrecognized executor"));
        unknown_responses
            .push_success(&encoded_version("unrecognized implementation"));
        let unknown_client = mocked_client(unknown_responses);
        let unknown_hash = B256::with_last_byte(address_byte.wrapping_add(4));
        let unknown_block = version_block(
            block_number + 2,
            unknown_hash,
            replacement_hash,
            vec![upgraded_log(
                proxy,
                Address::from([address_byte.wrapping_add(8); 20]),
                block_number + 2,
                unknown_hash,
                7,
            )],
        );
        let unknown = unknown_client
            .executor_version_schedule(ExecutorVersionRequest::from_block(
                &unknown_block,
                Some(proxy),
            ))
            .await;
        assert!(unknown.is_resolved());
        assert_eq!(unknown.version_for_log_index(6), None);
        assert_eq!(unknown.version_for_log_index(8), None);
    }

    #[tokio::test(flavor = "multi_thread", worker_threads = 2)]
    async fn persisted_parent_version_survives_database_and_http_client_restart(
    ) {
        use crate::database::ingest::{
            ingest_with_history_recovery_and_executor_versions, IngestOptions,
        };
        use crate::database::tfhe_event_propagate::Database;
        use fhevm_engine_common::chain_id::ChainId;
        use test_harness::instance::{setup_test_db, ImportMode};

        let instance = setup_test_db(ImportMode::None).await.expect("test db");
        let chain_id = ChainId::try_from(42_u64).expect("chain id");
        let proxy = Address::with_last_byte(0x42);
        let version = ExecutorVersion::new(0, 5, 0);
        let options = IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        };
        let acl = None;
        let kms = None;
        let protocol = None;
        let bridge = None;
        let mut first_db = Database::new(&instance.db_url, chain_id, 16)
            .await
            .expect("first database client");
        let parent = version_block(
            100,
            B256::with_last_byte(0x64),
            B256::with_last_byte(0x63),
            vec![],
        );
        ingest_with_history_recovery_and_executor_versions(
            chain_id,
            &mut first_db,
            &parent,
            &acl,
            &Some(proxy),
            &kms,
            &protocol,
            &bridge,
            options.clone(),
            |_| std::future::ready(Err(anyhow!("unexpected history gap"))),
            |request| async move {
                assert_eq!(request.persisted_parent_version, None);
                ExecutorVersionSchedule::new(Some(version))
            },
        )
        .await
        .expect("ingest parent block")
        .require_complete()
        .expect("parent block complete");
        drop(first_db);

        // Simulate process restart: both the database wrapper and poller client
        // are fresh, and the mocked RPC has no historical version response.
        let mut restarted_db = Database::new(&instance.db_url, chain_id, 16)
            .await
            .expect("restarted database client");
        let restarted_client = mocked_client(Asserter::new());
        let child = version_block(
            101,
            B256::with_last_byte(0x65),
            parent.summary.hash,
            vec![],
        );
        ingest_with_history_recovery_and_executor_versions(
            chain_id,
            &mut restarted_db,
            &child,
            &acl,
            &Some(proxy),
            &kms,
            &protocol,
            &bridge,
            options,
            |_| std::future::ready(Err(anyhow!("unexpected history gap"))),
            |request| {
                let client = &restarted_client;
                async move {
                    let schedule = client.executor_version_schedule(request).await;
                    assert_eq!(
                        schedule.version_for_log_index(0),
                        Some(version),
                        "the poller must use the version loaded from the canonical parent row"
                    );
                    schedule
                }
            },
        )
        .await
        .expect("ingest child block after restart")
        .require_complete()
        .expect("child block complete");

        let pool = restarted_db.pool().await;
        let child_version: Option<String> = sqlx::query_scalar(
            "SELECT tfhe_executor_version FROM host_chain_blocks_valid \
             WHERE chain_id = $1 AND block_number = 101 AND block_hash = $2",
        )
        .bind(chain_id.as_i64())
        .bind(child.summary.hash.as_slice())
        .fetch_one(&pool)
        .await
        .expect("persisted child version");
        assert_eq!(child_version.as_deref(), Some("FHEVMExecutor v0.5.0"));
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
