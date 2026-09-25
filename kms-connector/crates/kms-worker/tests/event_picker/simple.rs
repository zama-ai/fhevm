use alloy::primitives::U256;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::{
        db::requests::{InsertRequestOptions, TestEventType, insert_rand_request},
        rand::{rand_digest, rand_solana_handle},
        setup::TestInstanceBuilder,
    },
    types::{
        ProtocolEventKind,
        db::{RequestSource, insert_solana_public_decryption},
        solana_request::SolanaPublicDecryptionRequest,
    },
};
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use rstest::rstest;
use std::time::Duration;
use tracing::info;

#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption)]
#[case::user_decryption(TestEventType::UserDecryption)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2)]
#[case::prep_keygen(TestEventType::PrepKeygen)]
#[case::keygen(TestEventType::Keygen)]
#[case::crsgen(TestEventType::Crsgen)]
#[case::new_kms_context(TestEventType::NewKmsContext)]
#[case::new_kms_epoch(TestEventType::NewKmsEpoch)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_request(#[case] event_type: TestEventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    info!("Triggering Postgres notification with {event_type} insertion...");
    let inserted_request = insert_rand_request(
        test_instance.db(),
        event_type,
        InsertRequestOptions::default(),
    )
    .await?;

    info!("Picking {event_type}...");
    let events = event_picker.pick_events().await?;

    info!("Checking {event_type} data...");
    assert_eq!(
        events.into_iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![inserted_request],
    );
    info!("Data OK!");
    Ok(())
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_pick_solana_public_decryption_with_its_stores() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;
    // Two handles of one chain: they differ only outside the chain id bytes.
    let first = rand_solana_handle();
    let mut second = first;
    second[0] ^= 1;
    let request = SolanaPublicDecryptionRequest::new(
        U256::ONE,
        &[first, second],
        &[rand_digest(), rand_digest()],
        vec![0x00],
    )?;
    insert_solana_public_decryption(
        test_instance.db(),
        &request,
        None,
        sqlx::types::chrono::Utc::now(),
        &PropagationContext::default(),
        RequestSource::OnChain,
    )
    .await?;

    let events = event_picker.pick_events().await?;

    assert_eq!(
        events.into_iter().map(|e| e.kind).collect::<Vec<_>>(),
        vec![ProtocolEventKind::SolanaPublicDecryption(request)],
    );
    Ok(())
}
