//! A Solana user decryption as the worker runs it: the typed request the HTTP endpoint stores,
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
    tests::setup::{S3_SOLANA_CT_HANDLE, S3Instance},
    types::{
        KmsResponseKind, ProtocolEvent, ProtocolEventKind,
        db::RequestSource,
        extra_data::{ExtraData, parse_extra_data},
        handle::extract_chain_id_from_handle,
        solana_request::{
            SolanaPublicDecryptionRequest, SolanaUserDecryptFields, SolanaUserDecryptionRequestV1,
        },
    },
};
use kms_connector_api::ErrorCode;
use kms_grpc::kms::v1::{
    Empty, SigningSchemeType, TypedSignature, UserDecryptionRequest, UserDecryptionResponse,
    UserDecryptionResponsePayload,
};
use kms_worker::core::{
    Config,
    event_processor::{
        CiphertextManager, ContextManager, DbEventProcessor, DecryptionProcessor, EventProcessor,
        KMSGenerationProcessor, KmsClient, ProcessingErrorKind, ProtocolConfigProcessor,
        RequestCheckError, RequestCheckKind,
    },
    solana::{
        SolanaDecryptionVerifier, failure::AuthorizationFailure, membership::KmsContextMembers,
        pipeline::authorize_request, watermark::WatermarkFailure,
    },
};
use mocktail::{Request, StatusCode, matchers::Matcher, server::MockServer};
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
use zama_solana_merkle_proofs::MERKLE_PROOFS_PATH;
use zama_solana_permit::{Identity, KmsRouting, TRANSPORT_KEY_LEN};

/// A Gateway registry with no ciphertext bucket, for requests that never fetch a ciphertext.
const NO_BUCKET: &str = "http://unused-bucket-url";

/// A victim's request as the relayer submits it, and a host whose state authorizes it: one handle
/// the victim was allowed on directly.
struct Scenario {
    victim: Wallet,
    encrypted_store: EncryptedStoreFixture,
    fields: SolanaUserDecryptFields,
    host: HttpHost,
    chain_id: u64,
    live: [u8; 32],
    /// The KMS context nodes the worker checks its own node against: by default, its node is one.
    members: KmsContextMembers,
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
        let fields = RequestBuilder::new(&victim)
            .permit(
                PermitBuilder::new(victim.pubkey())
                    .chain_id(chain_id)
                    .window(now - 60, 3_600),
            )
            .direct(&encrypted_store, live)
            .fields();

        let mut scenario = Self {
            fields,
            host: HttpHost::start().await,
            victim,
            encrypted_store,
            chain_id,
            live,
            members: kms_context_members(true, Duration::ZERO).0,
        };
        scenario.serve_host(None);
        scenario.serve_proofs();
        scenario
    }

    /// The coprocessor's answer to the request: the proof of the victim's allow leaf.
    fn serve_proofs(&mut self) {
        let query = self
            .encrypted_store
            .allowed_query(self.live, self.victim.pubkey());
        self.host
            .serve_proofs(&[(query, self.encrypted_store.outcome(&query))]);
    }

    /// The first read of a direct request: the signer's invalidation record, holding
    /// `watermark` if the signer ever invalidated their permits, and the entry's encrypted store.
    fn serve_host(&mut self, watermark: Option<u64>) {
        let victim = self.victim.pubkey();
        self.host.serve_accounts(&[
            (
                invalidation_address(victim).0,
                watermark.map(|watermark| invalidation_account(victim, watermark)),
            ),
            (
                self.encrypted_store.account_key,
                Some(self.encrypted_store.account()),
            ),
        ]);
    }

    /// The typed request the endpoint stores for `fields`.
    fn request_with(&self, fields: SolanaUserDecryptFields) -> SolanaUserDecryptionRequestV1 {
        SolanaUserDecryptionRequestV1::new(U256::ONE, fields).unwrap()
    }

    fn request(&self) -> SolanaUserDecryptionRequestV1 {
        self.request_with(self.fields.clone())
    }

    /// A public decryption, in the user request's KMS context, of a handle its store made public.
    /// The host then serves that store and its proof only.
    fn public_request(&mut self) -> SolanaPublicDecryptionRequest {
        let public = handle(0x20, FHE_TYPE_UINT64);
        let mut store = EncryptedStoreFixture::new(public);
        store.mark_public();
        let query = store.public_query(public);
        self.host
            .serve_accounts(&[(store.account_key, Some(store.account()))]);
        self.host.serve_proofs(&[(query, store.outcome(&query))]);
        SolanaPublicDecryptionRequest::new(
            U256::ONE,
            &[B256::new(public)],
            &[B256::new(store.account_key.to_bytes())],
            self.request().extra_data(),
        )
        .unwrap()
    }

    /// The proof batches the coprocessor has answered.
    fn proof_batches(&self) -> usize {
        self.host
            .coprocessor
            .mocks()
            .iter()
            .map(|mock| mock.match_count())
            .sum()
    }

    fn verifier(&self) -> SolanaDecryptionVerifier {
        SolanaDecryptionVerifier::new(
            HashMap::from([(self.chain_id, self.host.host())]),
            self.members.clone(),
        )
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
        DecryptionProcessor::new(config, provider, HashMap::new(), ciphertext_manager)
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
            self.verifier(),
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

/// Rejects every KMS context as destroyed, after `delay`, and records the routing it was asked
/// about.
struct DestroyedContext {
    asked: Arc<Mutex<Vec<ExtraData>>>,
    delay: Duration,
}

impl ContextManager for DestroyedContext {
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        tokio::time::sleep(self.delay).await;
        self.asked.lock().unwrap().push(extra_data.clone());
        Err(RequestCheckError::irrecoverable(
            RequestCheckKind::KmsContext,
            ErrorCode::KmsContextDestroyed,
            anyhow::anyhow!("destroyed context"),
        ))
    }
}

/// The relayer sets the request's key, window and routing, and those are the values stored, so
/// changing one yields a permit the user never signed.
#[rstest]
#[case::transport_key(|fields: &mut SolanaUserDecryptFields| {
    fields.transport_key = vec![0x5a; TRANSPORT_KEY_LEN];
})]
#[case::window(|fields: &mut SolanaUserDecryptFields| {
    fields.start_timestamp -= 60;
})]
#[case::kms_routing(|fields: &mut SolanaUserDecryptFields| {
    fields.extra_data = KmsRouting::ContextAndEpoch {
        kms_context_id: Identity::new([0x13; 32]),
        kms_epoch_id: Identity::new([0x14; 32]),
    }
    .to_extra_data();
})]
#[tokio::test]
async fn a_field_the_relayer_changed_fails_the_signature(
    #[case] relayer: fn(&mut SolanaUserDecryptFields),
) {
    let scenario = Scenario::new().await;
    let mut fields = scenario.fields.clone();
    relayer(&mut fields);
    let request = scenario.request_with(fields);

    let error = scenario
        .verifier()
        .check_user_decryption(&request)
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

/// The permit's chain is the one the handles embed, so a handle moved to another chain changes the
/// signed text. The worker refuses a chain it does not serve before authorizing, so this runs the
/// authorization itself.
#[tokio::test]
async fn a_handle_of_another_chain_fails_the_signature() {
    let wallet = Wallet::new(1);
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, wallet.pubkey());
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let mut fields = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, live)
        .fields();
    fields.entries[0].handle[22..30].copy_from_slice(&(CHAIN_ID + 1).to_be_bytes());
    let request = SolanaUserDecryptionRequestV1::new(U256::ONE, fields).unwrap();

    let failure = authorize_request(
        &ScriptedReader::constant(world.clone()),
        &ScriptedProofReader::constant(world.record()),
        CONTEXT,
        &request,
    )
    .await
    .expect_err("the signer never signed this chain");

    assert!(matches!(failure, AuthorizationFailure::Signature(_)));
    assert!(!failure.is_recoverable());
}

/// A poll of a request already sent to the KMS authorizes it again, so permits the signer
/// invalidates after the send stop it.
#[tokio::test]
async fn a_poll_rechecks_the_host() {
    let mut scenario = Scenario::new().await;
    let request = scenario.request();
    scenario
        .verifier()
        .check_user_decryption(&request)
        .await
        .expect("the victim's permit authorizes the request");

    scenario.serve_host(Some(Utc::now().timestamp() as u64));
    let mut event = protocol_event(request);
    event.already_sent = true;
    let error = scenario
        .worker(ValidContext)
        .await
        .process(&mut event)
        .await
        .expect_err("the poll must reject the newly invalidated permit");

    assert!(matches!(
        error.source.downcast_ref::<AuthorizationFailure>(),
        Some(AuthorizationFailure::Watermark(
            WatermarkFailure::Invalidated { .. }
        ))
    ));
    assert!(event.already_sent);
}

/// The KMS context is checked like every other decryption's, against the signed routing.
#[tokio::test]
async fn a_solana_request_checks_its_signed_kms_context() {
    let scenario = Scenario::new().await;
    let request = scenario.request();
    let asked = Arc::default();

    let error = scenario
        .worker(DestroyedContext {
            asked: Arc::clone(&asked),
            delay: Duration::ZERO,
        })
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

/// A request of either kind that the host authorizes, as its worker would process it. Checking it
/// reads one proof batch.
async fn authorized_event(scenario: &mut Scenario, public: bool) -> ProtocolEvent {
    let kind = if public {
        let request = scenario.public_request();
        scenario
            .verifier()
            .check_public_decryption(&request)
            .await
            .expect("the host authorizes the request");
        ProtocolEventKind::SolanaPublicDecryption(request)
    } else {
        let request = scenario.request();
        scenario
            .verifier()
            .check_user_decryption(&request)
            .await
            .expect("the host authorizes the request");
        ProtocolEventKind::SolanaUserDecryptionV1(request)
    };
    assert_eq!(scenario.proof_batches(), 1);
    let mut event = ProtocolEvent::new(
        kind,
        None,
        PropagationContext::default(),
        RequestSource::Http,
    );
    // A poll runs only the checks: a send would also prepare the request, which fails first here
    // without a ciphertext bucket.
    event.already_sent = true;
    event
}

/// A node outside the request's KMS context is refused for good by its own worker, as the
/// tx-sender that signs its proof requests, and sends no proof request, even when the membership
/// read is slower than the host.
#[rstest]
#[case::user_decryption(false)]
#[case::public_decryption(true)]
#[tokio::test]
async fn a_node_outside_the_requests_context_sends_no_proof_request(#[case] public: bool) {
    let mut scenario = Scenario::new().await;
    let mut event = authorized_event(&mut scenario, public).await;
    let calls;
    (scenario.members, calls) = kms_context_members(false, Duration::from_millis(500));

    let error = scenario
        .worker(ValidContext)
        .await
        .process(&mut event)
        .await
        .expect_err("a node outside the context is refused");

    assert_eq!(
        (error.kind, error.code),
        (ProcessingErrorKind::Irrecoverable, ErrorCode::Unprocessable),
        "{error}"
    );
    let context_id = parse_extra_data(&scenario.request().extra_data())
        .unwrap()
        .context_id
        .unwrap();
    let asked: Vec<_> = calls
        .lock()
        .unwrap()
        .iter()
        .map(|call| (call.kmsContextId, call.txSender))
        .collect();
    assert_eq!(
        asked,
        vec![(context_id, scenario.host.host().proofs.tx_sender())]
    );
    assert_eq!(
        scenario.proof_batches(),
        1,
        "the worker sent a proof request"
    );
}

/// The request's KMS context is checked before the verifier reads anything: a destroyed context
/// stops the request with no proof request, even when the context check is slower than the host.
#[rstest]
#[case::user_decryption(false)]
#[case::public_decryption(true)]
#[tokio::test]
async fn a_destroyed_context_sends_no_proof_request(#[case] public: bool) {
    let mut scenario = Scenario::new().await;
    let mut event = authorized_event(&mut scenario, public).await;

    let error = scenario
        .worker(DestroyedContext {
            asked: Arc::default(),
            delay: Duration::from_millis(500),
        })
        .await
        .process(&mut event)
        .await
        .expect_err("a destroyed context stops the request");

    assert_eq!(error.code, ErrorCode::KmsContextDestroyed, "{error}");
    assert_eq!(
        scenario.proof_batches(),
        1,
        "the worker sent a proof request"
    );
}

/// The identity a `UserDecrypt` call must carry for a Solana user.
#[derive(Debug, PartialEq, PartialOrd)]
struct SolanaIdentity {
    /// Base58 of the signer's public key.
    client_address: String,
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
        request.client_address == self.client_address && request.enc_key == self.transport_key
    }
}

/// Scripts `kms` to answer every poll with a user decryption result.
fn serve_user_decryption_result(kms: &mut MockServer) {
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
}

/// A node whose coprocessor has not read its KMS context yet gets `upstream_transient`, which the
/// worker records as recoverable, so a later attempt can run. Once the coprocessor has read the
/// context, that attempt is answered. Here the test runs both attempts itself, as relayer-http's
/// retry would. Both attempts are polls, which run every check: a send
/// would also prepare the request, which needs a ciphertext bucket. Each attempt runs on its own
/// worker, as the host's RPC mock answers only a client's first request id.
#[tokio::test]
async fn a_node_its_coprocessor_does_not_know_yet_is_answered_at_a_later_attempt() {
    let mut scenario = Scenario::new().await;
    let not_yet = serde_json::to_vec(&serde_json::json!({
        "code": "upstream_transient",
        "message": "not, or not yet, the tx-sender of a node in an Active KMS context",
        "retryable": true,
    }))
    .unwrap();
    scenario.host.coprocessor.mocks().clear();
    scenario.host.coprocessor.mock(move |when, then| {
        when.post().path(MERKLE_PROOFS_PATH);
        then.status(StatusCode::BAD_GATEWAY).bytes(not_yet.clone());
    });
    let mut kms = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    serve_user_decryption_result(&mut kms);
    kms.start().await.unwrap();
    let config = Config {
        kms_core_endpoints: vec![kms.base_url().unwrap().to_string()],
        ..config()
    };
    let worker = async |scenario: &Scenario| {
        scenario
            .worker_against(
                ValidContext,
                KmsClient::connect(&config).await.unwrap(),
                &config,
                NO_BUCKET,
            )
            .await
    };
    let mut event = protocol_event(scenario.request());
    event.already_sent = true;

    let error = worker(&scenario)
        .await
        .process(&mut event)
        .await
        .expect_err("the coprocessor does not know the node yet");
    assert_eq!(
        (error.kind, error.code),
        (
            ProcessingErrorKind::Recoverable,
            ErrorCode::UpstreamTransient
        ),
        "{error}"
    );

    scenario.serve_proofs();
    let response = worker(&scenario)
        .await
        .process(&mut event)
        .await
        .expect("the coprocessor has read the node's context");
    assert!(matches!(response, Some(KmsResponseKind::UserDecryption(_))));
}

/// An authorized request reaches the KMS as its Solana signer: the signer's base58 public key as
/// `client_address`, and the request's transport key. A poll of a request already sent
/// authorizes it again and fetches the result without sending it twice.
#[rstest]
#[case::send(false)]
#[case::poll(true)]
#[timeout(Duration::from_secs(60))]
#[tokio::test]
async fn an_authorized_request_reaches_the_kms_as_its_signer(#[case] already_sent: bool) {
    let bucket = S3Instance::setup().await.unwrap();
    let scenario = Scenario::naming(B256::from_hex(S3_SOLANA_CT_HANDLE).unwrap().0).await;
    let identity = SolanaIdentity {
        client_address: scenario.victim.pubkey().to_string(),
        transport_key: scenario.fields.transport_key.clone(),
    };
    let mut kms = MockServer::new_grpc("kms_service.v1.CoreServiceEndpoint");
    kms.mock(|when, then| {
        when.path("/kms_service.v1.CoreServiceEndpoint/UserDecrypt")
            .matcher(identity);
        then.pb(Empty::default());
    });
    serve_user_decryption_result(&mut kms);
    kms.start().await.unwrap();
    let config = Config {
        kms_core_endpoints: vec![kms.base_url().unwrap().to_string()],
        ..config()
    };
    let request = scenario.request();
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
