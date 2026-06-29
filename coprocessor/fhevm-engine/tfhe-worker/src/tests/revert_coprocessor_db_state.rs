use serial_test::serial;
use sqlx::PgPool;
use test_harness::db_utils::revert_coprocessor_db_state_sql;
use test_harness::instance::{setup_test_db, ImportMode};

const CHAIN_A: i64 = 100;
const CHAIN_B: i64 = 200;
const KEY_ID_GW: [u8; 1] = [0xAA];

// Prefixes for make_id to generate unique IDs per table.
const PREFIX_TXN: u8 = 1;
const PREFIX_HANDLE: u8 = 2;
const PREFIX_DC: u8 = 3;
const PREFIX_BLOCK_HASH: u8 = 4;
const PREFIX_SRC_HANDLE: u8 = 5;
const PREFIX_GUID: u8 = 6;
const PREFIX_DST_HANDLE: u8 = 7;

async fn setup_chains(pool: &PgPool) {
    sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'chain_a', '0xACL')")
        .bind(CHAIN_A)
        .execute(pool)
        .await
        .expect("insert host_chain A");

    sqlx::query("INSERT INTO host_chains (chain_id, name, acl_contract_address) VALUES ($1, 'chain_b', '0xACL2')")
        .bind(CHAIN_B)
        .execute(pool)
        .await
        .expect("insert host_chain B");

    sqlx::query("INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key) VALUES ($1, $2, $3, $4)")
        .bind(&KEY_ID_GW[..])
        .bind(&[0xBBu8][..])
        .bind(&[0xCCu8][..])
        .bind(&[0xDDu8][..])
        .execute(pool)
        .await
        .expect("insert key");
}

fn make_id(prefix: u8, chain_id: i64, block_number: i64) -> Vec<u8> {
    [
        &[prefix],
        &chain_id.to_be_bytes()[..],
        &block_number.to_be_bytes()[..],
    ]
    .concat()
}

async fn setup_block(pool: &PgPool, chain_id: i64, block_number: i64, key_id_gw: &[u8]) {
    let txn_id = make_id(PREFIX_TXN, chain_id, block_number);
    let handle = make_id(PREFIX_HANDLE, chain_id, block_number);
    let dc_id = make_id(PREFIX_DC, chain_id, block_number);
    let block_hash = make_id(PREFIX_BLOCK_HASH, chain_id, block_number);

    sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, $3)")
        .bind(&txn_id)
        .bind(chain_id)
        .bind(block_number)
        .execute(pool)
        .await
        .expect("insert transaction");

    sqlx::query(
        "INSERT INTO computations_branch (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, dependence_chain_id, block_number)
         VALUES ($1, ARRAY[]::bytea[], 1, false, $2, $3, $4, $5)",
    )
    .bind(&handle)
    .bind(&txn_id)
    .bind(chain_id)
    .bind(&dc_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert computation");

    sqlx::query(
        "INSERT INTO dependence_chain (dependence_chain_id, status, last_updated_at) VALUES ($1, 'updated', NOW())",
    )
    .bind(&dc_id)
    .execute(pool)
    .await
    .expect("insert dependence_chain");

    sqlx::query(
        "INSERT INTO pbs_computations_branch (handle, transaction_id, host_chain_id, block_number) VALUES ($1, $2, $3, $4)",
    )
    .bind(&handle)
    .bind(&txn_id)
    .bind(chain_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert pbs_computation");

    sqlx::query(
        "INSERT INTO allowed_handles_branch (handle, account_address, event_type, transaction_id, txn_block_number, host_chain_id, block_number)
         VALUES ($1, '0xAccount', 0, $2, $3, $4, $3)",
    )
    .bind(&handle)
    .bind(&txn_id)
    .bind(block_number)
    .bind(chain_id)
    .execute(pool)
    .await
    .expect("insert allowed_handle");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch (host_chain_id, key_id_gw, handle, transaction_id, txn_block_number)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(chain_id)
    .bind(key_id_gw)
    .bind(&handle)
    .bind(&txn_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert ciphertext_digest");

    sqlx::query(
        "INSERT INTO ciphertexts_branch (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 4)",
    )
    .bind(&handle)
    .bind([0xFFu8; 4])
    .execute(pool)
    .await
    .expect("insert ciphertext");

    sqlx::query("INSERT INTO ciphertexts128_branch (handle, ciphertext) VALUES ($1, $2)")
        .bind(&handle)
        .bind([0xEEu8; 4])
        .execute(pool)
        .await
        .expect("insert ciphertext128");

    sqlx::query(
        "INSERT INTO delegate_user_decrypt
            (delegator, delegate, contract_address, delegation_counter,
             old_expiration_date, new_expiration_date,
             host_chain_id, block_number, block_hash, transaction_id, on_gateway, reorg_out)
         VALUES ($1, $2, $3, $4, 0, 1000, $5, $6, $7, $8, false, false)",
    )
    .bind(&[0x10u8][..])
    .bind(&[0x20u8][..])
    .bind(&[0x30u8][..])
    .bind(block_number)
    .bind(chain_id)
    .bind(block_number)
    .bind(&block_hash)
    .bind(&txn_id)
    .execute(pool)
    .await
    .expect("insert delegate_user_decrypt");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number) VALUES ($1, $2, $3)",
    )
    .bind(chain_id)
    .bind(&block_hash)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert host_chain_blocks_valid");

    let src_handle = make_id(PREFIX_SRC_HANDLE, chain_id, block_number);
    let guid = make_id(PREFIX_GUID, chain_id, block_number);
    let dst_handle = make_id(PREFIX_DST_HANDLE, chain_id, block_number);

    sqlx::query(
        "INSERT INTO bridge_handle_events (src_handle, dst_chain_id, src_chain_id, sender_dapp, guid, block_number, transaction_id)
         VALUES ($1, 999, $2, '\\xdada'::bytea, $3, $4, $5)",
    )
    .bind(&src_handle)
    .bind(chain_id)
    .bind(&guid)
    .bind(block_number)
    .bind(&txn_id)
    .execute(pool)
    .await
    .expect("insert bridge_handle_events");

    sqlx::query(
        "INSERT INTO handle_bridged_events (src_handle, dst_handle, dst_chain_id, receiver_dapp, guid, block_number, transaction_id)
         VALUES ($1, $2, $3, '\\xdada'::bytea, $4, $5, $6)",
    )
    .bind(&src_handle)
    .bind(&dst_handle)
    .bind(chain_id)
    .bind(&guid)
    .bind(block_number)
    .bind(&txn_id)
    .execute(pool)
    .await
    .expect("insert handle_bridged_events");

    // only keep the latest block
    sqlx::query(
        "INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
         VALUES ($1, $2)
         ON CONFLICT (chain_id) DO UPDATE SET last_caught_up_block = GREATEST(host_listener_poller_state.last_caught_up_block, $2)",
    )
    .bind(chain_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("upsert host_listener_poller_state");
}

async fn setup_pbs_only_branch_artifacts(
    pool: &PgPool,
    chain_id: i64,
    block_number: i64,
    handle: &[u8],
    producer_block_hash: &[u8],
    key_id_gw: &[u8],
) {
    let txn_id = make_id(PREFIX_TXN, chain_id, block_number);

    sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, $3)")
        .bind(&txn_id)
        .bind(chain_id)
        .bind(block_number)
        .execute(pool)
        .await
        .expect("insert transaction");

    sqlx::query(
        "INSERT INTO pbs_computations_branch \
         (handle, transaction_id, host_chain_id, block_number, producer_block_hash) \
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(handle)
    .bind(&txn_id)
    .bind(chain_id)
    .bind(block_number)
    .bind(producer_block_hash)
    .execute(pool)
    .await
    .expect("insert pbs_computation");

    sqlx::query(
        "INSERT INTO ciphertext_digest_branch \
         (host_chain_id, key_id_gw, handle, transaction_id, producer_block_hash, block_number, ciphertext, ciphertext128) \
         VALUES ($1, $2, $3, $4, $5, $6, $7, $8)",
    )
    .bind(chain_id)
    .bind(key_id_gw)
    .bind(handle)
    .bind(&txn_id)
    .bind(producer_block_hash)
    .bind(block_number)
    .bind([0xFAu8; 32])
    .bind([0xFBu8; 32])
    .execute(pool)
    .await
    .expect("insert ciphertext_digest");

    sqlx::query(
        "INSERT INTO ciphertexts_branch \
         (handle, producer_block_hash, block_number, ciphertext, ciphertext_version, ciphertext_type) \
         VALUES ($1, $2, $3, $4, 0, 4)",
    )
    .bind(handle)
    .bind(producer_block_hash)
    .bind(block_number)
    .bind([0xFCu8; 4])
    .execute(pool)
    .await
    .expect("insert ciphertext");

    sqlx::query(
        "INSERT INTO ciphertexts128_branch (handle, producer_block_hash, block_number, ciphertext) \
         VALUES ($1, $2, $3, $4)",
    )
    .bind(handle)
    .bind(producer_block_hash)
    .bind(block_number)
    .bind([0xFDu8; 4])
    .execute(pool)
    .await
    .expect("insert ciphertext128");

    sqlx::query(
        "INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number) \
         VALUES ($1, $2, $3)",
    )
    .bind(chain_id)
    .bind(producer_block_hash)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert host_chain_blocks_valid");

    sqlx::query(
        "INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) \
         VALUES ($1, $2) \
         ON CONFLICT (chain_id) DO UPDATE SET last_caught_up_block = GREATEST(host_listener_poller_state.last_caught_up_block, $2)",
    )
    .bind(chain_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("upsert host_listener_poller_state");
}

/// Setup blocks from `from_block` to `up_to_block` (inclusive).
async fn setup_block_range(
    pool: &PgPool,
    chain_id: i64,
    from_block: i64,
    up_to_block: i64,
    key_id_gw: &[u8],
) {
    for block in from_block..=up_to_block {
        setup_block(pool, chain_id, block, key_id_gw).await;
    }
}

async fn count(pool: &PgPool, query: &str) -> i64 {
    sqlx::query_scalar::<_, i64>(query)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn count_with_bind(pool: &PgPool, query: &str, bind: i64) -> i64 {
    sqlx::query_scalar::<_, i64>(query)
        .bind(bind)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn count_handle(pool: &PgPool, query: &str, handle: &[u8]) -> i64 {
    sqlx::query_scalar::<_, i64>(query)
        .bind(handle)
        .fetch_one(pool)
        .await
        .unwrap()
}

async fn count_with_handle_hash(pool: &PgPool, query: &str, handle: &[u8], hash: &[u8]) -> i64 {
    sqlx::query_scalar::<_, i64>(query)
        .bind(handle)
        .bind(hash)
        .fetch_one(pool)
        .await
        .unwrap()
}

#[tokio::test]
#[serial(db)]
async fn test_revert_deletes_data_after_block_n() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    // Setup blocks 1..20 for chain A
    setup_block_range(&pool, CHAIN_A, 1, 20, &KEY_ID_GW).await;

    // Revert chain A to block 15
    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    let expected: Vec<i64> = (1..=15).collect();

    // Verify exactly blocks 1-15 remain across all block-linked tables
    let remaining_txn_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM transactions WHERE chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(remaining_txn_blocks, expected, "transactions");

    let remaining_block_tracking: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_blocks_valid WHERE chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_block_tracking, expected,
        "host_chain_blocks_valid"
    );

    let remaining_delegation_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM delegate_user_decrypt WHERE host_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_delegation_blocks, expected,
        "delegate_user_decrypt"
    );

    // Tables linked via transaction_id: verify count matches; block-number
    // pruning is covered by the producer-row deletes above.
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM computations_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        15,
        "computations"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM pbs_computations_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        15,
        "pbs_computations"
    );
    assert_eq!(
        count_with_bind(&pool, "SELECT COUNT(*) FROM ciphertexts_branch WHERE handle IN (SELECT output_handle FROM computations_branch WHERE host_chain_id = $1)", CHAIN_A).await,
        15, "ciphertexts"
    );
    assert_eq!(
        count_with_bind(&pool, "SELECT COUNT(*) FROM ciphertexts128_branch WHERE handle IN (SELECT output_handle FROM computations_branch WHERE host_chain_id = $1)", CHAIN_A).await,
        15, "ciphertexts128"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM ciphertext_digest_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        15,
        "ciphertext_digest"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM allowed_handles_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        15,
        "allowed_handles"
    );
    let remaining_bridge_handle_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM bridge_handle_events WHERE src_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_bridge_handle_blocks, expected,
        "bridge_handle_events"
    );

    let remaining_handle_bridged_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM handle_bridged_events WHERE dst_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_handle_bridged_blocks, expected,
        "handle_bridged_events"
    );

    // Poller state should be reset to 15
    let poller_block: i64 = sqlx::query_scalar(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
    )
    .bind(CHAIN_A)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(poller_block, 15, "poller should be reset to block 15");
}

#[tokio::test]
#[serial(db)]
async fn test_revert_preserves_other_chain_data() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    // Setup blocks 1..20 for both chains
    setup_block_range(&pool, CHAIN_A, 1, 20, &KEY_ID_GW).await;
    setup_block_range(&pool, CHAIN_B, 1, 20, &KEY_ID_GW).await;

    // Revert only chain A to block 15
    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    // Chain A blocks 16-20 should be gone
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM transactions WHERE chain_id = $1",
            CHAIN_A
        )
        .await,
        15,
    );

    // Chain B should be completely untouched (all 20 blocks)
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM transactions WHERE chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B transactions should be untouched"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM computations_branch WHERE host_chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B computations should be untouched"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM host_chain_blocks_valid WHERE chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B block tracking should be untouched"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM delegate_user_decrypt WHERE host_chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B delegations should be untouched"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM bridge_handle_events WHERE src_chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B bridge_handle_events should be untouched"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM handle_bridged_events WHERE dst_chain_id = $1",
            CHAIN_B
        )
        .await,
        20,
        "chain B handle_bridged_events should be untouched"
    );

    // Chain B poller should be unchanged at 20
    let poller_b: i64 = sqlx::query_scalar(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
    )
    .bind(CHAIN_B)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(poller_b, 20, "chain B poller should be untouched");
}

#[tokio::test]
#[serial(db)]
async fn test_revert_no_op_when_no_data_above_block_n() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    // Setup blocks 1..10 for chain A
    setup_block_range(&pool, CHAIN_A, 1, 10, &KEY_ID_GW).await;

    // Revert to block 15 — no data above 15 exists
    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    // All 10 blocks should still be there across all tables
    let expected: Vec<i64> = (1..=10).collect();

    let remaining_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM transactions WHERE chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(remaining_blocks, expected, "transactions");

    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM computations_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        10,
        "computations"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM pbs_computations_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        10,
        "pbs_computations"
    );
    assert_eq!(
        count_with_bind(&pool, "SELECT COUNT(*) FROM ciphertexts_branch WHERE handle IN (SELECT output_handle FROM computations_branch WHERE host_chain_id = $1)", CHAIN_A).await,
        10, "ciphertexts"
    );
    assert_eq!(
        count_with_bind(&pool, "SELECT COUNT(*) FROM ciphertexts128_branch WHERE handle IN (SELECT output_handle FROM computations_branch WHERE host_chain_id = $1)", CHAIN_A).await,
        10, "ciphertexts128"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM ciphertext_digest_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        10,
        "ciphertext_digest"
    );
    assert_eq!(
        count_with_bind(
            &pool,
            "SELECT COUNT(*) FROM allowed_handles_branch WHERE host_chain_id = $1",
            CHAIN_A
        )
        .await,
        10,
        "allowed_handles"
    );
    let remaining_bridge_handle_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM bridge_handle_events WHERE src_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_bridge_handle_blocks, expected,
        "bridge_handle_events"
    );

    let remaining_handle_bridged_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM handle_bridged_events WHERE dst_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_handle_bridged_blocks, expected,
        "handle_bridged_events"
    );

    let remaining_delegation_blocks: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM delegate_user_decrypt WHERE host_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_delegation_blocks, expected,
        "delegate_user_decrypt"
    );

    let remaining_block_tracking: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM host_chain_blocks_valid WHERE chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining_block_tracking, expected,
        "host_chain_blocks_valid"
    );

    // Poller should NOT have been moved forward (was at 10, to_block_number is 15)
    let poller: i64 = sqlx::query_scalar(
        "SELECT last_caught_up_block FROM host_listener_poller_state WHERE chain_id = $1",
    )
    .bind(CHAIN_A)
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(poller, 10, "poller should stay at 10, not move to 15");
}

#[tokio::test]
#[serial(db)]
async fn test_revert_deletes_pbs_only_branch_artifacts() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let handle = [0xA5u8; 32];
    let retained_block_hash = [0x10u8; 32];
    let reverted_block_hash = [0x20u8; 32];

    setup_pbs_only_branch_artifacts(
        &pool,
        CHAIN_A,
        10,
        &handle,
        &retained_block_hash,
        &KEY_ID_GW,
    )
    .await;
    setup_pbs_only_branch_artifacts(
        &pool,
        CHAIN_A,
        20,
        &handle,
        &reverted_block_hash,
        &KEY_ID_GW,
    )
    .await;

    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    for (label, query) in [
        (
            "pbs_computations_branch",
            "SELECT COUNT(*) FROM pbs_computations_branch WHERE handle = $1 AND producer_block_hash = $2",
        ),
        (
            "ciphertext_digest_branch",
            "SELECT COUNT(*) FROM ciphertext_digest_branch WHERE handle = $1 AND producer_block_hash = $2",
        ),
        (
            "ciphertexts_branch",
            "SELECT COUNT(*) FROM ciphertexts_branch WHERE handle = $1 AND producer_block_hash = $2",
        ),
        (
            "ciphertexts128_branch",
            "SELECT COUNT(*) FROM ciphertexts128_branch WHERE handle = $1 AND producer_block_hash = $2",
        ),
    ] {
        assert_eq!(
            count_with_handle_hash(&pool, query, &handle, &retained_block_hash).await,
            1,
            "{label} retained branch row should remain"
        );
        assert_eq!(
            count_with_handle_hash(&pool, query, &handle, &reverted_block_hash).await,
            0,
            "{label} reverted branch row should be deleted"
        );
    }
}

#[tokio::test]
#[serial(db)]
async fn test_revert_preserves_shared_ciphertexts() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let shared_handle: &[u8] = &[0xAA; 4];
    let txn_block10: &[u8] = &[0x01, 0x10];
    let txn_block20: &[u8] = &[0x01, 0x20];

    // Transaction at block 10 (retained)
    sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, 10)")
        .bind(txn_block10)
        .bind(CHAIN_A)
        .execute(&pool)
        .await
        .unwrap();

    // Transaction at block 20 (reverted)
    sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, 20)")
        .bind(txn_block20)
        .bind(CHAIN_A)
        .execute(&pool)
        .await
        .unwrap();

    // Both computations produce the same handle
    sqlx::query(
        "INSERT INTO computations_branch (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number)
         VALUES ($1, ARRAY[]::bytea[], 1, false, $2, $3, 10)",
    )
    .bind(shared_handle)
    .bind(txn_block10)
    .bind(CHAIN_A)
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query(
        "INSERT INTO computations_branch (output_handle, dependencies, fhe_operation, is_scalar, transaction_id, host_chain_id, block_number)
         VALUES ($1, ARRAY[]::bytea[], 1, false, $2, $3, 20)",
    )
    .bind(shared_handle)
    .bind(txn_block20)
    .bind(CHAIN_A)
    .execute(&pool)
    .await
    .unwrap();

    // Single ciphertext for the shared handle
    sqlx::query(
        "INSERT INTO ciphertexts_branch (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 4)",
    )
    .bind(shared_handle)
    .bind([0xFFu8; 4])
    .execute(&pool)
    .await
    .unwrap();

    sqlx::query("INSERT INTO ciphertexts128_branch (handle, ciphertext) VALUES ($1, $2)")
        .bind(shared_handle)
        .bind([0xEEu8; 4])
        .execute(&pool)
        .await
        .unwrap();

    // Poller state
    sqlx::query(
        "INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block) VALUES ($1, 20)",
    )
    .bind(CHAIN_A)
    .execute(&pool)
    .await
    .unwrap();

    // Revert to block 15
    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    // Block 20 computation should be gone
    assert_eq!(
        count(
            &pool,
            "SELECT COUNT(*) FROM computations_branch WHERE transaction_id = '\\x0120'::bytea"
        )
        .await,
        0,
    );

    // Block 10 computation should remain
    assert_eq!(
        count(
            &pool,
            "SELECT COUNT(*) FROM computations_branch WHERE transaction_id = '\\x0110'::bytea"
        )
        .await,
        1,
    );

    // Ciphertext should be PRESERVED because block 10 computation still references it
    let ct_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts_branch WHERE handle = $1")
            .bind(shared_handle)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(ct_count, 1, "shared ciphertext should be preserved");

    let ct128_count: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM ciphertexts128_branch WHERE handle = $1")
            .bind(shared_handle)
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(ct128_count, 1, "shared ciphertext128 should be preserved");
}

#[tokio::test]
#[serial(db)]
async fn test_revert_fails_for_nonexistent_chain() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let nonexistent_chain: i64 = 999;
    let sql = revert_coprocessor_db_state_sql(nonexistent_chain, 10);
    let result = sqlx::raw_sql(&sql).execute(&pool).await;
    assert!(result.is_err(), "revert should fail for nonexistent chain");
}

#[tokio::test]
#[serial(db)]
async fn test_revert_fails_for_zero_block_number() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 0);
    let result = sqlx::raw_sql(&sql).execute(&pool).await;
    assert!(
        result.is_err(),
        "revert should fail for to_block_number = 0"
    );
}

#[tokio::test]
#[serial(db)]
async fn test_revert_fails_for_negative_block_number() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let sql = revert_coprocessor_db_state_sql(CHAIN_A, -1);
    let result = sqlx::raw_sql(&sql).execute(&pool).await;
    assert!(
        result.is_err(),
        "revert should fail for negative to_block_number"
    );
}

#[tokio::test]
#[serial(db)]
async fn test_revert_fails_on_key_rotation() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    // Simulate key rotation: blocks 1..10 use key1, block 11 uses key2.
    setup_block_range(&pool, CHAIN_A, 1, 10, &KEY_ID_GW).await;

    let key_id_gw_2: &[u8] = &[0xBB];
    let key_id_2: &[u8] = &[0xCC];
    let pks_key_2: &[u8] = &[0xDD];
    let sks_key_2: &[u8] = &[0xEE];
    sqlx::query("INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key) VALUES ($1, $2, $3, $4)")
        .bind(key_id_gw_2)
        .bind(key_id_2)
        .bind(pks_key_2)
        .bind(sks_key_2)
        .execute(&pool)
        .await
        .expect("insert second key");

    setup_block_range(&pool, CHAIN_A, 11, 11, key_id_gw_2).await;

    // Revert to block 5 — affected blocks 6..11 span both keys.
    // Block 6..10 ciphertext_digests use key1, but latest key is key2.
    // The script should detect the mismatch and fail.
    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 5);
    let result = sqlx::raw_sql(&sql).execute(&pool).await;
    assert!(
        result.is_err(),
        "revert should fail when key rotation detected"
    );
}

/// Inserts a copy-bridged destination handle: the bridge worker materializes it
/// as a ciphertext + retargeted ciphertext_digest, but there is NO computations
/// row for it. The only link to a chain/block is the handle_bridged_events row.
async fn insert_bridged_handle(pool: &PgPool, chain_id: i64, block_number: i64, dst_handle: &[u8]) {
    let txn_id = make_id(PREFIX_TXN, chain_id, block_number);
    let src_handle = make_id(PREFIX_SRC_HANDLE, chain_id, block_number);
    let guid = make_id(PREFIX_GUID, chain_id, block_number);

    sqlx::query(
        "INSERT INTO handle_bridged_events (src_handle, dst_handle, dst_chain_id, receiver_dapp, guid, block_number, transaction_id)
         VALUES ($1, $2, $3, '\\xdada'::bytea, $4, $5, $6)",
    )
    .bind(&src_handle)
    .bind(dst_handle)
    .bind(chain_id)
    .bind(&guid)
    .bind(block_number)
    .bind(&txn_id)
    .execute(pool)
    .await
    .expect("insert handle_bridged_events");

    sqlx::query(
        "INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type)
         VALUES ($1, $2, 0, 4)",
    )
    .bind(dst_handle)
    .bind([0xCCu8; 4])
    .execute(pool)
    .await
    .expect("insert bridged ciphertext");

    sqlx::query(
        "INSERT INTO ciphertext_digest (host_chain_id, key_id_gw, handle, transaction_id, txn_block_number)
         VALUES ($1, $2, $3, $4, $5)",
    )
    .bind(chain_id)
    .bind(&KEY_ID_GW[..])
    .bind(dst_handle)
    .bind(&txn_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert bridged ciphertext_digest");
}

/// A copy-bridged destination handle has a ciphertext + ciphertext_digest but
/// NO computations row, so the computations-derived cleanup cannot see it. The
/// revert must still delete those rows via handle_bridged_events when the
/// bridged block is past the revert point, while leaving earlier blocks and
/// other chains untouched.
#[tokio::test]
#[serial(db)]
async fn test_revert_deletes_bridged_ciphertext_without_computation() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chains(&pool).await;

    let a_reverted = make_id(PREFIX_DST_HANDLE, CHAIN_A, 20); // past cut -> deleted
    let a_retained = make_id(PREFIX_DST_HANDLE, CHAIN_A, 10); // before cut -> kept
    let b_other = make_id(PREFIX_DST_HANDLE, CHAIN_B, 20); // other chain -> kept
    insert_bridged_handle(&pool, CHAIN_A, 20, &a_reverted).await;
    insert_bridged_handle(&pool, CHAIN_A, 10, &a_retained).await;
    insert_bridged_handle(&pool, CHAIN_B, 20, &b_other).await;

    let sql = revert_coprocessor_db_state_sql(CHAIN_A, 15);
    sqlx::raw_sql(&sql)
        .execute(&pool)
        .await
        .expect("revert sql");

    // The bridged copy past the cut is deleted even though no computations row
    // ever referenced it.
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertexts WHERE handle = $1",
            &a_reverted,
        )
        .await,
        0,
        "reverted bridged ciphertext deleted"
    );
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1",
            &a_reverted,
        )
        .await,
        0,
        "reverted bridged digest deleted"
    );

    // The earlier block's copy is retained.
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertexts WHERE handle = $1",
            &a_retained,
        )
        .await,
        1,
        "retained bridged ciphertext kept"
    );
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1",
            &a_retained,
        )
        .await,
        1,
        "retained bridged digest kept"
    );

    // The other chain's copy is untouched (dst_chain_id scoping).
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertexts WHERE handle = $1",
            &b_other,
        )
        .await,
        1,
        "other-chain bridged ciphertext kept"
    );
    assert_eq!(
        count_handle(
            &pool,
            "SELECT COUNT(*) FROM ciphertext_digest WHERE handle = $1",
            &b_other,
        )
        .await,
        1,
        "other-chain bridged digest kept"
    );
}
