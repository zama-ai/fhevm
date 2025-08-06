use alloy::primitives::U256;
use connector_utils::{
    tests::{
        rand::{rand_signature, rand_u256},
        setup::TestInstanceBuilder,
    },
    types::{KmsGrpcResponse, KmsResponse},
};
use kms_grpc::kms::v1::{
    PublicDecryptionResponse, PublicDecryptionResponsePayload, UserDecryptionResponse,
    UserDecryptionResponsePayload,
};
use kms_worker::core::{DbKmsResponsePublisher, KmsResponsePublisher};
use sqlx::Row;

#[tokio::test]
async fn test_publish_public_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    println!("Mocking PublicDecryptionResponse from KMS Core...");
    let rand_decryption_id = rand_u256();
    let rand_signature = rand_signature();
    let grpc_response = KmsGrpcResponse::PublicDecryption {
        decryption_id: rand_decryption_id,
        grpc_response: PublicDecryptionResponse {
            signature: rand_signature.clone(),
            external_signature: Some(rand_signature.clone()),
            payload: Some(PublicDecryptionResponsePayload::default()),
            extra_data: vec![],
        },
    };
    let response = KmsResponse::process(grpc_response)?;

    publisher.publish(response).await?;
    println!("PublicDecryptionResponse successfully published!");

    println!("Checking PublicDecryptionResponse is stored in DB...");
    let row = sqlx::query(
        "SELECT decryption_id, decrypted_result, signature FROM public_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(decryption_id, rand_decryption_id);
    assert_eq!(signature, rand_signature);
    println!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_user_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;

    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    println!("Mocking UserDecryptionResponse from KMS Core...");
    let rand_decryption_id = rand_u256();
    let rand_signature = rand_signature();
    let grpc_response = KmsGrpcResponse::UserDecryption {
        decryption_id: rand_decryption_id,
        grpc_response: UserDecryptionResponse {
            signature: rand_signature.clone(),
            external_signature: rand_signature.clone(),
            payload: Some(UserDecryptionResponsePayload::default()),
            extra_data: vec![],
        },
    };
    let response = KmsResponse::process(grpc_response)?;

    publisher.publish(response).await?;
    println!("UserDecryptionResponse successfully published!");

    println!("Checking UserDecryptionResponse is stored in DB...");
    let row = sqlx::query(
        "SELECT decryption_id, user_decrypted_shares, signature FROM user_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(decryption_id, rand_decryption_id);
    assert_eq!(signature, rand_signature);
    println!("Response successfully stored!");
    Ok(())
}
