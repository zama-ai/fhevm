use std::{
    collections::HashMap,
    sync::{Arc, OnceLock, RwLock},
    time::Duration,
};

use alloy::{
    network::EthereumWallet,
    node_bindings::{Anvil, AnvilInstance},
    primitives::{Address, BlockHash, U256},
    providers::{Provider, ProviderBuilder, WsConnect},
    rpc::types::Filter,
    signers::local::PrivateKeySigner,
    sol,
};
use async_trait::async_trait;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::utils::DatabaseURL;
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;
use host_listener::kms_generation::aws_s3::AwsS3Interface;
use host_listener::kms_generation::{
    key_id_to_aws_key, key_id_to_database_bytes,
    process_kms_generation_activations, to_key_prefix, KeyType,
};
use serial_test::serial;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres, Row};
use test_harness::db_utils::ACL_CONTRACT_ADDR;
use test_harness::instance::ImportMode;
use tokio::time::sleep;
use tokio_util::bytes::{self, Bytes};
use tracing::{info, Level};
use tracing_subscriber::fmt::{writer::MakeWriterExt, MakeWriter};

sol!(
    #[sol(rpc)]
    KMSGenerationMock,
    "artifacts/KMSGenerationTest.sol/KMSGenerationTest.json"
);

static TEST_LOGS: OnceLock<Arc<RwLock<String>>> = OnceLock::new();

const TEST_CHAIN_ID: u64 = 12345;
const TEST_KEY_ID: u64 = 16;
const RETRY_EVENT_TO_DB: u64 = 3;
const RETRY_DELAY: Duration = Duration::from_millis(100);
const MATERIALIZER_ACTIVATION_STEPS: usize = 2;
const SEEDED_PUBLIC_KEY: &[u8] = b"seed_public_key";
const SEEDED_SERVER_KEY: &[u8] = b"seed_server_key";
const SEEDED_CRS: &[u8] = b"seed_crs";
const MATERIALIZED_KEY_BYTES: &[u8] = b"key_bytes";

#[derive(Clone)]
struct TestLogs {
    logs: Arc<RwLock<String>>,
}

impl TestLogs {
    fn new() -> Self {
        let logs =
            TEST_LOGS.get_or_init(|| Arc::new(RwLock::new(String::new())));
        logs.write().expect("test log lock").clear();
        Self { logs: logs.clone() }
    }

    fn add(&mut self, data: &[u8]) {
        let data = String::from_utf8_lossy(data).into_owned();
        *self.logs.write().expect("test log lock") += &data;
    }

    fn contains(&self, needle: &str) -> bool {
        self.logs.read().expect("test log lock").contains(needle)
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
    _test_instance: Option<test_harness::instance::DBInstance>,
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

        let mut test_instance = None;
        if std::env::var("FORCE_DATABASE_URL").is_err() {
            let instance = test_harness::instance::setup_test_db(
                ImportMode::WithKeysNoSns,
            )
            .await
            .expect("valid db instance");
            database_url = instance.db_url.clone();
            test_instance = Some(instance);
        }

        let db_pool = PgPoolOptions::new()
            .max_connections(16)
            .acquire_timeout(Duration::from_secs(5))
            .connect(database_url.as_str())
            .await?;

        seed_test_rows(&db_pool).await?;

        let anvil = Anvil::new().block_time(1).chain_id(TEST_CHAIN_ID).spawn();
        let signer: PrivateKeySigner = anvil.keys()[0].clone().into();
        let wallet = signer.clone().into();

        Ok(Self {
            wallet,
            database_url,
            db_pool,
            _test_instance: test_instance,
            anvil,
            test_logs,
        })
    }

    fn contains_log(&self, needle: &str) -> bool {
        self.test_logs.contains(needle)
    }
}

async fn seed_test_rows(db_pool: &Pool<Postgres>) -> anyhow::Result<()> {
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let key_id = key_id_to_database_bytes(U256::from(TEST_KEY_ID));

    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address)
         VALUES ($1, 'test chain', $2)
         ON CONFLICT (chain_id) DO NOTHING",
    )
    .bind(chain_id.as_i64())
    .bind(ACL_CONTRACT_ADDR)
    .execute(db_pool)
    .await?;

    sqlx::query("DELETE FROM host_chain_blocks_valid WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(db_pool)
        .await?;
    sqlx::query("DELETE FROM kms_key_activation_events WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(db_pool)
        .await?;
    sqlx::query("DELETE FROM kms_crs_activation_events WHERE chain_id = $1")
        .bind(chain_id.as_i64())
        .execute(db_pool)
        .await?;
    sqlx::query("DELETE FROM keys WHERE key_id_gw = $1")
        .bind(key_id)
        .execute(db_pool)
        .await?;
    sqlx::query("DELETE FROM crs WHERE crs_id = $1")
        .bind(key_id)
        .execute(db_pool)
        .await?;

    sqlx::query(
        "INSERT INTO keys (
            key_id,
            key_id_gw,
            pks_key,
            sks_key,
            cks_key,
            sns_pk,
            chain_id,
            block_hash
        )
        VALUES ($1, $2, $3, $4, NULL, NULL, NULL, NULL)",
    )
    .bind(key_id)
    .bind(key_id)
    .bind(SEEDED_PUBLIC_KEY)
    .bind(SEEDED_SERVER_KEY)
    .execute(db_pool)
    .await?;

    sqlx::query(
        "INSERT INTO crs (crs_id, crs, chain_id, block_hash)
         VALUES ($1, $2, NULL, NULL)",
    )
    .bind(key_id)
    .bind(SEEDED_CRS)
    .execute(db_pool)
    .await?;

    Ok(())
}

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
        let rows = sqlx::query("SELECT pks_key FROM keys WHERE key_id_gw = $1")
            .bind(key_id_to_database_bytes(key_id))
            .fetch_all(db_pool)
            .await?;
        if !rows.is_empty() {
            let pks_key: Vec<u8> = rows[0].try_get("pks_key")?;
            if pks_key == MATERIALIZED_KEY_BYTES {
                return Ok(true);
            } else {
                info!(
                    "Found public key for key_id {}, but it does not match materialized key bytes {} vs {MATERIALIZED_KEY_BYTES:?}",
                    key_id,
                    String::from_utf8_lossy(&pks_key)
                );
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
        let rows = sqlx::query("SELECT sks_key FROM keys WHERE key_id_gw = $1")
            .bind(key_id_to_database_bytes(key_id))
            .fetch_all(db_pool)
            .await?;
        if !rows.is_empty() {
            let sks_key: Vec<u8> = rows[0].try_get("sks_key")?;
            if sks_key == MATERIALIZED_KEY_BYTES {
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
        let rows = sqlx::query("SELECT crs FROM crs WHERE crs_id = $1")
            .bind(key_id_to_database_bytes(crs_id))
            .fetch_all(db_pool)
            .await?;
        if !rows.is_empty() {
            let crs: Vec<u8> = rows[0].try_get("crs")?;
            if crs == MATERIALIZED_KEY_BYTES {
                return Ok(true);
            }
        }
        if !retry {
            break;
        }
    }
    Ok(false)
}

async fn read_key_row(
    db_pool: &Pool<Postgres>,
    key_id: U256,
) -> anyhow::Result<(Vec<u8>, Vec<u8>)> {
    let row =
        sqlx::query("SELECT pks_key, sks_key FROM keys WHERE key_id_gw = $1")
            .bind(key_id_to_database_bytes(key_id))
            .fetch_one(db_pool)
            .await?;
    Ok((row.try_get("pks_key")?, row.try_get("sks_key")?))
}

#[derive(Clone, Default)]
struct AwsS3ClientMocked {
    bucket_objects: Arc<HashMap<String, HashMap<String, Bytes>>>,
}

impl AwsS3ClientMocked {
    fn new(
        buckets: &[&'static str],
        key_types: &[KeyType],
        key_id: U256,
        bad_content: bool,
        bad_key: bool,
    ) -> Self {
        let mut bucket_objects =
            HashMap::<String, HashMap<String, Bytes>>::new();
        let key_bytes = if bad_content {
            Bytes::from_static(b"bad_key_bytes")
        } else {
            Bytes::from_static(MATERIALIZED_KEY_BYTES)
        };

        for (index, bucket) in buckets.iter().enumerate() {
            let bucket_objects_for_bucket =
                bucket_objects.entry((*bucket).to_owned()).or_default();

            for key_type in key_types {
                if bad_key && index < 3 {
                    continue;
                }
                let full_key = format!(
                    "PUB-p1{}/{}",
                    to_key_prefix(*key_type),
                    key_id_to_aws_key(key_id)
                );
                bucket_objects_for_bucket.insert(full_key, key_bytes.clone());
            }

            let crs_key = format!("PUB/CRS/{}", key_id_to_aws_key(key_id));
            bucket_objects_for_bucket
                .insert(crs_key, Bytes::from_static(MATERIALIZED_KEY_BYTES));
        }

        Self {
            bucket_objects: Arc::new(bucket_objects),
        }
    }
}

#[async_trait]
impl AwsS3Interface for AwsS3ClientMocked {
    async fn get_bucket_key(
        &self,
        _url: &str,
        bucket: &str,
        key_suffix: &str,
    ) -> anyhow::Result<bytes::Bytes> {
        let Some(objects) = self.bucket_objects.get(bucket) else {
            anyhow::bail!("Bucket {bucket} not found");
        };

        let mut candidates = objects
            .keys()
            .filter(|candidate| candidate.ends_with(key_suffix))
            .cloned()
            .collect::<Vec<_>>();
        candidates.sort();

        let Some(full_key) = candidates.into_iter().next() else {
            anyhow::bail!("Key {key_suffix} not found in bucket {bucket}");
        };

        Ok(objects
            .get(&full_key)
            .expect("mocked object exists")
            .clone())
    }
}

async fn materialize_kms_generation_steps(
    db: &Database,
    aws_s3_client: &AwsS3ClientMocked,
    steps: usize,
) -> anyhow::Result<()> {
    let pool = db.pool().await;
    for _ in 0..steps {
        process_kms_generation_activations(pool.clone(), aws_s3_client.clone())
            .await?;
    }
    Ok(())
}

async fn process_and_finalize_logs_at_block<P>(
    provider: &P,
    db: &mut Database,
    kms_address: Address,
    aws_s3_client: &AwsS3ClientMocked,
    block_hash: BlockHash,
    materializer_steps: usize,
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
    let block_logs = BlockLogs {
        summary: block.header.into(),
        catchup: false,
        finalized: false,
        logs,
    };
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
    };

    ingest_block_logs(
        chain_id,
        db,
        &block_logs,
        &None,
        &None,
        &Some(kms_address),
        options,
    )
    .await?;

    let mut tx = db.new_transaction().await?;
    db.mark_block_as_valid(&mut tx, &block_logs.summary, true)
        .await?;
    tx.commit().await?;

    materialize_kms_generation_steps(db, aws_s3_client, materializer_steps)
        .await?;

    Ok(())
}

async fn key_activation_status(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    block_hash: BlockHash,
) -> anyhow::Result<Option<String>> {
    Ok(sqlx::query_scalar(
        "SELECT status
         FROM kms_key_activation_events
         WHERE chain_id = $1 AND block_hash = $2
         LIMIT 1",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_optional(db_pool)
    .await?)
}

async fn key_activation_last_error(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    block_hash: BlockHash,
) -> anyhow::Result<Option<String>> {
    let row = sqlx::query(
        "SELECT last_error
         FROM kms_key_activation_events
         WHERE chain_id = $1 AND block_hash = $2
         LIMIT 1",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_optional(db_pool)
    .await?;
    Ok(row
        .map(|row| row.try_get("last_error"))
        .transpose()?
        .flatten())
}

async fn key_activation_count_for_block(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    block_hash: BlockHash,
) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM kms_key_activation_events
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_one(db_pool)
    .await?)
}

async fn crs_activation_count_for_block(
    db_pool: &Pool<Postgres>,
    chain_id: ChainId,
    block_hash: BlockHash,
) -> anyhow::Result<i64> {
    Ok(sqlx::query_scalar(
        "SELECT COUNT(*)
         FROM kms_crs_activation_events
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(chain_id.as_i64())
    .bind(block_hash.as_slice())
    .fetch_one(db_pool)
    .await?)
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
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, false, false);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);
    assert!(has_not_crs(&env.db_pool, key_id).await?);

    let receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());

    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let mut database = Database::new(&env.database_url, chain_id, 10).await?;
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        receipt.block_hash.expect("receipt has block hash"),
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);

    let receipt = provider
        .send_transaction(kms_generation.crsgen().into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());

    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        receipt.block_hash.expect("receipt has block hash"),
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    assert!(has_crs(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_idempotent_replay() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, false, false);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;

    let keygen_receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    let keygen_block = keygen_receipt.block_hash.expect("block hash");

    let crsgen_receipt = provider
        .send_transaction(kms_generation.crsgen().into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    let crsgen_block = crsgen_receipt.block_hash.expect("block hash");

    let mut database = Database::new(&env.database_url, chain_id, 10).await?;
    for block_hash in [keygen_block, crsgen_block] {
        process_and_finalize_logs_at_block(
            &provider,
            &mut database,
            *kms_generation.address(),
            &aws_s3_client,
            block_hash,
            MATERIALIZER_ACTIVATION_STEPS,
        )
        .await?;
    }

    assert_eq!(
        key_activation_count_for_block(&env.db_pool, chain_id, keygen_block)
            .await?,
        1,
    );
    assert_eq!(
        crs_activation_count_for_block(&env.db_pool, chain_id, crsgen_block)
            .await?,
        1,
    );

    for block_hash in [keygen_block, crsgen_block] {
        process_and_finalize_logs_at_block(
            &provider,
            &mut database,
            *kms_generation.address(),
            &aws_s3_client,
            block_hash,
            MATERIALIZER_ACTIVATION_STEPS,
        )
        .await?;
    }

    assert_eq!(
        key_activation_count_for_block(&env.db_pool, chain_id, keygen_block)
            .await?,
        1,
    );
    assert_eq!(
        crs_activation_count_for_block(&env.db_pool, chain_id, crsgen_block)
            .await?,
        1,
    );
    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);
    assert!(has_crs(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_reorged_replay_keeps_same_transaction_hash_across_block_hashes(
) -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let transaction_hash = vec![0x11_u8; 32];
    let old_block_hash = vec![0x22_u8; 32];
    let canonical_block_hash = vec![0x33_u8; 32];
    let key_id = key_id_to_database_bytes(U256::from(TEST_KEY_ID));
    let storage_urls = vec!["s3://kms-node".to_owned()];

    for block_hash in [&old_block_hash, &canonical_block_hash] {
        sqlx::query(
            "INSERT INTO kms_key_activation_events (
                chain_id,
                block_hash,
                block_number,
                transaction_hash,
                key_id,
                storage_urls
            )
            VALUES ($1, $2, $3, $4, $5, $6)",
        )
        .bind(chain_id.as_i64())
        .bind(block_hash)
        .bind(7_i64)
        .bind(&transaction_hash)
        .bind(&key_id[..])
        .bind(&storage_urls)
        .execute(&env.db_pool)
        .await?;
    }

    let row = sqlx::query(
        "SELECT
            COUNT(*) AS row_count,
            COUNT(DISTINCT block_hash) AS block_hash_count
         FROM kms_key_activation_events
         WHERE chain_id = $1
           AND transaction_hash = $2",
    )
    .bind(chain_id.as_i64())
    .bind(&transaction_hash)
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
async fn keygen_orphaned_activation_is_cancelled() -> anyhow::Result<()> {
    let env = TestEnvironment::new().await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let database = Database::new(&env.database_url, chain_id, 16).await?;
    let orphaned_block_hash = vec![0x55_u8; 32];
    let transaction_hash = vec![0x66_u8; 32];
    let key_id = key_id_to_database_bytes(U256::from(TEST_KEY_ID));
    let storage_urls =
        vec!["https://s3.region.amazonaws.com/test-bucket1".to_owned()];

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (
            chain_id,
            block_hash,
            block_number,
            block_status
        )
        VALUES ($1, $2, $3, 'orphaned')",
    )
    .bind(chain_id.as_i64())
    .bind(&orphaned_block_hash)
    .bind(9_i64)
    .execute(&env.db_pool)
    .await?;

    sqlx::query(
        "INSERT INTO kms_key_activation_events (
            chain_id,
            block_hash,
            block_number,
            transaction_hash,
            key_id,
            storage_urls
        )
        VALUES ($1, $2, $3, $4, $5, $6)",
    )
    .bind(chain_id.as_i64())
    .bind(&orphaned_block_hash)
    .bind(9_i64)
    .bind(&transaction_hash)
    .bind(&key_id[..])
    .bind(&storage_urls)
    .execute(&env.db_pool)
    .await?;

    materialize_kms_generation_steps(
        &database,
        &AwsS3ClientMocked::default(),
        1,
    )
    .await?;

    let status: String = sqlx::query_scalar(
        "SELECT status
         FROM kms_key_activation_events
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(chain_id.as_i64())
    .bind(&orphaned_block_hash)
    .fetch_one(&env.db_pool)
    .await?;
    assert_eq!(status, "cancelled");
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_missing_target_row_stays_ready_after_materialization(
) -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);
    let key_id_bytes = key_id_to_database_bytes(key_id);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, false, false);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;

    sqlx::query("DELETE FROM keys WHERE key_id_gw = $1")
        .bind(key_id_bytes)
        .execute(&env.db_pool)
        .await?;

    let receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");

    let mut database = Database::new(&env.database_url, chain_id, 16).await?;
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    assert_eq!(
        key_activation_status(&env.db_pool, chain_id, block_hash).await?,
        Some("ready".to_owned()),
    );
    assert_eq!(
        key_activation_last_error(&env.db_pool, chain_id, block_hash).await?,
        None,
    );

    let key_row_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM keys WHERE key_id_gw = $1")
            .bind(key_id_bytes)
            .fetch_one(&env.db_pool)
            .await?;
    assert_eq!(key_row_count, 0);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_compromised_key_records_last_error() -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, true, false);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

    let receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());
    let block_hash = receipt.block_hash.expect("receipt has block hash");

    let mut database = Database::new(&env.database_url, chain_id, 16).await?;
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        block_hash,
        1,
    )
    .await?;

    assert!(env.contains_log("Invalid Key digest"));
    assert_eq!(
        key_activation_status(&env.db_pool, chain_id, block_hash).await?,
        Some("pending".to_owned()),
    );
    let last_error =
        key_activation_last_error(&env.db_pool, chain_id, block_hash).await?;
    assert!(last_error
        .as_deref()
        .unwrap_or_default()
        .contains("Invalid Key digest"),);
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
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, false, true);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

    let receipt = provider
        .send_transaction(kms_generation.keygen(1).into_transaction_request())
        .await?
        .get_receipt()
        .await?;
    assert!(receipt.status());

    let mut database = Database::new(&env.database_url, chain_id, 16).await?;
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        receipt.block_hash.expect("receipt has block hash"),
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    assert!(has_public_key(&env.db_pool, key_id).await?);
    assert!(has_server_key(&env.db_pool, key_id).await?);
    Ok(())
}

#[tokio::test]
#[serial(db)]
async fn keygen_only_public_or_server_key_updates_partial_row(
) -> anyhow::Result<()> {
    let buckets = vec![
        "test-bucket1",
        "test-bucket2",
        "test-bucket3",
        "test-bucket4",
    ];
    let key_types = vec![KeyType::PublicKey, KeyType::ServerKey];
    let key_id = U256::from(TEST_KEY_ID);

    let env = TestEnvironment::new().await?;
    let provider = ProviderBuilder::new()
        .wallet(env.wallet.clone())
        .connect_ws(WsConnect::new(env.anvil.ws_endpoint_url()))
        .await?;
    let aws_s3_client =
        AwsS3ClientMocked::new(&buckets, &key_types, key_id, false, false);
    let kms_generation = KMSGenerationMock::deploy(&provider).await?;
    let chain_id = ChainId::try_from(TEST_CHAIN_ID)?;
    let mut database = Database::new(&env.database_url, chain_id, 16).await?;

    assert!(has_not_public_key(&env.db_pool, key_id).await?);
    assert!(has_not_server_key(&env.db_pool, key_id).await?);

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
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        receipt.block_hash.expect("receipt has block hash"),
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    let (public_key, server_key) = read_key_row(&env.db_pool, key_id).await?;
    assert_eq!(public_key, MATERIALIZED_KEY_BYTES);
    assert_eq!(server_key, SEEDED_SERVER_KEY);

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
    process_and_finalize_logs_at_block(
        &provider,
        &mut database,
        *kms_generation.address(),
        &aws_s3_client,
        receipt.block_hash.expect("receipt has block hash"),
        MATERIALIZER_ACTIVATION_STEPS,
    )
    .await?;

    let (public_key, server_key) = read_key_row(&env.db_pool, key_id).await?;
    assert_eq!(public_key, MATERIALIZED_KEY_BYTES);
    assert_eq!(server_key, MATERIALIZED_KEY_BYTES);
    Ok(())
}
