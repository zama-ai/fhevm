use connector_tests::{
    rand::{rand_address, rand_digest, rand_public_key, rand_sns_ct, rand_u256},
    setup::test_instance_with_db_only,
};
use connector_utils::types::{GatewayEvent, db::SnsCiphertextMaterialDbItem};
use fhevm_gateway_rust_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kmsmanagement::KmsManagement::{
        CrsgenRequest, KeygenRequest, KskgenRequest, PreprocessKeygenRequest,
        PreprocessKskgenRequest,
    },
};
use kms_worker::core::{DbEventPicker, EventPicker};

#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let decryption_id = rand_u256();
    let sns_ct = vec![rand_sns_ct()];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    println!("Triggering Postgres notification with PublicDecryptionRequest insertion...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking PublicDecryptionRequest...");
    let event = event_picker.pick_event().await?;

    println!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        event,
        GatewayEvent::PublicDecryption(PublicDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct,
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

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
        "INSERT INTO user_decryption_requests VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking UserDecryptionRequest...");
    let event_tx = event_picker.pick_event().await?;

    println!("Checking UserDecryptionRequest data...");
    assert_eq!(
        event_tx,
        GatewayEvent::UserDecryption(UserDecryptionRequest {
            decryptionId: decryption_id,
            snsCtMaterials: sns_ct,
            userAddress: user_address,
            publicKey: public_key.into(),
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_preprocess_keygen() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let pre_keygen_request_id = rand_u256();
    let fhe_params_digest = rand_digest();

    println!("Triggering Postgres notification with PreprocessKeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO preprocess_keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        pre_keygen_request_id.as_le_slice(),
        fhe_params_digest.as_slice(),
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking PreprocessKeygenRequest...");
    let event_tx = event_picker.pick_event().await?;

    println!("Checking PreprocessKeygenRequest data...");
    assert_eq!(
        event_tx,
        GatewayEvent::PreprocessKeygen(PreprocessKeygenRequest {
            preKeygenRequestId: pre_keygen_request_id,
            fheParamsDigest: fhe_params_digest,
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_preprocess_kskgen() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let pre_kskgen_request_id = rand_u256();
    let fhe_params_digest = rand_digest();

    println!("Triggering Postgres notification with PreprocessKskgenRequest insertion...");
    sqlx::query!(
        "INSERT INTO preprocess_kskgen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        pre_kskgen_request_id.as_le_slice(),
        fhe_params_digest.as_slice(),
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking PreprocessKskgenRequest...");
    let event_tx = event_picker.pick_event().await?;

    println!("Checking PreprocessKskgenRequest data...");
    assert_eq!(
        event_tx,
        GatewayEvent::PreprocessKskgen(PreprocessKskgenRequest {
            preKskgenRequestId: pre_kskgen_request_id,
            fheParamsDigest: fhe_params_digest,
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_keygen() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let pre_key_id = rand_u256();
    let fhe_params_digest = rand_digest();

    println!("Triggering Postgres notification with KeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO keygen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        pre_key_id.as_le_slice(),
        fhe_params_digest.as_slice(),
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking KeygenRequest...");
    let event_tx = event_picker.pick_event().await?;

    println!("Checking KeygenRequest data...");
    assert_eq!(
        event_tx,
        GatewayEvent::Keygen(KeygenRequest {
            preKeyId: pre_key_id,
            fheParamsDigest: fhe_params_digest,
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_kskgen() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let pre_ksk_id = rand_u256();
    let source_key_id = rand_u256();
    let dest_key_id = rand_u256();
    let fhe_params_digest = rand_digest();

    println!("Triggering Postgres notification with KskgenRequest insertion...");
    sqlx::query!(
        "INSERT INTO kskgen_requests VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        pre_ksk_id.as_le_slice(),
        source_key_id.as_le_slice(),
        dest_key_id.as_le_slice(),
        fhe_params_digest.as_slice(),
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking KskgenRequest...");
    let event_tx = event_picker.pick_event().await?;

    println!("Checking KskgenRequest data...");
    assert_eq!(
        event_tx,
        GatewayEvent::Kskgen(KskgenRequest {
            preKskId: pre_ksk_id,
            sourceKeyId: source_key_id,
            destKeyId: dest_key_id,
            fheParamsDigest: fhe_params_digest,
        })
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_crsgen() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut event_picker = DbEventPicker::connect(test_instance.db.clone()).await?;

    let crsgen_request_id = rand_u256();
    let fhe_params_digest = rand_digest();

    println!("Triggering Postgres notification with CrsgenRequest insertion...");
    sqlx::query!(
        "INSERT INTO crsgen_requests VALUES ($1, $2) ON CONFLICT DO NOTHING",
        crsgen_request_id.as_le_slice(),
        fhe_params_digest.as_slice(),
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking CrsgenRequest...");
    let event = event_picker.pick_event().await?;

    println!("Checking CrsgenRequest data...");
    assert_eq!(
        event,
        GatewayEvent::Crsgen(CrsgenRequest {
            crsgenRequestId: crsgen_request_id,
            fheParamsDigest: fhe_params_digest,
        })
    );
    println!("Data OK!");
    Ok(())
}
