use super::localization_cache_tests::seed_history_task;
use super::*;

struct SlowPeerSource {
    inner: FakePeerSource,
    missing_history: bool,
}

impl PeerManifestSource for SlowPeerSource {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError> {
        tokio::time::sleep(Duration::from_millis(400)).await;
        if self.missing_history && request.publication_block_number == 42 {
            return Ok(vec![]);
        }
        self.inner.list_manifests(request).await
    }

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError> {
        tokio::time::sleep(Duration::from_millis(400)).await;
        self.inner.fetch_manifest(request, object_key).await
    }
}

async fn verify_slow_downloads(missing_history: bool) {
    let (_instance, pool) = setup_download_db().await;
    let signers = five_test_signers();
    seed_registry(&pool, &signers, 3).await;
    let source = SlowPeerSource {
        inner: FakePeerSource::default(),
        missing_history,
    };
    seed_history_task(&pool, &source.inner, &signers, 0xa9).await;
    let lease = Duration::from_secs(2);
    let started = tokio::time::Instant::now();
    let result = run_peer_manifest_download_once(&pool, &source, "slow-worker", lease, GENERATION)
        .await
        .expect("renewed downloads complete")
        .expect("due task");
    assert!(
        started.elapsed() > lease,
        "total work exceeds the initial lease"
    );
    assert_eq!(result.attempt, 1);
    assert_eq!(result.outcome, VerificationOutcome::Drift);
    assert_target(&pool, "pending", "drift", 1).await;
    let complete: bool =
        sqlx::query_scalar("SELECT localization_complete FROM block_manifest_verification_attempt")
            .fetch_one(&pool)
            .await
            .unwrap();
    assert_eq!(complete, !missing_history);
    for signer in &signers[1..] {
        assert_eq!(
            source.inner.body_downloads(signer.address()),
            if missing_history { 1 } else { 2 }
        );
    }
}

#[tokio::test]
async fn renewal_allows_current_and_historical_downloads_beyond_one_lease() {
    verify_slow_downloads(false).await;
}

#[tokio::test]
async fn renewal_lets_unavailable_history_consume_the_attempt_budget() {
    verify_slow_downloads(true).await;
}

#[tokio::test]
async fn expired_or_reclaimed_worker_cannot_renew_or_start_downloads() {
    let task = claimed_local_task(1).await;
    let source = FakePeerSource::default();
    let peer = &task.claim.peers[0];
    sqlx::query("UPDATE block_manifest_verification_task SET claim_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1")
        .bind(task.claim.task_id).execute(&task.pool).await.unwrap();
    assert!(
        download_claimed_peer(&task.pool, &source, &task.claim, peer)
            .await
            .is_err()
    );
    assert_eq!(
        source.list_calls(peer.publisher),
        0,
        "expired worker starts no network work"
    );
    let replacement = claim_due_task(
        &task.pool,
        "replacement",
        Duration::from_secs(60),
        GENERATION,
    )
    .await
    .unwrap()
    .unwrap();
    assert!(renew_download_claim(&task.pool, &task.claim).await.is_err());
    assert!(
        finish_claim(&task.pool, &task.claim).await.is_err(),
        "old worker cannot finalize"
    );
    renew_download_claim(&task.pool, &replacement)
        .await
        .unwrap();
    let owner: String =
        sqlx::query_scalar("SELECT claim_owner FROM block_manifest_verification_task")
            .fetch_one(&task.pool)
            .await
            .unwrap();
    assert_eq!(owner, "replacement");
    assert_verification_task(&task.pool, "claimed", 0).await;
}
