use anyhow::{Context, Result};
use solana_host_listener::{config::ResolvedPollerConfig, poller::SolanaHostPoller};
use sqlx::Row;
use test_harness::{
    instance::{setup_test_db, ImportMode},
    solana_localnet::SolanaLocalnet,
};

#[tokio::test]
#[ignore = "requires local Solana/Anchor toolchain"]
async fn ingests_batch_scenario_into_coprocessor_db() -> Result<()> {
    let db = setup_test_db(ImportMode::None)
        .await
        .map_err(|err| anyhow::anyhow!(err.to_string()))?;
    let localnet = SolanaLocalnet::start().await?;
    let scenario = localnet.run_scenario("scenario-batch").await?;
    let addresses = localnet.load_addresses()?;
    let signatures = scenario["signatures"]
        .as_array()
        .context("scenario signatures must be an array")?
        .iter()
        .map(|value| {
            value
                .as_str()
                .map(str::to_owned)
                .context("signature must be a string")
        })
        .collect::<Result<Vec<_>>>()?;

    let poller = SolanaHostPoller::new(ResolvedPollerConfig {
        rpc_url: addresses["SOLANA_HOST_RPC_URL"].clone(),
        program_id: addresses["SOLANA_HOST_PROGRAM_ID"].clone(),
        host_chain_id: addresses["SOLANA_HOST_CHAIN_ID"].parse()?,
        database_url: db.db_url().to_owned(),
        batch_size_slots: 128,
        poll_interval_ms: 1000,
        retry_interval_ms: 1000,
        health_port: 8085,
        once: true,
        commitment: "confirmed".to_owned(),
        addresses_env: localnet.addresses_env_path().to_path_buf(),
    })
    .await?;
    poller.ingest_signatures(&signatures).await?;

    let pool = sqlx::PgPool::connect(db.db_url()).await?;
    let computations =
        sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM computations WHERE host_chain_id = $1")
            .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
            .fetch_one(&pool)
            .await?;
    let allowed_handles = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM allowed_handles WHERE host_chain_id = $1",
    )
    .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
    .fetch_one(&pool)
    .await?;
    let pbs = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM pbs_computations WHERE host_chain_id = $1",
    )
    .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
    .fetch_one(&pool)
    .await?;
    let blocks = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = $1",
    )
    .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
    .fetch_one(&pool)
    .await?;
    let last_block = sqlx::query(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
    )
    .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
    .fetch_one(&pool)
    .await?
    .get::<i64, _>("last_caught_up_block");
    let ops = sqlx::query_scalar::<_, i16>(
        "SELECT DISTINCT fhe_operation FROM computations WHERE host_chain_id = $1 ORDER BY fhe_operation",
    )
    .bind(addresses["SOLANA_HOST_CHAIN_ID"].parse::<i64>()?)
    .fetch_all(&pool)
    .await?;

    assert_eq!(
        computations, 4,
        "expected batch scenario to emit 4 computations"
    );
    assert_eq!(allowed_handles, 4, "expected 4 allowed handles");
    assert_eq!(pbs, 4, "expected 4 PBS rows");
    assert!(blocks >= 1, "expected at least one finalized block row");
    assert!(
        last_block > 0,
        "expected host_listener_poller_state to be updated"
    );
    assert_eq!(
        ops,
        vec![0, 23, 24],
        "expected add/cast/trivial-encrypt operations"
    );

    Ok(())
}
