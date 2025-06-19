use std::time::Duration;

use alloy::primitives::U256;
use connector_tests::{rand::rand_sns_ct, setup::test_instance_with_db_only};
use connector_utils::types::{GatewayEvent, db::SnsCiphertextMaterialDbItem};
use fhevm_gateway_rust_bindings::decryption::Decryption::PublicDecryptionRequest;
use kms_worker::core::{DbEventPicker, EventPicker};
use tokio::time::timeout;

#[tokio::test]
async fn test_parallel_event_picker_one_events() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker0 = DbEventPicker::connect(test_instance.db.clone()).await?;
    let mut event_picker1 = DbEventPicker::connect(test_instance.db.clone()).await?;

    let id0 = U256::ZERO;
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Inserting only one PublicDecryptionRequest for two event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        id0.as_le_slice(),
        sns_ciphertexts_db.clone() as Vec<SnsCiphertextMaterialDbItem>,
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking PublicDecryptionRequest...");
    let event_tx0 = event_picker0.pick_event().await?;

    // Should wait forever
    if let Ok(res) = timeout(Duration::from_millis(300), event_picker1.pick_event()).await {
        panic!("Timeout was expected, got result instead: {:?}", res);
    }

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        event_tx0.event,
        GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id0,
            snsCtMaterials: sns_ct.clone(),
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_parallel_event_picker_two_events() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker0 = DbEventPicker::connect(test_instance.db.clone()).await?;
    let mut event_picker1 = DbEventPicker::connect(test_instance.db.clone()).await?;

    let id0 = U256::ZERO;
    let id1 = U256::ONE;
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Inserting two PublicDecryptionRequest for two event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        id0.as_le_slice(),
        sns_ciphertexts_db.clone() as Vec<SnsCiphertextMaterialDbItem>,
    )
    .execute(&test_instance.db)
    .await?;
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        id1.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking the two PublicDecryptionRequest...");
    let event_tx0 = event_picker0.pick_event().await?;
    let event_tx1 = event_picker1.pick_event().await?;

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        event_tx0.event,
        GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id0,
            snsCtMaterials: sns_ct.clone(),
        })
    );
    assert_eq!(
        event_tx1.event,
        GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: id1,
            snsCtMaterials: sns_ct,
        })
    );
    println!("Data OK!");
    Ok(())
}
