use connector_utils::{
    tests::rand::{rand_signature, rand_u256},
    types::KmsResponse,
};
use sqlx::{Pool, Postgres};

pub async fn insert_rand_public_decrypt_response(
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsResponse> {
    let decryption_id = rand_u256();
    let decrypted_result = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO public_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        decrypted_result,
        signature
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::PublicDecryption {
        decryption_id,
        decrypted_result,
        signature,
    })
}

pub async fn insert_rand_user_decrypt_response(db: &Pool<Postgres>) -> anyhow::Result<KmsResponse> {
    let decryption_id = rand_u256();
    let user_decrypted_shares = rand_signature();
    let signature = rand_signature();

    sqlx::query!(
        "INSERT INTO user_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        user_decrypted_shares,
        signature
    )
    .execute(db)
    .await?;

    Ok(KmsResponse::UserDecryption {
        decryption_id,
        user_decrypted_shares,
        signature,
    })
}
