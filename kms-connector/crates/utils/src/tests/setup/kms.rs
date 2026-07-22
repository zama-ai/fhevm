use crate::tests::setup::{MINIO_ACCESS_KEY, MINIO_SECRET_KEY, ROOT_CARGO_TOML};
use std::{path::PathBuf, str::FromStr, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, Host, WaitFor},
    runners::AsyncRunner,
};
use tracing::info;

pub struct KmsInstance {
    /// Use to keep the KMS container running during the tests.
    pub container: ContainerAsync<GenericImage>,
    pub url: String,
}

const KMS_PUBLIC_VAULT_URL: &str = "kms-public";
const KMS_PRIVATE_VAULT_URL: &str = "./keys";

impl KmsInstance {
    pub async fn setup(s3_url: &str) -> anyhow::Result<Self> {
        info!("Starting KMS Core container...");

        // Access s3 via internal docker host just for the KMS service, to avoid using network=host.
        // This way, the KMS Service is not started on a fixed port, which could avoid conflict
        // between parallel tests.
        let s3_port: u16 = s3_url.split(":").nth(2).unwrap().parse()?;
        let s3_internal_url = format!("http://host.docker.internal:{s3_port}");

        let cmd = "kms-gen-keys --config-file config/gen-keys-config.toml && \
            kms-server --config-file config/config.toml";
        let version = ROOT_CARGO_TOML.get_kms_grpc_version();
        let container = GenericImage::new("ghcr.io/zama-ai/kms/core-service", &version)
            .with_exposed_port(ContainerPort::Tcp(50051))
            .with_wait_for(WaitFor::message_on_stdout(
                "Starting KMS core on socket 0.0.0.0:50051",
            ))
            .with_entrypoint("/bin/sh")
            .with_host("host.docker.internal", Host::HostGateway)
            .with_env_var("AWS_ACCESS_KEY_ID", MINIO_ACCESS_KEY)
            .with_env_var("AWS_SECRET_ACCESS_KEY", MINIO_SECRET_KEY)
            .with_env_var(
                "KMS_CORE__PUBLIC_VAULT__STORAGE__S3__BUCKET",
                KMS_PUBLIC_VAULT_URL,
            )
            .with_env_var(
                "KMS_CORE__PRIVATE_VAULT__STORAGE__FILE__PATH",
                KMS_PRIVATE_VAULT_URL,
            )
            .with_env_var("KMS_CORE__AWS__S3_ENDPOINT", &s3_internal_url)
            .with_copy_to(
                "/app/kms/core/service/config/config.toml".to_string(),
                PathBuf::from_str(&format!(
                    "{}/tests/data/core-service-config.toml",
                    env!("CARGO_MANIFEST_DIR"),
                ))
                .unwrap(),
            )
            .with_copy_to(
                "/app/kms/core/service/config/gen-keys-config.toml".to_string(),
                PathBuf::from_str(&format!(
                    "{}/tests/data/gen-keys-config.toml",
                    env!("CARGO_MANIFEST_DIR"),
                ))
                .unwrap(),
            )
            .with_cmd(["-c", cmd])
            .with_startup_timeout(Duration::from_secs(180))
            .start()
            .await?;
        info!("KMS Core container ready!");

        let kms_host = container.get_host().await?;
        let kms_port = container.get_host_port_ipv4(50051).await?;
        let kms_url = format!("http://{kms_host}:{kms_port}");

        Ok(KmsInstance {
            container,
            url: kms_url,
        })
    }
}
