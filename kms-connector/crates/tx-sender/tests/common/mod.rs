#![allow(dead_code)]

use alloy::primitives::U256;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::rand::{rand_digest, rand_signature, rand_u256},
    types::{
        CrsgenResponse, KeygenResponse, KmsResponseKind, PrepKeygenResponse,
        PublicDecryptionResponse, UserDecryptionResponse,
        db::{KeyDigestDbItem, KeyType},
    },
};
use sqlx::{Pool, Postgres};

pub async fn insert_rand_response(
    db: &Pool<Postgres>,
    response_str: &str,
    id: Option<U256>,
) -> anyhow::Result<KmsResponseKind> {
    let inserted_response = match response_str {
        "PublicDecryptionResponse" => {
            KmsResponseKind::PublicDecryption(insert_rand_public_decrypt_response(db, id).await?)
        }
        "UserDecryptionResponse" => {
            KmsResponseKind::UserDecryption(insert_rand_user_decrypt_response(db, id).await?)
        }
        "PrepKeygenResponse" => {
            KmsResponseKind::PrepKeygen(insert_rand_prep_keygen_response(db, id).await?)
        }
        "KeygenResponse" => KmsResponseKind::Keygen(insert_rand_keygen_response(db, id).await?),
        "CrsgenResponse" => KmsResponseKind::Crsgen(insert_rand_crsgen_response(db, id).await?),
        s => panic!("Unexpected response kind: {s}"),
    };
    Ok(inserted_response)
}

pub async fn insert_rand_public_decrypt_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<PublicDecryptionResponse> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let decrypted_result = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO public_decryption_responses(decryption_id, decrypted_result, signature, extra_data, otlp_context) \
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        decrypted_result,
        signature,
        vec![],
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(PublicDecryptionResponse {
        decryption_id,
        decrypted_result,
        signature,
        extra_data: vec![],
    })
}

pub async fn insert_rand_user_decrypt_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<UserDecryptionResponse> {
    let decryption_id = id.unwrap_or_else(rand_u256);
    let user_decrypted_shares = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO user_decryption_responses(decryption_id, user_decrypted_shares, signature, extra_data, otlp_context) \
        VALUES ($1, $2, $3, $4, $5) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        user_decrypted_shares,
        signature,
        vec![],
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(UserDecryptionResponse {
        decryption_id,
        user_decrypted_shares,
        signature,
        extra_data: vec![],
    })
}

pub async fn insert_rand_prep_keygen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<PrepKeygenResponse> {
    let prep_keygen_id = id.unwrap_or_else(rand_u256);
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO prep_keygen_responses(prep_keygen_id, signature, otlp_context) \
        VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        signature,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(PrepKeygenResponse {
        prep_keygen_id,
        signature,
    })
}

pub async fn insert_rand_keygen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<KeygenResponse> {
    let key_id = id.unwrap_or_else(rand_u256);
    let key_digests = vec![KeyDigestDbItem {
        key_type: KeyType::Public,
        digest: rand_digest().to_vec(),
    }];
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO keygen_responses(key_id, key_digests, signature, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        key_id.as_le_slice(),
        key_digests.clone() as Vec<KeyDigestDbItem>,
        signature,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(KeygenResponse {
        key_id,
        key_digests,
        signature,
    })
}

pub async fn insert_rand_crsgen_response(
    db: &Pool<Postgres>,
    id: Option<U256>,
) -> anyhow::Result<CrsgenResponse> {
    let crs_id = id.unwrap_or_else(rand_u256);
    let crs_digest = rand_digest().to_vec();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO crsgen_responses(crs_id, crs_digest, signature, otlp_context) \
        VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        crs_id.as_le_slice(),
        crs_digest.clone(),
        signature,
        bc2wrap::serialize(&PropagationContext::empty())?,
    )
    .execute(db)
    .await?;

    Ok(CrsgenResponse {
        crs_id,
        crs_digest,
        signature,
    })
}
