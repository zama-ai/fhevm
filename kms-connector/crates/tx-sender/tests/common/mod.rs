use connector_utils::{
    tests::rand::{rand_digest, rand_signature, rand_u256},
    types::{
        KeygenResponse, KmsResponse, PrepKeygenResponse, PublicDecryptionResponse,
        UserDecryptionResponse,
        db::{KeyDigestDbItem, KeyType},
    },
};
use sqlx::{Pool, Postgres};

pub async fn insert_rand_public_decrypt_response(
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsResponse> {
    let decryption_id = rand_u256();
    let decrypted_result = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO public_decryption_responses VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        decrypted_result,
        signature,
        vec![],
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::PublicDecryption(PublicDecryptionResponse {
        decryption_id,
        decrypted_result,
        signature,
        extra_data: vec![],
    }))
}

pub async fn insert_rand_user_decrypt_response(db: &Pool<Postgres>) -> anyhow::Result<KmsResponse> {
    let decryption_id = rand_u256();
    let user_decrypted_shares = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO user_decryption_responses VALUES ($1, $2, $3, $4) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        user_decrypted_shares,
        signature,
        vec![],
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::UserDecryption(UserDecryptionResponse {
        decryption_id,
        user_decrypted_shares,
        signature,
        extra_data: vec![],
    }))
}

pub async fn insert_rand_prep_keygen_response(db: &Pool<Postgres>) -> anyhow::Result<KmsResponse> {
    let prep_keygen_id = rand_u256();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO prep_keygen_responses VALUES ($1, $2) ON CONFLICT DO NOTHING",
        prep_keygen_id.as_le_slice(),
        signature,
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::PrepKeygen(PrepKeygenResponse {
        prep_keygen_id,
        signature,
    }))
}

pub async fn insert_rand_keygen_response(db: &Pool<Postgres>) -> anyhow::Result<KmsResponse> {
    let key_id = rand_u256();
    let key_digests = vec![KeyDigestDbItem {
        key_type: KeyType::Public,
        digest: rand_digest().to_vec(),
    }];
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO keygen_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        key_id.as_le_slice(),
        key_digests.clone() as Vec<KeyDigestDbItem>,
        signature,
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::Keygen(KeygenResponse {
        key_id,
        key_digests,
        signature,
    }))
}
