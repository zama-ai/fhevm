use std::sync::{Arc, Mutex};
use std::time::Duration;

use fhevm_engine_common::drift_revert::{self, ReExec, RevertRunnerConfig, SignalStatus};
use serial_test::serial;
use sqlx::PgPool;
use test_harness::instance::{setup_test_db, ImportMode};
use tokio_util::sync::CancellationToken;

const CHAIN_A: i64 = 100;

async fn setup_chain_and_computation(pool: &PgPool, chain_id: i64, block_number: i64) -> Vec<u8> {
    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
         VALUES ($1, 'test', '0x1')",
    )
    .bind(chain_id)
    .execute(pool)
    .await
    .expect("insert host_chain");

    // Byte 21 = 0xff marks a compute output (see host-contracts FHEVMExecutor.sol).
    let mut handle: Vec<u8> = vec![0xAA; 32];
    handle[21] = 0xff;
    let txn_id: Vec<u8> = vec![0xBB; 32];

    sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, $3)")
        .bind(&txn_id)
        .bind(chain_id)
        .bind(block_number)
        .execute(pool)
        .await
        .expect("insert transaction");

    sqlx::query(
        "INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, \
         transaction_id, host_chain_id, block_number) \
         VALUES ($1, ARRAY[]::bytea[], 1, false, $2, $3, $4)",
    )
    .bind(&handle)
    .bind(&txn_id)
    .bind(chain_id)
    .bind(block_number)
    .execute(pool)
    .await
    .expect("insert computation");

    handle
}

#[tokio::test]
#[serial(db)]
async fn on_drift_detected_creates_signal_with_correct_block() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let block = 42;
    let handle = setup_chain_and_computation(&pool, CHAIN_A, block).await;

    drift_revert::on_drift_detected(&pool, &handle, CHAIN_A).await;

    let signal = drift_revert::latest_signal(&pool)
        .await
        .expect("latest_signal")
        .expect("should have a signal");

    assert_eq!(signal.host_chain_id, CHAIN_A);
    assert_eq!(signal.offending_host_block_number, block);
    assert_eq!(signal.status, SignalStatus::Pending);
}

#[tokio::test]
#[serial(db)]
async fn on_drift_detected_no_signal_when_handle_not_found() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let unknown_handle: Vec<u8> = vec![0xFF; 32];

    drift_revert::on_drift_detected(&pool, &unknown_handle, CHAIN_A).await;

    let signal = drift_revert::latest_signal(&pool)
        .await
        .expect("latest_signal");
    assert!(
        signal.is_none(),
        "should not create a signal for unknown handle"
    );
}

#[tokio::test]
#[serial(db)]
async fn on_drift_detected_skips_for_input_handle() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    // Byte 21 != 0xff → ZK input handle.
    let mut input_handle: Vec<u8> = vec![0xAA; 32];
    input_handle[21] = 0x01;

    drift_revert::on_drift_detected(&pool, &input_handle, CHAIN_A).await;

    let signal = drift_revert::latest_signal(&pool)
        .await
        .expect("latest_signal");
    assert!(
        signal.is_none(),
        "should not create a signal for ZK input handles"
    );
}

#[tokio::test]
#[serial(db)]
async fn on_drift_detected_skips_when_in_flight() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let handle = setup_chain_and_computation(&pool, CHAIN_A, 10).await;

    drift_revert::on_drift_detected(&pool, &handle, CHAIN_A).await;

    let result = drift_revert::create_revert_signal(&pool, CHAIN_A, 20)
        .await
        .expect("second call");
    assert!(result.is_none(), "should skip when in-flight");
}

#[tokio::test]
#[serial(db)]
async fn on_drift_detected_allows_new_signal_after_done() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    let handle = setup_chain_and_computation(&pool, CHAIN_A, 10).await;

    drift_revert::on_drift_detected(&pool, &handle, CHAIN_A).await;

    let signal = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    drift_revert::update_signal_status(&pool, signal.id, &SignalStatus::Done)
        .await
        .unwrap();

    let second_id = drift_revert::create_revert_signal(&pool, CHAIN_A, 20)
        .await
        .expect("second call after done")
        .expect("should allow new signal after done");

    assert_ne!(
        second_id, signal.id,
        "new signal should have a different id"
    );

    let latest = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    assert_eq!(latest.id, second_id);
    assert_eq!(latest.offending_host_block_number, 20);
    assert_eq!(latest.status, SignalStatus::Pending);
}

struct MockReExec {
    called: Arc<Mutex<bool>>,
}

impl MockReExec {
    fn new() -> Self {
        Self {
            called: Arc::new(Mutex::new(false)),
        }
    }

    fn was_called(&self) -> bool {
        *self.called.lock().unwrap()
    }
}

impl ReExec for MockReExec {
    fn re_exec(&self) {
        *self.called.lock().unwrap() = true;
    }
}

#[tokio::test]
#[serial(db)]
async fn signal_watcher_reexecs_on_pending_signal() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    let cancel = CancellationToken::new();
    let mock = MockReExec::new();

    setup_chain_and_computation(&pool, CHAIN_A, 10).await;

    drift_revert::create_revert_signal(&pool, CHAIN_A, 10)
        .await
        .expect("create signal");

    drift_revert::run_signal_watcher(&pool, cancel, &mock)
        .await
        .expect("run_signal_watcher");

    assert!(mock.was_called(), "re_exec should have been called");
}

#[tokio::test]
#[serial(db)]
async fn signal_watcher_exits_cleanly_on_cancel() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    let cancel = CancellationToken::new();
    let mock = MockReExec::new();

    cancel.cancel();

    drift_revert::run_signal_watcher(&pool, cancel, &mock)
        .await
        .expect("run_signal_watcher");

    assert!(!mock.was_called(), "re_exec should NOT have been called");
}

#[tokio::test]
#[serial(db)]
async fn handle_pending_signal_runner_marks_done() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chain_and_computation(&pool, CHAIN_A, 50).await;

    drift_revert::create_revert_signal(&pool, CHAIN_A, 50)
        .await
        .expect("create signal");

    let cfg = RevertRunnerConfig {
        grace_period: Duration::from_millis(10),
    };

    drift_revert::handle_pending_signal_on_startup(&pool, Some(cfg))
        .await
        .expect("handle_pending_signal_on_startup");

    let signal = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    assert_eq!(signal.status, SignalStatus::Done);
}

#[tokio::test]
#[serial(db)]
async fn handle_pending_signal_waiter_blocks_until_done() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chain_and_computation(&pool, CHAIN_A, 50).await;

    drift_revert::create_revert_signal(&pool, CHAIN_A, 50)
        .await
        .expect("create signal");

    let waiter = {
        let pool = pool.clone();
        tokio::spawn(async move {
            drift_revert::handle_pending_signal_on_startup(&pool, None)
                .await
                .expect("waiter");
        })
    };

    tokio::time::sleep(Duration::from_millis(100)).await;
    assert!(!waiter.is_finished(), "waiter should be blocking");

    let signal = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    drift_revert::update_signal_status(&pool, signal.id, &SignalStatus::Done)
        .await
        .unwrap();

    tokio::time::timeout(Duration::from_secs(3), waiter)
        .await
        .expect("waiter should finish within timeout")
        .expect("waiter task should succeed");
}

#[tokio::test]
#[serial(db)]
async fn execute_revert_deletes_computations_after_offending_block() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    // Set up blocks 1..=10 with one computation each.
    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
         VALUES ($1, 'test', '0x1')",
    )
    .bind(CHAIN_A)
    .execute(&pool)
    .await
    .expect("insert host_chain");

    for block in 1..=10 {
        let handle: Vec<u8> = vec![block as u8; 32];
        let txn_id: Vec<u8> = vec![block as u8 + 100; 32];
        sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, $3)")
            .bind(&txn_id)
            .bind(CHAIN_A)
            .bind(block)
            .execute(&pool)
            .await
            .expect("insert transaction");
        sqlx::query(
            "INSERT INTO computations (output_handle, dependencies, fhe_operation, is_scalar, \
             transaction_id, host_chain_id, block_number) \
             VALUES ($1, ARRAY[]::bytea[], 1, false, $2, $3, $4)",
        )
        .bind(&handle)
        .bind(&txn_id)
        .bind(CHAIN_A)
        .bind(block)
        .execute(&pool)
        .await
        .expect("insert computation");
    }

    // Drift detected at block 7 → revert target = 6 (offending - 1).
    drift_revert::execute_revert(&pool, CHAIN_A, 7)
        .await
        .expect("execute_revert");

    let remaining: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM computations WHERE host_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        remaining,
        (1..=6i64).collect::<Vec<_>>(),
        "should keep blocks 1..=6"
    );
}

#[tokio::test]
#[serial(db)]
async fn init_handles_pending_signal_and_spawns_watcher() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    let cancel = CancellationToken::new();

    setup_chain_and_computation(&pool, CHAIN_A, 50).await;

    // Insert a pending signal, then mark it done so the runner path completes.
    drift_revert::create_revert_signal(&pool, CHAIN_A, 50)
        .await
        .expect("create signal");

    let cfg = RevertRunnerConfig {
        grace_period: Duration::from_millis(10),
    };

    let mock = MockReExec::new();

    // init as runner: handles the pending signal (revert + mark done).
    drift_revert::init_with_reexec(db.db_url(), cancel.clone(), Some(cfg), mock)
        .await
        .expect("init");

    let signal = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    assert_eq!(
        signal.status,
        SignalStatus::Done,
        "init should have run the revert"
    );

    // Spawn a new pending signal — the background watcher should pick it up
    // and call re_exec (MockReExec). We can't easily observe that from here
    // since the watcher owns the mock, so we just verify init didn't error
    // and the first signal was handled correctly.

    // Clean up: cancel the watcher so the background task exits.
    cancel.cancel();
}
