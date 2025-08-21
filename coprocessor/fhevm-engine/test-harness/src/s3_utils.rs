use std::time::Duration;

use aws_sdk_s3::Client;
use tokio::time::sleep;

/// Asserts that a key exists in S3 bucket with an optional expected value length.
/// If the key is not found, it retries for a specified number of times.
///# Arguments
/// * `client` - The S3 client to use for the request.
/// * `bucket` - The name of the S3 bucket.
/// * `key` - The key to check in the S3 bucket.
/// * `expected_value_len` - An optional expected length of the value associated with the key
pub async fn assert_key_exists(
    client: Client,
    bucket: &String,
    key: &String,
    expected_value_len: Option<i64>,
    retries: u64,
) {
    let mut key_found = false;

    for _i in 0..retries {
        if let Result::Ok(output) = client.head_object().bucket(bucket).key(key).send().await {
            key_found = true;

            if let Some(expected_value_len) = expected_value_len {
                let content_length = output.content_length().unwrap_or(0);
                assert!(
                    content_length == expected_value_len,
                    "Expected value length: {}, got: {}",
                    expected_value_len,
                    content_length
                );
            }

            break;
        }

        sleep(Duration::from_millis(100)).await;
    }

    assert!(
        key_found,
        "Failed to find key {} in S3 bucket: {}",
        key, bucket
    );
}

/// Asserts that the number of objects in S3 matches the expected count
pub async fn assert_object_count(client: Client, bucket: &String, expected_count: i32) {
    let max_keys = 100_000;

    assert!(
        expected_count <= max_keys,
        "Expected count {} exceeds max keys {}",
        expected_count,
        max_keys
    );

    let result = client
        .list_objects()
        .set_max_keys(Some(max_keys))
        .bucket(bucket)
        .send()
        .await
        .expect("Failed to list objects in S3 bucket");

    tracing::info!(
        "Found {} objects in S3 bucket: {}",
        result.contents().len(),
        bucket
    );

    assert_eq!(
        result.contents().len(),
        expected_count as usize,
        "Expected {} ct objects in S3 bucket, found {}",
        expected_count,
        result.contents().len()
    );
}
