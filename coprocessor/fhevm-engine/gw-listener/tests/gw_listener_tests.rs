use std::time::Duration;

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::U256,
    providers::{Provider, ProviderBuilder, WsConnect},
    signers::local::PrivateKeySigner,
    sol,
};

use aws_sdk_s3::{operation::get_object::GetObjectError, Client};
use gw_listener::{
    aws_s3::{AwsS3Client, AwsS3Interface},
    gw_listener::{key_id_to_key_bucket, to_bucket_key_prefix, GatewayListener},
    ConfigSettings,
};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use test_harness::instance::ImportMode;
use tokio::time::sleep;
use tokio_util::bytes;
use tokio_util::sync::CancellationToken;
use tracing::Level;

sol!(
    #[sol(rpc)]
    InputVerification,
    "artifacts/InputVerification.sol/InputVerification.json"
);

sol!(
    #[sol(rpc)]
    KMSGeneration,
    "artifacts/KMSGeneration.sol/KMSGeneration.json"
);

struct TestEnvironment {
    wallet: EthereumWallet,
    conf: ConfigSettings,
    cancel_token: CancellationToken,
    _test_instance: Option<test_harness::instance::DBInstance>, // maintain db alive
    db_pool: Pool<Postgres>,
    anvil: AnvilInstance,
}

impl TestEnvironment {
    async fn new() -> anyhow::Result<Self> {
        let _ = tracing_subscriber::fmt()
            .json()
            .with_level(true)
            .with_max_level(Level::DEBUG)
            .with_test_writer()
            .try_init();

        let mut conf = ConfigSettings::default();

        let mut _test_instance = None;
        if std::env::var("FORCE_DATABASE_URL").is_err() {
            let instance = test_harness::instance::setup_test_db(ImportMode::WithKeysNoSns)
                .await
                .expect("valid db instance");
            eprintln!("New test database on {}", instance.db_url());
            conf.database_url = instance.db_url().to_owned();
            _test_instance = Some(instance);
        };
        let db_pool = PgPoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(5))
            .connect(&conf.database_url)
            .await?;

        // Delete all proofs from the database.
        sqlx::query!("TRUNCATE verify_proofs",)
            .execute(&db_pool)
            .await?;

        // Delete last block.
        sqlx::query!("TRUNCATE gw_listener_last_block",)
            .execute(&db_pool)
            .await?;

        let anvil = Anvil::new().block_time(1).chain_id(12345).try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.clone().into();
        Ok(Self {
            wallet,
            conf,
            cancel_token: CancellationToken::new(),
            db_pool,
            _test_instance,
            anvil,
        })
    }
}

const RETRY_EVENT_TO_DB: u64 = 20;
const RETRY_DELAY: Duration = Duration::from_millis(500);

#[tokio::test]
#[serial(db)]
async fn verify_proof_request_inserted_into_db() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3Client {};
    let input_verification = InputVerification::deploy(&provider).await?;
    let kms_generation = KMSGeneration::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        *kms_generation.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
        aws_s3_client.clone(),
    );

    let run_handle = tokio::spawn(async move { gw_listener.run().await });

    let contract_address = PrivateKeySigner::random().address();
    let user_address = PrivateKeySigner::random().address();
    let txn_req = input_verification
        .verifyProofRequest(
            U256::from(42),
            contract_address,
            user_address,
            (&[1u8; 2048]).into(),
            Vec::<u8>::new().into(),
        )
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());

    for retry in 0..=RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query!(
            "SELECT zk_proof_id, chain_id, contract_address, user_address, input, extra_data
             FROM verify_proofs",
        )
        .fetch_all(&env.db_pool)
        .await?;
        if !rows.is_empty() {
            let row = &rows[0];
            assert_eq!(row.chain_id, 42);
            assert_eq!(row.contract_address, contract_address.to_string());
            assert_eq!(row.user_address, user_address.to_string());
            assert_eq!(row.input, Some([1u8; 2048].to_vec()));
            assert!(row.extra_data.is_empty());
            break;
        }
        assert!(
            retry < RETRY_EVENT_TO_DB,
            "Timed out waiting for event to be processed"
        );
    }

    env.cancel_token.cancel();
    run_handle.await??;
    Ok(())
}

async fn has_public_key(db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query!("SELECT pks_key FROM tenants WHERE chain_id = $1", 12345,)
            .fetch_all(db_pool)
            .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            if rows[0].pks_key == expected_key_content {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

async fn has_server_key(db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query!("SELECT sks_key FROM tenants WHERE chain_id = $1", 12345,)
            .fetch_all(db_pool)
            .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            if rows[0].sks_key == expected_key_content {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

async fn has_crs(db_pool: &Pool<Postgres>) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query!(
            "SELECT public_params FROM tenants WHERE chain_id = $1",
            12345,
        )
        .fetch_all(db_pool)
        .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            if rows[0].public_params == expected_key_content {
                return Ok(true);
            }
        }
    }
    Ok(false)
}

#[derive(Clone)]
pub struct AwsS3ClientMocked(Client);

impl AwsS3Interface for AwsS3ClientMocked {
    async fn get_bucket_key(
        &self,
        _url: &str,
        bucket: &str,
        key: &str,
    ) -> anyhow::Result<bytes::Bytes> {
        Ok(self
            .0
            .get_object()
            .bucket(bucket)
            .key(key)
            .send()
            .await?
            .body
            .collect()
            .await?
            .into_bytes())
    }
}

// test bad bucket
// test bad key
#[tokio::test]
#[serial(db)]
async fn keygen_ok() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .try_init()
        .ok();

    use aws_sdk_s3::operation::get_object::GetObjectOutput;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_sdk_s3::Client;
    use aws_smithy_mocks::RuleMode;
    use aws_smithy_mocks::{mock, mock_client};
    use gw_listener::KeyType;

    // see ../contracts/KMSGeneration.sol
    let buckets = [
        "test-bucket1/PUB-P1",
        "test-bucket2/PUB-P2",
        "test-bucket3/PUB-P3",
        "test-bucket4/PUB-P4",
    ];

    let keys_digests = [KeyType::PublicKey, KeyType::ServerKey];

    let key_id = U256::from(16);

    let mut rules = vec![];
    for &bucket in &buckets {
        for key_type in &keys_digests {
            let key_type_str: &str = to_bucket_key_prefix(*key_type);
            let key_id_no_0x = key_id_to_key_bucket(key_id);
            let key = format!("{}/{}", key_type_str, key_id_no_0x);
            eprintln!("Adding {}/{}", bucket, key);
            let get_object_rule = mock!(Client::get_object)
                .match_requests(move |req| req.bucket() == Some(bucket) && req.key() == Some(&key))
                .then_output(|| {
                    GetObjectOutput::builder()
                        .body(ByteStream::from_static(b"key_bytes"))
                        .build()
                });
            rules.push(get_object_rule);
        }
    }
    for &bucket in &buckets {
        let key_id_no_0x = &format!("{key_id:064X}");
        let key = format!("PUB/CRS/{key_id_no_0x}");
        eprintln!("Adding {}/{}", bucket, key);
        let get_object_rule = mock!(Client::get_object)
            .match_requests(move |req| req.bucket() == Some(bucket) && req.key() == Some(&key))
            .then_output(|| {
                GetObjectOutput::builder()
                    .body(ByteStream::from_static(b"key_bytes"))
                    .build()
            });
        rules.push(get_object_rule);
    }
    let rules_ref: Vec<_> = rules.iter().collect();

    // Create a mocked client with the rule
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let input_verification = InputVerification::deploy(&provider).await?;
    let kms_generation = KMSGeneration::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        *kms_generation.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
        aws_s3_client.clone(),
    );

    let listener = tokio::spawn(async move { gw_listener.run().await });

    assert!(!has_public_key(&env.db_pool.clone()).await?);
    assert!(!has_server_key(&env.db_pool.clone()).await?);
    assert!(!has_crs(&env.db_pool.clone()).await?);

    let txn_req = kms_generation
        .keygen_public_key()
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    assert!(has_public_key(&env.db_pool.clone()).await?);

    let txn_req = kms_generation
        .keygen_server_key()
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    assert!(has_server_key(&env.db_pool.clone()).await?);

    let txn_req = kms_generation.crsgen().into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    assert!(has_crs(&env.db_pool.clone()).await?);

    env.cancel_token.cancel();
    listener.abort();
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_compromised_key() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .try_init()
        .ok();

    use aws_sdk_s3::operation::get_object::GetObjectOutput;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_sdk_s3::Client;
    use aws_smithy_mocks::RuleMode;
    use aws_smithy_mocks::{mock, mock_client};
    use gw_listener::KeyType;

    // see ../contracts/KMSGeneration.sol
    let buckets = [
        "test-bucket1/PUB-P1",
        "test-bucket2/PUB-P2",
        "test-bucket3/PUB-P3",
        "test-bucket4/PUB-P4",
    ];

    let keys_digests = [KeyType::PublicKey, KeyType::ServerKey];

    let key_id = U256::from(16);

    let mut rules = vec![];
    for bucket in buckets {
        for key_type in &keys_digests {
            let key_type_str: &str = to_bucket_key_prefix(*key_type);
            let key_id_no_0x = key_id_to_key_bucket(key_id);
            let key = format!("{}/{}", key_type_str, key_id_no_0x);
            eprintln!("Adding {}/{}", bucket, key);
            let get_object_rule = mock!(Client::get_object)
                .match_requests(move |req| req.bucket() == Some(bucket) && req.key() == Some(&key))
                .then_output(|| {
                    GetObjectOutput::builder()
                        .body(ByteStream::from_static(b"bad_key_bytes"))
                        .build()
                });
            rules.push(get_object_rule);
        }
    }
    let rules_ref: Vec<_> = rules.iter().collect();

    // Create a mocked client with the rule
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let input_verification = InputVerification::deploy(&provider).await?;
    let kms_generation = KMSGeneration::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        *kms_generation.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
        aws_s3_client.clone(),
    );

    let mut sleep_duration = 6_u64;
    let db_pool = env.db_pool.clone();
    let result =
        tokio::spawn(async move { gw_listener.run_loop(&db_pool, &mut sleep_duration).await });

    assert!(!has_public_key(&env.db_pool.clone()).await?);
    assert!(!has_server_key(&env.db_pool.clone()).await?);

    let txn_req = kms_generation
        .keygen(1) // Test
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    assert!(result.is_finished());
    let result = result.await;
    assert!(result.is_ok());
    let result = result.unwrap();
    assert!(result.is_err());
    let err = result.unwrap_err();
    assert!(err.to_string().contains("Invalid Key digest"));

    assert!(!has_public_key(&env.db_pool.clone()).await?);
    assert!(!has_server_key(&env.db_pool.clone()).await?);

    env.cancel_token.cancel();
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_bad_key_or_bucket() -> anyhow::Result<()> {
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .compact()
        .try_init()
        .ok();

    use aws_sdk_s3::operation::get_object::GetObjectOutput;
    use aws_sdk_s3::primitives::ByteStream;
    use aws_sdk_s3::Client;
    use aws_smithy_mocks::RuleMode;
    use aws_smithy_mocks::{mock, mock_client};
    use gw_listener::KeyType;

    // see ../contracts/KMSGeneration.sol
    let buckets = [
        "test-bucket1/PUB-P1",
        "test-bucket2/PUB-P2",
        "test-bucket3/PUB-P3",
        "test-bucket4/PUB-P4",
    ];

    let keys_digests = [KeyType::PublicKey, KeyType::ServerKey];

    let key_id = U256::from(16);

    let mut rules = vec![];
    for (i, bucket) in buckets.iter().copied().enumerate() {
        for key_type in &keys_digests {
            let key_type_str: &str = to_bucket_key_prefix(*key_type);
            let key_id_no_0x = key_id_to_key_bucket(key_id);
            let key = format!("{}/{}", key_type_str, key_id_no_0x);
            eprintln!("Adding {}/{}", bucket, key);
            let get_object_rule = mock!(Client::get_object)
                .match_requests(move |req| req.bucket() == Some(bucket) && req.key() == Some(&key));
            let get_object_rule = if i < 3 {
                // most bucket fails
                get_object_rule.then_error(|| {
                    let nsk = aws_sdk_s3::types::error::NoSuchKey::builder()
                        .message("")
                        .build();
                    GetObjectError::NoSuchKey(nsk)
                })
            } else {
                get_object_rule.then_output(|| {
                    GetObjectOutput::builder()
                        .body(ByteStream::from_static(b"key_bytes"))
                        .build()
                })
            };
            rules.push(get_object_rule);
        }
    }
    let rules_ref: Vec<_> = rules.iter().collect();

    // Create a mocked client with the rule
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet)
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let input_verification = InputVerification::deploy(&provider).await?;
    let kms_generation = KMSGeneration::deploy(&provider).await?;
    let gw_listener = GatewayListener::new(
        *input_verification.address(),
        *kms_generation.address(),
        env.conf.clone(),
        env.cancel_token.clone(),
        provider.clone(),
        aws_s3_client.clone(),
    );

    let listener = tokio::spawn(async move { gw_listener.run().await });

    assert!(!has_public_key(&env.db_pool.clone()).await?);
    assert!(!has_server_key(&env.db_pool.clone()).await?);

    let txn_req = kms_generation
        .keygen(1) // Test
        .into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());

    assert!(has_public_key(&env.db_pool.clone()).await?);
    assert!(has_server_key(&env.db_pool.clone()).await?);

    env.cancel_token.cancel();
    listener.abort();
    Ok(())
}
