mod dependence_chain;
mod errors;
mod event_helpers;
mod health_check;
mod inputs;
mod migrations;
mod operators;
mod operators_from_events;
mod random;
mod scheduling_bench;
mod test_cases;
mod utils;

use test_harness::db_utils::setup_test_key as setup_test_key_in_db;

#[tokio::test]
#[ignore]
/// setup test data with keys
async fn setup_test_key() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&std::env::var("DATABASE_URL").expect("expected to get db url"))
        .await?;

    setup_test_key_in_db(&pool, false).await?;

    Ok(())
}
