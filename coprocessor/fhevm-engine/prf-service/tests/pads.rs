mod common;

use alloy::primitives::{Address, B256};
use ciphertext_attestation::PrfOutputRef;
use common::{attestation, set_error, set_output, CONNECTOR, DEADLINE, GRANTS};
use coprocessor_api::MAX_PAD_LENGTH;
use fhevm_engine_common::healthz_server::HealthCheckService;
use prf_service::{
    pads::{read, read_output, Admission, Admissions, PadOutput, PadState, CONNECTOR_LOCK},
    waiter::{PadWaiter, PAD_READY},
    Health,
};
use sqlx::{
    postgres::{PgConnectOptions, PgPoolOptions},
    PgPool,
};
use std::{num::NonZeroUsize, sync::Arc, time::Duration};
use test_harness::instance::{setup_test_db, DBInstance, ImportMode};
use tokio::{task::JoinHandle, time::timeout};
use tokio_util::sync::CancellationToken;

const LIMIT: i64 = 100;
const MAX_ADDS: NonZeroUsize = NonZeroUsize::new(2).unwrap();
const PENDING_MAX_AGE: Duration = Duration::from_secs(600);
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
    let admissions = Arc::new(Admissions::new(
        service.clone(),
        LIMIT,
        MAX_ADDS,
        PENDING_MAX_AGE,
    ));
    (instance, pool, service, admissions)
}

async fn lock_connector(
    pool: &PgPool,
    connector: Address,
) -> sqlx::Transaction<'static, sqlx::Postgres> {
    let mut lock = pool.begin().await.unwrap();
    sqlx::query(CONNECTOR_LOCK)
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

/// Waits until `count` requests wait for a connector lock. Fails after `DEADLINE`.
async fn wait_for_lock_waiters(pool: &PgPool, count: i64) {
    let wait = async {
        while lock_waiters(pool).await < count {
            tokio::task::yield_now().await;
        }
    };
    timeout(DEADLINE, wait)
        .await
        .expect("requests wait for the connector lock");
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
    let pod_b = Arc::new(Admissions::new(service, LIMIT, MAX_ADDS, PENDING_MAX_AGE));
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
    wait_for_lock_waiters(&pool, 1).await;

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
    wait_for_lock_waiters(&pool, MAX_ADDS.get() as i64).await;
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
    wait_for_lock_waiters(&pool, MAX_ADDS.get() as i64).await;
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
async fn expired_pending_rows_do_not_count_in_the_budget() {
    let (_instance, pool, _service, admissions) = test_db().await;
    assert_eq!(
        admissions.admit(&path(1, 64), CONNECTOR).await.unwrap(),
        Admission::Created
    );
    assert_eq!(
        admissions.admit(&path(2, 64), CONNECTOR).await.unwrap(),
        Admission::OverBudget
    );

    // No lane computed the first row in time.
    sqlx::query("UPDATE pads SET created_at = NOW() - make_interval(secs => $1)")
        .bind(PENDING_MAX_AGE.as_secs_f64() + 1.0)
        .execute(&pool)
        .await
        .unwrap();

    assert_eq!(
        admissions.admit(&path(2, 64), CONNECTOR).await.unwrap(),
        Admission::Created
    );
    // A request for the expired row does not wait on it.
    assert_eq!(
        admissions
            .admit(&path(1, 64), OTHER_CONNECTOR)
            .await
            .unwrap(),
        Admission::Expired
    );
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

    assert_eq!(read_output(&pool, &path).await.unwrap(), None);
    set_output(&pool, &path, 1).await;

    let complete = PadState::Complete(output(1));
    assert_eq!(read(&pool, &path).await.unwrap(), Some(complete.clone()));
    assert_eq!(
        read_output(&pool, &path).await.unwrap(),
        Some((attestation(1), vec![1; 32]))
    );
    assert_eq!(
        admissions.admit(&path, OTHER_CONNECTOR).await.unwrap(),
        Admission::Existing(complete)
    );

    // A changed output: the attestation and the body come from the same read.
    set_output(&pool, &path, 2).await;
    assert_eq!(
        read_output(&pool, &path).await.unwrap(),
        Some((attestation(2), vec![2; 32]))
    );
}

#[tokio::test]
async fn unknown_attestation_version_is_an_error() {
    let (_instance, pool, _service, admissions) = test_db().await;
    let path = path(1, 64);
    admissions.admit(&path, CONNECTOR).await.unwrap();
    set_output(&pool, &path, 1).await;
    sqlx::query("UPDATE pads SET attestation_version = 2")
        .execute(&pool)
        .await
        .unwrap();

    assert!(read(&pool, &path).await.is_err());
    assert!(read_output(&pool, &path).await.is_err());
}

#[tokio::test]
async fn health_needs_the_rights_on_pads() {
    let (_instance, pool, service, _admissions) = test_db().await;
    let health = Health(service);
    assert!(health.health_check().await.is_healthy());

    for sql in [
        "REVOKE INSERT ON pads FROM prf_service",
        "GRANT INSERT ON pads TO prf_service",
        "REVOKE SELECT ON pads FROM prf_service",
    ] {
        sqlx::query(sql).execute(&pool).await.unwrap();
        let healthy = sql.starts_with("GRANT");
        assert_eq!(health.health_check().await.is_healthy(), healthy, "{sql}");
    }
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
    insert(i32::from(MAX_PAD_LENGTH)).await.unwrap();
    let err = insert(i32::from(MAX_PAD_LENGTH) + 1).await.unwrap_err();
    let code = err.as_database_error().and_then(|e| e.code());
    // 23514 is check_violation.
    assert_eq!(code.as_deref(), Some("23514"), "{err}");
}

fn output(byte: u8) -> PadOutput {
    PadOutput {
        body_length: 32,
        attestation: attestation(byte),
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
) -> (Arc<PadWaiter>, CancellationToken, JoinHandle<()>) {
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
    set_output(&pool, &short, 1).await;
    assert_eq!(finished(short_wait).await, PadState::Complete(output(1)));
    assert!(!long_wait.is_finished());

    set_error(&pool, &long).await;
    assert_eq!(finished(long_wait).await, PadState::Failed);

    cancel.cancel();
    run.await.unwrap();
}

#[tokio::test]
async fn a_new_row_wakes_the_lane_of_its_instance() {
    let (_instance, pool, _service, admissions) = test_db().await;
    let mut listener = sqlx::postgres::PgListener::connect_with(&pool)
        .await
        .unwrap();
    listener.listen("pad_work_available").await.unwrap();

    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    let notification = timeout(DEADLINE, listener.recv()).await.unwrap().unwrap();
    assert_eq!(notification.payload(), row.prf_id.to_string());

    // A request that joins the row adds no row, so it wakes nobody.
    admissions
        .admit(&path(1, 64), OTHER_CONNECTOR)
        .await
        .unwrap();
    assert!(timeout(Duration::from_millis(500), listener.recv())
        .await
        .is_err());
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
    set_output(&pool, &row, 1).await;

    assert_eq!(finished(wait).await, PadState::Complete(output(1)));
}

#[tokio::test]
async fn poll_wakes_a_request_whose_row_is_deleted() {
    let (_instance, pool, service, admissions) = test_db().await;
    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    let (waiter, _cancel, _run) = start_waiter(&pool, &service, Duration::from_millis(100)).await;
    let wait = start_wait(&waiter, row).await;

    sqlx::query("DELETE FROM pads")
        .execute(&pool)
        .await
        .unwrap();

    assert!(timeout(DEADLINE, wait).await.unwrap().unwrap().is_err());
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

    set_output(&pool, &row, 1).await;
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
    set_output(&pool, &row, 1).await;
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
async fn waiter_makes_a_new_listener_after_a_failed_reconnect() {
    let (instance, pool, _service, admissions) = test_db().await;
    let row = path(1, 64);
    admissions.admit(&row, CONNECTOR).await.unwrap();
    // A role of this test only: the other tests use `prf_service` at the same time.
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE prf_probe LOGIN PASSWORD 'prf_probe';
         EXCEPTION WHEN duplicate_object THEN NULL; END $$;
         ALTER ROLE prf_probe LOGIN;
         GRANT SELECT ON pads TO prf_probe;",
    )
    .execute(&pool)
    .await
    .unwrap();
    let options: PgConnectOptions = instance.db_url().parse().unwrap();
    let probe = PgPoolOptions::new()
        .acquire_timeout(Duration::from_millis(200))
        .connect_with(options.username("prf_probe").password("prf_probe"))
        .await
        .unwrap();
    let (waiter, _cancel, run) = start_waiter(&pool, &probe, Duration::from_millis(100)).await;
    let wait = start_wait(&waiter, row).await;

    // Cut every connection of the role and refuse new ones: sqlx cannot connect again.
    sqlx::raw_sql(
        "ALTER ROLE prf_probe NOLOGIN;
         SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE usename = 'prf_probe';",
    )
    .execute(&pool)
    .await
    .unwrap();
    tokio::time::sleep(Duration::from_millis(600)).await;
    assert!(!run.is_finished());

    sqlx::query("ALTER ROLE prf_probe LOGIN")
        .execute(&pool)
        .await
        .unwrap();
    set_output(&pool, &row, 1).await;
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
    as_role(pool, "prf_service", sql).await
}

/// Runs `sql` as `role` in a transaction, then rolls the transaction back.
async fn as_role(pool: &PgPool, role: &str, sql: &str) -> Result<(), sqlx::Error> {
    let mut tx = pool.begin().await?;
    sqlx::raw_sql(&format!("SET LOCAL ROLE {role}"))
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
    sqlx::raw_sql("DROP ROLE IF EXISTS prf_service; DROP ROLE IF EXISTS pad_worker")
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

    for allowed in [
        INSERT_PAD,
        "SELECT prf_instance, signature FROM pads",
        "SELECT ready_at, heartbeat_at FROM prf_lanes",
    ] {
        as_prf_service(&pool, allowed).await.expect(allowed);
    }

    for denied in [
        "SELECT 1 FROM computations",
        "SELECT 1 FROM ciphertexts",
        "SELECT 1 FROM keys",
        "UPDATE pads SET is_error = false",
        "UPDATE pads SET signature = NULL",
        "DELETE FROM pads",
        "INSERT INTO prf_lanes (prf_instance) VALUES (1)",
        "UPDATE prf_lanes SET ready_at = NOW()",
    ] {
        assert_denied(as_prf_service(&pool, denied).await, denied);
    }
}

#[tokio::test]
async fn pad_worker_role_writes_only_outputs_and_lane_state() {
    let (_instance, pool) = admin_db().await;
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE pad_worker NOLOGIN;
         EXCEPTION WHEN duplicate_object THEN NULL; END $$",
    )
    .execute(&pool)
    .await
    .unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();

    for allowed in [
        "SELECT 1 FROM pads WHERE signature IS NULL FOR UPDATE SKIP LOCKED",
        "UPDATE pads SET claimed_at = NOW(), attempts = attempts + 1, error_message = 'e',
             is_error = true, s3_verified_at = NOW()",
        "UPDATE pads SET ciphertext128 = NULL, ciphertext_digest = NULL, ciphertext128_digest = NULL,
             signer = NULL, signature = NULL, attestation_version = NULL, ciphertext64 = NULL",
        "DELETE FROM pads",
        "INSERT INTO prf_lanes (prf_instance) VALUES (1) ON CONFLICT DO NOTHING",
        "UPDATE prf_lanes SET ready_at = NOW(), heartbeat_at = NOW()",
        "SELECT reference_digest, reference_mismatch_at FROM prf_lanes",
        "SELECT 1 FROM keys",
    ] {
        as_role(&pool, "pad_worker", allowed).await.expect(allowed);
    }

    for denied in [
        "SELECT 1 FROM computations",
        "SELECT 1 FROM ciphertexts",
        INSERT_PAD,
        "UPDATE pads SET triggered_by = triggered_by",
        "UPDATE pads SET label = label",
        "UPDATE prf_lanes SET reference_digest = NULL",
        "UPDATE prf_lanes SET reference_mismatch_at = NOW()",
        "DELETE FROM prf_lanes",
    ] {
        assert_denied(as_role(&pool, "pad_worker", denied).await, denied);
    }
}

#[tokio::test]
async fn pad_worker_grants_keep_newer_rights_and_drop_forbidden_ones() {
    let (_instance, pool) = admin_db().await;
    sqlx::raw_sql(
        "DO $$ BEGIN CREATE ROLE pad_worker NOLOGIN;
         EXCEPTION WHEN duplicate_object THEN NULL; END $$;
         ALTER TABLE pads ADD COLUMN future_output BYTEA NULL;
         GRANT UPDATE (future_output) ON pads TO pad_worker;
         GRANT UPDATE (triggered_by) ON pads TO pad_worker;
         GRANT UPDATE (reference_digest) ON prf_lanes TO pad_worker;",
    )
    .execute(&pool)
    .await
    .unwrap();

    // An older copy of the file does not know `future_output`: the lane keeps that right.
    sqlx::raw_sql(GRANTS).execute(&pool).await.unwrap();

    as_role(&pool, "pad_worker", "UPDATE pads SET future_output = NULL")
        .await
        .expect("a newer right stays");
    for denied in [
        "UPDATE pads SET triggered_by = triggered_by",
        "UPDATE prf_lanes SET reference_digest = NULL",
    ] {
        assert_denied(as_role(&pool, "pad_worker", denied).await, denied);
    }
}
