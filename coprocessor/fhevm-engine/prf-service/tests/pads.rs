mod common;

use alloy::primitives::{Address, B256};
use ciphertext_attestation::{prf::Version, PrfOutputAttestation, PrfOutputRef};
use common::GRANTS;
use prf_service::{
    pads::{read, Admission, Admissions, PadOutput, PadState},
    waiter::{PadWaiter, PAD_READY},
};
use sqlx::{postgres::PgPoolOptions, PgPool};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::{task::JoinHandle, time::timeout};
use tokio_util::sync::CancellationToken;

const LIMIT: i64 = 100;
const MAX_ADDS: NonZeroUsize = NonZeroUsize::new(2).unwrap();
const CONNECTOR: Address = Address::repeat_byte(0x11);
const OTHER_CONNECTOR: Address = Address::repeat_byte(0x22);

/// A database and an admin pool.
async fn admin_db() -> (DBInstance, PgPool) {
    let instance = setup_test_db(ImportMode::None).await.expect("test db");
    let pool = PgPoolOptions::new()
        .max_connections(16)
        .connect(instance.db_url())
        .await
        .expect("pool");
    (instance, pool)
}

/// Also a pool of the `prf_service` role, and the admissions of one pod on it. The admin pool plays
/// the pad-worker.
async fn test_db() -> (DBInstance, PgPool, PgPool, Arc<Admissions>) {
    let (instance, pool) = admin_db().await;
    let service = common::prf_service_pool(&pool, instance.db_url(), 16).await;
    let admissions = Arc::new(Admissions::new(service.clone(), LIMIT, MAX_ADDS));
    (instance, pool, service, admissions)
}

async fn lock_connector(
    pool: &PgPool,
    connector: Address,
) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut lock = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock(hashtextextended('pads:' || encode($1, 'hex'), 0))")
        .bind(connector.as_slice())
        .execute(lock.as_mut())
        .await
        .unwrap();
    lock
}

async fn lock_waiters(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM pg_locks WHERE locktype = 'advisory' AND NOT granted")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Checks that exactly one request created the row, and that the others got `other`.
async fn assert_one_created(
    requests: Vec<JoinHandle<anyhow::Result<Admission>>>,
    other: Admission,
) {
    let mut created = 0;
    for request in requests {
        match request.await.unwrap().unwrap() {
            Admission::Created => created += 1,
            admission => assert_eq!(admission, other),
        }
    }
    assert_eq!(created, 1);
}

/// Starts one `admit` of `connector` for each label.
fn spawn_admits(
    admissions: &Arc<Admissions>,
    labels: std::ops::Range<u8>,
    connector: Address,
) -> Vec<JoinHandle<anyhow::Result<Admission>>> {
    labels
        .map(|label| {
            let admissions = admissions.clone();
            tokio::spawn(async move { admissions.admit(&path(label, 64), connector).await })
        })
        .collect()
}

fn path(label: u8, pad_length: u16) -> PrfOutputRef {
    PrfOutputRef {
        prf_id: 0,
        label: B256::repeat_byte(label),
        pad_length,
    }
}

async fn set_output(pool: &PgPool, path: &PrfOutputRef, output: &PadOutput) {
    let attestation = &output.attestation;
    sqlx::query(
        "UPDATE pads SET ciphertext128 = $4, ciphertext_digest = $5, sns_ciphertext_digest = $6,
                signer = $7, signature = $8
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf_id))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .bind(&output.ciphertext128)
    .bind(attestation.ciphertext_digest.as_slice())
    .bind(attestation.sns_ciphertext_digest.as_slice())
    .bind(attestation.signer.as_slice())
    .bind(&attestation.signature)
    .execute(pool)
    .await
    .unwrap();
}

async fn set_error(pool: &PgPool, path: &PrfOutputRef) {
    sqlx::query(
        "UPDATE pads SET is_error = true
          WHERE prf_instance = $1 AND label = $2 AND pad_length = $3",
    )
    .bind(i32::from(path.prf_id))
    .bind(path.label.as_slice())
    .bind(i32::from(path.pad_length))
    .execute(pool)
    .await
    .unwrap();
}

#[tokio::test]
async fn concurrent_requests_make_one_row() {
    let (_instance, _pool, _service, admissions) = test_db().await;
    let requests = (0..13)
        .map(|i| {
            let admissions = admissions.clone();
            let connector = Address::repeat_byte(i + 1);
            tokio::spawn(async move { admissions.admit(&path(1, 64), connector).await })
        })
        .collect();
    assert_one_created(requests, Admission::Existing(PadState::Pending)).await;
}

#[tokio::test]
async fn concurrent_requests_of_a_connector_stay_in_its_budget() {
    let (_instance, _pool, _service, admissions) = test_db().await;
    let requests = spawn_admits(&admissions, 0..10, CONNECTOR);
    assert_one_created(requests, Admission::OverBudget).await;
}

#[tokio::test]
async fn requests_on_two_pods_stay_in_the_budget() {
    let (_instance, _pool, service, pod_a) = test_db().await;
    let pod_b = Arc::new(Admissions::new(service, LIMIT, MAX_ADDS));
    let requests: Vec<_> = (0..10)
        .map(|label| {
            let pod = if label % 2 == 0 {
                pod_a.clone()
            } else {
                pod_b.clone()
            };
            tokio::spawn(async move { pod.admit(&path(label, 64), CONNECTOR).await })
        })
        .collect();
    assert_one_created(requests, Admission::OverBudget).await;
}

#[tokio::test]
async fn connector_over_budget_joins_a_row_added_during_its_admission() {
    let (_instance, pool, _service, admissions) = test_db().await;
    assert_eq!(
        admissions.admit(&path(1, 64), CONNECTOR).await.unwrap(),
        Admission::Created
    );

    // Hold the lock of CONNECTOR. Its `admit` then stops after the first read.
    let lock = lock_connector(&pool, CONNECTOR).await;
    let blocked = tokio::spawn({
        let admissions = admissions.clone();
        async move { admissions.admit(&path(2, 64), CONNECTOR).await }
    });
    while lock_waiters(&pool).await == 0 {
        tokio::task::yield_now().await;
    }

    // Another connector adds the row now.
    assert_eq!(
        admissions
            .admit(&path(2, 64), OTHER_CONNECTOR)
            .await
            .unwrap(),
        Admission::Created
    );
    lock.rollback().await.unwrap();
    assert_eq!(
        blocked.await.unwrap().unwrap(),
        Admission::Existing(PadState::Pending)
    );
}

#[tokio::test]
async fn requests_of_a_connector_wait_in_memory_not_on_the_lock() {
    let (_instance, pool, _service, admissions) = test_db().await;
    // Hold the lock of CONNECTOR, so its requests stop before they can add a row.
    let lock = lock_connector(&pool, CONNECTOR).await;
    let requests = spawn_admits(&admissions, 0..10, CONNECTOR);
    while lock_waiters(&pool).await < MAX_ADDS.get() as i64 {
        tokio::task::yield_now().await;
    }
    // Only MAX_ADDS requests wait for the lock with a connection. The others wait in memory.
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(lock_waiters(&pool).await, MAX_ADDS.get() as i64);

    // Another connector is not blocked.
    assert_eq!(
        admissions
            .admit(&path(20, 64), OTHER_CONNECTOR)
            .await
            .unwrap(),
        Admission::Created
    );

    lock.rollback().await.unwrap();
    assert_one_created(requests, Admission::OverBudget).await;
}

#[tokio::test]
async fn cancelled_requests_keep_their_slots() {
    let (_instance, pool, _service, admissions) = test_db().await;
    let lock = lock_connector(&pool, CONNECTOR).await;
    let cancelled = spawn_admits(&admissions, 0..MAX_ADDS.get() as u8, CONNECTOR);
    while lock_waiters(&pool).await < MAX_ADDS.get() as i64 {
        tokio::task::yield_now().await;
    }
    for request in &cancelled {
        request.abort();
    }

    // The cancelled requests still wait for the lock, so the new requests wait in memory.
    let requests = spawn_admits(&admissions, 10..10 + MAX_ADDS.get() as u8, CONNECTOR);
    tokio::time::sleep(Duration::from_millis(200)).await;
    assert_eq!(lock_waiters(&pool).await, MAX_ADDS.get() as i64);

    // A cancelled request adds its row first, so the new requests are over the budget.
    lock.rollback().await.unwrap();
    for request in requests {
        assert_eq!(request.await.unwrap().unwrap(), Admission::OverBudget);
    }
}

#[tokio::test]
async fn budget_counts_the_pending_pad_bits_of_each_connector() {
    let (_instance, pool, _service, admissions) = test_db().await;
    let admit = |path, connector| {
        let admissions = admissions.clone();
        async move { admissions.admit(&path, connector).await.unwrap() }
    };

    assert_eq!(admit(path(1, 64), CONNECTOR).await, Admission::Created);
    assert_eq!(admit(path(2, 64), CONNECTOR).await, Admission::OverBudget);
    // Another length of the same label is another output.
    assert_eq!(admit(path(1, 36), CONNECTOR).await, Admission::Created);
    // Joining a row takes no slot, and the other connector has its own budget.
    assert_eq!(
        admit(path(1, 64), OTHER_CONNECTOR).await,
        Admission::Existing(PadState::Pending)
    );
    assert_eq!(
        admit(path(2, 64), OTHER_CONNECTOR).await,
        Admission::Created
    );

    // A failed row frees its slots.
    set_error(&pool, &path(1, 64)).await;
    assert_eq!(
        admit(path(1, 64), CONNECTOR).await,
        Admission::Existing(PadState::Failed)
    );
    assert_eq!(admit(path(3, 64), CONNECTOR).await, Admission::Created);
}

#[tokio::test]
async fn complete_row_gives_the_output() {
    let (_instance, pool, _service, admissions) = test_db().await;
    let path = path(1, 64);
    assert_eq!(read(&pool, &path).await.unwrap(), None);
    assert_eq!(
        admissions.admit(&path, CONNECTOR).await.unwrap(),
        Admission::Created
    );

    let output = output(1);
    set_output(&pool, &path, &output).await;

    let complete = PadState::Complete(output);
    assert_eq!(read(&pool, &path).await.unwrap(), Some(complete.clone()));
    assert_eq!(
        admissions.admit(&path, OTHER_CONNECTOR).await.unwrap(),
        Admission::Existing(complete)
    );
}

#[tokio::test]
async fn table_refuses_a_pad_length_above_the_max() {
    let (_instance, pool) = admin_db().await;
    let label = B256::repeat_byte(1);
    let insert = |pad_length: i32| {
        sqlx::query(
            "INSERT INTO pads (prf_instance, label, pad_length, triggered_by)
             VALUES (0, $1, $2, $3)",
        )
        .bind(label.as_slice())
        .bind(pad_length)
        .bind(CONNECTOR.as_slice())
        .execute(&pool)
    };
    insert(2048).await.unwrap();
    let err = insert(2049).await.unwrap_err();
    let code = err.as_database_error().and_then(|e| e.code());
    // 23514 is check_violation.
    assert_eq!(code.as_deref(), Some("23514"), "{err}");
}

/// A different byte in each field, so a test finds two swapped columns.
fn output(byte: u8) -> PadOutput {
    PadOutput {
        ciphertext128: vec![byte; 32],
        attestation: PrfOutputAttestation {
            version: Version::V1,
            ciphertext_digest: B256::repeat_byte(byte + 1),
            sns_ciphertext_digest: B256::repeat_byte(byte + 2),
            signer: Address::repeat_byte(byte + 3),
            signature: vec![byte + 4; 65],
        },
    }
}

async fn notify(pool: &PgPool, payload: &str) {
    sqlx::query("SELECT pg_notify($1, $2)")
        .bind(PAD_READY)
        .bind(payload)
        .execute(pool)
        .await
        .unwrap();
}

fn payload(path: &PrfOutputRef) -> String {
    format!("{}:{:x}:{}", path.prf_id, path.label, path.pad_length)
}

/// Starts the waiter and returns once it listens.
async fn start_waiter(
    pool: &PgPool,
    service: &PgPool,
    poll_interval: Duration,
) -> (
    Arc<PadWaiter>,
    CancellationToken,
    JoinHandle<anyhow::Result<()>>,
) {
    let waiter = Arc::new(PadWaiter::new(service.clone(), poll_interval));
    let cancel = CancellationToken::new();
    let run = tokio::spawn({
        let (waiter, cancel) = (waiter.clone(), cancel.clone());
        async move { waiter.run(cancel).await }
    });
    while listeners(pool).await == 0 {
        tokio::task::yield_now().await;
    }
    (waiter, cancel, run)
}

async fn listeners(pool: &PgPool) -> i64 {
    sqlx::query_scalar("SELECT count(*) FROM pg_stat_activity WHERE query LIKE 'LISTEN%'")
        .fetch_one(pool)
        .await
        .unwrap()
}

/// Starts a request that waits for the row of `path`, and returns once it waits.
async fn start_wait(
    waiter: &Arc<PadWaiter>,
    path: PrfOutputRef,
) -> JoinHandle<anyhow::Result<PadState>> {
    let wait = tokio::spawn({
        let waiter = waiter.clone();
        async move { waiter.wait(&path).await }
    });
    tokio::time::sleep(Duration::from_millis(100)).await;
    wait
}

const NO_POLL: Duration = Duration::from_secs(3600);
const DEADLINE: Duration = Duration::from_secs(5);

/// The state that `wait` gives, within `DEADLINE`.
async fn finished(wait: JoinHandle<anyhow::Result<PadState>>) -> PadState {
    timeout(DEADLINE, wait).await.unwrap().unwrap().unwrap()
}

#[tokio::test]
async fn trigger_wakes_the_request_of_each_row() {
    let (_instance, pool, service, admissions) = test_db().await;
    let (short, long) = (path(1, 36), path(1, 64));
    for path in [&short, &long] {
        admissions.admit(path, CONNECTOR).await.unwrap();
    }
    let (waiter, cancel, run) = start_waiter(&pool, &service, NO_POLL).await;
    let short_wait = start_wait(&waiter, short).await;
    let long_wait = start_wait(&waiter, long).await;

    // A bad payload wakes nobody.
    notify(&pool, "not a row").await;
    // The trigger sends `pad_ready`.
    set_output(&pool, &short, &output(1)).await;
    assert_eq!(finished(short_wait).await, PadState::Complete(output(1)));
    assert!(!long_wait.is_finished());

    set_error(&pool, &long).await;
    assert_eq!(finished(long_wait).await, PadState::Failed);

    cancel.cancel();
    run.await.unwrap().unwrap();
}

#[tokio::test]
async fn poll_wakes_a_request_without_notification() {
    let (_instance, pool, service, admissions) = test_db().await;
    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    let (waiter, _cancel, _run) = start_waiter(&pool, &service, Duration::from_millis(100)).await;
    let wait = start_wait(&waiter, row).await;

    // Without the trigger, only the poll can wake the request.
    sqlx::query("ALTER TABLE pads DISABLE TRIGGER pads_ready")
        .execute(&pool)
        .await
        .unwrap();
    set_output(&pool, &row, &output(1)).await;

    assert_eq!(finished(wait).await, PadState::Complete(output(1)));
}

#[tokio::test]
async fn failed_poll_does_not_stop_the_waiter() {
    let (_instance, pool, service, admissions) = test_db().await;
    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    let (waiter, _cancel, run) = start_waiter(&pool, &service, Duration::from_millis(50)).await;
    let wait = start_wait(&waiter, row).await;

    // The poll query fails while the table has another name.
    sqlx::query("ALTER TABLE pads RENAME TO pads_hidden")
        .execute(&pool)
        .await
        .unwrap();
    tokio::time::sleep(Duration::from_millis(300)).await;
    assert!(!run.is_finished());
    sqlx::query("ALTER TABLE pads_hidden RENAME TO pads")
        .execute(&pool)
        .await
        .unwrap();

    set_output(&pool, &row, &output(1)).await;
    notify(&pool, &payload(&row)).await;
    assert_eq!(finished(wait).await, PadState::Complete(output(1)));
}

#[tokio::test]
async fn waiter_reads_the_rows_after_a_lost_connection() {
    let (_instance, pool, service, admissions) = test_db().await;
    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    let (waiter, _cancel, run) = start_waiter(&pool, &service, NO_POLL).await;
    let wait = start_wait(&waiter, row).await;

    // Complete the row with no notification, then cut the listen connection.
    sqlx::query("ALTER TABLE pads DISABLE TRIGGER pads_ready")
        .execute(&pool)
        .await
        .unwrap();
    set_output(&pool, &row, &output(1)).await;
    sqlx::query(
        "SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE query LIKE 'LISTEN%'",
    )
    .execute(&pool)
    .await
    .unwrap();

    assert_eq!(finished(wait).await, PadState::Complete(output(1)));
    assert!(!run.is_finished());
}

#[tokio::test]
async fn wait_for_a_missing_row_fails() {
    let (_instance, _pool, service, _admissions) = test_db().await;
    let waiter = PadWaiter::new(service, NO_POLL);
    assert!(waiter.wait(&path(1, 64)).await.is_err());
}

const INSERT_PAD: &str = "INSERT INTO pads (prf_instance, label, pad_length, triggered_by)
    VALUES (1, decode(repeat('ab', 32), 'hex'), 64, decode(repeat('11', 20), 'hex'))
    ON CONFLICT DO NOTHING RETURNING label";

/// Runs `sql` as `prf_service` in a transaction, then rolls the transaction back.
async fn as_prf_service(pool: &PgPool, sql: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::raw_sql("SET LOCAL ROLE prf_service")
        .execute(&mut *tx)
        .await?;
    let result = sqlx::raw_sql(sql).execute(&mut *tx).await.map(|_| ());
    tx.rollback().await?;
    result
}

fn assert_denied(result: Result<(), sqlx::Error>, sql: &str) {
    let err = result.expect_err(sql);
    let code = err.as_database_error().and_then(|e| e.code());
    assert_eq!(code.as_deref(), Some("42501"), "{sql}: {err}");
}

#[tokio::test]
async fn grants_skip_a_missing_role() {
    let (_instance, pool) = admin_db().await;
    sqlx::raw_sql("DROP ROLE IF EXISTS prf_service")
        .execute(&pool)
        .await
        .unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();
}

#[tokio::test]
async fn prf_service_role_writes_only_the_queue() {
    let (_instance, pool) = admin_db().await;
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE prf_service NOLOGIN;
         EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    // The grants run on each deploy, so a second run must give the same result.
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();

    for allowed in [INSERT_PAD, "SELECT prf_instance, signature FROM pads"] {
        as_prf_service(&pool, allowed).await.expect(allowed);
    }

    for denied in [
        "SELECT 1 FROM computations",
        "SELECT 1 FROM ciphertexts",
        "SELECT 1 FROM keys",
        "UPDATE pads SET is_error = false",
        "UPDATE pads SET signature = NULL",
        "DELETE FROM pads",
    ] {
        assert_denied(as_prf_service(&pool, denied).await, denied);
    }
}
