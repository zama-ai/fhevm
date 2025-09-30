use std::time::Duration;

use async_trait::async_trait;
use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::config::{Builder, ProvideCredentials};
use aws_sdk_s3::Client;
use tokio_util::bytes;
use tracing::{error, info, warn};

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

pub async fn create_s3_client(
    retry_policy: &S3Policy,
    url: &str,
) -> anyhow::Result<aws_sdk_s3::Client> {
    let sdk_config = aws_config::load_defaults(BehaviorVersion::latest()).await;

    let credentials = sdk_config
        .credentials_provider()
        .ok_or(anyhow::Error::msg("s3 client: no credential provider"))?
        .provide_credentials()
        .await?;
    info!(access_key = %credentials.access_key_id(), "Loaded AWS credentials");

    let region = sdk_config.region();
    info!(region = ?region, "Using AWS region");

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
        .endpoint_url(url)
        .build();

    Ok(Client::from_conf(config))
}

// Let's wrap Aws access to have an interface for it so we can mock it.
#[derive(Clone)]
pub struct AwsS3Client {}

#[async_trait]
impl AwsS3Interface for AwsS3Client {
    async fn get_bucket_key(
        &self,
        url: &str,
        bucket: &str,
        key: &str,
    ) -> anyhow::Result<bytes::Bytes> {
        Ok(create_s3_client(&S3Policy::DEFAULT, url)
            .await?
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?
            .body
            .collect()
            .await?
            .into_bytes())
    }
}

#[async_trait]
pub trait AwsS3Interface: Send + Sync {
    async fn get_bucket_key(
        &self,
        url: &str,
        bucket: &str,
        key: &str,
    ) -> anyhow::Result<bytes::Bytes>;
}

fn split_url(s3_bucket_url: &String) -> anyhow::Result<(String, String)> {
    let parsed_url_and_bucket = url::Url::parse(s3_bucket_url)?;
    let bucket = parsed_url_and_bucket.path();
    let host = s3_bucket_url
        .replace(bucket, "")
        .trim_end_matches('/')
        .to_owned();
    let host = if host.contains("minio:9000") {
        // TODO: replace by docker configuration
        warn!(s3_bucket_url, "Using localhost for minio access");
        host.replace("minio:9000", "172.17.0.1:9000")
    } else {
        host.to_owned()
    };
    let bucket = bucket.trim_start_matches('/');
    info!(s3_bucket_url, host, bucket, "Parsed S3 url");
    Ok((host.to_owned(), bucket.to_owned()))
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
            s3_bucket_url, i_s3_bucket_url, nb_urls, url_index, "Try downloading"
        );
        let Ok((url, bucket)) = split_url(s3_bucket_url) else {
            error!(s3_bucket_url, "Failed to parse S3 url");
            continue;
        };
        let result = s3_client.get_bucket_key(&url, &bucket, &key_path).await;
        let Ok(result) = result else {
            error!(s3_bucket_url, key_path, result = ?result, "Downloading failed");
            continue;
        };
        info!(key_path, "Downloaded");
        return Ok(result);
    }
    error!(key_path, "Failed to download key from all S3 buckets");
    anyhow::bail!("Failed to download key {key_path} from all S3 buckets");
}
