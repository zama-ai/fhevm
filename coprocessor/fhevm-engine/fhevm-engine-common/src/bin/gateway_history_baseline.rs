//! Seed a verified legacy Gateway cursor before the containment rollout.
use alloy::primitives::B256;
use clap::Parser;
use fhevm_engine_common::{
    chain_recovery::{establish_gateway_baseline, ListenerHeader},
    database::{connect_pool_with_options, resolve_database_url_from_option},
};
use sqlx::postgres::PgPoolOptions;

#[derive(Parser)]
#[command(about = "Check or apply an independently verified Gateway cursor baseline")]
struct Args {
    #[arg(long, default_value = "public")]
    schema: String,
    #[arg(long)]
    chain_id: i64,
    #[arg(long)]
    block_number: i64,
    #[arg(long)]
    block_hash: B256,
    #[arg(long)]
    parent_hash: B256,
    /// Reference to the independent canonical-history and consumed-work audit.
    #[arg(long)]
    audit_reference: String,
    /// Commit the checkpoint. Without this flag, validate and roll back.
    #[arg(long)]
    apply: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    anyhow::ensure!(
        !args.audit_reference.trim().is_empty(),
        "an audit reference is required"
    );
    let database_url = resolve_database_url_from_option(None)?;
    let (pool, _refresh) =
        connect_pool_with_options(&database_url, PgPoolOptions::new().max_connections(1), None)
            .await?;
    let header = ListenerHeader {
        number: args.block_number,
        hash: args.block_hash.to_vec(),
        parent_hash: args.parent_hash.to_vec(),
    };
    establish_gateway_baseline(&pool, &args.schema, args.chain_id, &header, args.apply).await?;
    // Save this receipt alongside the snapshot and independently verified
    // header evidence. No credentials or database connection string are logged.
    println!(
        "{}",
        serde_json::json!({
            "action": "gateway_history_baseline",
            "applied": args.apply,
            "schema": args.schema,
            "chain_id": args.chain_id,
            "block_number": args.block_number,
            "block_hash": args.block_hash.to_string(),
            "parent_hash": args.parent_hash.to_string(),
            "audit_reference": args.audit_reference,
        })
    );
    Ok(())
}
