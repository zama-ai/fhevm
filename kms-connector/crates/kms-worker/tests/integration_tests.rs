mod common;

use crate::common::{
    create_mock_user_decryption_request_tx, init_kms_worker, mock_copro_registry_load,
    testing_ct_attestation_config,
};
use alloy::{
    hex::FromHex,
    primitives::{Address, FixedBytes, Log, U256},
    providers::{ProviderBuilder, mock::Asserter},
    rpc::types::Log as RpcLog,
    sol_types::{SolEvent, SolValue},
};
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, TestEventType, check_no_uncompleted_request_in_db,
            insert_rand_request,
        },
        rand::rand_digest,
        setup::{
            DbInstance, S3_CT_HANDLE, S3Instance, TestInstanceBuilder, erc1271_magic_response,
            init_host_chains_acl_contracts_mock,
        },
    },
    types::{
        KmsGrpcResponse, KmsResponse, KmsResponseKind, ProtocolEventKind, kms_response,
        u256_to_request_id,
    },
};
use fhevm_host_bindings::protocol_config::ProtocolConfig::NewKmsContext;
use kms_grpc::kms::v1::{
    CrsGenResult, Empty, EpochResultResponse as GrpcEpochResultResponse, KeyGenPreprocResult,
    KeyGenResult, PublicDecryptionResponse, PublicDecryptionResponsePayload,
    UserDecryptionResponse, UserDecryptionResponsePayload,
};
use kms_worker::core::{Config, event_processor::compute_anchor_event_hash};
use mocktail::{MockSet, StatusCode, server::MockServer};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[case::public_decryption(TestEventType::PublicDecryption, false)]
#[case::user_decryption(TestEventType::UserDecryption, false)]
#[case::user_decryption_v2(TestEventType::UserDecryptionV2, false)]
#[case::prep_keygen(TestEventType::PrepKeygen, false)]
#[case::keygen(TestEventType::Keygen, false)]
#[case::crsgen(TestEventType::Crsgen, false)]
#[case::new_kms_context(TestEventType::NewKmsContext, false)]
#[case::new_kms_epoch(TestEventType::NewKmsEpoch, false)]
#[case::abort_keygen(TestEventType::AbortKeygen, false)]
#[case::abort_crsgen(TestEventType::AbortCrsgen, false)]
#[case::kms_context_destroyed(TestEventType::KmsContextDestroyed, false)]
#[case::kms_epoch_destroyed(TestEventType::KmsEpochDestroyed, false)]
#[case::public_decryption_already_sent(TestEventType::PublicDecryption, true)]
#[case::user_decryption_already_sent(TestEventType::UserDecryption, true)]
#[case::user_decryption_v2_already_sent(TestEventType::UserDecryptionV2, true)]
#[case::prep_keygen_already_sent(TestEventType::PrepKeygen, true)]
#[case::keygen_already_sent(TestEventType::Keygen, true)]
#[case::crsgen_already_sent(TestEventType::Crsgen, true)]
#[case::new_kms_context_already_sent(TestEventType::NewKmsContext, true)]
#[case::new_kms_epoch_already_sent(TestEventType::NewKmsEpoch, true)]
#[case::abort_keygen_already_sent(TestEventType::AbortKeygen, true)]
#[case::abort_crsgen_already_sent(TestEventType::AbortCrsgen, true)]
#[case::kms_context_destroyed_already_sent(TestEventType::KmsContextDestroyed, true)]
#[case::kms_epoch_destroyed_already_sent(TestEventType::KmsEpochDestroyed, true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_processing_request(
    #[case] event_type: TestEventType,
    #[case] already_sent: bool,
) -> anyhow::Result<()> {
    // Setup real DB and S3 instance
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup_external().await?)
        .with_s3(S3Instance::setup().await?)
        .build();

    // Mocking Gateway/Ethereum
    let asserter = Asserter::new();
    mock_copro_registry_load(&asserter, test_instance.s3_url());
    let handle = FixedBytes::<32>::from_hex(S3_CT_HANDLE)?;

    let mut insert_options = InsertRequestOptions::new()
        .with_already_sent(already_sent)
        .with_ct_handles(vec![handle]);

    match event_type {
        // Only the legacy `UserDecryptionRequest` path re-fetches calldata via `get_transaction_by_hash`
        // — the RFC016 V2 event carries the full payload in-event, so no such mock is needed.
        TestEventType::UserDecryption => {
            let tx_hash = rand_digest();
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, handle)?;
            insert_options = insert_options.with_tx_hash(tx_hash);
            asserter.push_success(&mock_tx);
        }
        // The `NewKmsContext` flow makes two RPC calls before going to the Core: an `eth_call`
        // on `getKmsContextAnchor(previousContextId)` and an `eth_getLogs` to fetch the
        // previous-context event. We fabricate a default previous event, hash it the same way
        // the production code does, and queue both responses on the FIFO asserter.
        TestEventType::NewKmsContext => {
            let previous_event = NewKmsContext::default();
            let anchor_hash = compute_anchor_event_hash(&previous_event);
            asserter.push_success(&(U256::ZERO, anchor_hash).abi_encode_sequence());
            let rpc_log = RpcLog {
                inner: Log {
                    address: Address::ZERO,
                    data: previous_event.encode_log_data(),
                },
                ..Default::default()
            };
            asserter.push_success(&vec![rpc_log]);
        }
        // The `NewKmsEpoch` flow fetches the previous epoch material from `KMSGeneration` via two
        // `eth_call`s at `materialBlockNumber`: `getCompletedKeyIds()` then `getCompletedCrsIds()`.
        // Returning empty id lists short-circuits the per-key/per-crs `getKeyInfo`/`getCrsMaterials`
        // follow-ups, so these two responses are all the flow needs.
        TestEventType::NewKmsEpoch => {
            asserter.push_success(&Vec::<U256>::new().abi_encode());
            asserter.push_success(&Vec::<U256>::new().abi_encode());
        }
        _ => (),
    }

    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());
    info!("Gateway + Ethereum mock started!");

    // Mocking Host chain. ACL call counts per variant:
    //   - Public:            1 `isAllowedForDecryption`
    //   - Legacy user:       2 `isAllowed` (user + contract)
    //   - RFC016 user (V2):  1 `isValidSignature` (RFC-012, ERC-1271 fallback — random fixture
    //                        signature can't ecrecover to userAddress so the call always fires)
    //                        + 1 U256 (`decryptionSignatureInvalidatedBefore`) + 1 `isAllowed`
    //                        (direct ownership path, empty `allowedContracts`). All three fire
    //                        concurrently via `try_join!`, consumed FIFO in poll order thanks
    //                        to the `biased` annotation.
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
    let acl_contracts_mock = init_host_chains_acl_contracts_mock(handle, acl_responses);

    // Insert request in DB to trigger kms_worker job
    let request = insert_rand_request(test_instance.db(), event_type, insert_options).await?;

    // Mocking KMS responses
    let kms_mocks = prepare_mocks(&request, already_sent);
    let kms_mock_server =
        MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint").with_mocks(kms_mocks);
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    // Starting kms_worker
    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
        ct_attestation: testing_ct_attestation_config(),
        ..Default::default()
    };
    let kms_worker = init_kms_worker(
        config,
        mock_provider,
        acl_contracts_mock,
        test_instance.db(),
    )
    .await?;
    let cancel_token = CancellationToken::new();
    let kms_worker_task = tokio::spawn(kms_worker.start(cancel_token.clone()));
    info!("KmsWorker started!");

    // Waiting for kms_worker to process the request
    if matches!(
        event_type,
        TestEventType::AbortKeygen
            | TestEventType::AbortCrsgen
            | TestEventType::KmsContextDestroyed
            | TestEventType::KmsEpochDestroyed
    ) {
        // Abort and destruction events yield no response row, so just wait for it to be `completed`
        wait_for_no_response_event_completed(test_instance.db(), event_type).await?;
    } else {
        let response = wait_for_response_in_db(test_instance.db(), &request).await?;
        check_response_data(&request, response)?;
    }
    check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;

    // Stopping the test
    cancel_token.cancel();
    kms_worker_task.await.unwrap();
    Ok(())
}

fn prepare_mocks(req: &ProtocolEventKind, already_sent: bool) -> MockSet {
    let mut kms_mocks = MockSet::new();

    // Gets the request ID and endpoints for the given request type
    let (request_id_u256, req_endpoint, resp_endpoint) = match req {
        ProtocolEventKind::PublicDecryption(r) => {
            (r.decryptionId, "PublicDecrypt", "GetPublicDecryptionResult")
        }
        ProtocolEventKind::UserDecryption(r) => {
            (r.decryptionId, "UserDecrypt", "GetUserDecryptionResult")
        }
        ProtocolEventKind::UserDecryptionV2(r) => {
            (r.decryptionId, "UserDecrypt", "GetUserDecryptionResult")
        }
        ProtocolEventKind::PrepKeygen(r) => {
            (r.prepKeygenId, "KeyGenPreproc", "GetKeyGenPreprocResult")
        }
        ProtocolEventKind::Keygen(r) => (r.keyId, "KeyGen", "GetKeyGenResult"),
        ProtocolEventKind::Crsgen(r) => (r.crsId, "CrsGen", "GetCrsGenResult"),
        ProtocolEventKind::NewKmsContext(r) => (r.contextId, "NewMpcContext", "unreachable"),
        ProtocolEventKind::NewKmsEpoch(r) => (r.epochId, "NewMpcEpoch", "GetEpochResult"),
        ProtocolEventKind::AbortKeygen(r) => (r.prepKeygenId, "AbortKeyGen", "unreachable"),
        ProtocolEventKind::AbortCrsgen(r) => (r.crsId, "AbortCrsGen", "unreachable"),
        ProtocolEventKind::KmsContextDestroyed(r) => {
            (r.kmsContextId, "DestroyMpcContext", "unreachable")
        }
        ProtocolEventKind::KmsEpochDestroyed(r) => (r.epochId, "DestroyMpcEpoch", "unreachable"),
    };
    let request_id = Some(u256_to_request_id(request_id_u256));

    // No mock if `already_sent` to ensure this request is skipped on kms_worker side
    if !already_sent {
        // Mock initial KMS response to initial GRPC request
        kms_mocks.mock(|when, then| {
            when.path(format!(
                "/kms_service.v1.CoreServiceEndpoint/{req_endpoint}"
            ));
            then.pb(Empty::default());
        });
    }

    // Mock response of result polling
    kms_mocks.mock(|when, then| {
        when.path(format!(
            "/kms_service.v1.CoreServiceEndpoint/{resp_endpoint}"
        ));
        match req {
            ProtocolEventKind::PublicDecryption(_) => then.pb(PublicDecryptionResponse {
                payload: Some(PublicDecryptionResponsePayload::default()),
                ..Default::default()
            }),
            ProtocolEventKind::UserDecryption(_) | ProtocolEventKind::UserDecryptionV2(_) => then
                .pb(UserDecryptionResponse {
                    payload: Some(UserDecryptionResponsePayload::default()),
                    ..Default::default()
                }),
            ProtocolEventKind::PrepKeygen(_) => then.pb(KeyGenPreprocResult {
                preprocessing_id: request_id,
                ..Default::default()
            }),
            ProtocolEventKind::Keygen(_) => then.pb(KeyGenResult {
                request_id,
                ..Default::default()
            }),
            ProtocolEventKind::Crsgen(_) => then.pb(CrsGenResult {
                request_id,
                ..Default::default()
            }),
            ProtocolEventKind::NewKmsEpoch(_) => then.pb(GrpcEpochResultResponse::default()),
            ProtocolEventKind::NewKmsContext(_)
            | ProtocolEventKind::AbortKeygen(_)
            | ProtocolEventKind::AbortCrsgen(_)
            | ProtocolEventKind::KmsContextDestroyed(_)
            | ProtocolEventKind::KmsEpochDestroyed(_) => then.error(
                StatusCode::BAD_REQUEST,
                "No response expected response from kms-core",
            ),
        };
    });

    kms_mocks
}

async fn wait_for_response_in_db(
    db: &Pool<Postgres>,
    req: &ProtocolEventKind,
) -> anyhow::Result<KmsResponse> {
    info!("Waiting for response to be stored in DB...");
    let query = match req {
        ProtocolEventKind::PublicDecryption(_) => "SELECT * FROM public_decryption_responses",
        ProtocolEventKind::UserDecryption(_) | ProtocolEventKind::UserDecryptionV2(_) => {
            "SELECT * FROM user_decryption_responses"
        }
        ProtocolEventKind::PrepKeygen(_) => "SELECT * FROM prep_keygen_responses",
        ProtocolEventKind::Keygen(_) => "SELECT * FROM keygen_responses",
        ProtocolEventKind::Crsgen(_) => "SELECT * FROM crsgen_responses",
        ProtocolEventKind::NewKmsContext(_) => "SELECT * FROM new_kms_context_responses",
        ProtocolEventKind::NewKmsEpoch(_) => "SELECT * FROM epoch_result_responses",
        ProtocolEventKind::AbortKeygen(_)
        | ProtocolEventKind::AbortCrsgen(_)
        | ProtocolEventKind::KmsContextDestroyed(_)
        | ProtocolEventKind::KmsEpochDestroyed(_) => {
            unreachable!("abort and destruction events produce no response row")
        }
    };
    let response = loop {
        let result = sqlx::query(query).fetch_all(db).await?;

        if result.is_empty() {
            warn!("Response not yet stored in DB...");
            tokio::time::sleep(Duration::from_millis(200)).await;
        } else {
            match req {
                ProtocolEventKind::PublicDecryption(_) => {
                    break kms_response::from_public_decryption_row(&result[0])?;
                }
                ProtocolEventKind::UserDecryption(_) | ProtocolEventKind::UserDecryptionV2(_) => {
                    break kms_response::from_user_decryption_row(&result[0])?;
                }
                ProtocolEventKind::PrepKeygen(_) => {
                    break kms_response::from_prep_keygen_row(&result[0])?;
                }
                ProtocolEventKind::Keygen(_) => {
                    break kms_response::from_keygen_row(&result[0])?;
                }
                ProtocolEventKind::Crsgen(_) => {
                    break kms_response::from_crsgen_row(&result[0])?;
                }
                ProtocolEventKind::NewKmsContext(_) => {
                    break kms_response::from_new_kms_context_response_row(&result[0])?;
                }
                ProtocolEventKind::NewKmsEpoch(_) => {
                    break kms_response::from_epoch_result_row(&result[0])?;
                }
                ProtocolEventKind::AbortKeygen(_)
                | ProtocolEventKind::AbortCrsgen(_)
                | ProtocolEventKind::KmsContextDestroyed(_)
                | ProtocolEventKind::KmsEpochDestroyed(_) => {
                    unreachable!("abort and destruction events produce no response row")
                }
            };
        }
    };
    info!("Response successfully stored in DB!");
    Ok(response)
}

fn check_response_data(request: &ProtocolEventKind, response: KmsResponse) -> anyhow::Result<()> {
    info!("Checking response data...");
    let expected_response = match request {
        ProtocolEventKind::PublicDecryption(r) => KmsGrpcResponse::PublicDecryption {
            decryption_id: r.decryptionId,
            grpc_response: PublicDecryptionResponse {
                payload: Some(PublicDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        ProtocolEventKind::UserDecryption(r) => KmsGrpcResponse::UserDecryption {
            decryption_id: r.decryptionId,
            grpc_response: UserDecryptionResponse {
                payload: Some(UserDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        ProtocolEventKind::UserDecryptionV2(r) => KmsGrpcResponse::UserDecryption {
            decryption_id: r.decryptionId,
            grpc_response: UserDecryptionResponse {
                payload: Some(UserDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        ProtocolEventKind::PrepKeygen(r) => KmsGrpcResponse::PrepKeygen(KeyGenPreprocResult {
            preprocessing_id: Some(u256_to_request_id(r.prepKeygenId)),
            ..Default::default()
        }),
        ProtocolEventKind::Keygen(r) => KmsGrpcResponse::Keygen(KeyGenResult {
            request_id: Some(u256_to_request_id(r.keyId)),
            ..Default::default()
        }),
        ProtocolEventKind::Crsgen(r) => KmsGrpcResponse::Crsgen(CrsGenResult {
            request_id: Some(u256_to_request_id(r.crsId)),
            ..Default::default()
        }),
        ProtocolEventKind::NewKmsContext(r) => KmsGrpcResponse::NewKmsContext {
            context_id: r.contextId,
        },
        ProtocolEventKind::NewKmsEpoch(r) => KmsGrpcResponse::EpochResult {
            context_id: r.kmsContextId,
            epoch_id: r.epochId,
            grpc_response: GrpcEpochResultResponse::default(),
        },
        ProtocolEventKind::AbortKeygen(_)
        | ProtocolEventKind::AbortCrsgen(_)
        | ProtocolEventKind::KmsContextDestroyed(_)
        | ProtocolEventKind::KmsEpochDestroyed(_) => {
            unreachable!("abort and destruction events produce no response to check")
        }
    };
    assert_eq!(response.kind, KmsResponseKind::process(expected_response)?);
    info!("Response data validated!");
    Ok(())
}

async fn wait_for_no_response_event_completed(
    db: &Pool<Postgres>,
    event_type: TestEventType,
) -> anyhow::Result<()> {
    info!("Waiting for request to be marked as completed in DB...");
    let query = match event_type {
        TestEventType::AbortKeygen => {
            "SELECT COUNT(prep_keygen_id) FROM abort_keygen_requests WHERE status = 'completed'"
        }
        TestEventType::AbortCrsgen => {
            "SELECT COUNT(crs_id) FROM abort_crsgen_requests WHERE status = 'completed'"
        }
        TestEventType::KmsContextDestroyed => {
            "SELECT COUNT(context_id) FROM kms_context_destroyed WHERE status = 'completed'"
        }
        TestEventType::KmsEpochDestroyed => {
            "SELECT COUNT(epoch_id) FROM kms_epoch_destroyed WHERE status = 'completed'"
        }
        _ => unreachable!(
            "wait_for_no_response_event_completed only handles abort and destruction events"
        ),
    };
    loop {
        let count: i64 = sqlx::query_scalar(query).fetch_one(db).await?;
        if count > 0 {
            info!("Request marked as completed in DB!");
            return Ok(());
        }
        warn!("Request not yet marked as completed in DB...");
        tokio::time::sleep(Duration::from_millis(200)).await;
    }
}
