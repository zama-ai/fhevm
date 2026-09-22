//! Signed KMS routing uses the connector context manager without translating its errors.

mod solana_support;

use alloy::primitives::U256;
use connector_utils::types::extra_data::ExtraData;
use kms_connector_api::ErrorCode;
use kms_worker::core::event_processor::{
    ContextManager, ProcessingErrorKind, RequestCheckError, RequestCheckKind,
};
use kms_worker::core::solana::pipeline::{AuthorizationContext, authorize_request};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_support::*;
use std::sync::Mutex;

/// A validator that serves a fixed set of pairs and records what it was asked about.
struct RecordingValidator {
    servable: Vec<(SolanaPubkeyBytes, SolanaPubkeyBytes)>,
    asked: Mutex<Vec<(SolanaPubkeyBytes, SolanaPubkeyBytes)>>,
}

impl RecordingValidator {
    fn serving(servable: &[(SolanaPubkeyBytes, SolanaPubkeyBytes)]) -> Self {
        Self {
            servable: servable.to_vec(),
            asked: Mutex::new(Vec::new()),
        }
    }

    fn asked(&self) -> Vec<(SolanaPubkeyBytes, SolanaPubkeyBytes)> {
        self.asked.lock().expect("validator lock").clone()
    }
}

impl ContextManager for RecordingValidator {
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        let kms_context_id = extra_data.context_id.unwrap().to_be_bytes::<32>();
        let kms_epoch_id = extra_data.epoch_id.unwrap().to_be_bytes::<32>();
        self.asked
            .lock()
            .expect("validator lock")
            .push((kms_context_id, kms_epoch_id));
        if self.servable.contains(&(kms_context_id, kms_epoch_id)) {
            Ok(())
        } else {
            Err(RequestCheckError::network(anyhow::anyhow!(
                "context unknown"
            )))
        }
    }
}

/// A request, a world that authorizes it, and this deployment.
fn scenario() -> (Wallet, EncryptedStoreFixture, [u8; 32]) {
    let wallet = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, wallet.pubkey());
    (wallet, encrypted_store, live)
}

fn context<'a>(
    deployment: &'a kms_worker::core::solana::deployment::DeploymentIdentity,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    }
}

/// Authorizes the reference request against a given validator, returning the outcome and the
/// number of account reads it cost.
async fn authorize_with<V: ContextManager>(
    validator: &V,
    permit: PermitBuilder,
) -> (Result<(), RequestCheckError>, usize) {
    let (wallet, encrypted_store, live) = scenario();
    let request = RequestBuilder::new(&wallet)
        .permit(permit)
        .direct(&encrypted_store, live)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let outcome = authorize_request(&reader, validator, &proofs, context(&deployment), &request)
        .await
        .map(|_| ());
    (outcome, reader.call_count())
}

/// The reference case: the signed pair is servable.
#[tokio::test]
async fn a_servable_pair_authorizes() {
    let (outcome, _) = authorize_with(
        &ServableKmsContext,
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    outcome.expect("a servable pair authorizes");
}

/// The pair that gets validated is the one inside the signed routing field — there is nowhere
/// else in the request for it to come from, and this test states which value arrives.
#[tokio::test]
async fn the_validated_pair_is_the_one_the_permit_signed() {
    let context_id: SolanaPubkeyBytes = [0x41; 32];
    let epoch_id: SolanaPubkeyBytes = [0x42; 32];
    let validator = RecordingValidator::serving(&[(context_id, epoch_id)]);

    let (outcome, _) = authorize_with(
        &validator,
        PermitBuilder::new(Wallet::new(1).pubkey()).kms_pair(context_id, epoch_id),
    )
    .await;

    outcome.expect("the signed pair is servable");
    assert_eq!(
        validator.asked(),
        vec![(context_id, epoch_id)],
        "the pair asked about is the signed one, asked once"
    );
}

/// A rotation moves the current pair on. A permit signed against the previous epoch keeps
/// working while that epoch is still servable: its shares were retained for exactly this.
#[tokio::test]
async fn a_rotation_alone_does_not_invalidate_an_outstanding_permit() {
    let context_id: SolanaPubkeyBytes = [0x41; 32];
    let previous_epoch: SolanaPubkeyBytes = [0x51; 32];
    let current_epoch: SolanaPubkeyBytes = [0x52; 32];
    // Both generations are servable, which is what "retained for in-flight use" means.
    let validator =
        RecordingValidator::serving(&[(context_id, previous_epoch), (context_id, current_epoch)]);

    let (outcome, _) = authorize_with(
        &validator,
        PermitBuilder::new(Wallet::new(1).pubkey()).kms_pair(context_id, previous_epoch),
    )
    .await;

    outcome.expect("a permit signed before the rotation is still servable after it");
}

struct RejectedContext {
    code: ErrorCode,
}
impl ContextManager for RejectedContext {
    async fn validate_context(&self, extra_data: &ExtraData) -> Result<(), RequestCheckError> {
        assert_eq!(
            extra_data.context_id,
            Some(U256::from_be_bytes(KMS_CONTEXT))
        );
        assert_eq!(extra_data.epoch_id, Some(U256::from_be_bytes(KMS_EPOCH)));
        if self.code == ErrorCode::KmsContextDestroyed {
            Err(RequestCheckError::irrecoverable(
                RequestCheckKind::KmsContext,
                ErrorCode::KmsContextDestroyed,
                anyhow::anyhow!("destroyed signed context"),
            ))
        } else if self.code == ErrorCode::KmsContextInvalid {
            Err(RequestCheckError::recoverable(
                RequestCheckKind::KmsContext,
                ErrorCode::KmsContextInvalid,
                anyhow::anyhow!("unservable context"),
            ))
        } else {
            Err(RequestCheckError::network(anyhow::anyhow!(
                "management state unavailable"
            )))
        }
    }
}

#[tokio::test]
async fn context_errors_keep_their_code_cause_and_retry_policy_before_any_account_read() {
    for (code, kind, message) in [
        (
            ErrorCode::KmsContextDestroyed,
            ProcessingErrorKind::Irrecoverable,
            "destroyed signed context",
        ),
        (
            ErrorCode::KmsContextInvalid,
            ProcessingErrorKind::Recoverable,
            "unservable context",
        ),
        (
            ErrorCode::UpstreamTransient,
            ProcessingErrorKind::Recoverable,
            "management state unavailable",
        ),
    ] {
        let (outcome, reads) = authorize_with(
            &RejectedContext { code },
            PermitBuilder::new(Wallet::new(1).pubkey()),
        )
        .await;
        let error = outcome.unwrap_err().record();
        assert_eq!(reads, 0);
        assert_eq!(error.kind, kind);
        assert_eq!(error.code, code);
        assert_eq!(error.source.to_string(), message);
    }
}
