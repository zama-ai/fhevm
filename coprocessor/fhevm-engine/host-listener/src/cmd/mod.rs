use alloy::eips::BlockId;
use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{Block, BlockNumberOrTag, Filter, Log};
use alloy::sol_types::SolEventInterface;
use anyhow::{anyhow, Result};
use futures_util::stream::StreamExt;
use sqlx::types::Uuid;
use std::collections::VecDeque;
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;
use tokio::sync::RwLock;
use tracing::{error, info, warn, Level};

use clap::Parser;

use rustls;

use tokio_util::sync::CancellationToken;

use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::types::BlockchainProvider;
use fhevm_engine_common::utils::HeartBeat;

use crate::contracts::{AclContract, TfheContract};
use crate::database::tfhe_event_propagate::{ChainId, Database};
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
        default_value = "false",
        help = "Disable block immediate recheck"
    )]
    pub no_block_immediate_recheck: bool,

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
}

// TODO: to merge with Levent works
struct InfiniteLogIter {
    url: String,
    block_time: u64, /* A default value that is refined with real-time
                      * events data */
    no_block_immediate_recheck: bool,
    contract_addresses: Vec<Address>,
    catchup_blocks: Option<(u64, Option<u64>)>, // to do catchup blocks by chunks
    // Option<(from_block, optional to_block)>
    catchup_logs: VecDeque<Log>,
    stream: Option<SubscriptionStream<Log>>,
    pub provider: Arc<RwLock<Option<BlockchainProvider>>>, // required to maintain the stream
    last_valid_block: Option<u64>,
    start_at_block: Option<i64>,
    end_at_block: Option<u64>,
    catchup_margin: u64,
    catchup_paging: u64,
    prev_event: Option<Log>,
    current_event: Option<Log>,
    last_block_event_count: u64,
    pub tick_timeout: HeartBeat,
    pub tick_block: HeartBeat,
    reorg_maximum_duration_in_blocks: u64, // in blocks
    block_history: BlockHistory,           // to detect reorgs
}
#[allow(clippy::large_enum_variant)]
enum LogOrBlockTimeout {
    Log(Option<Log>),
    BlockTimeout,
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
            no_block_immediate_recheck: args.no_block_immediate_recheck,
            contract_addresses,
            catchup_blocks: None,
            catchup_logs: VecDeque::new(),
            stream: None,
            provider: Arc::new(RwLock::new(None)),
            last_valid_block: None,
            start_at_block: args.start_at_block,
            end_at_block: args.end_at_block,
            catchup_paging: args.catchup_paging,
            catchup_margin: args.catchup_margin,
            prev_event: None,
            current_event: None,
            last_block_event_count: 0,
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

    async fn consume_catchup_blocks(&mut self) {
        let Some((_, to_block)) = self.catchup_blocks else {
            return;
        };
        let mut paging_size = self.catchup_paging;
        let (logs, from_block, paging_to_block) = loop {
            let Some((from_block, to_block)) = self.catchup_blocks else {
                return;
            };
            let paging_to_block = if let Some(to_block) = to_block {
                to_block.min(from_block + paging_size)
            } else {
                from_block + paging_size
            };
            let mut filter = Filter::new()
                .from_block(from_block)
                .to_block(paging_to_block);
            if !self.contract_addresses.is_empty() {
                filter = filter.address(self.contract_addresses.clone())
            }
            // TODO: function
            let logs = {
                let Some(provider) = &*self.provider.read().await else {
                    error!("No provider, inconsistent state");
                    return;
                };
                provider.get_logs(&filter).await
            };
            match logs {
                Ok(logs) => break (logs, from_block, paging_to_block),
                Err(err) => {
                    if err.to_string().contains("limited") {
                        // too much blocks or logs
                        if paging_size == 1 {
                            error!(block=from_block, "Cannot catchup block {filter:?} due to {err}, aborting this block");
                            self.catchup_blocks =
                                Some((from_block + 1, to_block));
                            continue;
                        } else {
                            // retry with paging size 1
                            info!("Retrying catchup with smaller paging size");
                            paging_size = (paging_size / 2).max(1);
                            continue;
                        }
                    }
                    warn!("Cannot get logs for {filter:?} due to {err}");
                    return;
                }
            };
        };

        info!(
            nb_events = logs.len(),
            from_block = from_block,
            to_block = paging_to_block,
            "Catchup get_logs step done"
        );

        let nb_logs = logs.len();
        self.catchup_logs.extend(logs);
        self.catchup_blocks = Some((paging_to_block + 1, to_block)); //default

        if Some(paging_to_block) == to_block {
            self.catchup_blocks = None;
        } else if let Some(to_block) = to_block {
            if paging_to_block + 1 > to_block {
                self.catchup_blocks = None;
            }
        } else if nb_logs == 0 {
            // either empty or future block
            let current_block = {
                let Some(provider) = &*self.provider.read().await else {
                    error!("No provider, inconsistent state");
                    return;
                };
                provider.get_block_number().await
            };
            if let Ok(current_block) = current_block {
                if current_block < paging_to_block + 1 {
                    self.catchup_blocks = None;
                }
            }
        };
        if self.catchup_blocks.is_none() {
            info!("Catchup no next get_logs step");
        }
    }

    async fn recheck_tip_block(&mut self) {
        let block = self.block_history.tip();
        if self.no_block_immediate_recheck {
            return;
        }
        let Some(block) = block else {
            return;
        };
        let logs = match self.get_logs_at_hash(block.hash).await {
            Ok(logs) => logs,
            Err(err) => {
                error!(
                    error = ?err,
                    block = ?block.number,
                    block_hash = ?block.hash,
                    "Error, Replaying Block"
                );
                return;
            }
        };
        let last_block_event_count = self.last_block_event_count;
        info!(
            block = ?block.number,
            block_hash = ?block.hash,
            events_count = logs.len(),
            last_block_event_count = last_block_event_count,
            "Replaying Block"
        );
        if logs.is_empty() {
            return;
        }
        self.last_block_event_count = 0;
        self.catchup_logs.extend(logs);
        if let Some(event) = self.current_event.take() {
            self.catchup_logs.push_back(event);
        }
    }

    async fn get_current_block(&self) -> Result<Block> {
        let block_id = BlockId::latest();
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
        // move current to catchup as catchup could overwrite it
        if let Some(event) = self.current_event.take() {
            self.catchup_logs.push_back(event);
        }
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
            self.catchup_logs.extend(logs);
            self.block_history.add_block(missing_block);
        }
    }

    async fn check_missing_ancestors(
        &mut self,
        event_block_hash: Option<BlockHash>,
    ) {
        let has_event = event_block_hash.is_some();
        let mut current_block = None;
        let current_block_hash = if has_event {
            event_block_hash
        } else {
            // if no event is available we do the check+catchup from current block
            // can happens in block timeout
            current_block = self.get_current_block().await.ok();
            current_block.as_ref().map(|b| b.header.hash)
        };
        let Some(current_block_hash) = current_block_hash else {
            // Cannot happen, but just in case
            error!("Check missing ancestors. No current block hash, skipping the check");
            return;
        };
        if self
            .block_history
            .block_has_not_changed(&current_block_hash)
        {
            return;
        }
        // starting the detection
        let current_block = match current_block {
            Some(current_block) => current_block,
            None => match self.get_block(current_block_hash).await {
                Ok(block) => block,
                Err(_) => match self.get_current_block().await {
                    Ok(block) => block,
                    Err(_) => {
                        error!(
                            current_block_hash = ?current_block_hash,
                            "Reorg. Cannot get current block, cannot detect reorgs",
                        );
                        return; // no reorg
                    }
                },
            },
        };
        if !self.block_history.is_ready_to_detect_reorg() {
            // at fresh restart no ancestor are known
            return;
        }

        let current_block_summary = current_block.into();
        let missing_blocks =
            self.get_missings_ancestors(current_block_summary).await;

        if missing_blocks.is_empty() {
            return; // no reorg
        }
        warn!(
            nb_missing_blocks = missing_blocks.len(),
            "Missing ancestors detected.",
        );
        self.populate_catchup_logs_from_missing_blocks(missing_blocks)
            .await;
        // let's maintain the tip block by re-adding at end
        // we don't add to history from which we have no event
        // e.g. at timeout, because empty blocks are not get_logs
        if has_event {
            self.block_history.add_block(current_block_summary);
        }
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
                    let filter =
                        Filter::new().address(self.contract_addresses.clone());
                    // subscribe_logs does not honor from_block and sometime not to_block
                    // so we rely on catchup_blocks and end_at_block_reached
                    self.stream = Some(
                        provider
                            .subscribe_logs(&filter)
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

    async fn next_event_or_block_end(&mut self) -> LogOrBlockTimeout {
        let Some(stream) = &mut self.stream else {
            error!("No stream, inconsistent state");
            return LogOrBlockTimeout::Log(None); // simulate a stream end to
                                                 // force reinit
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
            Err(_) => LogOrBlockTimeout::BlockTimeout,
            Ok(opt_log) => LogOrBlockTimeout::Log(opt_log),
        }
    }

    fn end_at_block_reached(&self, log: &Log) -> bool {
        let Some(end_at_block) = self.end_at_block else {
            return false;
        };
        let Some(current_block) = log.block_number else {
            return false;
        };
        current_block > end_at_block
    }

    async fn next(&mut self) -> Option<Log> {
        let mut not_initialized = self.stream.is_none();
        self.prev_event = self.current_event.take();
        while self.current_event.is_none() {
            if self.stream.is_none() {
                self.new_log_stream(not_initialized).await;
                not_initialized = false;
                continue;
            };
            if self.catchup_logs.is_empty() {
                self.consume_catchup_blocks().await;
            };
            if let Some(log) = self.catchup_logs.pop_front() {
                if self.catchup_logs.is_empty() {
                    info!("Going back to real-time events");
                };
                self.current_event = Some(log);
                break;
            };
            match self.next_event_or_block_end().await {
                LogOrBlockTimeout::Log(None) => {
                    // the stream ends, could be a restart of the full node, or
                    // just a temporary gap
                    self.stream = None;
                    info!("Nothing to read, retrying");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                LogOrBlockTimeout::Log(Some(log)) => {
                    if self.end_at_block_reached(&log) {
                        info!(
                            block = log.block_number,
                            end_at_block = self.end_at_block,
                            "Stopping due to --end-at-block"
                        );
                        return None;
                    }
                    info!(log = ?log, "Log event");
                    let block_hash = log.block_hash;
                    let block_hash_or_0 = block_hash.unwrap_or_default();
                    let is_first_of_block = !self
                        .block_history
                        .block_has_not_changed(&block_hash_or_0);
                    self.current_event = Some(log);
                    if is_first_of_block {
                        self.tick_block.update();
                        self.recheck_tip_block().await;
                        self.check_missing_ancestors(block_hash).await;
                        if self.current_event.is_none() {
                            // current log has been pushed in catchup_logs
                            // after events from previous block (recheck or reorg)
                            continue; // jump to process events from catchup_logs
                        }
                        if let Some(block_time) =
                            self.block_history.estimated_block_time()
                        {
                            self.block_time = block_time;
                        }
                    }
                    break; // we have a log to process
                }
                LogOrBlockTimeout::BlockTimeout => {
                    self.tick_timeout.update();
                    // check reorgs update the block history
                    warn!(
                        block_time = self.block_time,
                        "Block timeout, checking for missing ancestors"
                    );
                    self.recheck_tip_block().await;
                    self.check_missing_ancestors(None).await;
                    continue;
                }
            }
        }
        if self.current_event.is_some() {
            self.last_block_event_count += 1;
        };
        self.current_event.clone()
    }

    fn is_first_of_block(&self) -> bool {
        match (&self.current_event, &self.prev_event) {
            (Some(current_event), Some(prev_event)) => {
                current_event.block_number != prev_event.block_number
            }
            _ => false,
        }
    }
}

pub async fn main(args: Args) -> anyhow::Result<()> {
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

    let mut block_tfhe_errors = 0;
    while let Some(log) = log_iter.next().await {
        if log_iter.is_first_of_block() {
            if let Some(block_number) = log.block_number {
                if block_tfhe_errors == 0 {
                    let last_valid_block = db
                        .mark_prev_block_as_valid(
                            &log_iter.current_event,
                            &log_iter.prev_event,
                        )
                        .await;
                    if last_valid_block.is_some() {
                        log_iter.last_valid_block = last_valid_block;
                    }
                } else {
                    error!(
                        block_tfhe_errors = block_tfhe_errors,
                        "Errors in tfhe events"
                    );
                    block_tfhe_errors = 0;
                }
                info!(block = block_number, "Block");
            }
        };
        if block_tfhe_errors > 0 {
            error!(block_tfhe_errors = block_tfhe_errors, "Errors in block");
        }
        let current_address = Some(log.inner.address);
        let is_tfhe_address = current_address == tfhe_contract_address;
        if tfhe_contract_address.is_none() || is_tfhe_address {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner)
            {
                info!(tfhe_event = ?event, "TFHE event");
                let log = Log {
                    inner: event,
                    block_hash: log.block_hash,
                    block_number: log.block_number,
                    block_timestamp: log.block_timestamp,
                    transaction_hash: log.transaction_hash,
                    transaction_index: log.transaction_index,
                    log_index: log.log_index,
                    removed: log.removed,
                };
                let res = db.insert_tfhe_event(&log).await;
                if let Err(err) = res {
                    block_tfhe_errors += 1;
                    error!(error = %err, "Error inserting tfhe event");
                }
                continue;
            }
        }
        let is_acl_address = current_address == acl_contract_address;
        if acl_contract_address.is_none() || is_acl_address {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                info!(acl_event = ?event, "ACL event");
                let _ = db.handle_acl_event(&event).await;
                continue;
            }
        }
        if is_acl_address || is_tfhe_address {
            error!(
                event_address = ?log.inner.address,
                acl_contract_address = ?acl_contract_address,
                tfhe_contract_address = ?tfhe_contract_address,
                "Cannot decode event",
            );
        }
    }
    cancel_token.cancel();
    anyhow::Result::Ok(())
}
