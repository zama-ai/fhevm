mod common;

use crate::common::{create_mock_user_decryption_request_tx, init_kms_worker};
use alloy::{
    primitives::U256,
    providers::{ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
};
use connector_utils::tests::{
    db::requests::{
        InsertRequestOptions, TestEventType, check_no_uncompleted_request_in_db,
        check_request_failed_in_db, insert_rand_request,
    },
    rand::{rand_digest, rand_sns_ct},
    setup::{
        DbInstance, TESTING_KMS_CONTEXT, TestInstanceBuilder, erc1271_magic_response,
        init_host_chains_acl_contracts_mock,
    },
};
use kms_worker::core::Config;
use mocktail::server::MockServer;
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

/// Context ID does not exist in the DB → Recoverable error → retried until max attempts → failed.
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
        .with_db(DbInstance::setup().await?)
        .build();

    const MAX_DECRYPTION_ATTEMPTS: u16 = 3;

    // Use a context_id that does not exist in the DB
    let unknown_context_id = U256::from(69);

    let asserter = Asserter::new();
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
    }

    let gateway_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    info!("Gateway mock started!");

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
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        gateway_mock_provider,
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
        .with_db(DbInstance::setup().await?)
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

    let asserter = Asserter::new();
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

    let gateway_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    info!("Gateway mock started!");

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
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        gateway_mock_provider,
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
