//! Sweep repository tests: the step-6 claim/fail-out queries
//! (`PublicDecryptRepository::claim_incomplete_requests` /
//! `fail_exhausted_attempts`, mirrored on the user-decrypt and input-proof repositories).
//!
//! Exercised directly against the repository, against a schema-isolated database, rather than
//! through the full sweep worker or relayer harness - these are queries whose correctness is
//! about Postgres row-locking semantics, not about the orchestrator or HTTP layer above them.

mod common;

use common::test_schema::TestSchema;
use fhevm_relayer::config::settings::Settings;
use fhevm_relayer::orchestrator::DispatcherLock;
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

    /// Insert a minimal `queued` public_decrypt_req row, backdated by `age` so it clears any
    /// reasonable `claim_after`. The `req` payload is an empty JSON object - claim/fail-out
    /// never inspect it, only the caller re-dispatching a claimed row would.
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

    /// Push a row's `updated_at` back by `age`, simulating time passing since its last claim
    /// without a real sleep. `trigger_set_timestamp()` (the `BEFORE UPDATE` trigger on this
    /// table) unconditionally rewrites `updated_at` to `NOW()` on any change - including an
    /// explicit write to `updated_at` itself, since that also makes `OLD IS DISTINCT FROM
    /// NEW` - so this runs inside a transaction with the session's triggers disabled for the
    /// backdating write only.
    async fn backdate(&self, int_job_id: &[u8], age: std::time::Duration) {
        let age_secs = age.as_secs_f64();
        let mut tx = self.raw_pool.begin().await.expect("Failed to begin tx");
        sqlx::query("SET LOCAL session_replication_role = replica")
            .execute(&mut *tx)
            .await
            .expect("Failed to disable triggers");
        sqlx::query(
            r#"
            UPDATE public_decrypt_req
            SET updated_at = NOW() - make_interval(secs => $2)
            WHERE int_job_id = $1
            "#,
        )
        .bind(int_job_id)
        .bind(age_secs)
        .execute(&mut *tx)
        .await
        .expect("Failed to backdate row");
        tx.commit().await.expect("Failed to commit backdate");
    }

    /// Insert a `public_decrypt_req` row with an explicit status, age, and `owner_epoch` - for
    /// the two-tier claim tests, which need control over all three independently of the
    /// `queued`-only, `owner_epoch`-`NULL`-only helper above.
    async fn insert_public_decrypt_row(
        &self,
        status: &str,
        age: std::time::Duration,
        owner_epoch: Option<i64>,
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

    async fn status_and_owner_epoch_only(&self, int_job_id: &[u8]) -> (String, Option<i64>) {
        let row = sqlx::query(
            "SELECT req_status::text as status, owner_epoch FROM public_decrypt_req WHERE int_job_id = $1",
        )
        .bind(int_job_id)
        .fetch_one(&self.raw_pool)
        .await
        .expect("Failed to read row");
        (row.get("status"), row.get("owner_epoch"))
    }

    async fn attempts_and_owner_epoch(&self, int_job_id: &[u8]) -> (i32, Option<i64>) {
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

/// Two concurrent claim attempts on the same stale row, under the *same* epoch, must not both
/// win: the claim `UPDATE` is Postgres's row-lock-then-recheck-the-WHERE-clause CAS, so the
/// loser's predicate (`updated_at < NOW() - claim_after`) stops matching the instant the
/// winner's `UPDATE` commits and the `updated_at` trigger fires. Exactly one row comes back
/// across both calls, and `attempts` ends at 1, not 2 - the failure mode this guards against
/// is `update_status_to_tx_in_flight`'s bug, where two callers both believed they had won
/// because nobody checked how many rows a CAS actually touched.
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
    let claim_after = std::time::Duration::from_millis(50);
    let int_job_id = setup
        .insert_stale_public_decrypt(std::time::Duration::from_secs(60))
        .await;

    let claim_after_secs = claim_after.as_secs_f64();
    let repo_a = setup.repositories.public_decrypt.clone();
    let repo_b = setup.repositories.public_decrypt.clone();
    let (result_a, result_b) = tokio::join!(
        repo_a.claim_incomplete_requests(111, 5, claim_after_secs),
        repo_b.claim_incomplete_requests(111, 5, claim_after_secs),
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
    assert_eq!(owner_epoch, Some(111));

    // The winner's returned epoch matches the row's owner_epoch.
    if let Some((_, _, _, attempts_returned)) = claimed_a.into_iter().chain(claimed_b).next() {
        assert_eq!(attempts_returned, 1);
    }
}

/// The other shape a concurrent claim race can take: two *different* epochs, e.g. a genuine
/// handover where the ex-holder's sweep tick is still in flight on a surviving cron-pool
/// connection when the new holder's own sweep claims the same row. Unlike the same-epoch case
/// above, this is NOT mutually exclusive at the claim level. The claim's actual predicate is
/// three-way - `owner_epoch < $epoch` (immediate) `OR ((owner_epoch IS NULL OR owner_epoch =
/// $epoch) AND stale)` (see `claim_incomplete_requests`'s doc comment) - and once the lower
/// epoch's `UPDATE` has landed, the row it leaves behind (`owner_epoch = 111`) matches the
/// higher epoch's `owner_epoch < $epoch` branch immediately, no staleness needed, regardless of
/// which claim committed first. Both claims can therefore "succeed" at the SQL level. This is
/// deliberately tolerated, not fixed here, because it is caught one layer up: the loser's epoch
/// is by then stale on the row, so its very next status write (self-served from its own
/// `current_epoch()`) is refused by the same `owner_epoch <= $epoch` fence every other write
/// uses - see `public_decrypt_repo`'s module doc. The cost is one wasted claim and
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
    // excludes the lower epoch outright, since its predicate no longer matches). A small
    // claim_after so the initial NULL-owner claim (staleness-gated, not immediate - see
    // `claim_incomplete_requests`'s doc on why `NULL` differs from a provably older epoch)
    // clears it quickly; the row was backdated 60s by `insert_stale_public_decrypt`.
    let claimed_low = repo
        .claim_incomplete_requests(111, 5, 0.05)
        .await
        .expect("low-epoch claim failed");
    assert_eq!(claimed_low.len(), 1, "the lower epoch claims first");

    // The higher epoch's claim needs no staleness wait at all: 111 is now a strictly older,
    // necessarily dead epoch from its perspective.
    let claimed_high = repo
        .claim_incomplete_requests(222, 5, 300.0)
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
         change - it is never reset on a takeover (see claim_incomplete_requests's doc comment \
         for why a reset-on-takeover design was tried and reverted)"
    );
    assert_eq!(
        owner_epoch,
        Some(222),
        "the higher epoch's claim is what the row ends up owned by"
    );

    // What actually prevents this double-claim from causing harm is that the lower epoch's
    // own subsequent write is refused by the same `owner_epoch <= $epoch` fence -
    // `epoch_fencing_test.rs::test_stale_epoch_write_is_refused_current_epoch_write_succeeds`
    // covers that directly (it needs two real, held `DispatcherLock`s to self-serve distinct
    // epochs from a status-write method, which this file's claim-focused setup does not have).
}

/// A previously-confirmed blocker: a row already at `max_attempts` under an *older* epoch must
/// not become a zombie a successor's sweep can neither claim (`attempts < max_attempts` fails)
/// nor fail out. It should instead be failed cleanly by `fail_exhausted_attempts` under the new
/// epoch - which is correct, not a bug, because `attempts` is a budget on the row's total
/// life, not any one owner's turn at it (see `claim_incomplete_requests`'s doc comment).
#[tokio::test]
async fn test_exhausted_row_from_an_older_epoch_is_failed_not_stranded() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let max_attempts = 5;

    // owner_epoch=4, attempts already at the bound, stale (backdated well past claim_after).
    let int_job_id = setup
        .insert_public_decrypt_row("processing", std::time::Duration::from_secs(1000), Some(4))
        .await;
    sqlx::query("UPDATE public_decrypt_req SET attempts = $1 WHERE int_job_id = $2")
        .bind(max_attempts)
        .bind(&int_job_id)
        .execute(&setup.raw_pool)
        .await
        .expect("Failed to set attempts");
    // The trigger that maintains `updated_at` fires on the write above too (see `backdate`'s
    // doc comment), so re-backdate afterward rather than relying on the original INSERT's age.
    setup
        .backdate(&int_job_id, std::time::Duration::from_secs(1000))
        .await;

    // The newer epoch's claim must not touch it - attempts already at the bound.
    let claimed = repo
        .claim_incomplete_requests(9, max_attempts, 0.05)
        .await
        .expect("claim failed");
    assert!(
        claimed.is_empty(),
        "a row already at max_attempts must not be claimable, regardless of owner"
    );

    // fail_exhausted_attempts, fenced by `<=` (not narrowed to an exact match), is what
    // rescues it from becoming a zombie: epoch 9 never attempted this row, but it is still
    // the one that correctly gives up on it.
    let failed = repo
        .fail_exhausted_attempts(9, max_attempts, 0.05, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(
        failed, 1,
        "a newer epoch must be able to fail out a row an older epoch exhausted"
    );

    let (status, _) = setup.status_and_err_reason(&int_job_id).await;
    assert_eq!(status, "failure");
}

/// `max_attempts` bounds re-dispatch: a row claimed up to the bound stops being claimable,
/// and once stale again past the bound, `fail_exhausted_attempts` moves it to `failure`
/// rather than leaving it claimable (or claimed) forever.
#[tokio::test]
async fn test_attempts_bound_redispatch_then_fails_exhausted_row() {
    let setup = RepoTestSetup::new().await;
    let claim_after = std::time::Duration::from_millis(50);
    let claim_after_secs = claim_after.as_secs_f64();
    let max_attempts = 2;
    let repo = &setup.repositories.public_decrypt;

    let int_job_id = setup
        .insert_stale_public_decrypt(std::time::Duration::from_secs(60))
        .await;

    // First claim: attempts 0 -> 1.
    let claimed = repo
        .claim_incomplete_requests(1, max_attempts, claim_after_secs)
        .await
        .expect("first claim failed");
    assert_eq!(claimed.len(), 1, "first claim must succeed");

    // Second claim, after backdating past claim_after again: attempts 1 -> 2 (== max_attempts).
    setup
        .backdate(&int_job_id, std::time::Duration::from_secs(60))
        .await;
    let claimed = repo
        .claim_incomplete_requests(1, max_attempts, claim_after_secs)
        .await
        .expect("second claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "second claim must succeed (attempts == max_attempts after this)"
    );
    assert_eq!(claimed[0].3, max_attempts);

    // Third attempt, past the bound: attempts (2) is no longer < max_attempts (2), so the
    // claim query must not touch it.
    setup
        .backdate(&int_job_id, std::time::Duration::from_secs(60))
        .await;
    let claimed = repo
        .claim_incomplete_requests(1, max_attempts, claim_after_secs)
        .await
        .expect("third claim query failed");
    assert!(
        claimed.is_empty(),
        "a row at max_attempts must not be claimed again"
    );

    // fail_exhausted_attempts now moves it to failure - the row is stale again (backdated
    // above) and at the bound. Same epoch (1) as the claims: the row is mine, so the fence
    // (`owner_epoch IS NULL OR owner_epoch <= $epoch`) matches.
    let failed = repo
        .fail_exhausted_attempts(1, max_attempts, claim_after_secs, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(failed, 1, "exactly the exhausted row must be failed out");

    let (status, err_reason) = setup.status_and_err_reason(&int_job_id).await;
    assert_eq!(status, "failure");
    assert_eq!(err_reason.as_deref(), Some("test: exhausted"));
}

/// Two-tier claim, not-provably-dead branch: a `tx_in_flight` row with no owner (`NULL`)
/// requires `claim_after` staleness before it is claimed - unlike a row owned by a strictly
/// older epoch, `NULL` is not immediately claimable, because until dispatch is gated on the
/// lock (step 7) a `NULL` owner can be a *live* non-holder pod driving its own accepted
/// traffic in-process (see `claim_incomplete_requests`'s doc comment). Once claimed, it is
/// reset to `processing` in the same statement, so `on_tx_in_flight`'s CAS (which requires
/// `processing`) can claim it again. Without the reset, the row would be claimed to exhaustion
/// and failed out even though nothing was ever wrong with it.
#[tokio::test]
async fn test_null_owner_tx_in_flight_row_requires_staleness_then_is_reset_to_processing() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let claim_after = std::time::Duration::from_millis(50);

    // Fresh - must not be claimable yet even though nothing owns it.
    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, None)
        .await;
    let claimed = repo
        .claim_incomplete_requests(9, 5, claim_after.as_secs_f64())
        .await
        .expect("claim failed");
    assert!(
        claimed.is_empty(),
        "a fresh NULL-owner row must not be claimed before it goes stale - it may be a live \
         non-holder's own traffic"
    );

    // Backdate past claim_after: now claimable, and reset.
    setup
        .backdate(&int_job_id, std::time::Duration::from_secs(60))
        .await;
    let claimed = repo
        .claim_incomplete_requests(9, 5, claim_after.as_secs_f64())
        .await
        .expect("claim failed");
    assert_eq!(claimed.len(), 1, "a stale NULL-owner row is claimable");
    assert_eq!(
        claimed[0].2,
        fhevm_relayer::store::sql::models::req_status_enum_model::ReqStatus::Processing,
        "claim must return the post-reset status"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "processing", "tx_in_flight must be reset");
    assert_eq!(owner_epoch, Some(9));
}

/// Two-tier claim, not-mine branch: a `tx_in_flight` row owned by an *older* epoch is claimed
/// immediately (epochs are minted only on acquisition and monotonic, so an older epoch is
/// necessarily a dead predecessor's, not a live peer's) and reset the same way as a `NULL`-owner
/// row.
#[tokio::test]
async fn test_older_epoch_owned_tx_in_flight_row_is_claimed_immediately_and_reset() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;

    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, Some(3))
        .await;

    let claimed = repo
        .claim_incomplete_requests(9, 5, 300.0)
        .await
        .expect("claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "a row owned by an older epoch must be claimable with no staleness wait"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "processing", "tx_in_flight must be reset");
    assert_eq!(owner_epoch, Some(9));
}

/// Two-tier claim, mine branch: a `tx_in_flight` row already owned by the claiming epoch is
/// not claimable until `claim_after` has actually elapsed - the same staleness requirement as
/// the `NULL`-owner case. Once claimed, it *is* reset to `processing` (this changed from an
/// earlier design that never reset a self-owned row): the only way to reach this branch at all
/// is `claim_after` of silence, comfortably above every legitimate single-attempt latency in
/// the pipeline, so a row still here cannot be a send that is merely running long - see
/// `claim_incomplete_requests`'s doc comment for the full reasoning, including why the
/// occasional false positive is still safe (a tolerated duplicate send, never a wrong result).
#[tokio::test]
async fn test_own_epoch_owned_tx_in_flight_row_requires_staleness_then_is_reset_too() {
    let setup = RepoTestSetup::new().await;
    let repo = &setup.repositories.public_decrypt;
    let claim_after = std::time::Duration::from_millis(50);

    let int_job_id = setup
        .insert_public_decrypt_row("tx_in_flight", std::time::Duration::ZERO, Some(9))
        .await;

    // Fresh - must not be claimable yet even though it is "mine".
    let claimed = repo
        .claim_incomplete_requests(9, 5, claim_after.as_secs_f64())
        .await
        .expect("claim failed");
    assert!(
        claimed.is_empty(),
        "a fresh row owned by my own epoch must not be reclaimed before it goes stale"
    );

    // Backdate past claim_after: now claimable, and reset just like the NULL-owner case.
    setup
        .backdate(&int_job_id, std::time::Duration::from_secs(60))
        .await;
    let claimed = repo
        .claim_incomplete_requests(9, 5, claim_after.as_secs_f64())
        .await
        .expect("claim failed");
    assert_eq!(
        claimed.len(),
        1,
        "a stale row owned by my own epoch is reclaimable"
    );
    assert_eq!(
        claimed[0].2,
        fhevm_relayer::store::sql::models::req_status_enum_model::ReqStatus::Processing,
        "claim must return the post-reset status"
    );

    let (status, owner_epoch) = setup.status_and_owner_epoch_only(&int_job_id).await;
    assert_eq!(status, "processing", "tx_in_flight must be reset");
    assert_eq!(owner_epoch, Some(9));
}
