use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{Block, BlockNumberOrTag, Filter, Header, Log};
use alloy::sol_types::SolEventInterface;
use anyhow::{anyhow, Result};
use fhevm_engine_common::telemetry;
use futures_util::stream::StreamExt;
use sqlx::types::Uuid;

use std::collections::{HashSet, VecDeque};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn, Level};

use clap::Parser;

use rustls;

use tokio_util::sync::CancellationToken;

use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::types::{BlockchainProvider, Handle};
use fhevm_engine_common::utils::HeartBeat;

use crate::contracts::{AclContract, TfheContract};
use crate::database::tfhe_event_propagate::{
    acl_result_handles, tfhe_result_handle, ChainId, Database, LogTfhe,
};
use crate::health_check::HealthCheck;

pub mod block_history;
use block_history::{BlockHash, BlockHistory, BlockSummary};

const REORG_RETRY_GET_LOGS: u64 = 10; // retry 10 times to get logs for a block
const RETRY_GET_LOGS_DELAY_IN_MS: u64 = 100;
const REORG_RETRY_GET_BLOCK: u64 = 10; // retry 10 times to get logs for a block
const RETRY_GET_BLOCK_DELAY_IN_MS: u64 = 100;

const DEFAULT_BLOCK_TIME: u64 = 12;

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
    pub database_url: String,

    #[arg(long, default_value = None, help = "Can be negative from last block", allow_hyphen_values = true)]
    pub start_at_block: Option<i64>,

    #[arg(long, default_value = None)]
    pub end_at_block: Option<u64>,

    #[arg(long, help = "A Coprocessor API key is needed for database access")]
    pub coprocessor_api_key: Option<Uuid>,

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
        default_value = "128",
        help = "Pre-computation dependence chain cache size"
    )]
    pub dependence_cache_size: u16,

    #[arg(
        long,
        default_value = "50",
        help = "Maximum duration in blocks to detect reorgs"
    )]
    pub reorg_maximum_duration_in_blocks: u64,

    /// service name in OTLP traces
    #[arg(long, default_value = "host-listener")]
    pub service_name: String,

    #[arg(
        long,
        default_value_t = 12,
        help = "Maximum number of block to wait before a block is finalized"
    )]
    pub finalization_maximum_duration_in_blocks: u64,
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
    end_at_block: Option<u64>,
    catchup_margin: u64,
    catchup_paging: u64,
    pub tick_timeout: HeartBeat,
    pub tick_block: HeartBeat,
    reorg_maximum_duration_in_blocks: u64, // in blocks
    block_history: BlockHistory,           // to detect reorgs
    finalization_maximum_duration_in_blocks: u64,
}

struct BlockLogs<T> {
    logs: Vec<T>,
    summary: BlockSummary,
    catchup: bool,
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
            catchup_paging: args.catchup_paging,
            catchup_margin: args.catchup_margin,
            tick_timeout: HeartBeat::default(),
            tick_block: HeartBeat::default(),
            reorg_maximum_duration_in_blocks: args
                .reorg_maximum_duration_in_blocks,
            block_history: BlockHistory::new(
                args.reorg_maximum_duration_in_blocks as usize,
            ),
        }
    }

    async fn get_chain_id(&self) -> anyhow::Result<ChainId> {
        let ws = WsConnect::new(&self.url);
        let provider = ProviderBuilder::new().connect_ws(ws).await?;
        Ok(provider.get_chain_id().await?)
    }

    async fn catchup_block_from(
        &self,
        provider: &BlockchainProvider,
    ) -> BlockNumberOrTag {
        if let Some(last_seen_block) = self.last_valid_block {
            return BlockNumberOrTag::Number(
                last_seen_block - self.catchup_margin + 1,
            );
        }
        if let Some(start_at_block) = self.start_at_block {
            if start_at_block >= 0 {
                return BlockNumberOrTag::Number(
                    start_at_block.try_into().unwrap(),
                );
            }
        }
        let Ok(last_block) = provider.get_block_number().await else {
            return BlockNumberOrTag::Earliest; // should not happend
        };
        let catch_size = if let Some(start_at_block) = self.start_at_block {
            (-start_at_block).try_into().unwrap()
        } else {
            self.catchup_margin
        };
        BlockNumberOrTag::Number(last_block - catch_size.min(last_block))
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
        let ws = WsConnect::new(&self.url).with_max_retries(0); // disabled, alloy skips events
        let provider = match ProviderBuilder::new().connect_ws(ws).await {
            Ok(provider) => provider,
            Err(_) => anyhow::bail!("Cannot get a provider"),
        };
        provider.get_logs(&filter).await.map_err(|err| {
            if eth_rpc_err::too_much_blocks_or_events(&err) {
                anyhow::anyhow!("Too much blocks or events: {err}")
            } else {
                anyhow::anyhow!("Cannot get logs for {filter:?} due to {err}")
            }
        })
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
        let finalized_block = if let Some(current_block) =
            self.block_history.tip()
        {
            // non finalized block will be post-poned until they are finalized
            current_block.number - self.finalization_maximum_duration_in_blocks
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
            let block = provider.get_block(block_id).await;
            match block {
                Ok(Some(block)) => return Ok(block),
                Ok(None) => error!(
                    block_id = ?block_id,
                    "Cannot get current block {block_id}, retrying",
                ),
                Err(err) => error!(
                    block_id = ?block_id,
                    error = %err,
                    "Cannot get current block {block_id}, retrying",
                ),
            }
            if i != REORG_RETRY_GET_BLOCK {
                tokio::time::sleep(Duration::from_millis(
                    RETRY_GET_BLOCK_DELAY_IN_MS,
                ))
                .await;
            }
        }
        Err(anyhow::anyhow!("Cannot get current block after retries"))
    }

    async fn get_block(&self, block_hash: BlockHash) -> Result<Block> {
        for i in 0..=REORG_RETRY_GET_BLOCK {
            let Some(provider) = self.provider.read().await.clone() else {
                error!("No provider, inconsistent state");
                return Err(anyhow::anyhow!("No provider, inconsistent state"));
            };
            let block = provider.get_block_by_hash(block_hash).await;
            match block {
                Ok(Some(block)) => return Ok(block),
                Ok(None) => error!(
                    block_hash = ?block_hash,
                    "Cannot get block, retrying",
                ),
                Err(err) => error!(
                    block_hash = ?block_hash,
                    error = %err,
                    "Cannot get block, retrying",
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
            "Cannot get block {block_hash} after retries"
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
            let logs = provider.get_logs(&filter).await;
            match logs {
                Ok(logs) => return Ok(logs),
                Err(err) => {
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

    async fn get_missings_ancestors(
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
            self.get_missings_ancestors(current_block_summary).await;
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

    async fn new_log_stream(&mut self, not_initialized: bool) {
        let mut retry = 20;
        loop {
            let ws = WsConnect::new(&self.url).with_max_retries(0); // disabled, alloy skips events

            match ProviderBuilder::new().connect_ws(ws).await {
                Ok(provider) => {
                    let catch_up_from =
                        self.catchup_block_from(&provider).await;
                    self.catchup_blocks = Some((
                        catch_up_from.as_number().unwrap_or(0),
                        self.end_at_block,
                    ));
                    // note subscribing to real-time before reading catchup
                    // events to have the minimal gap between the two
                    // TODO: but it does not guarantee no gap for now
                    // (implementation dependant)
                    // subscribe_logs does not honor from_block and sometime not to_block
                    // so we rely on catchup_blocks and end_at_block_reached
                    self.stream = Some(
                        provider
                            .subscribe_blocks()
                            .await
                            .expect("BLA2")
                            .into_stream(),
                    );
                    let _ = self.provider.write().await.replace(provider);
                    info!(contracts = ?self.contract_addresses, "Listening on contracts");
                    return;
                }
                Err(err) => {
                    let delay = if not_initialized {
                        if retry == 0 {
                            // TODO: remove panic and, instead, propagate the error
                            error!(
                                error = %err,
                                "Cannot connect",
                            );
                            panic!("Cannot connect due to {err}.",)
                        }
                        5
                    } else {
                        1
                    };
                    if not_initialized {
                        warn!(
                            error = %err,
                            delay_secs = delay,
                            retry = retry,
                            "Cannot connect. Will retry",
                        );
                    } else {
                        warn!(
                            error = %err,
                            delay_secs = delay,
                            "Cannot connect. Will retry infinitely",
                        );
                    }
                    retry -= 1;
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }

    async fn next_block(&mut self) -> Result<BlockOrTimeoutOrNone> {
        let Some(stream) = &mut self.stream else {
            anyhow::bail!("No stream, inconsistent state");
        };
        let next_opt_event = stream.next();
        // it assume the eventual discard of next_opt_event is handled correctly
        // by alloy if not the case, the recheck mecanism ensures it's
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
        let Some(end_at_block) = self.end_at_block else {
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
        let mut not_initialized = self.stream.is_none();
        let block_logs = loop {
            if self.stream.is_none() {
                self.new_log_stream(not_initialized).await;
                not_initialized = false;
                continue;
            };
            if self.next_blocklogs.is_empty() {
                self.consume_catchup_blocks().await;
            };
            if !self.next_blocklogs.is_empty() {
                return self.next_blocklogs.pop_front();
            };
            if self.end_at_block_reached().await {
                eprintln!(
                    "End at block reached: {}",
                    self.end_at_block.unwrap()
                );
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
                        continue;
                    };
                    warn!(
                        new_block = ?block_logs.summary,
                        block_time = self.block_time,
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
}

async fn db_insert_block(
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
) -> anyhow::Result<()> {
    info!(
        block = ?block_logs.summary,
        nb_events = block_logs.logs.len(),
        catchup = block_logs.catchup,
        "Inserting block in coprocessor",
    );
    let mut retries = 10;
    loop {
        let res = db_insert_block_no_retry(
            db,
            block_logs,
            acl_contract_address,
            tfhe_contract_address,
        )
        .await;
        let Err(err) = res else {
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

async fn db_insert_block_no_retry(
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
) -> std::result::Result<(), sqlx::Error> {
    let mut tx = db.new_transaction().await?;
    let mut is_allowed = HashSet::<Handle>::new();
    let mut tfhe_event_log = vec![];
    for log in &block_logs.logs {
        let current_address = Some(log.inner.address);
        let is_acl_address = &current_address == acl_contract_address;
        if acl_contract_address.is_none() || is_acl_address {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                info!(acl_event = ?event, "ACL event");
                let handles = acl_result_handles(&event);
                for handle in handles {
                    is_allowed.insert(handle.to_vec());
                }
                db.handle_acl_event(
                    &mut tx,
                    &event,
                    &log.transaction_hash,
                    &log.block_number,
                )
                .await?;
                continue;
            }
        }
        let is_tfhe_address = &current_address == tfhe_contract_address;
        if tfhe_contract_address.is_none() || is_tfhe_address {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner)
            {
                let log = LogTfhe {
                    event,
                    transaction_hash: log.transaction_hash,
                    is_allowed: false, // updated in the next loop
                    block_number: log.block_number,
                };
                tfhe_event_log.push(log);
                continue;
            }
        }
        if is_acl_address || is_tfhe_address {
            error!(
                event_address = ?log.inner.address,
                acl_contract_address = ?acl_contract_address,
                tfhe_contract_address = ?tfhe_contract_address,
                log = ?log,
                "Cannot decode event",
            );
        }
    }
    for tfhe_log in tfhe_event_log {
        info!(tfhe_log = ?tfhe_log, "TFHE event");
        let is_allowed =
            if let Some(result_handle) = tfhe_result_handle(&tfhe_log.event) {
                is_allowed.contains(&result_handle.to_vec())
            } else {
                false
            };
        let tfhe_log = LogTfhe {
            is_allowed,
            ..tfhe_log
        };
        db.insert_tfhe_event(&mut tx, &tfhe_log).await?;
    }
    db.mark_block_as_valid(&mut tx, &block_logs.summary).await?;
    tx.commit().await
}

pub async fn main(args: Args) -> anyhow::Result<()> {
    info!("Starting main");
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

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

    if !args.service_name.is_empty() {
        if let Err(err) = telemetry::setup_otlp(&args.service_name) {
            error!(error = %err, "Failed to setup OTLP");
        }
    }

    let mut log_iter = InfiniteLogIter::new(&args);
    let chain_id = log_iter.get_chain_id().await?;
    info!(chain_id = chain_id, "Chain ID");
    if args.database_url.is_empty() {
        error!("Database URL is required");
        panic!("Database URL is required");
    };
    let Some(coprocessor_api_key) = args.coprocessor_api_key else {
        error!("A Coprocessor API key is required to access the database");
        panic!("A Coprocessor API key is required to access the database");
    };
    let mut db = Database::new(
        &args.database_url,
        &coprocessor_api_key,
        args.dependence_cache_size,
    )
    .await?;

    if chain_id != db.chain_id {
        error!(
            chain_id_blockchain = ?chain_id,
            chain_id_db = ?db.chain_id,
            tenant_id = ?db.tenant_id,
            coprocessor_api_key = ?coprocessor_api_key,
            "Chain ID mismatch with database",
        );
        return Err(anyhow!(
            "Chain ID mismatch with database, blockchain: {} vs db: {}, tenant_id: {}, coprocessor_api_key: {}",
            chain_id,
            db.chain_id,
            db.tenant_id,
            coprocessor_api_key
        ));
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

    log_iter.new_log_stream(true).await;

    while let Some(block_logs) = log_iter.next().await {
        let _ = db_insert_block(
            &mut db,
            &block_logs,
            &acl_contract_address,
            &tfhe_contract_address,
        )
        .await;
        // logging & retry on error is already done in db_insert_block
    }
    cancel_token.cancel();
    anyhow::Result::Ok(())
}
