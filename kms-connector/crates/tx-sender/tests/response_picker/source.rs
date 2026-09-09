use alloy::primitives::U256;
use connector_utils::{
    tests::{
        db::responses::{TestResponseType, insert_rand_response},
        rand::rand_u256,
        setup::TestInstanceBuilder,
    },
    types::db::RequestSource,
};
use rstest::rstest;
use sqlx::{Pool, Postgres, Row};
use std::time::Duration;
use tracing::info;
use tx_sender::core::{Config, DbKmsResponsePicker, KmsResponsePicker};

#[rstest]
#[case::public_decryption(TestResponseType::PublicDecryption)]
#[case::user_decryption(TestResponseType::UserDecryption)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_http_sourced_responses_are_ignored(
    #[case] response_type: TestResponseType,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let db = test_instance.db();

    info!("Inserting HTTP-sourced {response_type} before starting the picker...");
    let http_response_id = rand_u256();
    insert_rand_response(
        db,
        response_type,
        Some(http_response_id),
        None,
        RequestSource::Http,
    )
    .await?;

    info!("Inserting onchain-sourced {response_type} before starting the picker...");
    let onchain_response =
        insert_rand_response(db, response_type, None, None, RequestSource::OnChain).await?;

    let config = Config {
        database_polling_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let mut response_picker = DbKmsResponsePicker::connect(db.clone(), &config).await?;

    info!("Picking {response_type}...");
    let responses = response_picker.pick_responses().await?;

    info!("Checking only the onchain-sourced {response_type} was picked...");
    assert_eq!(responses.len(), 1);
    assert_eq!(responses[0].kind, onchain_response);
    assert!(response_is_pending(db, response_type, http_response_id).await?);
    info!("Data OK!");
    Ok(())
}

fn response_table(response_type: TestResponseType) -> &'static str {
    match response_type {
        TestResponseType::PublicDecryption => "public_decryption_responses",
        TestResponseType::UserDecryption => "user_decryption_responses",
        _ => panic!("only decryption responses carry a source"),
    }
}

async fn response_is_pending(
    db: &Pool<Postgres>,
    response_type: TestResponseType,
    decryption_id: U256,
) -> anyhow::Result<bool> {
    let table = response_table(response_type);
    let row = sqlx::query(&format!(
        "SELECT (status = 'pending') AS is_pending FROM {table} WHERE decryption_id = $1"
    ))
    .bind(decryption_id.as_le_slice())
    .fetch_one(db)
    .await?;
    Ok(row.try_get("is_pending")?)
}
