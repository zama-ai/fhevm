use alloy::hex;
use connector_tests::setup::{S3_CT, setup_test_s3_instance};
use kms_worker::core::event_processor::s3::retrieve_s3_ciphertext;

#[tokio::test]
async fn test_get_ciphertext_from_s3() -> anyhow::Result<()> {
    let minio_instance = setup_test_s3_instance().await?;
    let bucket_url = format!("{}/ct128", minio_instance.url);
    retrieve_s3_ciphertext(bucket_url, hex::decode(S3_CT)?).await?;
    Ok(())
}
