//! Tests for the confidential-bridge association worker.

use serial_test::serial;
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio_util::sync::CancellationToken;

const SRC_CHAIN: i64 = 100;
const DST_CHAIN: i64 = 200;
const SAME_CHAIN: i64 = 300;

const CT64: &[u8] = &[0x11; 8];
const CT64_DIGEST: &[u8] = &[0xA1; 4];
const CT128_DIGEST: &[u8] = &[0xB2; 4];
const CT128_FORMAT: i16 = 11;
const KEY_ID_GW: &[u8] = &[0xC3, 0xC4];
const S3_FORMAT_VERSION: i16 = 1;
const CIPHERTEXT_VERSION: i16 = 0;
const CIPHERTEXT_TYPE: i16 = 4;
const SRC_BLOCK_HASH: &[u8] = &[0x51; 32];
const DST_BLOCK_HASH: &[u8] = &[0x52; 32];

/// Subset of `ciphertext_digest` columns asserted to be copied verbatim.
#[derive(sqlx::FromRow)]
struct CopiedDigest {
    ciphertext: Option<Vec<u8>>,
    ciphertext128: Option<Vec<u8>>,
    ciphertext128_format: i16,
    key_id_gw: Vec<u8>,
    s3_format_version: Option<i16>,
}

/// Returns the `DBInstance` (kept alive by the caller) and a connected pool.
async fn fresh_db() -> (DBInstance, PgPool) {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.expect("connect pool");
    (db, pool)
}

fn handle(seed: u8) -> Vec<u8> {
    vec![seed; 32]
}

async fn insert_ciphertext(pool: &PgPool, handle: &[u8], ciphertext: &[u8]) {
    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, $3, $4)",
    )
    .bind(handle)
    .bind(ciphertext)
    .bind(CIPHERTEXT_VERSION)
    .bind(CIPHERTEXT_TYPE)
    .execute(pool)
    .await
    .expect("insert ciphertext");
}

async fn insert_digest(
    pool: &PgPool,
    handle: &[u8],
    ct64_digest: Option<&[u8]>,
    ct128_digest: Option<&[u8]>,
    host_chain_id: i64,
) {
    sqlx::query(
        "INSERT INTO ciphertext_digest
             (host_chain_id, key_id_gw, handle, ciphertext, ciphertext128, ciphertext128_format, s3_format_version)
         VALUES ($1, $2, $3, $4, $5, $6, $7)",
    )
    .bind(host_chain_id)
    .bind(KEY_ID_GW)
    .bind(handle)
    .bind(ct64_digest)
    .bind(ct128_digest)
    .bind(CT128_FORMAT)
    .bind(S3_FORMAT_VERSION)
    .execute(pool)
    .await
    .expect("insert ciphertext_digest");
}

async fn insert_src_event(pool: &PgPool, src_handle: &[u8], src_chain_id: i64, dst_chain_id: i64) {
    sqlx::query(
        "INSERT INTO bridge_handle_events
             (src_handle, dst_chain_id, src_chain_id, sender_dapp, guid, block_number)
         VALUES ($1, $2, $3, '\\xda'::bytea, '\\x01'::bytea, 1)",
    )
    .bind(src_handle)
    .bind(dst_chain_id)
    .bind(src_chain_id)
    .execute(pool)
    .await
    .expect("insert bridge_handle_events");
}

async fn insert_dst_event(pool: &PgPool, src_handle: &[u8], dst_handle: &[u8], dst_chain_id: i64) {
    sqlx::query(
        "INSERT INTO handle_bridged_events
             (src_handle, dst_handle, dst_chain_id, receiver_dapp, guid, block_number)
         VALUES ($1, $2, $3, '\\xdb'::bytea, '\\x01'::bytea, 1)",
    )
    .bind(src_handle)
    .bind(dst_handle)
    .bind(dst_chain_id)
    .execute(pool)
    .await
    .expect("insert handle_bridged_events");
}

async fn insert_block_status(pool: &PgPool, chain_id: i64, block_hash: &[u8], status: &str) {
    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number, block_status)
         VALUES ($1, $2, 1, $3)",
    )
    .bind(chain_id)
    .bind(block_hash)
    .bind(status)
    .execute(pool)
    .await
    .expect("insert host_chain_blocks_valid");
}

async fn insert_src_event_with_block_hash(
    pool: &PgPool,
    src_handle: &[u8],
    src_chain_id: i64,
    dst_chain_id: i64,
    block_hash: &[u8],
) {
    sqlx::query(
        "INSERT INTO bridge_handle_events
             (src_handle, dst_chain_id, src_chain_id, sender_dapp, guid, block_number, block_hash)
         VALUES ($1, $2, $3, '\\xda'::bytea, '\\x01'::bytea, 1, $4)",
    )
    .bind(src_handle)
    .bind(dst_chain_id)
    .bind(src_chain_id)
    .bind(block_hash)
    .execute(pool)
    .await
    .expect("insert bridge_handle_events");
}

async fn insert_dst_event_with_block_hash(
    pool: &PgPool,
    src_handle: &[u8],
    dst_handle: &[u8],
    dst_chain_id: i64,
    block_hash: &[u8],
) {
    sqlx::query(
        "INSERT INTO handle_bridged_events
             (src_handle, dst_handle, dst_chain_id, receiver_dapp, guid, block_number, block_hash)
         VALUES ($1, $2, $3, '\\xdb'::bytea, '\\x01'::bytea, 1, $4)",
    )
    .bind(src_handle)
    .bind(dst_handle)
    .bind(dst_chain_id)
    .bind(block_hash)
    .execute(pool)
    .await
    .expect("insert handle_bridged_events");
}

/// Inserts a fully-ready pair: a materialized source ciphertext, the source
/// approval, and the destination event.
async fn insert_ready_pair(pool: &PgPool, src_handle: &[u8], dst_handle: &[u8]) {
    insert_ciphertext(pool, src_handle, CT64).await;
    insert_digest(
        pool,
        src_handle,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SRC_CHAIN,
    )
    .await;
    insert_src_event(pool, src_handle, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(pool, src_handle, dst_handle, DST_CHAIN).await;
}

async fn insert_branch_source(pool: &PgPool, src_handle: &[u8]) {
    let producer_block_hash = vec![0x61_u8; 32];
    sqlx::query(
        "INSERT INTO ciphertexts_branch(
            handle, ciphertext, ciphertext_version, ciphertext_type,
            producer_block_hash, block_number
         )
         VALUES ($1, $2, $3, $4, $5, 1)",
    )
    .bind(src_handle)
    .bind(CT64)
    .bind(CIPHERTEXT_VERSION)
    .bind(CIPHERTEXT_TYPE)
    .bind(&producer_block_hash)
    .execute(pool)
    .await
    .expect("insert branch source ciphertext");
    sqlx::query(
        "INSERT INTO ciphertext_digest_branch(
            host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
            ciphertext128_format, s3_format_version, producer_block_hash,
            block_number, block_hash
         )
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8, 1, $8)",
    )
    .bind(SRC_CHAIN)
    .bind(KEY_ID_GW)
    .bind(src_handle)
    .bind(CT64_DIGEST)
    .bind(CT128_DIGEST)
    .bind(CT128_FORMAT)
    .bind(S3_FORMAT_VERSION)
    .bind(&producer_block_hash)
    .execute(pool)
    .await
    .expect("insert branch source digest");
}

async fn is_associated(pool: &PgPool, dst_handle: &[u8]) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT is_associated FROM handle_bridged_events WHERE dst_handle = $1",
    )
    .bind(dst_handle)
    .fetch_one(pool)
    .await
    .unwrap()
}

async fn digest_count(pool: &PgPool, handle: &[u8]) -> i64 {
    sqlx::query_scalar::<_, i64>("SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1")
        .bind(handle)
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Mirrors the transaction-sender's pickup predicate: is `handle` queued for
/// publication on `host_chain_id`?
async fn in_publish_queue(pool: &PgPool, handle: &[u8], host_chain_id: i64) -> bool {
    sqlx::query_scalar::<_, bool>(
        "SELECT EXISTS (
             SELECT 1 FROM ciphertext_digest
             WHERE handle = $1
               AND host_chain_id = $2
               AND txn_is_sent = false
               AND ciphertext IS NOT NULL
               AND ciphertext128 IS NOT NULL)",
    )
    .bind(handle)
    .bind(host_chain_id)
    .fetch_one(pool)
    .await
    .unwrap()
}

#[tokio::test]
#[serial]
async fn associates_ready_pair() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);
    insert_ready_pair(&pool, &src, &dst).await;

    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 1);

    // The ct64 blob is copied verbatim onto the destination handle (src -> dst).
    let (ciphertext, version, ct_type): (Vec<u8>, i16, i16) = sqlx::query_as(
        "SELECT ciphertext, ciphertext_version, ciphertext_type FROM ciphertexts WHERE handle = $1",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(ciphertext, CT64);
    assert_eq!(version, CIPHERTEXT_VERSION);
    assert_eq!(ct_type, CIPHERTEXT_TYPE);

    // The digest values are copied verbatim, so the destination handle resolves
    // to the same S3 blobs as the source (no re-SnS).
    let digest: CopiedDigest = sqlx::query_as(
        "SELECT ciphertext, ciphertext128, ciphertext128_format, key_id_gw, s3_format_version
         FROM ciphertext_digest WHERE handle = $1",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(digest.ciphertext.as_deref(), Some(CT64_DIGEST));
    assert_eq!(digest.ciphertext128.as_deref(), Some(CT128_DIGEST));
    assert_eq!(digest.ciphertext128_format, CT128_FORMAT);
    assert_eq!(digest.key_id_gw, KEY_ID_GW);
    // NULL here means "not uploaded" to the S3 migration machinery; the copy
    // points at the source's blobs, so it must inherit their format version.
    assert_eq!(digest.s3_format_version, Some(S3_FORMAT_VERSION));

    assert!(is_associated(&pool, &dst).await);

    assert!(in_publish_queue(&pool, &dst, DST_CHAIN).await);
}

#[tokio::test]
#[serial]
async fn post_cutover_association_reads_and_writes_branch_tables() {
    let (_db, pool) = fresh_db().await;
    let src = handle(3);
    let dst = handle(4);
    insert_branch_source(&pool, &src).await;
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;

    let associated =
        crate::bridge::drain_associations_at_cutover(&pool, 128, &CancellationToken::new(), 0)
            .await
            .unwrap();
    assert_eq!(associated, 1);
    assert!(is_associated(&pool, &dst).await);

    let legacy_ciphertexts: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    let legacy_digests: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(legacy_ciphertexts, 0);
    assert_eq!(legacy_digests, 0);

    let copied: (Vec<u8>, Vec<u8>) = sqlx::query_as(
        "SELECT c.ciphertext, c.producer_block_hash
         FROM ciphertexts_branch c
         WHERE c.handle = $1",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(copied.0, CT64);
    assert!(copied.1.is_empty(), "bridged destination is branchless");

    let digest: CopiedDigest = sqlx::query_as(
        "SELECT ciphertext, ciphertext128, ciphertext128_format, key_id_gw, s3_format_version
         FROM ciphertext_digest_branch
         WHERE handle = $1
           AND producer_block_hash = ''::bytea
           AND block_hash = ''::bytea",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(digest.ciphertext.as_deref(), Some(CT64_DIGEST));
    assert_eq!(digest.ciphertext128.as_deref(), Some(CT128_DIGEST));
    assert_eq!(digest.ciphertext128_format, CT128_FORMAT);
    assert_eq!(digest.key_id_gw, KEY_ID_GW);
    assert_eq!(digest.s3_format_version, Some(S3_FORMAT_VERSION));
}

#[tokio::test]
#[serial]
async fn skips_when_source_approval_missing() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    // Everything except the source-chain approval.
    insert_ciphertext(&pool, &src, CT64).await;
    insert_digest(
        &pool,
        &src,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SRC_CHAIN,
    )
    .await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;

    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 0);
    assert_eq!(digest_count(&pool, &dst).await, 0);
    assert!(!is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn associates_when_source_event_arrives_last() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    // Destination event and a fully-materialized source ciphertext are present,
    // but the source-chain approval has not arrived yet.
    insert_ciphertext(&pool, &src, CT64).await;
    insert_digest(
        &pool,
        &src,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SRC_CHAIN,
    )
    .await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );
    assert!(!is_associated(&pool, &dst).await);

    // The source `BridgeHandle` approval arrives last -> the pair associates.
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        1
    );
    assert!(is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn skips_events_from_orphaned_bridge_blocks() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    insert_ciphertext(&pool, &src, CT64).await;
    insert_digest(
        &pool,
        &src,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SRC_CHAIN,
    )
    .await;

    insert_block_status(&pool, SRC_CHAIN, SRC_BLOCK_HASH, "pending").await;
    insert_block_status(&pool, DST_CHAIN, DST_BLOCK_HASH, "finalized").await;
    insert_src_event_with_block_hash(&pool, &src, SRC_CHAIN, DST_CHAIN, SRC_BLOCK_HASH).await;
    insert_dst_event_with_block_hash(&pool, &src, &dst, DST_CHAIN, DST_BLOCK_HASH).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );

    sqlx::query(
        "UPDATE host_chain_blocks_valid
         SET block_status = 'orphaned'
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(SRC_CHAIN)
    .bind(SRC_BLOCK_HASH)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );

    sqlx::query(
        "UPDATE host_chain_blocks_valid
         SET block_status = 'finalized'
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(SRC_CHAIN)
    .bind(SRC_BLOCK_HASH)
    .execute(&pool)
    .await
    .unwrap();
    sqlx::query(
        "UPDATE host_chain_blocks_valid
         SET block_status = 'orphaned'
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(DST_CHAIN)
    .bind(DST_BLOCK_HASH)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );
    assert_eq!(digest_count(&pool, &dst).await, 0);
    assert!(!is_associated(&pool, &dst).await);

    sqlx::query(
        "UPDATE host_chain_blocks_valid
         SET block_status = 'finalized'
         WHERE chain_id = $1 AND block_hash = $2",
    )
    .bind(DST_CHAIN)
    .bind(DST_BLOCK_HASH)
    .execute(&pool)
    .await
    .unwrap();
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        1
    );
    assert!(is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn associates_pending_destination_block() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    insert_ciphertext(&pool, &src, CT64).await;
    insert_digest(
        &pool,
        &src,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SRC_CHAIN,
    )
    .await;

    // Source approval finalized; destination event still in a pending block.
    insert_block_status(&pool, SRC_CHAIN, SRC_BLOCK_HASH, "finalized").await;
    insert_block_status(&pool, DST_CHAIN, DST_BLOCK_HASH, "pending").await;
    insert_src_event_with_block_hash(&pool, &src, SRC_CHAIN, DST_CHAIN, SRC_BLOCK_HASH).await;
    insert_dst_event_with_block_hash(&pool, &src, &dst, DST_CHAIN, DST_BLOCK_HASH).await;

    // Destination finality is not awaited: the pair associates immediately.
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        1
    );
    assert!(is_associated(&pool, &dst).await);
    assert_eq!(digest_count(&pool, &dst).await, 1);
}

#[tokio::test]
#[serial]
async fn associates_only_when_source_fully_materialized() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    // Both events present, but the source ciphertext has not materialized yet.
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );

    // ct64 blob present, but no digest row yet.
    insert_ciphertext(&pool, &src, CT64).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );

    // Digest row present but ct128 digest still missing.
    insert_digest(&pool, &src, Some(CT64_DIGEST), None, SRC_CHAIN).await;
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );
    assert!(!is_associated(&pool, &dst).await);

    // Both digests now present -> ready.
    sqlx::query("UPDATE ciphertext_digest SET ciphertext128 = $1 WHERE handle = $2")
        .bind(CT128_DIGEST)
        .bind(&src)
        .execute(&pool)
        .await
        .unwrap();
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        1
    );
    assert!(is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn associates_only_once() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);
    insert_ready_pair(&pool, &src, &dst).await;

    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        1
    );
    // A second run finds nothing new: the associated row is skipped.
    assert_eq!(
        crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
            .await
            .unwrap(),
        0
    );
    assert_eq!(digest_count(&pool, &dst).await, 1);
}

#[tokio::test]
#[serial]
async fn skips_pair_when_destination_already_materialized() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);
    insert_ready_pair(&pool, &src, &dst).await;

    // A grantFallbackPlaintext recovery already materialized the destination
    // handle with a different ciphertext (a trivial encryption).
    let fallback_ct: &[u8] = &[0x99; 8];
    insert_ciphertext(&pool, &dst, fallback_ct).await;

    // The pair is skipped: the destination already has a ciphertext, so there is
    // nothing to copy.
    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 0);

    // The destination ciphertext is untouched and no source digest is copied, so
    // the published digest can never disagree with the stored ciphertext.
    let ciphertext: Vec<u8> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(ciphertext, fallback_ct);
    assert_eq!(digest_count(&pool, &dst).await, 0);

    // The bridge did not associate this handle, so the flag stays false.
    assert!(!is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn associate_pair_skips_digest_and_flag_when_copy_no_ops() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);
    insert_ready_pair(&pool, &src, &dst).await;

    let fallback_ct: &[u8] = &[0x99; 8];
    insert_ciphertext(&pool, &dst, fallback_ct).await;

    let id: i64 = sqlx::query_scalar("SELECT id FROM handle_bridged_events WHERE dst_handle = $1")
        .bind(&dst)
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut txn = pool.begin().await.unwrap();
    let associated = crate::bridge::associate_pair(&mut txn, id, &src, &dst, SRC_CHAIN, DST_CHAIN)
        .await
        .unwrap();
    txn.commit().await.unwrap();
    assert!(!associated, "a no-op copy must not report an association");

    // Destination ciphertext untouched, no digest copied, not marked associated.
    let ciphertext: Vec<u8> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(ciphertext, fallback_ct);
    assert_eq!(digest_count(&pool, &dst).await, 0);
    assert!(!is_associated(&pool, &dst).await);
}

#[tokio::test]
#[serial]
async fn drains_across_multiple_batches() {
    let (_db, pool) = fresh_db().await;

    // Three ready pairs, drained with a batch size of two: the loop runs a full
    // batch then a short one.
    let pairs: Vec<(Vec<u8>, Vec<u8>)> =
        (0..3).map(|i| (handle(10 + i), handle(110 + i))).collect();
    for (src, dst) in &pairs {
        insert_ready_pair(&pool, src, dst).await;
    }

    let associated = crate::bridge::drain_associations(&pool, 2, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 3);
    for (_src, dst) in &pairs {
        assert!(is_associated(&pool, dst).await);
        assert_eq!(digest_count(&pool, dst).await, 1);
    }
}

/// A same-chain bridge (source and destination on one chain) is allowed: the
/// worker copies the source ciphertext onto the distinct destination handle,
/// retargets the digest to that chain, and leaves the source intact — a clone,
/// not a move.
#[tokio::test]
#[serial]
async fn associates_same_chain_pair() {
    let (_db, pool) = fresh_db().await;
    let src = handle(1);
    let dst = handle(2);

    insert_ciphertext(&pool, &src, CT64).await;
    insert_digest(
        &pool,
        &src,
        Some(CT64_DIGEST),
        Some(CT128_DIGEST),
        SAME_CHAIN,
    )
    .await;
    // Source approval and destination event share one chain (src == dst).
    insert_src_event(&pool, &src, SAME_CHAIN, SAME_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, SAME_CHAIN).await;

    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 1);

    // Ciphertext copied onto the destination handle; digest retargeted to the
    // same chain and queued for publication there.
    let dst_ct: Vec<u8> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(dst_ct, CT64);
    assert_eq!(digest_count(&pool, &dst).await, 1);
    assert!(in_publish_queue(&pool, &dst, SAME_CHAIN).await);
    assert!(is_associated(&pool, &dst).await);

    // Source untouched: a same-chain bridge clones to a new handle.
    let src_ct: Vec<u8> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
            .bind(&src)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(src_ct, CT64);
    assert_eq!(digest_count(&pool, &src).await, 1);
    assert!(in_publish_queue(&pool, &src, SAME_CHAIN).await);
}

/// A destination event below the branch cutover whose source was produced by a
/// wave2 binary: bytes and digests exist only in the branch tables. The legacy
/// writer must still associate it (branch-aware source reads) and write the
/// destination to the legacy tables, matching pre-cutover destination
/// semantics.
#[tokio::test]
#[serial]
async fn pre_cutover_dst_with_branch_only_source_associates() {
    let (_db, pool) = fresh_db().await;
    let src = handle(5);
    let dst = handle(6);
    insert_branch_source(&pool, &src).await;
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;

    // drain_associations pins the cutover above every event block, so the
    // destination goes through the legacy writer.
    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 1);
    assert!(is_associated(&pool, &dst).await);

    let (ciphertext, version, ct_type): (Vec<u8>, i16, i16) = sqlx::query_as(
        "SELECT ciphertext, ciphertext_version, ciphertext_type FROM ciphertexts WHERE handle = $1",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(ciphertext, CT64);
    assert_eq!(version, CIPHERTEXT_VERSION);
    assert_eq!(ct_type, CIPHERTEXT_TYPE);

    // The digest values come from the branch digest row (no legacy digest
    // exists) and land in the legacy table, so the destination publishes
    // through the pre-cutover pipeline.
    let digest: CopiedDigest = sqlx::query_as(
        "SELECT ciphertext, ciphertext128, ciphertext128_format, key_id_gw, s3_format_version
         FROM ciphertext_digest WHERE handle = $1",
    )
    .bind(&dst)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(digest.ciphertext.as_deref(), Some(CT64_DIGEST));
    assert_eq!(digest.ciphertext128.as_deref(), Some(CT128_DIGEST));
    assert_eq!(digest.ciphertext128_format, CT128_FORMAT);
    assert_eq!(digest.key_id_gw, KEY_ID_GW);
    assert_eq!(digest.s3_format_version, Some(S3_FORMAT_VERSION));
    assert!(in_publish_queue(&pool, &dst, DST_CHAIN).await);

    // The destination is legacy-only: no branch-table rows were created.
    let branch_rows: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts_branch WHERE handle = $1")
            .bind(&dst)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(branch_rows, 0);
}

/// A ZK-input source handle: the verifier dual-writes the legacy `ciphertexts`
/// row, but the wave2 sns-worker records digests in the branch table only. A
/// pre-cutover destination must associate with the digest taken from the
/// branch row instead of half-associating (ciphertext copied, no digest).
#[tokio::test]
#[serial]
async fn pre_cutover_zk_input_source_without_legacy_digest_associates() {
    let (_db, pool) = fresh_db().await;
    let src = handle(7);
    let dst = handle(8);
    insert_ciphertext(&pool, &src, CT64).await;
    insert_branch_source(&pool, &src).await;
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;

    let associated = crate::bridge::drain_associations(&pool, 128, &CancellationToken::new())
        .await
        .unwrap();
    assert_eq!(associated, 1);
    assert!(is_associated(&pool, &dst).await);

    // The association is complete: ciphertext AND digest are materialized.
    assert_eq!(digest_count(&pool, &dst).await, 1);
    assert!(in_publish_queue(&pool, &dst, DST_CHAIN).await);
}

/// Without any source material the writer must report no association: the
/// event stays unmarked, nothing is copied, and the batch does not count it
/// (which would inflate the metric and spin the drain loop).
#[tokio::test]
#[serial]
async fn associate_pair_returns_false_without_source_material() {
    let (_db, pool) = fresh_db().await;
    let src = handle(9);
    let dst = handle(10);
    insert_src_event(&pool, &src, SRC_CHAIN, DST_CHAIN).await;
    insert_dst_event(&pool, &src, &dst, DST_CHAIN).await;

    let id: i64 = sqlx::query_scalar("SELECT id FROM handle_bridged_events WHERE dst_handle = $1")
        .bind(&dst)
        .fetch_one(&pool)
        .await
        .unwrap();

    let mut txn = pool.begin().await.unwrap();
    let associated = crate::bridge::associate_pair(&mut txn, id, &src, &dst, SRC_CHAIN, DST_CHAIN)
        .await
        .unwrap();
    txn.commit().await.unwrap();

    assert!(!associated);
    assert!(!is_associated(&pool, &dst).await);
    let dst_cts: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts WHERE handle = $1")
        .bind(&dst)
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(dst_cts, 0);
    assert_eq!(digest_count(&pool, &dst).await, 0);
}
