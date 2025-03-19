use alloy_provider::fillers::{
    BlobGasFiller, ChainIdFiller, FillProvider, GasFiller, JoinFill,
    NonceFiller,
};
use futures_util::stream::StreamExt;
use sqlx::types::Uuid;
use std::str::FromStr;
use std::time::Duration;

use alloy::primitives::Address;
use alloy::providers::{Provider, ProviderBuilder, RootProvider, WsConnect};
use alloy::pubsub::SubscriptionStream;
use alloy::rpc::types::{BlockNumberOrTag, Filter, Log};

use alloy_sol_types::SolEventInterface;

use clap::Parser;

use fhevm_listener::contracts::{AclContract, TfheContract};
use fhevm_listener::database::tfhe_event_propagate::{
    Database, EVENT_WORK_AVAILABLE,
};

const DEFAULT_CATCHUP: u64 = 5;

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
    contract_addresses: Vec<Address>,
    stream: Option<SubscriptionStream<Log>>,
    provider: Option<RProvider>, // required to maintain the stream
    last_seen_block: Option<u64>,
    start_at_block: Option<i64>,
    end_at_block: Option<u64>,
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
            contract_addresses,
            stream: None,
            provider: None,
            last_seen_block: None,
            start_at_block: args.start_at_block,
            end_at_block: args.end_at_block,
        }
    }

    async fn catchup_block_from(
        &self,
        provider: &RProvider,
    ) -> BlockNumberOrTag {
        if let Some(last_seen_block) = self.last_seen_block {
            return BlockNumberOrTag::Number(last_seen_block - 1);
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
            DEFAULT_CATCHUP
        };
        BlockNumberOrTag::Number(last_block - catch_size.min(last_block))
    }

    async fn new_log_stream(&mut self, not_initialized: bool) {
        let mut retry = 20;
        loop {
            let ws = WsConnect::new(&self.url);
            match ProviderBuilder::new().on_ws(ws).await {
                Ok(provider) => {
                    let catch_up_from =
                        self.catchup_block_from(&provider).await;
                    if not_initialized {
                        eprintln!("Catchup from {:?}", catch_up_from);
                    }
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
                    self.stream = Some(
                        provider
                            .subscribe_logs(&filter)
                            .await
                            .expect("BLA2")
                            .into_stream(),
                    );
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
                        eprintln!("Cannot connect to {} due to {err}. Will retry in {delay} secs, {retry} times.", &self.url);
                    } else {
                        eprintln!("Cannot connect to {} due to {err}. Will retry in {delay} secs, indefinitively.", &self.url);
                    }
                    retry -= 1;
                    tokio::time::sleep(Duration::from_secs(delay)).await;
                }
            }
        }
    }

    async fn next(&mut self) -> Option<Log> {
        let mut not_initialized = true;
        loop {
            let Some(stream) = &mut self.stream else {
                self.new_log_stream(not_initialized).await;
                not_initialized = false;
                continue;
            };
            let Some(log) = stream.next().await else {
                // the stream ends, could be a restart of the full node, or just
                // a temporary gap
                self.stream = None;
                if let (Some(end_at_block), Some(last_seen_block)) =
                    (self.end_at_block, self.last_seen_block)
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
            return Some(log);
        }
    }
}

#[tokio::main]
async fn main() {
    let args = Args::parse();
    let mut log_iter = InfiniteLogIter::new(&args);
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

    let mut db: Option<Database> = None;
    if !args.database_url.is_empty() {
        if let Some(coprocessor_api_key) = args.coprocessor_api_key {
            db = Some(
                Database::new(&args.database_url, &coprocessor_api_key).await,
            );
        } else {
            panic!("A Coprocessor API key is required to access the database");
        }
    }
    log_iter.new_log_stream(true).await;
    while let Some(log) = log_iter.next().await {
        if let Some(block_number) = log.block_number {
            eprintln!("Event at block: {}", { block_number });
            log_iter.last_seen_block = Some(block_number);
        }
        if !args.ignore_tfhe_events {
            if let Ok(event) =
                TfheContract::TfheContractEvents::decode_log(&log.inner, true)
            {
                // TODO: filter on contract address if known
                println!("\nTFHE {event:#?}");
                if let Some(ref mut db) = db {
                    match db.insert_tfhe_event(&event).await {
                        Ok(_) => db.notify_database(EVENT_WORK_AVAILABLE).await, /* we always notify, e.g. for catchup */
                        Err(err) => {
                            eprintln!("Error inserting tfhe event: {err}")
                        }
                    }
                }
                continue;
            }
        }
        if !args.ignore_tfhe_events {
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
