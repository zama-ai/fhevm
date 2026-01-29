use std::time::Duration;

use async_trait::async_trait;
use aws_config::{retry::RetryConfig, timeout::TimeoutConfig, BehaviorVersion};
use aws_sdk_s3::config::Builder;
use aws_sdk_s3::Client;
use tokio_util::bytes;
use tracing::{error, info, warn};
use url::Url;

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
    // Configure the AWS Client to be Anonymous as it is only used to fetch files from public buckets
    // .no_credentials() is the Rust equivalent of --no-sign-request on the aws CLI
    let sdk_config = aws_config::defaults(BehaviorVersion::latest())
        .no_credentials()
        .load()
        .await;

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

pub async fn find_key(
    client: &Client,
    url: &str,
    bucket: &str,
    key_suffix: &str,
) -> anyhow::Result<String> {
    let mut keys = client
        .list_objects_v2()
        .bucket(bucket)
        .send()
        .await?
        .contents
        .unwrap_or_default();

    keys.sort_by(|a, b| a.key.cmp(&b.key));

    for obj in keys {
        if let Some(candidate) = obj.key {
            if candidate.ends_with(key_suffix) {
                info!(
                    bucket,
                    key_suffix, candidate, "Found matching key in bucket"
                );
                return Ok(candidate);
            }
        }
    }
    anyhow::bail!("Key {key_suffix} not found in bucket {bucket} at {url}");
}

#[async_trait]
impl AwsS3Interface for AwsS3Client {
    async fn get_bucket_key(
        &self,
        url: &str,
        bucket: &str,
        key_suffix: &str,
    ) -> anyhow::Result<bytes::Bytes> {
        // pick the right key from all keys
        let s3_client = create_s3_client(&S3Policy::DEFAULT, url).await?;
        let full_key = find_key(&s3_client, url, bucket, key_suffix).await?;
        Ok(s3_client
            .get_object()
            .bucket(bucket)
            .key(full_key)
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

fn bucket_from_domain(url: &Url) -> anyhow::Result<String> {
    let Some(domain) = url.domain() else {
        anyhow::bail!("Cannot deduce the bucket name from url {:?}", url);
    };
    let domain_parts = domain.split('.').collect::<Vec<&str>>();
    if domain_parts.len() < 2 {
        anyhow::bail!("Cannot deduce the bucket name from url {:?}", url);
    }
    Ok(domain_parts[0].to_owned())
}

fn split_url(s3_bucket_url: &String) -> anyhow::Result<(String, String)> {
    // e.g BBBBBB.s3.bla.bli.amazonaws.blu, the bucket is part of the domain
    let s3_bucket_url = if s3_bucket_url.contains("minio:9000") {
        // TODO: replace by docker configuration
        warn!(s3_bucket_url, "Using localhost for minio access");
        s3_bucket_url
            .replace("minio:9000", "172.17.0.1:9000")
            .to_owned()
    } else {
        s3_bucket_url.to_owned()
    };
    let parsed_url_and_bucket = url::Url::parse(&s3_bucket_url)?;
    let mut bucket = parsed_url_and_bucket
        .path()
        .trim_start_matches('/')
        .to_owned();
    if bucket.is_empty() {
        // e.g BBBBBB.s3.eu-west-1.amazonaws.com, the bucket is part of the domain
        bucket = bucket_from_domain(&parsed_url_and_bucket)?;
        let url = s3_bucket_url
            .replace(&(bucket.clone() + "."), "")
            .trim_end_matches('/')
            .to_owned();
        info!(s3_bucket_url, url, bucket, "Bucket from domain");
        Ok((url, bucket))
    } else {
        let url = s3_bucket_url
            .replace(&bucket, "")
            .trim_end_matches('/')
            .to_owned();
        info!(s3_bucket_url, url, bucket, "Parsed S3 url");
        Ok((url, bucket))
    }
}

pub async fn download_key_from_s3<A: AwsS3Interface>(
    s3_client: &A,
    s3_bucket_urls: &[String],
    key_path_suffix: String,
    offset_bucket: usize, // to not ask the same bucket first
) -> anyhow::Result<bytes::Bytes> {
    let nb_urls = s3_bucket_urls.len();
    for i_s3_bucket_url in 0..nb_urls {
        // ask different order per key
        let url_index = (i_s3_bucket_url + offset_bucket) % s3_bucket_urls.len();
        let s3_bucket_url = &s3_bucket_urls[url_index];
        info!(
            key_path_suffix,
            s3_bucket_url, i_s3_bucket_url, nb_urls, url_index, "Try downloading"
        );
        let Ok((url, bucket)) = split_url(s3_bucket_url) else {
            error!(s3_bucket_url, "Failed to parse S3 url");
            continue;
        };
        let result = s3_client
            .get_bucket_key(&url, &bucket, &key_path_suffix)
            .await;
        let Ok(result) = result else {
            error!(s3_bucket_url, key_path_suffix, result = ?result, "Downloading failed");
            continue;
        };
        info!(key_path_suffix, "Downloaded");
        return Ok(result);
    }
    error!(
        key_path_suffix,
        "Failed to download key from all S3 buckets"
    );
    anyhow::bail!("Failed to download key {key_path_suffix} from all S3 buckets");
}

mod test {
    #[test]
    fn test_split_devnet_url() {
        let (url, bucket) = super::split_url(
            &"https://zama-zws-dev-tkms-b6q87.s3.eu-west-1.amazonaws.com/".to_string(),
        )
        .unwrap();
        assert_eq!(url.as_str(), "https://s3.eu-west-1.amazonaws.com");
        assert_eq!(bucket.as_str(), "zama-zws-dev-tkms-b6q87");
    }
}
