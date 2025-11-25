use sqlx::postgres::PgPoolOptions;

use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::tfhe_event_propagate::Database;
use test_harness::instance::ImportMode;

#[tokio::test]
async fn poller_state_round_trip() -> Result<(), Box<dyn std::error::Error>> {
    let db_instance =
        test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
            .await?;
    let pool = PgPoolOptions::new()
        .max_connections(1)
        .connect(db_instance.db_url())
        .await?;

    let coprocessor_api_key =
        sqlx::query!("SELECT tenant_api_key FROM tenants LIMIT 1")
            .fetch_one(&pool)
            .await?
            .tenant_api_key;

    let db_url: DatabaseURL = db_instance.db_url.clone();
    let mut db = Database::new(&db_url, &coprocessor_api_key, 128).await?;
    let chain_id = i64::try_from(db.chain_id).unwrap();

    let pool = db.pool.read().await.clone();
    sqlx::query("DELETE FROM host_listener_poller_state WHERE chain_id = $1")
        .bind(chain_id)
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
