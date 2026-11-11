//! The S3 copy of the pad outputs. The object metadata holds the attestation.

use crate::rows::{Claim, Output};
use alloy::primitives::keccak256;
use anyhow::{ensure, Context};
use aws_sdk_s3::{primitives::ByteStream, Client};
use ciphertext_attestation::{PrfOutputAttestation, S3_METADATA_ATTESTATION_KEY};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use tokio::sync::Semaphore;

#[derive(Clone)]
pub struct Uploader {
    client: Client,
    bucket: String,
    slots: Arc<Semaphore>,
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
            resubmit_interval,
        }
    }

    /// Uploads the output to `key`, then reads it back and checks it.
    pub async fn upload(&self, key: &str, output: &Output) -> anyhow::Result<()> {
        let _slot = self.slots.acquire().await?;
        let attestation = serde_json::to_string(&output.attestation)?;
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .metadata(S3_METADATA_ATTESTATION_KEY, &attestation)
            .body(ByteStream::from(output.ct128.clone()))
            .send()
            .await
            .context("put")?;

        let object = self
            .client
            .get_object()
            .bucket(&self.bucket)
            .key(key)
            .send()
            .await
            .context("read back")?;
        let stored: PrfOutputAttestation = serde_json::from_str(
            object
                .metadata()
                .and_then(|m| m.get(S3_METADATA_ATTESTATION_KEY))
                .context("no attestation in the metadata")?,
        )?;
        let body = object
            .body
            .collect()
            .await
            .context("read back")?
            .into_bytes();
        ensure!(stored == output.attestation, "other attestation in S3");
        ensure!(
            keccak256(&body) == output.attestation.sns_ciphertext_digest,
            "bad ct128 digest in S3"
        );
        Ok(())
    }
}

/// The key of an output, `pads/{prf}/{label}/{padLength}/ct128`.
pub fn s3_key(claim: &Claim) -> String {
    key("pads", claim)
}

/// The key of a reference output. No lifecycle rule deletes `pads-reference/`.
pub fn reference_key(claim: &Claim) -> String {
    key("pads-reference", claim)
}

fn key(prefix: &str, claim: &Claim) -> String {
    format!(
        "{prefix}/{}/{}/{}/ct128",
        claim.prf_instance,
        hex::encode(claim.label),
        claim.pad_length
    )
}
