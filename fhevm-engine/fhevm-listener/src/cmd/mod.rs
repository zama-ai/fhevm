use alloy_provider::fillers::{BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill, NonceFiller};
use futures_util::stream::StreamExt;
use sqlx::types::Uuid;
use std::collections::VecDeque;
use std::str::FromStr;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, RootProvider, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log};

use alloy_sol_types::SolEventInterface;

use clap::Parser;

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

    #[arg(long, default_value = "postgresql://postgres:testmdp@localhost:5432/postgres")]
    pub database_url: String,

    #[arg(long, default_value = None, help = "Can be negative from last block", allow_hyphen_values = true)]
    pub start_at_block: Option<i64>,

    #[arg(long, default_value = None)]
    pub end_at_block: Option<u64>,

    #[arg(long, default_value = None, help = "A Coprocessor API key is needed for database access")]
    pub coprocessor_api_key: Option<Uuid>,

    #[arg(long, default_value = "5", help = "Catchup margin relative the last seen block")]
    pub catchup_margin: u64,
}

type RProvider = FillProvider<
    JoinFill<
        alloy::providers::Identity,
        JoinFill<GasFiller, JoinFill<BlobGasFiller, JoinFill<NonceFiller, ChainIdFiller>>>,
    >,
    RootProvider,
>;

// TODO: to merge with Levent works
struct InfiniteLogIter {
    url: String,
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
}

impl InfiniteLogIter {
    fn new(args: &Args) -> Self {
        let mut contract_addresses = vec![];
        if let Some(acl_contract_address) = &args.acl_contract_address {
            contract_addresses.push(Address::from_str(acl_contract_address).unwrap());
        };
        if let Some(tfhe_contract_address) = &args.tfhe_contract_address {
            contract_addresses.push(Address::from_str(tfhe_contract_address).unwrap());
        };
        Self {
            url: args.url.clone(),
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
        }
    }

    async fn get_chain_id_or_panic(&self) -> ChainId {
        let ws = WsConnect::new(&self.url);
        let provider = ProviderBuilder::new().on_ws(ws).await.unwrap();
        provider.get_chain_id().await.unwrap()
    }

    async fn catchup_block_from(&self, provider: &RProvider) -> BlockNumberOrTag {
        if let Some(last_seen_block) = self.last_valid_block {
            return BlockNumberOrTag::Number(last_seen_block - self.catchup_margin + 1);
        }
        if let Some(start_at_block) = self.start_at_block {
            if start_at_block >= 0 {
                return BlockNumberOrTag::Number(start_at_block.try_into().unwrap());
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

    async fn fill_catchup_events(&mut self, provider: &RProvider, filter: &Filter) {
        let logs = provider.get_logs(&filter).await.expect("BLA2");
        self.catchup_logs.extend(logs);
    }

    async fn new_log_stream(&mut self, not_initialized: bool) {
        let mut retry = 20;
        loop {
            let ws = WsConnect::new(&self.url);
            match ProviderBuilder::new().on_ws(ws).await {
                Ok(provider) => {
                    let catch_up_from = self.catchup_block_from(&provider).await;
                    let mut filter = Filter::new().from_block(catch_up_from);
                    if let Some(end_at_block) = self.end_at_block {
                        filter = filter
                            .to_block(BlockNumberOrTag::Number(end_at_block));
                        // inclusive
                    }
                    if !self.contract_addresses.is_empty() {
                        filter = filter.address(self.contract_addresses.clone())
                    }
                    eprintln!("Listening on {}", &self.url);
                    eprintln!("Contracts {:?}", &self.contract_addresses);
                    // note subcribing to real-time before reading catchup events to have the minimal gap between the two
                    // TODO: but it does not guarantee no gap for now (implementation dependant)
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
                        eprintln!(
                            "Cannot connect to {} due to {err}. Will retry in {delay} secs, {retry} times.",
                            &self.url
                        );
                    } else {
                        eprintln!(
                            "Cannot connect to {} due to {err}. Will retry in {delay} secs, indefinitively.",
                            &self.url
                        );
                    }
                    retry -= 1;
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }

    async fn next(&mut self) -> Option<Log> {
        let mut not_initialized = true;
        self.prev_event = self.current_event.take();
        while self.current_event.is_none() {
            let Some(stream) = &mut self.stream else {
                self.new_log_stream(not_initialized).await;
                not_initialized = false;
                continue;
            };
            if let Some(log) = self.catchup_logs.pop_front() {
                if self.catchup_logs.is_empty() {
                    eprintln!("Last catchup event");
                };
                self.current_event = Some(log);
                break;
            };
            let Some(log) = stream.next().await else {
                // the stream ends, could be a restart of the full node, or just a temporary gap
                self.stream = None;
                if let (Some(end_at_block), Some(last_seen_block)) =
                    (self.end_at_block, self.last_valid_block)
                {
                    if end_at_block == last_seen_block {
                        eprintln!(
                            "Nothing to read, reached end of block range"
                        );
                        return None;
                    }
                }
                eprintln!("Nothing to read, retrying");
                tokio::time::sleep(Duration::from_secs(1)).await;
                continue;
            };
            self.current_event = Some(log);
            break;
        }
        let Some(current_event) = &self.current_event else {
            return None;
        };
        if let Some(block_number) = current_event.block_number  {
            // we subtract one because the current block is on going
            self.last_valid_block = Some(block_number.max(self.last_valid_block.unwrap_or_default()) - 1);
        }
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
}

pub async fn main(args: Args) {
    if let Some(acl_contract_address) = &args.acl_contract_address {
        if let Err(err) = Address::from_str(acl_contract_address) {
            panic!("Invalid acl contract address: {err}");
        };
    };
    if let Some(tfhe_contract_address) = &args.tfhe_contract_address {
        if let Err(err) = Address::from_str(tfhe_contract_address) {
            panic!("Invalid tfhe contract address: {err}");
        };
    }

    let mut log_iter = InfiniteLogIter::new(&args);
    let chain_id = log_iter.get_chain_id_or_panic().await;
    eprintln!("Chain ID: {chain_id}");

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
            panic!("A Coprocessor API key is required to access the database");
        }
    } else {
        None
    };

    log_iter.new_log_stream(true).await;

    let mut block_error_event_fthe = 0;
    while let Some(log) = log_iter.next().await {
        if log_iter.is_first_of_block() {
            if let Some(block_number) = log.block_number {
                if block_error_event_fthe == 0 {
                    if let Some(ref mut db) = db {
                        db.mark_prev_block_as_valid(
                            &log_iter.current_event,
                            &log_iter.prev_event,
                        )
                        .await;
                    }
                } else {
                    eprintln!(
                        "Errors in tfhe events: {block_error_event_fthe}"
                    );
                    block_error_event_fthe = 0;
                }
                eprintln!("\n--------------------");
                eprintln!("Block {block_number}");
            }
        };
        if block_error_event_fthe > 0 {
            eprintln!("Errors in block {block_error_event_fthe}");
        }
        if !args.ignore_tfhe_events {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner, true)
            {
                // TODO: filter on contract address if known
                println!("TFHE {event:#?}");
                if let Some(ref mut db) = db {
                    match db.insert_tfhe_event(&event).await {
                        Ok(_) => db.notify_scheduler().await,
                        Err(err) => {
                            block_error_event_fthe += 1;
                            eprintln!("Error inserting tfhe event: {err}")
                        }
                    }
                }
                continue;
            }
        }
        if !args.ignore_acl_events {
            if let Ok(event) =
                AclContract::AclContractEvents::decode_log(&log.inner, true)
            {
                println!("ACL {event:#?}");
                if let Some(ref mut db) = db {
                    let _ = db.handle_acl_event(&event).await;
                }
                continue;
            }
        }
    }
}
