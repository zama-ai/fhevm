use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{Block, Filter, Header, Log};
use alloy::transports::ws::WebSocketConfig;
use anyhow::{anyhow, Result};
use clap::Parser;
use futures_util::stream::StreamExt;
use rustls;
use tokio::sync::RwLock;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn, Level};

use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::telemetry;
use fhevm_engine_common::types::BlockchainProvider;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};

use crate::database::ingest::{ingest_block_logs, BlockLogs, IngestOptions};
use crate::database::tfhe_event_propagate::Database;
use crate::health_check::HealthCheck;
use fhevm_engine_common::chain_id::ChainId;

pub mod block_history;
use block_history::{BlockHash, BlockHistory, BlockSummary};

const REORG_RETRY_GET_LOGS: u64 = 10; // retry 10 times to get logs for a block
const RETRY_GET_LOGS_DELAY_IN_MS: u64 = 100;
const REORG_RETRY_GET_BLOCK: u64 = 10; // retry 10 times to get logs for a block
const RETRY_GET_BLOCK_DELAY_IN_MS: u64 = 100;

const DEFAULT_BLOCK_TIME: u64 = 12;
pub const DEFAULT_DEPENDENCE_CACHE_SIZE: u16 = 10_000;
pub const DEFAULT_DEPENDENCE_BY_CONNEXITY: bool = false;
pub const DEFAULT_DEPENDENCE_CROSS_BLOCK: bool = true;

const TIMEOUT_REQUEST_ON_WEBSOCKET: u64 = 15;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "ws://0.0.0.0:8545")]
    pub url: String,

    #[arg(long)]
    pub acl_contract_address: String,

    #[arg(long)]
    pub tfhe_contract_address: String,

    #[arg(
        long,
        default_value = "postgresql://postgres:postgres@localhost:5432/coprocessor"
    )]
    pub database_url: DatabaseURL,

    #[arg(long, default_value = None, help = "Can be negative from last block", allow_hyphen_values = true)]
    pub start_at_block: Option<i64>,

    #[arg(
        long,
        default_value = None,
        help = "End catchup at this block (can be negative from last block)",
        allow_hyphen_values = true
    )]
    pub end_at_block: Option<i64>,

    #[arg(
        long,
        default_value = "5",
        help = "Catchup margin relative the last seen block"
    )]
    pub catchup_margin: u64,

    #[arg(
        long,
        default_value = "100",
        help = "Catchup paging size in number of blocks"
    )]
    pub catchup_paging: u64,

    #[arg(
        long,
        default_value_t = DEFAULT_BLOCK_TIME,
        help = "Initial block time, refined on each block"
    )]
    pub initial_block_time: u64,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,

    #[arg(long, default_value = "8080", help = "Health check port")]
    pub health_port: u16,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_CACHE_SIZE,
        help = "Pre-computation dependence chain cache size"
    )]
    pub dependence_cache_size: u16,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_BY_CONNEXITY,
        help = "Dependence chain are connected components"
    )]
    pub dependence_by_connexity: bool,

    #[arg(
        long,
        default_value_t = DEFAULT_DEPENDENCE_CROSS_BLOCK,
        help = "Dependence chain are across blocks"
    )]
    pub dependence_cross_block: bool,

    #[arg(
        long,
        default_value_t = 0,
        help = "Max weighted dependent ops per chain before slow-lane (0 disables and keeps all chains fast)"
    )]
    pub dependent_ops_max_per_chain: u32,

    #[arg(
        long,
        default_value = "50",
        help = "Maximum duration in blocks to detect reorgs"
    )]
    pub reorg_maximum_duration_in_blocks: u64,

    /// service name in OTLP traces
    #[arg(long, env = "OTEL_SERVICE_NAME", default_value = "host-listener")]
    pub service_name: String,

    #[arg(
        long,
        default_value_t = 20,
        help = "Maximum number of blocks to wait before a block is finalized"
    )]
    pub catchup_finalization_in_blocks: u64,

    #[arg(
        long,
        default_value_t = false,
        requires = "end_at_block",
        help = "Run only catchup loop without real-time subscription"
    )]
    pub only_catchup_loop: bool,

    #[arg(
        long,
        default_value_t = 60u64,
        requires = "only_catchup_loop",
        help = "Sleep duration in seconds between catchup loop iterations"
    )]
    pub catchup_loop_sleep_secs: u64,

    #[arg(
        long,
        default_value_t = TIMEOUT_REQUEST_ON_WEBSOCKET,
        help = "Timeout in seconds for RPC calls over websocket"
    )]
    pub timeout_request_websocket: u64,
}

// TODO: to merge with Levent works
struct InfiniteLogIter {
    url: String,
    block_time: u64, /* A default value that is refined with real-time
                      * events data */
    contract_addresses: Vec<Address>,
    catchup_blocks: Option<(u64, Option<u64>)>, // to do catchup blocks by chunks
    // Option<(from_block, optional to_block)>
    next_blocklogs: VecDeque<BlockLogs<Log>>, // logs already fetched but not yet processed
    stream: Option<SubscriptionStream<Header>>,
    pub provider: Arc<RwLock<Option<BlockchainProvider>>>, // required to maintain the stream
    last_valid_block: Option<u64>,
    start_at_block: Option<i64>,
    end_at_block: Option<i64>,
    absolute_end_at_block: Option<u64>,
    catchup_margin: u64,
    catchup_paging: u64,
    pub tick_timeout: HeartBeat,
    pub tick_block: HeartBeat,
    reorg_maximum_duration_in_blocks: u64, // in blocks
    block_history: BlockHistory,           // to detect reorgs
    catchup_finalization_in_blocks: u64,
    timeout_request_websocket: u64,
}

enum BlockOrTimeoutOrNone {
    Block(BlockLogs<Log>),
    Timeout,
    None,
}

mod eth_rpc_err {
    use alloy::transports::{RpcError, TransportErrorKind};
    pub fn too_much_blocks_or_events(
        err: &RpcError<TransportErrorKind>,
    ) -> bool {
        // quicknode message about asking too much blocks can vary
        // e.g. doc: -32602	eth_getLogs and eth_newFilter are limited to a 10,000 blocks range
        // e.g. testnet: ErrorResp(ErrorPayload { code: -32614, message: "eth_getLogs is limited to a 10,000 range", data: None })
        // doc: -32005	Limit Exceeded
        // also some limitation are from alloy
        // {"message":"WS connection error","err":"Space limit exceeded: Message too long: 67112162 > 67108864"}
        let msg = err.to_string();
        (msg.contains("limited to a") && msg.contains("range"))
            || msg.contains("Limit Exceeded")
            || msg.contains("Space limit exceeded: Message too long")
    }
}

fn websocket_config() -> WebSocketConfig {
    WebSocketConfig::default().max_message_size(Some(256 * 1024 * 1024)) // 256MB
}

impl InfiniteLogIter {
    fn new(args: &Args) -> Self {
        let mut contract_addresses = vec![];
        if !args.acl_contract_address.is_empty() {
            contract_addresses
                .push(Address::from_str(&args.acl_contract_address).unwrap());
        };
        if !args.tfhe_contract_address.is_empty() {
            contract_addresses
                .push(Address::from_str(&args.tfhe_contract_address).unwrap());
        };
        Self {
            url: args.url.clone(),
            block_time: args.initial_block_time,
            contract_addresses,
            catchup_blocks: None,
            next_blocklogs: VecDeque::new(),
            stream: None,
            provider: Arc::new(RwLock::new(None)),
            last_valid_block: None,
            start_at_block: args.start_at_block,
            end_at_block: args.end_at_block,
            absolute_end_at_block: None,
            catchup_paging: args.catchup_paging.max(1),
            catchup_margin: args.catchup_margin,
            tick_timeout: HeartBeat::default(),
            tick_block: HeartBeat::default(),
            reorg_maximum_duration_in_blocks: args
                .reorg_maximum_duration_in_blocks,
            block_history: BlockHistory::new(
                args.reorg_maximum_duration_in_blocks as usize,
            ),
            catchup_finalization_in_blocks: args.catchup_finalization_in_blocks,
            timeout_request_websocket: args.timeout_request_websocket,
        }
    }

    async fn get_chain_id(&self) -> anyhow::Result<ChainId> {
        let config = websocket_config();
        let ws = WsConnect::new(&self.url).with_config(config);
        let provider = ProviderBuilder::new().connect_ws(ws).await?;
        let chain_id = tokio::time::timeout(
            Duration::from_secs(self.timeout_request_websocket),
            provider.get_chain_id(),
        )
        .await??;
        Ok(ChainId::try_from(chain_id)?)
    }

    /// Resolves `end_at_block` to an absolute block number.
    /// If `end_at_block` is negative, it is interpreted as relative to the current block.
    async fn resolve_end_at_block(
        &self,
        provider: &BlockchainProvider,
    ) -> Result<Option<u64>> {
        let Some(n) = self.end_at_block else {
            return Ok(None);
        };
        if n >= 0 {
            return Ok(Some(n as u64));
        }
        let last_block = tokio::time::timeout(
            Duration::from_secs(self.timeout_request_websocket),
            provider.get_block_number(),
        )
        .await??;
        Ok(Some(last_block.saturating_sub(n.unsigned_abs())))
    }

    async fn catchup_block_from(
        &self,
        provider: &BlockchainProvider,
    ) -> Result<u64> {
        if let Some(last_seen_block) = self.last_valid_block {
            return Ok(last_seen_block - self.catchup_margin + 1);
        }
        if let Some(start_at_block) = self.start_at_block {
            if start_at_block >= 0 {
                return Ok(start_at_block.try_into()?);
            }
        }
        let block_number = tokio::time::timeout(
            Duration::from_secs(self.timeout_request_websocket),
            provider.get_block_number(),
        )
        .await?;
        let Ok(last_block) = block_number else {
            anyhow::bail!("get_block_number failed");
        };
        let catch_size = if let Some(start_at_block) = self.start_at_block {
            (-start_at_block).try_into()?
        } else {
            self.catchup_margin
        };
        Ok(last_block - catch_size.min(last_block))
    }

    async fn get_blocks_logs_range_no_retry(
        &mut self,
        from_block: u64,
        to_block: u64,
    ) -> Result<Vec<Log>> {
        let mut filter =
            Filter::new().from_block(from_block).to_block(to_block);
        if !self.contract_addresses.is_empty() {
            filter = filter.address(self.contract_addresses.clone())
        }
        // we use a specific provider to not disturb the real-time one (no buffer shared)
        let config = websocket_config();
        let ws = WsConnect::new(&self.url)
            .with_config(config)
            .with_max_retries(0); // disabled, alloy skips events
        let provider = match ProviderBuilder::new().connect_ws(ws).await {
            Ok(provider) => provider,
            Err(_) => anyhow::bail!("Cannot get a provider"),
        };
        // Timeout to prevent hanging indefinitely on buggy node
        match tokio::time::timeout(
            Duration::from_secs(self.timeout_request_websocket),
            provider.get_logs(&filter),
        )
        .await
        {
            Err(_) => {
                anyhow::bail!("Timeout getting range logs for {filter:?}")
            }
            Ok(Err(err)) => {
                if eth_rpc_err::too_much_blocks_or_events(&err) {
                    anyhow::bail!("Too much blocks or events: {err}")
                } else {
                    anyhow::bail!(
                        "Cannot get range logs for {filter:?} due to {err}"
                    )
                }
            }
            Ok(Ok(logs)) => Ok(logs),
        }
    }

    async fn deduce_block_summary(
        &self,
        number: u64,
        log: &Log,
        previous_block: Option<&BlockLogs<Log>>,
    ) -> BlockSummary {
        // find in memory
        if let Some(summary) = self.block_history.find_block_by_number(number) {
            return *summary;
        };
        // ask to chain
        if let Ok(block_header) = self.get_block_by_number(number).await {
            return block_header.into();
        };
        error!(log = ?log, number, "Cannot get block header from chain, using log data and previous block data");
        let hash = log.block_hash.unwrap_or(BlockHash::ZERO);
        // fake hash may cause this block to be refetched later because it's considered missing
        let estimated_timestamp =
            previous_block.map(|b| b.summary.timestamp).unwrap_or(0)
                + self.block_time;
        let timestamp = log.block_timestamp.unwrap_or(estimated_timestamp);
        // inaccurate timestamp is ok
        let parent_hash = previous_block
            .map(|bl| bl.summary.hash)
            .unwrap_or(BlockHash::ZERO);
        // inaccurate parent hash is ok
        BlockSummary {
            number,
            hash,
            parent_hash,
            timestamp,
        }
    }

    async fn split_by_block(
        &mut self,
        mut logs: Vec<Log>,
    ) -> Vec<BlockLogs<Log>> {
        if logs.is_empty() {
            return vec![];
        }
        let mut is_sorted = true;
        let mut last_of_block = vec![false; logs.len()];
        let mut prev_block_number = 0;
        let last_index = logs.len() - 1;
        // Sort if needed and ensure log.block_number is not None
        for log in &mut logs[0..last_index] {
            let log_block_number =
                log.block_number.unwrap_or(prev_block_number);
            if log.block_number.is_none() {
                error!(log = ?log, assumed_block_number = prev_block_number, "Log without block number, assuming same block");
                log.block_number = Some(prev_block_number);
            };
            is_sorted = is_sorted && prev_block_number <= log_block_number;
            prev_block_number = log_block_number;
        }
        if !is_sorted {
            error!("Logs are not ordered by block number in catch-up");
            logs.sort_by_key(|log| log.block_number.unwrap());
        };
        // Find blocks limits and check block number ordering
        for (index, log) in logs[0..last_index].iter().enumerate() {
            last_of_block[index] =
                logs[index + 1].block_number != log.block_number
        }
        last_of_block[last_index] = true;
        // Regroup log by block in increasing block number order
        let mut blocks_logs = vec![];
        let mut current_logs: Vec<Log> = vec![];
        for (index, log) in logs.into_iter().enumerate() {
            if !last_of_block[index] {
                current_logs.push(log);
                continue;
            }
            let summary = self
                .deduce_block_summary(
                    log.block_number.unwrap(),
                    &log,
                    blocks_logs.last(),
                )
                .await;
            current_logs.push(log);
            let block_logs = BlockLogs {
                logs: std::mem::take(&mut current_logs),
                summary,
                catchup: true,
            };
            blocks_logs.push(block_logs);
        }
        assert!(current_logs.is_empty());
        blocks_logs
    }

    async fn consume_catchup_blocks(&mut self) {
        let Some((from_block, to_block)) = self.catchup_blocks else {
            // nothing to consume
            return;
        };
        let to_block_or_max = to_block.unwrap_or(u64::MAX);
        if from_block > to_block_or_max {
            self.catchup_blocks = None;
            info!("Catchup no next get_logs step");
            return;
        }
        let finalized_block =
            if let Some(current_block) = self.block_history.tip() {
                // non finalized block will be post-poned until they are finalized
                current_block
                    .number
                    .saturating_sub(self.catchup_finalization_in_blocks)
            } else {
                // happen at service start, assuming everything is finalized
                info!("Unknown top block, assuming full finalized catchup");
                from_block + self.catchup_paging
            };
        if from_block >= finalized_block {
            // non finalized blocks are post-poned
            info!("Post-pone catchup");
            return;
        }
        let mut paging_size = self.catchup_paging;
        let mut remain_retry = 3;
        let (logs, paging_to_block) = loop {
            let paging_to_block = from_block + paging_size - 1;
            // non finalized blocks are post-poned
            let paging_to_block =
                paging_to_block.min(finalized_block).min(to_block_or_max);
            let logs = self
                .get_blocks_logs_range_no_retry(from_block, paging_to_block)
                .await;
            match logs {
                Ok(logs) => break (logs, paging_to_block),
                Err(err) if from_block == paging_to_block => {
                    // we asked only one block and it still fails
                    // continue with a limited number of retry
                    if remain_retry > 0 {
                        warn!(block=from_block, error=?err, remain_retry=remain_retry, "Catchup of block failed, retrying");
                        remain_retry -= 1;
                        continue;
                    }
                    error!(block=from_block, error=?err, "Catchup of block impossible. Will be retried later after handling a real-time message.");
                    return;
                }
                Err(err) => {
                    // too big paging size detection cannot be done reliably for all provider
                    // so it assumes the error is due to too big paging size
                    // and it retries with reduced paging, this also serves as normal retry for transient error
                    warn!(error = ?err, "Retrying catchup with smaller paging size.");
                    paging_size = (paging_size / 2).max(1);
                    continue;
                }
            }
        };
        info!(
            nb_events = logs.len(),
            from_block = from_block,
            page_to_block = paging_to_block,
            to_block = to_block,
            "Catchup get_logs step done"
        );
        let by_blocks = self.split_by_block(logs).await;
        self.next_blocklogs.extend(by_blocks);
        self.catchup_blocks = Some((paging_to_block + 1, to_block)); // end is detected at function start
    }

    async fn get_block_by_number(&self, number: u64) -> Result<Block> {
        self.get_block_by_id(BlockId::number(number)).await
    }

    async fn get_current_block(&self) -> Result<Block> {
        self.get_block_by_id(BlockId::latest()).await
    }

    async fn get_block_by_id(&self, block_id: BlockId) -> Result<Block> {
        for i in 0..=REORG_RETRY_GET_BLOCK {
            let Some(provider) = self.provider.read().await.clone() else {
                error!("No provider, inconsistent state");
                return Err(anyhow::anyhow!("No provider, inconsistent state"));
            };
            let block = tokio::time::timeout(
                Duration::from_secs(self.timeout_request_websocket),
                provider.get_block(block_id),
            );
            match block.await {
                Ok(Ok(Some(block))) => return Ok(block),
                Ok(Ok(None)) => warn!(
                    block_id = ?block_id,
                    "Cannot get block {block_id}, retrying",
                ),
                Ok(Err(err)) => warn!(
                    block_id = ?block_id,
                    error = %err,
                    "Cannot get block {block_id}, retrying",
                ),
                Err(_) => error!(
                    block_id = ?block_id,
                    "Timeout getting block {block_id}, retrying",
                ),
            }
            if i != REORG_RETRY_GET_BLOCK {
                tokio::time::sleep(Duration::from_millis(
                    RETRY_GET_BLOCK_DELAY_IN_MS,
                ))
                .await;
            }
        }
        error!(block_id = ?block_id, "Cannot get block after many retries");
        anyhow::bail!("Cannot get block {block_id} after many retries")
    }

    async fn get_block(&self, block_hash: BlockHash) -> Result<Block> {
        for i in 0..=REORG_RETRY_GET_BLOCK {
            let Some(provider) = self.provider.read().await.clone() else {
                error!("No provider, inconsistent state");
                return Err(anyhow::anyhow!("No provider, inconsistent state"));
            };
            let block = tokio::time::timeout(
                Duration::from_secs(self.timeout_request_websocket),
                provider.get_block_by_hash(block_hash),
            );
            match block.await {
                Ok(Ok(Some(block))) => return Ok(block),
                Ok(Ok(None)) => error!(
                    block_hash = ?block_hash,
                    "Cannot get block by hash, retrying",
                ),
                Ok(Err(err)) => error!(
                    block_hash = ?block_hash,
                    error = %err,
                    "Cannot get block by hash, retrying",
                ),
                Err(_) => error!(
                    block_hash = ?block_hash,
                    "Timeout getting block by hash, retrying",
                ),
            }
            if i != REORG_RETRY_GET_BLOCK {
                tokio::time::sleep(Duration::from_millis(
                    RETRY_GET_BLOCK_DELAY_IN_MS,
                ))
                .await;
            }
        }
        Err(anyhow::anyhow!(
            "Cannot get block by hash {block_hash} after retries"
        ))
    }

    async fn get_logs_at_hash(
        &self,
        block_hash: BlockHash,
    ) -> Result<Vec<Log>> {
        let mut filter = Filter::new().at_block_hash(block_hash);
        if !self.contract_addresses.is_empty() {
            filter = filter.address(self.contract_addresses.clone())
        }
        for _ in 0..REORG_RETRY_GET_LOGS {
            let Some(provider) = self.provider.read().await.clone() else {
                error!("No provider, inconsistent state");
                return Err(anyhow::anyhow!("No provider, inconsistent state"));
            };
            match tokio::time::timeout(
                Duration::from_secs(self.timeout_request_websocket),
                provider.get_logs(&filter),
            )
            .await
            {
                Err(_) => {
                    error!(
                        block_hash = ?block_hash,
                        "Timeout getting logs for block {block_hash}, retrying",
                    );
                    tokio::time::sleep(Duration::from_millis(
                        RETRY_GET_LOGS_DELAY_IN_MS,
                    ))
                    .await;
                    continue;
                }
                Ok(Ok(logs)) => {
                    return Ok(logs);
                }
                Ok(Err(err)) => {
                    error!(
                        block_hash = ?block_hash,
                        error = %err,
                        "Cannot get logs for block {block_hash}, retrying",
                    );
                    tokio::time::sleep(Duration::from_millis(
                        RETRY_GET_LOGS_DELAY_IN_MS,
                    ))
                    .await;
                    continue;
                }
            }
        }
        Err(anyhow::anyhow!(
            "Cannot get logs for block {block_hash} after retries"
        ))
    }

    async fn get_missing_ancestors(
        &self,
        mut current_block: BlockSummary,
    ) -> Vec<BlockSummary> {
        // iter on current block ancestors to collect missing blocks
        let mut missing_blocks: Vec<BlockSummary> = Vec::new();
        for i in 1..=self.reorg_maximum_duration_in_blocks {
            let parent_block_hash = current_block.parent_hash;
            if self.block_history.is_known(&parent_block_hash) {
                break;
            }
            if parent_block_hash == BlockHash::ZERO {
                // can happen in tests
                break;
            }
            let Ok(parent_block) = self.get_block(parent_block_hash).await
            else {
                error!(
                    parent_block_hash = ?parent_block_hash,
                    "Reorg chaining stopped. Cannot get parent block.",
                );
                break;
            };
            current_block = parent_block.into();
            missing_blocks.push(current_block);
            if i == self.reorg_maximum_duration_in_blocks {
                error!(
                    history_size = self.block_history.size(),
                    reorg_maximum_duration_in_blocks = self.reorg_maximum_duration_in_blocks,
                    "reorg_maximum_duration_in_blocks may be too short for the last reorg or the listener was restarted during a reorg");
            }
        }
        missing_blocks.reverse();
        missing_blocks
    }

    async fn populate_catchup_logs_from_missing_blocks(
        &mut self,
        missing_blocks: Vec<BlockSummary>,
    ) {
        for missing_block in missing_blocks {
            let Ok(logs) = self.get_logs_at_hash(missing_block.hash).await
            else {
                error!(
                    block_summary = ?missing_block,
                    "Cannot get logs for missing block, skipping it.",
                );
                continue; // skip this block
            };
            warn!(
                block_summary = ?missing_block,
                nb_events = logs.len(),
                "Missing block retrieved",
            );
            self.next_blocklogs.push_back(BlockLogs {
                logs,
                summary: missing_block,
                catchup: true,
            });
            self.block_history.add_block(missing_block);
        }
    }

    async fn check_missing_ancestors(
        &mut self,
        current_block_summary: BlockSummary,
    ) {
        if !self.block_history.is_ready_to_detect_reorg() {
            // at fresh restart no ancestor are known
            self.block_history.add_block(current_block_summary);
            return;
        }

        let missing_blocks =
            self.get_missing_ancestors(current_block_summary).await;
        if missing_blocks.is_empty() {
            // we don't add to history from which we have no event
            // e.g. at timeout, because empty blocks are not get_logs
            self.block_history.add_block(current_block_summary);
            return; // no reorg
        }
        warn!(
            nb_missing_blocks = missing_blocks.len(),
            "Missing ancestors detected.",
        );
        self.populate_catchup_logs_from_missing_blocks(missing_blocks)
            .await;
        // we don't add to history from which we have no event
        // e.g. at timeout, because empty blocks are not get_logs
        self.block_history.add_block(current_block_summary);
        warn!("Missing ancestors catchup done.");
    }

    async fn new_log_stream_no_retry(&mut self) -> Result<()> {
        let config = websocket_config();
        let ws = WsConnect::new(&self.url)
            .with_config(config)
            .with_max_retries(0); // disabled, alloy skips events
        let provider = ProviderBuilder::new().connect_ws(ws).await?;
        let catch_up_from = self.catchup_block_from(&provider).await?;
        self.absolute_end_at_block =
            self.resolve_end_at_block(&provider).await?;
        self.catchup_blocks = Some((catch_up_from, self.absolute_end_at_block));
        // note subscribing to real-time before reading catchup
        // events to have the minimal gap between the two
        // TODO: but it does not guarantee no gap for now
        // (implementation dependent)
        // subscribe_logs does not honor from_block and sometime not to_block
        // so we rely on catchup_blocks and end_at_block_reached
        self.stream = Some(provider.subscribe_blocks().await?.into_stream());
        let _ = self.provider.write().await.replace(provider);
        info!(contracts = ?self.contract_addresses, "Listening on contracts");
        Ok(())
    }

    async fn new_log_stream(&mut self) {
        while let Err(err) = self.new_log_stream_no_retry().await {
            warn!(error = %err, "Error creating new log stream, retrying");
            tokio::time::sleep(Duration::from_secs(1)).await;
        }
    }

    async fn next_block(&mut self) -> Result<BlockOrTimeoutOrNone> {
        let Some(stream) = &mut self.stream else {
            anyhow::bail!("No stream, inconsistent state");
        };
        let next_opt_event = stream.next();
        // it assume the eventual discard of next_opt_event is handled correctly
        // by alloy if not the case, the recheck mechanism ensures it's
        // only extra latency
        match tokio::time::timeout(
            Duration::from_secs(self.block_time + 2),
            next_opt_event,
        )
        .await
        {
            Err(_) => Ok(BlockOrTimeoutOrNone::Timeout),
            Ok(None) => Ok(BlockOrTimeoutOrNone::None),
            Ok(Some(header)) => Ok(BlockOrTimeoutOrNone::Block(
                self.attach_logs_to(header).await?,
            )),
        }
    }

    async fn attach_logs_to(
        &self,
        block_header: Header,
    ) -> Result<BlockLogs<Log>> {
        Ok(BlockLogs {
            logs: self.get_logs_at_hash(block_header.hash).await?,
            summary: block_header.into(),
            catchup: false,
        })
    }

    async fn find_last_block_and_logs(&self) -> Result<BlockLogs<Log>> {
        let block = self.get_current_block().await?;
        self.attach_logs_to(block.header).await
    }

    async fn end_at_block_reached(&self) -> bool {
        let Some(end_at_block) = self.absolute_end_at_block else {
            return false;
        };
        let current_block_number =
            if let Some(current_block) = self.block_history.tip() {
                current_block.number
            } else if let Ok(current_block) = self.get_current_block().await {
                current_block.header.number
            } else {
                return false;
            };
        current_block_number > end_at_block
    }

    async fn next(&mut self) -> Option<BlockLogs<Log>> {
        let block_logs = loop {
            if self.stream.is_none() {
                self.new_log_stream().await;
                continue;
            };
            if self.next_blocklogs.is_empty() {
                self.consume_catchup_blocks().await;
            };
            if !self.next_blocklogs.is_empty() {
                return self.next_blocklogs.pop_front();
            };
            if self.end_at_block_reached().await {
                match self.end_at_block {
                    Some(n) if n < 0 => eprintln!(
                        "End at block reached: {:?} (from {})",
                        self.absolute_end_at_block, n
                    ),
                    _ => eprintln!(
                        "End at block reached: {:?}",
                        self.absolute_end_at_block
                    ),
                }
                warn!("Stopping due to --end-at-block");
                return None;
            }
            match self.next_block().await {
                Err(err) => {
                    error!(error = %err, "Error getting next block");
                    self.stream = None; // to restart
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Ok(BlockOrTimeoutOrNone::None) => {
                    // the stream ends, could be a restart of the full node, or
                    // just a temporary gap
                    self.stream = None;
                    info!("Nothing to read, retrying");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                Ok(BlockOrTimeoutOrNone::Timeout) => {
                    self.tick_timeout.update();
                    let Ok(block_logs) = self.find_last_block_and_logs().await
                    else {
                        error!("Cannot get last block and logs");
                        self.stream = None; // to restart
                        continue;
                    };
                    warn!(
                        new_block = ?block_logs.summary,
                        block_time = self.block_time,
                        nb_logs = block_logs.logs.len(),
                        "Block timeout, proceed with last block"
                    );
                    break block_logs;
                }
                Ok(BlockOrTimeoutOrNone::Block(block_logs)) => {
                    self.tick_block.update();
                    info!(new_block = ?block_logs.summary, nb_logs = block_logs.logs.len(), "New block");
                    break block_logs;
                }
            }
        };
        self.check_missing_ancestors(block_logs.summary).await;
        self.next_blocklogs.push_back(block_logs);
        self.next_blocklogs.pop_front()
    }

    /// Reset state for the next catchup loop iteration.
    fn reset_for_catchup_loop(&mut self) {
        self.catchup_blocks = None;
        self.next_blocklogs.clear();
        self.last_valid_block = None;
        self.absolute_end_at_block = None;
        self.block_history =
            BlockHistory::new(self.reorg_maximum_duration_in_blocks as usize);
    }
}

async fn db_insert_block(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
    args: &Args,
) -> anyhow::Result<()> {
    info!(
        block = ?block_logs.summary,
        nb_events = block_logs.logs.len(),
        catchup = block_logs.catchup,
        "Inserting block in coprocessor",
    );
    let mut retries = 10;
    loop {
        let res = ingest_block_logs(
            chain_id,
            db,
            block_logs,
            acl_contract_address,
            tfhe_contract_address,
            IngestOptions {
                dependence_by_connexity: args.dependence_by_connexity,
                dependence_cross_block: args.dependence_cross_block,
                dependent_ops_max_per_chain: args.dependent_ops_max_per_chain,
            },
        )
        .await;
        let Err(err) = res else {
            // Notify the database of the new block
            // Delayed delegation rely on this signal to reconsider ready delegation
            if !block_logs.catchup {
                if let Err(err) =
                    db.block_notification(block_logs.summary.number).await
                {
                    error!(error = %err, "Error notifying listener for new block");
                };
            }
            return Ok(());
        };
        if retries == 0 {
            error!(error = %err, block = ?block_logs.summary, "Error inserting block");
            anyhow::bail!("Error in block insertion transaction: {err}");
        } else if retries == 1 {
            warn!(error = %err, block = ?block_logs.summary, retries = retries,
                "Retry inserting block, last attempt"
            );
        } else {
            warn!(error = %err, block = ?block_logs.summary, retries = retries, "Retry inserting block");
        }
        retries -= 1;
        db.reconnect().await;
        tokio::time::sleep(Duration::from_millis(500)).await;
    }
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    info!("Starting main");
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    // Validate catchup-only mode arguments
    if args.only_catchup_loop {
        if let Some(start) = args.start_at_block {
            if start >= 0 {
                return Err(anyhow!(
                    "--only-catchup-loop requires negative --start-at-block (e.g., -40)"
                ));
            }

            let blocks_during_sleep =
                args.catchup_loop_sleep_secs / args.initial_block_time;
            let lookback_blocks = (-start) as u64;

            if blocks_during_sleep > lookback_blocks {
                return Err(anyhow!(
                    "--catchup-loop-sleep-secs {} too large for --start-at-block {}",
                    args.catchup_loop_sleep_secs,
                    start
                ));
            }
        }
    }

    let acl_contract_address = if args.acl_contract_address.is_empty() {
        error!("--acl-contract-address cannot be empty");
        #[cfg(not(debug_assertions))] // if release code abort
        return Err(anyhow!("--acl-contract-address cannot be empty"));
        #[cfg(debug_assertions)]
        None
    } else {
        Some(
            Address::from_str(&args.acl_contract_address).map_err(|err| {
                error!(error = %err, "Invalid ACL contract address");
                anyhow!("Invalid acl contract address: {err}")
            })?,
        )
    };
    let tfhe_contract_address = if args.tfhe_contract_address.is_empty() {
        error!("--tfhe-contract-address cannot be empty");
        #[cfg(not(debug_assertions))] // if release code abort
        return Err(anyhow!("--tfhe-contract-address cannot be empty"));
        #[cfg(debug_assertions)]
        None
    } else {
        Some(
            Address::from_str(&args.tfhe_contract_address).map_err(|err| {
                error!(error = %err, "Invalid TFHE contract address");
                anyhow!("Invalid tfhe contract address: {err}")
            })?,
        )
    };

    let _otel_guard = match telemetry::init_otel(&args.service_name) {
        Ok(otel_guard) => otel_guard,
        Err(err) => {
            error!(error = %err, "Failed to setup OTLP");
            None
        }
    };

    let mut log_iter = InfiniteLogIter::new(&args);
    let chain_id = log_iter.get_chain_id().await?;
    info!(chain_id = %chain_id, "Chain ID");
    if args.database_url.as_str().is_empty() {
        error!("Database URL is required");
        panic!("Database URL is required");
    };
    let mut db =
        Database::new(&args.database_url, chain_id, args.dependence_cache_size)
            .await?;

    if args.dependent_ops_max_per_chain == 0 {
        let reset = db.reset_schedule_priorities().await?;
        if reset > 0 {
            info!(
                count = reset,
                "Slow-lane disabled: reset priorities to fast"
            );
        }
    }

    let health_check = HealthCheck {
        blockchain_timeout_tick: log_iter.tick_timeout.clone(),
        blockchain_tick: log_iter.tick_block.clone(),
        blockchain_provider: log_iter.provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
    };
    let cancel_token = CancellationToken::new();
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check),
        args.health_port,
        cancel_token.clone(),
    );
    tokio::spawn(async move { health_check_server.start().await });

    if log_iter.start_at_block.is_none() {
        log_iter.start_at_block = db
            .read_last_valid_block()
            .await
            .map(|n| n - args.catchup_margin as i64);
    }

    // Check connection works
    log_iter.new_log_stream_no_retry().await?;

    loop {
        log_iter.stream = None; // force new connection each iteration

        while let Some(block_logs) = log_iter.next().await {
            if args.only_catchup_loop && !block_logs.catchup {
                break;
            }
            let status = db_insert_block(
                chain_id,
                &mut db,
                &block_logs,
                &acl_contract_address,
                &tfhe_contract_address,
                &args,
            )
            .await;
            if status.is_err() {
                // logging & retry on error is already done in db_insert_block
                continue;
            };
            log_iter.last_valid_block = Some(
                block_logs
                    .summary
                    .number
                    .max(log_iter.last_valid_block.unwrap_or(0)),
            );
        }

        if !args.only_catchup_loop {
            break;
        }

        info!(
            sleep_secs = args.catchup_loop_sleep_secs,
            "Catchup loop iteration complete, sleeping"
        );
        tokio::time::sleep(Duration::from_secs(args.catchup_loop_sleep_secs))
            .await;

        // Reset state for next iteration
        log_iter.reset_for_catchup_loop();
    }
    cancel_token.cancel();
    anyhow::Result::Ok(())
}
