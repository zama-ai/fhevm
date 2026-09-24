//! A Solana user decryption as the worker runs it: the Gateway event, the listener's conversion,
//! and the worker's checks against a host behind real HTTP.

// Only the Gateway registry mock is used here.
#[allow(dead_code)]
mod common;
mod solana_support;

use alloy::{
    hex::FromHex,
    primitives::{B256, U256},
    providers::{ProviderBuilder, RootProvider, mock::Asserter},
};
use common::{TEST_COPRO_REGISTRY_REFRESH, mock_copro_registry_load};
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::{
        rand::solana_user_decryption_event_for,
        setup::{S3_CT_HANDLE, S3Instance},
    },
    types::{
        KmsResponseKind, ProtocolEvent, ProtocolEventKind,
        db::RequestSource,
        extra_data::{ExtraData, parse_extra_data},
        handle::extract_chain_id_from_handle,
        solana_request::SolanaUserDecryptionRequestV1,
    },
};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_4;
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    Empty, SigningMetadata, SigningSchemeType, TypedSignature, UserDecryptionRequest,
    UserDecryptionResponse, UserDecryptionResponsePayload,
};
use kms_worker::core::{
    Config,
    event_processor::{
        CiphertextManager, ContextManager, DbEventProcessor, DecryptionProcessor, EventProcessor,
        HostChainAclBackend, KMSGenerationProcessor, KmsClient, ProcessingErrorKind,
        ProtocolConfigProcessor, RequestCheckError, RequestCheckKind,
    },
    solana::{failure::AuthorizationFailure, pause::PauseFailure},
};
use mocktail::{Request, matchers::Matcher, server::MockServer};
use prost::Message;
use rstest::rstest;
use solana_support::*;
use sqlx::{postgres::PgPoolOptions, types::chrono::Utc};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio_util::sync::CancellationToken;
use zama_solana_permit::{Identity, KmsRouting, TRANSPORT_KEY_LEN};

/// A Gateway registry with no ciphertext bucket, for requests that never fetch a ciphertext.
const NO_BUCKET: &str = "http://unused-bucket-url";

/// A victim's request as the Gateway emits it, and a host whose state authorizes it: one handle
/// the victim was allowed on directly.
struct Scenario {
    victim: Wallet,
    encrypted_store: EncryptedStoreFixture,
    event: UserDecryptionRequest_4,
    host: HttpHost,
    chain_id: u64,
}

impl Scenario {
    async fn new() -> Self {
        Self::naming(handle(0x10, FHE_TYPE_UINT64)).await
    }

    /// The request for `live`, signed for the chain the handle names.
    async fn naming(live: [u8; 32]) -> Self {
        let victim = Wallet::new(1);
        let chain_id = extract_chain_id_from_handle(&B256::from(live)).unwrap();
        let encrypted_store = EncryptedStoreFixture::allowing(live, victim.pubkey());
        let now = Utc::now().timestamp() as u64;
        let request = RequestBuilder::new(&victim)
            .permit(
                PermitBuilder::new(victim.pubkey())
                    .chain_id(chain_id)
                    .window(now - 60, 3_600),
            )
            .direct(&encrypted_store, live)
            .wire();
        let query = encrypted_store.allowed_query(live, victim.pubkey());

        let mut scenario = Self {
            event: solana_user_decryption_event_for(U256::ONE, &request),
            host: HttpHost::start().await,
            victim,
            encrypted_store,
            chain_id,
        };
        scenario.serve_host(false);
        scenario
            .host
            .serve_proofs(&[(query, scenario.encrypted_store.outcome(&query))]);
        scenario
    }

    /// The first read of a direct request: the pause switch, the signer's invalidation record
    /// (absent), and the entry's encrypted store.
    fn serve_host(&mut self, paused: bool) {
        self.host.serve_accounts(&[
            (host_config_address().0, Some(host_config_account(paused))),
            (invalidation_address(self.victim.pubkey()).0, None),
            (
                self.encrypted_store.account_key,
                Some(self.encrypted_store.account()),
            ),
        ]);
    }

    async fn processor(&self) -> DecryptionProcessor<RootProvider, RootProvider> {
        self.processor_reading(&config(), NO_BUCKET).await
    }

    /// A processor whose Gateway registry points at the ciphertext bucket `bucket_url`.
    async fn processor_reading(
        &self,
        config: &Config,
        bucket_url: &str,
    ) -> DecryptionProcessor<RootProvider, RootProvider> {
        let asserter = Asserter::new();
        mock_copro_registry_load(&asserter, bucket_url);
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let ciphertext_manager =
            CiphertextManager::connect(provider.clone(), config, CancellationToken::new())
                .await
                .unwrap();
        let backends = HashMap::from([(
            self.chain_id,
            HostChainAclBackend::Solana(Box::new(self.host.host())),
        )]);
        DecryptionProcessor::new(config, provider, backends, ciphertext_manager)
    }

    /// A worker whose KMS client has no channel, so a request that got past its checks would fail
    /// another way.
    async fn worker<C: ContextManager>(
        &self,
        context_manager: C,
    ) -> DbEventProcessor<RootProvider, RootProvider, C> {
        self.worker_against(
            context_manager,
            KmsClient::new(vec![], 0),
            &config(),
            NO_BUCKET,
        )
        .await
    }

    /// A worker sending to `kms` and reading ciphertexts from `bucket_url`. Its database is
    /// unreachable: a user decryption does not touch it while processing.
    async fn worker_against<C: ContextManager>(
        &self,
        context_manager: C,
        kms: KmsClient,
        config: &Config,
        bucket_url: &str,
    ) -> DbEventProcessor<RootProvider, RootProvider, C> {
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        DbEventProcessor::new(
            kms,
            context_manager,
            self.processor_reading(config, bucket_url).await,
            KMSGenerationProcessor::new(config),
            ProtocolConfigProcessor::new(config, provider),
            PgPoolOptions::new()
                .connect_lazy("postgresql://unused/unused")
                .unwrap(),
        )
    }
}

fn config() -> Config {
    Config {
        copro_registry_refresh: TEST_COPRO_REGISTRY_REFRESH,
        ..Config::default()
    }
}

fn protocol_event(request: SolanaUserDecryptionRequestV1) -> ProtocolEvent {
    ProtocolEvent::new(
        ProtocolEventKind::SolanaUserDecryptionV1(request),
        None,
        PropagationContext::default(),
        RequestSource::Http,
    )
}

struct ValidContext;

impl ContextManager for ValidContext {
    async fn validate_context(&self, _extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        Ok(())
    }
}

/// Rejects every KMS context as destroyed and records the routing it was asked about.
struct DestroyedContext(Arc<Mutex<Vec<ExtraData>>>);

impl ContextManager for DestroyedContext {
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        self.0.lock().unwrap().push(extra_data.clone());
        Err(RequestCheckError::irrecoverable(
            RequestCheckKind::KmsContext,
            ErrorCode::KmsContextDestroyed,
            anyhow::anyhow!("destroyed context"),
        ))
    }
}

/// The relayer sets the event's key, window and routing, and those are the values stored, so
/// changing one yields a permit the user never signed.
#[rstest]
#[case::transport_key(|event: &mut UserDecryptionRequest_4| {
    event.publicKey = vec![0x5a; TRANSPORT_KEY_LEN].into();
})]
#[case::window(|event: &mut UserDecryptionRequest_4| {
    event.requestValidity.startTimestamp -= U256::from(60);
})]
#[case::kms_routing(|event: &mut UserDecryptionRequest_4| {
    event.extraData = KmsRouting::ContextAndEpoch {
        kms_context_id: Identity::new([0x13; 32]),
        kms_epoch_id: Identity::new([0x14; 32]),
    }
    .to_extra_data()
    .into();
})]
#[tokio::test]
async fn a_field_the_relayer_changed_fails_the_signature(
    #[case] relayer: fn(&mut UserDecryptionRequest_4),
) {
    let scenario = Scenario::new().await;
    let mut event = scenario.event.clone();
    relayer(&mut event);
    let request = SolanaUserDecryptionRequestV1::try_from(event).unwrap();

    let error = scenario
        .processor()
        .await
        .check_solana_user_decryption_request(&request)
        .await
        .unwrap_err()
        .record();

    assert_eq!(error.kind, ProcessingErrorKind::Irrecoverable);
    assert_eq!(error.code, ErrorCode::UserSignatureRejected);
    assert!(matches!(
        error.source.downcast_ref::<AuthorizationFailure>(),
        Some(AuthorizationFailure::Signature(_))
    ));
}

/// A poll of a request already sent to the KMS authorizes it again, so a host paused after the
/// send stops it.
#[tokio::test]
async fn a_poll_rechecks_the_host() {
    let mut scenario = Scenario::new().await;
    let request = SolanaUserDecryptionRequestV1::try_from(scenario.event.clone()).unwrap();
    scenario
        .processor()
        .await
        .check_solana_user_decryption_request(&request)
        .await
        .expect("the victim's permit authorizes the request");

    scenario.serve_host(true);
    let mut event = protocol_event(request);
    event.already_sent = true;
    let error = scenario
        .worker(ValidContext)
        .await
        .process(&mut event)
        .await
        .expect_err("the poll must reject the newly paused host");

    assert!(matches!(
        error.source.downcast_ref::<AuthorizationFailure>(),
        Some(AuthorizationFailure::Pause(PauseFailure::Paused))
    ));
    assert!(event.already_sent);
}

/// The KMS context is checked like every other decryption's, against the signed routing.
#[tokio::test]
async fn a_solana_request_checks_its_signed_kms_context() {
    let scenario = Scenario::new().await;
    let request = SolanaUserDecryptionRequestV1::try_from(scenario.event.clone()).unwrap();
    let asked = Arc::default();

    let error = scenario
        .worker(DestroyedContext(Arc::clone(&asked)))
        .await
        .process(&mut protocol_event(request.clone()))
        .await
        .expect_err("a destroyed context stops the request");

    assert_eq!(error.kind, ProcessingErrorKind::Irrecoverable);
    assert_eq!(error.code, ErrorCode::KmsContextDestroyed);
    assert_eq!(
        *asked.lock().unwrap(),
        vec![parse_extra_data(&request.extra_data()).unwrap()]
    );
}

/// The identity a `UserDecrypt` call must carry for a Solana user.
#[derive(Debug, PartialEq, PartialOrd)]
struct SolanaIdentity {
    user_pubkey: Vec<u8>,
    verifying_program_id: Vec<u8>,
    transport_key: Vec<u8>,
}

impl Matcher for SolanaIdentity {
    fn name(&self) -> &str {
        "Solana user identity"
    }

    fn matches(&self, request: &Request) -> bool {
        let body: Vec<u8> = request.body.iter().flatten().copied().collect();
        // A gRPC message follows a five-byte frame header.
        let Some(Ok(request)) = body.get(5..).map(UserDecryptionRequest::decode) else {
            return false;
        };
        request.client_address.is_empty()
            && request.enc_key == self.transport_key
            && request.signing_metadata
                == [SigningMetadata::solana(
                    self.user_pubkey.clone(),
                    self.verifying_program_id.clone(),
                )]
    }
}

/// An authorized request reaches the KMS as its Solana signer: no EVM address, the signer and the
/// host program in the signing metadata, and the transport key the Gateway emitted. A poll of a
/// request already sent authorizes it again and fetches the result without sending it twice.
#[rstest]
#[case::send(false)]
#[case::poll(true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn an_authorized_request_reaches_the_kms_as_its_signer(#[case] already_sent: bool) {
    let bucket = S3Instance::setup().await.unwrap();
    let scenario = Scenario::naming(B256::from_hex(S3_CT_HANDLE).unwrap().0).await;
    let identity = SolanaIdentity {
        user_pubkey: scenario.victim.pubkey().to_vec(),
        verifying_program_id: PROGRAM_ID.to_vec(),
        transport_key: scenario.event.publicKey.to_vec(),
    };
    let mut kms = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    kms.mock(|when, then| {
        when.path("/kms_service.v1.CoreServiceEndpoint/UserDecrypt")
            .matcher(identity);
        then.pb(Empty::default());
    });
    kms.mock(|when, then| {
        when.path("/kms_service.v1.CoreServiceEndpoint/GetUserDecryptionResult");
        then.pb(UserDecryptionResponse {
            payload: Some(UserDecryptionResponsePayload::default()),
            signatures: vec![TypedSignature {
                scheme: SigningSchemeType::Ecdsa256k1 as i32,
                signature: vec![0; 65],
            }],
            ..Default::default()
        });
    });
    kms.start().await.unwrap();
    let config = Config {
        kms_core_endpoints: vec![kms.base_url().unwrap().to_string()],
        ..config()
    };
    let request = SolanaUserDecryptionRequestV1::try_from(scenario.event.clone()).unwrap();
    let mut event = protocol_event(request);
    event.already_sent = already_sent;

    let response = scenario
        .worker_against(
            ValidContext,
            KmsClient::connect(&config).await.unwrap(),
            &config,
            &bucket.url,
        )
        .await
        .process(&mut event)
        .await
        .expect("the KMS answers an authorized request");

    assert!(matches!(response, Some(KmsResponseKind::UserDecryption(_))));
    let sends = kms.mocks().iter().next().unwrap().match_count();
    assert_eq!(sends, usize::from(!already_sent));
    assert!(event.already_sent);
}
