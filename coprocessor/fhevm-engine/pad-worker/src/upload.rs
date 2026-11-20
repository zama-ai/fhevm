//! The S3 copy of the pad outputs: one object for each form. The object metadata holds the
//! attestation.

use crate::rows::{Claim, Output};
use anyhow::{ensure, Context};
use aws_sdk_s3::{primitives::ByteStream, types::ChecksumMode, Client};
use base64::Engine;
use bytes::Bytes;
use ciphertext_attestation::{PrfOutputAttestation, S3_METADATA_ATTESTATION_KEY};
use fhevm_engine_common::prf_output::prf_output_s3_key;
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

    /// Uploads the ct64, if the output has it, and the ct128, then checks the stored objects.
    /// Hold a `slot` while it runs.
    pub async fn upload(&self, output: &Output) -> anyhow::Result<()> {
        let attestation = serde_json::to_string(&output.attestation)?;
        if let Some(ct64) = &output.ct64 {
            self.put(
                &s3_key(&output.claim, Form::Ct64),
                ct64,
                &attestation,
                &output.attestation,
            )
            .await
            .context("ct64")?;
        }
        self.put(
            &s3_key(&output.claim, Form::Ct128),
            &output.ct128,
            &attestation,
            &output.attestation,
        )
        .await
        .context("ct128")
    }

    /// Puts `body` at `key` with `attestation`, the JSON of `expected`, then checks the stored
    /// object.
    async fn put(
        &self,
        key: &str,
        body: &Bytes,
        attestation: &str,
        expected: &PrfOutputAttestation,
    ) -> anyhow::Result<()> {
        let checksum = self
            .sha256_checksum
            .then(|| base64::engine::general_purpose::STANDARD.encode(Sha256::digest(body)));
        self.client
            .put_object()
            .bucket(&self.bucket)
            .key(key)
            .metadata(S3_METADATA_ATTESTATION_KEY, attestation)
            .set_checksum_sha256(checksum.clone())
            // A clone of `Bytes` copies no data.
            .body(ByteStream::from(body.clone()))
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
        ensure!(stored == *expected, "other attestation in S3");
        ensure!(
            object.content_length() == i64::try_from(body.len()).ok(),
            "other length in S3"
        );
        if let Some(checksum) = checksum {
            ensure!(
                object.checksum_sha256() == Some(checksum.as_str()),
                "other checksum in S3"
            );
        }
        Ok(())
    }
}

/// A form of an output. Each form has its own S3 object.
pub use fhevm_engine_common::prf_output::PrfOutputForm as Form;

/// The key of a form of an output, `pads/{form}/{prf}/{label}/{padLength}`.
pub fn s3_key(claim: &Claim, form: Form) -> String {
    prf_output_s3_key(claim.prf_instance, claim.label, claim.pad_length, form)
}
