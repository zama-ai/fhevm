use alloy::primitives::FixedBytes;
use fhevm_engine_common::chain_id::ChainId;
use host_listener::cmd::block_history::BlockSummary;
use host_listener::database::tfhe_event_propagate::{
    Database, StatsForConsumer,
};
use serial_test::serial;
use sqlx::postgres::PgPoolOptions;
use std::time::Duration;
use test_harness::instance::ImportMode;

fn block_summary(number: u64) -> BlockSummary {
    BlockSummary {
        number,
        hash: FixedBytes::with_last_byte(number as u8),
        parent_hash: FixedBytes::with_last_byte(number.saturating_sub(1) as u8),
        timestamp: number,
    }
}

async fn setup_db() -> Result<
    (Database, sqlx::PgPool, test_harness::instance::DBInstance),
    Box<dyn std::error::Error>,
> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::None).await?;
    let chain_id = ChainId::try_from(42_u64)?;
    let database = Database::new(&db_instance.db_url, chain_id, 128).await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;

    Ok((database, pool, db_instance))
}

async fn mark_seen_blocks(
    database: &Database,
    blocks: &[u64],
) -> Result<(), sqlx::Error> {
    for block in blocks {
        database
            .mark_block_as_seen_by_consumer(&block_summary(*block), false)
            .await?;
    }
    Ok(())
}

async fn simulate_main_host_listener_insert(
    database: &Database,
    pool: &sqlx::PgPool,
    block: &BlockSummary,
) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    database.mark_block_as_valid(&mut tx, block, false).await?;
    tx.commit().await
}

#[tokio::test]
#[serial(db)]
async fn mark_block_as_seen_by_consumer_counts_duplicates(
) -> Result<(), Box<dyn std::error::Error>> {
    let (database, pool, _db_instance) = setup_db().await?;
    let block = block_summary(10);

    database
        .mark_block_as_seen_by_consumer(&block, false)
        .await?;
    database
        .mark_block_as_seen_by_consumer(&block, false)
        .await?;
    database
        .mark_block_as_seen_by_consumer(&block, true)
        .await?;

    let row = sqlx::query!(
        r#"
        SELECT block_number, duplicate_count, stats_processed
        FROM host_chain_consumer_blocks
        WHERE chain_id = $1 AND block_hash = $2
        "#,
        database.chain_id.as_i64(),
        block.hash.to_vec(),
    )
    .fetch_one(&pool)
    .await?;

    assert_eq!(row.block_number, block.number as i64);
    assert_eq!(row.duplicate_count, 1);
    assert!(!row.stats_processed);

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn detect_gap_seen_by_consumer_reports_new_gaps_once(
) -> Result<(), Box<dyn std::error::Error>> {
    let (database, _pool, _db_instance) = setup_db().await?;
    mark_seen_blocks(&database, &[1, 2, 5, 6, 10, 11, 15]).await?;

    let stats = database.detect_gap_seen_by_consumer(0).await?;
    assert_eq!(
        stats,
        StatsForConsumer {
            number_of_new_gaps: 2,
            total_new_gap_size: 5,
            number_of_duplicated_inserts: 0,
        }
    );

    let stats = database.detect_gap_seen_by_consumer(0).await?;
    assert_eq!(
        stats,
        StatsForConsumer {
            number_of_new_gaps: 0,
            total_new_gap_size: 0,
            number_of_duplicated_inserts: 0,
        }
    );

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn detect_gap_seen_by_consumer_reports_duplicates_once(
) -> Result<(), Box<dyn std::error::Error>> {
    let (database, _pool, _db_instance) = setup_db().await?;
    mark_seen_blocks(&database, &[1, 2, 3, 4]).await?;
    database
        .mark_block_as_seen_by_consumer(&block_summary(2), false)
        .await?;
    database
        .mark_block_as_seen_by_consumer(&block_summary(2), true)
        .await?;
    database
        .mark_block_as_seen_by_consumer(&block_summary(3), false)
        .await?;

    let stats = database.detect_gap_seen_by_consumer(0).await?;
    assert_eq!(
        stats,
        StatsForConsumer {
            number_of_new_gaps: 0,
            total_new_gap_size: 0,
            number_of_duplicated_inserts: 2,
        }
    );

    let stats = database.detect_gap_seen_by_consumer(0).await?;
    assert_eq!(
        stats,
        StatsForConsumer {
            number_of_new_gaps: 0,
            total_new_gap_size: 0,
            number_of_duplicated_inserts: 0,
        }
    );

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn mark_block_as_seen_by_consumer_reports_delay_after_main_listener_insert(
) -> Result<(), Box<dyn std::error::Error>> {
    let (database, pool, _db_instance) = setup_db().await?;
    let block = block_summary(11);

    simulate_main_host_listener_insert(&database, &pool, &block).await?;
    tokio::time::sleep(Duration::from_millis(50)).await;

    let delay = database
        .mark_block_as_seen_by_consumer(&block, false)
        .await?;

    assert!(delay > 0.0, "expected positive delay, got {delay}");

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn mark_block_as_seen_by_consumer_reports_zero_when_consumer_inserts_first(
) -> Result<(), Box<dyn std::error::Error>> {
    let (database, pool, _db_instance) = setup_db().await?;
    let block = block_summary(12);

    let delay = database
        .mark_block_as_seen_by_consumer(&block, false)
        .await?;

    assert_eq!(delay, 0.0);

    tokio::time::sleep(Duration::from_millis(50)).await;
    simulate_main_host_listener_insert(&database, &pool, &block).await?;

    let duplicate_delay = database
        .mark_block_as_seen_by_consumer(&block, false)
        .await?;

    assert_eq!(duplicate_delay, 0.0);

    Ok(())
}
