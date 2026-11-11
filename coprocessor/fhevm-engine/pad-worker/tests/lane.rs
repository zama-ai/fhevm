use pad_worker::register_lane;
use sqlx::postgres::PgPoolOptions;
use test_harness::instance::{setup_test_db, ImportMode};

#[tokio::test]
async fn registers_a_lane_once() {
    let instance = setup_test_db(ImportMode::None).await.expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(2)
        .connect(instance.db_url())
        .await
        .unwrap();

    // Each pod of the lane registers it at start.
    register_lane(&pool, 7).await.unwrap();
    register_lane(&pool, 7).await.unwrap();

    let lanes: Vec<(i32, bool)> =
        sqlx::query_as("SELECT prf_instance, ready_at IS NULL FROM prf_lanes")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(lanes, vec![(7, true)]);
}
