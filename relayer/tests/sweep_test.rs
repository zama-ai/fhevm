//! Sweep repository tests: the claim query
//! (`PublicDecryptRepository::claim_incomplete_requests`, mirrored on the user-decrypt and
//! input-proof repositories).
//!
//! Exercised directly against the repository, against a schema-isolated database, rather than
//! through the full sweep worker or relayer harness - these are queries whose correctness is
//! about Postgres row-locking semantics, not about the orchestrator or HTTP layer above them.

mod common;

use common::test_schema::TestSchema;
use fhevm_relayer::config::settings::Settings;
use fhevm_relayer::orchestrator::{DispatcherLock, UNCLAIMED_EPOCH};
use fhevm_relayer::store::sql::repositories::Repositories;
use prometheus::Registry;
use sqlx::postgres::PgPoolOptions;
use sqlx::Row;
use uuid::Uuid;

const TEST_CONFIG_PATH: &str = "tests/relayer-test-config.yaml";

/// Above every row count these tests insert, so a claim's bound never truncates what an
/// assertion expects back. `test_a_backlog_larger_than_the_batch_drains_over_ticks` sets its
/// own bound instead.
const TEST_CLAIM_BATCH: i64 = 100;

/// Repositories pointed at a fresh schema-isolated database, plus a raw pool for test-only
/// setup (inserting rows, backdating `updated_at`) and assertions that go around the
/// repository's own API.
struct RepoTestSetup {
    repositories: Repositories,
    raw_pool: sqlx::PgPool,
    _schema: TestSchema,
}

impl RepoTestSetup {
    async fn new() -> Self {
        let schema = TestSchema::new()
            .await
            .expect("Failed to create test schema");

        let mut settings =
            Settings::new(Some(TEST_CONFIG_PATH.to_string())).expect("Failed to load config");
        // The repositories report query latency and status transitions through OnceLocks the
        // relayer's startup would normally fill; nextest gives each test its own process, so
        // this needs doing once per test (see `handled_events_test.rs::Harness::new` for the
        // same pattern with `init_db_metrics`).
        fhevm_relayer::metrics::init_db_metrics(&Registry::new(), settings.metrics.clone());
        fhevm_relayer::metrics::init_statuses_metrics(&Registry::new(), settings.metrics.clone());
        settings.storage.sql_database_url = schema.database_url();
        settings.storage.app_pool.max_connections = 4;
        settings.storage.app_pool.min_connections = 0;
        settings.storage.cron_pool.max_connections = 4;
        settings.storage.cron_pool.min_connections = 0;

        // Connected but never run: these tests exercise `claim_incomplete_requests` directly
        // with explicit epochs, not the self-served `current_epoch()` write-fencing paths, so
        // an unheld lock (epoch always `None`) is fine here.
        let dispatcher_lock =
            DispatcherLock::connect(&settings.dispatcher_lock, &schema.database_url())
                .await
                .expect("Failed to connect dispatcher lock");

        let repositories = Repositories::new(settings.storage.clone(), dispatcher_lock)
            .await
            .expect("Failed to create repositories");

        let raw_pool = PgPoolOptions::new()
            .max_connections(2)
            .connect(&schema.database_url())
            .await
            .expect("Failed to connect raw pool");

        Self {
            repositories,
            raw_pool,
            _schema: schema,
        }
    }

    /// Insert a minimal `queued` public_decrypt_req row, backdated by `age`, which is set for
    /// realism only - claim eligibility has no time term. The `req` payload is an empty JSON
    /// object - claim/fail-out never inspect it, only the caller re-dispatching a claimed row
    /// would.
    async fn insert_stale_public_decrypt(&self, age: std::time::Duration) -> Vec<u8> {
        let ext_job_id = Uuid::new_v4();
        let int_job_id = Uuid::new_v4().as_bytes().to_vec();
        let age_secs = age.as_secs_f64();

        sqlx::query(
            r#"
            INSERT INTO public_decrypt_req (ext_job_id, int_job_id, req, req_status, updated_at)
            VALUES ($1, $2, '{}'::jsonb, 'queued'::req_status, NOW() - make_interval(secs => $3))
            "#,
        )
        .bind(ext_job_id)
        .bind(&int_job_id)
        .bind(age_secs)
        .execute(&self.raw_pool)
        .await
        .expect("Failed to insert stale row");

        int_job_id
    }

    /// Insert a `public_decrypt_req` row with an explicit status, age, and `owner_epoch`. The
    /// age is set for realism only - claim eligibility has no time term, and the tests
    /// below assert exactly that where it matters.
    async fn insert_public_decrypt_row(
        &self,
        status: &str,
        age: std::time::Duration,
        owner_epoch: i64,
    ) -> Vec<u8> {
        let ext_job_id = Uuid::new_v4();
        let int_job_id = Uuid::new_v4().as_bytes().to_vec();
        let age_secs = age.as_secs_f64();

        sqlx::query(
            r#"
            INSERT INTO public_decrypt_req (ext_job_id, int_job_id, req, req_status, owner_epoch, updated_at)
            VALUES ($1, $2, '{}'::jsonb, $3::req_status, $4, NOW() - make_interval(secs => $5))
            "#,
        )
        .bind(ext_job_id)
        .bind(&int_job_id)
        .bind(status)
        .bind(owner_epoch)
        .bind(age_secs)
        .execute(&self.raw_pool)
        .await
        .expect("Failed to insert row");

        int_job_id
    }

    async fn status_and_owner_epoch_only(&self, int_job_id: &[u8]) -> (String, i64) {
        let row = sqlx::query(
            "SELECT req_status::text as status, owner_epoch FROM public_decrypt_req WHERE int_job_id = $1",
        )
        .bind(int_job_id)
        .fetch_one(&self.raw_pool)
        .await
        .expect("Failed to read row");
        (row.get("status"), row.get("owner_epoch"))
    }
}

/// Two concurrent claims of the same unowned row must not both win. The claim `UPDATE` is
/// Postgres's row-lock-then-recheck-the-WHERE-clause CAS, and the winner's `owner_epoch = $1`
/// stamp is what stops matching for the loser: the row is no longer `NULL`-owned, and its epoch
/// is not *below* the loser's own. Exactly one row comes back across both calls - the
/// failure mode this guards against is `update_status_to_tx_in_flight`'s bug, where two
/// callers both believed they had won because a CAS's row count went unchecked.
///
/// Same epoch, not different ones: only one pod's sweep loop ever ticks with a given epoch (it
/// is minted once per acquisition and the loop is sequential, so nothing within one pod races
/// itself), so this is the realistic concurrent-claimer shape. Two *different* epochs racing
/// the same row is also possible - during a real handover, not within one pod - and is
/// deliberately allowed to double-claim; see
/// `test_a_lower_epochs_claim_is_defeated_by_a_higher_epochs_concurrent_claim` for why that is
/// still safe.
#[tokio::test]
async fn test_claim_is_not_double_claimed_under_concurrent_claimers_of_the_same_epoch() {
    let setup = RepoTestSetup::new().await;
    let int_job_id = setup
        .insert_stale_public_decrypt(std::time::Duration::from_secs(60))
        .await;

    let repo_a = setup.repositories.public_decrypt.clone();
    let repo_b = setup.repositories.public_decrypt.clone();
    let (result_a, result_b) = tokio::join!(
        repo_a.claim_incomplete_requests(111, TEST_CLAIM_BATCH),
        repo_b.claim_incomplete_requests(111, TEST_CLAIM_BATCH),
    );
    let claimed_a = result_a.expect("claim a failed");
    let claimed_b = result_b.expect("claim b failed");

    let total_claimed = claimed_a.len() + claimed_b.len();
    assert_eq!(
        total_claimed, 1,
        "exactly one of the two concurrent same-epoch claimers must win the row"
    );

    let (_, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(owner_epoch, 111);
}

/// The other shape a concurrent claim race can take: two *different* epochs, e.g. a genuine
/// handover where the ex-holder's sweep tick is still in flight on a surviving cron-pool
/// connection when the new holder's own sweep claims the same row. Unlike the same-epoch case
/// above, this is NOT mutually exclusive at the claim level: once the lower epoch's `UPDATE`
/// has landed, the row it leaves behind (`owner_epoch = 111`) satisfies the higher epoch's
/// `owner_epoch < $epoch` immediately. Both claims can therefore succeed at the SQL level.
///
/// Deliberately tolerated rather than fixed here, because it is caught one layer up: the
/// loser's epoch is by then stale on the row, so its very next status write (self-served from
/// its own `current_epoch()`) is refused by the `owner_epoch <= $epoch` fence every write uses
/// - see `public_decrypt_repo`'s module doc. The cost is one wasted claim, never a wrong
/// final state.
#[tokio::test]
async fn test_a_lower_epochs_claim_is_defeated_by_a_higher_epochs_concurrent_claim() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let int_job_id = setup
        .insert_stale_public_decrypt(std::time::Duration::from_secs(60))
        .await;

    // Sequential, not concurrent: this isolates the "lower epoch already committed" half of
    // the race deterministically, which is the half that matters (the other order already
    // excludes the lower epoch outright, since its predicate no longer matches).
    let claimed_low = repo
        .claim_incomplete_requests(111, TEST_CLAIM_BATCH)
        .await
        .expect("low-epoch claim failed");
    assert_eq!(claimed_low.len(), 1, "the lower epoch claims first");

    let claimed_high = repo
        .claim_incomplete_requests(222, TEST_CLAIM_BATCH)
        .await
        .expect("high-epoch claim failed");
    assert_eq!(
        claimed_high.len(),
        1,
        "a higher epoch must still be able to claim a row a lower epoch just claimed"
    );

    let (_, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(
        owner_epoch, 222,
        "the higher epoch's claim is what the row ends up owned by"
    );
}

/// An unclaimed `tx_in_flight` row is claimed immediately, however fresh. `UNCLAIMED_EPOCH`
/// means intake declined to claim the row, which means the accepting pod also declined to drive
/// it - so there is nothing to wait for.
///
/// The row is also reset to `processing` in the same statement, so `on_tx_in_flight`'s CAS
/// (which requires `processing`) can claim it again. Without the reset, every re-dispatch of
/// the row is refused even though nothing was ever wrong with it.
#[tokio::test]
async fn test_unclaimed_tx_in_flight_row_is_claimed_immediately_and_reset() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    // Zero age: freshness is not a factor.
    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, UNCLAIMED_EPOCH)
        .await;

    let claimed = repo
        .claim_incomplete_requests(9, TEST_CLAIM_BATCH)
        .await
        .expect("claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "an unowned row has nobody driving it, so it is claimable at once"
    );
    assert_eq!(
        claimed[0].status,
        fhevm_relayer::store::sql::models::req_status_enum_model::ReqStatus::Processing,
        "claim must return the post-reset status"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "processing", "tx_in_flight must be reset");
    assert_eq!(owner_epoch, 9);
}

/// A `tx_in_flight` row owned by an *older* epoch is claimed immediately too (epochs are minted
/// only on acquisition and monotonic, so an older epoch is necessarily a dead predecessor's,
/// not a live peer's) and reset the same way as an unowned row.
#[tokio::test]
async fn test_older_epoch_owned_tx_in_flight_row_is_claimed_immediately_and_reset() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, 3)
        .await;

    let claimed = repo
        .claim_incomplete_requests(9, TEST_CLAIM_BATCH)
        .await
        .expect("claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "a row owned by an older epoch must be claimable with no staleness wait"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "processing", "tx_in_flight must be reset");
    assert_eq!(owner_epoch, 9);
}

/// A row owned by the claiming epoch is never claimed, no matter how long it has sat there.
/// This pod is driving it, and `updated_at` cannot tell "still working" from "silently died":
/// the legitimate dwell is unbounded on two paths (an RPC call with no client-side timeout, and
/// a saturated tx throttler), so no staleness window over that signal can be made safe: a row
/// whose in-process task dies without a terminal write waits for this pod's restart, which
/// mints a higher epoch and makes the row claimable as a predecessor's.
#[tokio::test]
async fn test_own_epoch_owned_row_is_never_claimed_however_old() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::from_secs(86_400), 9)
        .await;

    let claimed = repo
        .claim_incomplete_requests(9, TEST_CLAIM_BATCH)
        .await
        .expect("claim failed");
    assert!(
        claimed.is_empty(),
        "a row under the claiming epoch is this pod's own in-flight work, at any age"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "tx_in_flight", "the row must be untouched");
    assert_eq!(owner_epoch, 9);

    // A restart is what resolves it: the next incarnation's epoch is higher, so the row is a
    // predecessor's and claimable at once.
    let claimed = repo
        .claim_incomplete_requests(10, TEST_CLAIM_BATCH)
        .await
        .expect("successor claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "the same row is claimable by the next epoch, with no wait"
    );
}

/// A backlog past the claim's bound is drained over consecutive ticks rather than truncated:
/// each claim stamps the current epoch, which is what makes the next tick pick up where the
/// last stopped without an `ORDER BY` to order the set (see
/// `PublicDecryptRepository::claim_incomplete_requests`'s doc comment).
#[tokio::test]
async fn test_a_backlog_larger_than_the_batch_drains_over_ticks() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    const BACKLOG: usize = 7;
    const BATCH: i64 = 3;

    for _ in 0..BACKLOG {
        setup
            .insert_public_decrypt_row("queued", std::time::Duration::ZERO, UNCLAIMED_EPOCH)
            .await;
    }

    let mut claimed_total = 0usize;
    let mut ticks = 0usize;
    loop {
        let claimed = repo
            .claim_incomplete_requests(9, BATCH)
            .await
            .expect("claim failed");
        if claimed.is_empty() {
            break;
        }
        assert!(
            claimed.len() as i64 <= BATCH,
            "a tick must never claim more than its bound (got {})",
            claimed.len()
        );
        claimed_total += claimed.len();
        ticks += 1;
        assert!(ticks <= BACKLOG, "the drain must terminate");
    }

    assert_eq!(
        claimed_total, BACKLOG,
        "every backlogged row must be claimed exactly once across the ticks"
    );
    assert_eq!(
        ticks, 3,
        "7 rows at a bound of 3 must take ceil(7/3) ticks, so no tick claims a row twice"
    );
}
