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
        let cmd = format!(
            "mc alias set myminio {} minioadmin minioadmin &&
            mc admin user add myminio {MINIO_ACCESS_KEY} {MINIO_SECRET_KEY} &&
            mc admin policy attach myminio readwrite --user {MINIO_ACCESS_KEY} &&
            mc mb --with-lock --ignore-existing myminio/kms-public &&
            mc mb --with-lock --ignore-existing myminio/ct64 &&
            mc mb --with-lock --ignore-existing myminio/ct128 &&
            mc anonymous set public myminio/kms-public &&
            mc anonymous set public myminio/ct64 &&
            mc anonymous set public myminio/ct128 &&
            mc cp /data/{S3_CT_DIGEST} --attr Ct-Format=compressed_on_cpu myminio/ct128/{S3_CT_DIGEST}",
            self.url
        );

        GenericImage::new("quay.io/minio/mc", "latest")
            .with_wait_for(WaitFor::Exit(ExitWaitStrategy::new().with_exit_code(0)))
            .with_entrypoint("/bin/sh")
            .with_network("host")
            .with_copy_to(
                format!("/data/{S3_CT_DIGEST}"),
                PathBuf::from_str(&format!(
                    "{}/../../tests/data/{}",
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
