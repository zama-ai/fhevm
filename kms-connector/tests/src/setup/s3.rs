use std::{path::PathBuf, str::FromStr};

use testcontainers::{
    ContainerAsync, GenericImage, ImageExt,
    core::{ContainerPort, WaitFor, wait::ExitWaitStrategy},
    runners::AsyncRunner,
};

pub async fn setup_test_s3_instance() -> anyhow::Result<S3Instance> {
    let container = GenericImage::new("quay.io/minio/minio", "latest")
        .with_exposed_port(ContainerPort::Tcp(9000))
        .with_exposed_port(ContainerPort::Tcp(9001))
        .with_wait_for(WaitFor::message_on_stderr("MinIO Object Storage Server"))
        .with_cmd(["server", "/data --console-address ':9001'"])
        .start()
        .await?;
    println!("MinIO started...");

    let cont_host = container.get_host().await?;
    let cont_port = container.get_host_port_ipv4(9000).await?;
    let minio_url = format!("http://{cont_host}:{cont_port}");

    configure_minio(&minio_url).await?;
    println!("MinIO configured!");

    Ok(S3Instance::new(minio_url, container))
}

pub const MINIO_ACCESS_KEY: &str = "fhevm-access-key";
pub const MINIO_SECRET_KEY: &str = "fhevm-access-secret-key";

pub const S3_CT: &str = "011e517540a10486971fbf81dcf64c1b2fc9965744d0c8f7da0e4b338f1a31a9";

async fn configure_minio(url: &str) -> anyhow::Result<()> {
    let cmd = format!(
        "mc alias set myminio {url} minioadmin minioadmin &&
        mc admin user add myminio {MINIO_ACCESS_KEY} {MINIO_SECRET_KEY} &&
        mc admin policy attach myminio readwrite --user {MINIO_ACCESS_KEY} &&
        mc mb --with-lock --ignore-existing myminio/kms-public &&
        mc mb --with-lock --ignore-existing myminio/ct64 &&
        mc mb --with-lock --ignore-existing myminio/ct128 &&
        mc anonymous set public myminio/kms-public &&
        mc anonymous set public myminio/ct64 &&
        mc anonymous set public myminio/ct128 &&
        mc put /data/{S3_CT} myminio/ct128"
    );

    GenericImage::new("quay.io/minio/mc", "latest")
        .with_wait_for(WaitFor::Exit(ExitWaitStrategy::new().with_exit_code(0)))
        .with_entrypoint("/bin/sh")
        .with_network("host")
        .with_copy_to(
            format!("/data/{S3_CT}"),
            PathBuf::from_str(&format!("{}/data/{}", env!("CARGO_MANIFEST_DIR"), S3_CT)).unwrap(),
        )
        .with_cmd(["-c", &cmd])
        .start()
        .await?;
    Ok(())
}

pub struct S3Instance {
    pub url: String,
    pub _container: ContainerAsync<GenericImage>,
}

impl S3Instance {
    pub fn new(url: String, container: ContainerAsync<GenericImage>) -> Self {
        Self {
            url,
            _container: container,
        }
    }
}
