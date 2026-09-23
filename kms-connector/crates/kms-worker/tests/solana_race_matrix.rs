//! The race matrix: six state transitions, each asserted at both observation points.
//!
//! Every scenario is two observations and one request. The request is authorized against the
//! state before the transition and against the state after it, and both outcomes are stated. That
//! is the only honest way to talk about races in this system, because authorization is a function
//! of an observation rather than of a moment: there is no timing to arrange, no sleep to tune, and
//! no flake to chase.
//!
//! Each invocation uses its planned snapshot reads. The worker reauthorizes on later attempts.
//!
//! Two of the observers can disagree here — the chain this connector reads and the coprocessors'
//! record it fetches proofs from — and the last three rows are about that disagreement: a record
//! behind the chain whose proof still verifies, a record behind the chain whose proof does not,
//! and a record ahead of the chain.
//!
//! The rows:
//!
//! 1. the current handle is replaced;
//! 2. a key is allowed on a handle;
//! 3. the delegation is revoked;
//! 4. an append the record has not seen, which does not merge the proof's peak;
//! 5. an append the record has not seen, which merges the proof's peak;
//! 6. the record is ahead of this connector.
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;

mod solana_support;

use kms_worker::core::solana::{
    delegation::DelegationFailure, failure::AuthorizationFailure,
    handle_binding::HandleBindingFailure, pipeline::authorize_request,
};
use solana_support::*;

const BEFORE: u64 = 500;
const AFTER: u64 = 501;

/// What one authorization cost: account reads and leaf-record reads.
struct Reads {
    accounts: usize,
    proofs: usize,
}

/// Authorizes a request against one world and one leaf record, which every read of the scenario
/// sees.
async fn observe_with_record(
    world: World,
    record: ProofRecord,
    request: &SolanaUserDecryptionRequestV1,
) -> (Result<(), AuthorizationFailure>, Reads) {
    let proofs = ScriptedProofReader::constant(record);
    let reader = ScriptedReader::constant(world);
    let context = CONTEXT;
    let outcome = authorize_request(&reader, &proofs, context, request).await;
    (
        outcome,
        Reads {
            accounts: reader.call_count(),
            proofs: proofs.call_count(),
        },
    )
}

/// Authorizes a request against one world, with a leaf record in step with it.
async fn observe(
    world: World,
    request: &SolanaUserDecryptionRequestV1,
) -> (Result<(), AuthorizationFailure>, Reads) {
    let record = world.record();
    observe_with_record(world, record, request).await
}

/// Each authorization invocation uses exactly the planned account and proof reads.
fn assert_planned_reads(_authorized: &(), reads: Reads, expected_account_reads: usize) {
    assert_eq!(
        reads.accounts, expected_account_reads,
        "one authorization attempt uses only its planned account reads"
    );
    assert_eq!(
        reads.proofs, 1,
        "one proof read suffices when the record matches the deciding snapshot"
    );
}

// ---------------------------------------------------------------------------
// 1. A current handle being replaced
// ---------------------------------------------------------------------------

/// A handle update does not race a request for the handle it replaces: the allow leaf names the
/// handle, and the update seals nothing about it. Reauthorizing the same request after the
/// update still succeeds.
#[tokio::test]
async fn a_handle_update_does_not_reach_a_request_for_the_replaced_handle() {
    let signer = Wallet::new(1);
    let named = handle(0x10, FHE_TYPE_UINT64);
    let before = EncryptedStoreFixture::allowing(named, signer.pubkey());
    let mut after = before.clone();
    after.update(handle(0x11, FHE_TYPE_UINT64));
    let request = RequestBuilder::new(&signer).direct(&before, named).typed();

    let (accepted, reads) = observe(
        World::running_at_slot(BEFORE)
            .with_encrypted_store(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    assert_planned_reads(
        &accepted.expect("before the update the leaf authorizes"),
        reads,
        1,
    );

    let (outcome, _) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("after the update the same leaf still authorizes the same handle");
}

/// What the update does change is which handle a key allowed on the *new* one can decrypt: the
/// new handle needs its own leaf, and until it is sealed a request for it is refused terminally at
/// that observation.
#[tokio::test]
async fn a_handle_update_leaves_the_new_handle_unallowed_until_a_leaf_is_sealed() {
    let signer = Wallet::new(1);
    let replacement = handle(0x13, FHE_TYPE_UINT64);
    let mut after = EncryptedStoreFixture::allowing(handle(0x12, FHE_TYPE_UINT64), signer.pubkey());
    after.update(replacement);
    let request = RequestBuilder::new(&signer)
        .direct(&after, replacement)
        .typed();

    let (outcome, _) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    let failure = outcome.expect_err("no leaf names the new handle yet");
    assert!(matches!(
        failure,
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NoLeaf { .. }
        }
    ));
    assert!(!failure.is_recoverable());
}

// ---------------------------------------------------------------------------
// 2. A key being allowed
// ---------------------------------------------------------------------------

/// A request racing the allow that would authorize it is refused at the earlier observation and
/// authorized at the later one. The refusal is terminal *for that observation*: the record has the
/// chain's history and no leaf, and nothing in the request can change that; what changes it is the
/// application sealing the leaf, which is a new state and a new observation.
#[tokio::test]
async fn an_allow_authorizes_a_request_only_from_the_observation_that_holds_it() {
    let signer = Wallet::new(1);
    let live = handle(0x20, FHE_TYPE_UINT64);
    let mut before = EncryptedStoreFixture::new(live);
    before.allow(Wallet::new(9).pubkey());
    let mut after = before.clone();
    after.allow(signer.pubkey());
    let request = RequestBuilder::new(&signer).direct(&before, live).typed();

    let (outcome, _) = observe(
        World::running_at_slot(BEFORE)
            .with_encrypted_store(&before)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;
    let failure = outcome.expect_err("before the allow the signer holds no leaf");
    assert!(matches!(
        failure,
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NoLeaf { .. }
        }
    ));
    assert!(!failure.is_recoverable());

    let (outcome, _) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&after)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("after the allow the same request authorizes");
}

// ---------------------------------------------------------------------------
// 3. Delegation revocation
// ---------------------------------------------------------------------------

/// A delegated entry racing a revocation fails at the later observation. The revocation reaches
/// the next authorization attempt, including a poll of the same request.
#[tokio::test]
async fn delegation_revocation_rejects_its_entry_at_the_later_observation() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let granted = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), BEFORE);
    let mut revoked = granted;
    revoked.revoked = true;
    revoked.last_update_slot = AFTER;
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (accepted, reads) = observe(
        World::running_at_slot(BEFORE)
            .with_encrypted_store(&encrypted_store)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&granted),
        &request,
    )
    .await;
    assert_planned_reads(
        &accepted.expect("before the revocation the delegation is live"),
        reads,
        2,
    );

    let (outcome, _) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&encrypted_store)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&revoked),
        &request,
    )
    .await;

    let failure = outcome.expect_err("after revocation the exact delegation is dead");
    assert!(failure.is_recoverable());
    assert!(
        matches!(failure, AuthorizationFailure::Delegation { index:0, source: DelegationFailure::NoLiveGrant { exact, wildcard } }
        if matches!(*exact, DelegationFailure::Revoked) && matches!(*wildcard, DelegationFailure::Absent { .. }))
    );
}

/// The direct branch is untouched by a delegation revocation: the signer's own leaves are not
/// held on anybody's grant.
#[tokio::test]
async fn delegation_revocation_does_not_touch_the_direct_branch() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let own = handle(0x31, FHE_TYPE_UINT64);
    let own_encrypted_store = EncryptedStoreFixture::allowing(own, signer.pubkey());
    let mut revoked = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), BEFORE);
    revoked.revoked = true;
    let request = RequestBuilder::new(&signer)
        .direct(&own_encrypted_store, own)
        .typed();

    let (outcome, reads) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&own_encrypted_store)
            .with_watermark(signer.pubkey(), 0)
            .with_delegation(&revoked),
        &request,
    )
    .await;

    outcome.expect("a revoked delegation says nothing about the signer's own leaves");
    assert_eq!(
        reads.accounts, 1,
        "a direct-only request does not even read the delegation record"
    );
}

// ---------------------------------------------------------------------------
// 4. An append the record has not seen, which does not merge the proof's peak
// ---------------------------------------------------------------------------

/// The record is behind the chain by one leaf, and the append it missed left the proof's peak
/// alone. The proof it serves still verifies, so the request is authorized without a refresh.
/// Rejecting on age would break every request that raced any write.
#[tokio::test]
async fn a_record_behind_by_a_non_merging_append_still_authorizes() {
    let signer = Wallet::new(1);
    let sealed = handle(0x40, FHE_TYPE_UINT64);
    let mut record_state = EncryptedStoreFixture::allowing(sealed, signer.pubkey());
    record_state.allow(Wallet::new(2).pubkey());
    assert_eq!(record_state.encrypted_store.leaf_count, 2);
    let mut chain_state = record_state.clone();
    chain_state.allow(Wallet::new(3).pubkey());
    assert_eq!(chain_state.encrypted_store.leaf_count, 3);
    let request = RequestBuilder::new(&signer)
        .direct(&chain_state, sealed)
        .typed();

    let (outcome, reads) = observe_with_record(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&chain_state)
            .with_watermark(signer.pubkey(), 0),
        ProofRecord::of(&[&record_state]),
        &request,
    )
    .await;

    outcome.expect("the proof of the first leaf still reaches the two-leaf peak");
    assert_eq!(
        reads.proofs, 1,
        "a valid proof needs no refresh even when its record is behind the chain"
    );
}

// ---------------------------------------------------------------------------
// 5. An append the record has not seen, which merges the proof's peak
// ---------------------------------------------------------------------------

/// The record is behind the chain by the one append that merged the proof's peak. The sibling
/// path it serves no longer reaches any peak the chain holds: the request is refused retryably
/// after the repeat, and authorized once the record has caught up.
#[tokio::test]
async fn a_record_behind_by_a_merging_append_is_retryable_and_then_authorized() {
    let signer = Wallet::new(1);
    let sealed = handle(0x50, FHE_TYPE_UINT64);
    let record_state = EncryptedStoreFixture::allowing(sealed, signer.pubkey());
    assert_eq!(record_state.encrypted_store.leaf_count, 1);
    let mut chain_state = record_state.clone();
    chain_state.allow(Wallet::new(2).pubkey());
    assert_eq!(chain_state.encrypted_store.leaf_count, 2);
    let request = RequestBuilder::new(&signer)
        .direct(&chain_state, sealed)
        .typed();
    let world = World::running_at_slot(AFTER)
        .with_encrypted_store(&chain_state)
        .with_watermark(signer.pubkey(), 0);

    let (outcome, reads) =
        observe_with_record(world.clone(), ProofRecord::of(&[&record_state]), &request).await;

    let failure = outcome.expect_err("the lone-leaf peak was merged away");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::ProofDoesNotVerify {
                    record_leaf_count: 1,
                    live_leaf_count: 2,
                }
            }
        ),
        "expected a proof that does not verify, got {failure}"
    );
    assert!(failure.is_recoverable());
    assert_eq!(
        reads.proofs, 2,
        "a proof that does not verify gets one refresh against the same observation"
    );

    let (outcome, _) = observe(world, &request).await;

    outcome.expect("once the record has caught up, the unchanged request is authorized");
}

// ---------------------------------------------------------------------------
// 6. The record ahead of this connector
// ---------------------------------------------------------------------------

/// The mirror image of the rows above, and the reason the classification exists at all. Here the
/// record has sealed a leaf this observation does not have yet: the proof names a position past
/// the observed leaf count, and the request is worth repeating — at the later observation, once
/// the chain view has caught up, the same request is authorized.
///
/// Note which direction the transition runs in this row: the state does not change under the
/// request, the observer catches up to it.
#[tokio::test]
async fn a_record_ahead_of_the_observation_is_retryable_and_then_authorized() {
    let signer = Wallet::new(1);
    let live = handle(0x60, FHE_TYPE_UINT64);
    let behind = EncryptedStoreFixture::new(live);
    let mut caught_up = behind.clone();
    caught_up.allow(signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .direct(&caught_up, live)
        .typed();

    let (outcome, reads) = observe_with_record(
        World::running_at_slot(BEFORE)
            .with_encrypted_store(&behind)
            .with_watermark(signer.pubkey(), 0),
        ProofRecord::of(&[&caught_up]),
        &request,
    )
    .await;

    let failure =
        outcome.expect_err("a proof of state this observation has not seen cannot verify");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::LeafIndexOutOfRange {
                    leaf_index: 0,
                    leaf_count: 0,
                }
            }
        ),
        "expected a position the observation does not have, got {failure}"
    );
    assert!(failure.is_recoverable());
    assert_eq!(
        reads.proofs, 2,
        "an out-of-range leaf is retryable and gets one refresh against the same observation"
    );

    let (outcome, _) = observe(
        World::running_at_slot(AFTER)
            .with_encrypted_store(&caught_up)
            .with_watermark(signer.pubkey(), 0),
        &request,
    )
    .await;

    outcome.expect("once the observation catches up, the unchanged request is authorized");
}
