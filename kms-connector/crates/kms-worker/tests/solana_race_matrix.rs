//! The race matrix: six state transitions, each asserted at both observation points.
//!
//! Every scenario is two worlds and one request. The request is authorized against the state
//! before the transition and against the state after it, and both outcomes are stated. That is
//! the only honest way to talk about races in this system, because authorization is a function of
//! an observation rather than of a moment: there is no timing to arrange, no sleep to tune, and
//! no flake to chase.
//!
//! Each scenario also asserts its mirror image — that a request accepted before the transition is
//! not reopened by it. The mirror is enforced structurally as much as asserted: the scripted
//! reader panics if authorization reads state more times than the scenario allows, so an
//! implementation that "checked once more before sending" would fail the mirror rather than
//! quietly making the accept conditional on a later state.
//!
//! The rows, in the order the specification lists them:
//!
//! 1. the current handle is replaced;
//! 2. the subject set rotates and the owner leaves it;
//! 3. the delegation is revoked;
//! 4. an append that does not merge the proof's peak;
//! 5. an append that merges the proof's peak;
//! 6. the proof service is ahead of this Connector, or the two are on disagreeing forks.

mod solana_support;

use kms_worker::core::solana::{
    delegation::DelegationFailure,
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::{HandleBindingFailure, InclusionAction},
    pipeline::{AuthorizationContext, AuthorizedRequest, authorize_request},
    request::SolanaUserDecryptRequest,
};
use solana_support::*;

const BEFORE: u64 = 500;
const AFTER: u64 = 501;

/// Authorizes a request against one world, which every read of the scenario sees.
async fn observe(
    world: World,
    request: &SolanaUserDecryptRequest,
) -> (Result<AuthorizedRequest, AuthorizationFailure>, usize) {
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();
    let context = AuthorizationContext {
        deployment: &deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    };
    let outcome = authorize_request(&reader, &ServableKmsPair, context, request).await;
    (outcome, reader.call_count())
}

/// Asserts the mirror: accepted at the pre-transition observation, recorded at that observation,
/// and nothing read afterwards.
fn assert_frozen_at(authorized: &AuthorizedRequest, reads: usize, expected_reads: usize) {
    assert_eq!(
        authorized.observed_slot(),
        BEFORE,
        "an accepted request records the observation it was accepted at"
    );
    assert_eq!(
        reads, expected_reads,
        "nothing re-reads state after acceptance, so the transition cannot reach this request"
    );
}

// ---------------------------------------------------------------------------
// 1. Current-handle supersession
// ---------------------------------------------------------------------------

/// A current-mode entry racing a supersession fails at the later observation, and fails
/// terminally: the handle it named will never be current again. What it does not do is return the
/// value that is current instead.
#[tokio::test]
async fn supersession_rejects_a_current_entry_at_the_later_observation() {
    let signer = Wallet::new(1);
    let named = handle(0x10, FHE_TYPE_UINT64);
    let before = EncryptedValueAccountFixture::new(named, &[signer.pubkey()]);
    let mut after = before.clone();
    after.update(handle(0x11, FHE_TYPE_UINT64));
    let request = RequestBuilder::new(&signer)
        .direct_current(&before, named)
        .typed();

    let (accepted, reads) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    assert_frozen_at(
        &accepted.expect("before the supersession the handle is current"),
        reads,
        1,
    );

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    let failure = outcome.expect_err("after the supersession the handle is not current");
    assert!(matches!(
        failure,
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NotCurrentHandle { .. }
        }
    ));
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// The griefed request is not a dead end. Supersession seals a leaf for the subjects that held the
/// handle, so the same handle is reachable at the later observation as historical access — under
/// the same permit, with no new signature.
#[tokio::test]
async fn supersession_leaves_the_historical_path_open() {
    let signer = Wallet::new(1);
    let named = handle(0x12, FHE_TYPE_UINT64);
    let mut after = EncryptedValueAccountFixture::new(named, &[signer.pubkey()]);
    after.update(handle(0x13, FHE_TYPE_UINT64));
    let retry = RequestBuilder::new(&signer)
        .historical(&after, named, signer.pubkey(), &after.proof(0), 1)
        .typed();

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &retry,
    )
    .await;

    outcome.expect("the sealed leaf authorizes the same handle after supersession");
}

// ---------------------------------------------------------------------------
// 2. Subject rotation
// ---------------------------------------------------------------------------

/// A rotation that removes the owner from the subject set rejects current access at the later
/// observation.
#[tokio::test]
async fn subject_rotation_rejects_a_current_entry_at_the_later_observation() {
    let signer = Wallet::new(1);
    let stranger = Wallet::new(9);
    let live = handle(0x21, FHE_TYPE_UINT64);
    let mut before =
        EncryptedValueAccountFixture::new(handle(0x20, FHE_TYPE_UINT64), &[signer.pubkey()]);
    before.update(live);
    let mut after = before.clone();
    after.rotate_subjects(&[stranger.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .direct_current(&before, live)
        .typed();

    let (accepted, reads) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    assert_frozen_at(
        &accepted.expect("before the rotation the signer is a member"),
        reads,
        1,
    );

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    assert!(matches!(
        outcome.expect_err("after the rotation the signer is not a member"),
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NotAMember { .. }
        }
    ));
}

/// Leaves sealed before the rotation are untouched by it. The subject is inside the leaf, so
/// membership at the time of the request is irrelevant to a handle that was already sealed.
#[tokio::test]
async fn subject_rotation_does_not_reach_leaves_sealed_before_it() {
    let signer = Wallet::new(1);
    let stranger = Wallet::new(9);
    let sealed = handle(0x22, FHE_TYPE_UINT64);
    let mut after = EncryptedValueAccountFixture::new(sealed, &[signer.pubkey()]);
    after.update(handle(0x23, FHE_TYPE_UINT64));
    let proof = after.proof(0);
    after.rotate_subjects(&[stranger.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .historical(&after, sealed, signer.pubkey(), &proof, 1)
        .typed();

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("a leaf sealed before the rotation still authorizes its subject");
}

// ---------------------------------------------------------------------------
// 3. Delegation revocation
// ---------------------------------------------------------------------------

/// A delegated entry racing a revocation fails at the later observation. The revocation reaches
/// the next request rather than the next permit, which is what "immediately" means here.
#[tokio::test]
async fn delegation_revocation_rejects_its_entry_at_the_later_observation() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let granted = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), BEFORE);
    let mut revoked = granted;
    revoked.revoked = true;
    revoked.last_update_slot = AFTER;
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (accepted, reads) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&encrypted_value_account)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&granted),
        &request,
    )
    .await;
    assert_frozen_at(
        &accepted.expect("before the revocation the delegation is live"),
        reads,
        2,
    );

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&encrypted_value_account)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&revoked),
        &request,
    )
    .await;

    assert!(matches!(
        outcome.expect_err("after the revocation the delegation is dead"),
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Revoked
        }
    ));
}

/// The direct branch is untouched by a delegation revocation: the signer's own handles are not
/// held on anybody's grant.
#[tokio::test]
async fn delegation_revocation_does_not_touch_the_direct_branch() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let own = handle(0x31, FHE_TYPE_UINT64);
    let own_encrypted_value_account = EncryptedValueAccountFixture::new(own, &[signer.pubkey()]);
    let mut revoked = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), BEFORE);
    revoked.revoked = true;
    let request = RequestBuilder::new(&signer)
        .direct_current(&own_encrypted_value_account, own)
        .typed();

    let (outcome, reads) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&own_encrypted_value_account)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&revoked),
        &request,
    )
    .await;

    outcome.expect("a revoked delegation says nothing about the signer's own handles");
    assert_eq!(
        reads, 1,
        "a direct-only request does not even read the delegation record"
    );
}

// ---------------------------------------------------------------------------
// 4. An append that does not merge the proof's peak
// ---------------------------------------------------------------------------

/// The append case that must be accepted. The proof was built against an earlier leaf count, the
/// append left its peak alone, and it still verifies — so it is taken. Rejecting on age would
/// break every historical request that raced any write.
#[tokio::test]
async fn an_append_that_leaves_the_peak_alone_still_authorizes() {
    let signer = Wallet::new(1);
    let sealed = handle(0x40, FHE_TYPE_UINT64);
    let mut before = EncryptedValueAccountFixture::new(sealed, &[signer.pubkey()]);
    before.update(handle(0x41, FHE_TYPE_UINT64));
    before.update(handle(0x42, FHE_TYPE_UINT64));
    assert_eq!(before.encrypted_value.leaf_count, 2);
    let proof = before.proof(0);
    let claimed = before.encrypted_value.leaf_count;
    let mut after = before.clone();
    after.update(handle(0x43, FHE_TYPE_UINT64));
    assert_eq!(after.encrypted_value.leaf_count, 3);
    let request = RequestBuilder::new(&signer)
        .historical(&before, sealed, signer.pubkey(), &proof, claimed)
        .typed();

    let (accepted, reads) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    assert_frozen_at(
        &accepted.expect("the proof verifies before the append"),
        reads,
        1,
    );

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("the proof still verifies after an append that did not merge its peak");
}

// ---------------------------------------------------------------------------
// 5. An append that merges the proof's peak
// ---------------------------------------------------------------------------

/// The append case that cannot be accepted: the merge removed the sibling path the proof carries.
/// The claimed count is below the observed one, so the client is told to rebuild rather than to
/// retry — the same request will fail forever.
#[tokio::test]
async fn an_append_that_merges_the_peak_asks_for_a_rebuilt_proof() {
    let signer = Wallet::new(1);
    let sealed = handle(0x52, FHE_TYPE_UINT64);
    let mut before =
        EncryptedValueAccountFixture::new(handle(0x50, FHE_TYPE_UINT64), &[signer.pubkey()]);
    before.update(handle(0x51, FHE_TYPE_UINT64));
    before.update(sealed);
    before.update(handle(0x53, FHE_TYPE_UINT64));
    assert_eq!(before.encrypted_value.leaf_count, 3);
    let proof = before.proof(2);
    let claimed = before.encrypted_value.leaf_count;
    let mut after = before.clone();
    after.update(handle(0x54, FHE_TYPE_UINT64));
    assert_eq!(after.encrypted_value.leaf_count, 4);
    let request = RequestBuilder::new(&signer)
        .historical(&before, sealed, signer.pubkey(), &proof, claimed)
        .typed();

    let (accepted, reads) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    assert_frozen_at(
        &accepted.expect("the proof verifies before the merging append"),
        reads,
        1,
    );

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a merged-away sibling path cannot verify");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::InclusionFailed {
                index: 0,
                action: InclusionAction::RebuildProof,
                proof_leaf_count: 3,
                live_leaf_count: 4,
            }
        ),
        "expected a rebuild classification, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
}

// ---------------------------------------------------------------------------
// 6. The proof service ahead of this Connector
// ---------------------------------------------------------------------------

/// The mirror image of the row above, and the reason the classification exists at all. Here the
/// proof is fine and the observation is behind it: the proof service saw four leaves, this
/// Connector sees two. The claimed count is not below the observed one, so the request is worth
/// repeating — and at the later observation, once the view has caught up, the same request is
/// authorized.
///
/// Note which direction the transition runs in this row: the state does not change under the
/// request, the observer catches up to it.
#[tokio::test]
async fn a_proof_ahead_of_the_observation_is_retryable_and_then_authorized() {
    let signer = Wallet::new(1);
    let sealed = handle(0x62, FHE_TYPE_UINT64);
    let mut behind =
        EncryptedValueAccountFixture::new(handle(0x60, FHE_TYPE_UINT64), &[signer.pubkey()]);
    behind.update(handle(0x61, FHE_TYPE_UINT64));
    behind.update(sealed);
    assert_eq!(behind.encrypted_value.leaf_count, 2);

    // The state the proof service observed: two more leaves, and a proof of the sealed handle
    // built against that later state.
    let mut caught_up = behind.clone();
    caught_up.update(handle(0x63, FHE_TYPE_UINT64));
    caught_up.update(handle(0x64, FHE_TYPE_UINT64));
    assert_eq!(caught_up.encrypted_value.leaf_count, 4);
    // Leaf 2 is the one that commits `sealed`: replacing a handle seals the *outgoing* one, so
    // the leaf appended by `update(0x63)` is the one naming 0x62.
    let proof = caught_up.proof(2);
    let claimed = caught_up.encrypted_value.leaf_count;
    let request = RequestBuilder::new(&signer)
        .historical(&caught_up, sealed, signer.pubkey(), &proof, claimed)
        .typed();

    let (outcome, _) = observe(
        World::at_slot(BEFORE)
            .with_encrypted_value_account(&behind)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    let failure =
        outcome.expect_err("a proof of state this observation has not seen cannot verify");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::InclusionFailed {
                index: 0,
                action: InclusionAction::RetryAtLaterSnapshot,
                proof_leaf_count: 4,
                live_leaf_count: 2,
            }
        ),
        "expected a retry classification, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Retryable);

    let (outcome, _) = observe(
        World::at_slot(AFTER)
            .with_encrypted_value_account(&caught_up)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("once the observation catches up, the unchanged request is authorized");
}
