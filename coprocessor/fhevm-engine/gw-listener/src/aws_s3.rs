use std::time::Duration;

use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::{config::Builder, Client};
use tokio_util::bytes;
use tracing::{error, info};

#[derive(Clone, Debug, Default)]
pub struct S3Policy {
    pub max_attempt: u32,
    pub max_backoff: Duration,
    pub max_retries_timeout: Duration,
    pub recheck_duration: Duration,
    pub regular_recheck_duration: Duration,
    pub connect_timeout: Duration,
}

impl S3Policy {
    const DEFAULT: Self = Self {
        max_attempt: 10,
        max_backoff: Duration::from_secs(20),
        max_retries_timeout: Duration::from_secs(300),
        recheck_duration: Duration::from_secs(10),
        regular_recheck_duration: Duration::from_secs(300),
        connect_timeout: Duration::from_secs(10),
    };
}

pub async fn create_s3_client(retry_policy: &S3Policy) -> aws_sdk_s3::Client {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(retry_policy.connect_timeout)
        .operation_attempt_timeout(retry_policy.max_retries_timeout)
        .build();

    let retry_config = RetryConfig::standard()
        .with_max_attempts(retry_policy.max_attempt)
        .with_max_backoff(retry_policy.max_backoff);

    let config = Builder::from(&sdk_config)
        .timeout_config(timeout_config)
        .retry_config(retry_config)
        .build();

    Client::from_conf(config)
}

pub async fn default_aws_s3_client() -> AwsS3Client {
    let s3_client = create_s3_client(&S3Policy::DEFAULT).await;
    AwsS3Client { s3_client }
}

// Let's wrap Aws Client to have an interface for it so we can mock it.
#[derive(Clone)]
pub struct AwsS3Client {
    pub s3_client: Client,
}

impl AwsS3Interface for AwsS3Client {
    async fn get_bucket_key(&self, bucket: &str, key: &str) -> anyhow::Result<bytes::Bytes> {
        let result = self
            .s3_client
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?;
        Ok(result.body.collect().await?.into_bytes())
    }
}

pub trait AwsS3Interface {
    fn get_bucket_key(
        &self,
        bucket: &str,
        key: &str,
    ) -> impl std::future::Future<Output = anyhow::Result<bytes::Bytes>>;
}

pub async fn download_key_from_s3<A: AwsS3Interface>(
    s3_client: &A,
    s3_bucket_urls: &[String],
    key_path: String,
    offset_bucket: usize, // to not ask the same bucket first
) -> anyhow::Result<bytes::Bytes> {
    let nb_urls = s3_bucket_urls.len();
    for i_s3_bucket_url in 0..nb_urls {
        // ask different order per key
        let url_index = (i_s3_bucket_url + offset_bucket) % s3_bucket_urls.len();
        let s3_bucket_url = &s3_bucket_urls[url_index];
        info!(
            key_path,
            i_s3_bucket_url, nb_urls, url_index, "Try downloading"
        );
        let result = s3_client.get_bucket_key(s3_bucket_url, &key_path).await;
        let Ok(result) = result else {
            error!(key_path, result = ?result, "Downloading failed");
            continue;
        };
        info!(key_path, "Downloaded");
        return Ok(result);
    }
    error!(key_path, "Failed to download key from all S3 buckets");
    anyhow::bail!("Failed to download key {key_path} from all S3 buckets");
}
