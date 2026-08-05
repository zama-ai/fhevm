//! Handle binding: which handle of a lineage a subject may decrypt, and on what evidence.
//!
//! Two modes, chosen per entry. Without a proof, the claim is "this handle is the live one and
//! I am a current member". With a proof, the claim is "this handle was mine when it was live,
//! and here is the sealed leaf that says so" — a claim that survives both later supersession
//! and later membership changes, because the subject is bound inside the leaf rather than read
//! from the current set.
//!
//! The file is dominated by substitutions of the leaf commitment: another lineage, another
//! subject, another handle, another position. Each of them is a proof that verifies against
//! *something*, and the question is whether the code checks that it verifies against the thing
//! that was asked. A leaf commitment binds four values, and a test per value is the only way to
//! know all four are in the preimage.
//!
//! Two accepts carry as much weight as the rejections:
//!
//! * a proof built against an older leaf count, whose peak no later append merged, still
//!   verifies and must be taken — the alternative is rejecting valid proofs after every append;
//! * a proof that verifies is taken whatever leaf count the request claims for it. The claimed
//!   count is not an input to the rule; it is an input to the diagnosis of a rule that already
//!   failed. That is why [`check_handle_binding`] cannot see it.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::{
        HandleBindingFailure, InclusionAction, check_handle_binding, classify_inclusion_failure,
    },
    lineage::{ResolvedLineage, resolve_lineage},
    pipeline::{AuthorizationContext, authorize_request},
    request::{
        AccessEvidence, MAX_ACCESS_PROOF_SIBLINGS, RequestFormError, SolanaUserDecryptRequest,
    },
    snapshot::SnapshotKeys,
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_support::*;
use zama_solana_acl::{MmrProof, historical_access_leaf_commitment};

/// Resolves a lineage the way the pipeline does, so the binding rules are exercised against a
/// validated account rather than a hand-made value.
fn resolved(lineage: &LineageFixture) -> ResolvedLineage {
    let world = World::at_slot(1).with_lineage(lineage);
    let snapshot = world
        .read(&SnapshotKeys::new([lineage.account_key]))
        .expect("the world reads");
    resolve_lineage(&snapshot, PROGRAM_ID, lineage.value_key())
        .expect("the fixture lineage resolves")
}

fn current() -> AccessEvidence {
    AccessEvidence::Current
}

fn historical(proof: MmrProof) -> AccessEvidence {
    AccessEvidence::Historical(proof)
}

fn context<'a>(
    deployment: &'a kms_worker::core::solana::deployment::DeploymentIdentity,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    }
}

// ---------------------------------------------------------------------------
// Current access
// ---------------------------------------------------------------------------

/// The reference current-mode case: the named handle is live and the subject is a member.
#[test]
fn a_live_handle_authorizes_a_current_member() {
    let subject = Wallet::new(1).pubkey();
    let live = handle(0x10, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[subject]);

    check_handle_binding(&resolved(&lineage), live, subject, &current())
        .expect("a member decrypts the live handle");
}

/// A handle that has been superseded fails, and is not quietly replaced by whatever is live
/// now. Returning the live value would answer a question the requester did not ask, and the
/// response could no longer be tied back to the request the client built.
#[test]
fn a_superseded_handle_is_not_replaced_by_the_live_one() {
    let subject = Wallet::new(1).pubkey();
    let superseded = handle(0x11, FHE_TYPE_UINT64);
    let live = handle(0x12, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[subject]);
    lineage.supersede(live);

    let failure = check_handle_binding(&resolved(&lineage), superseded, subject, &current())
        .expect_err("a superseded handle is not current");

    assert!(matches!(
        failure,
        HandleBindingFailure::Superseded { requested, current }
            if requested == superseded && current == live
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal,
        "the live handle will not become the requested one; the historical path is the way \
         forward, not a retry"
    );
}

/// Current mode requires membership of the current subject set, and the set is the account's,
/// not the request's.
#[test]
fn a_non_member_is_rejected_in_current_mode() {
    let member = Wallet::new(1).pubkey();
    let stranger = Wallet::new(2).pubkey();
    let live = handle(0x13, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[member]);

    let failure = check_handle_binding(&resolved(&lineage), live, stranger, &current())
        .expect_err("a non-member decrypts nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::NotAMember { subject } if subject == stranger
    ));
}

// ---------------------------------------------------------------------------
// Historical access
// ---------------------------------------------------------------------------

/// Supersession seals a leaf for each subject that held the handle. That leaf is what makes the
/// same handle reachable afterwards, under the same permit, with no new signature.
#[test]
fn a_sealed_leaf_authorizes_its_subject_after_supersession() {
    let subject = Wallet::new(1).pubkey();
    let superseded = handle(0x20, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[subject]);
    lineage.supersede(handle(0x21, FHE_TYPE_UINT64));

    check_handle_binding(
        &resolved(&lineage),
        superseded,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect("the sealed leaf authorizes the subject it names");
}

/// The subject is bound inside the leaf, so historical access outlives membership: a rotation
/// that removes the subject from the current set does not reach leaves that were already
/// sealed.
#[test]
fn historical_access_survives_a_membership_rotation() {
    let subject = Wallet::new(1).pubkey();
    let stranger = Wallet::new(2).pubkey();
    let superseded = handle(0x22, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[subject]);
    lineage.supersede(handle(0x23, FHE_TYPE_UINT64));
    lineage.rotate_subjects(&[stranger]);
    let resolved = resolved(&lineage);

    check_handle_binding(
        &resolved,
        superseded,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect("a sealed leaf is not revoked by a later rotation");

    assert!(
        check_handle_binding(&resolved, superseded, subject, &current()).is_err(),
        "the same subject has no current standing, which is what makes the accept above about \
         the leaf and not about membership"
    );
}

/// An append merges only some peaks. A proof built before an append that left its peak alone
/// still verifies against the live peaks, and verifying is the whole test — age is not a
/// failure. Rejecting on age would fail valid proofs after every write to the lineage.
#[test]
fn a_proof_whose_peak_no_later_append_merged_still_verifies() {
    let subject = Wallet::new(1).pubkey();
    let first = handle(0x24, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(first, &[subject]);
    lineage.supersede(handle(0x25, FHE_TYPE_UINT64));
    lineage.supersede(handle(0x26, FHE_TYPE_UINT64));
    assert_eq!(lineage.lineage.leaf_count, 2);
    let proof = lineage.proof(0);
    lineage.supersede(handle(0x27, FHE_TYPE_UINT64));
    assert_eq!(lineage.lineage.leaf_count, 3);

    check_handle_binding(&resolved(&lineage), first, subject, &historical(proof))
        .expect("a proof that verifies against the observed peaks is accepted");
}

/// An append that merges the proof's peak invalidates it permanently: the sibling path it
/// carries no longer exists in the tree. Nothing but a rebuilt proof will do.
#[test]
fn a_proof_invalidated_by_a_merging_append_no_longer_verifies() {
    let subject = Wallet::new(1).pubkey();
    let third = handle(0x30, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(handle(0x2e, FHE_TYPE_UINT64), &[subject]);
    lineage.supersede(handle(0x2f, FHE_TYPE_UINT64));
    lineage.supersede(third);
    lineage.supersede(handle(0x31, FHE_TYPE_UINT64));
    assert_eq!(lineage.lineage.leaf_count, 3);
    let proof = lineage.proof(2);
    lineage.supersede(handle(0x32, FHE_TYPE_UINT64));
    assert_eq!(lineage.lineage.leaf_count, 4);

    let failure = check_handle_binding(&resolved(&lineage), third, subject, &historical(proof))
        .expect_err("a merged-away sibling path does not verify");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { live_leaf_count: 4 }
    ));
}

/// A leaf position the lineage does not have is refused before any hashing: there is nothing
/// for the proof to be a proof of.
#[test]
fn a_leaf_index_at_or_above_the_observed_count_is_rejected() {
    let subject = Wallet::new(1).pubkey();
    let superseded = handle(0x33, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[subject]);
    lineage.supersede(handle(0x34, FHE_TYPE_UINT64));
    let beyond = MmrProof {
        leaf_index: lineage.lineage.leaf_count,
        siblings: Vec::new(),
    };

    let failure = check_handle_binding(
        &resolved(&lineage),
        superseded,
        subject,
        &historical(beyond),
    )
    .expect_err("a position the lineage does not have proves nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index: 1,
            leaf_count: 1
        }
    ));
}

/// A tampered sibling path does not verify.
#[test]
fn a_proof_with_tampered_siblings_is_rejected() {
    let subject = Wallet::new(1).pubkey();
    let superseded = handle(0x35, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[subject, Wallet::new(2).pubkey()]);
    lineage.supersede(handle(0x36, FHE_TYPE_UINT64));
    let mut proof = lineage.proof(0);
    assert!(
        !proof.siblings.is_empty(),
        "the fixture must produce a proof with a sibling to tamper with"
    );
    proof.siblings[0][0] ^= 0xff;

    let failure =
        check_handle_binding(&resolved(&lineage), superseded, subject, &historical(proof))
            .expect_err("a tampered path does not verify");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

/// The four substitutions of the leaf commitment. Each seals a leaf that is genuine in every
/// respect except one, and each must fail: together they are the evidence that the lineage
/// account, the position, the handle and the subject are all inside the committed preimage.
#[test]
fn a_leaf_commitment_for_another_lineage_does_not_authorize() {
    let subject = Wallet::new(1).pubkey();
    let target = handle(0x40, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(target, &[subject]);
    let other_account: SolanaPubkeyBytes = [0x99; 32];
    // A leaf that would be valid if this lineage lived at another address.
    lineage.append(historical_access_leaf_commitment(
        other_account,
        0,
        target,
        subject,
    ));

    let failure = check_handle_binding(
        &resolved(&lineage),
        target,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect_err("a leaf sealed for another lineage account authorizes nothing here");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

#[test]
fn a_leaf_commitment_for_another_subject_does_not_authorize() {
    let subject = Wallet::new(1).pubkey();
    let other_subject = Wallet::new(2).pubkey();
    let target = handle(0x41, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(target, &[subject]);
    let account = lineage.account_key;
    lineage.append(historical_access_leaf_commitment(
        account,
        0,
        target,
        other_subject,
    ));

    let failure = check_handle_binding(
        &resolved(&lineage),
        target,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect_err("a leaf sealed for another subject authorizes only that subject");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

#[test]
fn a_leaf_commitment_for_another_handle_does_not_authorize() {
    let subject = Wallet::new(1).pubkey();
    let target = handle(0x42, FHE_TYPE_UINT64);
    let other_handle = handle(0x43, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(target, &[subject]);
    let account = lineage.account_key;
    lineage.append(historical_access_leaf_commitment(
        account,
        0,
        other_handle,
        subject,
    ));

    let failure = check_handle_binding(
        &resolved(&lineage),
        target,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect_err("a leaf sealed for another handle authorizes only that handle");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

#[test]
fn a_leaf_commitment_at_another_position_does_not_authorize() {
    let subject = Wallet::new(1).pubkey();
    let target = handle(0x44, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(target, &[subject]);
    let account = lineage.account_key;
    // Sealed as though it were the second leaf, but appended as the first.
    lineage.append(historical_access_leaf_commitment(
        account, 1, target, subject,
    ));

    let failure = check_handle_binding(
        &resolved(&lineage),
        target,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect_err("a leaf whose committed position is not its own authorizes nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

/// Public decryptability is a separate flow with its own leaf domain. A public-decrypt leaf must
/// not double as evidence that some subject held the handle: it says nothing about any subject.
#[test]
fn a_public_decrypt_leaf_does_not_authorize_historical_access() {
    let subject = Wallet::new(1).pubkey();
    let target = handle(0x45, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(target, &[subject]);
    lineage.mark_public();
    lineage.supersede(handle(0x46, FHE_TYPE_UINT64));

    let failure = check_handle_binding(
        &resolved(&lineage),
        target,
        subject,
        &historical(lineage.proof(0)),
    )
    .expect_err("a public-decrypt leaf is not a historical-access leaf");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

// ---------------------------------------------------------------------------
// Classification of a failed inclusion
// ---------------------------------------------------------------------------

/// Behind the observed count, the proof predates an append that merged its peak: it will never
/// verify again, and the client must rebuild it.
#[test]
fn an_inclusion_failure_behind_the_observed_count_asks_for_a_rebuild() {
    assert_eq!(
        classify_inclusion_failure(3, 4),
        InclusionAction::RebuildProof
    );
    assert_eq!(
        InclusionAction::RebuildProof.class(),
        FailureClass::Terminal,
        "rebuilding is the client's move; repeating the request is not"
    );
}

/// At or ahead of the observed count, the proof service has seen more state than this
/// observation — it is ahead, or the two are on disagreeing confirmed forks. The same proof may
/// verify later, so the same request is worth repeating.
#[test]
fn an_inclusion_failure_at_or_ahead_of_the_observed_count_asks_for_a_retry() {
    assert_eq!(
        classify_inclusion_failure(4, 4),
        InclusionAction::RetryAtLaterSnapshot,
        "equal counts with differing peaks is fork disagreement, not staleness"
    );
    assert_eq!(
        classify_inclusion_failure(5, 4),
        InclusionAction::RetryAtLaterSnapshot
    );
    assert_eq!(
        InclusionAction::RetryAtLaterSnapshot.class(),
        FailureClass::Retryable
    );
}

/// The claimed count classifies a failure and never causes one. A request declaring a count
/// that matches nothing still authorizes, as long as its proof verifies against the observed
/// peaks — otherwise an unsigned request field would be deciding whether proofs are accepted.
#[tokio::test]
async fn a_verifying_proof_is_accepted_whatever_count_the_request_claims() {
    let wallet = Wallet::new(1);
    let superseded = handle(0x50, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[wallet.pubkey()]);
    lineage.supersede(handle(0x51, FHE_TYPE_UINT64));
    let proof = lineage.proof(0);
    let nonsense_count = 9_999;
    let request = RequestBuilder::new(&wallet)
        .historical(
            &lineage,
            superseded,
            wallet.pubkey(),
            &proof,
            nonsense_count,
        )
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&lineage)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a proof that verifies is accepted whatever count accompanies it");
}

/// When inclusion does fail, the request's claimed count is what turns it into an action — and
/// the failure reports both numbers, so a client can tell which of the two situations it is in
/// without guessing.
#[tokio::test]
async fn a_failed_inclusion_is_classified_by_the_claimed_count() {
    let wallet = Wallet::new(1);
    let third = handle(0x60, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(handle(0x5e, FHE_TYPE_UINT64), &[wallet.pubkey()]);
    lineage.supersede(handle(0x5f, FHE_TYPE_UINT64));
    lineage.supersede(third);
    lineage.supersede(handle(0x61, FHE_TYPE_UINT64));
    let stale_proof = lineage.proof(2);
    let claimed_count = lineage.lineage.leaf_count;
    lineage.supersede(handle(0x62, FHE_TYPE_UINT64));

    let request = RequestBuilder::new(&wallet)
        .historical(
            &lineage,
            third,
            wallet.pubkey(),
            &stale_proof,
            claimed_count,
        )
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&lineage)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("a merged-away proof does not verify");

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
        "expected a rebuild classification naming both counts, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
}

// ---------------------------------------------------------------------------
// The form of the access evidence
// ---------------------------------------------------------------------------

/// A proof followed by extra bytes is rejected, unlike an account followed by extra bytes.
/// The asymmetry is deliberate: an account grows and keeps its tail forever, while two byte
/// strings for one proof would give two implementations two different answers about whether
/// they are looking at the same request.
#[test]
fn an_access_proof_with_trailing_bytes_is_rejected() {
    let wallet = Wallet::new(1);
    let superseded = handle(0x70, FHE_TYPE_UINT64);
    let mut lineage = LineageFixture::new(superseded, &[wallet.pubkey()]);
    lineage.supersede(handle(0x71, FHE_TYPE_UINT64));
    let mut bytes = borsh::to_vec(&lineage.proof(0)).expect("a proof serializes");
    bytes.extend_from_slice(&[0xde, 0xad, 0xbe, 0xef]);
    let wire = RequestBuilder::new(&wallet)
        .entry(superseded, wallet.pubkey(), lineage.value_key(), 1, bytes)
        .wire();

    let failure = SolanaUserDecryptRequest::decode(&wire)
        .expect_err("a proof with a tail is not a well-formed proof");

    assert!(matches!(
        failure,
        RequestFormError::AccessProofTrailingBytes {
            index: 0,
            trailing: 4
        }
    ));
}

/// The sibling count is bounded by what the tree can produce, so an untrusted request cannot
/// make the decoder allocate an arbitrary list.
#[test]
fn an_access_proof_with_more_siblings_than_the_tree_can_produce_is_rejected() {
    let wallet = Wallet::new(1);
    let target = handle(0x72, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(target, &[wallet.pubkey()]);
    let oversized = MmrProof {
        leaf_index: 0,
        siblings: vec![[0; 32]; MAX_ACCESS_PROOF_SIBLINGS + 1],
    };
    let wire = RequestBuilder::new(&wallet)
        .historical(&lineage, target, wallet.pubkey(), &oversized, 1)
        .wire();

    let failure =
        SolanaUserDecryptRequest::decode(&wire).expect_err("an oversized sibling list is refused");

    assert!(matches!(
        failure,
        RequestFormError::AccessProofTooManySiblings { index: 0, .. }
    ));

    let at_cap = MmrProof {
        leaf_index: 0,
        siblings: vec![[0; 32]; MAX_ACCESS_PROOF_SIBLINGS],
    };
    let wire = RequestBuilder::new(&wallet)
        .historical(&lineage, target, wallet.pubkey(), &at_cap, 1)
        .wire();
    SolanaUserDecryptRequest::decode(&wire)
        .expect("the cap itself is a legal sibling count, so the bound is not off by one");
}

/// Bytes that are not a proof at all are refused as a form error, before any state is read.
#[test]
fn a_malformed_access_proof_is_rejected() {
    let wallet = Wallet::new(1);
    let target = handle(0x73, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(target, &[wallet.pubkey()]);
    let wire = RequestBuilder::new(&wallet)
        .entry(
            target,
            wallet.pubkey(),
            lineage.value_key(),
            1,
            vec![0xff, 0xff, 0xff],
        )
        .wire();

    let failure = SolanaUserDecryptRequest::decode(&wire)
        .expect_err("arbitrary bytes are not an access proof");

    assert!(matches!(
        failure,
        RequestFormError::AccessProofMalformed { index: 0 }
    ));
}

/// An empty access proof is not malformed: it is the current-access mode.
#[test]
fn an_empty_access_proof_selects_current_mode() {
    let wallet = Wallet::new(1);
    let live = handle(0x74, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&lineage, live)
        .typed();

    assert_eq!(
        request.handles()[0].access(),
        &AccessEvidence::Current,
        "no proof means the current-handle claim, not an absent field"
    );
}
