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
        rand::{rand_digest, rand_sns_ct},
        setup::{
            DbInstance, S3_CT_DIGEST, S3_CT_HANDLE, S3_CT_KEY_ID, S3Instance, TestInstanceBuilder,
            erc1271_magic_response, init_host_chains_acl_contracts_mock,
        },
    },
    types::{
        KmsGrpcResponse, KmsResponse, KmsResponseKind, ProtocolEventKind, kms_response,
        u256_to_request_id,
    },
};
use fhevm_host_bindings::protocol_config::ProtocolConfig::NewKmsContext;
use kms_grpc::kms::v1::{
    CrsGenResult, Empty, EpochResultResponse as GrpcEpochResultResponse,
    KeyDigest as GrpcKeyDigest, KeyGenPreprocResult, KeyGenResult, PublicDecryptionResponse,
    PublicDecryptionResponsePayload, UserDecryptionResponse, UserDecryptionResponsePayload,
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
#[case::compressed_key_migration(TestEventType::CompressedKeyMigrationKeygen, false)]
#[case::crsgen(TestEventType::Crsgen, false)]
#[case::new_kms_context(TestEventType::NewKmsContext, false)]
#[case::new_kms_epoch(TestEventType::NewKmsEpoch, false)]
#[case::public_decryption_already_sent(TestEventType::PublicDecryption, true)]
#[case::user_decryption_already_sent(TestEventType::UserDecryption, true)]
#[case::user_decryption_v2_already_sent(TestEventType::UserDecryptionV2, true)]
#[case::prep_keygen_already_sent(TestEventType::PrepKeygen, true)]
#[case::keygen_already_sent(TestEventType::Keygen, true)]
#[case::crsgen_already_sent(TestEventType::Crsgen, true)]
#[case::new_kms_context_already_sent(TestEventType::NewKmsContext, true)]
#[case::new_kms_epoch_already_sent(TestEventType::NewKmsEpoch, true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_processing_request(
    #[case] event_type: TestEventType,
    #[case] already_sent: bool,
) -> anyhow::Result<()> {
    // Setup real DB and S3 instance
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup().await?)
        .with_s3(S3Instance::setup().await?)
        .build();

    // Mocking Gateway/Ethereum
    let asserter = Asserter::new();
    let copro_tx_sender = mock_copro_registry_load(&asserter, test_instance.s3_url());
    let mut sns_ct = rand_sns_ct();
    sns_ct.keyId = S3_CT_KEY_ID;
    sns_ct.ctHandle = FixedBytes::<32>::from_hex(S3_CT_HANDLE)?;
    sns_ct.snsCiphertextDigest = FixedBytes::<32>::from_hex(S3_CT_DIGEST)?;
    sns_ct.coprocessorTxSenderAddresses = vec![copro_tx_sender];

    let mut insert_options = InsertRequestOptions::new()
        .with_already_sent(already_sent)
        .with_sns_ct_materials(vec![sns_ct.clone()]);

    match event_type {
        // Only the legacy `UserDecryptionRequest` path re-fetches calldata via `get_transaction_by_hash`
        // — the RFC016 V2 event carries the full payload in-event, so no such mock is needed.
        TestEventType::UserDecryption => {
            let tx_hash = rand_digest();
            let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
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
    let acl_contracts_mock =
        init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), acl_responses);

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
        ct_attestation: testing_ct_attestation_config(true),
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
    let response = wait_for_response_in_db(test_instance.db(), &request).await?;
    check_response_data(&request, response)?;
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
        ProtocolEventKind::CompressedKeyMigrationKeygen(r) => {
            (r.migrationRequestId, "KeyGen", "GetKeyGenResult")
        }
        ProtocolEventKind::Crsgen(r) => (r.crsId, "CrsGen", "GetCrsGenResult"),
        ProtocolEventKind::NewKmsContext(r) => (r.contextId, "NewMpcContext", "unreachable"),
        ProtocolEventKind::NewKmsEpoch(r) => (r.epochId, "NewMpcEpoch", "GetEpochResult"),
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
            // Lock in the real digest type the migration keygen returns:
            // the connector must parse "CompressedXofKeySet".
            ProtocolEventKind::CompressedKeyMigrationKeygen(_) => then.pb(KeyGenResult {
                request_id,
                key_digests: vec![GrpcKeyDigest {
                    key_type: "CompressedXofKeySet".to_string(),
                    digest: vec![0xC0, 0xFF, 0xEE],
                }],
                ..Default::default()
            }),
            ProtocolEventKind::Crsgen(_) => then.pb(CrsGenResult {
                request_id,
                ..Default::default()
            }),
            ProtocolEventKind::NewKmsContext(_) => then.error(
                StatusCode::BAD_REQUEST,
                "No response expected response from kms-core",
            ),
            ProtocolEventKind::NewKmsEpoch(_) => then.pb(GrpcEpochResultResponse::default()),
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
        ProtocolEventKind::Keygen(_) | ProtocolEventKind::CompressedKeyMigrationKeygen(_) => {
            "SELECT * FROM keygen_responses"
        }
        ProtocolEventKind::Crsgen(_) => "SELECT * FROM crsgen_responses",
        ProtocolEventKind::NewKmsContext(_) => "SELECT * FROM new_kms_context_responses",
        ProtocolEventKind::NewKmsEpoch(_) => "SELECT * FROM epoch_result_responses",
    };
    let response = loop {
        let result = sqlx::query(query).fetch_all(db).await?;

        if result.is_empty() {
            warn!("Not yet...");
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
                ProtocolEventKind::Keygen(_)
                | ProtocolEventKind::CompressedKeyMigrationKeygen(_) => {
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
            };
        }
    };
    info!("OK!");
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
        ProtocolEventKind::CompressedKeyMigrationKeygen(r) => {
            KmsGrpcResponse::Keygen(KeyGenResult {
                request_id: Some(u256_to_request_id(r.migrationRequestId)),
                key_digests: vec![GrpcKeyDigest {
                    key_type: "CompressedXofKeySet".to_string(),
                    digest: vec![0xC0, 0xFF, 0xEE],
                }],
                ..Default::default()
            })
        }
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
    };
    let mut expected_kind = KmsResponseKind::process(expected_response)?;
    // The gRPC layer is migration-agnostic; the kind is re-typed from the request row when
    // read back from the DB (RFC-029).
    if matches!(request, ProtocolEventKind::CompressedKeyMigrationKeygen(_))
        && let KmsResponseKind::Keygen(r) = expected_kind
    {
        expected_kind = KmsResponseKind::CompressedKeyMigration(r);
    }
    assert_eq!(response.kind, expected_kind);
    info!("OK!");
    Ok(())
}
