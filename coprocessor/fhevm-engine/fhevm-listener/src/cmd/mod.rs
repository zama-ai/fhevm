use alloy_provider::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
    NonceFiller,
};
use futures_util::stream::StreamExt;
use sqlx::types::Uuid;
use std::collections::VecDeque;
use std::str::FromStr;
use std::time::Duration;
use tracing::{error, info, warn, Level};

use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, RootProvider, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log};

use alloy_sol_types::SolEventInterface;

use clap::Parser;

use rustls;

use crate::contracts::{AclContract, TfheContract};
use crate::database::tfhe_event_propagate::{ChainId, Database};

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value = "ws://0.0.0.0:8746")]
    pub url: String,

    #[arg(long, default_value = "false")]
    pub ignore_tfhe_events: bool,

    #[arg(long, default_value = "false")]
    pub ignore_acl_events: bool,

    #[arg(long, default_value = None)]
    pub acl_contract_address: Option<String>,

    #[arg(long, default_value = None)]
    pub tfhe_contract_address: Option<String>,

    #[arg(
        long,
        default_value = "postgresql://postgres:testmdp@localhost:5432/postgres"
    )]
    pub database_url: String,

    #[arg(long, default_value = None, help = "Can be negative from last block", allow_hyphen_values = true)]
    pub start_at_block: Option<i64>,

    #[arg(long, default_value = None)]
    pub end_at_block: Option<u64>,

    #[arg(long, default_value = None, help = "A Coprocessor API key is needed for database access")]
    pub coprocessor_api_key: Option<Uuid>,

    #[arg(
        long,
        default_value = "5",
        help = "Catchup margin relative the last seen block"
    )]
    pub catchup_margin: u64,

    #[arg(
        long,
        default_value = "false",
        help = "Disable block immediate recheck"
    )]
    pub no_block_immediate_recheck: bool,

    #[arg(
        long,
        default_value = "5",
        help = "Initial block time, refined on each block"
    )]
    pub initial_block_time: u64,

    #[arg(
        long,
        value_parser = clap::value_parser!(Level),
        default_value_t = Level::INFO)]
    pub log_level: Level,
}

type RProvider = FillProvider<
    JoinFill<
        alloy::providers::Identity,
        JoinFill<
            GasFiller,
            JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>,
        >,
    >,
    RootProvider,
>;

// TODO: to merge with Levent works
struct InfiniteLogIter {
    url: String,
    block_time: u64, /* A default value that is refined with real-time
                      * events data */
    no_block_immediate_recheck: bool,
    contract_addresses: Vec<Address>,
    catchup_logs: VecDeque<Log>,
    stream: Option<SubscriptionStream<Log>>,
    provider: Option<RProvider>, // required to maintain the stream
    last_valid_block: Option<u64>,
    start_at_block: Option<i64>,
    end_at_block: Option<u64>,
    catchup_margin: u64,
    prev_event: Option<Log>,
    current_event: Option<Log>,
    last_block_event_count: u64,
    last_block_recheck_planned: u64,
}
enum LogOrBlockTimeout {
    Log(Option<Log>),
    BlockTimeout,
}

impl InfiniteLogIter {
    fn new(args: &Args) -> Self {
        let mut contract_addresses = vec![];
        if let Some(acl_contract_address) = &args.acl_contract_address {
            contract_addresses
                .push(Address::from_str(acl_contract_address).unwrap());
        };
        if let Some(tfhe_contract_address) = &args.tfhe_contract_address {
            contract_addresses
                .push(Address::from_str(tfhe_contract_address).unwrap());
        };
        Self {
            url: args.url.clone(),
            block_time: args.catchup_margin,
            no_block_immediate_recheck: args.no_block_immediate_recheck,
            contract_addresses,
            catchup_logs: VecDeque::new(),
            stream: None,
            provider: None,
            last_valid_block: None,
            start_at_block: args.start_at_block,
            end_at_block: args.end_at_block,
            catchup_margin: args.catchup_margin,
            prev_event: None,
            current_event: None,
            last_block_event_count: 0,
            last_block_recheck_planned: 0,
        }
    }

    async fn get_chain_id_or_panic(&self) -> ChainId {
        // TODO: remove expect and, instead, propagate the error
        let ws = WsConnect::new(&self.url);
        let provider = ProviderBuilder::new()
            .connect_ws(ws)
            .await
            .expect("Cannot connect to host chain");
        provider
            .get_chain_id()
            .await
            .expect("Cannot retrieve chain id")
    }

    async fn catchup_block_from(
        &self,
        provider: &RProvider,
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

    async fn fill_catchup_events(
        &mut self,
        provider: &RProvider,
        filter: &Filter,
    ) {
        let logs = provider.get_logs(&filter).await.expect("BLA2");
        self.catchup_logs.extend(logs);
    }

    async fn recheck_prev_block(&mut self) -> bool {
        let Some(provider) = &self.provider else {
            error!("No provider, inconsistent state");
            return false;
        };
        let Some(event) = &self.prev_event else {
            return false;
        };
        let Some(block) = event.block_number else {
            return false;
        };
        let last_block_event_count = self.last_block_event_count;
        self.last_block_event_count = 0;
        if self.last_block_recheck_planned == block {
            // no need to replan anything
            return false;
        }
        let mut filter = Filter::new().from_block(block).to_block(block); // inclusive
        if !self.contract_addresses.is_empty() {
            filter = filter.address(self.contract_addresses.clone())
        }
        let Ok(logs) = provider.get_logs(&filter).await else {
            return false;
        };
        if logs.is_empty() {
            return false;
        }
        if logs.len() as u64 == last_block_event_count {
            return false;
        }
        info!(
            block = block,
            events_count = logs.len(),
            last_block_event_count = last_block_event_count,
            "Replaying Block"
        );
        self.catchup_logs.extend(logs);
        if let Some(event) = self.current_event.take() {
            self.catchup_logs.push_back(event);
        }
        self.last_block_recheck_planned = block;
        true
    }

    async fn new_log_stream(&mut self, not_initialized: bool) {
        let mut retry = 20;
        loop {
            let ws = WsConnect::new(&self.url).with_max_retries(0); // disabled, alloy skips events

            match ProviderBuilder::new().connect_ws(ws).await {
                Ok(provider) => {
                    let catch_up_from =
                        self.catchup_block_from(&provider).await;
                    let mut filter = Filter::new().from_block(catch_up_from);
                    if let Some(end_at_block) = self.end_at_block {
                        filter = filter
                            .to_block(BlockNumberOrTag::Number(end_at_block));
                        // inclusive
                    }
                    if !self.contract_addresses.is_empty() {
                        filter = filter.address(self.contract_addresses.clone())
                    }
                    info!(url = %self.url, "Listening on");
                    info!(contracts = ?self.contract_addresses, "Contracts addresses");
                    // note subcribing to real-time before reading catchup
                    // events to have the minimal gap between the two
                    // TODO: but it does not guarantee no gap for now
                    // (implementation dependant)
                    self.stream = Some(
                        provider
                            .subscribe_logs(&filter)
                            .await
                            .expect("BLA2")
                            .into_stream(),
                    );
                    self.fill_catchup_events(&provider, &filter).await;
                    self.provider = Some(provider);
                    return;
                }
                Err(err) => {
                    let delay = if not_initialized {
                        if retry == 0 {
                            // TODO: remove panic and, instead, propagate the error
                            error!(
                                url = %self.url,
                                error = %err,
                                "Cannot connect",
                            );
                            panic!(
                                "Cannot connect to {} due to {err}.",
                                &self.url
                            )
                        }
                        5
                    } else {
                        1
                    };
                    if not_initialized {
                        warn!(
                            url = %self.url,
                            error = %err,
                            delay_secs = delay,
                            retry = retry,
                            "Cannot connect. Will retry",
                        );
                    } else {
                        warn!(
                            url = %self.url,
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

    async fn next(&mut self) -> Option<Log> {
        let mut not_initialized = self.stream.is_none();
        self.prev_event = self.current_event.take();
        while self.current_event.is_none() {
            if self.stream.is_none() {
                self.new_log_stream(not_initialized).await;
                not_initialized = false;
                continue;
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
                    if let (Some(end_at_block), Some(last_seen_block)) =
                        (self.end_at_block, self.last_valid_block)
                    {
                        if end_at_block == last_seen_block {
                            return None;
                        }
                    }
                    info!("Nothing to read, retrying");
                    tokio::time::sleep(Duration::from_secs(1)).await;
                    continue;
                }
                LogOrBlockTimeout::Log(Some(log)) => {
                    self.current_event = Some(log);
                    let recheck_planned = if !self.no_block_immediate_recheck
                        && self.is_first_of_block()
                    {
                        self.recheck_prev_block().await
                    } else {
                        false
                    };
                    if recheck_planned {
                        // current log is delayed and pushed to be replayed
                        // after the previous block in catchup
                        continue; // jump to the first event of catchup phase
                    } else {
                        break;
                    }
                }
                LogOrBlockTimeout::BlockTimeout => {
                    self.recheck_prev_block().await;
                    continue;
                }
            }
        }
        let Some(current_event) = &self.current_event else {
            return None;
        };
        self.last_block_event_count += 1;
        return self.current_event.clone();
    }

    fn is_first_of_block(&self) -> bool {
        match (&self.current_event, &self.prev_event) {
            (Some(current_event), Some(prev_event)) => {
                current_event.block_number != prev_event.block_number
            }
            _ => false,
        }
    }

    fn reestimated_block_time(&mut self) {
        match (&self.current_event, &self.prev_event) {
            (
                Some(Log {
                    block_timestamp: Some(curr_t),
                    block_number: Some(curr_n),
                    ..
                }),
                Some(Log {
                    block_timestamp: Some(prev_t),
                    block_number: Some(prev_n),
                    ..
                }),
            ) => self.block_time = (curr_t - prev_t) / (curr_n - prev_n),
            _ => (),
        }
    }
}

pub async fn main(args: Args) {
    let _ = rustls::crypto::aws_lc_rs::default_provider().install_default();

    if let Some(acl_contract_address) = &args.acl_contract_address {
        if let Err(err) = Address::from_str(acl_contract_address) {
            // TODO: remove panic and, instead, propagate the error
            error!(error = %err, "Invalid ACL contract address");
            panic!("Invalid acl contract address: {err}");
        };
    };
    if let Some(tfhe_contract_address) = &args.tfhe_contract_address {
        if let Err(err) = Address::from_str(tfhe_contract_address) {
            // TODO: remove panic and, instead, propagate the error
            error!(error = %err, "Invalid TFHE contract address");
            panic!("Invalid TFHE contract address: {err}");
        };
    }

    let mut log_iter = InfiniteLogIter::new(&args);
    let chain_id = log_iter.get_chain_id_or_panic().await;
    info!(chain_id = chain_id, "Chain ID");

    let mut db = if !args.database_url.is_empty() {
        if let Some(coprocessor_api_key) = args.coprocessor_api_key {
            let mut db = Database::new(
                &args.database_url,
                &coprocessor_api_key,
                chain_id,
            )
            .await;
            if log_iter.start_at_block.is_none() {
                log_iter.start_at_block = db
                    .read_last_valid_block()
                    .await
                    .map(|n| n - args.catchup_margin as i64);
            }
            Some(db)
        } else {
            // TODO: remove panic and, instead, propagate the error
            error!("A Coprocessor API key is required to access the database");
            panic!("A Coprocessor API key is required to access the database");
        }
    } else {
        None
    };

    log_iter.new_log_stream(true).await;

    let mut block_tfhe_errors = 0;
    while let Some(log) = log_iter.next().await {
        if log_iter.is_first_of_block() {
            log_iter.reestimated_block_time();
            if let Some(block_number) = log.block_number {
                if block_tfhe_errors == 0 {
                    if let Some(ref mut db) = db {
                        let last_valid_block = db
                            .mark_prev_block_as_valid(
                                &log_iter.current_event,
                                &log_iter.prev_event,
                            )
                            .await;
                        if last_valid_block.is_some() {
                            log_iter.last_valid_block = last_valid_block;
                        }
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
        if !args.ignore_tfhe_events {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner)
            {
                // TODO: filter on contract address if known
                info!(tfhe_event = ?event, "TFHE event");
                if let Some(ref mut db) = db {
                    let res = db.insert_tfhe_event(&event).await;
                    if let Err(err) = res {
                        block_tfhe_errors += 1;
                        error!(error = %err, "Error inserting tfhe event");
                    }
                }
                continue;
            }
        }
        if !args.ignore_acl_events {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner)
            {
                info!(acl_event = ?event, "ACL event");
                if let Some(ref mut db) = db {
                    let _ = db.handle_acl_event(&event).await;
                }
                continue;
            }
        }
    }
}
