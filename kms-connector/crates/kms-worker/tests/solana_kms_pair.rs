//! KMS context and epoch servability: the rule that decides which share generation a request
//! may be answered from.
//!
//! The pair comes out of the permit's signed routing field, so it is the pair the wallet agreed
//! to. Two things follow that the tests here pin.
//!
//! First, rotation is not invalidation. A context switch or an epoch rotation does not kill
//! outstanding permits: old-epoch shares are retained for in-flight use, and the signed pair
//! stays servable until the permit expires, is invalidated, or governance destroys that epoch or
//! context explicitly. Treating rotation as invalidation would turn every key resharing into a
//! mass revocation of every permit in the wild.
//!
//! Second, the classification is the reason this is not a boolean. Unknown and not-yet-active
//! may become servable and are worth repeating; destroyed never will be. Getting the two
//! backwards means clients either retry forever or bury a request that would have succeeded.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    kms_pair::{KmsPairFailure, KmsPairValidator},
    pipeline::{AuthorizationContext, authorize_request},
};
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

impl KmsPairValidator for RecordingValidator {
    async fn validate_pair(
        &self,
        kms_context_id: &SolanaPubkeyBytes,
        kms_epoch_id: &SolanaPubkeyBytes,
    ) -> Result<(), KmsPairFailure> {
        self.asked
            .lock()
            .expect("validator lock")
            .push((*kms_context_id, *kms_epoch_id));
        if self.servable.contains(&(*kms_context_id, *kms_epoch_id)) {
            Ok(())
        } else {
            Err(KmsPairFailure::ContextUnknown)
        }
    }
}

/// A request, a world that authorizes it, and this deployment.
fn scenario() -> (Wallet, LineageFixture, [u8; 32]) {
    let wallet = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[wallet.pubkey()]);
    (wallet, lineage, live)
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
async fn authorize_with<V: KmsPairValidator>(
    validator: &V,
    permit: PermitBuilder,
) -> (Result<(), AuthorizationFailure>, usize) {
    let (wallet, lineage, live) = scenario();
    let request = RequestBuilder::new(&wallet)
        .permit(permit)
        .direct_current(&lineage, live)
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&lineage)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let outcome = authorize_request(&reader, validator, context(&deployment), &request)
        .await
        .map(|_| ());
    (outcome, reader.call_count())
}

/// The reference case: the signed pair is servable.
#[tokio::test]
async fn a_servable_pair_authorizes() {
    let (outcome, _) = authorize_with(
        &ServableKmsPair,
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

/// A destroyed context is terminal: governance has said this material is gone, and no retry
/// brings it back.
#[tokio::test]
async fn a_destroyed_context_is_terminal() {
    let (outcome, _) = authorize_with(
        &UnservableKmsPair(KmsPairFailure::ContextDestroyed),
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    let failure = outcome.expect_err("a destroyed context serves nothing");
    assert!(matches!(
        failure,
        AuthorizationFailure::KmsPair(KmsPairFailure::ContextDestroyed)
    ));
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// Everything the source cannot tell apart arrives as one transient outcome.
///
/// Three different worlds land here — the epoch is not active yet, it belongs to another context,
/// it was destroyed — because the inherited validation learns about all three from a single
/// boolean that is true only when the epoch is active *and* belongs to the signed context. Nothing
/// exposes the epoch's state or its context separately.
///
/// The uncomfortable half of that is the destroyed epoch: it will never become servable, and it is
/// still retried within the attempt budget. That is inherited behaviour rather than a decision of
/// this path, and it is fail-closed either way. Making it terminal means feeding epoch destruction
/// into the local database — validation the EVM path shares, so a change to EVM behaviour.
#[tokio::test]
async fn every_epoch_level_reason_arrives_as_one_transient_outcome() {
    let (outcome, _) = authorize_with(
        &UnservableKmsPair(KmsPairFailure::PairNotServable),
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    let failure = outcome.expect_err("a pair that is not servable authorizes nothing");
    assert!(matches!(
        failure,
        AuthorizationFailure::KmsPair(KmsPairFailure::PairNotServable)
    ));
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// The classification has exactly two outcomes, because exactly two are distinguishable. Written
/// as a table so that adding a terminal case without adding a source of knowledge fails here —
/// the tempting edit is to call a destroyed epoch terminal, and this is where that stops.
#[test]
fn the_classification_is_the_two_outcomes_the_shared_validation_can_tell_apart() {
    for (failure, expected) in [
        (KmsPairFailure::ContextDestroyed, FailureClass::Terminal),
        (KmsPairFailure::ContextUnknown, FailureClass::Transient),
        (KmsPairFailure::PairNotServable, FailureClass::Transient),
        (
            KmsPairFailure::Unavailable {
                reason: "connection refused".to_owned(),
            },
            FailureClass::Transient,
        ),
    ] {
        assert_eq!(
            AuthorizationFailure::KmsPair(failure.clone()).class(),
            expected,
            "{failure} is classified as {expected:?}"
        );
    }
}

/// An unknown context may simply not have reached this party's view of management state yet.
#[tokio::test]
async fn an_unknown_context_is_transient() {
    let (outcome, _) = authorize_with(
        &UnservableKmsPair(KmsPairFailure::ContextUnknown),
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    assert_eq!(
        outcome
            .expect_err("an unknown context is not servable yet")
            .class(),
        FailureClass::Transient
    );
}

/// Management state being unreachable says nothing about the pair, so it is transient — the
/// request is not condemned by an outage.
#[tokio::test]
async fn unreachable_management_state_is_transient() {
    let (outcome, _) = authorize_with(
        &UnservableKmsPair(KmsPairFailure::Unavailable {
            reason: "connection refused".to_owned(),
        }),
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    assert_eq!(
        outcome.expect_err("an outage is not an answer").class(),
        FailureClass::Transient
    );
}

/// The pair is checked before host state is read. A request that cannot be answered by this KMS
/// generation costs no account read — the ordering is not an optimization detail, it is what
/// keeps a misrouted flood of requests from becoming a flood of RPC calls.
#[tokio::test]
async fn an_unservable_pair_costs_no_account_read() {
    let (outcome, reads) = authorize_with(
        &UnservableKmsPair(KmsPairFailure::ContextDestroyed),
        PermitBuilder::new(Wallet::new(1).pubkey()),
    )
    .await;

    assert!(outcome.is_err());
    assert_eq!(reads, 0, "state-free rules run before the snapshot");
}
