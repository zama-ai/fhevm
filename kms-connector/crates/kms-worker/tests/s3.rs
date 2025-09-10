use alloy::{
    hex,
    providers::{ProviderBuilder, mock::Asserter},
};
use connector_utils::tests::{
    rand::rand_u256,
    setup::{S3_CT, S3Instance, TestInstance},
};
use kms_worker::core::{Config, event_processor::s3::S3Service};

#[tokio::test]
async fn test_get_ciphertext_from_s3() -> anyhow::Result<()> {
    let test_instance = TestInstance::builder()
        .with_s3(S3Instance::setup().await?)
        .build();
    let config = Config::default();
    let mock_provider = ProviderBuilder::new().connect_mocked_client(Asserter::new());

    let handle = rand_u256().to_be_bytes_vec(); // dummy handle
    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let s3_service = S3Service::new(&config, mock_provider);
    s3_service
        .retrieve_s3_ciphertext_with_retry(vec![bucket_url], &handle, &hex::decode(S3_CT)?, S3_CT)
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

    let handle = rand_u256().to_be_bytes_vec(); // dummy handle
    let bucket_url = format!("{}/ct128", test_instance.s3_url());
    let s3_service = S3Service::new(&config, mock_provider);
    if let Some(ct) = s3_service
        .retrieve_s3_ciphertext_with_retry(
            vec![bucket_url],
            &handle,
            &hex::decode(S3_CT_UNSTORED)?,
            S3_CT_UNSTORED,
        )
        .await
    {
        panic!("Unexpected ciphertext retrievd {ct:?}");
    }

    Ok(())
}
