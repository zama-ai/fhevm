mod common;

use crate::common::{create_mock_user_decryption_request_tx, init_kms_worker};
use alloy::{
    hex::FromHex,
    primitives::{FixedBytes, U256},
    providers::{ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
};
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, TestEventType, check_no_uncompleted_request_in_db,
            insert_rand_request,
        },
        rand::{rand_digest, rand_sns_ct},
        setup::{
            DbInstance, S3_CT_DIGEST, S3_CT_HANDLE, S3Instance, TestInstanceBuilder,
            erc1271_magic_response, init_host_chains_acl_contracts_mock,
        },
    },
    types::ProtocolEventKind,
};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::Coprocessor;
use kms_grpc::kms::v1::Empty;
use kms_worker::core::Config;
use mocktail::{MockSet, StatusCode, server::MockServer};
use rstest::rstest;
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[case::public_decryption_removal_after_max_attempt_reached(TestEventType::PublicDecryption)]
#[case::user_decryption_removal_after_max_attempt_reached(TestEventType::UserDecryption)]
#[case::user_decryption_v2_removal_after_max_attempt_reached(TestEventType::UserDecryptionV2)]
#[case::prep_keygen_processing_not_removed_on_error(TestEventType::PrepKeygen)]
#[case::keygen_processing_not_removed_on_error(TestEventType::Keygen)]
#[case::crsgen_processing_not_removed_on_error(TestEventType::Crsgen)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_request_processing(#[case] event_type: TestEventType) -> anyhow::Result<()> {
    // Setup real DB and S3 instance
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup().await?)
        .with_s3(S3Instance::setup().await?)
        .build();

    // Test constants
    const MAX_DECRYPTION_ATTEMPTS: u16 = 3;
    const GRPC_REQUEST_RETRIES: u8 = 2;

    // Mocking Gateway
    let asserter = Asserter::new();
    let mut sns_ct = rand_sns_ct();
    sns_ct.ctHandle = FixedBytes::<32>::from_hex(S3_CT_HANDLE)?;
    sns_ct.snsCiphertextDigest = FixedBytes::<32>::from_hex(S3_CT_DIGEST)?;
    let tx_hash = rand_digest();
    let insert_options = InsertRequestOptions::new()
        .with_sns_ct_materials(vec![sns_ct.clone()])
        .with_tx_hash(tx_hash);
    for attempt in 0..MAX_DECRYPTION_ATTEMPTS {
        if matches!(event_type, TestEventType::UserDecryption) {
            // Mocking `get_transaction_by_hash` call result
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
            asserter.push_success(&mock_tx);
        }

        // First attempt, the copro URL is not cached yet
        if attempt == 0 {
            let get_copro_call_response = Coprocessor {
                s3BucketUrl: format!("{}/ct128", test_instance.s3_url()),
                ..Default::default()
            };
            asserter.push_success(&get_copro_call_response.abi_encode());
        }
    }

    let gateway_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());
    info!("Gateway mock started!");

    // Mocking Host chain.
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

    // Insert request in DB to trigger kms_worker job
    let request = insert_rand_request(test_instance.db(), event_type, insert_options).await?;

    // Mocking KMS responses
    let kms_mocks = prepare_mocks(&request);
    let kms_mock_server =
        MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint").with_mocks(kms_mocks);
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    // Starting kms_worker
    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
        max_decryption_attempts: MAX_DECRYPTION_ATTEMPTS,
        grpc_request_retries: GRPC_REQUEST_RETRIES,
        db_fast_event_polling: Duration::from_millis(500),
        db_long_event_polling: Duration::from_millis(500),
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        gateway_mock_provider,
        acl_contracts_mock,
        test_instance.db(),
    )
    .await?;
    let cancel_token = CancellationToken::new();
    let kms_worker_task = tokio::spawn(kms_worker.start(cancel_token.clone()));
    info!("KmsWorker started!");

    match &request {
        // Wait for kms_worker to remove the request from DB, then stop it
        ProtocolEventKind::PublicDecryption(_)
        | ProtocolEventKind::UserDecryption(_)
        | ProtocolEventKind::UserDecryptionV2(_) => {
            while check_no_uncompleted_request_in_db(test_instance.db(), event_type)
                .await
                .is_err()
            {
                warn!("Still requests in DB!");
                tokio::time::sleep(Duration::from_millis(200)).await;
            }

            cancel_token.cancel();
            kms_worker_task.await.unwrap();
        }

        // Stop worker and check the request is still in DB despite the errors.
        _ => {
            tokio::time::sleep(Duration::from_secs(5)).await;
            cancel_token.cancel();
            kms_worker_task.await.unwrap();

            check_no_uncompleted_request_in_db(test_instance.db(), event_type)
                .await
                .unwrap_err();
        }
    }

    Ok(())
}

fn prepare_mocks(req: &ProtocolEventKind) -> MockSet {
    let mut kms_mocks = MockSet::new();

    // Gets the endpoints for the given request type
    let (req_endpoint, resp_endpoint) = match req {
        ProtocolEventKind::PublicDecryption(_) => ("PublicDecrypt", "GetPublicDecryptionResult"),
        ProtocolEventKind::UserDecryption(_) | ProtocolEventKind::UserDecryptionV2(_) => {
            ("UserDecrypt", "GetUserDecryptionResult")
        }
        ProtocolEventKind::PrepKeygen(_) => ("KeyGenPreproc", "GetKeyGenPreprocResult"),
        ProtocolEventKind::Keygen(_) => ("KeyGen", "GetKeyGenResult"),
        ProtocolEventKind::Crsgen(_) => ("CrsGen", "GetCrsGenResult"),
    };

    // Mock initial KMS response to initial GRPC request
    kms_mocks.mock(|when, then| {
        when.path(format!(
            "/kms_service.v1.CoreServiceEndpoint/{req_endpoint}"
        ));
        then.pb(Empty::default());
    });

    // Mock error response of result polling
    kms_mocks.mock(|when, then| {
        when.path(format!(
            "/kms_service.v1.CoreServiceEndpoint/{resp_endpoint}"
        ));
        then.error(StatusCode::SERVICE_UNAVAILABLE, "unavailable");
    });

    kms_mocks
}
