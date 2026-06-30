use alloy::{
    hex,
    primitives::{B256, U256},
    signers::local::PrivateKeySigner,
};
use ciphertext_attestation::{CiphertextAttestationPayload, CiphertextFormat, Version};
use std::{path::PathBuf, str::FromStr};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, WaitFor, wait::ExitWaitStrategy},
    runners::AsyncRunner,
};
use tracing::info;

pub struct S3Instance {
    pub url: String,
    pub _container: ContainerAsync<GenericImage>,
}

pub const MINIO_ACCESS_KEY: &str = "fhevm-access-key";
pub const MINIO_SECRET_KEY: &str = "fhevm-access-secret-key";
pub const S3_CT_HANDLE: &str = "5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff00000000000030390500";
pub const S3_CT_DIGEST: &str = "3a002df21130bda55f78d4403a73007a797f4a888174a620bbffc9052a045239";

/// Bucket storing the test ciphertext under the RFC-023 layout (`{handle}/{context_id}`) only.
///
/// Kept separate from `ct128` (old `{digest}` layout only) so tests can exercise each URL
/// format in isolation: retrieval from this bucket cannot succeed via the old-URL fallback,
/// and retrieval from `ct128` cannot succeed via the RFC-023 URL.
pub const S3_CT_RFC023_BUCKET: &str = "ct128-rfc023";
const COPROCESSOR_CONTEXT_ID: U256 = U256::ONE;

/// The `keyId` bound by the test ciphertext attestation. On-chain `SnsCiphertextMaterial`
/// fixtures must use the same value for the attestation verification to succeed.
pub const S3_CT_KEY_ID: U256 = U256::ZERO;

/// Deterministic signer of the test ciphertext attestation (well-known Anvil test key #1).
///
/// Tests can register its address as an authorized Coprocessor signer on their mocked Gateway
/// so the attestation verification of the test ciphertext succeeds.
pub fn s3_ct_attestation_signer() -> PrivateKeySigner {
    PrivateKeySigner::from_str("59c6995e998f97a5a0044966f0945389dc9e86dae88c7a8412f4603b6b78690d")
        .unwrap()
}

impl S3Instance {
    pub fn new(url: String, container: ContainerAsync<GenericImage>) -> Self {
        Self {
            url,
            _container: container,
        }
    }

    pub async fn setup() -> anyhow::Result<Self> {
        info!("Starting MinIO container...");
        let container = GenericImage::new("quay.io/minio/minio", "latest")
            .with_exposed_port(ContainerPort::Tcp(9000))
            .with_exposed_port(ContainerPort::Tcp(9001))
            .with_wait_for(WaitFor::message_on_stderr("MinIO Object Storage Server"))
            .with_cmd(["server", "/data --console-address ':9001'"])
            .start()
            .await?;
        info!("MinIO container started!");

        let cont_host = container.get_host().await?;
        let cont_port = container.get_host_port_ipv4(9000).await?;
        let minio_url = format!("http://{cont_host}:{cont_port}");

        info!("Configuring MinIO...");
        let s3_instance = S3Instance::new(minio_url, container);
        s3_instance.configure().await?;
        info!("MinIO configured!");

        Ok(s3_instance)
    }

    async fn configure(&self) -> anyhow::Result<()> {
        // `mc`'s `--attr` parser consumes unquoted double quotes, but preserves them within
        // single-quoted sections. So the JSON attestation is wrapped in single quotes for `mc`,
        // and the whole attr is wrapped in double quotes (with the JSON's own double quotes
        // escaped) so the shell forwards the single quotes to `mc` untouched.
        let attestation = rfc023_attestation_json().await?.replace('"', "\\\"");
        let cmd = format!(
            "mc alias set myminio {} minioadmin minioadmin &&
            mc admin user add myminio {MINIO_ACCESS_KEY} {MINIO_SECRET_KEY} &&
            mc admin policy attach myminio readwrite --user {MINIO_ACCESS_KEY} &&
            mc mb --with-lock --ignore-existing myminio/kms-public &&
            mc mb --with-lock --ignore-existing myminio/ct128 &&
            mc mb --with-lock --ignore-existing myminio/{S3_CT_RFC023_BUCKET} &&
            mc anonymous set public myminio/kms-public &&
            mc anonymous set public myminio/ct128 &&
            mc anonymous set public myminio/{S3_CT_RFC023_BUCKET} &&
            mc cp /data/{S3_CT_DIGEST} --attr Ct-Format=compressed_on_cpu myminio/ct128/{S3_CT_DIGEST} &&
            mc cp /data/{S3_CT_DIGEST} --attr \"ct-attestation='{attestation}'\" \
                myminio/{S3_CT_RFC023_BUCKET}/{S3_CT_HANDLE}/{COPROCESSOR_CONTEXT_ID}",
            self.url
        );

        GenericImage::new("quay.io/minio/mc", "latest")
            .with_wait_for(WaitFor::Exit(ExitWaitStrategy::new().with_exit_code(0)))
            .with_entrypoint("/bin/sh")
            .with_network("host")
            .with_copy_to(
                format!("/data/{S3_CT_DIGEST}"),
                PathBuf::from_str(&format!(
                    "{}/tests/data/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    S3_CT_DIGEST
                ))
                .unwrap(),
            )
            .with_cmd(["-c", &cmd])
            .start()
            .await?;
        Ok(())
    }
}

async fn rfc023_attestation_json() -> anyhow::Result<String> {
    let attestation = CiphertextAttestationPayload::new(
        Version::V1,
        B256::from_slice(&hex::decode(S3_CT_HANDLE)?),
        S3_CT_KEY_ID,
        COPROCESSOR_CONTEXT_ID,
        B256::ZERO, // regular ciphertext digest, unused by retrieval tests
        B256::from_slice(&hex::decode(S3_CT_DIGEST)?),
        CiphertextFormat::CompressedOnCpu,
    )
    .sign(&s3_ct_attestation_signer())
    .await?;
    Ok(serde_json::to_string(&attestation)?)
}
