use alloy::{hex, primitives::B256, transports::http::reqwest};
use anyhow::anyhow;
use ciphertext_attestation::{CiphertextFormat, consensus::ConsensusMaterial};
use connector_utils::tests::setup::{
    S3_CT_BUCKET, S3_CT_DIGEST, S3_CT_HANDLE, S3_CT_KEY_ID, S3Instance, TestInstance,
};
use kms_grpc::kms::v1::CiphertextFormat as GrpcCiphertextFormat;
use kms_worker::core::{
    Config,
    event_processor::{ProcessingError, ciphertext::s3::retrieve_verified_ciphertext},
};

fn s3_http_client() -> anyhow::Result<reqwest::Client> {
    let config = Config::default();
    reqwest::Client::builder()
        .connect_timeout(config.s3_connect_timeout)
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
}

fn stored_handle() -> anyhow::Result<B256> {
    Ok(B256::from_slice(&hex::decode(S3_CT_HANDLE)?))
}

/// The attested material of the test ciphertext, as it would be resolved from the winning
/// consensus group (see `connector_utils::tests::setup::s3`).
fn stored_material() -> anyhow::Result<ConsensusMaterial> {
    Ok(ConsensusMaterial {
        key_id: S3_CT_KEY_ID,
        ciphertext_digest: B256::ZERO, // regular ciphertext digest, unused by SNS retrieval
        sns_ciphertext_digest: B256::from_slice(&hex::decode(S3_CT_DIGEST)?),
        format: CiphertextFormat::CompressedOnCpu,
    })
}

#[tokio::test]
async fn test_get_ciphertext_from_winning_bucket() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    let bucket_url = format!("{}/{S3_CT_BUCKET}", test_instance.s3_url());
    let ct = retrieve_verified_ciphertext(
        &s3_client,
        stored_handle()?,
        &stored_material()?,
        &[bucket_url],
        Config::default().s3_ciphertext_retrieval_retries,
    )
    .await
    .unwrap();

    // The format is read from the attested material (`compressed_on_cpu`).
    assert_eq!(
        ct.ciphertext_format,
        GrpcCiphertextFormat::BigCompressed as i32
    );

    Ok(())
}

#[tokio::test]
async fn test_get_ciphertext_rejects_digest_mismatch() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    let bucket_url = format!("{}/{S3_CT_BUCKET}", test_instance.s3_url());
    // The object is served, but its bytes do not match the attested digest: the copy is rejected.
    let mut material = stored_material()?;
    material.sns_ciphertext_digest = B256::repeat_byte(0xAB);

    let err = retrieve_verified_ciphertext(
        &s3_client,
        stored_handle()?,
        &material,
        &[bucket_url],
        Config::default().s3_ciphertext_retrieval_retries,
    )
    .await
    .unwrap_err();

    assert!(matches!(err, ProcessingError::Recoverable(_)));

    Ok(())
}

#[tokio::test]
async fn test_get_unstored_ciphertext_is_unavailable() -> anyhow::Result<()> {
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .finish();
    let _tracing_guard = tracing::subscriber::set_default(subscriber);

    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    let bucket_url = format!("{}/{S3_CT_BUCKET}", test_instance.s3_url());
    // A handle with no stored object: every winning-group bucket returns a not-found.
    let unstored_handle = B256::repeat_byte(0x02);

    let err = retrieve_verified_ciphertext(
        &s3_client,
        unstored_handle,
        &stored_material()?,
        &[bucket_url],
        Config::default().s3_ciphertext_retrieval_retries,
    )
    .await
    .unwrap_err();

    // Unavailability is retryable: it surfaces as a recoverable error.
    assert!(matches!(err, ProcessingError::Recoverable(_)));

    Ok(())
}
