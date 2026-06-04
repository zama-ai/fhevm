use alloy::{hex, transports::http::reqwest};
use anyhow::anyhow;
use connector_utils::tests::{
    rand::rand_u256,
    setup::{S3_CT_DIGEST, S3_CT_HANDLE, S3_CT_RFC023_BUCKET, S3Instance, TestInstance},
};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use kms_grpc::kms::v1::CiphertextFormat;
use kms_worker::core::{Config, event_processor::ciphertext::s3::retrieve_s3_ciphertext};

fn s3_http_client() -> anyhow::Result<reqwest::Client> {
    let config = Config::default();
    reqwest::Client::builder()
        .connect_timeout(config.s3_connect_timeout)
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
}

fn stored_sns_ct() -> anyhow::Result<SnsCiphertextMaterial> {
    Ok(SnsCiphertextMaterial {
        ctHandle: <[u8; 32]>::try_from(hex::decode(S3_CT_HANDLE)?)
            .unwrap()
            .into(),
        snsCiphertextDigest: <[u8; 32]>::try_from(hex::decode(S3_CT_DIGEST)?)
            .unwrap()
            .into(),
        ..Default::default()
    })
}

#[tokio::test]
async fn test_get_ciphertext_from_s3_rfc023_url_format() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    // This bucket only stores the ciphertext under the RFC-023 layout (`{handle}/{context_id}`),
    // so a successful retrieval cannot have gone through the old-URL fallback.
    let bucket_url = format!("{}/{S3_CT_RFC023_BUCKET}", test_instance.s3_url());
    let sns_ct = stored_sns_ct()?;
    let ct = retrieve_s3_ciphertext(&s3_client, &bucket_url, &sns_ct, S3_CT_DIGEST)
        .await
        .unwrap();

    // The format is extracted from the RFC-023 attestation metadata (`compressed_on_cpu`)
    assert_eq!(ct.ciphertext_format, CiphertextFormat::BigCompressed as i32);

    Ok(())
}

#[tokio::test]
async fn test_get_ciphertext_from_s3_old_url_format() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let sns_ct = stored_sns_ct()?;
    let ct = retrieve_s3_ciphertext(&s3_client, &bucket_url, &sns_ct, S3_CT_DIGEST)
        .await
        .unwrap();

    // The format is extracted from the old `Ct-Format` metadata (`compressed_on_cpu`)
    assert_eq!(ct.ciphertext_format, CiphertextFormat::BigCompressed as i32);

    Ok(())
}

const S3_CT_UNSTORED: &str = "0222222222a10486971fbf81dcf64c1b2fc9965744d0c8f7da0e4b338f1a31a9";

#[tokio::test]
async fn test_get_unstored_s3_ciphertext() -> anyhow::Result<()> {
    // Initialize tracing for this test
    let subscriber = tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .with_test_writer()
        .finish();
    let _tracing_guard = tracing::subscriber::set_default(subscriber);

    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let s3_client = s3_http_client()?;

    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let sns_ct = SnsCiphertextMaterial {
        ctHandle: rand_u256().into(),
        snsCiphertextDigest: <[u8; 32]>::try_from(hex::decode(S3_CT_UNSTORED)?)
            .unwrap()
            .into(),
        ..Default::default()
    };
    if let Ok(ct) = retrieve_s3_ciphertext(&s3_client, &bucket_url, &sns_ct, S3_CT_UNSTORED).await {
        panic!("Unexpected ciphertext retrievd {ct:?}");
    }

    Ok(())
}
