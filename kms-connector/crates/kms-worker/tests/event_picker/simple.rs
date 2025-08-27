use std::time::Duration;

use connector_utils::{
    tests::{
        rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256},
        setup::TestInstanceBuilder,
    },
    types::{
        GatewayEvent,
        db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_management::KmsManagement::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
};
use kms_worker::core::{Config, DbEventPicker, EventPicker};

#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let decryption_id = rand_u256();
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Triggering Postgres notification with PublicDecryptionRequest insertion...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking PublicDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct,
            extraData: vec![].into(),
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let decryption_id = rand_u256();
    let sns_ct = vec![rand_sns_ct()];
    let user_address = rand_address();
    let public_key = rand_public_key();
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Triggering Postgres notification with UserDecryptionRequest insertion...");
    sqlx::query!(
        "INSERT INTO user_decryption_requests VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
        vec![],
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking UserDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking UserDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::UserDecryption(UserDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct,
            userAddress: user_address,
            publicKey: public_key.into(),
            extraData: vec![].into(),
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_prep_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let prep_keygen_request_id = rand_u256();
    let epoch_id = rand_u256();
    let params_type = ParamsTypeDb::Test;

    println!("Triggering Postgres notification with PrepKeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO prep_keygen_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        prep_keygen_request_id.as_le_slice(),
        epoch_id.as_le_slice(),
        params_type as ParamsTypeDb,
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking PrepKeygenRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking PrepKeygenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::PrepKeygen(PrepKeygenRequest {
            prepKeygenId: prep_keygen_request_id,
            epochId: epoch_id,
            paramsType: params_type as u8,
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let prep_key_id = rand_u256();
    let key_id = rand_u256();

    println!("Triggering Postgres notification with KeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking KeygenRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking KeygenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::Keygen(KeygenRequest {
            prepKeygenId: prep_key_id,
            keyId: key_id,
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_crsgen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let crs_id = rand_u256();
    let max_bit_length = rand_u256();
    let params_type = ParamsTypeDb::Test;

    println!("Triggering Postgres notification with CrsgenRequest insertion...");
    sqlx::query!(
        "INSERT INTO crsgen_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
    )
    .execute(test_instance.db())
    .await?;

    println!("Picking CrsgenRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking CrsgenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::Crsgen(CrsgenRequest {
            crsId: crs_id,
            maxBitLength: max_bit_length,
            paramsType: params_type as u8,
        })]
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_polling_backup() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let decryption_id = rand_u256();
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();
    println!("Inserting PublicDecryptionRequest before starting the event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
    )
    .execute(test_instance.db())
    .await?;

    let config = Config {
        database_polling_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let mut event_picker = DbEventPicker::connect(test_instance.db().clone(), &config).await?;

    println!("Picking PublicDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct,
            extraData: vec![].into(),
        })]
    );
    println!("Data OK!");
    Ok(())
}
