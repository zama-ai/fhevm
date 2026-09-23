//! A Solana user decryption as the worker runs it: the Gateway event, the listener's conversion,
//! and the worker's checks against a host behind real HTTP.

// Only the Gateway registry mock is used here.
#[allow(dead_code)]
mod common;
mod solana_support;

use alloy::{
    primitives::U256,
    providers::{ProviderBuilder, RootProvider, mock::Asserter},
};
use common::{TEST_COPRO_REGISTRY_REFRESH, mock_copro_registry_load};
use connector_utils::{
    monitoring::otlp::PropagationContext,
    tests::rand::solana_user_decryption_event_for,
    types::{
        ProtocolEvent, ProtocolEventKind,
        db::RequestSource,
        extra_data::{ExtraData, parse_extra_data},
        solana_request::SolanaUserDecryptionRequestV1,
    },
};
use fhevm_gateway_bindings::decryption::Decryption::UserDecryptionRequest_4;
use kms_connector_api::ErrorCode;
use kms_worker::core::{
    Config,
    event_processor::{
        CiphertextManager, ContextManager, DbEventProcessor, DecryptionProcessor, EventProcessor,
        HostChainAclBackend, KMSGenerationProcessor, KmsClient, ProcessingErrorKind,
        ProtocolConfigProcessor, RequestCheckError, RequestCheckKind,
    },
    solana::{failure::AuthorizationFailure, pause::PauseFailure},
};
use rstest::rstest;
use solana_support::*;
use sqlx::{postgres::PgPoolOptions, types::chrono::Utc};
use std::{
    collections::HashMap,
    sync::{Arc, Mutex},
};
use tokio_util::sync::CancellationToken;
use zama_solana_permit::{Identity, KmsRouting, TRANSPORT_KEY_LEN};

/// A victim's request as the Gateway emits it, and a host whose state authorizes it: one handle
/// the victim was allowed on directly.
struct Scenario {
    victim: Wallet,
    encrypted_store: EncryptedStoreFixture,
    event: UserDecryptionRequest_4,
    host: HttpHost,
}

impl Scenario {
    async fn new() -> Self {
        let victim = Wallet::new(1);
        let live = handle(0x10, FHE_TYPE_UINT64);
        let encrypted_store = EncryptedStoreFixture::allowing(live, victim.pubkey());
        let now = Utc::now().timestamp() as u64;
        let request = RequestBuilder::new(&victim)
            .permit(PermitBuilder::new(victim.pubkey()).window(now - 60, 3_600))
            .direct(&encrypted_store, live)
            .wire();
        let query = encrypted_store.allowed_query(live, victim.pubkey());

        let mut scenario = Self {
            event: solana_user_decryption_event_for(U256::ONE, &request),
            host: HttpHost::start().await,
            victim,
            encrypted_store,
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
        let config = config();
        let asserter = Asserter::new();
        mock_copro_registry_load(&asserter, "http://unused-bucket-url");
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(asserter);
        let ciphertext_manager =
            CiphertextManager::connect(provider.clone(), &config, CancellationToken::new())
                .await
                .unwrap();
        let backends = HashMap::from([(
            CHAIN_ID,
            HostChainAclBackend::Solana(Box::new(self.host.host())),
        )]);
        DecryptionProcessor::new(&config, provider, backends, ciphertext_manager)
    }

    /// A worker whose KMS client has no channel and whose database is unreachable, so a request
    /// that got past its checks would fail another way.
    async fn worker<C: ContextManager>(
        &self,
        context_manager: C,
    ) -> DbEventProcessor<RootProvider, RootProvider, C> {
        let config = config();
        let provider = ProviderBuilder::new()
            .disable_recommended_fillers()
            .connect_mocked_client(Asserter::new());
        DbEventProcessor::new(
            KmsClient::new(vec![], 0),
            context_manager,
            self.processor().await,
            KMSGenerationProcessor::new(&config),
            ProtocolConfigProcessor::new(&config, provider),
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
