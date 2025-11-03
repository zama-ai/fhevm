mod notif;
mod parallel;
mod polling;

use alloy::primitives::U256;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::rand::{rand_address, rand_public_key, rand_sns_ct, rand_u256},
    types::{
        GatewayEventKind,
        db::{ParamsTypeDb, SnsCiphertextMaterialDbItem},
        gw_event::PRSS_INIT_ID,
    },
};
use fhevm_gateway_bindings::{
    decryption::Decryption::{PublicDecryptionRequest, UserDecryptionRequest},
    kms_generation::KMSGeneration::{
        CrsgenRequest, KeyReshareSameSet, KeygenRequest, PRSSInit, PrepKeygenRequest,
    },
};
use sqlx::{Pool, Postgres};

pub async fn insert_rand_request(
    db: &Pool<Postgres>,
    request_str: &str,
    id: Option<U256>,
) -> anyhow::Result<GatewayEventKind> {
    let inserted_response = match request_str {
        "PublicDecryptionRequest" => insert_rand_public_decryption_request(db, id).await?.into(),
        "UserDecryptionRequest" => insert_rand_user_decryption_request(db, id).await?.into(),
        "PrepKeygenRequest" => insert_rand_prep_keygen_request(db, id).await?.into(),
        "KeygenRequest" => insert_rand_keygen_request(db, id).await?.into(),
        "CrsgenRequest" => insert_rand_crsgen_request(db, id).await?.into(),
        "PrssInit" => insert_rand_prss_init(db, id).await?.into(),
        "KeyReshareSameSet" => insert_rand_key_reshare_same_set(db, id).await?.into(),
        s => panic!("Unexpected response kind: {s}"),
    };
    Ok(inserted_response)
}

async fn insert_rand_public_decryption_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<PublicDecryptionRequest> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let sns_ct = vec![rand_sns_ct()];
    let extra_data = vec![];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO public_decryption_requests(decryption_id, sns_ct_materials, extra_data, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        extra_data.clone(),
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(PublicDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: sns_ct,
        extraData: extra_data.into(),
    })
}

async fn insert_rand_user_decryption_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<UserDecryptionRequest> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let sns_ct = vec![rand_sns_ct()];
    let user_address = rand_address();
    let public_key = rand_public_key();
    let extra_data = vec![];
    let sns_ciphertexts_db = sns_ct
        .iter()
        .map(SnsCiphertextMaterialDbItem::from)
        .collect::<Vec<SnsCiphertextMaterialDbItem>>();

    sqlx::query!(
        "INSERT INTO user_decryption_requests(\
            decryption_id, sns_ct_materials, user_address, public_key, extra_data, otlp_context\
        ) \
        VALUES ($1, $2, $3, $4, $5, $6) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        sns_ciphertexts_db as Vec<SnsCiphertextMaterialDbItem>,
        user_address.as_slice(),
        &public_key,
        extra_data.clone(),
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(UserDecryptionRequest {
        decryptionId: decryption_id,
        snsCtMaterials: sns_ct,
        userAddress: user_address,
        publicKey: public_key.into(),
        extraData: extra_data.into(),
    })
}

async fn insert_rand_prep_keygen_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<PrepKeygenRequest> {
    let prep_keygen_request_id = id.unwrap_or_else(rand_u256);
    let epoch_id = rand_u256();
    let params_type = ParamsTypeDb::Test;

    sqlx::query!(
        "INSERT INTO prep_keygen_requests(prep_keygen_id, epoch_id, params_type, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        prep_keygen_request_id.as_le_slice(),
        epoch_id.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(PrepKeygenRequest {
        prepKeygenId: prep_keygen_request_id,
        epochId: epoch_id,
        paramsType: params_type as u8,
    })
}

async fn insert_rand_keygen_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<KeygenRequest> {
    let key_id = id.unwrap_or_else(rand_u256);
    let prep_key_id = rand_u256();

    sqlx::query!(
        "INSERT INTO keygen_requests(prep_keygen_id, key_id, otlp_context) \
        VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        prep_key_id.as_le_slice(),
        key_id.as_le_slice(),
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(KeygenRequest {
        prepKeygenId: prep_key_id,
        keyId: key_id,
    })
}

async fn insert_rand_crsgen_request(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<CrsgenRequest> {
    let crs_id = id.unwrap_or_else(rand_u256);
    let max_bit_length = rand_u256();
    let params_type = ParamsTypeDb::Test;

    sqlx::query!(
        "INSERT INTO crsgen_requests(crs_id, max_bit_length, params_type, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        max_bit_length.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(CrsgenRequest {
        crsId: crs_id,
        maxBitLength: max_bit_length,
        paramsType: params_type as u8,
    })
}

async fn insert_rand_prss_init(db: &Pool<Postgres>, id: Option<U256>) -> anyhow::Result<PRSSInit> {
    let prss_init_id = id.unwrap_or(PRSS_INIT_ID);
    sqlx::query!(
        "INSERT INTO prss_init(id, otlp_context) VALUES ($1, $2) ON CONFLICT DO NOTHING",
        prss_init_id.as_le_slice(),
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(PRSSInit)
}

async fn insert_rand_key_reshare_same_set(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<KeyReshareSameSet> {
    let key_id = id.unwrap_or_else(rand_u256);
    let prep_keygen_id = rand_u256();
    let key_reshare_id = rand_u256();
    let params_type = ParamsTypeDb::Test;

    sqlx::query!(
        "INSERT INTO key_reshare_same_set(prep_keygen_id, key_id, key_reshare_id, params_type, otlp_context) \
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        key_id.as_le_slice(),
        key_reshare_id.as_le_slice(),
        params_type as ParamsTypeDb,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(KeyReshareSameSet {
        prepKeygenId: prep_keygen_id,
        keyId: key_id,
        keyReshareId: key_reshare_id,
        paramsType: params_type as u8,
    })
}

async fn check_db_empty(db: &Pool<Postgres>, request_str: &str) -> sqlx::Result<()> {
    let query = match request_str {
        "PublicDecryptionRequest" => "SELECT COUNT(decryption_id) FROM public_decryption_requests",
        "UserDecryptionRequest" => "SELECT COUNT(decryption_id) FROM user_decryption_requests",
        "PrepKeygenRequest" => "SELECT COUNT(prep_keygen_id) FROM prep_keygen_requests",
        "KeygenRequest" => "SELECT COUNT(key_id) FROM keygen_requests",
        "CrsgenRequest" => "SELECT COUNT(crs_id) FROM crsgen_requests",
        "PrssInit" => "SELECT COUNT(id) FROM prss_init",
        "KeyReshareSameSet" => "SELECT COUNT(key_id) FROM key_reshare_same_set",
        s => panic!("Unexpected request kind: {s}"),
    };
    let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
    assert_eq!(count, 0);
    Ok(())
}
