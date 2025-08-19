use std::time::Duration;

use alloy::primitives::U256;
use connector_utils::{
    tests::{rand::rand_sns_ct, setup::TestInstanceBuilder},
    types::{GatewayEvent, db::SnsCiphertextMaterialDbItem},
};
use fhevm_gateway_rust_bindings::decryption::Decryption::PublicDecryptionRequest;
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use tokio::time::timeout;

#[tokio::test]
async fn test_parallel_event_picker_one_events() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let config = Config::default();
    let mut event_picker0 = DbEventPicker::connect(test_instance.db().clone(), &config).await?;
    let mut event_picker1 = DbEventPicker::connect(test_instance.db().clone(), &config).await?;

    let id0 = U256::ZERO;
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Inserting only one PublicDecryptionRequest for two event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        id0.as_le_slice(),
        sns_ciphertexts_db.clone() as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking PublicDecryptionRequest...");
    let events0 = event_picker0.pick_events().await?;

    // Should wait forever
    if let Ok(res) = timeout(Duration::from_millis(300), event_picker1.pick_events()).await {
        panic!("Timeout was expected, got result instead: {res:?}");
    }

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events0,
        vec![GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id0,
            snsCtMaterials: sns_ct.clone(),
            extraData: vec![].into()
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_parallel_event_picker_two_events() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let config = Config {
        events_batch_size: 1,
        ..Default::default()
    };
    let mut event_picker0 = DbEventPicker::connect(test_instance.db().clone(), &config).await?;
    let mut event_picker1 = DbEventPicker::connect(test_instance.db().clone(), &config).await?;

    let id0 = U256::ZERO;
    let id1 = U256::ONE;
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Inserting two PublicDecryptionRequest for two event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        id0.as_le_slice(),
        sns_ciphertexts_db.clone() as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
    )
    .execute(test_instance.db())
    .await?;
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        id1.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking the two PublicDecryptionRequest...");
    let events0 = event_picker0.pick_events().await?;
    let events1 = event_picker1.pick_events().await?;

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events0,
        vec![GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id0,
            snsCtMaterials: sns_ct.clone(),
            extraData: vec![].into(),
        })]
    );
    assert_eq!(
        events1,
        vec![GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id1,
            snsCtMaterials: sns_ct,
            extraData: vec![].into(),
        })]
    );
    println!("Data OK!");
    Ok(())
}
