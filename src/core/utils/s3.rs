use alloy::{hex::encode, primitives::Address, providers::Provider, transports::http::reqwest};
use aws_config::BehaviorVersion;
use aws_sdk_s3::{config::Region, Client as S3Client};
use dashmap::DashMap;
use once_cell::sync::Lazy;
use sha3::{Digest, Keccak256};
use std::{sync::Arc, time::Duration};
use tracing::{debug, info, warn};

use crate::{
    core::config::S3Config,
    error::{Error, Result},
    gwl2_contracts::HTTPZ,
};

// Global cache for coprocessor S3 bucket URLs
static S3_BUCKET_CACHE: Lazy<DashMap<Address, String>> = Lazy::new(DashMap::new);

/// Logs the current state of the S3 bucket cache
fn log_cache_state() {
    let cache_size = S3_BUCKET_CACHE.len();
    info!("S3_BUCKET_CACHE state: {} entries", cache_size);

    if cache_size > 0 {
        let mut cache_entries = Vec::new();
        for entry in S3_BUCKET_CACHE.iter() {
            cache_entries.push(format!("{:?}: {}", entry.key(), entry.value()));
        }
        debug!("S3_BUCKET_CACHE contents: {}", cache_entries.join(", "));
    }
}

/// Retrieves the S3 bucket URL for a coprocessor from the HTTPZ contract
pub async fn call_httpz_to_get_s3_url<P: Provider + Clone>(
    coprocessor_address: Address,
    httpz_address: Address,
    provider: Arc<P>,
) -> Option<String> {
    info!(
        "Attempting to get S3 bucket URL for coprocessor {:?}",
        coprocessor_address
    );

    // Try to find a cached S3 bucket URL for any of the coprocessors
    if let Some(url) = S3_BUCKET_CACHE.get(&coprocessor_address) {
        info!(
            "CACHE HIT: Using cached S3 bucket URL for coprocessor {:?}: {}",
            coprocessor_address,
            url.value()
        );
        return Some(url.value().clone());
    }

    // If no cached URL found, query the HTTPZ contract for the first available coprocessor
    info!(
        "CACHE MISS: Querying HTTPZ contract for coprocessor {:?} S3 bucket URL",
        coprocessor_address
    );

    // Create HTTPZ contract instance
    let contract = HTTPZ::new(httpz_address, provider);

    // Call getCoprocessor method
    let coprocessor = match contract.getCoprocessor(coprocessor_address).call().await {
        Ok(result) => result,
        Err(e) => {
            warn!(
                "HTTPZ contract call failed for coprocessor {:?}: {}",
                coprocessor_address, e
            );
            return None;
        }
    };

    // Extract S3 bucket URL from the coprocessor
    let s3_bucket_url = coprocessor._0.s3BucketUrl.to_string();

    if s3_bucket_url.is_empty() {
        warn!(
            "Coprocessor {:?} returned empty S3 bucket URL",
            coprocessor_address
        );
        return None;
    }

    // Cache the URL for future use
    info!(
        "CACHE UPDATE: Adding S3 bucket URL for coprocessor {:?}: {}",
        coprocessor_address, s3_bucket_url
    );
    S3_BUCKET_CACHE.insert(coprocessor_address, s3_bucket_url.clone());

    // Log the updated cache state
    log_cache_state();

    info!(
        "Successfully retrieved and cached S3 bucket URL for coprocessor {:?}: {}",
        coprocessor_address, s3_bucket_url
    );
    Some(s3_bucket_url)
}

/// Process an S3 bucket URL to extract the region, endpoint URL, and bucket name
///
/// Handles various S3 URL formats:
/// - Standard AWS URLs: https://bucket-name.s3.region.amazonaws.com
/// - Path-style URLs: https://s3.region.amazonaws.com/bucket-name
/// - Custom endpoints: https://custom-endpoint.com/bucket-name
/// - Custom endpoints with region: https://custom-endpoint.com/s3/region/bucket
/// - URLs with trailing slashes: https://endpoint:9000/bucket/
///
/// Returns Option with tuple of (region, endpoint_url, bucket) or None if extraction fails
fn process_s3_bucket_url(s3_bucket_url: String) -> Option<(String, String, String)> {
    info!("Processing S3 bucket URL: {}", s3_bucket_url);

    // Parse the URL
    let url = match url::Url::parse(&s3_bucket_url) {
        Ok(url) => url,
        Err(e) => {
            warn!(
                "Failed to parse S3 bucket URL: {} - Error: {}",
                s3_bucket_url, e
            );
            return None;
        }
    };

    // Extract hostname
    let host = match url.host_str() {
        Some(host) => host,
        None => {
            warn!("No host in S3 bucket URL: {}", s3_bucket_url);
            return None;
        }
    };

    // Helper function to create endpoint URL with port if needed
    let make_endpoint = |host: &str| {
        if let Some(port) = url.port() {
            format!("{}://{}:{}", url.scheme(), host, port)
        } else {
            format!("{}://{}", url.scheme(), host)
        }
    };

    // Check if it's an AWS S3 URL
    if host.contains("amazonaws.com") {
        let parts: Vec<&str> = host.split('.').collect();

        // Handle bucket-name.s3.region.amazonaws.com format (virtual-hosted style)
        if parts.len() >= 4 && parts[1] == "s3" {
            let bucket = parts[0].to_string();
            let region = parts[2].to_string();
            let endpoint = make_endpoint(host);

            info!(
                "Extracted virtual-hosted style S3 URL - Region: {}, Endpoint: {}, Bucket: {}",
                region, endpoint, bucket
            );
            return Some((region, endpoint, bucket));
        }

        // Handle s3.region.amazonaws.com/bucket-name format (path-style)
        if parts.len() >= 3 && parts[0] == "s3" {
            if let Some(mut path_segments) = url.path_segments() {
                if let Some(bucket) = path_segments.next().filter(|s| !s.is_empty()) {
                    let region = parts[1].to_string();
                    let endpoint = make_endpoint(host);

                    info!(
                        "Extracted path-style S3 URL - Region: {}, Endpoint: {}, Bucket: {}",
                        region, endpoint, bucket
                    );
                    return Some((region, endpoint, bucket.to_string()));
                }
            }

            warn!(
                "Could not extract bucket from path-style S3 URL: {}",
                s3_bucket_url
            );
            return None;
        }
    }

    // For custom endpoints, get path segments
    let path_segments: Vec<&str> = url.path_segments().map_or(Vec::new(), |segments| {
        segments
            .collect::<Vec<&str>>()
            .into_iter()
            .filter(|s| !s.is_empty())
            .collect()
    });

    // No path segments means there's no bucket in the path
    if path_segments.is_empty() {
        warn!("No path segments found in URL: {}", s3_bucket_url);
        return None;
    }

    // Handle simple case: http://localhost:9000/bucket
    if path_segments.len() == 1 {
        let bucket = path_segments[0].to_string();
        // Use a default region for simple URLs
        // Note: This is an assumption, but necessary for simple URLs without region info
        let region = "us-east-1".to_string();
        let endpoint = make_endpoint(host);

        info!(
            "Extracted simple URL - Region: {} (default), Endpoint: {}, Bucket: {}",
            region, endpoint, bucket
        );
        return Some((region, endpoint, bucket));
    }

    // Check for region in path (some S3-compatible services put region in path)
    for (i, segment) in path_segments.iter().enumerate() {
        if *segment == "s3" && i + 1 < path_segments.len() {
            let region = path_segments[i + 1].to_string();

            // Try to extract bucket from the path
            if i + 2 < path_segments.len() {
                let bucket = path_segments[i + 2].to_string();
                let endpoint = make_endpoint(host);

                info!(
                    "Extracted custom endpoint S3 URL - Region: {}, Endpoint: {}, Bucket: {}",
                    region, endpoint, bucket
                );
                return Some((region, endpoint, bucket));
            }

            warn!(
                "Could extract region '{}' but not bucket from URL: {}",
                region, s3_bucket_url
            );
            return None;
        }
    }

    // If we can't determine the region or bucket with confidence, log and return None
    warn!(
        "Could not extract region and bucket from S3 URL: {}",
        s3_bucket_url
    );
    None
}

/// Compute Keccak256 digest of a byte array
pub fn compute_digest(ct: &[u8]) -> Vec<u8> {
    debug!("Computing Keccak256 digest for {} bytes of data", ct.len());
    let mut hasher = Keccak256::new();
    hasher.update(ct);
    let result = hasher.finalize().to_vec();
    debug!("Digest computed: {}", encode(&result));
    result
}

/// Retrieves a ciphertext from S3 using the bucket URL and ciphertext digest
pub async fn call_s3_ciphertext_retrieval(
    s3_bucket_url: String,
    ciphertext_digest: Vec<u8>,
    s3_config: Option<S3Config>,
) -> Option<Vec<u8>> {
    let digest_hex = encode(&ciphertext_digest);
    info!(
        "S3 RETRIEVAL START: Retrieving ciphertext with digest {} from S3 bucket {}",
        digest_hex, s3_bucket_url
    );

    // Process S3 bucket URL or use fallback configuration
    let (region, endpoint, bucket) = get_s3_components(&s3_bucket_url, s3_config.as_ref())?;

    // Try direct HTTP retrieval first if we have an HTTP endpoint
    if endpoint.starts_with("http") {
        if let Some(data) =
            try_direct_http_retrieval(&endpoint, &bucket, &digest_hex, &ciphertext_digest).await
        {
            return Some(data);
        }
    }

    // Fall back to S3 SDK retrieval
    try_s3_sdk_retrieval(&region, &endpoint, &bucket, &digest_hex, &ciphertext_digest).await
}

/// Helper function to get S3 components from URL or fallback config
fn get_s3_components(
    s3_bucket_url: &str,
    s3_config: Option<&S3Config>,
) -> Option<(String, String, String)> {
    info!("Processing S3 bucket URL to extract components");

    // Try to extract components from URL
    if let Some((region, endpoint, bucket)) = process_s3_bucket_url(s3_bucket_url.to_string()) {
        info!(
            "Successfully extracted S3 components - Region: {}, Endpoint: {}, Bucket: {}",
            region, endpoint, bucket
        );
        return Some((region, endpoint, bucket));
    }

    // Fall back to provided config if URL processing fails
    warn!(
        "URL processing failed. Using fallback configuration for S3 URL: {}",
        s3_bucket_url
    );

    let config = s3_config?;
    let endpoint = config.endpoint.as_ref()?;

    info!(
        "Using fallback configuration - Region: {}, Endpoint: {}, Bucket: {}",
        config.region, endpoint, config.bucket
    );

    Some((
        config.region.clone(),
        endpoint.clone(),
        config.bucket.clone(),
    ))
}

/// Try to retrieve ciphertext via direct HTTP
async fn try_direct_http_retrieval(
    endpoint: &str,
    bucket: &str,
    digest_hex: &str,
    ciphertext_digest: &[u8],
) -> Option<Vec<u8>> {
    info!(
        "Attempting direct HTTP retrieval from endpoint: {}, bucket: {}, key: {}",
        endpoint, bucket, digest_hex
    );

    // Construct direct URL
    let direct_url = format!("{}/{}/{}", endpoint, bucket, digest_hex);
    info!("Direct URL: {}", direct_url);

    // Try direct HTTP retrieval
    let ciphertext = match direct_http_retrieval(&direct_url).await {
        Ok(data) => data,
        Err(e) => {
            warn!(
                "Direct HTTP retrieval failed, falling back to S3 SDK: {}",
                e
            );
            return None;
        }
    };

    info!(
        "DIRECT HTTP RETRIEVAL SUCCESS: Retrieved {} bytes",
        ciphertext.len()
    );

    // Verify digest
    let calculated_digest = compute_digest(&ciphertext);
    if calculated_digest != ciphertext_digest {
        warn!(
            "DIGEST MISMATCH: Expected: {}, Got: {}",
            encode(ciphertext_digest),
            encode(&calculated_digest)
        );
    } else {
        info!("DIRECT HTTP RETRIEVAL COMPLETE: Successfully verified ciphertext digest");
    }

    // Return data even with digest mismatch for non-failability
    Some(ciphertext)
}

/// Try to retrieve ciphertext via S3 SDK
async fn try_s3_sdk_retrieval(
    region: &str,
    endpoint: &str,
    bucket: &str,
    digest_hex: &str,
    ciphertext_digest: &[u8],
) -> Option<Vec<u8>> {
    info!(
        "Configuring S3 client - Region: {}, Endpoint: {}, Bucket: {}",
        region, endpoint, bucket
    );

    // Create S3 client with custom timeout and retry configs
    let config = aws_config::defaults(BehaviorVersion::latest())
        .region(Region::new(region.to_string()))
        .endpoint_url(endpoint)
        .timeout_config(
            aws_sdk_s3::config::timeout::TimeoutConfig::builder()
                .operation_timeout(Duration::from_secs(1))
                .operation_attempt_timeout(Duration::from_millis(750))
                .build(),
        )
        .retry_config(
            aws_sdk_s3::config::retry::RetryConfig::standard()
                .with_max_attempts(2)
                .with_initial_backoff(Duration::from_millis(50)),
        )
        .load()
        .await;

    let client = S3Client::new(&config);
    info!(
        "S3 client configured with region: {}, endpoint: {}, timeouts: 1s/750ms, retries: 2",
        region, endpoint
    );

    // Get the object from S3
    info!("S3 GET REQUEST: bucket={}, key={}", bucket, digest_hex);
    let resp = match client
        .get_object()
        .bucket(bucket)
        .key(digest_hex)
        .send()
        .await
    {
        Ok(resp) => resp,
        Err(e) => {
            warn!(
                "S3 GET FAILED: bucket={}, key={}, error={}",
                bucket, digest_hex, e
            );
            return None;
        }
    };

    info!(
        "S3 GET SUCCESS: bucket={}, key={}, content-length={:?}",
        bucket,
        digest_hex,
        resp.content_length()
    );

    // Read the object body
    let body = match resp.body.collect().await {
        Ok(body) => body,
        Err(e) => {
            warn!("Failed to read S3 object body: {}", e);
            return None;
        }
    };

    let ciphertext = body.into_bytes().to_vec();
    info!(
        "S3 BODY READ COMPLETE: Retrieved {} bytes",
        ciphertext.len()
    );

    // Verify digest
    let calculated_digest = compute_digest(&ciphertext);
    if calculated_digest != ciphertext_digest {
        warn!(
            "S3 DIGEST MISMATCH: Expected: {}, Got: {}",
            encode(ciphertext_digest),
            encode(&calculated_digest)
        );
    } else {
        info!("S3 RETRIEVAL COMPLETE: Successfully verified ciphertext digest");
    }

    // Return data even with digest mismatch for non-failability
    Some(ciphertext)
}

/// Retrieves a file directly via HTTP
async fn direct_http_retrieval(url: &str) -> Result<Vec<u8>> {
    debug!("Attempting direct HTTP retrieval from URL: {}", url);

    // Create a reqwest client with appropriate timeouts
    let client = reqwest::Client::builder()
        .timeout(Duration::from_secs(1))
        .connect_timeout(Duration::from_millis(750))
        .build()
        .map_err(|e| Error::S3Error(format!("Failed to create HTTP client: {}", e)))?;

    // Send the GET request
    let response = client
        .get(url)
        .send()
        .await
        .map_err(|e| Error::S3Error(format!("HTTP request failed: {}", e)))?;

    // Check if the request was successful
    if !response.status().is_success() {
        return Err(Error::S3Error(format!(
            "HTTP request failed with status: {}",
            response.status()
        )));
    }

    // Read the response body
    let body = response
        .bytes()
        .await
        .map_err(|e| Error::S3Error(format!("Failed to read HTTP response body: {}", e)))?;

    Ok(body.to_vec())
}

/// Prefetches and caches S3 bucket URLs to return a list of coprocessor s3 urls
pub async fn prefetch_coprocessor_buckets<P: Provider + Clone>(
    coprocessor_addresses: Vec<Address>,
    httpz_address: Address,
    provider: Arc<P>,
    fallback_config: Option<&S3Config>,
) -> Vec<String> {
    info!(
        "S3 PREFETCH START: Prefetching S3 bucket URLs for {} coprocessors",
        coprocessor_addresses.len()
    );

    // Log current cache state before prefetching
    info!("S3 cache state before prefetching:");
    log_cache_state();

    let mut s3_urls = Vec::new();
    let mut success_count = 0;
    let mut cache_hit_count = 0;
    let mut cache_miss_count = 0;
    let mut fallback_used = false;

    for (idx, address) in coprocessor_addresses.iter().enumerate() {
        info!(
            "Processing coprocessor {}/{}: {:?}",
            idx + 1,
            coprocessor_addresses.len(),
            address
        );

        if S3_BUCKET_CACHE.contains_key(address) {
            info!(
                "CACHE HIT: S3 bucket URL for coprocessor {:?} already cached",
                address
            );
            cache_hit_count += 1;
            success_count += 1;

            // Add the cached URL to our result list
            if let Some(url) = S3_BUCKET_CACHE.get(address) {
                s3_urls.push(url.value().clone());
            }
            continue;
        }

        cache_miss_count += 1;
        info!(
            "CACHE MISS: Fetching S3 bucket URL for coprocessor {:?}",
            address
        );

        match call_httpz_to_get_s3_url(*address, httpz_address, provider.clone()).await {
            Some(s3_url) => {
                info!(
                    "Successfully fetched S3 bucket URL for coprocessor {:?}: {}",
                    address, s3_url
                );
                success_count += 1;
                s3_urls.push(s3_url);
            }
            None => {
                warn!(
                    "Failed to prefetch S3 bucket URL for coprocessor {:?}",
                    address
                );
            }
        };
    }

    // Log cache state after prefetching
    info!("S3 cache state after prefetching:");
    log_cache_state();

    // If we couldn't get any URLs but have a fallback config, use it
    if s3_urls.is_empty() {
        if let Some(config) = fallback_config {
            if !config.bucket.is_empty() {
                let fallback_url = format!(
                    "https://s3.{}.amazonaws.com/{}",
                    config.region, config.bucket
                );
                warn!(
                    "All S3 URL retrievals failed. Using global fallback S3 URL: {}",
                    fallback_url
                );
                s3_urls.push(fallback_url);
                success_count += 1;
                fallback_used = true;
            } else {
                warn!(
                    "All S3 URL retrievals failed and fallback bucket is empty. No URLs available."
                );
            }
        } else {
            warn!("All S3 URL retrievals failed and no fallback configuration available. No URLs available.");
        }
    }

    info!(
        "S3 PREFETCH COMPLETE: Successfully prefetched {}/{} S3 bucket URLs (cache hits: {}, cache misses: {}, fallback used: {})",
        success_count,
        coprocessor_addresses.len(),
        cache_hit_count,
        cache_miss_count,
        fallback_used
    );
    s3_urls
}

#[cfg(test)]
mod tests {
    use super::*;
    use tracing_subscriber::fmt;

    #[test]
    fn test_process_s3_bucket_url_virtual_hosted() {
        // Test virtual-hosted style URL (bucket-name.s3.region.amazonaws.com)
        let url = "https://my-bucket.s3.us-west-2.amazonaws.com".to_string();
        let result = process_s3_bucket_url(url);
        assert!(result.is_some());
        let (region, endpoint, bucket) = result.unwrap();
        assert_eq!(region, "us-west-2");
        assert_eq!(endpoint, "https://my-bucket.s3.us-west-2.amazonaws.com");
        assert_eq!(bucket, "my-bucket");
    }

    #[test]
    fn test_process_s3_bucket_url_path_style() {
        // Test path-style URL (s3.region.amazonaws.com/bucket-name)
        let url = "https://s3.eu-central-1.amazonaws.com/my-bucket".to_string();
        let result = process_s3_bucket_url(url);
        assert!(result.is_some());
        let (region, endpoint, bucket) = result.unwrap();
        assert_eq!(region, "eu-central-1");
        assert_eq!(endpoint, "https://s3.eu-central-1.amazonaws.com");
        assert_eq!(bucket, "my-bucket");
    }

    #[test]
    fn test_process_s3_bucket_url_path_region() {
        // Test URL with region in path
        let url = "https://storage.example.com/s3/ap-southeast-1/ct128".to_string();
        let result = process_s3_bucket_url(url);
        assert!(result.is_some());
        let (region, endpoint, bucket) = result.unwrap();
        assert_eq!(region, "ap-southeast-1");
        assert_eq!(endpoint, "https://storage.example.com");
        assert_eq!(bucket, "ct128");
    }

    #[test]
    fn test_process_s3_bucket_url_negative_cases() {
        // Test URLs that should not be parsed successfully

        // URL with no path segments
        let url1 = "https://storage.example.com".to_string();
        assert!(process_s3_bucket_url(url1).is_none());

        // URL with multiple path segments but no recognizable pattern
        let url2 = "https://storage.example.com/bucket/folder".to_string();
        assert!(process_s3_bucket_url(url2).is_none());

        // Malformed URL
        let url3 = "not-a-url".to_string();
        assert!(process_s3_bucket_url(url3).is_none());
    }

    #[test]
    fn test_process_s3_bucket_url_simple_format() {
        // Test simple URL format with just host and bucket in first path segment
        let url = "http://localhost:9000/bucket-name".to_string();
        let result = process_s3_bucket_url(url);
        assert!(result.is_some());
        let (region, endpoint, bucket) = result.unwrap();
        assert_eq!(region, "us-east-1"); // Default region for simple URLs
        assert_eq!(endpoint, "http://localhost:9000");
        assert_eq!(bucket, "bucket-name");

        // Test with trailing slash
        let url_with_slash = "http://localhost:9000/bucket-name/".to_string();
        let result_with_slash = process_s3_bucket_url(url_with_slash);
        assert!(result_with_slash.is_some());
        let (region, endpoint, bucket) = result_with_slash.unwrap();
        assert_eq!(region, "us-east-1");
        assert_eq!(endpoint, "http://localhost:9000");
        assert_eq!(bucket, "bucket-name");
    }

    #[test]
    fn test_process_s3_bucket_url_custom_endpoint_with_region_path() {
        // Test URL with custom endpoint and region in path segment
        let url = "http://minio.httpz-utils.svc.cluster.local:9000/s3/us-east-1/ct128".to_string();
        let result = process_s3_bucket_url(url);
        assert!(result.is_some());
        let (region, endpoint, bucket) = result.unwrap();
        assert_eq!(region, "us-east-1");
        assert_eq!(endpoint, "http://minio.httpz-utils.svc.cluster.local:9000");
        assert_eq!(bucket, "ct128");
    }

    #[test]
    fn test_compute_digest_empty_input() {
        // Test digest calculation for empty input
        let empty_data: Vec<u8> = vec![];
        let digest = compute_digest(&empty_data);

        // Keccak256 of empty input is a known value
        let expected_hex = "c5d2460186f7233c927e7db2dcc703c0e500b653ca82273b7bfad8045d85a470";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
        assert_eq!(digest.len(), 32); // Keccak256 produces 32 bytes
    }

    #[test]
    fn test_compute_digest_known_input() {
        // Test digest calculation for a known input
        let data = b"hello world";
        let digest = compute_digest(data);

        // Known Keccak256 hash of "hello world"
        let expected_hex = "47173285a8d7341e5e972fc677286384f802f8ef42a5ec5f03bbfa254cb01fad";
        let expected_bytes = alloy::hex::decode(expected_hex).unwrap();

        assert_eq!(digest, expected_bytes);
    }

    #[test]
    fn test_compute_digest_different_inputs() {
        // Test that different inputs produce different digests
        let data1 = b"test data 1";
        let data2 = b"test data 2";

        let digest1 = compute_digest(data1);
        let digest2 = compute_digest(data2);

        assert_ne!(digest1, digest2);
    }

    #[test]
    fn test_compute_digest_large_input() {
        // Test digest calculation for a larger input
        let large_data = vec![0u8; 1024 * 1024]; // 1MB of zeros
        let digest = compute_digest(&large_data);
        assert_eq!(digest.len(), 32);
        // The digest of 1MB of zeros is deterministic
        assert_eq!(
            encode(&digest),
            "7b6ff0a03e9c5a8e77a2059bf28e26a7f0e8d3939a7cfe2193908ad8d683be90"
        );
    }

    // TODO: to remove after integration: sanity check
    // This test requires a running MinIO server
    #[ignore]
    #[tokio::test]
    async fn test_direct_http_retrieval_minio() {
        // Initialize tracing for this test
        let subscriber = fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);

        let minio_url = "http://localhost:9000";
        let bucket = "ct128";
        let key = "1c37ba3cfd0151dd03584cd4819c6296d6a8b4d7ac3e31554fb0e842eab8ada9";

        println!(
            "Testing direct HTTP retrieval from MinIO at URL: {}",
            minio_url
        );

        let data = direct_http_retrieval(&format!("{}/{}/{}", minio_url, bucket, key)).await.expect("Failed to retrieve file from MinIO. Make sure MinIO server is running at http://localhost:9000 with a bucket named 'ct128'");

        println!("Successfully retrieved {} bytes from MinIO", data.len());

        let calculated_digest = compute_digest(&data);
        println!("Retrieved data digest: {}", encode(&calculated_digest));

        assert!(!data.is_empty(), "Retrieved data should not be empty");
    }

    // TODO: to remove after integration: sanity check
    // This test requires a running MinIO server
    #[ignore]
    #[tokio::test]
    async fn test_call_s3_ciphertext_retrieval_minio() {
        // Initialize tracing for this test
        let subscriber = fmt()
            .with_max_level(tracing::Level::INFO)
            .with_test_writer()
            .finish();
        let _guard = tracing::subscriber::set_default(subscriber);

        let minio_url = "http://localhost:9000";
        let bucket = "ct128";
        let call_url = format!("{}/{}", minio_url, bucket);
        println!(
            "Testing S3 ciphertext retrieval from MinIO at URL: {}",
            call_url
        );

        let digest_hex = "1c37ba3cfd0151dd03584cd4819c6296d6a8b4d7ac3e31554fb0e842eab8ada9";
        let digest_bytes = alloy::hex::decode(digest_hex).expect("Failed to decode hex digest");

        let s3_config = Some(S3Config {
            region: "us-east-1".to_string(),
            bucket: bucket.to_string(),
            endpoint: Some(minio_url.to_string()),
        });

        // Test the full retrieval function
        let result = call_s3_ciphertext_retrieval(call_url, digest_bytes.clone(), s3_config).await;

        match result {
            Some(data) => {
                println!(
                    "Successfully retrieved {} bytes from MinIO via call_s3_ciphertext_retrieval",
                    data.len()
                );

                assert!(!data.is_empty(), "Retrieved data should not be empty");

                let calculated_digest = compute_digest(&data);
                let calculated_hex = encode(&calculated_digest);
                println!("Retrieved data digest: {}", calculated_hex);
                println!("Expected digest: {}", digest_hex);
            }
            None => {
                // This should only happen if both direct HTTP and S3 SDK retrievals fail completely
                println!("Failed to retrieve ciphertext from MinIO");
                println!(
                    "This is expected if MinIO is not running or the bucket/object doesn't exist"
                );
            }
        }
    }
}
