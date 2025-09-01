use std::collections::HashMap;

use alloy::{hex, primitives::U256};
use connector_utils::{
    tests::{
        rand::{rand_digest, rand_signature, rand_u256},
        setup::TestInstanceBuilder,
    },
    types::{
        KmsGrpcResponse, KmsResponse,
        db::{KeyDigestDbItem, KeyType},
    },
};
use kms_grpc::kms::v1::{
    KeyGenPreprocResult, KeyGenResult, PublicDecryptionResponse, PublicDecryptionResponsePayload,
    RequestId, UserDecryptionResponse, UserDecryptionResponsePayload,
};
use kms_worker::core::{DbKmsResponsePublisher, KmsResponsePublisher};
use sqlx::Row;
use tracing::info;

#[tokio::test]
async fn test_publish_public_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking PublicDecryptionResponse from KMS Core...");
    let rand_decryption_id = rand_u256();
    let rand_signature = rand_signature();
    let grpc_response = KmsGrpcResponse::PublicDecryption {
        decryption_id: rand_decryption_id,
        grpc_response: PublicDecryptionResponse {
            signature: rand_signature.clone(),
            external_signature: rand_signature.clone(),
            payload: Some(PublicDecryptionResponsePayload::default()),
            extra_data: vec![],
        },
    };
    let response = KmsResponse::process(grpc_response)?;

    publisher.publish(response).await?;
    info!("PublicDecryptionResponse successfully published!");

    info!("Checking PublicDecryptionResponse is stored in DB...");
    let row = sqlx::query(
        "SELECT decryption_id, decrypted_result, signature FROM public_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(decryption_id, rand_decryption_id);
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_user_decryption_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking UserDecryptionResponse from KMS Core...");
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
    info!("UserDecryptionResponse successfully published!");

    info!("Checking UserDecryptionResponse is stored in DB...");
    let row = sqlx::query(
        "SELECT decryption_id, user_decrypted_shares, signature FROM user_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;

    let decryption_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?);
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(decryption_id, rand_decryption_id);
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_prep_keygen_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking PrepKeygenResponse from KMS Core...");
    let rand_prep_keygen_id = rand_u256();
    let rand_signature = rand_signature();
    let grpc_response = KmsGrpcResponse::PrepKeygen(KeyGenPreprocResult {
        preprocessing_id: Some(RequestId {
            request_id: hex::encode(rand_prep_keygen_id.to_be_bytes_vec()),
        }),
        external_signature: rand_signature.clone(),
    });
    let response = KmsResponse::process(grpc_response)?;

    publisher.publish(response).await?;
    info!("PrepKeygenResponse successfully published!");

    info!("Checking PrepKeygenResponse is stored in DB...");
    let row = sqlx::query("SELECT prep_keygen_id, signature FROM prep_keygen_responses")
        .fetch_one(test_instance.db())
        .await?;

    let prep_keygen_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("prep_keygen_id")?);
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(prep_keygen_id, rand_prep_keygen_id);
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_keygen_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking KeygenResponse from KMS Core...");
    let rand_key_id = rand_u256();
    let rand_prep_keygen_id = rand_u256();
    let rand_signature = rand_signature();
    let mut rand_key_digests = HashMap::new();
    rand_key_digests.insert(String::from("server"), rand_digest().to_vec());
    rand_key_digests.insert(String::from("public"), rand_digest().to_vec());

    let grpc_response = KmsGrpcResponse::Keygen(KeyGenResult {
        request_id: Some(RequestId {
            request_id: hex::encode(rand_key_id.to_be_bytes_vec()),
        }),
        external_signature: rand_signature.clone(),
        preprocessing_id: Some(RequestId {
            request_id: hex::encode(rand_prep_keygen_id.to_be_bytes_vec()),
        }),
        key_digests: rand_key_digests.clone(),
    });
    let response = KmsResponse::process(grpc_response)?;

    publisher.publish(response).await?;
    info!("KeygenResponse successfully published!");

    info!("Checking KeygenResponse is stored in DB...");
    let row = sqlx::query("SELECT key_id, key_digests, signature FROM keygen_responses")
        .fetch_one(test_instance.db())
        .await?;

    let key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?);
    let key_digests = row.try_get::<Vec<KeyDigestDbItem>, _>("key_digests")?;
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(key_id, rand_key_id);
    for kd in key_digests {
        let key_type_str = match kd.key_type {
            KeyType::Public => "public",
            KeyType::Server => "server",
        };
        assert_eq!(Some(&kd.digest), rand_key_digests.get(key_type_str));
    }
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}
