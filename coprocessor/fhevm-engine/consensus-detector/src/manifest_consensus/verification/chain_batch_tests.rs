use super::*;

async fn chain_manifest(signer: &PrivateKeySigner, chain: u64, height: i64) -> SignedManifest {
    let mut payload = payload_at(signer.address(), 1, height, test_block_hash());
    payload.host_chain_id = U256::from(chain);
    let block = &mut payload.detailed_range.blocks[0];
    block.block_content_digest = block_content_digest(
        payload.version,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        block.block_number,
        block.block_hash,
        &block.ciphertexts,
    )
    .unwrap();
    payload.detailed_range.digest = detailed_range_digest(
        payload.version,
        payload.coprocessor_context_id,
        payload.host_chain_id,
        block.block_number,
        block.block_number,
        &[block.block_content_digest],
    );
    sign_payload(signer, payload).await
}

struct ConcurrentSource {
    inner: FakePeerSource,
    started: Barrier,
}

impl PeerManifestSource for ConcurrentSource {
    async fn list_manifests(
        &self,
        request: &PeerDownloadRequest,
    ) -> Result<Vec<String>, ExecutionError> {
        // Neither chain can finish until both have reached network work.
        self.started.wait().await;
        self.inner.list_manifests(request).await
    }

    async fn fetch_manifest(
        &self,
        request: &PeerDownloadRequest,
        object_key: &str,
    ) -> Result<PeerManifestObject, ExecutionError> {
        self.inner.fetch_manifest(request, object_key).await
    }
}

#[tokio::test]
#[serial(db)]
async fn batch_verifies_chains_concurrently_and_leaves_same_chain_successor_pending() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    // Schedule before registry arrival to exercise binding in each separate claim.
    for (chain, height) in [(7, 42), (7, 43), (8, 42)] {
        schedule_local(&pool, &chain_manifest(&signers[0], chain, height).await, 1).await;
    }
    seed_registry(&pool, &signers[..2], 2).await;
    sqlx::query("UPDATE block_manifest_verification_task SET next_attempt_at = NOW()")
        .execute(&pool)
        .await
        .unwrap();
    let candidates = ready_verification_tasks(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap();
    assert_eq!(candidates.len(), 2);
    let source = ConcurrentSource {
        inner: FakePeerSource::default(),
        started: Barrier::new(2),
    };
    source.inner.set_manifests(
        signers[1].address(),
        &[
            chain_manifest(&signers[1], 7, 42).await,
            chain_manifest(&signers[1], 8, 42).await,
        ],
    );
    tokio::time::timeout(
        Duration::from_secs(15),
        run_verification_batch(&pool, &source, "batch", &ManifestWorkGate::always_enabled()),
    )
    .await
    .expect("both chains must reach the barrier concurrently")
    .unwrap();
    let rows: Vec<(i64, String, i32)> = sqlx::query_as(
        "SELECT id, state, attempt_count FROM block_manifest_verification_task ORDER BY id",
    )
    .fetch_all(&pool)
    .await
    .unwrap();
    for (id, state, attempts) in rows {
        if candidates.contains(&id) {
            assert_eq!(state, "consensus");
            assert_eq!(attempts, 1);
        } else {
            assert_eq!(state, "pending");
            assert_eq!(attempts, 0);
        }
    }
    assert_eq!(source.inner.body_downloads(signers[1].address()), 2);
}

#[tokio::test]
#[serial(db)]
async fn selected_task_is_rechecked_and_claimed_exclusively() {
    let (_instance, pool) = setup_download_db().await;
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    for height in [42, 43] {
        schedule_local(&pool, &chain_manifest(&signers[0], 7, height).await, 1).await;
    }
    let ids = ready_verification_tasks(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap();
    assert_eq!(ids.len(), 1);
    let id = ids[0];
    let mut lock = pool.begin().await.unwrap();
    sqlx::query("SELECT id FROM block_manifest_verification_task WHERE id = $1 FOR UPDATE")
        .bind(id)
        .fetch_one(lock.as_mut())
        .await
        .unwrap();
    assert!(
        claim_verification_task(&pool, "locked", DOWNLOAD_CLAIM, CONSENSUS_EPOCH, id)
            .await
            .unwrap()
            .is_none()
    );
    lock.rollback().await.unwrap();
    let (a, b) = tokio::join!(
        claim_verification_task(&pool, "a", DOWNLOAD_CLAIM, CONSENSUS_EPOCH, id),
        claim_verification_task(&pool, "b", DOWNLOAD_CLAIM, CONSENSUS_EPOCH, id),
    );
    assert_eq!(
        usize::from(a.unwrap().is_some()) + usize::from(b.unwrap().is_some()),
        1
    );
    assert!(
        claim_verification_task(&pool, "stale", DOWNLOAD_CLAIM, CONSENSUS_EPOCH, id)
            .await
            .unwrap()
            .is_none()
    );
    sqlx::query("UPDATE block_manifest_verification_task SET claim_expires_at = NOW() - INTERVAL '1 second' WHERE id = $1")
        .bind(id).execute(&pool).await.unwrap();
    assert_eq!(
        ready_verification_tasks(&pool, CONSENSUS_EPOCH)
            .await
            .unwrap(),
        ids
    );
    assert!(
        claim_verification_task(&pool, "recovery", DOWNLOAD_CLAIM, CONSENSUS_EPOCH, id)
            .await
            .unwrap()
            .is_some()
    );
    assert!(ready_verification_tasks(&pool, "another-epoch")
        .await
        .unwrap()
        .is_empty());
}

#[test]
fn polling_delay_uses_the_configured_verification_delay_as_its_cap() {
    let delay = Duration::from_secs(10);
    assert_eq!(
        verification_poll_delay(None, Duration::from_secs(3)),
        Duration::from_millis(3100)
    );
    assert_eq!(
        verification_poll_delay(None, delay),
        Duration::from_millis(10_100)
    );
    assert_eq!(
        verification_poll_delay(Some(Duration::ZERO), delay),
        Duration::from_millis(100)
    );
    assert_eq!(
        verification_poll_delay(Some(Duration::from_millis(1500)), delay),
        Duration::from_millis(1600)
    );
    assert_eq!(
        verification_poll_delay(Some(Duration::from_secs(60)), delay),
        Duration::from_millis(10_100)
    );
}

#[tokio::test]
#[serial(db)]
async fn next_poll_tracks_pending_tasks_and_claim_expiry_in_the_active_epoch() {
    let (_instance, pool) = setup_download_db().await;
    assert!(next_verification_delay(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap()
        .is_none());
    let signers = test_signers();
    seed_registry(&pool, &signers[..2], 2).await;
    schedule_local(&pool, &chain_manifest(&signers[0], 7, 42).await, 1).await;
    schedule_local(&pool, &chain_manifest(&signers[0], 8, 42).await, 1).await;
    assert_eq!(
        next_verification_delay(&pool, CONSENSUS_EPOCH)
            .await
            .unwrap(),
        Some(Duration::ZERO)
    );
    assert!(next_verification_delay(&pool, "different-epoch")
        .await
        .unwrap()
        .is_none());

    // One task is due sooner, while the other remains a minute away.
    sqlx::query("UPDATE block_manifest_verification_task SET next_attempt_at = NOW() + CASE WHEN id = (SELECT MIN(id) FROM block_manifest_verification_task) THEN INTERVAL '5 seconds' ELSE INTERVAL '1 minute' END")
        .execute(&pool).await.unwrap();
    let delay = next_verification_delay(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap()
        .unwrap();
    assert!(
        delay > Duration::from_secs(3) && delay <= Duration::from_secs(5),
        "{delay:?}"
    );

    // A past retry deadline must not cause 100 ms polling while a lease is live.
    sqlx::query("UPDATE block_manifest_verification_task SET state = 'claimed', claim_owner = 'other-worker', next_attempt_at = NOW() - INTERVAL '1 minute', claim_expires_at = NOW() + INTERVAL '8 seconds' WHERE id = (SELECT MIN(id) FROM block_manifest_verification_task)")
        .execute(&pool).await.unwrap();
    let delay = next_verification_delay(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap()
        .unwrap();
    assert!(
        delay > Duration::from_secs(6) && delay <= Duration::from_secs(8),
        "{delay:?}"
    );
    sqlx::query("UPDATE block_manifest_verification_task SET claim_expires_at = NOW() - INTERVAL '1 second' WHERE state = 'claimed'")
        .execute(&pool).await.unwrap();
    assert_eq!(
        next_verification_delay(&pool, CONSENSUS_EPOCH)
            .await
            .unwrap(),
        Some(Duration::ZERO)
    );

    sqlx::query("UPDATE block_manifest_verification_task SET state = 'consensus', claim_owner = NULL, claim_expires_at = NULL")
        .execute(&pool).await.unwrap();
    assert!(next_verification_delay(&pool, CONSENSUS_EPOCH)
        .await
        .unwrap()
        .is_none());
}
