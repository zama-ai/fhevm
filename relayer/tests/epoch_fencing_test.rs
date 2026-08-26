//! Epoch-fencing tests (build-order step 8).
//!
//! Exercised directly against the repositories, against a schema-isolated database: a write
//! stamped with a stale dispatcher epoch is refused, a `NULL`-owner row is claimable by any
//! epoch, and the specific terminal-write interleaving the build-order brief describes - an
//! ex-holder's fast-failing send racing its successor's receipt - cannot corrupt a row's
//! status once the fence is in place.

mod common;

use alloy::primitives::U256;
use common::test_schema::TestSchema;
use fhevm_relayer::config::settings::{DispatcherLockConfig, Settings};
use fhevm_relayer::orchestrator::{DispatcherLock, LockState};
use fhevm_relayer::store::sql::repositories::Repositories;
use prometheus::Registry;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use uuid::Uuid;

const TEST_CONFIG_PATH: &str = "tests/relayer-test-config.yaml";

/// Spawn `lock.run(...)` in the background and wait until it reports `Held`. Never shut down
/// explicitly - nextest gives each test its own process, so there is nothing to release.
async fn connect_and_hold(config: &DispatcherLockConfig, database_url: &str) -> DispatcherLock {
    let lock = DispatcherLock::connect(config, database_url)
        .await
        .expect("Failed to connect dispatcher lock");
    {
        let lock = lock.clone();
        tokio::spawn(async move { lock.run(CancellationToken::new()).await });
    }
    tokio::time::timeout(Duration::from_secs(5), async {
        while lock.state() != LockState::Held {
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("lock did not converge on Held within budget");
    lock
}

/// Two `Repositories` sets pointed at the same schema, each backed by its own held, live
/// `DispatcherLock` on a distinct key so both can be `Held` simultaneously without contending
/// for the same advisory lock. `epoch_b` is always greater than `epoch_a`: `owner_epoch` is
/// minted from one sequence shared by the whole database (`dispatcher_epoch_seq`), not scoped
/// per key, and `lock_b` acquires after `lock_a`. This models an old pod (`repos_a`, carrying
/// a stale epoch) and its successor (`repos_b`, current) without forcing a real failover.
struct TwoEpochSetup {
    repos_a: Repositories,
    epoch_a: i64,
    repos_b: Repositories,
    epoch_b: i64,
    raw_pool: sqlx::PgPool,
    /// For building further `Repositories`/`DispatcherLock` instances against the same
    /// schema, e.g. a third, deliberately-unheld lock (`current_epoch() == None`).
    storage: fhevm_relayer::config::settings::StorageConfig,
    database_url: String,
    lock_config: DispatcherLockConfig,
    _schema: TestSchema,
}

impl TwoEpochSetup {
    async fn new() -> Self {
        let schema = TestSchema::new()
            .await
            .expect("Failed to create test schema");

        let mut settings =
            Settings::new(Some(TEST_CONFIG_PATH.to_string())).expect("Failed to load config");
        // OnceLocks the relayer's startup would normally fill; nextest gives each test its own
        // process, so this needs doing once per test (see `sweep_test.rs` for the same pattern).
        fhevm_relayer::metrics::init_db_metrics(&Registry::new(), settings.metrics.clone());
        fhevm_relayer::metrics::init_statuses_metrics(&Registry::new(), settings.metrics.clone());
        settings.storage.sql_database_url = schema.database_url();
        settings.storage.app_pool.max_connections = 4;
        settings.storage.app_pool.min_connections = 0;
        settings.storage.cron_pool.max_connections = 2;
        settings.storage.cron_pool.min_connections = 0;

        let fast = DispatcherLockConfig {
            poll_interval: Duration::from_millis(20),
            heartbeat_interval: Duration::from_millis(50),
            heartbeat_timeout: Duration::from_secs(2),
            heartbeat_failures_before_exit: 3,
            connect_timeout: Duration::from_secs(5),
            key_override: Some(1_700_000_001),
        };
        let lock_a = connect_and_hold(&fast, &schema.database_url()).await;
        let epoch_a = lock_a.current_epoch().expect("epoch minted for a");

        let config_b = DispatcherLockConfig {
            key_override: Some(1_700_000_002),
            ..fast
        };
        let lock_b = connect_and_hold(&config_b, &schema.database_url()).await;
        let epoch_b = lock_b.current_epoch().expect("epoch minted for b");

        assert!(
            epoch_b > epoch_a,
            "test precondition: epoch_b ({epoch_b}) must be minted after epoch_a ({epoch_a})"
        );

        let repos_a = Repositories::new(settings.storage.clone(), lock_a)
            .await
            .expect("Failed to create repositories a");
        let repos_b = Repositories::new(settings.storage.clone(), lock_b)
            .await
            .expect("Failed to create repositories b");

        let raw_pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&schema.database_url())
            .await
            .expect("Failed to connect raw pool");

        Self {
            repos_a,
            epoch_a,
            repos_b,
            epoch_b,
            raw_pool,
            storage: settings.storage.clone(),
            database_url: schema.database_url(),
            lock_config: fast,
            _schema: schema,
        }
    }

    /// Insert a `public_decrypt_req` row directly, bypassing the repository, so the row's
    /// starting state (status, `owner_epoch`) is exactly what each test wants regardless of
    /// what the epoch-aware `INSERT` under test would itself produce.
    async fn insert_public_decrypt_row(&self, status: &str, owner_epoch: Option<i64>) -> Vec<u8> {
        let ext_job_id = Uuid::new_v4();
        let int_job_id = Uuid::new_v4().as_bytes().to_vec();

        sqlx::query(
            r#"
            INSERT INTO public_decrypt_req (ext_job_id, int_job_id, req, req_status, owner_epoch)
            VALUES ($1, $2, '{}'::jsonb, $3::req_status, $4)
            "#,
        )
        .bind(ext_job_id)
        .bind(&int_job_id)
        .bind(status)
        .bind(owner_epoch)
        .execute(&self.raw_pool)
        .await
        .expect("Failed to insert row");

        int_job_id
    }

    async fn status_and_owner_epoch(&self, int_job_id: &[u8]) -> (String, Option<i64>) {
        let row = sqlx::query(
            "SELECT req_status::text as status, owner_epoch FROM public_decrypt_req WHERE int_job_id = $1",
        )
        .bind(int_job_id)
        .fetch_one(&self.raw_pool)
        .await
        .expect("Failed to read row");
        (row.get("status"), row.get("owner_epoch"))
    }

    async fn gw_reference_id(&self, int_job_id: &[u8]) -> Option<Vec<u8>> {
        let row =
            sqlx::query("SELECT gw_reference_id FROM public_decrypt_req WHERE int_job_id = $1")
                .bind(int_job_id)
                .fetch_one(&self.raw_pool)
                .await
                .expect("Failed to read row");
        row.get("gw_reference_id")
    }
}

/// A write stamped with a stale epoch is refused (zero rows, row untouched); the same write
/// from the row's actual current owner succeeds and re-stamps its own epoch.
#[tokio::test]
async fn test_stale_epoch_write_is_refused_current_epoch_write_succeeds() {
    let setup = TwoEpochSetup::new().await;

    // Row already claimed under the CURRENT epoch - e.g. the sweep just claimed it.
    let int_job_id = setup
        .insert_public_decrypt_row("processing", Some(setup.epoch_b))
        .await;

    // The stale pod (epoch_a) tries to drive it forward - refused.
    let rows = setup
        .repos_a
        .public_decrypt
        .update_status_to_tx_in_flight(&int_job_id)
        .await
        .expect("query a");
    assert_eq!(rows, 0, "a write from a stale epoch must be refused");
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(
        status, "processing",
        "status must not move under a refused write"
    );
    assert_eq!(
        owner_epoch,
        Some(setup.epoch_b),
        "owner_epoch must not change under a refused write"
    );

    // The current owner (epoch_b) succeeds.
    let rows = setup
        .repos_b
        .public_decrypt
        .update_status_to_tx_in_flight(&int_job_id)
        .await
        .expect("query b");
    assert_eq!(rows, 1, "a write from the current epoch must succeed");
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(status, "tx_in_flight");
    assert_eq!(owner_epoch, Some(setup.epoch_b));
}

/// The other direction: a *newer* epoch must be able to write a row an *older* epoch last
/// touched - this is the fencing-token property (`owner_epoch IS NULL OR owner_epoch <=
/// $epoch`, not `= $epoch`) that lets a successor take over a genuinely dead predecessor's
/// rows (startup recovery re-driving this same pod's own previous incarnation's rows, or the
/// sweep reclaiming a peer's). Equality alone would wrongly refuse this forever.
#[tokio::test]
async fn test_newer_epoch_write_succeeds_over_an_older_owned_row() {
    let setup = TwoEpochSetup::new().await;

    // Row last touched by the OLDER epoch (epoch_a) - e.g. this pod's own previous
    // incarnation, or a dead peer's claim.
    let int_job_id = setup
        .insert_public_decrypt_row("processing", Some(setup.epoch_a))
        .await;

    let rows = setup
        .repos_b
        .public_decrypt
        .update_status_to_tx_in_flight(&int_job_id)
        .await
        .expect("query b");
    assert_eq!(
        rows, 1,
        "a write from a newer epoch must succeed over an older-owned row"
    );
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(status, "tx_in_flight");
    assert_eq!(
        owner_epoch,
        Some(setup.epoch_b),
        "the write must advance owner_epoch to the newer epoch"
    );
}

/// The `current_epoch() == None` fence path: a pod that has never acquired the lock (the
/// pre-first-acquisition window every replica passes through at startup, and the entire
/// failure mode `FIRST_EPOCH_WAIT_BUDGET` in `startup.rs` bounds) can still write a `NULL`-
/// owned row - the "no behaviour change at one replica" invariant depends on this - but is
/// refused against a row a real epoch already owns.
#[tokio::test]
async fn test_none_epoch_write_succeeds_against_null_owner_and_is_refused_against_a_real_one() {
    let setup = TwoEpochSetup::new().await;

    // Connected, never run: `current_epoch()` stays `None` for its whole lifetime, matching a
    // pod still waiting on its first acquisition. The key doesn't matter - never acquired.
    let unheld = DispatcherLock::connect(&setup.lock_config, &setup.database_url)
        .await
        .expect("connect unheld lock");
    let repos_unheld = Repositories::new(setup.storage.clone(), unheld)
        .await
        .expect("repos for unheld lock");

    let null_owned = setup.insert_public_decrypt_row("processing", None).await;
    let rows = repos_unheld
        .public_decrypt
        .update_status_to_tx_in_flight(&null_owned)
        .await
        .expect("query null-owned");
    assert_eq!(
        rows, 1,
        "a None-epoch write must succeed against a NULL-owned row"
    );
    let (status, owner_epoch) = setup.status_and_owner_epoch(&null_owned).await;
    assert_eq!(status, "tx_in_flight");
    assert_eq!(
        owner_epoch, None,
        "a None-epoch write stamps NULL, not a phantom epoch"
    );

    let real_owned = setup
        .insert_public_decrypt_row("processing", Some(setup.epoch_a))
        .await;
    let rows = repos_unheld
        .public_decrypt
        .update_status_to_tx_in_flight(&real_owned)
        .await
        .expect("query real-owned");
    assert_eq!(
        rows, 0,
        "a None-epoch write must be refused against a row a real epoch already owns"
    );
    let (status, owner_epoch) = setup.status_and_owner_epoch(&real_owned).await;
    assert_eq!(status, "processing", "row must be untouched");
    assert_eq!(owner_epoch, Some(setup.epoch_a));
}

/// `owner_epoch IS NULL` means "unclaimed" - every pre-migration row and every row an image
/// predating this column inserts - and must be claimable by any epoch, not rejected as owned
/// by a phantom epoch.
#[tokio::test]
async fn test_null_owner_epoch_row_is_claimable_by_any_epoch() {
    let setup = TwoEpochSetup::new().await;
    let int_job_id = setup.insert_public_decrypt_row("processing", None).await;

    let rows = setup
        .repos_a
        .public_decrypt
        .update_status_to_tx_in_flight(&int_job_id)
        .await
        .expect("query");
    assert_eq!(rows, 1, "a NULL-owner row must be claimable by any epoch");
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(status, "tx_in_flight");
    assert_eq!(
        owner_epoch,
        Some(setup.epoch_a),
        "the write must stamp the claiming epoch"
    );
}

/// The interleaving the build-order brief describes: a Postgres failover kills the old
/// holder's lock session while its app-pool connections survive; before its heartbeat
/// notices, the new holder resets and re-drives the row; the old pod's send fails fast and
/// tries to write `failure`. Without the fence this succeeds (status was still
/// `tx_in_flight`), and the new holder's later `receipt_received` write then hits zero rows
/// because the status no longer matches - `gw_reference_id` is never stored, the response
/// event is dropped, and the client polls `failure` forever for a decryption that succeeded
/// on chain. With the fence, the old pod's write is refused outright.
#[tokio::test]
async fn test_stale_owner_failure_write_cannot_shadow_the_new_owners_receipt() {
    let setup = TwoEpochSetup::new().await;
    // The new holder already claimed and is driving this row.
    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", Some(setup.epoch_b))
        .await;

    // The old pod has not yet noticed it lost the lock - its DispatcherLock still reports
    // epoch_a - and its own in-flight send just failed.
    let rows = setup
        .repos_a
        .public_decrypt
        .update_status_to_failure_on_tx_failed(&int_job_id, "send failed")
        .await
        .expect("query a");
    assert_eq!(rows, 0, "the stale pod's failure write must be refused");
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(
        status, "tx_in_flight",
        "status must still be tx_in_flight, not failure"
    );
    assert_eq!(owner_epoch, Some(setup.epoch_b));

    // The new holder's own send succeeds and records the receipt.
    let gw_reference_id = U256::from(42u64);
    let rows = setup
        .repos_b
        .public_decrypt
        .update_status_to_receipt_received_on_tx_success(&int_job_id, "0xabc", gw_reference_id)
        .await
        .expect("query b");
    assert_eq!(rows, 1, "the current owner's receipt write must succeed");
    let (status, owner_epoch) = setup.status_and_owner_epoch(&int_job_id).await;
    assert_eq!(status, "receipt_received");
    assert_eq!(owner_epoch, Some(setup.epoch_b));
    assert!(
        setup.gw_reference_id(&int_job_id).await.is_some(),
        "gw_reference_id must be stored - the gateway-event listener keys off it to answer \
         the client"
    );
}

/// `ChainCursorRepository::advance` is monotonic but that alone does not stop a stalled
/// ex-holder from pushing the cursor forward past events its successor has not finished
/// handling - the fence refuses the write outright, even though the block number offered is
/// higher (monotonicity alone would have allowed it).
#[tokio::test]
async fn test_chain_cursor_advance_is_fenced_by_owner_epoch() {
    let setup = TwoEpochSetup::new().await;

    let moved = setup
        .repos_b
        .chain_cursor
        .advance(100)
        .await
        .expect("advance b");
    assert!(moved, "first advance must succeed and claim the cursor");

    let moved = setup
        .repos_a
        .chain_cursor
        .advance(200)
        .await
        .expect("advance a");
    assert!(
        !moved,
        "a stale-epoch advance must be refused even though the block number is higher"
    );
    assert_eq!(
        setup.repos_b.chain_cursor.get().await.expect("get"),
        Some(100)
    );

    let moved = setup
        .repos_b
        .chain_cursor
        .advance(200)
        .await
        .expect("advance b again");
    assert!(moved, "the current owner can still advance the cursor");
    assert_eq!(
        setup.repos_b.chain_cursor.get().await.expect("get"),
        Some(200)
    );
}

/// The other direction: a *newer* epoch must be able to advance a cursor an *older* epoch last
/// claimed - the same fencing-token property (`<=`, not `=`) as the request-row writes. Without
/// it the cursor would be bricked by its very first handover: no other mechanism ever re-stamps
/// `gateway_chain_cursor.owner_epoch`, unlike a request row, which the sweep's claim can.
#[tokio::test]
async fn test_chain_cursor_advance_succeeds_from_a_newer_epoch_over_an_older_owner() {
    let setup = TwoEpochSetup::new().await;

    let moved = setup
        .repos_a
        .chain_cursor
        .advance(100)
        .await
        .expect("advance a");
    assert!(moved, "first advance must succeed and claim the cursor");
    assert_eq!(
        setup.repos_a.chain_cursor.get().await.expect("get"),
        Some(100)
    );

    let moved = setup
        .repos_b
        .chain_cursor
        .advance(200)
        .await
        .expect("advance b");
    assert!(
        moved,
        "a newer epoch must be able to advance a cursor an older epoch claimed"
    );
    assert_eq!(
        setup.repos_b.chain_cursor.get().await.expect("get"),
        Some(200)
    );
}
