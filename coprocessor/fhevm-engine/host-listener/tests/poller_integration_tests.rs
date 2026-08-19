mod common;

use serial_test::serial;

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::ingest::IngestOptions;
use host_listener::database::tfhe_event_propagate::Database;
use test_harness::instance::ImportMode;

use alloy::primitives::{FixedBytes, U256};
use common::{
    allowed_log, block_summary, caller_at, ingest_logs, trivial_encrypt_log,
    tx_hash_for,
};

#[tokio::test]
#[serial(db)]
async fn poller_state_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let chain_id = ChainId::try_from(42_u64).unwrap();

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let mut db = Database::new(&db_url, chain_id, 128).await?;

    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;

    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, None);

    db.poller_set_last_caught_up_block(chain_id, 5).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(5));

    db.reconnect().await;
    db.poller_set_last_caught_up_block(chain_id, 7).await?;
    assert_eq!(db.poller_get_last_caught_up_block(chain_id).await?, Some(7));

    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn poller_catches_up_to_safe_tip(
) -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let chain_id = ChainId::try_from(42_u64).unwrap();

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let mut db = Database::new(&db_url, chain_id, 128).await?;
    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM computations_branch")
        .execute(&pool)
        .await?;
    sqlx::query("DELETE FROM allowed_handles_branch")
        .execute(&pool)
        .await?;

    let latest_block = 5u64;
    let finality_lag = 2u64;
    let safe_tip = latest_block.saturating_sub(finality_lag);
    let caller = caller_at(0);
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
        is_protocol_config_listener: true,
    };

    let mut expected_tfhe = 0i64;
    let mut expected_acl = 0i64;
    for i in 1..=latest_block {
        let pt = U256::from(i);
        let handle = FixedBytes::<32>::from(pt.to_be_bytes());
        let tx_hash = tx_hash_for(i);
        let logs = vec![
            trivial_encrypt_log(caller, pt, 4_u8, handle, tx_hash, 0),
            allowed_log(caller, caller, handle, tx_hash, 1),
        ];
        if i <= safe_tip {
            ingest_logs(&mut db, logs, block_summary(i), true, options.clone())
                .await?;
            expected_tfhe += 1;
            expected_acl += 1;
        }
    }

    assert!(expected_tfhe > 0, "no finalized TFHE events to ingest");
    assert!(expected_acl > 0, "no finalized ACL events to ingest");

    let computations_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM computations_branch",
    )
    .fetch_one(&pool)
    .await?;
    let allowed_count = sqlx::query_scalar::<_, i64>(
        "SELECT COUNT(*) FROM allowed_handles_branch",
    )
    .fetch_one(&pool)
    .await?;
    let last_valid_block = sqlx::query_scalar::<_, Option<i64>>(
        "SELECT MAX(block_number) FROM host_chain_blocks_valid \
         WHERE chain_id = $1",
    )
    .bind(chain_id.as_i64())
    .fetch_one(&pool)
    .await?
    .unwrap_or_default();

    assert_eq!(computations_count, expected_tfhe);
    assert_eq!(allowed_count, expected_acl);
    assert_eq!(last_valid_block as u64, safe_tip);

    Ok(())
}
