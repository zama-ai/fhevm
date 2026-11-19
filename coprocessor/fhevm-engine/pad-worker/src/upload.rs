//! The S3 copy of the pad outputs. The object metadata holds the attestation.

use crate::rows::{Claim, Output};
use anyhow::{ensure, Context};
use aws_sdk_s3::{primitives::ByteStream, types::ChecksumMode, Client};
use base64::Engine;
use ciphertext_attestation::{PrfOutputAttestation, S3_METADATA_ATTESTATION_KEY};
use sha2::{Digest, Sha256};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::sync::{OwnedSemaphorePermit, Semaphore};

#[derive(Clone)]
pub struct Uploader {
    client: Client,
    bucket: String,
    slots: Arc<Semaphore>,
    /// S3 checks the body against its SHA-256 on the PUT.
    sha256_checksum: bool,
    /// Interval of the read that finds failed uploads.
    pub resubmit_interval: Duration,
}

impl Uploader {
    pub fn new(
        client: Client,
        bucket: String,
        max_concurrent: NonZeroUsize,
        resubmit_interval: Duration,
    ) -> Self {
        Self {
            client,
            bucket,
            slots: Arc::new(Semaphore::new(max_concurrent.get())),
            sha256_checksum: true,
            resubmit_interval,
        }
    }

    /// For S3 servers without checksum support: the check reads the metadata and the length.
    pub fn without_sha256_checksum(mut self) -> Self {
        self.sha256_checksum = false;
        self
    }

    /// An upload slot. With `wait` false, `None` if all slots are taken.
    pub async fn slot(&self, wait: bool) -> Option<OwnedSemaphorePermit> {
        if wait {
            self.slots.clone().acquire_owned().await.ok()
        } else {
            self.slots.clone().try_acquire_owned().ok()
        }
    }

    /// Uploads the output to `key`, then checks the stored object. Hold a `slot` while it runs.
    pub async fn upload(&self, key: &str, output: &Output) -> anyhow::Result<()> {
        let attestation = serde_json::to_string(&output.attestation)?;
        let checksum = self.sha256_checksum.then(|| {
            base64::engine::general_purpose::STANDARD.encode(Sha256::digest(&output.ct128))
        });
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .metadata(S3_METADATA_ATTESTATION_KEY, &attestation)
            .set_checksum_sha256(checksum.clone())
            .body(ByteStream::from(output.ct128.clone()))
            .send()
            .await
            .context("put")?;

        let mut head = self.client.head_object().bucket(&self.bucket).key(key);
        if checksum.is_some() {
            head = head.checksum_mode(ChecksumMode::Enabled);
        }
        let object = head.send().await.context("head")?;
        let stored: PrfOutputAttestation = serde_json::from_str(
            object
                .metadata()
                .and_then(|m| m.get(S3_METADATA_ATTESTATION_KEY))
                .context("no attestation in the metadata")?,
        )?;
        ensure!(stored == output.attestation, "other attestation in S3");
        ensure!(
            object.content_length() == i64::try_from(output.ct128.len()).ok(),
            "other ct128 length in S3"
        );
        if let Some(checksum) = checksum {
            ensure!(
                object.checksum_sha256() == Some(checksum.as_str()),
                "other ct128 checksum in S3"
            );
        }
        Ok(())
    }
}

/// The key of an output, `pads/{prf}/{label}/{padLength}/ct128`.
pub fn s3_key(claim: &Claim) -> String {
    format!(
        "pads/{}/{}/{}/ct128",
        claim.prf_instance,
        hex::encode(claim.label),
        claim.pad_length
    )
}
