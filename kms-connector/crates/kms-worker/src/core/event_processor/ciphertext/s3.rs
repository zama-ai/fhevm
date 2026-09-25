//! Thin wrapper around the shared `ciphertext-attestation` S3 client: retries ciphertext retrieval
//! across the winning-group buckets, and reports the retrieval metrics.

use crate::{
    core::{config::Config, event_processor::ProcessingError},
    monitoring::metrics::{S3_CIPHERTEXT_RETRIEVAL_COUNTER, S3_CIPHERTEXT_RETRIEVAL_ERRORS},
};
use alloy::{primitives::B256, transports::http::Client};
use anyhow::anyhow;
use ciphertext_attestation::{
    BoundedClient, COPROCESSOR_CONTEXT_ID_V1, CiphertextFormat, FetchCiphertextError,
    consensus::ConsensusMaterial,
};
use connector_utils::types::handle::extract_fhe_type_from_handle;
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{CiphertextFormat as GrpcCiphertextFormat, TypedCiphertext};
use std::num::NonZeroUsize;
use tracing::{debug, warn};

pub fn s3_client_from_config(config: &Config) -> anyhow::Result<BoundedClient> {
    let client = Client::builder()
        .connect_timeout(config.s3_connect_timeout)
        .build()
        .map_err(|e| anyhow!("Failed to create S3 HTTP client: {e}"))?;

    Ok(BoundedClient::for_attestations_and_ciphertexts(
        client,
        config.s3_max_concurrent_heads_per_bucket,
        config.s3_head_timeout,
        COPROCESSOR_CONTEXT_ID_V1,
        config.s3_max_concurrent_gets,
        config.s3_max_ciphertext_size,
        config.s3_get_timeout,
    ))
}

/// Retrieves the SNS ciphertext of `handle` from a bucket in the winning consensus group and
/// verifies it against the attested digest.
///
/// Tries every winning-group bucket before moving on to the next attempt, up to `attempts` times.
pub async fn retrieve_verified_ciphertext(
    client: &BoundedClient,
    handle: B256,
    material: &ConsensusMaterial,
    winning_buckets: &[String],
    attempts: NonZeroUsize,
) -> Result<TypedCiphertext, ProcessingError> {
    // A handle that carries no valid FHE type is malformed; retrying cannot fix it.
    let fhe_type = extract_fhe_type_from_handle(&handle).map_err(|e| {
        ProcessingError::irrecoverable(
            ErrorCode::Unprocessable,
            anyhow!("cannot extract FHE type from handle {handle}: {e}"),
        )
    })?;
    let ct_format = grpc_ciphertext_format(material.format);

    if winning_buckets.is_empty() {
        return Err(ProcessingError::recoverable(
            ErrorCode::CoproConsensusFailed,
            anyhow!("no winning-group bucket resolved for handle {handle}"),
        ));
    }

    let mut last_error: Option<FetchCiphertextError> = None;
    for attempt in 1..=attempts.get() {
        for bucket in winning_buckets {
            match client
                .fetch_ciphertext(bucket, handle, material.sns_ciphertext_digest)
                .await
            {
                Ok(body) => {
                    S3_CIPHERTEXT_RETRIEVAL_COUNTER.inc();
                    debug!(
                        %handle,
                        "Ciphertext retrieved and verified: format {}, length {}, FHE type {:?}",
                        ct_format.as_str_name(),
                        body.len(),
                        fhe_type
                    );
                    return Ok(TypedCiphertext {
                        ciphertext: body,
                        external_handle: handle.to_vec(),
                        fhe_type: fhe_type as i32,
                        ciphertext_format: ct_format.into(),
                    });
                }
                Err(e) => {
                    S3_CIPHERTEXT_RETRIEVAL_ERRORS.inc();
                    warn!(attempt, %handle, %bucket, "Failed to retrieve ciphertext: {e}");
                    last_error = Some(e);
                }
            }
        }
    }

    let last_error = last_error.map_or_else(
        || "no retrieval attempt made".to_string(),
        |e| e.to_string(),
    );
    Err(ProcessingError::recoverable(
        ErrorCode::CiphertextNotFound,
        anyhow!(
            "ciphertext unavailable for handle {handle}: all retrieval attempts failed \
             (last: {last_error})"
        ),
    ))
}

/// Maps the attested [`CiphertextFormat`] onto the KMS gRPC format.
fn grpc_ciphertext_format(format: CiphertextFormat) -> GrpcCiphertextFormat {
    match format {
        CiphertextFormat::CompressedOnCpu | CiphertextFormat::CompressedOnGpu => {
            GrpcCiphertextFormat::BigCompressed
        }
        CiphertextFormat::UncompressedOnCpu | CiphertextFormat::UncompressedOnGpu => {
            GrpcCiphertextFormat::BigExpanded
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::core::event_processor::ProcessingErrorKind;
    use alloy::primitives::U256;
    use ciphertext_attestation::sign::keccak_b256;
    use tokio::{io::AsyncWriteExt, net::TcpListener, task::JoinHandle};

    fn material() -> ConsensusMaterial {
        ConsensusMaterial {
            key_id: U256::ONE,
            ciphertext_digest: B256::ZERO,
            sns_ciphertext_digest: B256::ZERO,
            format: CiphertextFormat::CompressedOnCpu,
        }
    }

    fn single_permit_client() -> BoundedClient {
        s3_client_from_config(&Config {
            s3_max_concurrent_heads_per_bucket: std::num::NonZeroUsize::MIN,
            s3_max_concurrent_gets: std::num::NonZeroUsize::MIN,
            ..Default::default()
        })
        .unwrap()
    }

    /// An empty winning-group set is a recoverable consensus failure: there is nowhere to fetch
    /// from, and a retry may resolve a different winning group.
    #[tokio::test]
    async fn empty_winning_buckets_is_recoverable() {
        let result = retrieve_verified_ciphertext(
            &single_permit_client(),
            B256::ZERO,
            &material(),
            &[],
            NonZeroUsize::new(3).unwrap(),
        )
        .await;
        assert!(matches!(result, Err(ref e) if e.kind == ProcessingErrorKind::Recoverable));
    }

    /// A handle with no valid FHE type is irrecoverable: no amount of retrying resolves it.
    #[tokio::test]
    async fn unprocessable_handle_is_irrecoverable() {
        // Handle bytes below encode no valid FHE type (see `extract_fhe_type_from_handle`).
        let bad_handle = B256::from_slice(&[0xff; 32]);
        let result = retrieve_verified_ciphertext(
            &single_permit_client(),
            bad_handle,
            &material(),
            &["http://127.0.0.1:1".to_string()],
            NonZeroUsize::new(3).unwrap(),
        )
        .await;
        assert!(matches!(result, Err(ref e) if e.kind == ProcessingErrorKind::Irrecoverable));
    }

    /// A bucket serving `body` at any path. Returns its URL, and its accept loop to abort.
    async fn ciphertext_bucket(body: Vec<u8>) -> (String, JoinHandle<()>) {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let url = format!("http://{}", listener.local_addr().unwrap());
        let accept_loop = tokio::spawn(async move {
            while let Ok((mut stream, _)) = listener.accept().await {
                let head = format!("HTTP/1.1 200 OK\r\nContent-Length: {}\r\n\r\n", body.len());
                let _ = stream.write_all(head.as_bytes()).await;
                let _ = stream.write_all(&body).await;
                let _ = stream.shutdown().await;
            }
        });
        (url, accept_loop)
    }

    /// A failing bucket does not stall the retrieval: the next winning-group bucket is tried
    /// within the same attempt.
    #[tokio::test]
    async fn a_failing_bucket_is_skipped_in_favor_of_the_next_one() {
        let body = vec![0u8; 64];
        let digest = keccak_b256(&body);
        let (good_bucket, accept_loop) = ciphertext_bucket(body).await;
        let material = ConsensusMaterial {
            sns_ciphertext_digest: digest,
            ..material()
        };

        let result = retrieve_verified_ciphertext(
            &single_permit_client(),
            B256::ZERO,
            &material,
            &["http://127.0.0.1:1".to_string(), good_bucket],
            NonZeroUsize::MIN,
        )
        .await
        .expect("the second bucket should have served the ciphertext");
        assert_eq!(result.ciphertext, vec![0u8; 64]);

        accept_loop.abort();
    }
}
