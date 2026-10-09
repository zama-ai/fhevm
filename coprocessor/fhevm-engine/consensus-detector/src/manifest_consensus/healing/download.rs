use std::sync::Arc;
use std::time::Duration;

use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::Client;
use block_manifest::{LEGACY_CONSENSUS_EPOCH, S3_METADATA_CONSENSUS_EPOCH_KEY};
use ciphertext_attestation::{s3_ct64_key, CiphertextAttestation, S3_METADATA_ATTESTATION_KEY};
use tokio::time::timeout;

use crate::manifest_consensus::verification::peer_manifest_source::s3_bucket_location;
use crate::manifest_consensus::ExecutionError;

const DOWNLOAD_TIMEOUT: Duration = Duration::from_secs(30);
const MAX_CT64_BYTES: usize = 16 * 1024 * 1024;

/// A peer's attested ct64 digest and the epoch whose stack uploaded it.
pub(super) struct AttestedCt64 {
    pub(super) digest: B256,
    pub(super) consensus_epoch: String,
}

impl AttestedCt64 {
    /// An object without the epoch tag was uploaded by the `legacy` epoch.
    pub(super) fn new(digest: B256, consensus_epoch: Option<&str>) -> Self {
        Self {
            digest,
            consensus_epoch: uploading_epoch(consensus_epoch),
        }
    }
}

/// A peer's ct64 bytes and the epoch whose stack uploaded them.
pub(super) struct DownloadedCt64 {
    pub(super) bytes: Vec<u8>,
    pub(super) consensus_epoch: String,
}

impl DownloadedCt64 {
    /// An object without the epoch tag was uploaded by the `legacy` epoch.
    pub(super) fn new(bytes: Vec<u8>, consensus_epoch: Option<&str>) -> Self {
        Self {
            bytes,
            consensus_epoch: uploading_epoch(consensus_epoch),
        }
    }
}

fn uploading_epoch(consensus_epoch: Option<&str>) -> String {
    consensus_epoch.unwrap_or(LEGACY_CONSENSUS_EPOCH).to_owned()
}

pub(super) trait Ct64Source: Clone + Send + Sync + 'static {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError>;

    /// Attested ct64 digest and uploading epoch from object metadata (HEAD).
    /// `expected_signer` is the registry signer for this bucket when known.
    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError>;
}

#[derive(Clone)]
pub(super) struct S3Ct64Source {
    client: Arc<Client>,
}

impl S3Ct64Source {
    pub(super) fn new(client: Arc<Client>) -> Self {
        Self { client }
    }
}

impl Ct64Source for S3Ct64Source {
    async fn get_ct64(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
    ) -> Result<DownloadedCt64, ExecutionError> {
        let location = s3_bucket_location(bucket_url)?;
        let key = location.object_key(&s3_ct64_key(handle, coprocessor_context_id));
        let response = timeout(
            DOWNLOAD_TIMEOUT,
            self.client
                .get_object()
                .bucket(&location.bucket)
                .key(&key)
                .send(),
        )
        .await
        .map_err(|_| {
            ExecutionError::S3TransientError(format!(
                "timed out downloading ct64 {key} from {}",
                location.bucket
            ))
        })?
        .map_err(|err| {
            if err
                .as_service_error()
                .is_some_and(|error| error.is_no_such_key())
            {
                ExecutionError::S3ObjectNotFound(format!("ct64 {key} from {}", location.bucket))
            } else {
                ExecutionError::S3TransientError(format!(
                    "failed to download ct64 {key} from {}: {err}",
                    location.bucket
                ))
            }
        })?;
        if response
            .content_length()
            .is_some_and(|length| length < 0 || length as usize > MAX_CT64_BYTES)
        {
            return Err(ExecutionError::InternalError(format!(
                "ct64 {key} from {} exceeds {MAX_CT64_BYTES} bytes",
                location.bucket
            )));
        }
        let consensus_epoch = response
            .metadata()
            .and_then(|meta| meta.get(S3_METADATA_CONSENSUS_EPOCH_KEY))
            .cloned();
        let collected = timeout(DOWNLOAD_TIMEOUT, response.body.collect())
            .await
            .map_err(|_| {
                ExecutionError::S3TransientError(format!(
                    "timed out reading ct64 {key} from {}",
                    location.bucket
                ))
            })?
            .map_err(|err| {
                ExecutionError::S3TransientError(format!(
                    "failed to read ct64 {key} from {}: {err}",
                    location.bucket
                ))
            })?;
        let body = collected.into_bytes();
        if body.len() > MAX_CT64_BYTES {
            return Err(ExecutionError::InternalError(format!(
                "ct64 {key} from {} exceeds {MAX_CT64_BYTES} bytes",
                location.bucket
            )));
        }
        Ok(DownloadedCt64::new(
            body.to_vec(),
            consensus_epoch.as_deref(),
        ))
    }

    async fn head_ct64_digest(
        &self,
        bucket_url: &str,
        handle: &[u8],
        coprocessor_context_id: U256,
        expected_signer: Option<Address>,
    ) -> Result<AttestedCt64, ExecutionError> {
        let location = s3_bucket_location(bucket_url)?;
        let key = location.object_key(&s3_ct64_key(handle, coprocessor_context_id));
        let response = timeout(
            DOWNLOAD_TIMEOUT,
            self.client
                .head_object()
                .bucket(&location.bucket)
                .key(&key)
                .send(),
        )
        .await
        .map_err(|_| {
            ExecutionError::S3TransientError(format!(
                "timed out heading ct64 {key} from {}",
                location.bucket
            ))
        })?
        .map_err(|err| {
            if err
                .as_service_error()
                .is_some_and(|error| error.is_not_found())
            {
                ExecutionError::S3ObjectNotFound(format!("ct64 {key} from {}", location.bucket))
            } else {
                ExecutionError::S3TransientError(format!(
                    "failed to head ct64 {key} from {}: {err}",
                    location.bucket
                ))
            }
        })?;
        let json = response
            .metadata()
            .and_then(|meta| {
                meta.get(S3_METADATA_ATTESTATION_KEY)
                    .or_else(|| meta.get("ct-attestation"))
            })
            .ok_or_else(|| {
                ExecutionError::S3ObjectNotFound(format!(
                    "ct64 {key} from {} has no attestation metadata",
                    location.bucket
                ))
            })?;
        let attestation: CiphertextAttestation = serde_json::from_str(json).map_err(|err| {
            ExecutionError::DeserializationError(format!(
                "ct64 attestation on {key} from {}: {err}",
                location.bucket
            ))
        })?;
        let handle: B256 = handle.try_into().map_err(|_| {
            ExecutionError::InternalError(format!("handle must be 32 bytes, got {}", handle.len()))
        })?;
        attestation
            .verify(
                handle,
                coprocessor_context_id,
                expected_signer.unwrap_or(attestation.signer),
            )
            .map_err(|err| {
                ExecutionError::DeserializationError(format!(
                    "ct64 attestation on {key} from {} failed verify: {err}",
                    location.bucket
                ))
            })?;
        let consensus_epoch = response
            .metadata()
            .and_then(|meta| meta.get(S3_METADATA_CONSENSUS_EPOCH_KEY))
            .map(String::as_str);
        Ok(AttestedCt64::new(
            attestation.ciphertext_digest,
            consensus_epoch,
        ))
    }
}
