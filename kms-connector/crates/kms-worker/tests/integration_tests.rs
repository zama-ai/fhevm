mod common;

use crate::common::{create_mock_user_decryption_request_tx, init_kms_worker};
use alloy::{
    hex::{self, FromHex},
    primitives::{FixedBytes, U256},
    providers::{ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
};
use connector_utils::{
    tests::{
        db::requests::{
            InsertRequestOptions, check_no_uncompleted_request_in_db, insert_rand_request,
        },
        rand::{rand_digest, rand_sns_ct},
        setup::{
            DbInstance, S3_CT_DIGEST, S3_CT_HANDLE, S3Instance, TestInstanceBuilder,
            init_host_chains_acl_contracts_mock,
        },
    },
    types::{
        GatewayEventKind, KmsGrpcResponse, KmsResponse, KmsResponseKind, db::EventType,
        kms_response,
    },
};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::Coprocessor;
use kms_grpc::kms::v1::{
    CrsGenResult, Empty, InitiateResharingResponse, KeyGenPreprocResult, KeyGenResult,
    PublicDecryptionResponse, PublicDecryptionResponsePayload, RequestId, UserDecryptionResponse,
    UserDecryptionResponsePayload,
};
use kms_worker::core::Config;
use mocktail::{MockSet, server::MockServer};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::{collections::HashMap, time::Duration};
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[case::public_decryption(EventType::PublicDecryptionRequest, false)]
#[case::user_decryption(EventType::UserDecryptionRequest, false)]
#[case::prep_keygen(EventType::PrepKeygenRequest, false)]
#[case::keygen(EventType::KeygenRequest, false)]
#[case::crsgen(EventType::CrsgenRequest, false)]
#[case::prss_init(EventType::PrssInit, false)]
#[case::key_reshare_same_set(EventType::KeyReshareSameSet, false)]
#[case::public_decryption_already_sent(EventType::PublicDecryptionRequest, true)]
#[case::user_decryption_already_sent(EventType::UserDecryptionRequest, true)]
#[case::prep_keygen_already_sent(EventType::PrepKeygenRequest, true)]
#[case::keygen_already_sent(EventType::KeygenRequest, true)]
#[case::crsgen_already_sent(EventType::CrsgenRequest, true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_processing_request(
    #[case] event_type: EventType,
    #[case] already_sent: bool,
) -> anyhow::Result<()> {
    // Setup real DB and S3 instance
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup().await?)
        .with_s3(S3Instance::setup().await?)
        .build();

    // Mocking Gateway
    let asserter = Asserter::new();
    let mut sns_ct = rand_sns_ct();
    sns_ct.ctHandle = FixedBytes::<32>::from_hex(S3_CT_HANDLE)?;
    sns_ct.snsCiphertextDigest = FixedBytes::<32>::from_hex(S3_CT_DIGEST)?;
    let mut insert_options = InsertRequestOptions::new()
        .with_already_sent(already_sent)
        .with_sns_ct_materials(vec![sns_ct.clone()]);
    if matches!(event_type, EventType::UserDecryptionRequest) {
        // Mocking `get_transaction_by_hash` call result
        let tx_hash = rand_digest();
        let mock_tx = create_mock_user_decryption_request_tx(tx_hash, sns_ct.ctHandle)?;
        insert_options = insert_options.with_tx_hash(tx_hash);
        asserter.push_success(&mock_tx);
    }

    let get_copro_call_response = Coprocessor {
        s3BucketUrl: format!("{}/ct128", test_instance.s3_url()),
        ..Default::default()
    };
    asserter.push_success(&get_copro_call_response.abi_encode());
    let gateway_mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());
    info!("Gateway mock started!");

    // Mocking Host chain
    let acl_contracts_mock = match &event_type {
        EventType::PublicDecryptionRequest => {
            init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), vec![true])
        }
        EventType::UserDecryptionRequest => {
            init_host_chains_acl_contracts_mock(sns_ct.ctHandle.as_slice(), vec![true, true])
        }
        _ => HashMap::new(),
    };

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

    // Waiting for kms_worker to process the request
    match &request {
        GatewayEventKind::PrssInit(_) | GatewayEventKind::KeyReshareSameSet(_) => {
            while let Err(e) =
                check_no_uncompleted_request_in_db(test_instance.db(), event_type).await
            {
                warn!("Still requests in DB: {e}");
                tokio::time::sleep(Duration::from_millis(200)).await;
            }
        }
        _ => {
            let response = wait_for_response_in_db(test_instance.db(), &request).await?;
            check_response_data(&request, response)?;
            check_no_uncompleted_request_in_db(test_instance.db(), event_type).await?;
        }
    }

    // Stopping the test
    cancel_token.cancel();
    kms_worker_task.await.unwrap();
    Ok(())
}

fn prepare_mocks(req: &GatewayEventKind, already_sent: bool) -> MockSet {
    let mut kms_mocks = MockSet::new();

    // Gets the request ID and endpoints for the given request type
    let (request_id_u256, req_endpoint, resp_endpoint) = match req {
        GatewayEventKind::PublicDecryption(r) => {
            (r.decryptionId, "PublicDecrypt", "GetPublicDecryptionResult")
        }
        GatewayEventKind::UserDecryption(r) => {
            (r.decryptionId, "UserDecrypt", "GetUserDecryptionResult")
        }
        GatewayEventKind::PrepKeygen(r) => {
            (r.prepKeygenId, "KeyGenPreproc", "GetKeyGenPreprocResult")
        }
        GatewayEventKind::Keygen(r) => (r.keyId, "KeyGen", "GetKeyGenResult"),
        GatewayEventKind::Crsgen(r) => (r.crsId, "CrsGen", "GetCrsGenResult"),
        GatewayEventKind::PrssInit(id) => (*id, "Init", ""),
        GatewayEventKind::KeyReshareSameSet(r) => (r.keyId, "InitiateResharing", ""),
    };
    let request_id = u256_to_request_id(request_id_u256);

    // No mock if `already_sent` to ensure this request is skipped on kms_worker side
    if !already_sent {
        // Mock initial KMS response to initial GRPC request
        kms_mocks.mock(|when, then| {
            when.path(format!(
                "/kms_service.v1.CoreServiceEndpoint/{req_endpoint}"
            ));
            match req {
                GatewayEventKind::KeyReshareSameSet(_) => {
                    then.pb(InitiateResharingResponse::default())
                }
                // KMS returns `Empty` for all kind of requests except `KeyReshareSameSet`
                _ => then.pb(Empty::default()),
            };
        });
    }

    // Mock response of result polling
    kms_mocks.mock(|when, then| {
        when.path(format!(
            "/kms_service.v1.CoreServiceEndpoint/{resp_endpoint}"
        ));
        match req {
            GatewayEventKind::PublicDecryption(_) => then.pb(PublicDecryptionResponse {
                payload: Some(PublicDecryptionResponsePayload::default()),
                ..Default::default()
            }),
            GatewayEventKind::UserDecryption(_) => then.pb(UserDecryptionResponse {
                payload: Some(UserDecryptionResponsePayload::default()),
                ..Default::default()
            }),
            GatewayEventKind::PrepKeygen(_) => then.pb(KeyGenPreprocResult {
                preprocessing_id: request_id,
                ..Default::default()
            }),
            GatewayEventKind::Keygen(_) => then.pb(KeyGenResult {
                request_id,
                ..Default::default()
            }),
            GatewayEventKind::Crsgen(_) => then.pb(CrsGenResult {
                request_id,
                ..Default::default()
            }),
            _ => then.pb(Empty::default()),
        };
    });

    kms_mocks
}

async fn wait_for_response_in_db(
    db: &Pool<Postgres>,
    req: &GatewayEventKind,
) -> anyhow::Result<KmsResponse> {
    info!("Waiting for response to be stored in DB...");
    let query = match req {
        GatewayEventKind::PublicDecryption(_) => "SELECT * FROM public_decryption_responses",
        GatewayEventKind::UserDecryption(_) => "SELECT * FROM user_decryption_responses",
        GatewayEventKind::PrepKeygen(_) => "SELECT * FROM prep_keygen_responses",
        GatewayEventKind::Keygen(_) => "SELECT * FROM keygen_responses",
        GatewayEventKind::Crsgen(_) => "SELECT * FROM crsgen_responses",
        _ => unimplemented!(),
    };
    let response = loop {
        let result = sqlx::query(query).fetch_all(db).await?;

        if result.is_empty() {
            warn!("Not yet...");
            tokio::time::sleep(Duration::from_millis(200)).await;
        } else {
            match req {
                GatewayEventKind::PublicDecryption(_) => {
                    break kms_response::from_public_decryption_row(&result[0])?;
                }
                GatewayEventKind::UserDecryption(_) => {
                    break kms_response::from_user_decryption_row(&result[0])?;
                }
                GatewayEventKind::PrepKeygen(_) => {
                    break kms_response::from_prep_keygen_row(&result[0])?;
                }
                GatewayEventKind::Keygen(_) => {
                    break kms_response::from_keygen_row(&result[0])?;
                }
                GatewayEventKind::Crsgen(_) => {
                    break kms_response::from_crsgen_row(&result[0])?;
                }
                _ => unimplemented!(),
            };
        }
    };
    info!("OK!");
    Ok(response)
}

fn check_response_data(request: &GatewayEventKind, response: KmsResponse) -> anyhow::Result<()> {
    info!("Checking response data...");
    let expected_response = match request {
        GatewayEventKind::PublicDecryption(r) => KmsGrpcResponse::PublicDecryption {
            decryption_id: r.decryptionId,
            grpc_response: PublicDecryptionResponse {
                payload: Some(PublicDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        GatewayEventKind::UserDecryption(r) => KmsGrpcResponse::UserDecryption {
            decryption_id: r.decryptionId,
            grpc_response: UserDecryptionResponse {
                payload: Some(UserDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        GatewayEventKind::PrepKeygen(r) => KmsGrpcResponse::PrepKeygen(KeyGenPreprocResult {
            preprocessing_id: u256_to_request_id(r.prepKeygenId),
            ..Default::default()
        }),
        GatewayEventKind::Keygen(r) => KmsGrpcResponse::Keygen(KeyGenResult {
            request_id: u256_to_request_id(r.keyId),
            ..Default::default()
        }),
        GatewayEventKind::Crsgen(r) => KmsGrpcResponse::Crsgen(CrsGenResult {
            request_id: u256_to_request_id(r.crsId),
            ..Default::default()
        }),
        _ => unimplemented!(),
    };
    assert_eq!(response.kind, KmsResponseKind::process(expected_response)?);
    info!("OK!");
    Ok(())
}

fn u256_to_request_id(value: U256) -> Option<RequestId> {
    Some(RequestId {
        request_id: hex::encode(value.to_be_bytes::<32>()),
    })
}
