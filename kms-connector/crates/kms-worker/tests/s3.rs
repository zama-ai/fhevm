use alloy::{
    hex,
    providers::{ProviderBuilder, mock::Asserter},
    transports::http::reqwest,
};
use anyhow::anyhow;
use connector_utils::tests::{
    rand::rand_u256,
    setup::{S3_CT_DIGEST, S3_CT_HANDLE, S3Instance, TestInstance},
};
use fhevm_gateway_bindings::decryption::Decryption::SnsCiphertextMaterial;
use kms_worker::core::{Config, event_processor::s3::S3Service};

#[tokio::test]
async fn test_get_ciphertext_from_s3() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let config = Config::default();
    let mock_provider = ProviderBuilder::new().connect_mocked_client(Asserter::new());
    let s3_client = reqwest::Client::builder()
        .connect_timeout(config.s3_connect_timeout)
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let sns_ct = SnsCiphertextMaterial {
        ctHandle: <[u8; 32]>::try_from(hex::decode(S3_CT_HANDLE)?)
            .unwrap()
            .into(),
        snsCiphertextDigest: <[u8; 32]>::try_from(hex::decode(S3_CT_DIGEST)?)
            .unwrap()
            .into(),
        ..Default::default()
    };
    let s3_service = S3Service::new(&config, mock_provider, s3_client);
    s3_service
        .retrieve_s3_ciphertext(&bucket_url, &sns_ct, S3_CT_DIGEST)
        .await
        .unwrap();

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
    let config = Config::default();
    let mock_provider = ProviderBuilder::new().connect_mocked_client(Asserter::new());
    let s3_client = reqwest::Client::builder()
        .connect_timeout(config.s3_connect_timeout)
        .build()
        .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))?;

    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let sns_ct = SnsCiphertextMaterial {
        ctHandle: rand_u256().into(),
        snsCiphertextDigest: <[u8; 32]>::try_from(hex::decode(S3_CT_UNSTORED)?)
            .unwrap()
            .into(),
        ..Default::default()
    };
    let s3_service = S3Service::new(&config, mock_provider, s3_client);
    if let Ok(ct) = s3_service
        .retrieve_s3_ciphertext(&bucket_url, &sns_ct, S3_CT_UNSTORED)
        .await
    {
        panic!("Unexpected ciphertext retrievd {ct:?}");
    }

    Ok(())
}
