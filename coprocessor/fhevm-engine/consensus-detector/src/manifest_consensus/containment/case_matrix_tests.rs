use super::*;

#[derive(Debug)]
struct Case {
    kind: &'static str,
    reason: &'static str,
    local_present: bool,
    observed_present: bool,
    target: bool,
    healed: bool,
    status: &'static str,
    contained: bool,
}

impl Case {
    fn propagates(&self) -> bool {
        self.reason == "ct64_mismatch" && !self.healed
    }
}

#[derive(Debug, PartialEq, sqlx::FromRow)]
struct Descendant {
    detection_kind: String,
    reason: String,
    local_present: bool,
    is_contained: bool,
    target_ct64_digest: Option<Vec<u8>>,
}

fn cases() -> Vec<Case> {
    let mut cases = Vec::new();
    for kind in ["verified", "inferred"] {
        for reason in [
            "ct64_mismatch",
            "ct128_mismatch",
            "missing_here",
            "unknown_on_peer",
            "error_here",
            "error_on_peer",
            "uncomputed_here",
            "uncomputed_on_peer",
            "metadata_mismatch",
        ] {
            if kind == "inferred" && reason != "ct64_mismatch" {
                continue;
            }
            for local_present in [false, true] {
                let local_computed =
                    !matches!(reason, "missing_here" | "error_here" | "uncomputed_here");
                if kind == "verified" && local_present != local_computed {
                    continue;
                }
                let observed_present = kind == "verified"
                    && !matches!(
                        reason,
                        "unknown_on_peer" | "error_on_peer" | "uncomputed_on_peer"
                    );
                for target in [false, true] {
                    // Peer-side absence/error has no ct64 target. Inferred rows
                    // may acquire one later through healing's metadata quorum.
                    if target && !observed_present && kind != "inferred" {
                        continue;
                    }
                    for healed in [false, true] {
                        if healed
                            && !(target
                                && matches!(
                                    reason,
                                    "ct64_mismatch"
                                        | "missing_here"
                                        | "error_here"
                                        | "uncomputed_here"
                                ))
                        {
                            continue;
                        }
                        for status in ["unresolved", "resolved"] {
                            for contained in [false, true] {
                                cases.push(Case {
                                    kind,
                                    reason,
                                    local_present,
                                    observed_present,
                                    target,
                                    healed,
                                    status,
                                    contained,
                                });
                            }
                        }
                    }
                }
            }
        }
    }
    cases
}

async fn seed_case(pool: &PgPool, handle: u8, case: &Case) -> i64 {
    let id = direct_root(pool, handle, case.reason).await;
    let task: i64 =
        sqlx::query_scalar("SELECT last_observed_task_id FROM drifted_handle WHERE id = $1")
            .bind(id)
            .fetch_one(pool)
            .await
            .unwrap();
    let peer_ct64 = if matches!(case.reason, "ct128_mismatch" | "metadata_mismatch") {
        bytes(1)
    } else {
        bytes(2)
    };
    let target = case.target.then(|| peer_ct64.clone());
    sqlx::query(
        "UPDATE drifted_handle SET
            detection_kind = $2, reason = $3, local_present = $4, observed_present = $5,
            target_ct64_digest = $6, healed_at = CASE WHEN $7 THEN NOW() ELSE NULL END,
            status = $8, is_contained = $9,
            last_observed_task_id = CASE WHEN $2 = 'verified' THEN $10 ELSE NULL END,
            resolved_task_id = CASE WHEN $8 = 'resolved' THEN $10 ELSE NULL END,
            observed_commitment_digest = CASE WHEN $2 = 'verified' THEN $13 ELSE NULL END,
            local_keyset_id = CASE WHEN $4 THEN $11 ELSE NULL END,
            local_ct64_digest = CASE WHEN $4 THEN $11 ELSE NULL END,
            local_ct128_digest = CASE WHEN $4 THEN $11 ELSE NULL END,
            local_ct128_format = CASE WHEN $4 THEN 0 ELSE NULL END,
            observed_keyset_id = CASE WHEN $5 THEN
                CASE WHEN $3 = 'metadata_mismatch' THEN $13 ELSE $11 END ELSE NULL END,
            observed_ct64_digest = CASE WHEN $5 THEN $12 ELSE NULL END,
            observed_ct128_digest = CASE WHEN $5 THEN $13 ELSE NULL END,
            observed_ct128_format = CASE WHEN $5 THEN 0 ELSE NULL END
         WHERE id = $1",
    )
    .bind(id)
    .bind(case.kind)
    .bind(case.reason)
    .bind(case.local_present)
    .bind(case.observed_present)
    .bind(target)
    .bind(case.healed)
    .bind(case.status)
    .bind(case.contained)
    .bind(task)
    .bind(bytes(1))
    .bind(peer_ct64)
    .bind(bytes(2))
    .execute(pool)
    .await
    .unwrap_or_else(|e| panic!("invalid case {case:?}: {e}"));
    id
}

async fn assert_matrix(pool: &PgPool, cases: &[Case], guaranteed: bool) {
    for (index, case) in cases.iter().enumerate() {
        let handle = u8::try_from(index + 1).unwrap();
        let contained: bool =
            sqlx::query_scalar("SELECT is_contained FROM drifted_handle WHERE handle = $1")
                .bind(bytes(handle))
                .fetch_one(pool)
                .await
                .unwrap();
        assert_eq!(
            contained,
            case.contained || (guaranteed && case.propagates()),
            "root {case:?}"
        );
        let child: Option<Descendant> = sqlx::query_as(
            "SELECT detection_kind, reason, local_present, is_contained, target_ct64_digest FROM drifted_handle WHERE handle = $1"
        ).bind(bytes(handle + 100)).fetch_optional(pool).await.unwrap();
        let expected = case.propagates().then(|| Descendant {
            detection_kind: "inferred".to_owned(),
            reason: "ct64_mismatch".to_owned(),
            local_present: true,
            is_contained: guaranteed,
            target_ct64_digest: None,
        });
        assert_eq!(child, expected, "descendant of {case:?}");
    }
}

#[tokio::test]
#[serial(db)]
async fn all_valid_finding_cases_preserve_evidence_and_only_propagate_unhealed_ct64() {
    let (_db, pool) = setup().await;
    let cases = cases();
    assert_eq!(cases.len(), 100); // 76 unhealed combinations, 24 healed controls.
    let mut root_ids = Vec::new();
    for (index, case) in cases.iter().enumerate() {
        let handle = u8::try_from(index + 1).unwrap();
        root_ids.push(seed_case(&pool, handle, case).await);
        // Every row gets a retained descendant, so excluded cases must actively
        // leave it untouched. This tests inventory semantics, not FHE execution.
        computation(&pool, handle + 100, handle, handle + 100, true, true).await;
    }
    let before: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(d) - 'is_contained' FROM drifted_handle d WHERE id = ANY($1) ORDER BY id",
    )
    .bind(&root_ids)
    .fetch_all(&pool)
    .await
    .unwrap();
    let mut trx = pool.begin().await.unwrap();
    let fast = propagate_for_test(&mut trx, false).await.unwrap();
    trx.commit().await.unwrap();
    assert_eq!(
        fast,
        PropagationResult {
            inferred_handles: 24,
            contained_findings: 0
        }
    );
    assert_matrix(&pool, &cases, false).await;
    let mut trx = pool.begin().await.unwrap();
    let guaranteed = propagate_for_test(&mut trx, true).await.unwrap();
    trx.commit().await.unwrap();
    assert_eq!(
        guaranteed,
        PropagationResult {
            inferred_handles: 0,
            contained_findings: 36
        }
    );
    assert_matrix(&pool, &cases, true).await;
    let after: Vec<serde_json::Value> = sqlx::query_scalar(
        "SELECT to_jsonb(d) - 'is_contained' FROM drifted_handle d WHERE id = ANY($1) ORDER BY id",
    )
    .bind(&root_ids)
    .fetch_all(&pool)
    .await
    .unwrap();
    assert_eq!(
        before, after,
        "propagation must not change evidence, targets or observation status"
    );
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
}

#[tokio::test]
#[serial(db)]
async fn failed_descendants_are_inferred_without_claiming_a_successful_peer_result() {
    let (_db, pool) = setup().await;
    root(&pool, 1).await;
    for (output, input) in [(2, 1), (4, 2)] {
        computation(&pool, output, input, output, false, true).await;
        producer(&pool, output, 1).await;
        sqlx::query("UPDATE computations SET is_allowed = TRUE, is_error = TRUE, error_message = 'execution failed' WHERE output_handle = $1")
            .bind(bytes(output)).execute(&pool).await.unwrap();
    }
    computation(&pool, 3, 2, 3, false, true).await; // Pending remains implicit.
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult {
            inferred_handles: 2,
            contained_findings: 3
        }
    );
    let failed: Vec<(Vec<u8>, String, String, bool, bool, bool)> = sqlx::query_as(
        "SELECT handle, detection_kind, reason, local_present, observed_present, is_contained FROM drifted_handle WHERE handle <> $1 ORDER BY handle"
    ).bind(bytes(1)).fetch_all(&pool).await.unwrap();
    assert_eq!(
        failed,
        vec![
            (
                bytes(2),
                "inferred".into(),
                "ct64_mismatch".into(),
                false,
                false,
                true
            ),
            (
                bytes(4),
                "inferred".into(),
                "ct64_mismatch".into(),
                false,
                false,
                true
            ),
        ]
    );
    let claimed_peer_evidence: i64 = sqlx::query_scalar(
        "SELECT COUNT(*) FROM drifted_handle WHERE detection_kind = 'inferred' AND (target_ct64_digest IS NOT NULL OR observed_commitment_digest IS NOT NULL OR last_observed_task_id IS NOT NULL)"
    ).fetch_one(&pool).await.unwrap();
    assert_eq!(claimed_peer_evidence, 0);
    assert_eq!(
        enforce_guaranteed_containment(&pool).await.unwrap(),
        PropagationResult::default()
    );
}
