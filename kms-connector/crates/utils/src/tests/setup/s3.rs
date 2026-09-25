use alloy::{
    hex,
    primitives::{B256, U256},
    signers::local::PrivateKeySigner,
};
use ciphertext_attestation::{
    CiphertextAttestationPayload, CiphertextFormat, S3_CT128_KEY_PREFIX, Version,
};
use std::{path::PathBuf, str::FromStr, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, Healthcheck, ImageExt,
    core::{CmdWaitFor, ContainerPort, ExecCommand, WaitFor},
    runners::AsyncRunner,
};
use tracing::info;

pub struct S3Instance {
    pub url: String,
    pub container: ContainerAsync<GenericImage>,
}

const RUSTFS_IMAGE: &str = "rustfs/rustfs";
const RUSTFS_TAG: &str = "1.0.0";
const S3_PORT: u16 = 9000;
const S3_ROOT_ACCESS_KEY: &str = "rustfs-root";
const S3_ROOT_SECRET_KEY: &str = "rustfs-root-secret";
const S3_CT_PATH: &str = "/tmp/ct";

pub const MINIO_ACCESS_KEY: &str = "fhevm-access-key";
pub const MINIO_SECRET_KEY: &str = "fhevm-access-secret-key";
pub const S3_CT_HANDLE: &str = "5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff00000000000030390500";
/// The same ciphertext under a handle of the Solana host chain with cluster tag 12345.
pub const S3_SOLANA_CT_HANDLE: &str =
    "5a88e7aa46f312ff70df6e84c85eb40cdfd42b18a9ff01000000000030390500";
pub const S3_CT_DIGEST: &str = "3a002df21130bda55f78d4403a73007a797f4a888174a620bbffc9052a045239";

pub const S3_CT_BUCKET: &str = "copro";
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
        Self { url, container }
    }

    pub async fn setup() -> anyhow::Result<Self> {
        info!("Starting RustFS container...");
        let container = GenericImage::new(RUSTFS_IMAGE, RUSTFS_TAG)
            .with_exposed_port(ContainerPort::Tcp(S3_PORT))
            // `/health/ready` only succeeds once storage and IAM are initialized.
            .with_wait_for(WaitFor::healthcheck())
            .with_health_check(
                Healthcheck::cmd([
                    "curl",
                    "-fsS",
                    "-o",
                    "/dev/null",
                    &format!("http://127.0.0.1:{S3_PORT}/health/ready"),
                ])
                .with_interval(Duration::from_millis(500))
                .with_retries(60),
            )
            .with_env_var("RUSTFS_ACCESS_KEY", S3_ROOT_ACCESS_KEY)
            .with_env_var("RUSTFS_SECRET_KEY", S3_ROOT_SECRET_KEY)
            .with_env_var("RUSTFS_CHECK_UPDATE", "false")
            .with_env_var("RUSTFS_OBS_LOG_DIRECTORY", "")
            .with_copy_to(
                S3_CT_PATH,
                PathBuf::from_str(&format!(
                    "{}/tests/data/{}",
                    env!("CARGO_MANIFEST_DIR"),
                    S3_CT_DIGEST
                ))
                .unwrap(),
            )
            .start()
            .await?;
        info!("RustFS container started!");

        let cont_host = container.get_host().await?;
        let cont_port = container.get_host_port_ipv4(S3_PORT).await?;
        let s3_url = format!("http://{cont_host}:{cont_port}");

        info!("Configuring RustFS...");
        let s3_instance = S3Instance::new(s3_url, container);
        s3_instance.configure().await?;
        info!("RustFS configured!");

        Ok(s3_instance)
    }

    async fn configure(&self) -> anyhow::Result<()> {
        let user = serde_json::json!({ "secretKey": MINIO_SECRET_KEY, "status": "enabled" });
        self.curl(
            &["-X", "PUT", "-d", &user.to_string()],
            &format!("rustfs/admin/v3/add-user?accessKey={MINIO_ACCESS_KEY}"),
        )
        .await?;
        self.curl(
            &["-X", "PUT"],
            &format!(
                "rustfs/admin/v3/set-user-or-group-policy?policyName=readwrite&userOrGroup={MINIO_ACCESS_KEY}&isGroup=false"
            ),
        )
        .await?;

        for bucket in ["kms-public", S3_CT_BUCKET] {
            // Object Lock can only be enabled at bucket creation.
            self.curl(
                &["-X", "PUT", "-H", "x-amz-bucket-object-lock-enabled: true"],
                bucket,
            )
            .await?;
            self.curl(
                &["-X", "PUT", "-d", &public_bucket_policy(bucket)],
                &format!("{bucket}?policy"),
            )
            .await?;
        }

        for handle in [S3_CT_HANDLE, S3_SOLANA_CT_HANDLE] {
            let attestation = format!(
                "x-amz-meta-ct-attestation: {}",
                rfc023_attestation_json(handle).await?
            );
            self.curl(
                &["-H", &attestation, "--upload-file", S3_CT_PATH],
                &format!("{S3_CT_BUCKET}/{S3_CT128_KEY_PREFIX}/{handle}/{COPROCESSOR_CONTEXT_ID}"),
            )
            .await?;
        }
        Ok(())
    }

    /// Runs a SigV4-signed `curl` request, with the root credentials, against the S3 API from
    /// inside the RustFS container.
    async fn curl(&self, args: &[&str], path: &str) -> anyhow::Result<()> {
        let user = format!("{S3_ROOT_ACCESS_KEY}:{S3_ROOT_SECRET_KEY}");
        let url = format!("http://127.0.0.1:{S3_PORT}/{path}");
        let mut cmd = vec![
            "curl",
            "-fsS",
            "--aws-sigv4",
            "aws:amz:us-east-1:s3",
            "--user",
            &user,
        ];
        cmd.extend_from_slice(args);
        cmd.push(&url);

        let mut result = self
            .container
            .exec(ExecCommand::new(cmd).with_cmd_ready_condition(CmdWaitFor::exit()))
            .await?;
        match result.exit_code().await? {
            Some(0) => Ok(()),
            code => Err(anyhow::anyhow!(
                "S3 request to /{path} failed ({code:?}): {}",
                String::from_utf8_lossy(&result.stderr_to_vec().await?)
            )),
        }
    }
}

/// Public read and write access for anonymous requests, like `mc anonymous set public`.
fn public_bucket_policy(bucket: &str) -> String {
    serde_json::json!({
        "Version": "2012-10-17",
        "Statement": [
            {
                "Effect": "Allow",
                "Principal": { "AWS": ["*"] },
                "Action": ["s3:GetBucketLocation", "s3:ListBucket", "s3:ListBucketMultipartUploads"],
                "Resource": [format!("arn:aws:s3:::{bucket}")],
            },
            {
                "Effect": "Allow",
                "Principal": { "AWS": ["*"] },
                "Action": [
                    "s3:AbortMultipartUpload",
                    "s3:DeleteObject",
                    "s3:GetObject",
                    "s3:ListMultipartUploadParts",
                    "s3:PutObject",
                ],
                "Resource": [format!("arn:aws:s3:::{bucket}/*")],
            },
        ]
    })
    .to_string()
}

async fn rfc023_attestation_json(handle: &str) -> anyhow::Result<String> {
    let attestation = CiphertextAttestationPayload::new(
        Version::V1,
        B256::from_slice(&hex::decode(handle)?),
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
