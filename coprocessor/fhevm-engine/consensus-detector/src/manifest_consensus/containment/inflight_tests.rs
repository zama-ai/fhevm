//! Simulate the worker transaction contract against real PostgreSQL locks.
//! Result acceptance is test-only until TFHE worker integration is implemented.

use super::*;

#[derive(Clone, Copy, Debug)]
enum Outcome {
    Success,
    Error,
}

#[derive(Clone, Copy, Debug)]
enum Acceptance {
    Commit,
    Discard,
}

async fn input_is_forbidden(trx: &mut Transaction<'_, Postgres>) -> bool {
    sqlx::query_scalar(
        "SELECT EXISTS (SELECT 1 FROM public.drifted_handle
          WHERE consensus_epoch = (SELECT consensus_epoch FROM blue_green_consensus_epoch WHERE singleton)
            AND coprocessor_context_id = $1 AND host_chain_id = 1
            AND handle = $1 AND block_hash = $1
            AND reason = 'ct64_mismatch' AND healed_at IS NULL)",
    )
    .bind(bytes(1))
    .fetch_one(trx.as_mut())
    .await
    .unwrap()
}

async fn stage_result(trx: &mut Transaction<'_, Postgres>, outcome: Outcome) {
    match outcome {
        Outcome::Success => {
            sqlx::query("UPDATE computations SET is_completed = TRUE, completed_at = NOW() WHERE output_handle = $1")
                .bind(bytes(2)).execute(trx.as_mut()).await.unwrap();
            sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0)")
                .bind(bytes(2)).bind(bytes(20)).execute(trx.as_mut()).await.unwrap();
        }
        Outcome::Error => {
            sqlx::query("UPDATE computations SET is_error = TRUE, error_message = 'simulated FHE failure' WHERE output_handle = $1")
                .bind(bytes(2)).execute(trx.as_mut()).await.unwrap();
        }
    }
}

async fn wait_for_guaranteed_pass(pool: &PgPool) {
    tokio::time::timeout(Duration::from_secs(10), async {
        loop {
            let waiting: bool = sqlx::query_scalar(
                "SELECT EXISTS (SELECT 1 FROM pg_locks
                  WHERE locktype = 'advisory' AND mode = 'ExclusiveLock'
                    AND NOT granted AND objsubid = 1
                    AND classid = ($1::bigint >> 32)::oid
                    AND objid = ($1::bigint & 4294967295)::oid)",
            )
            .bind(DRIFT_CONTAINMENT_BARRIER)
            .fetch_one(pool)
            .await
            .unwrap();
            if waiting {
                return;
            }
            tokio::time::sleep(Duration::from_millis(10)).await;
        }
    })
    .await
    .expect("guaranteed pass must reach the worker barrier");
}

async fn run_inflight_case(outcome: Outcome, acceptance: Acceptance) {
    let (_db, pool) = setup().await;
    producer(&pool, 1, 1).await;
    sqlx::query("INSERT INTO ciphertexts (handle, ciphertext, ciphertext_version, ciphertext_type) VALUES ($1, $2, 0, 0)")
        .bind(bytes(1)).bind(bytes(10)).execute(&pool).await.unwrap();
    computation(&pool, 2, 1, 2, false, true).await;
    producer(&pool, 2, 1).await;
    computation(&pool, 3, 2, 3, false, true).await;
    sqlx::query("UPDATE computations SET is_allowed = TRUE")
        .execute(&pool)
        .await
        .unwrap();
    let pending_before: serde_json::Value =
        sqlx::query_scalar("SELECT to_jsonb(c) FROM computations c WHERE output_handle = $1")
            .bind(bytes(2))
            .fetch_one(&pool)
            .await
            .unwrap();

    let mut batch = pool.begin().await.unwrap();
    sqlx::query("SELECT pg_advisory_xact_lock_shared($1)")
        .bind(DRIFT_CONTAINMENT_BARRIER)
        .execute(batch.as_mut())
        .await
        .unwrap();
    sqlx::query("SELECT output_handle FROM computations WHERE output_handle = $1 FOR UPDATE")
        .bind(bytes(2))
        .fetch_one(batch.as_mut())
        .await
        .unwrap();
    assert!(
        !input_is_forbidden(&mut batch).await,
        "input is healthy when selected"
    );
    stage_result(&mut batch, outcome).await;

    // Detection races after the earlier acceptance read. The real result/error
    // writes remain uncommitted in the same transaction that holds the barrier.
    insert_root(&pool, 1).await;
    let p = pool.clone();
    let containment = tokio::spawn(async move { enforce_guaranteed_containment(&p).await });
    wait_for_guaranteed_pass(&pool).await;
    assert!(!containment.is_finished());

    // The Fast pass has completed but cannot see either staged outcome. This
    // deliberately defeats optimistic propagation using transaction isolation.
    let visible: serde_json::Value =
        sqlx::query_scalar("SELECT to_jsonb(c) FROM computations c WHERE output_handle = $1")
            .bind(bytes(2))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        visible, pending_before,
        "uncommitted {outcome:?} must stay invisible"
    );
    let visible_ciphertext: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM ciphertexts WHERE handle = $1)")
            .bind(bytes(2))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(!visible_ciphertext);
    assert_eq!(
        flags(&pool).await,
        vec![(bytes(1), false)],
        "Fast must not claim containment of an in-flight outcome"
    );

    match acceptance {
        Acceptance::Discard => {
            // Stand-in for the future result check: its fresh read sees drift
            // even though is_contained is still false. Revert staged writes.
            assert!(input_is_forbidden(&mut batch).await);
            batch.rollback().await.unwrap();
        }
        Acceptance::Commit => {
            // Simulate the escaping case: the earlier check was already done
            // before detection, so the worker publishes its result/error.
            batch.commit().await.unwrap();
        }
    }
    let result = tokio::time::timeout(Duration::from_secs(10), containment)
        .await
        .unwrap()
        .unwrap()
        .unwrap();
    let ciphertext: Option<Vec<u8>> =
        sqlx::query_scalar("SELECT ciphertext FROM ciphertexts WHERE handle = $1")
            .bind(bytes(2))
            .fetch_optional(&pool)
            .await
            .unwrap();
    match acceptance {
        Acceptance::Discard => {
            assert_eq!(
                result,
                PropagationResult {
                    inferred_handles: 0,
                    contained_findings: 1
                }
            );
            let reverted: serde_json::Value = sqlx::query_scalar(
                "SELECT to_jsonb(c) FROM computations c WHERE output_handle = $1",
            )
            .bind(bytes(2))
            .fetch_one(&pool)
            .await
            .unwrap();
            assert_eq!(
                reverted, pending_before,
                "discard must revert completion, error and timestamp writes"
            );
            assert_eq!(ciphertext, None, "discard must revert ciphertext insertion");
            assert_eq!(
                flags(&pool).await,
                vec![(bytes(1), true)],
                "discarded work must not become inferred drift"
            );
        }
        Acceptance::Commit => {
            assert_eq!(
                result,
                PropagationResult {
                    inferred_handles: 1,
                    contained_findings: 2
                }
            );
            let finding: (String, String, bool, bool) = sqlx::query_as(
                "SELECT detection_kind, reason, local_present, is_contained FROM drifted_handle WHERE handle = $1"
            ).bind(bytes(2)).fetch_one(&pool).await.unwrap();
            assert_eq!(
                finding,
                (
                    "inferred".into(),
                    "ct64_mismatch".into(),
                    matches!(outcome, Outcome::Success),
                    true
                )
            );
            let state: (bool, bool, Option<String>) = sqlx::query_as(
                "SELECT is_completed, is_error, error_message FROM computations WHERE output_handle = $1"
            ).bind(bytes(2)).fetch_one(&pool).await.unwrap();
            match outcome {
                Outcome::Success => {
                    assert_eq!(state, (true, false, None));
                    assert_eq!(ciphertext, Some(bytes(20)));
                }
                Outcome::Error => {
                    assert_eq!(state, (false, true, Some("simulated FHE failure".into())));
                    assert_eq!(ciphertext, None);
                }
            }
        }
    }
    let dependent: (bool, bool) =
        sqlx::query_as("SELECT is_completed, is_error FROM computations WHERE output_handle = $1")
            .bind(bytes(3))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(
        dependent,
        (false, false),
        "pending dependent remains pending"
    );
    let inferred_dependent: bool =
        sqlx::query_scalar("SELECT EXISTS (SELECT 1 FROM drifted_handle WHERE handle = $1)")
            .bind(bytes(3))
            .fetch_one(&pool)
            .await
            .unwrap();
    assert!(!inferred_dependent);
}

#[tokio::test]
#[serial(db)]
async fn successful_inflight_result_is_reverted_when_the_check_sees_drift() {
    run_inflight_case(Outcome::Success, Acceptance::Discard).await;
}

#[tokio::test]
#[serial(db)]
async fn failed_inflight_result_is_reverted_when_the_check_sees_drift() {
    run_inflight_case(Outcome::Error, Acceptance::Discard).await;
}

#[tokio::test]
#[serial(db)]
async fn successful_inflight_result_escaping_the_check_is_inferred_under_the_barrier() {
    run_inflight_case(Outcome::Success, Acceptance::Commit).await;
}

#[tokio::test]
#[serial(db)]
async fn failed_inflight_result_escaping_the_check_is_inferred_under_the_barrier() {
    run_inflight_case(Outcome::Error, Acceptance::Commit).await;
}
