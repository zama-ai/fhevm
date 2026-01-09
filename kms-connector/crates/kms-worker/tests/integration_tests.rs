use alloy::{
    hex,
    primitives::U256,
    providers::{Provider, ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
    transports::http::reqwest,
};
use connector_utils::{
    tests::{
        db::requests::{check_no_uncompleted_request_in_db, insert_rand_request},
        setup::{DbInstance, S3Instance, TestInstanceBuilder},
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
use kms_worker::core::{
    Config, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{
        DbEventProcessor, DecryptionProcessor, KMSGenerationProcessor, KmsClient, s3::S3Service,
    },
};
use mocktail::{MockSet, server::MockServer};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_public_decryption_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::PublicDecryptionRequest, false).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_user_decryption_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::UserDecryptionRequest, false).await
}
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_prep_keygen_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::PrepKeygenRequest, false).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_keygen_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::KeygenRequest, false).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_crsgen_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::CrsgenRequest, false).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_public_decryption_processing_already_sent() -> anyhow::Result<()> {
    test_processing_request(EventType::PublicDecryptionRequest, true).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_user_decryption_processing_already_sent() -> anyhow::Result<()> {
    test_processing_request(EventType::UserDecryptionRequest, true).await
}
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_prep_keygen_processing_already_sent() -> anyhow::Result<()> {
    test_processing_request(EventType::PrepKeygenRequest, true).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_keygen_processing_already_sent() -> anyhow::Result<()> {
    test_processing_request(EventType::KeygenRequest, true).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_crsgen_processing_already_sent() -> anyhow::Result<()> {
    test_processing_request(EventType::CrsgenRequest, true).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_prss_init_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::PrssInit, false).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_key_reshare_same_set_processing() -> anyhow::Result<()> {
    test_processing_request(EventType::KeyReshareSameSet, false).await
}

async fn test_processing_request(event_type: EventType, already_sent: bool) -> anyhow::Result<()> {
    // Setup real DB and S3 instance
    let test_instance = TestInstanceBuilder::default()
        .with_db(DbInstance::setup().await?)
        .with_s3(S3Instance::setup().await?)
        .build();

    // Mocking Gateway
    let asserter = Asserter::new();

    if matches!(event_type, EventType::PublicDecryptionRequest) {
        let is_decryption_done_call_response = false;
        asserter.push_success(&is_decryption_done_call_response.abi_encode());
    }

    let get_copro_call_response = Coprocessor {
        s3BucketUrl: format!("{}/ct128", test_instance.s3_url()),
        ..Default::default()
    };
    asserter.push_success(&get_copro_call_response.abi_encode());
    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());
    info!("Gateway mock started!");

    // Insert request in DB to trigger kms_worker job
    let request =
        insert_rand_request(test_instance.db(), event_type, None, already_sent, None).await?;

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
    let kms_worker = init_kms_worker(config, mock_provider, test_instance.db()).await?;
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
            (r.request_id, "PublicDecrypt", "GetPublicDecryptionResult")
        }
        GatewayEventKind::UserDecryption(r) => {
            (r.request_id, "UserDecrypt", "GetUserDecryptionResult")
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
            decryption_id: r.request_id,
            grpc_response: PublicDecryptionResponse {
                payload: Some(PublicDecryptionResponsePayload::default()),
                ..Default::default()
            },
        },
        GatewayEventKind::UserDecryption(r) => KmsGrpcResponse::UserDecryption {
            decryption_id: r.request_id,
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

async fn init_kms_worker<P: Provider + Clone + 'static>(
    config: Config,
    provider: P,
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsWorker<DbEventPicker, DbEventProcessor<P>>> {
    let kms_client = KmsClient::connect(&config).await?;
    let s3_client = reqwest::Client::new();
    let event_picker = DbEventPicker::connect(db.clone(), &config).await?;

    let _s3_service = S3Service::new(&config, provider.clone(), s3_client);
    let decryption_processor = DecryptionProcessor::new(&config, provider.clone());
    let kms_generation_processor = KMSGenerationProcessor::new(&config);
    let event_processor = DbEventProcessor::new(
        kms_client.clone(),
        decryption_processor,
        kms_generation_processor,
        config.max_decryption_attempts,
        db.clone(),
    );
    let response_publisher = DbKmsResponsePublisher::new(db.clone());
    let kms_worker = KmsWorker::new(event_picker, event_processor, response_publisher);
    Ok(kms_worker)
}
