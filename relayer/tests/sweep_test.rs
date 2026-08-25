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

        let repositories = Repositories::new(settings.storage.clone())
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

/// Two concurrent claim attempts on the same stale row, with different epochs, must not both
/// win: the claim `UPDATE` is Postgres's row-lock-then-recheck-the-WHERE-clause CAS, so the
/// loser's predicate (`updated_at < NOW() - claim_after`) stops matching the instant the
/// winner's `UPDATE` commits and the `updated_at` trigger fires. Exactly one row comes back
/// across both calls, and `attempts` ends at 1, not 2 - the failure mode this guards against
/// is `update_status_to_tx_in_flight`'s bug, where two callers both believed they had won
/// because nobody checked how many rows a CAS actually touched.
#[tokio::test]
async fn test_claim_is_not_double_claimed_under_concurrent_claimers() {
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
        repo_b.claim_incomplete_requests(222, 5, claim_after_secs),
    );
    let claimed_a = result_a.expect("claim a failed");
    let claimed_b = result_b.expect("claim b failed");

    let total_claimed = claimed_a.len() + claimed_b.len();
    assert_eq!(
        total_claimed, 1,
        "exactly one of the two concurrent claimers must win the row"
    );

    let (attempts, owner_epoch) = setup.attempts_and_owner_epoch(&int_job_id).await;
    assert_eq!(
        attempts, 1,
        "attempts must increment exactly once, not twice"
    );
    assert!(
        owner_epoch == Some(111) || owner_epoch == Some(222),
        "owner_epoch must be stamped with whichever epoch actually won"
    );

    // The winner's returned epoch matches the row's owner_epoch.
    if let Some((_, _, _, attempts_returned)) = claimed_a.into_iter().chain(claimed_b).next() {
        assert_eq!(attempts_returned, 1);
    }
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
    // above) and at the bound.
    let failed = repo
        .fail_exhausted_attempts(max_attempts, claim_after_secs, "test: exhausted")
        .await
        .expect("fail_exhausted_attempts failed");
    assert_eq!(failed, 1, "exactly the exhausted row must be failed out");

    let (status, err_reason) = setup.status_and_err_reason(&int_job_id).await;
    assert_eq!(status, "failure");
    assert_eq!(err_reason.as_deref(), Some("test: exhausted"));
}
