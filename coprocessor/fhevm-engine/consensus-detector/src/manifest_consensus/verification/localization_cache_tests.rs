use super::*;
use crate::manifest_consensus::verification::localization_cache::{
    load_completed_history, localization_is_cacheable,
};

pub(super) async fn seed_history_task(
    pool: &PgPool,
    source: &FakePeerSource,
    signers: &[PrivateKeySigner],
    fork: u8,
) {
    for (index, signer) in signers.iter().enumerate() {
        let previous = sign_payload(
            signer,
            payload_at(
                signer.address(),
                if index == 0 { 1 } else { 2 },
                42,
                B256::repeat_byte(fork),
            ),
        )
        .await;
        let current = sign_payload(
            signer,
            payload_with_history(
                signer.address(),
                1,
                43,
                B256::repeat_byte(fork + 1),
                &previous,
            ),
        )
        .await;
        if index == 0 {
            archive_local_only(pool, &previous).await;
            schedule_local(pool, &current, 2).await;
        } else {
            source.set_manifests(signer.address(), &[previous, current]);
        }
    }
}

async fn run_once(pool: &PgPool, source: &FakePeerSource) {
    let result = run_peer_manifest_download_once(
        pool,
        source,
        "cache-test",
        Duration::from_secs(60),
        GENERATION,
    )
    .await
    .expect("verification succeeds")
    .expect("one due task");
    assert_eq!(result.outcome, VerificationOutcome::Drift);
}

#[tokio::test]
async fn complete_localization_without_quorum_is_not_cached() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 3).await;
    let source = FakePeerSource::default();
    // Only two of three required publishers are available, with visible drift.
    seed_history_task(&pool, &source, &signers[..2], 0xa9).await;
    run_once(&pool, &source).await;
    let (complete, cacheable): (bool, bool) = sqlx::query_as(
        "SELECT localization_complete, localization_cacheable FROM block_manifest_verification_attempt",
    ).fetch_one(&pool).await.unwrap();
    assert!(
        complete,
        "the observed drift is fully localized even without quorum"
    );
    assert!(
        !cacheable,
        "missing quorum must prevent durable cache progress"
    );
    let findings: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert!(findings > 0, "below-quorum drift remains reportable");
}

#[tokio::test]
async fn completed_history_requires_the_exact_scope_and_compared_evidence() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let source = FakePeerSource::default();
    seed_history_task(&pool, &source, &signers, 0xa9).await;
    run_once(&pool, &source).await;
    sqlx::query("UPDATE block_manifest_verification_task SET next_attempt_at = NOW()")
        .execute(&pool)
        .await
        .unwrap();
    let claim = claim_due_task(&pool, "next-worker", Duration::from_secs(60), GENERATION)
        .await
        .unwrap()
        .unwrap();
    let mut trx = pool.begin().await.unwrap();
    let manifests = load_claim_manifests(&mut trx, &claim).await.unwrap();
    let evaluation = evaluate_quorum_with_history(
        &manifests,
        &[],
        claim.scope.local_publisher,
        claim.required_quorum,
    );
    assert_eq!(
        load_completed_history(&mut trx, &claim, &evaluation)
            .await
            .unwrap()
            .len(),
        1
    );
    assert!(
        !localization_is_cacheable(&evaluation, false),
        "incomplete attempts cannot populate the cache"
    );

    let historical_index = evaluation
        .scopes
        .iter()
        .position(|scope| matches!(scope.scope, CommitmentScope::Historical { .. }))
        .unwrap();
    for field in [
        "first",
        "last",
        "scale",
        "hash",
        "local digest",
        "peer digest",
        "publishers",
        "missing historical quorum",
        "missing detailed quorum",
    ] {
        let mut changed = evaluation.clone();
        let scope = &mut changed.scopes[historical_index];
        match field {
            "first" => {
                if let CommitmentScope::Historical { first, .. } = &mut scope.scope {
                    *first -= U256::ONE;
                }
            }
            "last" => {
                if let CommitmentScope::Historical { last, .. } = &mut scope.scope {
                    *last += U256::ONE;
                }
            }
            "scale" => {
                if let CommitmentScope::Historical { scale, .. } = &mut scope.scope {
                    *scale += 1;
                }
            }
            "hash" => {
                if let CommitmentScope::Historical { end_block_hash, .. } = &mut scope.scope {
                    *end_block_hash = B256::repeat_byte(0xb9);
                }
            }
            "local digest" => scope.local_digest = Some(B256::repeat_byte(0x81)),
            "peer digest" => {
                scope
                    .groups
                    .iter_mut()
                    .find(|group| Some(group.digest) != scope.local_digest)
                    .unwrap()
                    .digest = B256::repeat_byte(0x82)
            }
            "publishers" => scope
                .groups
                .iter_mut()
                .find(|group| Some(group.digest) != scope.local_digest)
                .unwrap()
                .publishers
                .push(Address::repeat_byte(0x83)),
            "missing historical quorum" => scope.quorum_digest = None,
            "missing detailed quorum" => {
                changed
                    .scopes
                    .iter_mut()
                    .find(|scope| matches!(scope.scope, CommitmentScope::Detailed { .. }))
                    .unwrap()
                    .quorum_digest = None
            }
            _ => unreachable!(),
        }
        assert!(
            load_completed_history(&mut trx, &claim, &changed)
                .await
                .unwrap()
                .is_empty(),
            "must not reuse changed {field}"
        );
    }
    for field in ["generation", "chain", "context", "publisher", "threshold"] {
        let mut changed = claim.clone();
        match field {
            "generation" => changed.scope.generation = "another-epoch".to_owned(),
            "chain" => changed.scope.host_chain_id += 1,
            "context" => changed.scope.coprocessor_context_id += U256::ONE,
            "publisher" => changed.scope.local_publisher = Address::repeat_byte(0x84),
            "threshold" => changed.required_quorum += 1,
            _ => unreachable!(),
        }
        assert!(
            load_completed_history(&mut trx, &changed, &evaluation)
                .await
                .unwrap()
                .is_empty(),
            "must not reuse changed {field}"
        );
    }
    // The publication identity is not the historical comparison identity.
    let mut later = claim.clone();
    later.scope.publication_block_number += 1;
    later.scope.publication_block_hash = B256::repeat_byte(0xdd);
    later.scope.revision += 1;
    assert_eq!(
        load_completed_history(&mut trx, &later, &evaluation)
            .await
            .unwrap()
            .len(),
        1
    );
    trx.rollback().await.unwrap();
}

#[tokio::test]
async fn divergent_forks_at_the_same_height_are_both_localized() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let source = FakePeerSource::default();
    for fork in [0xa9, 0xb9] {
        seed_history_task(&pool, &source, &signers, fork).await;
        run_once(&pool, &source).await;
    }
    let forks: i64 = sqlx::query_scalar("SELECT COUNT(DISTINCT block_hash) FROM drifted_handle")
        .fetch_one(&pool)
        .await
        .unwrap();
    assert_eq!(
        forks, 2,
        "one fork's completed localization cannot hide another fork"
    );
}

async fn recover_historical_localization(history_archived_before_crash: bool) {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let source = FakePeerSource::default();
    seed_history_task(&pool, &source, &signers, 0xa9).await;
    let claim = claim_due_task(&pool, "crashed", Duration::from_secs(60), GENERATION)
        .await
        .unwrap()
        .unwrap();
    for peer in &claim.peers {
        download_claimed_peer(&pool, &source, &claim, peer)
            .await
            .unwrap();
        assert_eq!(source.body_downloads(peer.publisher), 1);
    }
    if history_archived_before_crash {
        archive_history_for_disagreements(&pool, &source, &claim)
            .await
            .unwrap();
        for peer in &claim.peers {
            assert_eq!(source.body_downloads(peer.publisher), 2);
        }
    }
    sqlx::query(
        "UPDATE block_manifest_verification_task SET claim_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1",
    ).bind(claim.task_id).execute(&pool).await.unwrap();

    run_once(&pool, &source).await;
    let (attempt, complete, cacheable, handles): (i32, bool, bool, i64) = sqlx::query_as(
        "SELECT attempt, localization_complete, localization_cacheable, drifted_handle_count
           FROM block_manifest_verification_attempt",
    )
    .fetch_one(&pool)
    .await
    .unwrap();
    assert_eq!(attempt, 1, "recovery resumes the interrupted attempt");
    assert!(complete, "all pinned peers remain available for history");
    assert!(cacheable, "recovered historical comparison retains quorum");
    assert!(handles > 0, "historical drift is localized after recovery");
    for peer in &claim.peers {
        assert_eq!(
            source.body_downloads(peer.publisher),
            2,
            "each current and historical body is downloaded exactly once"
        );
        assert_eq!(
            source.list_calls(peer.publisher),
            2,
            "recovery does not repeat completed current or archived historical listings"
        );
    }
}

#[tokio::test]
async fn recovery_fetches_history_after_all_current_downloads_completed() {
    recover_historical_localization(false).await;
}

#[tokio::test]
async fn recovery_localizes_history_already_archived_before_crash() {
    recover_historical_localization(true).await;
}

// Signed current roots contradict signed predecessor bodies. Compare those
// bodies anyway, without claiming the advertised roots have been localized.
async fn inconsistent_history_remains_unresolved(peer_content: u8) {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers, 2).await;
    let source = FakePeerSource::default();
    for (index, signer) in signers.iter().enumerate() {
        let previous = sign_payload(
            signer,
            payload_at(
                signer.address(),
                if index == 0 { 1 } else { peer_content },
                42,
                B256::repeat_byte(0xa9),
            ),
        )
        .await;
        let mut current =
            payload_with_history(signer.address(), 1, 43, B256::repeat_byte(0xaa), &previous);
        if index != 0 {
            current.historical_ranges[0].digest = B256::repeat_byte(0xee);
        }
        let current = sign_payload(signer, current).await;
        if index == 0 {
            archive_local_only(&pool, &previous).await;
            schedule_local(&pool, &current, 1).await;
        } else {
            source.set_manifests(signer.address(), &[previous, current]);
        }
    }
    for attempt in 1..=2 {
        run_once(&pool, &source).await;
        assert_target(
            &pool,
            if attempt == 1 {
                "pending"
            } else {
                "retry_exhausted"
            },
            "drift",
            attempt,
        )
        .await;
        let (complete, cacheable): (bool, bool) = sqlx::query_as(
            "SELECT localization_complete, localization_cacheable FROM block_manifest_verification_attempt WHERE attempt = $1",
        ).bind(attempt).fetch_one(&pool).await.unwrap();
        assert!(!complete, "contradictory commitments remain unresolved");
        assert!(
            !cacheable,
            "contradictory commitments cannot populate the cache"
        );
        let findings: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM drifted_handle")
            .fetch_one(&pool)
            .await
            .unwrap();
        if peer_content == 1 {
            assert_eq!(findings, 0, "equal handles must not create drift findings");
        } else {
            assert!(
                findings > 0,
                "retain handle differences despite inconsistent roots"
            );
        }
        sqlx::query("UPDATE block_manifest_verification_task SET next_attempt_at = NOW() - INTERVAL '1 second' WHERE state = 'pending'")
            .execute(&pool).await.unwrap();
    }
    assert!(
        run_peer_manifest_download_once(
            &pool,
            &source,
            "after-exhaustion",
            Duration::from_secs(60),
            GENERATION
        )
        .await
        .unwrap()
        .is_none(),
        "unchanged inconsistent evidence must not retry forever"
    );
}

#[tokio::test]
async fn inconsistent_history_with_equal_handles_exhausts_without_findings() {
    inconsistent_history_remains_unresolved(1).await;
}

#[tokio::test]
async fn inconsistent_history_preserves_handle_findings() {
    inconsistent_history_remains_unresolved(2).await;
}
