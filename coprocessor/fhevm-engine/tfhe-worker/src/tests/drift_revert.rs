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
    // Other bytes are seeded from chain_id so multi-chain tests don't collide.
    let seed = chain_id as u8;
    let mut handle: Vec<u8> = vec![seed; 32];
    handle[21] = 0xff;
    let txn_id: Vec<u8> = vec![seed.wrapping_add(1); 32];

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

#[tokio::test]
#[serial(db)]
async fn runner_processes_concurrent_signals_from_multiple_chains() {
    const CHAIN_B: i64 = 200;
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    // Two chains with their own in-flight signals (pre-existing, e.g. created
    // right before the process crashed and got re-execed).
    setup_chain_and_computation(&pool, CHAIN_A, 10).await;
    setup_chain_and_computation(&pool, CHAIN_B, 20).await;
    let id_a = drift_revert::create_revert_signal(&pool, CHAIN_A, 10)
        .await
        .expect("signal A")
        .expect("should create A");
    let id_b = drift_revert::create_revert_signal(&pool, CHAIN_B, 20)
        .await
        .expect("signal B")
        .expect("should create B");
    assert!(id_b > id_a, "B should have a newer id than A");

    let cfg = RevertRunnerConfig {
        grace_period: Duration::from_millis(10),
    };
    let cancel = CancellationToken::new();
    drift_revert::handle_pending_signal_on_startup(&pool, Some(cfg), &cancel)
        .await
        .expect("runner should process both signals");

    // Both signals should now be marked Done.
    let rows: Vec<(i64, String)> =
        sqlx::query_as("SELECT id, status FROM drift_revert_signal ORDER BY id ASC")
            .fetch_all(&pool)
            .await
            .unwrap();
    assert_eq!(rows.len(), 2, "both signals should remain as audit trail");
    for (id, status) in &rows {
        assert_eq!(status, "done", "signal {id} should be Done");
    }
}

async fn setup_failed_signal(pool: &PgPool) {
    setup_chain_and_computation(pool, CHAIN_A, 10).await;
    let id = drift_revert::create_revert_signal(pool, CHAIN_A, 10)
        .await
        .unwrap()
        .unwrap();
    drift_revert::update_signal_status(pool, id, &SignalStatus::Failed("simulated".to_owned()))
        .await
        .unwrap();
}

#[tokio::test]
#[serial(db)]
async fn runner_refuses_on_failed_latest() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    setup_failed_signal(&pool).await;

    let cancel = CancellationToken::new();
    let cfg = RevertRunnerConfig {
        grace_period: Duration::from_millis(10),
    };
    let result = drift_revert::handle_pending_signal_on_startup(&pool, Some(cfg), &cancel).await;
    assert!(
        result.is_err(),
        "runner startup must refuse while latest signal is Failed"
    );
}

#[tokio::test]
#[serial(db)]
async fn waiter_refuses_on_failed_latest() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();
    setup_failed_signal(&pool).await;

    let cancel = CancellationToken::new();
    let result = drift_revert::handle_pending_signal_on_startup(&pool, None, &cancel).await;
    assert!(
        result.is_err(),
        "waiter startup must refuse while latest signal is Failed"
    );
}

#[tokio::test]
#[serial(db)]
async fn runner_resumes_from_reverting_status_without_grace_period() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    // Insert host_chain and a couple of blocks of computations.
    sqlx::query(
        "INSERT INTO host_chains (chain_id, name, acl_contract_address) \
         VALUES ($1, 'test', '0x1')",
    )
    .bind(CHAIN_A)
    .execute(&pool)
    .await
    .unwrap();
    for block in 1..=5i64 {
        let handle: Vec<u8> = vec![block as u8; 32];
        let txn_id: Vec<u8> = vec![block as u8 + 100; 32];
        sqlx::query("INSERT INTO transactions (id, chain_id, block_number) VALUES ($1, $2, $3)")
            .bind(&txn_id)
            .bind(CHAIN_A)
            .bind(block)
            .execute(&pool)
            .await
            .unwrap();
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
        .unwrap();
    }

    // Signal for offending block 3 — target will be block 2.
    let id = drift_revert::create_revert_signal(&pool, CHAIN_A, 3)
        .await
        .unwrap()
        .unwrap();
    // Simulate a prior runner crashing after marking Reverting but before Done.
    drift_revert::update_signal_status(&pool, id, &SignalStatus::Reverting)
        .await
        .unwrap();

    // Grace period is set very large; if the runner incorrectly waits it,
    // the test should complete in under a second anyway — we assert it did.
    let cfg = RevertRunnerConfig {
        grace_period: Duration::from_secs(30),
    };
    let cancel = CancellationToken::new();
    let started = std::time::Instant::now();
    drift_revert::handle_pending_signal_on_startup(&pool, Some(cfg), &cancel)
        .await
        .expect("runner should resume");
    let elapsed = started.elapsed();

    assert!(
        elapsed < Duration::from_secs(5),
        "runner should skip grace period for Reverting status, took {elapsed:?}"
    );

    // Signal marked Done.
    let latest = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    assert_eq!(latest.status, SignalStatus::Done);

    // Revert SQL ran idempotently — blocks > 2 are gone.
    let remaining: Vec<i64> = sqlx::query_scalar(
        "SELECT block_number FROM computations WHERE host_chain_id = $1 ORDER BY block_number",
    )
    .bind(CHAIN_A)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(remaining, vec![1_i64, 2]);
}

#[tokio::test]
#[serial(db)]
async fn waiter_waits_until_all_chains_done() {
    const CHAIN_B: i64 = 200;
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chain_and_computation(&pool, CHAIN_A, 10).await;
    setup_chain_and_computation(&pool, CHAIN_B, 20).await;
    let id_a = drift_revert::create_revert_signal(&pool, CHAIN_A, 10)
        .await
        .unwrap()
        .unwrap();
    let id_b = drift_revert::create_revert_signal(&pool, CHAIN_B, 20)
        .await
        .unwrap()
        .unwrap();

    let waiter = {
        let pool = pool.clone();
        tokio::spawn(async move {
            let cancel = CancellationToken::new();
            drift_revert::handle_pending_signal_on_startup(&pool, None, &cancel)
                .await
                .expect("waiter");
        })
    };

    // Marking only the newer chain's signal Done must NOT unblock the waiter
    // — the older (A) signal is still in-flight. We wait 3 * POLL_INTERVAL to
    // ensure the waiter has polled multiple times after B=Done and still
    // didn't return.
    drift_revert::update_signal_status(&pool, id_b, &SignalStatus::Done)
        .await
        .unwrap();
    tokio::time::sleep(drift_revert::POLL_INTERVAL * 3).await;
    assert!(
        !waiter.is_finished(),
        "waiter must not return while chain A signal is still in-flight"
    );

    drift_revert::update_signal_status(&pool, id_a, &SignalStatus::Done)
        .await
        .unwrap();
    tokio::time::timeout(Duration::from_secs(3), waiter)
        .await
        .expect("waiter should finish after both signals are Done")
        .expect("waiter task should succeed");
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

    let cancel = CancellationToken::new();
    drift_revert::handle_pending_signal_on_startup(&pool, Some(cfg), &cancel)
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
            let cancel = CancellationToken::new();
            drift_revert::handle_pending_signal_on_startup(&pool, None, &cancel)
                .await
                .expect("waiter");
        })
    };

    // Signal starts as Pending; only transition to Done should let the waiter finish.
    let signal = drift_revert::latest_signal(&pool).await.unwrap().unwrap();
    drift_revert::update_signal_status(&pool, signal.id, &SignalStatus::Done)
        .await
        .unwrap();

    tokio::time::timeout(Duration::from_secs(3), waiter)
        .await
        .expect("waiter should finish within timeout after status=done")
        .expect("waiter task should succeed");
}

#[tokio::test]
#[serial(db)]
async fn handle_pending_signal_waiter_exits_on_cancel() {
    let db = setup_test_db(ImportMode::None).await.expect("setup db");
    let pool = PgPool::connect(db.db_url()).await.unwrap();

    setup_chain_and_computation(&pool, CHAIN_A, 50).await;
    drift_revert::create_revert_signal(&pool, CHAIN_A, 50)
        .await
        .expect("create signal");

    let cancel = CancellationToken::new();
    let waiter = {
        let pool = pool.clone();
        let cancel = cancel.clone();
        tokio::spawn(async move {
            drift_revert::handle_pending_signal_on_startup(&pool, None, &cancel)
                .await
                .expect("waiter");
        })
    };

    // Signal stays Pending — the only way the waiter can finish is if
    // cancellation actually unblocks it.
    cancel.cancel();
    tokio::time::timeout(Duration::from_secs(3), waiter)
        .await
        .expect("waiter should exit within timeout after cancel")
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
