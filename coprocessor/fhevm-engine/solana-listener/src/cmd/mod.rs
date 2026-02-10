use std::time::Duration;

use anyhow::Result;
use clap::Parser;
use solana_pubkey::Pubkey;
use tracing::Level;

use crate::database::solana_event_propagate::Database;
use crate::poller::solana_rpc_source::SolanaRpcEventSource;
use crate::poller::{Cursor, PollerConfig};

#[derive(Parser, Debug, Clone)]
pub struct Args {
    #[arg(long, env = "DATABASE_URL")]
    pub database_url: String,

    /// DB partition key for current coprocessor schema compatibility.
    #[arg(long, env = "TENANT_ID", help = "DB tenant partition key")]
    pub tenant_id: i32,

    /// Listener-side chain namespace persisted in DB.
    /// This is configured off-chain and is not read from Solana state.
    #[arg(
        long,
        env = "HOST_CHAIN_ID",
        help = "Off-chain host chain namespace for DB rows"
    )]
    pub host_chain_id: i64,

    #[arg(long, env = "SOLANA_RPC_URL", default_value = "http://127.0.0.1:8899")]
    pub solana_rpc_url: String,

    /// Solana host program id to ingest from. The listener only decodes
    /// events/CPI payloads emitted by this program.
    #[arg(
        long,
        env = "SOLANA_PROGRAM_ID",
        help = "Target Solana host program id (base58 pubkey)"
    )]
    pub solana_program_id: Pubkey,

    #[arg(long, env = "SOLANA_START_SLOT", default_value_t = 0)]
    pub start_slot: u64,

    #[arg(long, env = "SOLANA_POLLER_INTERVAL_MS", default_value_t = 1000)]
    pub poller_interval_ms: u64,

    #[arg(long, env = "SOLANA_MAX_BATCH_SIZE", default_value_t = 200)]
    pub max_batch_size: usize,

    #[arg(long, env = "SOLANA_FINALIZED_ONLY", default_value_t = true)]
    pub finalized_only: bool,

    #[arg(long, env = "SOLANA_DRY_RUN", default_value_t = true)]
    pub dry_run: bool,

    #[arg(long, env = "LOG_LEVEL", default_value_t = Level::INFO)]
    pub log_level: Level,
}

pub async fn main(args: Args) -> Result<()> {
    let poller_config = PollerConfig {
        poll_interval: Duration::from_millis(args.poller_interval_ms),
        max_batch_size: args.max_batch_size,
        finalized_only: args.finalized_only,
    };

    let mut source = SolanaRpcEventSource::new(
        args.solana_rpc_url.clone(),
        args.solana_program_id.to_string(),
        args.host_chain_id,
    );
    let mut cursor = Cursor {
        slot: args.start_slot,
        tx_index: 0,
        op_index: 0,
    };

    let mut store = if args.dry_run {
        Database::new_dry_run(args.host_chain_id, args.tenant_id)
    } else {
        Database::connect(&args.database_url, args.host_chain_id, args.tenant_id).await?
    };

    crate::poller::run_poller(&poller_config, &mut source, &mut store, &mut cursor).await
}
