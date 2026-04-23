use std::{
    collections::HashSet,
    sync::{Arc, OnceLock, RwLock},
    time::Duration,
};

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    signers::local::PrivateKeySigner,
    sol,
};

use async_trait::async_trait;
use aws_sdk_s3::operation::get_object::{GetObjectError, GetObjectOutput};
use aws_sdk_s3::primitives::ByteStream;
use aws_sdk_s3::Client;
use aws_smithy_mocks::RuleMode;
use aws_smithy_mocks::{mock, mock_client};

use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::kms_generation::aws_s3::{find_key, AwsS3Interface};
use host_listener::kms_generation::{
    key_id_to_aws_key, key_id_to_database_bytes,
    process_finalized_kms_generation_events_until_idle,
    process_kms_generation_logs, to_key_prefix, KeyType,
};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use test_harness::instance::ImportMode;
use tokio::time::sleep;
use tokio_util::bytes;
use tracing::Level;
use tracing_subscriber::fmt::{writer::MakeWriterExt, MakeWriter};

// Test mock of the KMSGeneration contract. ABI events match production so
// process_kms_generation_logs can decode logs emitted here.
sol!(
    #[sol(rpc)]
    KMSGenerationMock,
    "artifacts/KMSGeneration.sol/KMSGeneration.json"
);

static TEST_LOGS: OnceLock<Arc<RwLock<String>>> = OnceLock::new();

#[derive(Clone)]
struct TestLogs {
    logs: Arc<RwLock<String>>,
}

impl TestLogs {
    fn new() -> Self {
        let logs =
            TEST_LOGS.get_or_init(|| Arc::new(RwLock::new(String::new())));
        // Flush logs every time a new test starts.
        logs.write().unwrap().clear();
        Self { logs: logs.clone() }
    }

    fn add(&mut self, data: &[u8]) {
        let data = String::from_utf8_lossy(data).into_owned();
        *self.logs.write().unwrap() += &data;
    }

    fn contains(&self, substr: &str) -> bool {
        self.logs.read().unwrap().contains(substr)
    }
}

struct Writer {
    logs: TestLogs,
}

impl Writer {
    fn new(logs: TestLogs) -> Self {
        Self { logs }
    }
}

impl std::io::Write for Writer {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.logs.add(buf);
        Ok(buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

impl<'a> MakeWriter<'a> for Writer {
    type Writer = Self;

    fn make_writer(&'a self) -> Self::Writer {
        Self {
            logs: self.logs.clone(),
        }
    }
}

struct TestEnvironment {
    wallet: EthereumWallet,
    _test_instance: Option<test_harness::instance::DBInstance>, // keep db alive
    database_url: DatabaseURL,
    db_pool: Pool<Postgres>,
    anvil: AnvilInstance,
    test_logs: TestLogs,
}

impl TestEnvironment {
    async fn new() -> anyhow::Result<Self> {
        let test_logs = TestLogs::new();
        let writer = Writer::new(test_logs.clone());

        let _ = tracing_subscriber::fmt()
            .compact()
            .with_writer(writer.and(std::io::stdout))
            .with_level(true)
            .with_max_level(Level::INFO)
            .try_init();

        let mut database_url =
            fhevm_engine_common::utils::DatabaseURL::default();

        let mut _test_instance = None;
        if std::env::var("FORCE_DATABASE_URL").is_err() {
            let instance = test_harness::instance::setup_test_db(
                ImportMode::WithKeysNoSns,
            )
            .await
            .expect("valid db instance");
            eprintln!("New test database on {}", instance.db_url());
            database_url = instance.db_url.clone();
            _test_instance = Some(instance);
        };

        let db_pool = PgPoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url.as_str())
            .await?;

        // Clean out any keys/crs from previous tests so has_not_* assertions
        // start from a known state. Use runtime queries here so we don't need
        // to regenerate the sqlx offline cache.
        sqlx::query("DELETE FROM keys").execute(&db_pool).await?;
        sqlx::query("DELETE FROM crs").execute(&db_pool).await?;
        sqlx::query("DELETE FROM kms_key_activation_events")
            .execute(&db_pool)
            .await?;
        sqlx::query("DELETE FROM kms_crs_activation_events")
            .execute(&db_pool)
            .await?;

        let anvil = Anvil::new().block_time(1).chain_id(12345).try_spawn()?;
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.clone().into();
        Ok(Self {
            wallet,
            database_url,
            db_pool,
            _test_instance,
            anvil,
            test_logs,
        })
    }

    fn contains_log(&self, log: &str) -> bool {
        self.test_logs.contains(log)
    }
}

const RETRY_EVENT_TO_DB: u64 = 3;
const RETRY_DELAY: Duration = Duration::from_millis(100);

async fn has_not_public_key(
    db_pool: &Pool<Postgres>,
    key_id: U256,
) -> anyhow::Result<bool> {
    has_public_key_gen(db_pool, false, key_id).await.map(|b| !b)
}

async fn has_public_key(
    db_pool: &Pool<Postgres>,
    key_id: U256,
) -> anyhow::Result<bool> {
    has_public_key_gen(db_pool, true, key_id).await
}

async fn has_public_key_gen(
    db_pool: &Pool<Postgres>,
    retry: bool,
    key_id: U256,
) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query(
            "SELECT pks_key FROM keys WHERE key_id_gw = $1 AND status = 'active'",
        )
        .bind(&key_id_to_database_bytes(key_id))
        .fetch_all(db_pool)
        .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            let pks_key: Vec<u8> = rows[0].try_get("pks_key")?;
            if pks_key == expected_key_content {
                return Ok(true);
            }
        }
        if !retry {
            break;
        }
    }
    Ok(false)
}

async fn has_not_server_key(
    db_pool: &Pool<Postgres>,
    key_id: U256,
) -> anyhow::Result<bool> {
    has_server_key_gen(db_pool, false, key_id).await.map(|b| !b)
}

async fn has_server_key(
    db_pool: &Pool<Postgres>,
    key_id: U256,
) -> anyhow::Result<bool> {
    has_server_key_gen(db_pool, true, key_id).await
}

async fn has_server_key_gen(
    db_pool: &Pool<Postgres>,
    retry: bool,
    key_id: U256,
) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query(
            "SELECT sks_key FROM keys WHERE key_id_gw = $1 AND status = 'active'",
        )
        .bind(&key_id_to_database_bytes(key_id))
        .fetch_all(db_pool)
        .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            let sks_key: Vec<u8> = rows[0].try_get("sks_key")?;
            if sks_key == expected_key_content {
                return Ok(true);
            }
        }
        if !retry {
            break;
        }
    }
    Ok(false)
}

async fn has_not_crs(
    db_pool: &Pool<Postgres>,
    crs_id: U256,
) -> anyhow::Result<bool> {
    has_crs_gen(db_pool, false, crs_id).await.map(|b| !b)
}

async fn has_crs(
    db_pool: &Pool<Postgres>,
    crs_id: U256,
) -> anyhow::Result<bool> {
    has_crs_gen(db_pool, true, crs_id).await
}

async fn has_crs_gen(
    db_pool: &Pool<Postgres>,
    retry: bool,
    crs_id: U256,
) -> anyhow::Result<bool> {
    for _ in 0..RETRY_EVENT_TO_DB {
        sleep(RETRY_DELAY).await;
        let rows = sqlx::query(
            "SELECT crs FROM crs WHERE crs_id = $1 AND status = 'active'",
        )
        .bind(&key_id_to_database_bytes(crs_id))
        .fetch_all(db_pool)
        .await?;
        if !rows.is_empty() {
            let expected_key_content = "key_bytes".as_bytes().to_vec();
            let crs: Vec<u8> = rows[0].try_get("crs")?;
            if crs == expected_key_content {
                return Ok(true);
            }
        }
        if !retry {
            break;
        }
    }
    Ok(false)
}

#[derive(Clone)]
pub struct AwsS3ClientMocked(Client);

#[async_trait]
impl AwsS3Interface for AwsS3ClientMocked {
    async fn get_bucket_key(
        &self,
        url: &str,
        bucket: &str,
        key: &str,
    ) -> anyhow::Result<bytes::Bytes> {
        let full_key = find_key(&self.0, url, bucket, key).await?;
        Ok(self
            .0
            .get_object()
            .bucket(bucket)
            .key(full_key)
            .send()
            .await?
            .body
            .collect()
            .await?
            .into_bytes())
    }
}

fn rules(
    buckets: Vec<&'static str>,
    keys_digests: Vec<KeyType>,
    key_id: U256,
    bad_content: bool,
    bad_key: bool,
) -> Vec<aws_smithy_mocks::Rule> {
    let mut rules = vec![];
    let mut keys = HashSet::<String>::new();
    for (i, &bucket) in buckets.iter().enumerate() {
        for key_type in &keys_digests {
            let key_type_str: &str = to_key_prefix(*key_type);
            let key_id_no_0x = key_id_to_aws_key(key_id);
            // mpc style PUB-p1
            let key = format!("PUB-p1{}/{}", key_type_str, key_id_no_0x);
            keys.insert(key.clone());
            eprintln!("Adding {}/{}", bucket, key);
            let get_object_rule =
                mock!(Client::get_object).match_requests(move |req| {
                    req.bucket() == Some(bucket) && req.key() == Some(&key)
                });
            let get_object_rule = if bad_key && i < 3 {
                // most bucket fails
                get_object_rule.then_error(|| {
                    let nsk = aws_sdk_s3::types::error::NoSuchKey::builder()
                        .message("")
                        .build();
                    GetObjectError::NoSuchKey(nsk)
                })
            } else {
                get_object_rule.then_output(move || {
                    GetObjectOutput::builder()
                        .body(ByteStream::from_static(if bad_content {
                            b"bad_key_bytes"
                        } else {
                            b"key_bytes"
                        }))
                        .build()
                })
            };
            rules.push(get_object_rule);
        }
    }
    for &bucket in &buckets {
        let key_id_no_0x = &format!("{key_id:064X}");
        // centralized style PUB-p1
        let key = format!("PUB/CRS/{key_id_no_0x}");
        keys.insert(key.clone());
        eprintln!("Adding {}/{}", bucket, key);
        let get_object_rule = mock!(Client::get_object)
            .match_requests(move |req| {
                req.bucket() == Some(bucket) && req.key() == Some(&key)
            })
            .then_output(|| {
                GetObjectOutput::builder()
                    .body(ByteStream::from_static(b"key_bytes"))
                    .build()
            });
        rules.push(get_object_rule);
    }

    for &bucket in &buckets {
        let keys = keys.clone();
        let get_object_rule = mock!(Client::list_objects_v2)
            .match_requests(|req| req.bucket() == Some(bucket))
            .then_output(move || {
                aws_sdk_s3::operation::list_objects_v2::ListObjectsV2Output::builder()
                    .set_contents(
                        keys.iter()
                            .map(move |k| {
                                Some(aws_sdk_s3::types::Object::builder().key(k).build())
                            })
                            .collect(),
                    )
                    .build()
            });
        rules.push(get_object_rule);
    }
    rules
}

const TEST_CHAIN_ID: u64 = 12345;

/// Stages KMSGeneration logs for a block, marks the source block finalized,
/// then runs the KMS materializer until the staging queues are drained.
async fn process_and_finalize_logs_at_block<P>(
    provider: &P,
    db_pool: &Pool<Postgres>,
    kms_address: Address,
    aws_s3_client: &AwsS3ClientMocked,
    block_hash: alloy::primitives::BlockHash,
) -> anyhow::Result<()>
where
    P: Provider<alloy::network::Ethereum>,
{
    let block = provider
        .get_block_by_hash(block_hash)
        .await?
        .expect("block exists");
    let filter = Filter::new().at_block_hash(block_hash).address(kms_address);
    let logs = provider.get_logs(&filter).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    process_kms_generation_logs(
        db_pool,
        kms_address,
        aws_s3_client,
        &logs,
        chain_id,
        block_hash.as_slice(),
    )
    .await?;
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, $3, 'finalized')
         ON CONFLICT (chain_id, block_hash)
         DO UPDATE SET block_status = EXCLUDED.block_status",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .bind(block.header.number as i64)
    .execute(db_pool)
    .await?;
    process_finalized_kms_generation_events_until_idle(
        db_pool,
        chain_id,
        aws_s3_client,
    )
    .await?;
    if let Some(err) =
        first_terminal_kms_error_at_block(db_pool, chain_id, block_hash).await?
    {
        anyhow::bail!(err);
    }
    Ok(())
}

async fn first_terminal_kms_error_at_block(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    block_hash: alloy::primitives::BlockHash,
) -> anyhow::Result<Option<String>> {
    let key_error = sqlx::query(
        "SELECT last_error
         FROM kms_key_activation_events
         WHERE chain_id = $1
           AND block_hash = $2
           AND download_status IN ('failed', 'invalid_event')
         ORDER BY sequence_number
         LIMIT 1",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_optional(db_pool)
    .await?;
    if let Some(row) = key_error {
        let error: Option<String> = row.try_get("last_error")?;
        return Ok(Some(
            error.unwrap_or_else(|| "Unknown key activation error".to_owned()),
        ));
    }

    let crs_error = sqlx::query(
        "SELECT last_error
         FROM kms_crs_activation_events
         WHERE chain_id = $1
           AND block_hash = $2
           AND download_status IN ('failed', 'invalid_event')
         ORDER BY sequence_number
         LIMIT 1",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_optional(db_pool)
    .await?;
    if let Some(row) = crs_error {
        let error: Option<String> = row.try_get("last_error")?;
        return Ok(Some(
            error.unwrap_or_else(|| "Unknown CRS activation error".to_owned()),
        ));
    }

    Ok(None)
}

#[tokio::test]
#[serial(db)]
async fn keygen_ok_simple() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, false, false);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    assert!(has_not_crs(&env.db_pool, key_id).await?);

    let txn_req = kms_generation.keygen(1).into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await?;
    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);

    let txn_req = kms_generation.crsgen().into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await?;
    assert!(has_crs(&env.db_pool, key_id).await?);
    Ok(())
}

/// Replaying the same staged block twice is a no-op thanks to the
/// (chain_id, transaction_hash, log_index) uniqueness on staging rows.
#[tokio::test]
#[serial(db)]
async fn keygen_idempotent_replay() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, false, false);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    let keygen_receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(keygen_receipt.status());
    let keygen_block = keygen_receipt.block_hash.expect("block hash");

    let crsgen_receipt = provider
        .send_transaction(kms_generation.crsgen().into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(crsgen_receipt.status());
    let crsgen_block = crsgen_receipt.block_hash.expect("block hash");

    // Emit another crsgen to validate ON CONFLICT DO NOTHING branch.
    let crsgen2_receipt = provider
        .send_transaction(kms_generation.crsgen().into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(crsgen2_receipt.status());
    let crsgen2_block = crsgen2_receipt.block_hash.expect("block hash");

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    assert!(has_not_crs(&env.db_pool, key_id).await?);

    // First pass: process each block once.
    for hash in [keygen_block, crsgen_block, crsgen2_block] {
        process_and_finalize_logs_at_block(
            &provider,
            &env.db_pool,
            *kms_generation.address(),
            &aws_s3_client,
            hash,
        )
        .await?;
    }

    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);
    assert!(has_crs(&env.db_pool, key_id).await?);

    // Second pass: same blocks again should be a no-op thanks to upsert.
    for hash in [keygen_block, crsgen_block, crsgen2_block] {
        process_and_finalize_logs_at_block(
            &provider,
            &env.db_pool,
            *kms_generation.address(),
            &aws_s3_client,
            hash,
        )
        .await?;
    }

    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);
    assert!(has_crs(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_reorged_replay_keeps_same_tx_log_index_across_block_hashes(
) -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let transaction_hash = vec![0x11_u8; 32];
    let old_block_hash = vec![0x22_u8; 32];
    let canonical_block_hash = vec![0x33_u8; 32];
    let key_id = key_id_to_database_bytes(U256::from(16));
    let s3_bucket_urls = vec!["s3://kms-node".to_owned()];

    sqlx::query(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            log_index,
            key_id_gw,
            key_digests,
            s3_bucket_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, '[]'::jsonb, $7)
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&old_block_hash)
    .bind(7_i64)
    .bind(&transaction_hash)
    .bind(0_i64)
    .bind(&key_id[..])
    .bind(&s3_bucket_urls)
    .execute(&env.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            log_index,
            key_id_gw,
            key_digests,
            s3_bucket_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6, '[]'::jsonb, $7)
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&canonical_block_hash)
    .bind(7_i64)
    .bind(&transaction_hash)
    .bind(0_i64)
    .bind(&key_id[..])
    .bind(&s3_bucket_urls)
    .execute(&env.db_pool)
    .await?;

    let row = sqlx::query(
        r#"
        SELECT COUNT(*) AS row_count, COUNT(DISTINCT block_hash) AS block_hash_count
        FROM kms_key_activation_events
        WHERE chain_id = $1
          AND transaction_hash = $2
          AND log_index = $3
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&transaction_hash)
    .bind(0_i64)
    .fetch_one(&env.db_pool)
    .await?;

    let row_count: i64 = row.try_get("row_count")?;
    let block_hash_count: i64 = row.try_get("block_hash_count")?;
    assert_eq!(row_count, 2);
    assert_eq!(block_hash_count, 2);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_finalization_restores_orphaned_canonical_activation_to_pending(
) -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let database = Database::new(&env.database_url, chain_id, 16).await?;
    let block_number = 9_i64;
    let old_block_hash = vec![0x44_u8; 32];
    let canonical_block_hash = vec![0x55_u8; 32];
    let transaction_hash = vec![0x66_u8; 32];
    let key_id = key_id_to_database_bytes(U256::from(16));
    let s3_bucket_urls = vec!["s3://kms-node".to_owned()];

    sqlx::query(
        r#"
        INSERT INTO host_chain_blocks_valid (
            chain_id,
            block_hash,
            block_number,
            block_status
        )
        VALUES
            ($1, $2, $3, 'finalized'),
            ($1, $4, $3, 'orphaned')
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&old_block_hash)
    .bind(block_number)
    .bind(&canonical_block_hash)
    .execute(&env.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            log_index,
            key_id_gw,
            key_digests,
            s3_bucket_urls,
            download_status,
            last_error
        )
        VALUES ($1, $2, $3, $4, $5, $6, '[]'::jsonb, $7, 'orphaned', 'reorged')
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&canonical_block_hash)
    .bind(block_number)
    .bind(&transaction_hash)
    .bind(0_i64)
    .bind(&key_id[..])
    .bind(&s3_bucket_urls)
    .execute(&env.db_pool)
    .await?;

    let mut tx = database.new_transaction().await?;
    database
        .update_block_as_finalized(
            &mut tx,
            block_number,
            &host_listener::cmd::block_history::BlockHash::from([0x55; 32]),
        )
        .await?;
    tx.commit().await?;

    let row = sqlx::query(
        r#"
        SELECT download_status, last_error
        FROM kms_key_activation_events
        WHERE chain_id = $1 AND block_hash = $2
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(&canonical_block_hash)
    .fetch_one(&env.db_pool)
    .await?;

    let download_status: String = row.try_get("download_status")?;
    let last_error: Option<String> = row.try_get("last_error")?;
    assert_eq!(download_status, "pending");
    assert_eq!(last_error, None);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_materialization_does_not_mark_staging_row_materialized_after_insert_noop(
) -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, false, false);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let key_id_bytes = key_id_to_database_bytes(key_id);

    let receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    let block = provider
        .get_block_by_hash(block_hash)
        .await?
        .expect("block exists");
    let filter = Filter::new()
        .at_block_hash(block_hash)
        .address(*kms_generation.address());
    let logs = provider.get_logs(&filter).await?;

    process_kms_generation_logs(
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        &logs,
        chain_id,
        block_hash.as_slice(),
    )
    .await?;

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, $3, 'finalized')",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .bind(block.header.number as i64)
    .execute(&env.db_pool)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO keys (
            key_id,
            key_id_gw,
            pks_key,
            sks_key,
            sns_pk,
            status,
            chain_id,
            block_hash
        )
        VALUES ($1, $2, $3, $4, NULL, 'pending', $5, $6)
        "#,
    )
    .bind(Vec::<u8>::new())
    .bind(&key_id_bytes[..])
    .bind(b"existing_public_key".as_slice())
    .bind(b"existing_server_key".as_slice())
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .execute(&env.db_pool)
    .await?;

    let result = tokio::time::timeout(
        Duration::from_secs(2),
        process_finalized_kms_generation_events_until_idle(
            &env.db_pool,
            chain_id,
            &aws_s3_client,
        ),
    )
    .await
    .map_err(|_| anyhow::anyhow!("KMS materializer did not become idle"))?;
    let err =
        result.expect_err("expected key insert no-op to fail materialization");
    assert!(err
        .to_string()
        .contains("ActivateKey insert did not create an active key row"),);

    let download_status: String = sqlx::query_scalar(
        r#"
        SELECT download_status
        FROM kms_key_activation_events
        WHERE chain_id = $1 AND block_hash = $2
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_one(&env.db_pool)
    .await?;

    let active_key_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM keys
        WHERE key_id_gw = $1 AND status = 'active'
        "#,
    )
    .bind(&key_id_bytes[..])
    .fetch_one(&env.db_pool)
    .await?;

    let key_row_count: i64 = sqlx::query_scalar(
        r#"
        SELECT COUNT(*)
        FROM keys
        WHERE chain_id = $1 AND block_hash = $2 AND key_id_gw = $3
        "#,
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .bind(&key_id_bytes[..])
    .fetch_one(&env.db_pool)
    .await?;

    assert_eq!(key_row_count, 1);
    assert_eq!(active_key_count, 0);
    assert_ne!(download_status, "materialized");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_compromised_key() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    // bad_content = true: downloaded bytes don't hash to the expected digest,
    // so the listener must swallow the DigestMismatchError and not insert any
    // keys.
    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, true, false);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

    let txn_req = kms_generation.keygen(1).into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await?;

    assert!(env.contains_log("Invalid Key digest"));
    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_bad_key_or_bucket() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    // bad_key = true: 3 out of 4 buckets return NoSuchKey for the key, but the
    // download_key_from_s3 helper rotates through buckets and eventually
    // succeeds on the last one.
    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, false, true);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

    let txn_req = kms_generation.keygen(1).into_transaction_request();
    let pending_txn = provider.send_transaction(txn_req).await?;
    let receipt = pending_txn.get_receipt().await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await?;

    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_only_public_or_server_key() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let keys_digests = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(16);

    let rules_ref: Vec<_> = rules(buckets, keys_digests, key_id, false, false);
    let s3 = mock_client!(aws_sdk_s3, RuleMode::MatchAny, &rules_ref);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client = AwsS3ClientMocked(s3);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    assert!(has_not_crs(&env.db_pool, key_id).await?);

    // keygen_public_key emits an ActivateKey event with only the public key
    // digest. activate_key should bail with "Incomplete key record", which
    // process_kms_generation_logs propagates. We assert the error is returned.
    let receipt = provider
        .send_transaction(
            kms_generation
                .keygen_public_key()
                .into_transaction_request(),
        )
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    let result = process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await;
    assert!(result.is_err(), "expected incomplete-key-record error");
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Incomplete key record"),
        "unexpected error message",
    );

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

    // Same check for keygen_server_key.
    let receipt = provider
        .send_transaction(
            kms_generation
                .keygen_server_key()
                .into_transaction_request(),
        )
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");
    let result = process_and_finalize_logs_at_block(
        &provider,
        &env.db_pool,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
    )
    .await;
    assert!(result.is_err(), "expected incomplete-key-record error");
    assert!(
        result
            .unwrap_err()
            .to_string()
            .contains("Incomplete key record"),
        "unexpected error message",
    );

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    Ok(())
}
