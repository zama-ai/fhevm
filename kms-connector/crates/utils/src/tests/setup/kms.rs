use crate::tests::setup::{MINIO_ACCESS_KEY, MINIO_SECRET_KEY, ROOT_CARGO_TOML};
use std::{path::PathBuf, str::FromStr, time::Duration};
use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, Host, WaitFor, wait::ExitWaitStrategy},
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
        let cmd = format!(
            "kms-gen-keys --public-storage s3 --public-s3-bucket '{KMS_PUBLIC_VAULT_URL}' \\
            --aws-s3-endpoint '{s3_internal_url}' --private-storage file --private-file-path \\
            '{KMS_PRIVATE_VAULT_URL}' --cmd signing-keys centralized &&
            kms-server --config-file config/config.toml"
        );
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
                    "{}/../../../test-suite/fhevm/config/kms-core/config.toml",
                    env!("CARGO_MANIFEST_DIR"),
                ))
                .unwrap(),
            )
            .with_cmd(["-c", &cmd])
            .with_startup_timeout(Duration::from_secs(180))
            .start()
            .await?;
        info!("KMS Core container ready!");

        let kms_host = container.get_host().await?;
        let kms_port = container.get_host_port_ipv4(50051).await?;
        let kms_url = format!("http://{kms_host}:{kms_port}");
        let kms_instance = KmsInstance {
            container,
            url: kms_url,
        };

        info!("Generating KMS Core keys...");
        kms_instance
            .configure(kms_host.to_string(), kms_port, s3_url)
            .await?;
        info!("KMS Core keys successfully generated!");

        Ok(kms_instance)
    }

    async fn configure(&self, kms_host: String, kms_port: u16, s3_url: &str) -> anyhow::Result<()> {
        let s3_url_no_http = s3_url.strip_prefix("http://").unwrap();
        let cmd = format!(
            "cd /app/kms-core-client &&
            sed -i 's/minio:9000/{s3_url_no_http}/g' config.toml &&
            sed -i 's/kms-core:50051/{kms_host}:{kms_port}/g' config.toml &&
            bin/kms-core-client -f config.toml insecure-key-gen &&
            bin/kms-core-client -f config.toml insecure-crs-gen --max-num-bits 2048"
        );

        let version = ROOT_CARGO_TOML.get_kms_grpc_version();
        GenericImage::new("ghcr.io/zama-ai/kms/core-client", &version)
            .with_wait_for(WaitFor::Exit(ExitWaitStrategy::new().with_exit_code(0)))
            .with_entrypoint("/bin/sh")
            .with_network("host")
            .with_copy_to(
                "/app/kms-core-client/config.toml".to_string(),
                PathBuf::from_str(&format!(
                    "{}/../../../test-suite/fhevm/config/core-client/config.toml",
                    env!("CARGO_MANIFEST_DIR"),
                ))
                .unwrap(),
            )
            .with_cmd(["-c", &cmd])
            .with_startup_timeout(Duration::from_secs(180))
            .start()
            .await?;
        Ok(())
    }
}
