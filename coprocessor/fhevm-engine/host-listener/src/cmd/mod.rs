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
use tokio::task::JoinSet;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn, Level};

use std::collections::{BTreeMap, VecDeque};
use std::str::FromStr;
use std::sync::Arc;
use std::time::Duration;

use fhevm_engine_common::database::connect_pool_with_options;
use fhevm_engine_common::drift_revert;
use fhevm_engine_common::healthz_server::HttpServer as HealthHttpServer;
use fhevm_engine_common::types::BlockchainProvider;
use fhevm_engine_common::utils::{DatabaseURL, HeartBeat};
use fhevm_engine_common::versioning::{reconcile_stack_mode, StackMode};
use sqlx::postgres::PgPoolOptions;

use crate::database::history_verification::verify_finalized_history;
use crate::database::ingest::{
    update_finalized_blocks, validate_block_logs, BlockLogs, IngestOptions,
};
use crate::database::tfhe_event_propagate::{
    spawn_stack_version_listener, Database,
};
use crate::health_check::HealthCheck;
use crate::kms_generation::aws_s3::AwsS3Client;
use crate::kms_generation::process_kms_generation_activations;

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

    #[arg(long, default_value = "")]
    pub kms_generation_address: String,

    #[command(flatten)]
    pub protocol_config: crate::protocol_config::ProtocolConfigArgs,

    #[arg(long, default_value = "")]
    pub confidential_bridge_address: String,

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
        help = "Max dependent ops per chain before slow-lane (0 disables; startup promotes all chains to fast)"
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

    #[arg(
        long,
        default_value_t = false,
        help = "Stop injecting the synthetic work a GCS listener uses to anchor consensus"
    )]
    pub disable_synthetic_ops: bool,

    /// Print the compiled-in coprocessor stack version and exit.
    #[arg(long)]
    pub stack_version: bool,
}

// TODO: to merge with Levent works
pub struct InfiniteLogIter {
    url: String,
    // Chain block interval (e.g. 12s for Sepolia); combined with a fixed
    // +5s slack as the WS subscription timeout budget. Not currently
    // refined at runtime — see fhevm-internal#1382.
    block_time: u64,
    contract_addresses: Vec<Address>,
    catchup_blocks: Option<(u64, Option<u64>)>, // to do catchup blocks by chunks
    pending_catchup_page: Option<CatchupPage>,
    // Option<(from_block, optional to_block)>
    next_blocklogs: VecDeque<BlockLogs<Log>>, // logs already fetched but not yet processed
    stream: Option<SubscriptionStream<Header>>,
    #[cfg(test)]
    catchup_provider: Option<BlockchainProvider>,
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
    tracks_kms_generation: bool,
    rpc_finalized_block: Option<i64>,
    timeout_request_websocket: u64,
}

struct CatchupPage {
    from_block: u64,
    next_page: u64,
    remaining_blocks: usize,
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
    WebSocketConfig::default()
        .max_message_size(Some(256 * 1024 * 1024))
        .max_frame_size(Some(256 * 1024 * 1024))
}

impl InfiniteLogIter {
    pub fn new(args: &Args) -> Self {
        let mut contract_addresses = vec![];
        if !args.acl_contract_address.is_empty() {
            contract_addresses
                .push(Address::from_str(&args.acl_contract_address).unwrap());
        };
        if !args.tfhe_contract_address.is_empty() {
            contract_addresses
                .push(Address::from_str(&args.tfhe_contract_address).unwrap());
        };
        if !args.kms_generation_address.is_empty() {
            contract_addresses
                .push(Address::from_str(&args.kms_generation_address).unwrap());
        };
        if !args.protocol_config.address.is_empty() {
            contract_addresses.push(
                Address::from_str(&args.protocol_config.address).unwrap(),
            );
        };
        if !args.confidential_bridge_address.is_empty() {
            contract_addresses.push(
                Address::from_str(&args.confidential_bridge_address).unwrap(),
            );
        };
        Self {
            url: args.url.clone(),
            block_time: args.initial_block_time,
            contract_addresses,
            catchup_blocks: None,
            pending_catchup_page: None,
            next_blocklogs: VecDeque::new(),
            stream: None,
            #[cfg(test)]
            catchup_provider: None,
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
            tracks_kms_generation: !args.kms_generation_address.is_empty(),
            rpc_finalized_block: None,
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
            return Ok(last_seen_block
                .saturating_add(1)
                .saturating_sub(self.catchup_margin));
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
            self.catchup_finalization_in_blocks
                .saturating_add(self.catchup_margin)
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
        let provider = self.connect_catchup_provider().await?;
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

    async fn connect_catchup_provider(&self) -> Result<BlockchainProvider> {
        #[cfg(test)]
        if let Some(provider) = &self.catchup_provider {
            return Ok(provider.clone());
        }
        // we use a specific provider to not disturb the real-time one (no buffer shared)
        let config = websocket_config();
        let ws = WsConnect::new(&self.url)
            .with_config(config)
            // disabled, retried explicitly later
            .with_max_retries(0);
        // Timeout to prevent slow reconnection
        let provider = tokio::time::timeout(
            Duration::from_secs(self.timeout_request_websocket),
            ProviderBuilder::new().connect_ws(ws),
        );
        match provider.await {
            Err(_) => {
                anyhow::bail!("Timeout getting provider for logs range")
            }
            Ok(Err(err)) => {
                anyhow::bail!("Cannot get provider for logs range due to {err}")
            }
            Ok(Ok(provider)) => Ok(provider),
        }
    }

    async fn split_by_block(
        &mut self,
        logs: Vec<Log>,
    ) -> Result<Vec<BlockLogs<Log>>> {
        let mut by_number = BTreeMap::<u64, Vec<Log>>::new();
        for log in logs {
            let number = log.block_number.ok_or_else(|| {
                anyhow!("Catchup log is missing its block number")
            })?;
            by_number.entry(number).or_default().push(log);
        }
        if by_number.is_empty() {
            return Ok(vec![]);
        }
        let head = match self.block_history.tip() {
            Some(tip) => tip.number,
            None => self.get_current_block().await?.header.number,
        };
        let finalized_through =
            head.saturating_sub(self.catchup_finalization_in_blocks);
        let mut blocks_logs = vec![];
        for (number, logs) in by_number {
            // A cached header at this height may belong to an orphan. Read the
            // current header and reject the whole page if the range RPC raced
            // with a reorg. The caller retains its cursor and refetches it.
            let summary: BlockSummary =
                self.get_block_by_number(number).await?.into();
            if summary.number != number
                || logs.iter().any(|log| {
                    log.block_hash != Some(summary.hash) || log.removed
                })
            {
                anyhow::bail!(
                    "Catchup logs do not match canonical block {number} ({})",
                    summary.hash
                );
            }
            let block_logs = BlockLogs {
                logs,
                summary,
                catchup: true,
                finalized: summary.number <= finalized_through,
            };
            validate_block_logs(&block_logs)?;
            blocks_logs.push(block_logs);
        }
        Ok(blocks_logs)
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
            current_block
                .number
                .saturating_sub(self.catchup_finalization_in_blocks)
        } else {
            // Startup must obey the same finality lag as steady state.
            // Otherwise the first page can finalize a block mid-reorg.
            match self.get_current_block().await {
                Ok(block) => block
                    .header
                    .number
                    .saturating_sub(self.catchup_finalization_in_blocks),
                Err(err) => {
                    warn!(error = %err, "Cannot determine catchup finality boundary");
                    return;
                }
            }
        };
        if self.tracks_kms_generation {
            match self.get_block_by_id(BlockId::finalized()).await {
                Ok(block) => {
                    self.rpc_finalized_block =
                        i64::try_from(block.header.number).ok();
                }
                Err(error) => {
                    self.rpc_finalized_block = None;
                    warn!(%error, "Cannot resolve finalized block, postponing compressed key migration");
                }
            }
        }
        if from_block > finalized_block {
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
        let by_blocks = match self.split_by_block(logs).await {
            Ok(blocks) => blocks,
            Err(err) => {
                warn!(error = %err, from_block, paging_to_block,
                    "Catchup snapshot changed, retaining cursor for refetch");
                return;
            }
        };
        // Historical pages are blockchain progress too, including empty ones.
        // Draining catchup can bypass the realtime heartbeat for a long time.
        self.tick_block.update();
        let remaining_blocks = by_blocks.len();
        self.next_blocklogs.extend(by_blocks);
        self.pending_catchup_page = Some(CatchupPage {
            from_block,
            next_page: paging_to_block + 1,
            remaining_blocks,
        });
        if remaining_blocks == 0 {
            self.finish_pending_catchup_page();
        }
    }

    fn finish_pending_catchup_page(&mut self) {
        let Some(page) = &self.pending_catchup_page else {
            return;
        };
        if page.remaining_blocks != 0 {
            return;
        }
        self.catchup_blocks =
            Some((page.next_page, self.absolute_end_at_block));
        self.pending_catchup_page = None;
    }

    fn acknowledge_catchup_block(&mut self) {
        if let Some(page) = &mut self.pending_catchup_page {
            page.remaining_blocks = page.remaining_blocks.saturating_sub(1);
        }
        self.finish_pending_catchup_page();
    }

    pub async fn get_block_by_number(&self, number: u64) -> Result<Block> {
        self.get_block_by_id(BlockId::number(number)).await
    }

    async fn verify_committed_history(&self, db: &Database) -> Result<()> {
        verify_finalized_history(db, |number| async move {
            let block = self.get_block_by_number(number).await?;
            // A cold scan may outlast the probe interval. Successful header
            // reads are progress; failed reads must not sustain liveness.
            self.tick_block.update();
            Ok(block.header.hash)
        })
        .await
        .map(|_| ())
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
        &mut self,
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
            // Fetched ancestors are not committed coverage. Retain the gap
            // in memory even if fetching the parent fails.
            self.extend_catchup_from(current_block.number.saturating_sub(1));
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
                finalized: false, // let catchups with finality conditions do the finalize later
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
        // A reconnect must not discard an unfinished catchup range merely
        // because realtime ingestion has advanced the high watermark.
        let catch_up_from = self
            .catchup_blocks
            .map(|(from, _)| from.min(catch_up_from))
            .unwrap_or(catch_up_from);
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

    pub async fn init_provider_for_rpc(&mut self) -> Result<()> {
        let config = websocket_config();
        let ws = WsConnect::new(&self.url).with_config(config);
        let provider = ProviderBuilder::new().connect_ws(ws).await?;
        let _ = self.provider.write().await.replace(provider);
        Ok(())
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
            Duration::from_secs(self.block_time + 5),
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
            finalized: false,
        })
    }

    pub(crate) async fn queue_canonical_block_for_retry(
        &mut self,
        number: u64,
    ) -> Result<()> {
        // This repair is queued after a live block's finalization pass, outside
        // any range page. Keep its acknowledgement from consuming a page slot.
        anyhow::ensure!(
            self.pending_catchup_page.is_none(),
            "cannot queue a finalization repair while a catchup page is pending"
        );
        let block_logs = self.canonical_block_for_recovery(number).await?;
        self.next_blocklogs.push_front(block_logs);
        Ok(())
    }

    async fn canonical_block_for_recovery(
        &self,
        number: u64,
    ) -> Result<BlockLogs<Log>> {
        let header = self.get_block_by_number(number).await?.header;
        anyhow::ensure!(header.number == number, "wrong recovery block height");
        let mut block_logs = self.attach_logs_to(header).await?;
        let confirmed = self.get_block_by_number(number).await?.header;
        anyhow::ensure!(
            confirmed.number == number
                && confirmed.hash == block_logs.summary.hash,
            "canonical block changed during finalization recovery"
        );
        validate_block_logs(&block_logs)?;
        // Only the finalizer requests these repairs, for heights already below
        // its confirmation boundary. Treat the coherent canonical observation
        // as authoritative so a contradiction with retained finalized history
        // is contained rather than silently discarded as a delayed live fork.
        block_logs.catchup = true;
        block_logs.finalized = true;
        self.tick_block.update();
        Ok(block_logs)
    }

    async fn find_last_block_and_logs(&self) -> Result<BlockLogs<Log>> {
        let block = self.get_current_block().await?;
        self.attach_logs_to(block.header).await
    }

    async fn end_at_block_reached(&self) -> bool {
        let Some(end_at_block) = self.absolute_end_at_block else {
            return false;
        };
        if self
            .catchup_blocks
            .is_some_and(|(from, _)| from <= end_at_block)
        {
            return false;
        }
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
                let before = self.catchup_blocks;
                self.consume_catchup_blocks().await;
                // An empty page is still successful progress. Keep draining
                // historical pages before considering the realtime stream or
                // --end-at-block, which may already be behind the chain tip.
                if self.next_blocklogs.is_empty()
                    && self.catchup_blocks != before
                {
                    continue;
                }
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
        // Keep the next unread height even when the previous range was empty
        // or complete. Finality waits and slow iterations must not let the new
        // relative start skip work. Reconnection can still widen this range to
        // include the configured lookback.
        let next_block =
            self.catchup_blocks.map(|(from, _)| from).or_else(|| {
                self.absolute_end_at_block.map(|end| end.saturating_add(1))
            });
        self.catchup_blocks = next_block.map(|from| (from, None));
        self.pending_catchup_page = None;
        self.next_blocklogs.clear();
        self.last_valid_block = None;
        self.absolute_end_at_block = None;
        self.block_history =
            BlockHistory::new(self.reorg_maximum_duration_in_blocks as usize);
    }

    /// Retain an earlier gap until catchup has successfully ingested it.
    fn extend_catchup_from(&mut self, from: u64) {
        let from = self.catchup_blocks.map_or(from, |(next, _)| next.min(from));
        self.catchup_blocks = Some((from, self.absolute_end_at_block));
    }

    /// Refetch all uncommitted work, including queued ancestors older than the
    /// failed block. History contains fetched hashes, not successful DB commits.
    /// This cursor is in memory: gaps older than the startup lookback still
    /// require the HTTP poller or an explicit replay after a process restart.
    fn retry_from_block(&mut self, block_number: u64) {
        let mut from = self
            .pending_catchup_page
            .as_ref()
            .map_or(block_number, |page| page.from_block.min(block_number));
        for queued in &self.next_blocklogs {
            from = from.min(queued.summary.number);
        }
        self.extend_catchup_from(from);
        self.next_blocklogs.clear();
        self.pending_catchup_page = None;
        self.last_valid_block = Some(
            self.last_valid_block
                .unwrap_or(from)
                .min(from.saturating_sub(1)),
        );
        self.block_history =
            BlockHistory::new(self.reorg_maximum_duration_in_blocks as usize);
    }
}

#[allow(clippy::too_many_arguments)]
async fn db_insert_block(
    chain_id: ChainId,
    db: &mut Database,
    block_logs: &BlockLogs<Log>,
    acl_contract_address: &Option<Address>,
    tfhe_contract_address: &Option<Address>,
    kms_generation_address: &Option<Address>,
    protocol_config_address: &Option<Address>,
    confidential_bridge_address: &Option<Address>,
    args: &Args,
    is_protocol_config_listener: bool,
    log_iter: &InfiniteLogIter,
) -> anyhow::Result<crate::database::ingest::IngestResult> {
    // An inconsistent RPC snapshot must be fetched again; retrying the same
    // immutable batch against the database cannot repair it.
    validate_block_logs(block_logs)?;
    info!(
        block = ?block_logs.summary,
        nb_events = block_logs.logs.len(),
        catchup = block_logs.catchup,
        "Inserting block in coprocessor",
    );
    let mut retries = 10;
    loop {
        let res = crate::database::ingest::ingest_with_history_recovery(
            chain_id,
            db,
            block_logs,
            acl_contract_address,
            tfhe_contract_address,
            kms_generation_address,
            protocol_config_address,
            confidential_bridge_address,
            IngestOptions {
                dependence_by_connexity: args.dependence_by_connexity,
                dependence_cross_block: args.dependence_cross_block,
                dependent_ops_max_per_chain: args.dependent_ops_max_per_chain,
                is_protocol_config_listener,
                disable_synthetic_ops: args.disable_synthetic_ops,
            },
            |number| log_iter.canonical_block_for_recovery(number),
        )
        .await;
        let err = match res {
            Ok(result) => return Ok(result),
            Err(err) => err,
        };
        // Repeating rejected event bytes cannot repair them. Exit so restart
        // refetches from durable progress; never advance past this block.
        if matches!(
            err.downcast_ref::<sqlx::Error>(),
            Some(sqlx::Error::Protocol(_))
        ) {
            return Err(err.context(format!(
                "Rejected block {}; refetch required",
                block_logs.summary.number
            )));
        }
        // RPC recovery must refetch later, without rebuilding the DB pool.
        if err.downcast_ref::<sqlx::Error>().is_none() {
            return Err(err);
        }
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

/// Pause without consuming catchup pages or failing the liveness check.
pub(crate) async fn wait_while_paused(
    stack_mode: &StackMode,
    heartbeat: &HeartBeat,
    cancel: &CancellationToken,
) {
    if stack_mode.is_paused() {
        info!("Listener stack retired; pausing ingestion");
    }
    let mut interval = tokio::time::interval(Duration::from_secs(5));
    while stack_mode.is_paused() {
        tokio::select! {
            _ = cancel.cancelled() => return,
            _ = interval.tick() => heartbeat.update(),
        }
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
    let kms_generation_address = if args.kms_generation_address.is_empty() {
        None
    } else {
        Some(Address::from_str(&args.kms_generation_address).map_err(
            |err| {
                error!(error = %err, "Invalid KMSGeneration contract address");
                anyhow!("Invalid KMSGeneration contract address: {err}")
            },
        )?)
    };
    let protocol_config_address = args.protocol_config.parsed_address()?;
    let confidential_bridge_address = if args
        .confidential_bridge_address
        .is_empty()
    {
        None
    } else {
        Some(
                Address::from_str(&args.confidential_bridge_address).map_err(
                    |err| {
                        error!(error = %err, "Invalid ConfidentialBridge contract address");
                        anyhow!("Invalid ConfidentialBridge contract address: {err}")
                    },
                )?,
            )
    };
    let aws_s3_client = AwsS3Client {};

    let mut log_iter = InfiniteLogIter::new(&args);
    let chain_id = log_iter.get_chain_id().await?;

    info!(chain_id = %chain_id, "Chain ID");
    let is_protocol_config_listener =
        crate::protocol_config::resolve_protocol_config_listener(
            args.protocol_config.chain_id,
            chain_id.as_u64(),
            protocol_config_address,
        )?;
    if args.database_url.as_str().is_empty() {
        error!("Database URL is required");
        panic!("Database URL is required");
    };

    let gcs_mode = match fhevm_engine_common::versioning::resolve_gcs_mode(
        args.database_url.as_str(),
    )
    .await
    {
        Ok(gcs_mode) => gcs_mode,
        Err(err) => {
            error!(error = %err, "Failed to resolve gcs_mode from versioning table");
            return Err(err);
        }
    };

    let stack_mode = StackMode::new(gcs_mode);
    let mut db = Database::new_with_stack_mode(
        &args.database_url,
        chain_id,
        args.dependence_cache_size,
        stack_mode.clone(),
    )
    .await?;

    let health_check = HealthCheck {
        blockchain_timeout_tick: log_iter.tick_timeout.clone(),
        blockchain_tick: log_iter.tick_block.clone(),
        blockchain_provider: log_iter.provider.clone(),
        database_pool: db.pool.clone(),
        database_tick: db.tick.clone(),
        recovery_paused: Default::default(),
    };

    let cancel_token = CancellationToken::new();
    let _cancel_on_exit = cancel_token.clone().drop_guard();

    // Runtime stack mode + `event_stack_version_upgraded` listener: at cutover
    // this (blue) stack is retired and `stack_mode` flips to paused, turning
    // the ingest loop below into a no-op (no DB writes).
    spawn_stack_version_listener(
        db.clone(),
        stack_mode.clone(),
        cancel_token.clone(),
    );

    // Start health check before drift-revert init so the service stays
    // healthy while waiting for a revert to complete.
    let health_check_server = HealthHttpServer::new(
        Arc::new(health_check.clone()),
        args.health_port,
        cancel_token.clone(),
    );
    tokio::spawn(async move {
        if let Err(err) = health_check_server.start().await {
            error!(error = %err, "Health check server failed");
        }
    });

    let work = async {
        let mut background_tasks = JoinSet::new();

        // Drift-revert: must run before any DB state reads so we don't read
        // pre-revert state.
        let (drift_revert_pool, _pool_refresh_handle) =
            connect_pool_with_options(
                &args.database_url,
                PgPoolOptions::new().max_connections(1),
                Some(&cancel_token),
            )
            .await?;
        drift_revert::init(
            drift_revert_pool,
            cancel_token.clone(),
            None,
            drift_revert::WatcherTimeouts::default(),
        )
        .await?;

        // Notifications cover a running listener; reconcile explicitly when a
        // retired stack starts after the cutover notification has already passed.
        reconcile_stack_mode(&db.pool().await, &stack_mode).await?;
        wait_while_paused(&stack_mode, &log_iter.tick_block, &cancel_token)
            .await;

        if args.dependent_ops_max_per_chain == 0 {
            let promoted = db.promote_all_dep_chains_to_fast_priority().await?;
            if promoted > 0 {
                info!(
                count = promoted,
                "Slow-lane disabled: promoted all chains to fast on startup"
            );
            }
        }

        if log_iter.start_at_block.is_none() {
            log_iter.start_at_block =
                db.read_last_valid_block_checked().await?.map(|n| {
                    n.saturating_sub_unsigned(
                        args.catchup_finalization_in_blocks
                            .saturating_add(args.catchup_margin),
                    )
                    .max(0)
                });
        }

        // Check connection works
        log_iter.new_log_stream_no_retry().await?;
        // Check retained segments even if catchup has no logs or terminates at
        // --end-at-block without ever reaching the realtime finalization loop.
        loop {
            match log_iter.verify_committed_history(&db).await {
                Ok(()) => break,
                Err(err)
                    if matches!(
                        err.downcast_ref::<sqlx::Error>(),
                        Some(sqlx::Error::Protocol(_))
                    ) =>
                {
                    return Err(err)
                }
                Err(err) => {
                    warn!(%err, "Cannot verify startup history; retrying");
                    log_iter.tick_timeout.update();
                    tokio::time::sleep(Duration::from_millis(500)).await;
                    // Verification may have lost its websocket provider.
                    // Reconnect before retrying instead of polling a closed
                    // transport forever; unfinished catchup is preserved.
                    log_iter.new_log_stream().await;
                }
            }
        }

        loop {
            log_iter.stream = None; // force new connection each iteration

            while let Some(block_logs) = log_iter.next().await {
                // Keep the pending page intact while retired. Dropping its blocks
                // without acknowledgement would repeatedly refetch the same page.
                wait_while_paused(
                    &stack_mode,
                    &log_iter.tick_block,
                    &cancel_token,
                )
                .await;
                if args.only_catchup_loop && !block_logs.catchup {
                    break;
                }
                if !block_logs.catchup
                    && log_iter
                        .absolute_end_at_block
                        .is_some_and(|end| block_logs.summary.number > end)
                {
                    // Keep observing the tip so pending catchup can reach finality,
                    // without ingesting realtime blocks beyond a requested range.
                    continue;
                }
                if block_logs.finalized {
                    if let Err(err) =
                        log_iter.verify_committed_history(&db).await
                    {
                        if matches!(
                            err.downcast_ref::<sqlx::Error>(),
                            Some(sqlx::Error::Protocol(_))
                        ) {
                            return Err(err);
                        }
                        warn!(%err, block_number = block_logs.summary.number,
                            "Cannot verify committed history; retaining catchup for retry");
                        log_iter.retry_from_block(block_logs.summary.number);
                        tokio::time::sleep(Duration::from_millis(500)).await;
                        continue;
                    }
                }
                let status = db_insert_block(
                    chain_id,
                    &mut db,
                    &block_logs,
                    &acl_contract_address,
                    &tfhe_contract_address,
                    &kms_generation_address,
                    &protocol_config_address,
                    &confidential_bridge_address,
                    &args,
                    is_protocol_config_listener,
                    &log_iter,
                )
                .await;
                if matches!(
                    status,
                    Ok(crate::database::ingest::IngestResult::NeedsHistory(_))
                ) {
                    // Recovery committed a bounded batch. Preserve this exact
                    // delivery and its page acknowledgement until it can attach.
                    log_iter.next_blocklogs.push_front(block_logs);
                    tokio::task::yield_now().await;
                    continue;
                }
                if let Err(err) = status {
                    // Keep CFI fail-closed admission: rejected event metadata and
                    // committed-history conflicts must not be skipped.
                    if matches!(
                        err.downcast_ref::<sqlx::Error>(),
                        Some(sqlx::Error::Protocol(_))
                    ) {
                        return Err(err);
                    }
                    // logging & retry on error is already done in db_insert_block
                    log_iter.retry_from_block(block_logs.summary.number);
                    continue;
                };
                // Advance the page only after its block transactions commit.
                log_iter.acknowledge_catchup_block();
                log_iter.last_valid_block = Some(
                    block_logs
                        .summary
                        .number
                        .max(log_iter.last_valid_block.unwrap_or(0)),
                );
                if !block_logs.catchup {
                    if kms_generation_address.is_some() {
                        match log_iter
                            .get_block_by_id(BlockId::finalized())
                            .await
                        {
                            Ok(finalized) => {
                                log_iter.rpc_finalized_block =
                                    i64::try_from(finalized.header.number).ok();
                            }
                            Err(error) => {
                                log_iter.rpc_finalized_block = None;
                                warn!(%error, "Cannot resolve finalized block, postponing compressed key migration");
                            }
                        }
                    }
                    update_finalized_blocks(
                        &mut db,
                        &mut log_iter,
                        block_logs.summary.number,
                        args.catchup_finalization_in_blocks,
                    )
                    .await;
                }
                if kms_generation_address.is_some() {
                    background_tasks.spawn(process_kms_generation_activations(
                        db.pool.read().await.clone(),
                        aws_s3_client,
                        log_iter.rpc_finalized_block,
                    ));
                }
            }

            if !args.only_catchup_loop {
                break;
            }

            info!(
                sleep_secs = args.catchup_loop_sleep_secs,
                "Catchup loop iteration complete, sleeping"
            );
            tokio::time::sleep(Duration::from_secs(
                args.catchup_loop_sleep_secs,
            ))
            .await;

            // Reset state for next iteration
            log_iter.reset_for_catchup_loop();
        }
        for t in background_tasks.join_all().await.iter() {
            if let Err(err) = t {
                error!(error = %err, "Error in background task");
            }
        }
        anyhow::Result::Ok(())
    };
    health_check.run_with_containment(&cancel_token, work).await
}

#[cfg(test)]
mod tests {
    use std::collections::VecDeque;
    use std::sync::Arc;

    use alloy::node_bindings::Anvil;
    use alloy::providers::ext::AnvilApi;
    use alloy::providers::{Provider, ProviderBuilder, WsConnect};
    use tokio::sync::RwLock;

    use fhevm_engine_common::utils::HeartBeat;

    use super::*;

    fn new_test_iter(reorg_max: u64) -> InfiniteLogIter {
        InfiniteLogIter {
            url: String::new(),
            block_time: 12,
            contract_addresses: vec![],
            catchup_blocks: None,
            pending_catchup_page: None,
            next_blocklogs: VecDeque::new(),
            stream: None,
            catchup_provider: None,
            provider: Arc::new(RwLock::new(None)),
            last_valid_block: None,
            start_at_block: None,
            end_at_block: None,
            absolute_end_at_block: None,
            catchup_margin: 5,
            catchup_paging: 100,
            tick_timeout: HeartBeat::new(),
            tick_block: HeartBeat::new(),
            reorg_maximum_duration_in_blocks: reorg_max,
            block_history: BlockHistory::new(reorg_max as usize),
            catchup_finalization_in_blocks: 20,
            tracks_kms_generation: false,
            rpc_finalized_block: None,
            timeout_request_websocket: 15,
        }
    }

    fn rpc_block(hash: u8) -> Block {
        let mut block: Block = Block::default();
        block.header.hash = BlockHash::with_last_byte(hash);
        block.header.inner.number = 42;
        block
    }

    #[tokio::test]
    async fn finalization_repair_queues_empty_and_nonempty_blocks() {
        use alloy::transports::mock::Asserter;
        for with_event in [false, true] {
            let canonical = rpc_block(2);
            let logs: Vec<Log> = if with_event {
                vec![Log {
                    block_hash: Some(canonical.header.hash),
                    block_number: Some(42),
                    transaction_hash: Some(BlockHash::with_last_byte(3)),
                    log_index: Some(0),
                    ..Default::default()
                }]
            } else {
                vec![]
            };
            let asserter = Asserter::new();
            asserter.push_success(&canonical);
            asserter.push_success(&logs);
            asserter.push_success(&canonical);
            let mut iter = new_test_iter(50);
            iter.catchup_blocks = Some((100, None));
            *iter.provider.write().await =
                Some(ProviderBuilder::new().connect_mocked_client(asserter));
            iter.queue_canonical_block_for_retry(42).await.unwrap();
            let repair = iter.next_blocklogs.pop_front().unwrap();
            assert_eq!(repair.summary.hash, canonical.header.hash);
            assert_eq!(repair.logs.len(), usize::from(with_event));
            assert!(repair.catchup);
            assert!(repair.finalized);
            iter.acknowledge_catchup_block();
            assert_eq!(iter.catchup_blocks, Some((100, None)));
        }
    }

    #[tokio::test]
    async fn finalization_repair_preserves_unrelated_page_acknowledgements() {
        let mut iter = new_test_iter(50);
        iter.catchup_blocks = Some((100, None));
        iter.pending_catchup_page = Some(CatchupPage {
            from_block: 100,
            next_page: 105,
            remaining_blocks: 2,
        });
        assert!(iter.queue_canonical_block_for_retry(42).await.is_err());
        assert!(iter.next_blocklogs.is_empty());
        assert_eq!(iter.pending_catchup_page.unwrap().remaining_blocks, 2);
        assert_eq!(iter.catchup_blocks, Some((100, None)));
    }

    #[tokio::test]
    async fn finalization_repair_rejects_changed_or_unavailable_headers() {
        use alloy::transports::mock::Asserter;
        for changed in [false, true] {
            let asserter = Asserter::new();
            if changed {
                asserter.push_success(&rpc_block(2));
                asserter.push_success(&Vec::<Log>::new());
                asserter.push_success(&rpc_block(3));
            } else {
                for _ in 0..=REORG_RETRY_GET_BLOCK {
                    asserter.push_success(&Option::<Block>::None);
                }
            }
            let mut iter = new_test_iter(50);
            iter.catchup_blocks = Some((100, None));
            *iter.provider.write().await =
                Some(ProviderBuilder::new().connect_mocked_client(asserter));
            assert!(iter.queue_canonical_block_for_retry(42).await.is_err());
            assert!(iter.next_blocklogs.is_empty());
            assert!(iter.pending_catchup_page.is_none());
            assert_eq!(iter.catchup_blocks, Some((100, None)));
        }
    }

    #[tokio::test]
    async fn catchup_uses_current_header_and_rejects_mixed_snapshots() {
        use alloy::transports::mock::Asserter;
        let asserter = Asserter::new();
        let old = rpc_block(1);
        let canonical = rpc_block(2);
        asserter.push_success(&canonical);
        asserter.push_success(&canonical);
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let mut iter = new_test_iter(50);
        iter.block_history.add_block(old.clone().into());
        *iter.provider.write().await = Some(provider);
        let log = Log {
            block_hash: Some(canonical.header.hash),
            block_number: Some(42),
            transaction_hash: Some(BlockHash::with_last_byte(3)),
            log_index: Some(0),
            ..Default::default()
        };

        let blocks = iter.split_by_block(vec![log.clone()]).await.unwrap();
        assert_eq!(blocks[0].summary.hash, canonical.header.hash);
        let orphan_log = Log {
            block_hash: Some(old.header.hash),
            ..log
        };
        assert!(iter.split_by_block(vec![orphan_log]).await.is_err());
    }

    #[test]
    fn catchup_progress_waits_for_whole_page_and_rewinds_on_failure() {
        let mut iter = new_test_iter(50);
        iter.catchup_blocks = Some((40, Some(100)));
        iter.absolute_end_at_block = Some(100);
        iter.last_valid_block = Some(200); // realtime is ahead of catchup
        iter.pending_catchup_page = Some(CatchupPage {
            from_block: 40,
            next_page: 60,
            remaining_blocks: 2,
        });
        iter.acknowledge_catchup_block();
        assert_eq!(iter.catchup_blocks.unwrap().0, 40);

        iter.retry_from_block(50);
        assert_eq!(iter.catchup_blocks.unwrap().0, 40);
        assert_eq!(iter.last_valid_block, Some(39));
        assert!(iter.pending_catchup_page.is_none());

        iter.pending_catchup_page = Some(CatchupPage {
            from_block: 40,
            next_page: 60,
            remaining_blocks: 2,
        });
        iter.acknowledge_catchup_block();
        assert_eq!(iter.catchup_blocks.unwrap().0, 40);
        iter.acknowledge_catchup_block();
        assert_eq!(iter.catchup_blocks.unwrap().0, 60);
    }

    #[tokio::test]
    async fn catchup_end_waits_for_remaining_historical_pages() {
        let mut iter = new_test_iter(50);
        iter.absolute_end_at_block = Some(100);
        let mut tip = rpc_block(1);
        tip.header.inner.number = 200;
        iter.block_history.add_block(tip.into());
        // An empty page or failed fetch must not let the newer realtime head
        // terminate a bounded replay before the rest of its range is read.
        iter.catchup_blocks = Some((60, Some(100)));
        assert!(!iter.end_at_block_reached().await);
        iter.catchup_blocks = Some((101, Some(100)));
        assert!(iter.end_at_block_reached().await);
    }

    #[tokio::test]
    async fn catchup_includes_the_finalized_boundary() {
        use alloy::transports::mock::Asserter;

        let asserter = Asserter::new();
        let mut block = rpc_block(1);
        block.header.inner.number = 40;
        let log: Log = Log {
            block_hash: Some(block.header.hash),
            block_number: Some(40),
            transaction_hash: Some(BlockHash::with_last_byte(3)),
            log_index: Some(0),
            ..Default::default()
        };
        asserter.push_success(&vec![log]);
        asserter.push_success(&block);
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let mut iter = new_test_iter(50);
        iter.catchup_provider = Some(provider.clone());
        *iter.provider.write().await = Some(provider);
        iter.catchup_blocks = Some((40, Some(40)));
        iter.absolute_end_at_block = Some(40);
        block.header.inner.number = 60; // finality lag is 20
        iter.block_history.add_block(block.into());

        iter.consume_catchup_blocks().await;
        let block = iter
            .next_blocklogs
            .pop_front()
            .expect("the block exactly at head minus finality lag is eligible");
        assert_eq!(block.summary.number, 40);
        assert!(block.finalized);
        iter.acknowledge_catchup_block();
        assert!(iter.end_at_block_reached().await);
    }

    #[tokio::test]
    async fn catchup_empty_page_continues_before_bounded_replay_ends() {
        use alloy::pubsub::RawSubscription;
        use alloy::transports::mock::Asserter;

        let asserter = Asserter::new();
        asserter.push_success(&Vec::<Log>::new()); // 40..=59 is empty
        let mut block = rpc_block(2);
        block.header.inner.number = 60;
        let log: Log = Log {
            block_hash: Some(block.header.hash),
            block_number: Some(60),
            transaction_hash: Some(BlockHash::with_last_byte(3)),
            log_index: Some(0),
            ..Default::default()
        };
        asserter.push_success(&vec![log]);
        asserter.push_success(&block);
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let mut iter = new_test_iter(50);
        iter.catchup_provider = Some(provider.clone());
        *iter.provider.write().await = Some(provider);
        let (_sender, rx) = tokio::sync::broadcast::channel(1);
        iter.stream = Some(
            RawSubscription {
                rx,
                local_id: BlockHash::ZERO,
            }
            .into_typed::<Header>()
            .into_stream(),
        );
        iter.catchup_paging = 20;
        iter.catchup_blocks = Some((40, Some(79)));
        iter.absolute_end_at_block = Some(79);
        let mut tip = rpc_block(1);
        tip.header.inner.number = 200;
        iter.block_history.add_block(tip.into());

        // No realtime message is needed to move past the empty first page.
        let next = tokio::time::timeout(Duration::from_secs(1), iter.next())
            .await
            .expect("catchup must continue without reading realtime")
            .expect("the second page has a computation");
        assert_eq!(next.summary.number, 60);
        assert_eq!(iter.catchup_blocks.unwrap().0, 60);
        iter.acknowledge_catchup_block();
        assert!(iter.next().await.is_none());
    }

    #[tokio::test]
    async fn catchup_pages_refresh_liveness_only_after_success() {
        use alloy::transports::mock::Asserter;
        use fhevm_engine_common::healthz_server::HealthCheckService;

        for (empty, valid) in [(true, true), (false, true), (false, false)] {
            let asserter = Asserter::new();
            let mut block = rpc_block(1);
            block.header.inner.number = 40;
            let logs: Vec<Log> = if empty {
                vec![]
            } else {
                vec![Log {
                    block_hash: Some(if valid {
                        block.header.hash
                    } else {
                        BlockHash::with_last_byte(2)
                    }),
                    block_number: Some(40),
                    transaction_hash: Some(BlockHash::with_last_byte(3)),
                    log_index: Some(0),
                    ..Default::default()
                }]
            };
            asserter.push_success(&logs);
            if !empty {
                asserter.push_success(&block);
            }
            let provider =
                ProviderBuilder::new().connect_mocked_client(asserter);
            let mut iter = new_test_iter(50);
            iter.catchup_provider = Some(provider.clone());
            *iter.provider.write().await = Some(provider);
            iter.catchup_blocks = Some((40, Some(40)));
            iter.absolute_end_at_block = Some(40);
            block.header.inner.number = 60;
            iter.block_history.add_block(block.into());
            iter.tick_block = HeartBeat::with_elapsed_secs(30);
            iter.tick_timeout = HeartBeat::with_elapsed_secs(30);
            let health = HealthCheck {
                blockchain_tick: iter.tick_block.clone(),
                blockchain_timeout_tick: iter.tick_timeout.clone(),
                blockchain_provider: iter.provider.clone(),
                database_pool: Arc::new(RwLock::new(
                    PgPoolOptions::new()
                        .connect_lazy("postgres://test:test@localhost/test")
                        .unwrap(),
                )),
                database_tick: HeartBeat::with_elapsed_secs(30),
                recovery_paused: Default::default(),
            };
            assert!(!health.is_alive().await);

            iter.consume_catchup_blocks().await;

            assert_eq!(health.is_alive().await, valid, "empty={empty}");
            assert!(!iter.tick_timeout.is_recent(&Duration::from_secs(20)));
        }
    }

    #[tokio::test]
    async fn catchup_loop_reset_preserves_unread_blocks_and_lookback() {
        let anvil = Anvil::new().spawn();
        let provider = ProviderBuilder::new()
            .connect_ws(WsConnect::new(anvil.ws_endpoint()))
            .await
            .unwrap();
        provider.anvil_mine(Some(100), None).await.unwrap();

        // Pending pages survive reset; completed (including empty) ranges
        // retain their next height if the moving window has skipped ahead.
        // An overlapping lookback can still replay earlier committed blocks.
        for (pending_from, previous_end, lookback, expected_from) in [
            (Some(40), Some(95), -10, 40),
            (None, Some(50), -10, 51),
            (Some(96), Some(95), -40, 60),
            (None, None, -10, 90),
        ] {
            let mut iter = new_test_iter(50);
            iter.url = anvil.ws_endpoint();
            iter.start_at_block = Some(lookback);
            iter.end_at_block = Some(-5);
            iter.absolute_end_at_block = previous_end;
            iter.catchup_blocks = pending_from.map(|from| (from, previous_end));
            iter.pending_catchup_page = pending_from.map(|from| CatchupPage {
                from_block: from,
                next_page: from + 20,
                remaining_blocks: 1,
            });
            iter.last_valid_block = Some(100);

            iter.reset_for_catchup_loop();
            iter.new_log_stream_no_retry().await.unwrap();

            assert_eq!(iter.catchup_blocks, Some((expected_from, Some(95))));
            assert!(iter.pending_catchup_page.is_none());
            assert!(iter.last_valid_block.is_none());
        }
    }

    async fn setup_iter_with_chain(
        num_blocks: u64,
        reorg_max: u64,
        known: std::ops::RangeInclusive<usize>,
    ) -> (
        alloy::node_bindings::AnvilInstance,
        InfiniteLogIter,
        Vec<BlockSummary>,
    ) {
        let anvil = Anvil::new().spawn();
        let ws = WsConnect::new(anvil.ws_endpoint());
        let provider = ProviderBuilder::new().connect_ws(ws).await.unwrap();
        provider.anvil_mine(Some(num_blocks), None).await.unwrap();
        let mut blocks = Vec::with_capacity((num_blocks + 1) as usize);
        for i in 0..=num_blocks {
            let b = provider
                .get_block_by_number(i.into())
                .await
                .unwrap()
                .unwrap();
            blocks.push(BlockSummary::from(b));
        }
        let mut iter = new_test_iter(reorg_max);
        for b in &blocks[known] {
            iter.block_history.add_block(*b);
        }
        *iter.provider.write().await = Some(provider);
        (anvil, iter, blocks)
    }

    // Walks back 2 blocks before finding a known ancestor in history.
    // Tests the common case where only a few blocks were missed.
    #[tokio::test]
    async fn test_get_missing_ancestors_shallow_reorg() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(5, 50, 0..=2).await;

        let missing = iter.get_missing_ancestors(blocks[5]).await;

        assert_eq!(missing.len(), 2);
        assert_eq!(missing[0].number, 3);
        assert_eq!(missing[1].number, 4);
        assert_eq!(missing[0].parent_hash, blocks[2].hash);
    }

    // Walks back 13 blocks through a long gap before hitting a known ancestor.
    // Tests that the walk handles a long gap correctly.
    #[tokio::test]
    async fn test_get_missing_ancestors_deep_reorg() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(15, 50, 0..=1).await;

        let missing = iter.get_missing_ancestors(blocks[15]).await;

        assert_eq!(missing.len(), 13);
        assert_eq!(missing[0].number, 2);
        assert_eq!(missing.last().unwrap().number, 14);
    }

    // Stops walking at reorg_maximum_duration_in_blocks even if more unknown ancestors remain.
    // Tests that the function doesn't walk forever and respects the configured max depth.
    #[tokio::test]
    async fn test_get_missing_ancestors_beyond_max_depth() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(10, 3, 0..=0).await;

        let missing = iter.get_missing_ancestors(blocks[10]).await;

        assert_eq!(missing.len(), 3);
        assert_eq!(missing[0].number, 7);
        assert_eq!(missing[1].number, 8);
        assert_eq!(missing[2].number, 9);
    }

    // Skips reorg detection when history has fewer than 2 blocks and just adds the block.
    // Tests that the guard condition prevents false reorg detection.
    #[tokio::test]
    async fn test_check_missing_ancestors_not_ready() {
        let mut iter = new_test_iter(50);
        assert!(!iter.block_history.is_ready_to_detect_reorg());

        let block_a = BlockSummary {
            number: 100,
            hash: BlockHash::with_last_byte(0xAA),
            parent_hash: BlockHash::with_last_byte(0x99),
            timestamp: 1000,
        };
        iter.check_missing_ancestors(block_a).await;
        assert!(iter.block_history.is_known(&block_a.hash));
        assert!(!iter.block_history.is_ready_to_detect_reorg());

        let block_b = BlockSummary {
            number: 101,
            hash: BlockHash::with_last_byte(0xBB),
            parent_hash: BlockHash::with_last_byte(0xAA),
            timestamp: 1012,
        };
        iter.check_missing_ancestors(block_b).await;
        assert!(iter.block_history.is_known(&block_b.hash));
        assert!(iter.block_history.is_ready_to_detect_reorg());
    }

    #[tokio::test]
    async fn get_block_by_number_fails_when_header_unavailable() {
        let iter = new_test_iter(50);
        assert!(iter.get_block_by_number(42).await.is_err());
    }

    #[tokio::test]
    async fn cached_header_does_not_mask_unavailable_canonical_header() {
        let mut iter = new_test_iter(50);
        let summary = BlockSummary {
            number: 7,
            hash: BlockHash::with_last_byte(0xAA),
            parent_hash: BlockHash::with_last_byte(0x99),
            timestamp: 1000,
        };
        iter.block_history.add_block(summary);

        let log = Log {
            block_number: Some(summary.number),
            block_hash: Some(summary.hash),
            transaction_hash: Some(BlockHash::with_last_byte(1)),
            log_index: Some(0),
            ..Default::default()
        };
        assert!(iter.split_by_block(vec![log]).await.is_err());
    }
    #[tokio::test]
    async fn first_start_covers_finality_and_overlap() -> Result<()> {
        use alloy::transports::mock::Asserter;
        let asserter = Asserter::new();
        asserter.push_success(&alloy::primitives::U64::from(113));
        let provider = ProviderBuilder::new().connect_mocked_client(asserter);
        let mut iter = new_test_iter(50);
        assert_eq!(iter.catchup_block_from(&provider).await?, 88);
        // An explicit replay start is not constrained by saved listener state.
        iter.start_at_block = Some(100);
        assert_eq!(iter.catchup_block_from(&provider).await?, 100);
        Ok(())
    }

    #[test]
    fn failure_preserves_queued_ancestors_even_when_their_hashes_are_known() {
        let mut iter = new_test_iter(50);
        iter.catchup_blocks = Some((100, None));
        for number in [95, 96] {
            let mut ancestor = rpc_block(number as u8);
            ancestor.header.inner.number = number;
            let summary = ancestor.into();
            iter.block_history.add_block(summary);
            iter.next_blocklogs.push_back(BlockLogs {
                summary,
                logs: vec![],
                catchup: true,
                finalized: false,
            });
        }
        iter.last_valid_block = Some(200);
        iter.retry_from_block(99);
        assert!(iter.next_blocklogs.is_empty());
        assert_eq!(iter.block_history.size(), 0);
        assert_eq!(iter.catchup_blocks.unwrap().0, 95);
        assert_eq!(iter.last_valid_block, Some(94));
    }

    #[tokio::test]
    async fn missing_ancestor_fetch_failure_retains_the_gap() {
        let mut iter = new_test_iter(50);
        iter.catchup_blocks = Some((100, None));
        let current = BlockSummary {
            number: 96,
            hash: BlockHash::with_last_byte(96),
            parent_hash: BlockHash::with_last_byte(95),
            timestamp: 96,
        };
        assert!(iter.get_missing_ancestors(current).await.is_empty());
        assert_eq!(iter.catchup_blocks.unwrap().0, 95);
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn bootstrap_database_failure_is_not_treated_as_empty_history(
    ) -> Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow!("{err}"))?;
        let db =
            Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
                .await?;
        db.pool().await.close().await;
        assert!(db.read_last_valid_block_checked().await.is_err());
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    #[tracing_test::traced_test]
    async fn contained_listener_startup_stays_alive() -> Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow!("{err}"))?;
        let db =
            Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
                .await?;
        let pool = db.pool().await;
        fhevm_engine_common::chain_recovery::record_halt(
            &pool,
            fhevm_engine_common::chain_recovery::HistoryMismatch {
                source:
                    fhevm_engine_common::chain_recovery::ListenerSource::Host,
                gcs_mode: false,
                chain_id: 42,
                block_number: 65,
                expected_hash: Some(&[1_u8; 32]),
                observed_hash: Some(&[2_u8; 32]),
                reason: "test committed history conflict",
            },
        )
        .await?;
        let anvil = Anvil::new().spawn();
        let args = Args::parse_from([
            "host-listener",
            "--url",
            &anvil.ws_endpoint(),
            "--database-url",
            instance.db_url(),
            "--acl-contract-address",
            "0x0000000000000000000000000000000000000001",
            "--tfhe-contract-address",
            "0x0000000000000000000000000000000000000002",
            "--health-port",
            "0",
            "--start-at-block",
            "0",
        ]);
        let listener = main(args);
        tokio::pin!(listener);
        tokio::select! {
            result = &mut listener => panic!("contained listener exited: {result:?}"),
            result = tokio::time::timeout(Duration::from_secs(10), async {
                while !logs_contain("Host listener paused by chain history containment") {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }) => result?,
        }
        assert!(tokio::time::timeout(
            Duration::from_millis(100),
            &mut listener
        )
        .await
        .is_err());
        assert!(!logs_contain("Error creating new log stream, retrying"));

        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    #[tracing_test::traced_test]
    async fn retired_listener_startup_stays_alive() -> Result<()> {
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|err| anyhow!("{err}"))?;
        let db =
            Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
                .await?;
        let pool = db.pool().await;
        sqlx::query("UPDATE versioning SET consensus_version = 9999")
            .execute(&pool)
            .await?;
        let anvil = Anvil::new().spawn();
        let args = Args::parse_from([
            "host-listener",
            "--url",
            &anvil.ws_endpoint(),
            "--database-url",
            instance.db_url(),
            "--acl-contract-address",
            "0x0000000000000000000000000000000000000001",
            "--tfhe-contract-address",
            "0x0000000000000000000000000000000000000002",
            "--health-port",
            "0",
            "--start-at-block",
            "0",
        ]);
        let listener = main(args);
        tokio::pin!(listener);
        tokio::select! {
            result = &mut listener => panic!("retired listener exited: {result:?}"),
            result = tokio::time::timeout(Duration::from_secs(10), async {
                while !logs_contain("Listener stack retired; pausing ingestion") {
                    tokio::time::sleep(Duration::from_millis(10)).await;
                }
            }) => result?,
        }
        assert!(tokio::time::timeout(
            Duration::from_millis(100),
            &mut listener
        )
        .await
        .is_err());
        assert!(!logs_contain("Error creating new log stream, retrying"));

        // The same pause used during ingestion keeps liveness healthy and can
        // be cancelled, without dropping or acknowledging queued work.
        let mode = StackMode::new(false);
        reconcile_stack_mode(&pool, &mode).await?;
        assert!(mode.is_paused());
        let heartbeat = HeartBeat::with_elapsed_secs(30);
        let cancel = CancellationToken::new();
        let paused = wait_while_paused(&mode, &heartbeat, &cancel);
        tokio::pin!(paused);
        assert!(tokio::time::timeout(Duration::from_millis(20), &mut paused)
            .await
            .is_err());
        assert!(heartbeat.is_recent(&Duration::from_secs(1)));
        cancel.cancel();
        tokio::time::timeout(Duration::from_secs(1), paused).await?;
        Ok(())
    }

    #[tokio::test]
    async fn startup_catchup_does_not_finalize_recent_history() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(30, 50, 0..=0).await;
        iter.block_history = BlockHistory::new(50);
        // The configured lag is 20: height 10 is safe, height 11 is pending.
        let logs = [10, 11, 30].map(|n| Log {
            block_number: Some(n as u64),
            block_hash: Some(blocks[n].hash),
            transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
            log_index: Some(0),
            ..Log::default()
        });
        let grouped = iter.split_by_block(logs.to_vec()).await.unwrap();
        assert_eq!(
            grouped.iter().map(|b| b.finalized).collect::<Vec<_>>(),
            vec![true, false, false]
        );
    }

    #[tokio::test]
    async fn get_block_summary_fails_when_header_unavailable() {
        let iter = new_test_iter(50);
        assert!(iter.get_block_by_number(42).await.is_err());
    }

    #[tokio::test]
    async fn catchup_uses_canonical_header_after_same_height_fork() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(1, 50, 0..=1).await;
        let provider = iter.provider.read().await.clone().unwrap();
        let snapshot = provider.anvil_snapshot().await.unwrap();
        provider.anvil_mine(Some(1), None).await.unwrap();
        let old: BlockSummary = provider
            .get_block_by_number(2.into())
            .await
            .unwrap()
            .unwrap()
            .into();
        iter.block_history.add_block(old);

        assert!(provider.anvil_revert(snapshot).await.unwrap());
        provider
            .anvil_set_next_block_timestamp(old.timestamp + 60)
            .await
            .unwrap();
        provider.anvil_mine(Some(1), None).await.unwrap();
        let replacement: BlockSummary = provider
            .get_block_by_number(2.into())
            .await
            .unwrap()
            .unwrap()
            .into();
        assert_ne!(old.hash, replacement.hash);
        assert_eq!(replacement.parent_hash, blocks[1].hash);

        let replacement_log = Log {
            block_number: Some(2),
            block_hash: Some(replacement.hash),
            transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
            log_index: Some(0),
            ..Log::default()
        };
        let grouped = iter.split_by_block(vec![replacement_log]).await.unwrap();
        assert_eq!(grouped.len(), 1);
        assert_eq!(grouped[0].summary.hash, replacement.hash);
        assert_eq!(grouped[0].summary.parent_hash, blocks[1].hash);
        assert_eq!(grouped[0].summary.timestamp, replacement.timestamp);

        let stale_log = Log {
            block_number: Some(2),
            block_hash: Some(old.hash),
            transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
            log_index: Some(0),
            ..Log::default()
        };
        assert!(iter.split_by_block(vec![stale_log]).await.is_err());
    }

    #[tokio::test]
    async fn catchup_rejects_incomplete_and_mixed_block_metadata() {
        let (_anvil, mut iter, blocks) =
            setup_iter_with_chain(2, 50, 0..=1).await;
        let valid = Log {
            block_number: Some(2),
            block_hash: Some(blocks[2].hash),
            transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
            log_index: Some(0),
            ..Log::default()
        };
        assert!(iter
            .split_by_block(vec![Log {
                removed: true,
                ..valid.clone()
            }])
            .await
            .is_err());
        assert!(iter
            .split_by_block(vec![valid.clone(), Log::default()])
            .await
            .is_err());
        assert!(iter
            .split_by_block(vec![
                valid.clone(),
                Log {
                    block_number: Some(2),
                    block_hash: Some(blocks[1].hash),
                    transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
                    log_index: Some(0),
                    ..Log::default()
                }
            ])
            .await
            .is_err());
        let earlier = Log {
            block_number: Some(1),
            block_hash: Some(blocks[1].hash),
            transaction_hash: Some(BlockHash::with_last_byte(0xfe)),
            log_index: Some(0),
            ..Log::default()
        };
        let reordered = iter
            .split_by_block(vec![valid.clone(), earlier])
            .await
            .unwrap();
        assert_eq!(
            reordered
                .iter()
                .map(|block| block.summary.number)
                .collect::<Vec<_>>(),
            vec![1, 2]
        );
        let grouped = iter.split_by_block(vec![valid]).await.unwrap();
        assert_eq!(grouped[0].summary.hash, blocks[2].hash);
    }

    #[tokio::test]
    async fn catchup_start_near_genesis_saturates() {
        let (_anvil, mut iter, _) = setup_iter_with_chain(1, 50, 0..=1).await;
        let provider = iter.provider.read().await.clone().unwrap();
        iter.catchup_margin = 5;
        iter.last_valid_block = Some(1);
        assert_eq!(iter.catchup_block_from(&provider).await.unwrap(), 0);
        iter.last_valid_block = Some(5);
        assert_eq!(iter.catchup_block_from(&provider).await.unwrap(), 1);
        iter.catchup_margin = 0;
        assert_eq!(iter.catchup_block_from(&provider).await.unwrap(), 6);
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn history_verification_refreshes_liveness_during_successful_rpc_work(
    ) -> Result<()> {
        use alloy::transports::mock::Asserter;
        use fhevm_engine_common::chain_recovery::{
            lock_history_tx, ListenerSource,
        };
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        let db =
            Database::new(&instance.db_url, ChainId::try_from(42_u64)?, 128)
                .await?;
        let pool = db.pool().await;
        let block = rpc_block(1);
        sqlx::query("INSERT INTO host_chain_blocks_valid (chain_id, block_number, block_hash, block_status) VALUES (42, $1, $2, 'finalized')")
            .bind(block.header.inner.number as i64)
            .bind(block.header.hash.to_vec())
            .execute(&pool).await?;
        let mut iter = new_test_iter(50);
        iter.tick_block = HeartBeat::with_elapsed_secs(30);
        assert!(iter.verify_committed_history(&db).await.is_err());
        assert!(
            !iter.tick_block.is_recent(&Duration::from_secs(20)),
            "failed RPC is not progress"
        );

        let asserter = Asserter::new();
        asserter.push_success(&block);
        *iter.provider.write().await =
            Some(ProviderBuilder::new().connect_mocked_client(asserter));
        // Keep verification pending after the successful RPC, so a heartbeat
        // only at the end of the full scan cannot satisfy this assertion.
        let mut gate = pool.begin().await?;
        lock_history_tx(&mut gate, ListenerSource::Host, false, 42).await?;
        let verification = iter.verify_committed_history(&db);
        tokio::pin!(verification);
        tokio::select! {
            result = &mut verification => panic!("verification bypassed admission lock: {result:?}"),
            result = tokio::time::timeout(Duration::from_secs(10), async {
                while !iter.tick_block.is_recent(&Duration::from_secs(20)) {
                    tokio::task::yield_now().await;
                }
            }) => result?,
        }
        gate.rollback().await?;
        verification.await?;
        Ok(())
    }

    #[tokio::test]
    #[serial_test::serial(db)]
    async fn sparse_catchup_checks_old_history_before_admitting_a_new_tip(
    ) -> Result<()> {
        use crate::contracts::{AclContract, TfheContract};
        use crate::database::ingest::{ingest_block_logs, IngestOptions};
        use alloy::{
            primitives::U256, sol_types::SolEvent, transports::mock::Asserter,
        };
        use fhevm_engine_common::chain_recovery::is_halted;
        use test_harness::instance::{setup_test_db, ImportMode};
        let instance = setup_test_db(ImportMode::None)
            .await
            .map_err(|e| anyhow!("{e}"))?;
        let chain = ChainId::try_from(42_u64)?;
        let mut db = Database::new(&instance.db_url, chain, 128).await?;
        let pool = db.pool().await;
        let make = |n: u64, canonical: bool| {
            let h = |n| {
                BlockHash::with_last_byte(
                    (n + if canonical && n >= 75 { 100 } else { 0 }) as u8,
                )
            };
            let mut b = rpc_block(0);
            b.header.hash = h(n);
            b.header.inner.number = n;
            b.header.inner.parent_hash = h(n - 1);
            b.header.inner.timestamp = n;
            b
        };
        let options = IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: true,
            dependent_ops_max_per_chain: 0,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        };
        for n in 74..=80 {
            let block = BlockLogs {
                logs: vec![],
                summary: make(n, false).into(),
                catchup: true,
                finalized: true,
            };
            ingest_block_logs(
                chain,
                &mut db,
                &block,
                &None,
                &None,
                &None,
                &None,
                &None,
                options.clone(),
            )
            .await?
            .require_complete()?;
        }
        let canonical = make(82, true);
        let mut handle = BlockHash::with_last_byte(0);
        handle.0[0] = 0x51;
        handle.0[21] = 0xff;
        handle.0[22..30].copy_from_slice(&42_u64.to_be_bytes());
        handle.0[30] = 5;
        let events = [
            TfheContract::TrivialEncrypt {
                caller: Address::ZERO,
                pt: U256::from(7),
                toType: 5,
                result: handle,
            }
            .encode_log_data(),
            AclContract::Allowed {
                caller: Address::ZERO,
                account: Address::ZERO,
                handle,
            }
            .encode_log_data(),
        ];
        let logs = events
            .into_iter()
            .enumerate()
            .map(|(i, data)| Log {
                inner: alloy::primitives::Log {
                    address: Address::ZERO,
                    data,
                },
                block_number: Some(82),
                block_hash: Some(canonical.header.hash),
                transaction_hash: Some(BlockHash::with_last_byte(0xe1)),
                log_index: Some(i as u64),
                ..Default::default()
            })
            .collect();
        let asserter = Asserter::new();
        asserter.push_success(&canonical);
        asserter.push_success(&make(80, true));
        let mut iter = new_test_iter(50);
        iter.block_history.add_block(make(110, true).into());
        *iter.provider.write().await =
            Some(ProviderBuilder::new().connect_mocked_client(asserter));
        let page = iter.split_by_block(logs).await?;
        assert_eq!(page.len(), 1);
        assert!(page[0].finalized);
        let err = iter.verify_committed_history(&db).await.unwrap_err();
        assert!(err.to_string().contains("committed canonical history"));
        assert!(is_halted(&pool).await?);
        let new_tip: bool = sqlx::query_scalar(
            "SELECT EXISTS (SELECT 1 FROM host_chain_blocks_valid WHERE block_number = 82)",
        ).fetch_one(&pool).await?;
        assert!(
            !new_tip,
            "history verification must precede catchup admission"
        );
        Ok(())
    }
}
