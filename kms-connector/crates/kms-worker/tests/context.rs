mod common;

use crate::common::{
    create_mock_user_decryption_request_tx, init_kms_worker, mock_copro_registry_load,
    testing_ct_attestation_config,
};
use alloy::{
    primitives::U256,
    providers::{Provider, ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
};
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, TestEventType, check_no_uncompleted_request_in_db,
            check_request_failed_in_db, insert_rand_request,
        },
        rand::{rand_digest, rand_sns_ct},
        setup::{
            DbInstance, TestInstance, TestInstanceBuilder, erc1271_magic_response,
            init_host_chains_acl_contracts_mock,
        },
    },
    types::{DEFAULT_EPOCH_ID, TESTING_KMS_CONTEXT, extra_data::ExtraData},
};
use kms_worker::core::{
    Config,
    event_processor::{ContextManager, DbContextManager, ProcessingError, RequestCheckError},
};
use mocktail::server::MockServer;
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

/// Context ID neither in the DB nor valid on-chain → Recoverable error → retried until max
/// attempts → failed.
#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption)]
#[case::user_decryption(TestEventType::UserDecryption)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_decryption_context_not_found(
    #[case] event_type: TestEventType,
) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup_external().await?)
        .build();

    const MAX_DECRYPTION_ATTEMPTS: u16 = 3;

    // Use a context_id that does not exist in the DB
    let unknown_context_id = U256::from(69);

    // The registry mocks are only consumed by its initial load — this suite fails at the
    // context-validation stage, before any ciphertext interaction.
    let asserter = Asserter::new();
    mock_copro_registry_load(&asserter, "http://unused-bucket-url");
    let sns_ct = rand_sns_ct();
    let tx_hash = rand_digest();
    let insert_options = InsertRequestOptions::new()
        .with_sns_ct_materials(vec![sns_ct.clone()])
        .with_tx_hash(tx_hash)
        .with_context_id(unknown_context_id);

    for _ in 0..MAX_DECRYPTION_ATTEMPTS {
        if matches!(event_type, TestEventType::UserDecryption) {
            // Mocking `get_transaction_by_hash` call result
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
            asserter.push_success(&mock_tx);
        }
        // The unknown context falls back to on-chain validation: `isValidKmsContext` → false
        asserter.push_success(&false.abi_encode());
    }

    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    info!("Gateway + Ethereum mock started!");

    // Mocking Host chain ACL to ALLOW decryption.
    // Per attempt: Public → 1 bool; Legacy user → 2 bools;
    // V2 → 1 `isValidSignature` (RFC-012) + 1 U256 (invalidation) + 1 bool (ownership).
    let acl_responses = match event_type {
        TestEventType::PublicDecryption => {
            vec![true.abi_encode(); MAX_DECRYPTION_ATTEMPTS as usize]
        }
        TestEventType::UserDecryptionV2 => (0..MAX_DECRYPTION_ATTEMPTS)
            .flat_map(|_| {
                vec![
                    erc1271_magic_response(),
                    U256::ZERO.abi_encode(),
                    true.abi_encode(),
                ]
            })
            .collect(),
        TestEventType::UserDecryption => {
            vec![true.abi_encode(); 2 * MAX_DECRYPTION_ATTEMPTS as usize]
        }
        _ => vec![],
    };
    let acl_contracts_mock =
        init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), acl_responses);

    // No KMS mocks needed - request should fail before reaching KMS
    let kms_mock_server = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
        max_decryption_attempts: MAX_DECRYPTION_ATTEMPTS,
        db_fast_event_polling: Duration::from_millis(500),
        ct_attestation: testing_ct_attestation_config(false),
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        mock_provider,
        acl_contracts_mock,
        test_instance.db(),
    )
    .await?;

    insert_rand_request(test_instance.db(), event_type, insert_options).await?;

    let cancel_token = CancellationToken::new();
    let kms_worker_task = tokio::spawn(kms_worker.start(cancel_token.clone()));
    info!("KmsWorker started!");

    // Waiting for kms_worker to mark the request as failed (after MAX_DECRYPTION_ATTEMPTS retries)
    while let Err(e) = check_request_failed_in_db(test_instance.db(), event_type).await {
        warn!("Request not yet failed: {e}");
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Verify no pending requests remain
    check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;

    cancel_token.cancel();
    kms_worker_task.await.unwrap();
    Ok(())
}

/// Context exists but is_valid = false → Irrecoverable error → immediately failed.
#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption)]
#[case::user_decryption(TestEventType::UserDecryption)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_decryption_context_invalid(#[case] event_type: TestEventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup_external().await?)
        .build();

    const MAX_DECRYPTION_ATTEMPTS: u16 = 3;

    // Invalidate the testing context that was created by DbInstance::setup
    sqlx::query!(
        "UPDATE kms_context SET is_valid = false WHERE id = $1",
        TESTING_KMS_CONTEXT.as_le_slice(),
    )
    .execute(test_instance.db())
    .await?;
    info!("Context #{TESTING_KMS_CONTEXT} marked as invalid!");

    // The registry mocks are only consumed by its initial load — this suite fails at the
    // context-validation stage, before any ciphertext interaction.
    let asserter = Asserter::new();
    mock_copro_registry_load(&asserter, "http://unused-bucket-url");
    let sns_ct = rand_sns_ct();
    let tx_hash = rand_digest();
    let insert_options = InsertRequestOptions::new()
        .with_sns_ct_materials(vec![sns_ct.clone()])
        .with_tx_hash(tx_hash);
    // Default context_id = TESTING_KMS_CONTEXT

    // Only 1 attempt needed — irrecoverable error means no retry
    match event_type {
        TestEventType::PublicDecryption => {
            asserter.push_success(&false.abi_encode());
        }
        TestEventType::UserDecryption => {
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
            asserter.push_success(&mock_tx);
        }
        TestEventType::UserDecryptionV2 => (),
        _ => panic!("Unexpected event kind"),
    };

    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    info!("Gateway + Ethereum mock started!");

    // Mocking Host chain ACL to ALLOW decryption (1 attempt only, irrecoverable error).
    // Per attempt: Public → 1 bool; Legacy user → 2 bools;
    // V2 → 1 `isValidSignature` (RFC-012) + 1 U256 (invalidation) + 1 bool (ownership).
    let acl_responses = match event_type {
        TestEventType::PublicDecryption => vec![true.abi_encode()],
        TestEventType::UserDecryptionV2 => vec![
            erc1271_magic_response(),
            U256::ZERO.abi_encode(),
            true.abi_encode(),
        ],
        TestEventType::UserDecryption => vec![true.abi_encode(); 2],
        _ => vec![],
    };
    let acl_contracts_mock =
        init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), acl_responses);

    let kms_mock_server = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
        max_decryption_attempts: MAX_DECRYPTION_ATTEMPTS,
        db_fast_event_polling: Duration::from_millis(500),
        ct_attestation: testing_ct_attestation_config(false),
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        mock_provider,
        acl_contracts_mock,
        test_instance.db(),
    )
    .await?;

    insert_rand_request(test_instance.db(), event_type, insert_options).await?;

    let cancel_token = CancellationToken::new();
    let kms_worker_task = tokio::spawn(kms_worker.start(cancel_token.clone()));
    info!("KmsWorker started!");

    // Waiting for kms_worker to mark the request as failed (immediately — irrecoverable)
    while let Err(e) = check_request_failed_in_db(test_instance.db(), event_type).await {
        warn!("Request not yet failed: {e}");
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Verify no pending requests remain
    check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;

    cancel_token.cancel();
    kms_worker_task.await.unwrap();
    Ok(())
}

/// Builds a `DbContextManager` whose on-chain fallback is served by the given `Asserter`.
async fn setup_context_manager(
    asserter: Asserter,
) -> anyhow::Result<(TestInstance, DbContextManager<impl Provider + Clone>)> {
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup_external().await?)
        .build();
    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    let context_manager = DbContextManager::new(
        test_instance.db().clone(),
        &Config::default(),
        mock_provider,
    );
    Ok((test_instance, context_manager))
}

/// Pair unknown locally but valid on-chain → fallback validates and caches it: the second
/// validation must succeed from the DB alone (the asserter queue is then empty, so any other
/// RPC call would fail).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_fallback_caches_valid_pair() -> anyhow::Result<()> {
    let asserter = Asserter::new();
    asserter.push_success(&true.abi_encode()); // isValidKmsContext
    asserter.push_success(&true.abi_encode()); // isValidEpochForContext
    let (test_instance, context_manager) = setup_context_manager(asserter).await?;

    let context_id = U256::from(33);
    let epoch_id = U256::from(5);
    let extra_data = ExtraData {
        context_id: Some(context_id),
        epoch_id: Some(epoch_id),
    };
    context_manager.validate_context(&extra_data).await?;
    context_manager.validate_context(&extra_data).await?;

    let context_cached: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kms_context WHERE id = $1")
        .bind(context_id.as_le_slice())
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(context_cached, 1);
    let epoch_cached: i64 =
        sqlx::query_scalar("SELECT COUNT(*) FROM kms_epoch WHERE id = $1 AND context_id = $2")
            .bind(epoch_id.as_le_slice())
            .bind(context_id.as_le_slice())
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(epoch_cached, 1);
    Ok(())
}

/// Valid context but epoch not active on-chain (e.g. still pending) → Recoverable error, and
/// the pair must not be cached.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_pending_epoch_is_recoverable() -> anyhow::Result<()> {
    let asserter = Asserter::new();
    asserter.push_success(&true.abi_encode()); // isValidKmsContext
    asserter.push_success(&false.abi_encode()); // isValidEpochForContext
    let (test_instance, context_manager) = setup_context_manager(asserter).await?;

    let context_id = U256::from(33);
    let epoch_id = U256::from(5);
    let extra_data = ExtraData {
        context_id: Some(context_id),
        epoch_id: Some(epoch_id),
    };
    let err = context_manager
        .validate_context(&extra_data)
        .await
        .map_err(RequestCheckError::record)
        .unwrap_err();
    assert!(
        matches!(err, ProcessingError::Recoverable(_)),
        "unexpected error: {err}"
    );

    let cached: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kms_context WHERE id = $1")
        .bind(context_id.as_le_slice())
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(cached, 0, "an inactive pair should not be cached");
    Ok(())
}

/// Destroyed context → Irrecoverable error for any epoch, even one unknown locally, without
/// falling back to any RPC call (the asserter queue is empty): the context invalidation alone
/// concludes.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_destroyed_rejects_any_epoch() -> anyhow::Result<()> {
    let (test_instance, context_manager) = setup_context_manager(Asserter::new()).await?;

    sqlx::query!(
        "UPDATE kms_context SET is_valid = false WHERE id = $1",
        TESTING_KMS_CONTEXT.as_le_slice(),
    )
    .execute(test_instance.db())
    .await?;

    for epoch_id in [Some(DEFAULT_EPOCH_ID), Some(U256::from(99)), None] {
        let extra_data = ExtraData {
            context_id: Some(TESTING_KMS_CONTEXT),
            epoch_id,
        };
        let err = context_manager
            .validate_context(&extra_data)
            .await
            .map_err(RequestCheckError::record)
            .unwrap_err();
        assert!(
            matches!(err, ProcessingError::Irrecoverable(_)),
            "unexpected error for epoch {epoch_id:?}: {err}"
        );
    }
    Ok(())
}

/// A destroyed epoch → Irrecoverable error for requests referencing it, while the other epochs
/// of the same context keep validating from the DB alone, without falling back to any RPC call
/// (the asserter queue is empty).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_destroyed_epoch_leaves_siblings_valid() -> anyhow::Result<()> {
    let (test_instance, context_manager) = setup_context_manager(Asserter::new()).await?;

    // Invalidate an epoch of the testing context, mirroring what the gw-listener does on
    // `KmsEpochDestroyed`. The `context_id` is left NULL, as the event does not carry it: the
    // epoch may not even be cached when the destruction event arrives.
    let destroyed_epoch_id = U256::from(42);
    sqlx::query(
        "INSERT INTO kms_epoch(id, is_valid, created_at, updated_at)
        VALUES ($1, FALSE, NOW(), NOW())",
    )
    .bind(destroyed_epoch_id.as_le_slice())
    .execute(test_instance.db())
    .await?;

    let err = context_manager
        .validate_context(&ExtraData {
            context_id: Some(TESTING_KMS_CONTEXT),
            epoch_id: Some(destroyed_epoch_id),
        })
        .await
        .map_err(RequestCheckError::record)
        .unwrap_err();
    assert!(
        matches!(err, ProcessingError::Irrecoverable(_)),
        "unexpected error: {err}"
    );

    // The sibling epoch seeded by `DbInstance::setup` must remain valid, from the DB alone.
    context_manager
        .validate_context(&ExtraData {
            context_id: Some(TESTING_KMS_CONTEXT),
            epoch_id: Some(DEFAULT_EPOCH_ID),
        })
        .await?;
    Ok(())
}

/// An epoch cached as valid under another context does not conclude locally (the cached
/// association is not authoritative): the pair falls back to on-chain validation, which
/// rejects it → Recoverable error, and the cached association is left untouched.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_epoch_of_other_context_falls_back_on_chain() -> anyhow::Result<()> {
    let asserter = Asserter::new();
    asserter.push_success(&true.abi_encode()); // isValidKmsContext
    asserter.push_success(&false.abi_encode()); // isValidEpochForContext
    let (test_instance, context_manager) = setup_context_manager(asserter).await?;

    // A second valid context, requested with the epoch seeded for `TESTING_KMS_CONTEXT`.
    let other_context_id = U256::from(34);
    sqlx::query(
        "INSERT INTO kms_context(id, is_valid, created_at, updated_at)
        VALUES ($1, TRUE, NOW(), NOW())",
    )
    .bind(other_context_id.as_le_slice())
    .execute(test_instance.db())
    .await?;

    let err = context_manager
        .validate_context(&ExtraData {
            context_id: Some(other_context_id),
            epoch_id: Some(DEFAULT_EPOCH_ID),
        })
        .await
        .map_err(RequestCheckError::record)
        .unwrap_err();
    assert!(
        matches!(err, ProcessingError::Recoverable(_)),
        "unexpected error: {err}"
    );

    let cached_context: Vec<u8> =
        sqlx::query_scalar("SELECT context_id FROM kms_epoch WHERE id = $1")
            .bind(DEFAULT_EPOCH_ID.as_le_slice())
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(
        cached_context,
        TESTING_KMS_CONTEXT.as_le_slice(),
        "a rejected pair should not alter the cached association"
    );
    Ok(())
}

/// An epoch cached as valid under another context, but whose requested pair the chain confirms
/// (e.g. the cached association went stale after a reorg) → Valid, and the cached association
/// is repaired: the second validation must succeed from the DB alone (the asserter queue is
/// then empty, so any other RPC call would fail).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_stale_epoch_association_self_heals() -> anyhow::Result<()> {
    let asserter = Asserter::new();
    asserter.push_success(&true.abi_encode()); // isValidKmsContext
    asserter.push_success(&true.abi_encode()); // isValidEpochForContext
    let (test_instance, context_manager) = setup_context_manager(asserter).await?;

    // A second valid context, requested with the epoch seeded for `TESTING_KMS_CONTEXT`.
    let other_context_id = U256::from(34);
    sqlx::query(
        "INSERT INTO kms_context(id, is_valid, created_at, updated_at)
        VALUES ($1, TRUE, NOW(), NOW())",
    )
    .bind(other_context_id.as_le_slice())
    .execute(test_instance.db())
    .await?;

    let extra_data = ExtraData {
        context_id: Some(other_context_id),
        epoch_id: Some(DEFAULT_EPOCH_ID),
    };
    context_manager.validate_context(&extra_data).await?;

    let cached_context: Vec<u8> =
        sqlx::query_scalar("SELECT context_id FROM kms_epoch WHERE id = $1")
            .bind(DEFAULT_EPOCH_ID.as_le_slice())
            .fetch_one(test_instance.db())
            .await?;
    assert_eq!(
        cached_context,
        other_context_id.as_le_slice(),
        "the cached association should be repaired from the on-chain result"
    );

    context_manager.validate_context(&extra_data).await?;
    Ok(())
}

/// v1 extra_data (no epoch) referencing a context known locally → Valid from the DB alone,
/// without any RPC call (the asserter queue is empty).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_v1_extra_data_validates_context_only() -> anyhow::Result<()> {
    let (_test_instance, context_manager) = setup_context_manager(Asserter::new()).await?;

    let extra_data = ExtraData {
        context_id: Some(TESTING_KMS_CONTEXT),
        epoch_id: None,
    };
    context_manager.validate_context(&extra_data).await?;
    Ok(())
}

/// v1 extra_data (no epoch) referencing a context unknown locally → on-chain fallback checks
/// `isValidKmsContext` only (a single RPC response is queued), and nothing is cached since
/// there is no epoch to cache the pair with.
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_v1_unknown_context_falls_back_without_caching() -> anyhow::Result<()>
{
    let asserter = Asserter::new();
    asserter.push_success(&true.abi_encode()); // isValidKmsContext
    let (test_instance, context_manager) = setup_context_manager(asserter).await?;

    let context_id = U256::from(33);
    let extra_data = ExtraData {
        context_id: Some(context_id),
        epoch_id: None,
    };
    context_manager.validate_context(&extra_data).await?;

    let cached: i64 = sqlx::query_scalar("SELECT COUNT(*) FROM kms_context WHERE id = $1")
        .bind(context_id.as_le_slice())
        .fetch_one(test_instance.db())
        .await?;
    assert_eq!(cached, 0, "a context without an epoch should not be cached");
    Ok(())
}

/// v2 extra_data carrying the production `DEFAULT_EPOCH_ID` encoding (`0x08…01`, the value the
/// migration backfills) → Valid from the seeded DB row alone, without any RPC call (the
/// asserter queue is empty).
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_validate_context_default_epoch_id_matches_seeded_row() -> anyhow::Result<()> {
    let (_test_instance, context_manager) = setup_context_manager(Asserter::new()).await?;

    let extra_data = ExtraData {
        context_id: Some(TESTING_KMS_CONTEXT),
        epoch_id: Some(DEFAULT_EPOCH_ID),
    };
    context_manager.validate_context(&extra_data).await?;
    Ok(())
}
