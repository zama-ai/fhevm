use alloy::{
    providers::{Provider, ProviderBuilder, mock::Asserter},
    sol_types::SolValue,
    transports::http::reqwest,
};
use connector_utils::{
    tests::{
        db::requests::{check_no_uncompleted_request_in_db, insert_rand_request},
        setup::{DbInstance, S3Instance, TestInstanceBuilder},
    },
    types::{GatewayEventKind, db::EventType},
};
use fhevm_gateway_bindings::gateway_config::GatewayConfig::Coprocessor;
use kms_grpc::kms::v1::{Empty, InitiateResharingResponse};
use kms_worker::core::{
    Config, DbEventPicker, DbKmsResponsePublisher, KmsWorker,
    event_processor::{
        DbEventProcessor, DecryptionProcessor, KMSGenerationProcessor, KmsClient, s3::S3Service,
    },
};
use mocktail::{MockSet, StatusCode, server::MockServer};
use rstest::rstest;
use sqlx::{Pool, Postgres};
use std::time::Duration;
use tokio_util::sync::CancellationToken;
use tracing::{info, warn};

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_public_decryption_removal_after_max_attempt_reached() -> anyhow::Result<()> {
    test_processing_request(EventType::PublicDecryptionRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_user_decryption_removal_after_max_attempt_reached() -> anyhow::Result<()> {
    test_processing_request(EventType::UserDecryptionRequest).await
}
#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_prep_keygen_processing_not_removed_on_error() -> anyhow::Result<()> {
    test_processing_request(EventType::PrepKeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_keygen_processing_not_removed_on_error() -> anyhow::Result<()> {
    test_processing_request(EventType::KeygenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_crsgen_processing_not_removed_on_error() -> anyhow::Result<()> {
    test_processing_request(EventType::CrsgenRequest).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_prss_init_processing_removal_on_error() -> anyhow::Result<()> {
    test_processing_request(EventType::PrssInit).await
}

#[rstest]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn test_key_reshare_same_set_processing_removal_on_error() -> anyhow::Result<()> {
    test_processing_request(EventType::KeyReshareSameSet).await
}

async fn test_processing_request(event_type: EventType) -> anyhow::Result<()> {
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
    if matches!(event_type, EventType::PublicDecryptionRequest) {
        let is_decryption_done_call_response = false;
        asserter.push_success(&is_decryption_done_call_response.abi_encode());
    }

    let mock_provider = ProviderBuilder::new()
        .disable_recommended_fillers()
        .connect_mocked_client(asserter.clone());
    info!("Gateway mock started!");

    // Insert request in DB to trigger kms_worker job
    let request = insert_rand_request(test_instance.db(), event_type, None, false, None).await?;

    // Mocking KMS responses
    let kms_mocks = prepare_mocks(&request);
    let kms_mock_server =
        MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint").with_mocks(kms_mocks);
    kms_mock_server.start().await?;
    info!("KMS mock server started!");

    // Starting kms_worker
    let config = Config {
        kms_core_endpoints: vec![kms_mock_server.base_url().unwrap().to_string()],
        max_decryption_attempts: 3,
        grpc_request_retries: 2,
        db_fast_event_polling: Duration::from_millis(500),
        db_long_event_polling: Duration::from_millis(500),
        ..Default::default()
    };
    let kms_worker = init_kms_worker(config, mock_provider, test_instance.db()).await?;
    let cancel_token = CancellationToken::new();
    let kms_worker_task = tokio::spawn(kms_worker.start(cancel_token.clone()));
    info!("KmsWorker started!");

    match &request {
        // Wait for kms_worker to remove the request from DB, then stop it
        GatewayEventKind::PublicDecryption(_)
        | GatewayEventKind::UserDecryption(_)
        | GatewayEventKind::PrssInit(_)
        | GatewayEventKind::KeyReshareSameSet(_) => {
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

fn prepare_mocks(req: &GatewayEventKind) -> MockSet {
    let mut kms_mocks = MockSet::new();

    // Gets the endpoints for the given request type
    let (req_endpoint, resp_endpoint) = match req {
        GatewayEventKind::PublicDecryption(_) => ("PublicDecrypt", "GetPublicDecryptionResult"),
        GatewayEventKind::UserDecryption(_) => ("UserDecrypt", "GetUserDecryptionResult"),
        GatewayEventKind::PrepKeygen(_) => ("KeyGenPreproc", "GetKeyGenPreprocResult"),
        GatewayEventKind::Keygen(_) => ("KeyGen", "GetKeyGenResult"),
        GatewayEventKind::Crsgen(_) => ("CrsGen", "GetCrsGenResult"),
        GatewayEventKind::PrssInit(_) => ("Init", ""),
        GatewayEventKind::KeyReshareSameSet(_) => ("InitiateResharing", ""),
    };

    // Mock initial KMS response to initial GRPC request
    kms_mocks.mock(|when, then| {
        when.path(format!(
            "/kms_service.v1.CoreServiceEndpoint/{req_endpoint}"
        ));
        match req {
            GatewayEventKind::KeyReshareSameSet(_) => then.pb(InitiateResharingResponse::default()),
            // KMS returns `Empty` for all kind of requests except `KeyReshareSameSet`
            _ => then.pb(Empty::default()),
        };
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

async fn init_kms_worker<P: Provider + Clone + 'static>(
    config: Config,
    provider: P,
    db: &Pool<Postgres>,
) -> anyhow::Result<KmsWorker<DbEventPicker, DbEventProcessor<P>>> {
    let kms_client = KmsClient::connect(&config).await?;
    let s3_client = reqwest::Client::new();
    let event_picker = DbEventPicker::connect(db.clone(), &config).await?;

    let s3_service = S3Service::new(&config, provider.clone(), s3_client);
    let decryption_processor = DecryptionProcessor::new(&config, provider.clone(), s3_service);
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
