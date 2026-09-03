//! Sweep repository tests: the claim/fail-out queries
//! (`PublicDecryptRepository::claim_incomplete_requests` /
//! `fail_exhausted_attempts`, mirrored on the user-decrypt and input-proof repositories).
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

        // Connected but never run: these tests exercise `claim_incomplete_requests` and
        // `fail_exhausted_attempts` directly with explicit epochs, not the self-served
        // `current_epoch()` write-fencing paths, so an unheld lock (epoch always `None`) is
        // fine here.
        let dispatcher_lock = DispatcherLock::connect(
            &fhevm_relayer::config::settings::DispatcherLockConfig::default(),
            &schema.database_url(),
        )
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

    async fn attempts_and_owner_epoch(&self, int_job_id: &[u8]) -> (i32, i64) {
        let row = sqlx::query(
            "SELECT attempts, owner_epoch FROM public_decrypt_req WHERE int_job_id = $1",
        )
        .bind(int_job_id)
        .fetch_one(&self.raw_pool)
        .await
        .expect("Failed to read row");
        (row.get("attempts"), row.get("owner_epoch"))
    }

    async fn status_and_err_reason(&self, int_job_id: &[u8]) -> (String, Option<String>) {
        let row = sqlx::query(
            "SELECT req_status::text as status, err_reason FROM public_decrypt_req WHERE int_job_id = $1",
        )
        .bind(int_job_id)
        .fetch_one(&self.raw_pool)
        .await
        .expect("Failed to read row");
        (row.get("status"), row.get("err_reason"))
    }
}

/// Two concurrent claims of the same unowned row must not both win. The claim `UPDATE` is
/// Postgres's row-lock-then-recheck-the-WHERE-clause CAS, and the winner's `owner_epoch = $1`
/// stamp is what stops matching for the loser: the row is no longer `NULL`-owned, and its epoch
/// is not *below* the loser's own. Exactly one row comes back across both calls, and `attempts`
/// ends at 1, not 2 - the failure mode this guards against is
/// `update_status_to_tx_in_flight`'s bug, where two callers both believed they had won because
/// nobody checked how many rows a CAS actually touched.
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
        repo_a.claim_incomplete_requests(111, 5),
        repo_b.claim_incomplete_requests(111, 5),
    );
    let claimed_a = result_a.expect("claim a failed");
    let claimed_b = result_b.expect("claim b failed");

    let total_claimed = claimed_a.len() + claimed_b.len();
    assert_eq!(
        total_claimed, 1,
        "exactly one of the two concurrent same-epoch claimers must win the row"
    );

    let (attempts, owner_epoch) = setup.attempts_and_owner_epoch(&int_job_id).await;
    assert_eq!(
        attempts, 1,
        "attempts must increment exactly once, not twice"
    );
    assert_eq!(owner_epoch, 111);

    // The winner's returned epoch matches the row's owner_epoch.
    if let Some((_, _, _, attempts_returned)) = claimed_a.into_iter().chain(claimed_b).next() {
        assert_eq!(attempts_returned, 1);
    }
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
/// - see `public_decrypt_repo`'s module doc. The cost is one wasted claim and
/// attempts-increment, never a wrong final state.
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
        .claim_incomplete_requests(111, 5)
        .await
        .expect("low-epoch claim failed");
    assert_eq!(claimed_low.len(), 1, "the lower epoch claims first");

    let claimed_high = repo
        .claim_incomplete_requests(222, 5)
        .await
        .expect("high-epoch claim failed");
    assert_eq!(
        claimed_high.len(),
        1,
        "a higher epoch must still be able to claim a row a lower epoch just claimed"
    );

    let (attempts, owner_epoch) = setup.attempts_and_owner_epoch(&int_job_id).await;
    assert_eq!(
        attempts, 2,
        "attempts is a global budget, incremented by both claims regardless of ownership \
         change - it is never reset on a takeover (see claim_incomplete_requests's doc comment)"
    );
    assert_eq!(
        owner_epoch, 222,
        "the higher epoch's claim is what the row ends up owned by"
    );
}

/// A row already at `max_attempts` under an *older* epoch must not become a zombie a
/// successor's sweep can neither claim (`attempts < max_attempts` fails)
/// nor fail out. It should instead be failed cleanly by `fail_exhausted_attempts` under the new
/// epoch - which is correct, not a bug, because `attempts` is a budget on the row's total
/// life, not any one owner's turn at it (see `claim_incomplete_requests`'s doc comment).
#[tokio::test]
async fn test_exhausted_row_from_an_older_epoch_is_failed_not_stranded() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let max_attempts = 5;

    // owner_epoch=4, attempts already at the bound.
    let int_job_id = setup
        .insert_public_decrypt_row("processing", std::time::Duration::from_secs(1000), 4)
        .await;
    sqlx::query("UPDATE public_decrypt_req SET attempts = $1 WHERE int_job_id = $2")
        .bind(max_attempts)
        .bind(&int_job_id)
        .execute(&setup.raw_pool)
        .await
        .expect("Failed to set attempts");

    // The newer epoch's claim must not touch it - attempts already at the bound.
    let claimed = repo
        .claim_incomplete_requests(9, max_attempts)
        .await
        .expect("claim failed");
    assert!(
        claimed.is_empty(),
        "a row already at max_attempts must not be claimable, regardless of owner"
    );

    // `fail_exhausted_attempts` is what rescues it from becoming a zombie: epoch 9 never
    // attempted this row, but it is still the one that correctly gives up on it.
    let failed = repo
        .fail_exhausted_attempts(9, max_attempts, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(
        failed, 1,
        "a newer epoch must be able to fail out a row an older epoch exhausted"
    );

    let (status, _) = setup.status_and_err_reason(&int_job_id).await;
    assert_eq!(status, "failure");
}

/// The other side of that predicate, and the reason `fail_exhausted_attempts` compares `<`
/// rather than the `<=` every fenced *write* uses: an exhausted row owned by the **current**
/// epoch must be left alone. This pod is driving that row, and a row reaches `attempts ==
/// max_attempts` the moment its own last claim incremented the counter to the bound - so `<=`
/// here would fail a live request out from under an in-flight send on the very next tick.
#[tokio::test]
async fn test_exhausted_row_owned_by_the_current_epoch_is_not_failed() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let max_attempts = 5;

    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::from_secs(1000), 9)
        .await;
    sqlx::query("UPDATE public_decrypt_req SET attempts = $1 WHERE int_job_id = $2")
        .bind(max_attempts)
        .bind(&int_job_id)
        .execute(&setup.raw_pool)
        .await
        .expect("Failed to set attempts");

    let failed = repo
        .fail_exhausted_attempts(9, max_attempts, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(
        failed, 0,
        "a row under the current epoch is being driven by this pod, however old it looks"
    );

    let (status, _) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "tx_in_flight", "the row must be untouched");
}

/// `max_attempts` bounds re-dispatch across owners: each successive epoch claims the row once,
/// and once the budget is spent no epoch can claim it again - at which point
/// `fail_exhausted_attempts` gives up on it rather than leaving it claimable forever.
///
/// One claim per epoch is the shape the claim now enforces: a row under the claiming epoch is
/// not re-claimable at all, so spending a budget of 2 takes two distinct epochs, exactly as a
/// row surviving two handovers would.
#[tokio::test]
async fn test_attempts_bound_redispatch_then_fails_exhausted_row() {
    let setup = RepoTestSetup::new().await;
    let max_attempts = 2;
    let repo = &setup.repositories.public_decrypt;

    let int_job_id = setup
        .insert_stale_public_decrypt(std::time::Duration::from_secs(60))
        .await;

    // First claim, by epoch 1 (row is unowned): attempts 0 -> 1.
    let claimed = repo
        .claim_incomplete_requests(1, max_attempts)
        .await
        .expect("first claim failed");
    assert_eq!(claimed.len(), 1, "first claim must succeed");

    // Same epoch again: the row is now this epoch's own, so it is not claimable.
    let claimed = repo
        .claim_incomplete_requests(1, max_attempts)
        .await
        .expect("same-epoch claim query failed");
    assert!(
        claimed.is_empty(),
        "a row under the claiming epoch is never re-claimed - this pod is driving it"
    );

    // A successor epoch takes over: attempts 1 -> 2 (== max_attempts).
    let claimed = repo
        .claim_incomplete_requests(2, max_attempts)
        .await
        .expect("successor claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "a successor epoch claims the row (attempts == max_attempts after this)"
    );
    assert_eq!(claimed[0].3, max_attempts);

    // A third epoch, past the bound: attempts (2) is no longer < max_attempts (2).
    let claimed = repo
        .claim_incomplete_requests(3, max_attempts)
        .await
        .expect("third claim query failed");
    assert!(
        claimed.is_empty(),
        "a row at max_attempts must not be claimed again"
    );

    // `fail_exhausted_attempts` under that third epoch moves it to failure: the row's own
    // epoch (2) is strictly below, so nothing is driving it.
    let failed = repo
        .fail_exhausted_attempts(3, max_attempts, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(failed, 1, "exactly the exhausted row must be failed out");

    let (status, err_reason) = setup.status_and_err_reason(&int_job_id).await;
    assert_eq!(status, "failure");
    assert_eq!(err_reason.as_deref(), Some("test: exhausted"));
}

/// An unclaimed `tx_in_flight` row is claimed immediately, however fresh. `UNCLAIMED_EPOCH`
/// means intake declined to claim the row, which means the accepting pod also declined to drive
/// it - so there is nothing to wait for.
///
/// The row is also reset to `processing` in the same statement, so `on_tx_in_flight`'s CAS
/// (which requires `processing`) can claim it again. Without the reset, the row would be claimed
/// to exhaustion and failed out even though nothing was ever wrong with it.
#[tokio::test]
async fn test_unclaimed_tx_in_flight_row_is_claimed_immediately_and_reset() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    // Zero age: freshness is not a factor.
    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, UNCLAIMED_EPOCH)
        .await;

    let claimed = repo
        .claim_incomplete_requests(9, 5)
        .await
        .expect("claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "an unowned row has nobody driving it, so it is claimable at once"
    );
    assert_eq!(
        claimed[0].2,
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
        .claim_incomplete_requests(9, 5)
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
        .claim_incomplete_requests(9, 5)
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
        .claim_incomplete_requests(10, 5)
        .await
        .expect("successor claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "the same row is claimable by the next epoch, with no wait"
    );
}
