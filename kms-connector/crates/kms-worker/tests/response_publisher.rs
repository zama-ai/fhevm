use alloy::primitives::U256;
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::{
        db::requests::{InsertRequestOptions, TestEventType, insert_rand_request},
        rand::{rand_digest, rand_signature, rand_u256},
        setup::TestInstanceBuilder,
    },
    types::{
        KmsGrpcResponse, KmsResponse, KmsResponseKind,
        db::{KeyDigestDbItem, KeyType, OperationStatus, RequestSource},
        u256_to_request_id,
    },
};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    CrsGenResult, EpochResultResponse as GrpcEpochResultResponse, KeyDigest, KeyGenPreprocResult,
    KeyGenResult, PublicDecryptionResponse, PublicDecryptionResponsePayload,
    UserDecryptionResponse, UserDecryptionResponsePayload,
};
use kms_worker::core::{DbKmsResponsePublisher, KmsResponsePublisher};
use sqlx::Row;
use std::str::FromStr;
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
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
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
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
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
        preprocessing_id: Some(u256_to_request_id(rand_prep_keygen_id)),
        external_signature: rand_signature.clone(),
    });
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
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
    let rand_key_digests = vec![
        KeyDigest {
            key_type: String::from("ServerKey"),
            digest: rand_digest().to_vec(),
        },
        KeyDigest {
            key_type: String::from("PublicKey"),
            digest: rand_digest().to_vec(),
        },
    ];

    let grpc_response = KmsGrpcResponse::Keygen(KeyGenResult {
        request_id: Some(u256_to_request_id(rand_key_id)),
        external_signature: rand_signature.clone(),
        preprocessing_id: Some(u256_to_request_id(rand_prep_keygen_id)),
        key_digests: rand_key_digests.clone(),
    });
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
    info!("KeygenResponse successfully published!");

    info!("Checking KeygenResponse is stored in DB...");
    let row = sqlx::query("SELECT key_id, key_digests, signature FROM keygen_responses")
        .fetch_one(test_instance.db())
        .await?;

    let key_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("key_id")?);
    let key_digests = row.try_get::<Vec<KeyDigestDbItem>, _>("key_digests")?;
    let signature = row.try_get::<Vec<u8>, _>("signature")?;
    assert_eq!(key_id, rand_key_id);
    for (i, kd) in key_digests.iter().enumerate() {
        assert_eq!(
            kd.key_type,
            KeyType::from_str(&rand_key_digests[i].key_type)?
        );
        assert_eq!(kd.digest, rand_key_digests[i].digest);
    }
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_crsgen_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking CrsgenResponse from KMS Core...");
    let rand_crs_id = rand_u256();
    let rand_crs_digest = rand_digest().to_vec();
    let rand_signature = rand_signature();
    let grpc_response = KmsGrpcResponse::Crsgen(CrsGenResult {
        request_id: Some(u256_to_request_id(rand_crs_id)),
        crs_digest: rand_crs_digest.clone(),
        external_signature: rand_signature.clone(),
        max_num_bits: 256,
    });
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
    info!("CrsgenResponse successfully published!");

    info!("Checking CrsgenResponse is stored in DB...");
    let row = sqlx::query("SELECT crs_id, crs_digest, signature FROM crsgen_responses")
        .fetch_one(test_instance.db())
        .await?;

    let crs_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("crs_id")?);
    let crs_digest = row.try_get::<Vec<u8>, _>("crs_digest")?;
    let signature = row.try_get::<Vec<u8>, _>("signature")?;

    assert_eq!(crs_id, rand_crs_id);
    assert_eq!(crs_digest, rand_crs_digest);
    assert_eq!(signature, rand_signature);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_new_kms_context_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking NewKmsContextResponse from KMS Core...");
    let rand_context_id = rand_u256();
    let grpc_response = KmsGrpcResponse::NewKmsContext {
        context_id: rand_context_id,
    };
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
    info!("NewKmsContextResponse successfully published!");

    info!("Checking NewKmsContextResponse is stored in DB...");
    let row = sqlx::query("SELECT context_id FROM new_kms_context_responses")
        .fetch_one(test_instance.db())
        .await?;

    let context_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?);
    assert_eq!(context_id, rand_context_id);
    info!("Response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_epoch_result_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Mocking EpochResultResponse from KMS Core...");
    let rand_context_id = rand_u256();
    let rand_epoch_id = rand_u256();
    let grpc_response = KmsGrpcResponse::EpochResult {
        context_id: rand_context_id,
        epoch_id: rand_epoch_id,
        grpc_response: GrpcEpochResultResponse::default(),
    };
    let response = KmsResponse::new(
        KmsResponseKind::process(grpc_response)?,
        PropagationContext::empty(),
        RequestSource::OnChain,
    );

    publisher.publish_response(response).await?;
    info!("EpochResultResponse successfully published!");

    info!("Checking EpochResultResponse is stored in DB...");
    let row =
        sqlx::query("SELECT context_id, epoch_id, keys, crs_list FROM epoch_result_responses")
            .fetch_one(test_instance.db())
            .await?;

    let context_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("context_id")?);
    let epoch_id = U256::from_le_bytes(row.try_get::<[u8; 32], _>("epoch_id")?);
    assert_eq!(context_id, rand_context_id);
    assert_eq!(epoch_id, rand_epoch_id);
    info!("Response successfully stored!");
    Ok(())
}

/// Mocks a public decryption payload response from the KMS Core for the given id and source.
fn mock_public_decryption_response(decryption_id: U256, source: RequestSource) -> KmsResponse {
    let grpc_response = KmsGrpcResponse::PublicDecryption {
        decryption_id,
        grpc_response: PublicDecryptionResponse {
            signature: rand_signature(),
            external_signature: rand_signature(),
            payload: Some(PublicDecryptionResponsePayload::default()),
            extra_data: vec![],
        },
    };
    KmsResponse::new(
        KmsResponseKind::process(grpc_response).unwrap(),
        PropagationContext::empty(),
        source,
    )
}

/// Mocks a user decryption payload response from the KMS Core for the given id and source.
fn mock_user_decryption_response(decryption_id: U256, source: RequestSource) -> KmsResponse {
    let grpc_response = KmsGrpcResponse::UserDecryption {
        decryption_id,
        grpc_response: UserDecryptionResponse {
            signature: rand_signature(),
            external_signature: rand_signature(),
            payload: Some(UserDecryptionResponsePayload::default()),
            extra_data: vec![],
        },
    };
    KmsResponse::new(
        KmsResponseKind::process(grpc_response).unwrap(),
        PropagationContext::empty(),
        source,
    )
}

#[tokio::test]
async fn test_publish_public_decryption_error_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Inserting HTTP-sourced PublicDecryptionRequest...");
    let rand_decryption_id = rand_u256();
    insert_rand_request(
        test_instance.db(),
        TestEventType::PublicDecryption,
        InsertRequestOptions::default()
            .with_id(rand_decryption_id)
            .with_source(RequestSource::Http),
    )
    .await?;

    info!("Publishing error response for the request...");
    publisher
        .publish_public_decryption_error(
            rand_decryption_id,
            ErrorCode::AclDenied,
            "handles not allowed for public decryption",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the error response row...");
    let row = sqlx::query(
        "SELECT decryption_id, decrypted_result, signature, error_code, error_details, source,
        status FROM public_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(
        U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
        rand_decryption_id
    );
    assert_eq!(
        row.try_get::<Option<String>, _>("error_code")?.as_deref(),
        Some(ErrorCode::AclDenied.as_str())
    );
    assert_eq!(
        row.try_get::<Option<String>, _>("error_details")?
            .as_deref(),
        Some("handles not allowed for public decryption")
    );
    assert_eq!(row.try_get::<Option<Vec<u8>>, _>("decrypted_result")?, None);
    assert_eq!(row.try_get::<Option<Vec<u8>>, _>("signature")?, None);
    assert_eq!(
        row.try_get::<RequestSource, _>("source")?,
        RequestSource::Http
    );
    assert_eq!(
        row.try_get::<OperationStatus, _>("status")?,
        OperationStatus::Completed
    );

    info!("Checking the request row was marked as failed by the error publication...");
    let request_status = sqlx::query_scalar::<_, OperationStatus>(
        "SELECT status FROM public_decryption_requests WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(request_status, OperationStatus::Failed);
    info!("Error response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_publish_user_decryption_error_response() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Publishing error response for a user decryption request...");
    let rand_decryption_id = rand_u256();
    publisher
        .publish_user_decryption_error(
            rand_decryption_id,
            ErrorCode::UpstreamTransient,
            "KMS Core is unavailable",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the error response row...");
    let row = sqlx::query(
        "SELECT decryption_id, user_decrypted_shares, signature, error_code, error_details,
        source, status FROM user_decryption_responses",
    )
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(
        U256::from_le_bytes(row.try_get::<[u8; 32], _>("decryption_id")?),
        rand_decryption_id
    );
    assert_eq!(
        row.try_get::<Option<String>, _>("error_code")?.as_deref(),
        Some(ErrorCode::UpstreamTransient.as_str())
    );
    assert_eq!(
        row.try_get::<Option<Vec<u8>>, _>("user_decrypted_shares")?,
        None
    );
    assert_eq!(row.try_get::<Option<Vec<u8>>, _>("signature")?, None);
    assert_eq!(
        row.try_get::<RequestSource, _>("source")?,
        RequestSource::Http
    );
    info!("Error response successfully stored!");
    Ok(())
}

#[tokio::test]
async fn test_public_error_response_does_not_overwrite_payload() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Publishing payload response...");
    let rand_decryption_id = rand_u256();
    let response = mock_public_decryption_response(rand_decryption_id, RequestSource::Http);
    publisher.publish_response(response).await?;

    info!("Publishing error response for the same decryption_id...");
    publisher
        .publish_public_decryption_error(
            rand_decryption_id,
            ErrorCode::Unprocessable,
            "should not be stored",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the payload response row was preserved...");
    let row = sqlx::query(
        "SELECT decrypted_result, error_code FROM public_decryption_responses
        WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert!(
        row.try_get::<Option<Vec<u8>>, _>("decrypted_result")?
            .is_some()
    );
    assert_eq!(row.try_get::<Option<String>, _>("error_code")?, None);
    info!("Payload response preserved!");
    Ok(())
}

#[tokio::test]
async fn test_public_error_response_overrides_previous_error() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Publishing a first error response...");
    let rand_decryption_id = rand_u256();
    publisher
        .publish_public_decryption_error(
            rand_decryption_id,
            ErrorCode::UpstreamTransient,
            "KMS Core is unavailable",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Publishing a second error response for the same decryption_id...");
    publisher
        .publish_public_decryption_error(
            rand_decryption_id,
            ErrorCode::AclDenied,
            "handles not allowed for public decryption",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the error row was overridden by the retry...");
    let row = sqlx::query(
        "SELECT error_code, error_details FROM public_decryption_responses
        WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(
        row.try_get::<Option<String>, _>("error_code")?.as_deref(),
        Some(ErrorCode::AclDenied.as_str())
    );
    assert_eq!(
        row.try_get::<Option<String>, _>("error_details")?
            .as_deref(),
        Some("handles not allowed for public decryption")
    );
    info!("Error response successfully overridden!");
    Ok(())
}

#[tokio::test]
async fn test_public_payload_response_overrides_previous_error() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Inserting HTTP-sourced PublicDecryptionRequest...");
    let rand_decryption_id = rand_u256();
    insert_rand_request(
        test_instance.db(),
        TestEventType::PublicDecryption,
        InsertRequestOptions::default()
            .with_id(rand_decryption_id)
            .with_source(RequestSource::Http),
    )
    .await?;

    info!("Publishing error response for the request...");
    publisher
        .publish_public_decryption_error(
            rand_decryption_id,
            ErrorCode::UpstreamTransient,
            "KMS Core is unavailable",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Simulating a retry of the request...");
    sqlx::query(
        "UPDATE public_decryption_requests SET status = 'under_process' WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .execute(test_instance.db())
    .await?;

    info!("Publishing payload response for the same decryption_id...");
    let response = mock_public_decryption_response(rand_decryption_id, RequestSource::Http);
    publisher.publish_response(response).await?;

    info!("Checking the error row was overridden by the payload...");
    let row = sqlx::query(
        "SELECT decrypted_result, signature, error_code, error_details, source, status
        FROM public_decryption_responses WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert!(
        row.try_get::<Option<Vec<u8>>, _>("decrypted_result")?
            .is_some()
    );
    assert!(row.try_get::<Option<Vec<u8>>, _>("signature")?.is_some());
    assert_eq!(row.try_get::<Option<String>, _>("error_code")?, None);
    assert_eq!(row.try_get::<Option<String>, _>("error_details")?, None);
    assert_eq!(
        row.try_get::<RequestSource, _>("source")?,
        RequestSource::Http
    );
    // Terminal on insert: the tx-sender's `status = 'pending'` picking queries must never see it.
    assert_eq!(
        row.try_get::<OperationStatus, _>("status")?,
        OperationStatus::Completed
    );

    info!("Checking the request row was completed by the response override trigger...");
    let request_status = sqlx::query_scalar::<_, OperationStatus>(
        "SELECT status FROM public_decryption_requests WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(request_status, OperationStatus::Completed);
    info!("Payload response successfully overrode the error response!");
    Ok(())
}

#[tokio::test]
async fn test_user_error_response_does_not_overwrite_payload() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Publishing payload response...");
    let rand_decryption_id = rand_u256();
    let response = mock_user_decryption_response(rand_decryption_id, RequestSource::Http);
    publisher.publish_response(response).await?;

    info!("Publishing error response for the same decryption_id...");
    publisher
        .publish_user_decryption_error(
            rand_decryption_id,
            ErrorCode::Unprocessable,
            "should not be stored",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the payload response row was preserved...");
    let row = sqlx::query(
        "SELECT user_decrypted_shares, error_code FROM user_decryption_responses
        WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert!(
        row.try_get::<Option<Vec<u8>>, _>("user_decrypted_shares")?
            .is_some()
    );
    assert_eq!(row.try_get::<Option<String>, _>("error_code")?, None);
    info!("Payload response preserved!");
    Ok(())
}

#[tokio::test]
async fn test_user_error_response_overrides_previous_error() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Publishing a first error response...");
    let rand_decryption_id = rand_u256();
    publisher
        .publish_user_decryption_error(
            rand_decryption_id,
            ErrorCode::UpstreamTransient,
            "KMS Core is unavailable",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Publishing a second error response for the same decryption_id...");
    publisher
        .publish_user_decryption_error(
            rand_decryption_id,
            ErrorCode::AclDenied,
            "user not allowed to decrypt the handles",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Checking the error row was overridden by the retry...");
    let row = sqlx::query(
        "SELECT error_code, error_details FROM user_decryption_responses
        WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(
        row.try_get::<Option<String>, _>("error_code")?.as_deref(),
        Some(ErrorCode::AclDenied.as_str())
    );
    assert_eq!(
        row.try_get::<Option<String>, _>("error_details")?
            .as_deref(),
        Some("user not allowed to decrypt the handles")
    );
    info!("Error response successfully overridden!");
    Ok(())
}

#[tokio::test]
async fn test_user_payload_response_overrides_previous_error() -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::db_setup().await?;
    let publisher = DbKmsResponsePublisher::new(test_instance.db().clone());

    info!("Inserting HTTP-sourced UserDecryptionRequest...");
    let rand_decryption_id = rand_u256();
    insert_rand_request(
        test_instance.db(),
        TestEventType::UserDecryption,
        InsertRequestOptions::default()
            .with_id(rand_decryption_id)
            .with_source(RequestSource::Http),
    )
    .await?;

    info!("Publishing error response for the request...");
    publisher
        .publish_user_decryption_error(
            rand_decryption_id,
            ErrorCode::UpstreamTransient,
            "KMS Core is unavailable",
            &[],
            &PropagationContext::empty(),
        )
        .await?;

    info!("Simulating a retry of the request...");
    sqlx::query(
        "UPDATE user_decryption_requests SET status = 'under_process' WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .execute(test_instance.db())
    .await?;

    info!("Publishing payload response for the same decryption_id...");
    let response = mock_user_decryption_response(rand_decryption_id, RequestSource::Http);
    publisher.publish_response(response).await?;

    info!("Checking the error row was overridden by the payload...");
    let row = sqlx::query(
        "SELECT user_decrypted_shares, signature, error_code, error_details, source, status
        FROM user_decryption_responses WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert!(
        row.try_get::<Option<Vec<u8>>, _>("user_decrypted_shares")?
            .is_some()
    );
    assert!(row.try_get::<Option<Vec<u8>>, _>("signature")?.is_some());
    assert_eq!(row.try_get::<Option<String>, _>("error_code")?, None);
    assert_eq!(row.try_get::<Option<String>, _>("error_details")?, None);
    assert_eq!(
        row.try_get::<RequestSource, _>("source")?,
        RequestSource::Http
    );
    // Terminal on insert: the tx-sender's `status = 'pending'` picking queries must never see it.
    assert_eq!(
        row.try_get::<OperationStatus, _>("status")?,
        OperationStatus::Completed
    );

    info!("Checking the request row was completed by the response override trigger...");
    let request_status = sqlx::query_scalar::<_, OperationStatus>(
        "SELECT status FROM user_decryption_requests WHERE decryption_id = $1",
    )
    .bind(rand_decryption_id.as_le_slice())
    .fetch_one(test_instance.db())
    .await?;
    assert_eq!(request_status, OperationStatus::Completed);
    info!("Payload response successfully overrode the error response!");
    Ok(())
}
