use connector_tests::{
    rand::{rand_signature, rand_u256},
    setup::test_instance_with_db_only,
};
use connector_utils::types::KmsResponse;
use tx_sender::core::{DbKmsResponsePicker, KmsResponsePicker};

#[tokio::test]
async fn test_pick_public_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db.clone()).await?;

    let decryption_id = rand_u256();
    let decrypted_result = rand_signature();
    let signature = rand_signature();

    println!("Triggering Postgres notification with PublicDecryptionResponse insertion...");
    sqlx::query!(
        "INSERT INTO public_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        decrypted_result,
        signature
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking PublicDecryptionResponse...");
    let response = response_picker.pick_response().await?;

    println!("Checking PublicDecryptionResponse data...");
    assert_eq!(
        response,
        KmsResponse::PublicDecryption {
            decryption_id,
            decrypted_result,
            signature,
        }
    );
    println!("Data OK!");
    Ok(())
}

#[tokio::test]
async fn test_pick_user_decryption() -> anyhow::Result<()> {
    let test_instance = test_instance_with_db_only().await?;

    let mut response_picker = DbKmsResponsePicker::connect(test_instance.db.clone()).await?;

    let decryption_id = rand_u256();
    let user_decrypted_shares = rand_signature();
    let signature = rand_signature();

    println!("Triggering Postgres notification with UserDecryptionResponse insertion...");
    sqlx::query!(
        "INSERT INTO user_decryption_responses VALUES ($1, $2, $3) ON CONFLICT DO NOTHING",
        decryption_id.as_le_slice(),
        user_decrypted_shares,
        signature
    )
    .execute(&test_instance.db)
    .await?;

    println!("Picking UserDecryptionResponse...");
    let response = response_picker.pick_response().await?;

    println!("Checking UserDecryptionResponse data...");
    assert_eq!(
        response,
        KmsResponse::UserDecryption {
            decryption_id,
            user_decrypted_shares,
            signature,
        }
    );
    println!("Data OK!");
    Ok(())
}
