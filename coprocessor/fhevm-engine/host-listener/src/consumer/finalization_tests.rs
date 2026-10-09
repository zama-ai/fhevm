use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use std::time::Duration;

use alloy::{
    primitives::{Address, B256, U256},
    sol_types::SolEvent,
};
use consumer::{AckDecision, BlockPayload};
use fhevm_engine_common::chain_id::ChainId;
use primitives::event::{BlockFlow, IndexedLog, TransactionPayload};
use sqlx::PgPool;

use super::{ingest_payload, ConsumerConfig};
use crate::{
    contracts::KMSGeneration,
    database::{ingest::IngestOptions, tfhe_event_propagate::Database},
    kms_generation::{
        aws_s3::AwsS3Interface,
        digest::{digest_crs, digest_key},
    },
};

const CHAIN_ID: u64 = 12345;
const KMS_ADDRESS: Address = Address::repeat_byte(2);

/// Serves the expected material once `available` is set.
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

fn config() -> ConsumerConfig {
    ConsumerConfig {
        url: String::new(),
        acl_address: Address::ZERO,
        tfhe_address: Address::repeat_byte(1),
        kms_generation_address: Some(KMS_ADDRESS),
        protocol_config_address: None,
        confidential_bridge_address: None,
        database_url: Default::default(),
        database_retry_interval: Duration::from_millis(1),
        migrate_from_service_name: None,
        health_port: 0,
        dependence_cache_size: 16,
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 1,
        chain_id: CHAIN_ID.to_string(),
        gcs_mode: false,
        disable_synthetic_ops: true,
        canonical_protocol_config_chain_id: None,
    }
}

fn options() -> IngestOptions {
    IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: false,
        dependent_ops_max_per_chain: 1,
        is_protocol_config_listener: false,
        disable_synthetic_ops: true,
    }
}

/// A block carrying one KMS key or CRS activation.
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
        chain_id: CHAIN_ID,
        flow,
        block_number: number,
        block_hash: B256::from(U256::from(number)),
        parent_hash: B256::from(U256::from(number - 1)),
        timestamp: 1_700_000_000,
        transactions: vec![TransactionPayload {
            from: Address::ZERO,
            to: Some(KMS_ADDRESS),
            hash: B256::from(U256::from(number)),
            transaction_index: 0,
            value: U256::ZERO,
            data: Default::default(),
            logs: vec![IndexedLog {
                address: KMS_ADDRESS,
                log_index: 0,
                topics: data.topics().to_vec(),
                data: data.data,
            }],
        }],
    }
}

async fn status(pool: &PgPool, table: &str, number: u64) -> Option<String> {
    sqlx::query_scalar(&format!(
        "SELECT status FROM {table} WHERE chain_id = $1 AND block_number = $2"
    ))
    .bind(CHAIN_ID as i64)
    .bind(number as i64)
    .fetch_optional(pool)
    .await
    .unwrap()
}

struct Harness {
    db: Database,
    pool: PgPool,
    store: MaterialStore,
}

impl Harness {
    async fn deliver(&mut self, payload: &BlockPayload) {
        let ack = ingest_payload(
            &mut self.db,
            &config(),
            &options(),
            &CHAIN_ID.to_string(),
            payload,
            self.store.clone(),
            &tokio_util::sync::CancellationToken::new(),
        )
        .await
        .unwrap();
        assert!(matches!(ack, AckDecision::Ack));
    }

    /// Redeliver `payload`, as the broker would, until the activation reaches
    /// `expected`. Each delivery spawns one KMS activation pass.
    async fn deliver_until(
        &mut self,
        payload: &BlockPayload,
        table: &str,
        number: u64,
        expected: &str,
    ) {
        tokio::time::timeout(Duration::from_secs(25), async {
            loop {
                self.deliver(payload).await;
                tokio::time::sleep(Duration::from_millis(200)).await;
                if status(&self.pool, table, number).await.as_deref()
                    == Some(expected)
                {
                    return;
                }
            }
        })
        .await
        .unwrap_or_else(|_| {
            panic!("{table} block {number} never became {expected}")
        });
    }
}

#[tokio::test]
async fn consumer_activates_kms_material_only_on_finalized_blocks() {
    use test_harness::instance::{setup_test_db, ImportMode};
    let instance = setup_test_db(ImportMode::None).await.unwrap();
    let db = Database::new(
        &instance.db_url,
        ChainId::try_from(CHAIN_ID).unwrap(),
        16,
    )
    .await
    .unwrap();
    let pool = db.pool().await;
    let mut harness = Harness {
        db,
        pool: pool.clone(),
        store: MaterialStore(Arc::new(AtomicBool::new(false))),
    };

    // Live ingestion stages the key; the download fails while storage is down.
    let key = activation(100, BlockFlow::Live, false);
    harness.deliver(&key).await;
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let retries: i32 = sqlx::query_scalar(
                "SELECT retry_count FROM kms_key_activation_events \
                 WHERE chain_id = $1 AND block_number = 100",
            )
            .bind(CHAIN_ID as i64)
            .fetch_one(&pool)
            .await
            .unwrap();
            if retries > 0 {
                break;
            }
            tokio::time::sleep(Duration::from_millis(50)).await;
        }
    })
    .await
    .expect("the failed download must be recorded as a retry");

    // The next delivery retries and downloads the material.
    harness.store.0.store(true, Ordering::SeqCst);
    harness
        .deliver_until(&key, "kms_key_activation_events", 100, "ready")
        .await;

    // An observed fork carries a CRS event, but its finalized sibling is empty.
    let orphan = activation(200, BlockFlow::Live, true);
    harness.deliver(&orphan).await;
    let mut canonical = orphan;
    canonical.flow = BlockFlow::Final;
    canonical.block_hash = B256::repeat_byte(55);
    canonical.transactions.clear();
    harness
        .deliver_until(
            &canonical,
            "kms_crs_activation_events",
            200,
            "cancelled",
        )
        .await;
    let keys: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM keys")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(keys, 0, "downloaded material must wait for block finality");

    // The finalized delivery of the key block activates it.
    let mut final_key = key;
    final_key.flow = BlockFlow::Final;
    harness
        .deliver_until(
            &final_key,
            "kms_key_activation_events",
            100,
            "activated",
        )
        .await;

    // A CRS event missed by live ingestion is staged and activated by its
    // finalized delivery alone.
    let crs = activation(101, BlockFlow::Final, true);
    harness
        .deliver_until(&crs, "kms_crs_activation_events", 101, "activated")
        .await;

    let counts: (i64, i64) = sqlx::query_as(
        "SELECT (SELECT COUNT(*) FROM keys), (SELECT COUNT(*) FROM crs)",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(counts, (1, 1), "duplicate deliveries activate once");
    let finalized: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM host_chain_blocks_valid \
         WHERE chain_id = $1 AND block_status = 'finalized'",
    )
    .bind(CHAIN_ID as i64)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(finalized, 3);

    // A drift revert that is not done holds every KMS pass.
    use fhevm_engine_common::drift_revert::{
        create_revert_signal, update_signal_status, SignalStatus,
    };
    let signal = create_revert_signal(&pool, CHAIN_ID as i64, 500)
        .await
        .unwrap()
        .unwrap();
    let held = activation(300, BlockFlow::Final, false);
    for _ in 0..3 {
        harness.deliver(&held).await;
    }
    tokio::time::sleep(Duration::from_millis(500)).await;
    assert_eq!(
        status(&pool, "kms_key_activation_events", 300)
            .await
            .as_deref(),
        Some("pending"),
        "no KMS pass may run during a drift revert"
    );
    update_signal_status(&pool, signal, &SignalStatus::Done)
        .await
        .unwrap();
    harness
        .deliver_until(&held, "kms_key_activation_events", 300, "activated")
        .await;
}
