use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::{
        rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256},
        setup::TestInstanceBuilder,
    },
    types::{
        GatewayEvent, GatewayEventKind,
        db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_generation::KMSGeneration::{CrsgenRequest, KeygenRequest, PrepKeygenRequest},
};
use kms_worker::core::{Config, DbEventPicker, EventPicker};
use std::time::Duration;
use tracing::info;

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

    info!("Triggering Postgres notification with PublicDecryptionRequest insertion...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    info!("Picking PublicDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::PublicDecryption(PublicDecryptionRequest {
                decryptionId: decryption_id,
                snsCtMaterials: sns_ct,
                extraData: vec![].into(),
            }),
        }]
    );
    info!("Data OK!");
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

    info!("Triggering Postgres notification with UserDecryptionRequest insertion...");
    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
        vec![],
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    info!("Picking UserDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking UserDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::UserDecryption(UserDecryptionRequest {
                decryptionId: decryption_id,
                snsCtMaterials: sns_ct,
                userAddress: user_address,
                publicKey: public_key.into(),
                extraData: vec![].into(),
            })
        }]
    );
    info!("Data OK!");
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

    info!("Triggering Postgres notification with PrepKeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO prep_keygen_requests(prep_keygen_id, epoch_id, params_type, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        prep_keygen_request_id.as_le_slice(),
        epoch_id.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    info!("Picking PrepKeygenRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking PrepKeygenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::PrepKeygen(PrepKeygenRequest {
                prepKeygenId: prep_keygen_request_id,
                epochId: epoch_id,
                paramsType: params_type as u8,
            })
        }]
    );
    info!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_keygen() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let mut event_picker =
        DbEventPicker::connect(test_instance.db().clone(), &Config::default()).await?;

    let prep_key_id = rand_u256();
    let key_id = rand_u256();

    info!("Triggering Postgres notification with KeygenRequest insertion...");
    sqlx::query!(
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, otlp_context) \
        VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    info!("Picking KeygenRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking KeygenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::Keygen(KeygenRequest {
                prepKeygenId: prep_key_id,
                keyId: key_id,
            }),
        }]
    );
    info!("Data OK!");
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

    info!("Triggering Postgres notification with CrsgenRequest insertion...");
    sqlx::query!(
        "INSERT INTO crsgen_requests(crs_id, max_bit_length, params_type, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    info!("Picking CrsgenRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking CrsgenRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::Crsgen(CrsgenRequest {
                crsId: crs_id,
                maxBitLength: max_bit_length,
                paramsType: params_type as u8,
            })
        }]
    );
    info!("Data OK!");
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
    info!("Inserting PublicDecryptionRequest before starting the event picker...");
    sqlx::query!(
        "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        vec![],
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(test_instance.db())
    .await?;

    let config = Config {
        database_polling_timeout: Duration::from_millis(500),
        ..Default::default()
    };
    let mut event_picker = DbEventPicker::connect(test_instance.db().clone(), &config).await?;

    info!("Picking PublicDecryptionRequest...");
    let events = event_picker.pick_events().await?;

    info!("Checking PublicDecryptionRequest data...");
    assert_eq!(
        events,
        vec![GatewayEvent {
            otlp_context: PropagationContext::empty(),
            already_sent: false,
            kind: GatewayEventKind::PublicDecryption(PublicDecryptionRequest {
                decryptionId: decryption_id,
                snsCtMaterials: sns_ct,
                extraData: vec![].into(),
            })
        }]
    );
    info!("Data OK!");
    Ok(())
}
