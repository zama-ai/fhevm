use std::net::TcpListener;

use alloy::signers::k256::pkcs8::EncodePrivateKey;
use aws_config::BehaviorVersion;
use aws_sdk_kms::types::{KeySpec, KeyUsageType, OriginType};
use base64::Engine;
use k256::SecretKey;
use testcontainers::{core::WaitFor, runners::AsyncRunner, ContainerAsync, GenericImage, ImageExt};

fn pick_free_port() -> u16 {
    TcpListener::bind("127.0.0.1:0")
        .unwrap()
        .local_addr()
        .unwrap()
        .port()
}

pub const LOCALSTACK_PORT: u16 = 4566;

pub struct LocalstackContainer {
    pub container: ContainerAsync<GenericImage>,
    pub host_port: u16,
}

pub async fn start_localstack() -> anyhow::Result<LocalstackContainer> {
    let host_port = pick_free_port();
    let container = GenericImage::new("localstack/localstack", "stable")
        .with_exposed_port(LOCALSTACK_PORT.into())
        .with_wait_for(WaitFor::message_on_stdout("Ready."))
        .with_mapped_port(host_port, LOCALSTACK_PORT.into())
        .start()
        .await?;
    Ok(LocalstackContainer {
        container,
        host_port,
    })
}

// Note that this function sets the AWS environment variables to point to the LocalStack instance.
pub async fn create_aws_aws_kms_client(host_port: u16) -> anyhow::Result<aws_sdk_kms::Client> {
    let endpoint_url = format!("http://localhost:{}", host_port);
    std::env::set_var("AWS_ENDPOINT_URL", endpoint_url);
    std::env::set_var("AWS_REGION", "us-east-1");
    std::env::set_var("AWS_ACCESS_KEY_ID", "test");
    std::env::set_var("AWS_SECRET_ACCESS_KEY", "test");
    let aws_conf = aws_config::load_defaults(BehaviorVersion::latest()).await;
    let aws_kms_client = aws_sdk_kms::Client::new(&aws_conf);
    Ok(aws_kms_client)
}

// Creates an AWS KMS key in LocalStack with the provided secret key material and returns the key ID.
pub async fn create_localstack_kms_signing_key(
    aws_kms_client: &aws_sdk_kms::Client,
    key: &[u8],
) -> anyhow::Result<String> {
    let key = SecretKey::from_bytes(key.into())?;
    let key_pkcs8_der_base64: String =
        base64::engine::general_purpose::STANDARD.encode(key.to_pkcs8_der().unwrap().as_bytes());

    // References on how to import the key material into the localstack AWS KMS:
    // - https://docs.localstack.cloud/user-guide/aws/kms/
    let tags = vec![aws_sdk_kms::types::Tag::builder()
        .tag_key("_custom_key_material_")
        .tag_value(key_pkcs8_der_base64)
        .build()
        .unwrap()];
    let out = aws_kms_client
        .create_key()
        .key_spec(KeySpec::EccSecgP256K1)
        .key_usage(KeyUsageType::SignVerify)
        .origin(OriginType::External)
        .set_tags(Some(tags))
        .send()
        .await?;
    Ok(out.key_metadata.unwrap().key_id().to_string())
}
