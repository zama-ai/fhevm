use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};

use alloy::{
    primitives::{Address, B256, U256},
    sol_types::SolEvent,
};
use consumer::{AckDecision, BlockPayload};
use fhevm_engine_common::{
    chain_id::ChainId, utils::HeartBeat, versioning::StackMode,
};
use primitives::event::{BlockFlow, IndexedLog, TransactionPayload};
use sqlx::PgPool;
use tokio::sync::mpsc;
use tokio_util::sync::CancellationToken;

use crate::{
    consumer::{
        catchup::{LiveReference, ManualCatchupArgs},
        ingestion::BlockIngestor,
        ConsumerConfig,
    },
    contracts::KMSGeneration,
    database::{ingest::IngestOptions, tfhe_event_propagate::Database},
    kms_generation::{
        aws_s3::AwsS3Interface,
        digest::{digest_crs, digest_key},
    },
};

#[derive(Clone)]
struct MaterialStore(Arc<AtomicBool>);

#[async_trait::async_trait]
impl AwsS3Interface for MaterialStore {
    async fn get_bucket_key(
        &self,
        _: &str,
        _: &str,
        _: &str,
    ) -> anyhow::Result<tokio_util::bytes::Bytes> {
        if !self.0.load(Ordering::SeqCst) {
            anyhow::bail!("temporary storage outage");
        }
        Ok(tokio_util::bytes::Bytes::from_static(b"key_bytes"))
    }
}

async fn ingestor(db: Database, cancel: CancellationToken) -> BlockIngestor {
    let chain_id = db.chain_id;
    let (live_reference, _) = LiveReference::new(chain_id.as_u64());
    let (drift_tx, _) = mpsc::unbounded_channel();
    BlockIngestor {
        db,
        chain_id,
        config: ConsumerConfig {
            manual_catchup: ManualCatchupArgs::default(),
            url: String::new(),
            acl_address: Address::ZERO,
            tfhe_address: Address::repeat_byte(1),
            kms_generation_address: Address::repeat_byte(2),
            protocol_config_address: None,
            confidential_bridge_address: None,
            database_url: Default::default(),
            database_retry_interval: std::time::Duration::from_millis(1),
            service_name: "kms-finality-test".into(),
            health_port: 0,
            dependence_cache_size: 16,
            dependence_by_connexity: false,
            dependence_cross_block: false,
            dependent_ops_max_per_chain: 1,
            chain_id: chain_id.to_string(),
            gcs_mode: false,
            disable_synthetic_ops: true,
            canonical_protocol_config_chain_id: None,
        },
        options: IngestOptions {
            dependence_by_connexity: false,
            dependence_cross_block: false,
            dependent_ops_max_per_chain: 1,
            is_protocol_config_listener: false,
            disable_synthetic_ops: true,
        },
        live_reference,
        checkpoint: None,
        drift_tx,
        mode: StackMode::new(false),
        tick: HeartBeat::new(),
        cancel,
        in_flight_handlers: tokio::sync::RwLock::new(()),
    }
}

fn activation(number: u64, flow: BlockFlow, crs: bool) -> BlockPayload {
    use fhevm_host_bindings::kms_generation::IKMSGeneration::KeyDigest;
    let data = if crs {
        KMSGeneration::ActivateCrs {
            crsId: U256::from(number),
            crsDigest: digest_crs(b"key_bytes").into(),
            kmsNodeStorageUrls: vec!["https://keys.s3.example.com".into()],
        }
        .encode_log_data()
    } else {
        KMSGeneration::ActivateKey {
            keyId: U256::from(number),
            existingKeyId: U256::ZERO,
            kmsNodeStorageUrls: vec!["https://keys.s3.example.com".into()],
            keyDigests: vec![0, 1]
                .into_iter()
                .map(|key_type| KeyDigest {
                    keyType: key_type,
                    digest: digest_key(b"key_bytes").into(),
                })
                .collect(),
        }
        .encode_log_data()
    };
    BlockPayload {
        chain_id: 12345,
        flow,
        block_number: number,
        block_hash: B256::from(U256::from(number)),
        parent_hash: B256::from(U256::from(number - 1)),
        timestamp: 1_700_000_000,
        transactions: vec![TransactionPayload {
            from: Address::ZERO,
            to: Some(Address::repeat_byte(2)),
            hash: B256::from(U256::from(number)),
            transaction_index: 0,
            value: U256::ZERO,
            data: Default::default(),
            logs: vec![IndexedLog {
                address: Address::repeat_byte(2),
                log_index: 0,
                topics: data.topics().to_vec(),
                data: data.data,
            }],
        }],
    }
}

async fn status(pool: &PgPool, table: &str, number: u64) -> String {
    sqlx::query_scalar(&format!("SELECT status FROM {table} WHERE chain_id = 12345 AND block_number = $1"))
        .bind(number as i64).fetch_one(pool).await.unwrap()
}

async fn wait_status(pool: &PgPool, table: &str, number: u64, expected: &str) {
    tokio::time::timeout(std::time::Duration::from_secs(25), async {
        while status(pool, table, number).await != expected {
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    })
    .await
    .unwrap();
}

#[tokio::test]
async fn kms_retries_on_quiet_chain_and_finalized_delivery_recovers_missed_events(
) {
    use test_harness::instance::{setup_test_db, ImportMode};
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let db = Database::new_with_gcs_mode(
        &instance.db_url,
        ChainId::try_from(12345_u64).unwrap(),
        16,
        false,
    )
    .await
    .unwrap();
    let pool = db.pool().await;
    let cancel = CancellationToken::new();
    let other_db = Database::new_with_gcs_mode(
        &instance.db_url,
        ChainId::try_from(12346_u64).unwrap(),
        16,
        false,
    )
    .await
    .unwrap();
    let other = ingestor(other_db, CancellationToken::new()).await;
    let mut other_key = activation(100, BlockFlow::Final, false);
    other_key.chain_id = 12346;
    other.ingest(other_key).await.unwrap();
    let ingestor = Arc::new(ingestor(db, cancel.clone()).await);
    let key = activation(100, BlockFlow::Live, false);
    assert!(matches!(
        ingestor.ingest(key.clone()).await.unwrap(),
        AckDecision::Ack
    ));
    let store = MaterialStore(Arc::new(AtomicBool::new(false)));
    let worker = ingestor.clone();
    let worker_store = store.clone();
    let worker_cancel = cancel.clone();
    let task = tokio::spawn(async move {
        worker.process_kms(worker_store, worker_cancel).await;
    });
    tokio::time::timeout(std::time::Duration::from_secs(10), async {
        loop {
            let retries: i32 = sqlx::query_scalar("SELECT retry_count FROM kms_key_activation_events WHERE block_number = 100 AND chain_id = 12345").fetch_one(&pool).await.unwrap();
            if retries > 0 { break; }
            tokio::time::sleep(std::time::Duration::from_millis(50)).await;
        }
    }).await.unwrap();
    store.0.store(true, Ordering::SeqCst);
    wait_status(&pool, "kms_key_activation_events", 100, "ready").await;
    // An observed fork has a KMS event, but the finalized sibling is empty.
    let orphan = activation(200, BlockFlow::Live, true);
    ingestor.ingest(orphan.clone()).await.unwrap();
    let mut canonical = orphan;
    canonical.flow = BlockFlow::Final;
    canonical.block_hash = B256::repeat_byte(55);
    canonical.transactions.clear();
    ingestor.ingest(canonical).await.unwrap();
    wait_status(&pool, "kms_crs_activation_events", 200, "cancelled").await;
    let keys: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM keys")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(keys, 0, "downloaded material must wait for block finality");
    let mut notifications = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .unwrap();
    notifications.listen("new_host_block").await.unwrap();
    let mut final_key = key;
    final_key.flow = BlockFlow::Final;
    ingestor.ingest(final_key.clone()).await.unwrap();
    ingestor.ingest(final_key).await.unwrap();
    tokio::time::timeout(
        std::time::Duration::from_secs(2),
        notifications.recv(),
    )
    .await
    .unwrap()
    .unwrap();
    drop(notifications);
    // The CRS event was missed by live ingestion entirely.
    let crs = activation(101, BlockFlow::FinalCatchup, true);
    ingestor.ingest(crs.clone()).await.unwrap();
    ingestor.ingest(crs).await.unwrap();
    wait_status(&pool, "kms_key_activation_events", 100, "activated").await;
    wait_status(&pool, "kms_crs_activation_events", 101, "activated").await;
    let counts: (i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM keys), (SELECT COUNT(*) FROM crs)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (1, 1));
    let finalized: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = 12345 AND block_status = 'finalized'").fetch_one(&pool).await.unwrap();
    assert_eq!(finalized, 3);
    let other_status: String = sqlx::query_scalar(
        "SELECT status FROM kms_key_activation_events WHERE chain_id = 12346",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(
        other_status, "pending",
        "a consumer must not process another chain's KMS events"
    );
    // A finalized child contradicting its finalized parent must roll back its event.
    let mut bad = activation(102, BlockFlow::Final, true);
    bad.parent_hash = B256::repeat_byte(99);
    assert!(ingestor.ingest(bad).await.is_err());
    let pool = ingestor.db.pool().await;
    let bad_rows: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kms_crs_activation_events WHERE block_number = 102").fetch_one(&pool).await.unwrap();
    assert_eq!(bad_rows, 0);
    // Block a real handler on its database access, then verify shutdown drains it.
    let pool_guard = ingestor.db.pool.write().await;
    let handler = ingestor.clone();
    let blocked = tokio::spawn(async move {
        handler.ingest(activation(103, BlockFlow::Live, true)).await
    });
    tokio::time::timeout(std::time::Duration::from_secs(2), async {
        while ingestor.in_flight_handlers.try_write().is_ok() {
            tokio::task::yield_now().await;
        }
    })
    .await
    .unwrap();
    tokio::time::timeout(std::time::Duration::from_secs(2), ingestor.stop())
        .await
        .unwrap();
    assert!(matches!(blocked.await.unwrap().unwrap(), AckDecision::Ack));
    // A late handler must exit even while database access remains blocked.
    assert!(matches!(
        tokio::time::timeout(
            std::time::Duration::from_secs(2),
            ingestor.ingest(activation(104, BlockFlow::Live, true))
        )
        .await
        .unwrap()
        .unwrap(),
        AckDecision::Ack
    ));
    drop(pool_guard);
    tokio::time::timeout(std::time::Duration::from_secs(2), task)
        .await
        .unwrap()
        .unwrap();
}
