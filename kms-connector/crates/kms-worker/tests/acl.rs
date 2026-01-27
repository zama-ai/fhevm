mod common;

use crate::common::{create_mock_user_decryption_request_tx, init_kms_worker};
use alloy::{
    providers::{ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
};
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, check_no_uncompleted_request_in_db, check_request_failed_in_db,
            insert_rand_request,
        },
        rand::{rand_digest, rand_sns_ct},
        setup::{DbInstance, TestInstanceBuilder, init_host_chains_acl_contracts_mock},
    },
    types::db::EventType,
};
use kms_worker::core::Config;
use mocktail::server::MockServer;
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[case::public_decryption(EventType::PublicDecryptionRequest)]
#[case::user_decryption(EventType::UserDecryptionRequest)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_decryption_acl_failure(#[case] event_type: EventType) -> anyhow::Result<()> {
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup().await?)
        .build();

    // Mocking Gateway
    let asserter = Asserter::new();
    let sns_ct = rand_sns_ct();
    let mut insert_options =
        InsertRequestOptions::new().with_sns_ct_materials(vec![sns_ct.clone()]);
    match event_type {
        EventType::PublicDecryptionRequest => {
            // Mocking isDecryptionDone returns false
            asserter.push_success(&false.abi_encode());
        }
        EventType::UserDecryptionRequest => {
            // Mocking `get_transaction_by_hash` call result
            let tx_hash = rand_digest();
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
            insert_options = insert_options.with_tx_hash(tx_hash);
            asserter.push_success(&mock_tx);
        }
        _ => unreachable!(),
    };
    let gateway_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter);
    info!("Gateway mock started!");

    // Mocking Host chain ACL to DENY decryption
    // Public: 1 ACL check, User: 2 ACL checks (user + contract)
    let acl_responses = match event_type {
        EventType::PublicDecryptionRequest => vec![false],
        EventType::UserDecryptionRequest => vec![false, false],
        _ => unreachable!(),
    };
    let acl_contracts_mock =
        init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), acl_responses);

    // No KMS mocks needed - request should fail before reaching KMS
    let kms_mock_server = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    // Starting kms_worker
    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
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

    // Waiting for kms_worker to mark the request as failed
    while let Err(e) = check_request_failed_in_db(test_instance.db(), event_type).await {
        warn!("Request not yet failed: {e}");
        tokio::time::sleep(Duration::from_millis(200)).await;
    }

    // Verify no pending requests remain
    check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;

    // Stopping the test
    cancel_token.cancel();
    kms_worker_task.await.unwrap();
    Ok(())
}
