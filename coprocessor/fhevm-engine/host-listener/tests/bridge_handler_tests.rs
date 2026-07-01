use alloy::primitives::{Address, FixedBytes, Log, U256};
use alloy::sol_types::SolEvent;
use fhevm_engine_common::bridge::derive_dst_handle;
use fhevm_engine_common::chain_id::ChainId;
use fhevm_engine_common::types::{
    SupportedFheOperations, COMPUTED_HANDLE_INDEX_MARKER, HANDLE_VERSION,
};
use serial_test::serial;
use sqlx::Row;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};

use host_listener::cmd::block_history::BlockSummary;
use host_listener::contracts::BridgeContract;
use host_listener::contracts::BridgeContract::BridgeContractEvents;
use host_listener::database::ingest::{
    ingest_block_logs, BlockLogs, IngestOptions,
};
use host_listener::database::tfhe_event_propagate::Database;

const SRC_CHAIN_ID: u64 = 1000;
const DST_CHAIN_ID: u64 = 2000;
const BLOCK_NUMBER: u64 = 42;
const BLOCK_TIMESTAMP: u64 = 1_700_000_000;
const BLOCK_HASH: [u8; 32] = [0xBC; 32];
const ACL: [u8; 20] = [0xAC; 20];

fn handle_for_chain(chain_id: u64, seed: u8) -> FixedBytes<32> {
    let mut bytes = [seed; 32];
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    FixedBytes::from(bytes)
}

async fn fresh_db(chain_id: u64) -> (Database, DBInstance) {
    let inst = setup_test_db(ImportMode::None).await.expect("test db");
    let db =
        Database::new(&inst.db_url, ChainId::try_from(chain_id).unwrap(), 16)
            .await
            .expect("database");
    (db, inst)
}

/// Ingests one bridge event end-to-end and returns whether a row was written.
async fn ingest(
    db: &Database,
    event: BridgeContractEvents,
    prev_block_hash: FixedBytes<32>,
    acl: Option<Address>,
) -> bool {
    let log = Log {
        address: Address::ZERO,
        data: event,
    };
    let mut tx = db.new_transaction().await.expect("tx");
    let inserted = db
        .handle_bridge_event(
            &mut tx,
            &log,
            &None,
            BLOCK_NUMBER,
            &FixedBytes::from(BLOCK_HASH),
            &prev_block_hash,
            BLOCK_TIMESTAMP,
            &acl,
        )
        .await
        .expect("handle_bridge_event");
    tx.commit().await.expect("commit");
    inserted
}

fn bridge_handle_event(
    src_handle: FixedBytes<32>,
    dst_chain_id: u64,
    guid: FixedBytes<32>,
) -> BridgeContractEvents {
    BridgeContractEvents::BridgeHandle(BridgeContract::BridgeHandle {
        senderDapp: Address::from([0xDA; 20]),
        srcHandle: src_handle,
        dstChainId: dst_chain_id,
        guid,
    })
}

#[tokio::test]
#[serial(db)]
async fn bridge_handle_with_matching_chain_id_is_inserted() {
    let (db, _inst) = fresh_db(SRC_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x11);
    let guid = FixedBytes::from([0x22; 32]);

    let inserted = ingest(
        &db,
        bridge_handle_event(src_handle, DST_CHAIN_ID, guid),
        FixedBytes::ZERO,
        None,
    )
    .await;
    assert!(inserted, "valid BridgeHandle should insert a row");

    let pool = db.pool().await;
    let row = sqlx::query(
        "SELECT src_chain_id, dst_chain_id, sender_dapp, guid, block_number, block_hash
         FROM bridge_handle_events WHERE src_handle = $1",
    )
    .bind(src_handle.as_slice())
    .fetch_one(&pool)
    .await
    .expect("row exists");
    assert_eq!(row.get::<i64, _>("src_chain_id"), SRC_CHAIN_ID as i64);
    assert_eq!(row.get::<i64, _>("dst_chain_id"), DST_CHAIN_ID as i64);
    assert_eq!(row.get::<Vec<u8>, _>("sender_dapp"), vec![0xDA; 20]);
    assert_eq!(row.get::<Vec<u8>, _>("guid"), guid.to_vec());
    assert_eq!(row.get::<i64, _>("block_number"), BLOCK_NUMBER as i64);
    assert_eq!(row.get::<Vec<u8>, _>("block_hash"), BLOCK_HASH.to_vec());
}

#[tokio::test]
#[serial(db)]
async fn bridge_handle_with_foreign_chain_id_is_ignored() {
    let (db, _inst) = fresh_db(SRC_CHAIN_ID).await;
    // srcHandle embeds a different chain than this (source) listener -> dropped.
    let src_handle = handle_for_chain(SRC_CHAIN_ID + 1, 0x11);
    let guid = FixedBytes::from([0x22; 32]);

    let inserted = ingest(
        &db,
        bridge_handle_event(src_handle, DST_CHAIN_ID, guid),
        FixedBytes::ZERO,
        None,
    )
    .await;
    assert!(!inserted, "foreign-chain BridgeHandle should be ignored");

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM bridge_handle_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial(db)]
async fn bridge_handle_with_out_of_range_dst_chain_id_is_ignored() {
    let (db, _inst) = fresh_db(SRC_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x11);
    let guid = FixedBytes::from([0x22; 32]);

    // dstChainId that overflows i64 cannot be a real chain id: it must be
    // ignored, not cause an insert error that stalls the block forever.
    let inserted = ingest(
        &db,
        bridge_handle_event(src_handle, 1u64 << 63, guid),
        FixedBytes::ZERO,
        None,
    )
    .await;
    assert!(!inserted, "out-of-range dstChainId should be ignored");

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM bridge_handle_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial(db)]
async fn bridge_handle_is_idempotent() {
    let (db, _inst) = fresh_db(SRC_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x11);
    let guid = FixedBytes::from([0x22; 32]);

    let first = ingest(
        &db,
        bridge_handle_event(src_handle, DST_CHAIN_ID, guid),
        FixedBytes::ZERO,
        None,
    )
    .await;
    let second = ingest(
        &db,
        bridge_handle_event(src_handle, DST_CHAIN_ID, guid),
        FixedBytes::ZERO,
        None,
    )
    .await;
    assert!(first, "first ingest inserts");
    assert!(!second, "second ingest is a no-op");

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM bridge_handle_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[serial(db)]
async fn handle_bridged_with_valid_derivation_is_inserted() {
    let (db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x33);
    let guid = FixedBytes::from([0x44; 32]);
    let prev = FixedBytes::from([0xBB; 32]);

    // dst_handle computed exactly as the contract (and the handler) would.
    let dst_handle = FixedBytes::from(derive_dst_handle(
        &src_handle.0,
        &ACL,
        DST_CHAIN_ID,
        &prev.0,
        BLOCK_TIMESTAMP,
    ));
    let event =
        BridgeContractEvents::HandleBridged(BridgeContract::HandleBridged {
            receiverDapp: Address::from([0xDB; 20]),
            srcHandle: src_handle,
            dstHandle: dst_handle,
            guid,
        });

    let inserted = ingest(&db, event, prev, Some(Address::from(ACL))).await;
    assert!(inserted, "correctly-derived HandleBridged should insert");

    let pool = db.pool().await;
    let row = sqlx::query(
        "SELECT src_handle, dst_chain_id, receiver_dapp, guid, block_number, block_hash
         FROM handle_bridged_events WHERE dst_handle = $1",
    )
    .bind(dst_handle.as_slice())
    .fetch_one(&pool)
    .await
    .expect("row exists");
    assert_eq!(row.get::<Vec<u8>, _>("src_handle"), src_handle.to_vec());
    assert_eq!(row.get::<i64, _>("dst_chain_id"), DST_CHAIN_ID as i64);
    assert_eq!(row.get::<Vec<u8>, _>("receiver_dapp"), vec![0xDB; 20]);
    assert_eq!(row.get::<Vec<u8>, _>("guid"), guid.to_vec());
    assert_eq!(row.get::<i64, _>("block_number"), BLOCK_NUMBER as i64);
    assert_eq!(row.get::<Vec<u8>, _>("block_hash"), BLOCK_HASH.to_vec());
}

#[tokio::test]
#[serial(db)]
async fn handle_bridged_with_invalid_derivation_is_ignored() {
    let (db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x33);
    // dst_handle that does NOT match the derivation -> must be ignored.
    let dst_handle = FixedBytes::from([0xFF; 32]);
    let event =
        BridgeContractEvents::HandleBridged(BridgeContract::HandleBridged {
            receiverDapp: Address::from([0xDB; 20]),
            srcHandle: src_handle,
            dstHandle: dst_handle,
            guid: FixedBytes::from([0x44; 32]),
        });

    let inserted = ingest(
        &db,
        event,
        FixedBytes::from([0xBB; 32]),
        Some(Address::from(ACL)),
    )
    .await;
    assert!(
        !inserted,
        "invalid handle derivation in HandleBridged should be ignored"
    );

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM handle_bridged_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

#[tokio::test]
#[serial(db)]
async fn handle_bridged_is_idempotent() {
    let (db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x33);
    let prev = FixedBytes::from([0xBB; 32]);
    let dst_handle = FixedBytes::from(derive_dst_handle(
        &src_handle.0,
        &ACL,
        DST_CHAIN_ID,
        &prev.0,
        BLOCK_TIMESTAMP,
    ));
    let event = || {
        BridgeContractEvents::HandleBridged(BridgeContract::HandleBridged {
            receiverDapp: Address::from([0xDB; 20]),
            srcHandle: src_handle,
            dstHandle: dst_handle,
            guid: FixedBytes::from([0x44; 32]),
        })
    };

    // Re-observation of the same event (catchup/reorg re-scan): the
    // dst_handle conflict key keeps a single row.
    let first = ingest(&db, event(), prev, Some(Address::from(ACL))).await;
    let second = ingest(&db, event(), prev, Some(Address::from(ACL))).await;
    assert!(first, "first ingest inserts");
    assert!(!second, "second ingest is a no-op");

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM handle_bridged_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 1);
}

#[tokio::test]
#[serial(db)]
async fn handle_bridged_without_acl_address_is_ignored() {
    let (db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let src_handle = handle_for_chain(SRC_CHAIN_ID, 0x33);
    let prev = FixedBytes::from([0xBB; 32]);
    // Correctly derived, but without a configured ACL address the derivation
    // cannot be verified, so the event must be ignored.
    let dst_handle = FixedBytes::from(derive_dst_handle(
        &src_handle.0,
        &ACL,
        DST_CHAIN_ID,
        &prev.0,
        BLOCK_TIMESTAMP,
    ));
    let event =
        BridgeContractEvents::HandleBridged(BridgeContract::HandleBridged {
            receiverDapp: Address::from([0xDB; 20]),
            srcHandle: src_handle,
            dstHandle: dst_handle,
            guid: FixedBytes::from([0x44; 32]),
        });

    let inserted = ingest(&db, event, prev, None).await;
    assert!(!inserted, "unverifiable HandleBridged should be ignored");

    let pool = db.pool().await;
    let count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM handle_bridged_events")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(count, 0);
}

const BRIDGE: [u8; 20] = [0xBD; 20];

fn fallback_dst_handle(chain_id: u64, fhe_type: u8) -> FixedBytes<32> {
    let mut bytes = [0xAB; 32];
    bytes[21] = COMPUTED_HANDLE_INDEX_MARKER;
    bytes[22..30].copy_from_slice(&chain_id.to_be_bytes());
    bytes[30] = fhe_type;
    bytes[31] = HANDLE_VERSION;
    FixedBytes::from(bytes)
}

// Calls `ingest_block_logs` over one block of `FallbackGrantedPlaintext`
// logs emitted by the bridge contract, exactly as the listener would see
// them. Each `(dst_handle, plaintext, tx_seed)` event gets a transaction
// hash built from `tx_seed`. Duplicate events for a handle within the same
// transaction already collapse to the first one via the
// `(output_handle, transaction_id)` conflict key on `computations`, so the
// duplicate tests use distinct seeds: separate host-chain transactions are
// the case that needs the ingest-side dedup.
async fn ingest_fallback_block(
    db: &mut Database,
    events: &[(FixedBytes<32>, U256, u8)],
    block_number: u64,
) {
    let logs = events
        .iter()
        .enumerate()
        .map(|(log_index, (dst_handle, plaintext, tx_seed))| {
            let event = BridgeContract::FallbackGrantedPlaintext {
                dstHandle: *dst_handle,
                plaintext: *plaintext,
            };
            alloy::rpc::types::Log {
                inner: Log {
                    address: Address::from(BRIDGE),
                    data: event.encode_log_data(),
                },
                transaction_hash: Some(FixedBytes::from([*tx_seed; 32])),
                log_index: Some(log_index as u64),
                ..Default::default()
            }
        })
        .collect();
    let block_logs = BlockLogs {
        logs,
        summary: BlockSummary {
            number: block_number,
            hash: FixedBytes::from([block_number as u8; 32]),
            parent_hash: FixedBytes::ZERO,
            timestamp: BLOCK_TIMESTAMP,
        },
        catchup: false,
        finalized: false,
    };
    let options = IngestOptions {
        dependence_by_connexity: false,
        dependence_cross_block: true,
        dependent_ops_max_per_chain: 0,
    };
    let chain_id = db.chain_id;
    ingest_block_logs(
        chain_id,
        db,
        &block_logs,
        &None,
        &None,
        &None,
        &Some(Address::from(BRIDGE)),
        options,
    )
    .await
    .expect("ingest_block_logs");
}

// Single-event convenience wrapper around `ingest_fallback_block`.
async fn ingest_fallback(
    db: &mut Database,
    dst_handle: FixedBytes<32>,
    plaintext: U256,
) {
    ingest_fallback_block(db, &[(dst_handle, plaintext, 0x77)], BLOCK_NUMBER)
        .await
}

async fn computation_count(db: &Database, handle: FixedBytes<32>) -> i64 {
    let pool = db.pool().await;
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM computations WHERE output_handle = $1",
    )
    .bind(handle.as_slice())
    .fetch_one(&pool)
    .await
    .unwrap()
}

async fn pbs_count(db: &Database, handle: FixedBytes<32>) -> i64 {
    let pool = db.pool().await;
    sqlx::query_scalar(
        "SELECT COUNT(*) FROM pbs_computations WHERE handle = $1",
    )
    .bind(handle.as_slice())
    .fetch_one(&pool)
    .await
    .unwrap()
}

#[tokio::test]
#[serial(db)]
async fn fallback_granted_plaintext_becomes_trivial_encrypt() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID, 5);

    ingest_fallback(&mut db, dst_handle, U256::from(123_456_789_u64)).await;

    let pool = db.pool().await;
    let row = sqlx::query(
        "SELECT fhe_operation, is_allowed, is_completed, host_chain_id, block_number
         FROM computations WHERE output_handle = $1",
    )
    .bind(dst_handle.as_slice())
    .fetch_one(&pool)
    .await
    .expect("computation row exists");
    assert_eq!(
        row.get::<i16, _>("fhe_operation"),
        SupportedFheOperations::FheTrivialEncrypt as i16
    );
    // Forced allowed so the worker picks it up (is_completed = !is_allowed).
    assert!(row.get::<bool, _>("is_allowed"));
    assert!(!row.get::<bool, _>("is_completed"));
    assert_eq!(row.get::<i64, _>("host_chain_id"), DST_CHAIN_ID as i64);
    assert_eq!(row.get::<i64, _>("block_number"), BLOCK_NUMBER as i64);
    assert_trivial_encrypt_operands(&db, dst_handle, 123_456_789, 5).await;

    // PBS enqueued so the ct128/digest get computed and published.
    assert_eq!(computation_count(&db, dst_handle).await, 1);
    assert_eq!(pbs_count(&db, dst_handle).await, 1);
}

#[tokio::test]
#[serial(db)]
async fn fallback_with_foreign_chain_handle_is_ignored() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    // dstHandle embeds a different chain than this listener -> rejected.
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID + 1, 5);

    ingest_fallback(&mut db, dst_handle, U256::from(1_u64)).await;

    assert_eq!(computation_count(&db, dst_handle).await, 0);
    assert_eq!(pbs_count(&db, dst_handle).await, 0);
}

#[tokio::test]
#[serial(db)]
async fn fallback_with_unsupported_fhe_type_is_ignored() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    // FheType 1 (Uint4) is excluded from the bridging allowlist.
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID, 1);

    ingest_fallback(&mut db, dst_handle, U256::from(1_u64)).await;

    assert_eq!(computation_count(&db, dst_handle).await, 0);
    assert_eq!(pbs_count(&db, dst_handle).await, 0);
}

#[tokio::test]
#[serial(db)]
async fn fallback_with_missing_marker_is_ignored() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    // Byte 21 must carry the computed-handle marker.
    let mut bytes = fallback_dst_handle(DST_CHAIN_ID, 5).0;
    bytes[21] = 0x00;
    let dst_handle = FixedBytes::from(bytes);

    ingest_fallback(&mut db, dst_handle, U256::from(1_u64)).await;

    assert_eq!(computation_count(&db, dst_handle).await, 0);
    assert_eq!(pbs_count(&db, dst_handle).await, 0);
}

#[tokio::test]
#[serial(db)]
async fn fallback_with_wrong_handle_version_is_ignored() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    // Byte 31 must carry the expected handle version.
    let mut bytes = fallback_dst_handle(DST_CHAIN_ID, 5).0;
    bytes[31] = HANDLE_VERSION + 1;
    let dst_handle = FixedBytes::from(bytes);

    ingest_fallback(&mut db, dst_handle, U256::from(1_u64)).await;

    assert_eq!(computation_count(&db, dst_handle).await, 0);
    assert_eq!(pbs_count(&db, dst_handle).await, 0);
}

/// Asserts the single computation for `handle` stores the synthetic trivial
/// encrypt's scalar operands `[plaintext, fhe_type]`.
async fn assert_trivial_encrypt_operands(
    db: &Database,
    handle: FixedBytes<32>,
    plaintext: u64,
    fhe_type: u8,
) {
    let pool = db.pool().await;
    let row = sqlx::query(
        "SELECT dependencies, is_scalar FROM computations
         WHERE output_handle = $1",
    )
    .bind(handle.as_slice())
    .fetch_one(&pool)
    .await
    .expect("computation row exists");
    let dependencies: Vec<Vec<u8>> = row.get("dependencies");
    assert_eq!(dependencies.len(), 2);
    assert_eq!(dependencies[0], U256::from(plaintext).to_be_bytes_vec());
    assert_eq!(dependencies[1], vec![fhe_type]);
    assert!(row.get::<bool, _>("is_scalar"));
}

#[tokio::test]
#[serial(db)]
async fn fallback_only_first_event_per_handle_is_used() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID, 5);

    // A second grant for the same handle in a later block (and a different
    // transaction): the contract specifies the first event is the source of
    // truth.
    ingest_fallback_block(
        &mut db,
        &[(dst_handle, U256::from(111_u64), 0x11)],
        BLOCK_NUMBER,
    )
    .await;
    ingest_fallback_block(
        &mut db,
        &[(dst_handle, U256::from(222_u64), 0x22)],
        BLOCK_NUMBER + 1,
    )
    .await;

    assert_eq!(computation_count(&db, dst_handle).await, 1);
    assert_eq!(pbs_count(&db, dst_handle).await, 1);
    assert_trivial_encrypt_operands(&db, dst_handle, 111, 5).await;
}

#[tokio::test]
#[serial(db)]
async fn fallback_duplicates_in_one_block_use_first_event() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID, 5);

    // Two grants for the same handle in one block, in different transactions.
    ingest_fallback_block(
        &mut db,
        &[
            (dst_handle, U256::from(111_u64), 0x11),
            (dst_handle, U256::from(222_u64), 0x22),
        ],
        BLOCK_NUMBER,
    )
    .await;

    assert_eq!(computation_count(&db, dst_handle).await, 1);
    assert_eq!(pbs_count(&db, dst_handle).await, 1);
    assert_trivial_encrypt_operands(&db, dst_handle, 111, 5).await;
}

#[tokio::test]
#[serial(db)]
async fn fallback_ignored_when_handle_has_ciphertext() {
    let (mut db, _inst) = fresh_db(DST_CHAIN_ID).await;
    let dst_handle = fallback_dst_handle(DST_CHAIN_ID, 5);

    let pool = db.pool().await;
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 5)",
    )
    .bind(dst_handle.as_slice())
    .bind(&[0xCCu8; 4][..])
    .execute(&pool)
    .await
    .expect("insert ciphertext");

    ingest_fallback(&mut db, dst_handle, U256::from(123_u64)).await;

    // The fallback was skipped: no synthetic computation or PBS was created.
    assert_eq!(computation_count(&db, dst_handle).await, 0);
    assert_eq!(pbs_count(&db, dst_handle).await, 0);
}
