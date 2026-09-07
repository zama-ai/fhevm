//! Handle binding: which key may decrypt which handle of an encrypted value account, and on what
//! evidence.
//!
//! The evidence is always the same: an `Allowed(key, handle)` leaf sealed in the account's MMR,
//! located by the coprocessors' leaf record and proven against the peaks this connector observed
//! on chain. There is no live membership set to consult and no current-handle rule: a leaf, once
//! sealed, names its handle and its key forever, and a later write to the account changes which
//! handle is current without touching what was allowed on the old one.
//!
//! The file is dominated by substitutions of the leaf commitment: another encrypted value account,
//! another key, another handle, another position, another leaf kind. Each of them is a proof that
//! verifies against *something*, and the question is whether the code checks that it verifies
//! against the thing that was asked. A leaf commitment binds four values and a domain prefix, and
//! a test per value is the only way to know all of them are in the preimage.
//!
//! The record's answer is classified only once it carries no proof, and the classification is the
//! second half of this file: a record with the chain's history and no leaf is terminal; a record
//! behind the chain, a record that does not know the account yet, and a proof from a record ahead
//! of this observation are disagreements to retry; a record whose history has a gap is terminal.
//! Two accepts carry as much weight as the rejections: a proof from a record behind the chain still
//! verifies when the append that followed left its peak alone, and it must be taken.

mod solana_support;

use kms_worker::core::solana::{
    encrypted_value_account::{ResolvedEncryptedValueAccount, resolve_encrypted_value_account},
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::{HandleBindingFailure, check_handle_binding, check_public_binding},
    pipeline::{AuthorizationContext, authorize_request},
    proof::{LeafKind, LeafProofOutcome, LeafQuery},
    snapshot::SnapshotKeys,
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_support::*;
use zama_solana_acl::{historical_access_leaf_commitment, public_decrypt_leaf_commitment};

/// Resolves an encrypted value account the way the pipeline does, so the binding rules are
/// exercised against a validated account rather than a hand-made value.
fn resolved(
    encrypted_value_account: &EncryptedValueAccountFixture,
) -> ResolvedEncryptedValueAccount {
    let world = World::running_at_slot(1).with_encrypted_value_account(encrypted_value_account);
    let snapshot = world
        .read(&SnapshotKeys::new([encrypted_value_account.account_key]))
        .expect("the world reads");
    resolve_encrypted_value_account(&snapshot, PROGRAM_ID, encrypted_value_account.account_key)
        .expect("the fixture encrypted value account resolves")
}

/// The record's answer for `key` on `handle` when it has sealed exactly this account's leaves.
fn answer(
    encrypted_value_account: &EncryptedValueAccountFixture,
    handle: [u8; 32],
    key: SolanaPubkeyBytes,
) -> LeafProofOutcome {
    encrypted_value_account.outcome(&encrypted_value_account.allowed_query(handle, key))
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
// The allow leaf
// ---------------------------------------------------------------------------

/// The reference case: the key was allowed on the handle, the record serves the leaf, the proof
/// verifies against the account's peaks.
#[test]
fn an_allow_leaf_the_record_serves_binds_the_handle_to_its_key() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);

    check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &answer(&encrypted_value_account, live, key),
    )
    .expect("an allowed key decrypts its handle");
}

/// A write replaces the current handle and seals nothing about the old one, and the old one stays
/// decryptable by whoever was allowed on it: the leaf names the handle, not the account's current
/// state. This is what makes "historical access" the ordinary case rather than a mode.
#[test]
fn a_leaf_outlives_the_handle_it_names_being_replaced() {
    let key = Wallet::new(1).pubkey();
    let sealed = handle(0x11, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::allowing(sealed, key);
    encrypted_value_account.update(handle(0x12, FHE_TYPE_UINT64));
    encrypted_value_account.allow(key);

    check_handle_binding(
        &resolved(&encrypted_value_account),
        sealed,
        key,
        &answer(&encrypted_value_account, sealed, key),
    )
    .expect("the old handle is still the allowed key's to decrypt");
}

/// Being allowed on the account's current handle says nothing about a handle that was never
/// allowed to the key. Access is per handle, and the account is only where the handles live.
#[test]
fn a_leaf_on_one_handle_does_not_bind_another() {
    let key = Wallet::new(1).pubkey();
    let allowed = handle(0x13, FHE_TYPE_UINT64);
    let never_allowed = handle(0x14, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::allowing(allowed, key);
    encrypted_value_account.update(never_allowed);

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        never_allowed,
        key,
        &answer(&encrypted_value_account, never_allowed, key),
    )
    .expect_err("the current handle was never allowed to the key");

    assert!(matches!(
        failure,
        HandleBindingFailure::NoLeaf {
            record_leaf_count: 1,
            live_leaf_count: 1
        }
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

/// Several keys allowed on one handle each hold their own leaf, and each is proven on its own.
#[test]
fn each_allowed_key_holds_its_own_leaf() {
    let first = Wallet::new(1).pubkey();
    let second = Wallet::new(2).pubkey();
    let live = handle(0x15, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::allowing(live, first);
    encrypted_value_account.allow(second);
    let account = resolved(&encrypted_value_account);

    check_handle_binding(
        &account,
        live,
        first,
        &answer(&encrypted_value_account, live, first),
    )
    .expect("the first key's leaf verifies");
    check_handle_binding(
        &account,
        live,
        second,
        &answer(&encrypted_value_account, live, second),
    )
    .expect("the second key's leaf verifies");
}

// ---------------------------------------------------------------------------
// The four values of the commitment, and its domain
// ---------------------------------------------------------------------------

/// Seals a leaf whose commitment is genuine except in one value, has the record serve it as the
/// proof of the genuine query, and returns the verdict. Every one of these must be a proof that
/// does not verify: the leaf is in the MMR, so the sibling path is right, and only the commitment
/// is wrong — which is exactly the case the preimage check exists for.
fn verdict_on_substituted_leaf(
    substitute: impl Fn(SolanaPubkeyBytes, [u8; 32], SolanaPubkeyBytes) -> [u8; 32],
) -> Result<(), HandleBindingFailure> {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x20, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::new(live);
    let commitment = substitute(encrypted_value_account.account_key, live, key);
    encrypted_value_account.append(encrypted_value_account.allowed_query(live, key), commitment);

    check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &answer(&encrypted_value_account, live, key),
    )
}

fn assert_does_not_verify(verdict: Result<(), HandleBindingFailure>) {
    let failure = verdict.expect_err("a substituted commitment must not verify");
    assert!(
        matches!(failure, HandleBindingFailure::ProofDoesNotVerify { .. }),
        "expected a proof that does not verify, got {failure}"
    );
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Retryable
    );
}

/// A leaf sealed for another encrypted value account authorizes nothing here.
#[test]
fn a_leaf_committing_to_another_encrypted_value_account_does_not_verify() {
    assert_does_not_verify(verdict_on_substituted_leaf(|_, handle, key| {
        historical_access_leaf_commitment([0x99; 32], 0, handle, key)
    }));
}

/// A leaf sealed for another key authorizes only that key.
#[test]
fn a_leaf_committing_to_another_key_does_not_verify() {
    assert_does_not_verify(verdict_on_substituted_leaf(|account, handle, _| {
        historical_access_leaf_commitment(account, 0, handle, Wallet::new(9).pubkey())
    }));
}

/// A leaf sealed for another handle authorizes only that handle.
#[test]
fn a_leaf_committing_to_another_handle_does_not_verify() {
    assert_does_not_verify(verdict_on_substituted_leaf(|account, _, key| {
        historical_access_leaf_commitment(account, 0, handle(0xbe, FHE_TYPE_UINT64), key)
    }));
}

/// A leaf whose committed position is not the position it occupies authorizes nothing.
#[test]
fn a_leaf_committing_to_another_position_does_not_verify() {
    assert_does_not_verify(verdict_on_substituted_leaf(|account, handle, key| {
        historical_access_leaf_commitment(account, 1, handle, key)
    }));
}

/// Public decryptability is a separate leaf domain. Its leaf says nothing about any key and must
/// not double as evidence that one was allowed.
#[test]
fn a_public_decrypt_leaf_does_not_bind_a_key() {
    assert_does_not_verify(verdict_on_substituted_leaf(|account, handle, _| {
        public_decrypt_leaf_commitment(account, 0, handle)
    }));
}

/// The mirror: an allow leaf does not make a handle public.
#[test]
fn an_allow_leaf_does_not_prove_public_ness() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);

    let failure = check_public_binding(
        &resolved(&encrypted_value_account),
        live,
        &answer(&encrypted_value_account, live, key),
    )
    .expect_err("an allow leaf is not a public leaf");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify { .. }
    ));
}

/// And a public leaf the record serves does prove public-ness.
#[test]
fn a_public_decrypt_leaf_the_record_serves_proves_public_ness() {
    let live = handle(0x22, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::new(live);
    encrypted_value_account.mark_public();

    check_public_binding(
        &resolved(&encrypted_value_account),
        live,
        &encrypted_value_account.outcome(&encrypted_value_account.public_query(live)),
    )
    .expect("a public leaf proven against the peaks makes the handle public");
}

/// A sibling path with one hash altered does not reach the peak. The record supplies the path
/// and the chain decides.
#[test]
fn a_tampered_sibling_path_does_not_verify() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x23, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);
    encrypted_value_account.allow(Wallet::new(2).pubkey());
    let LeafProofOutcome::Found {
        leaf_index,
        leaf_count,
        mut siblings,
    } = answer(&encrypted_value_account, live, key)
    else {
        panic!("the record holds the leaf");
    };
    assert!(!siblings.is_empty(), "two leaves give the first a sibling");
    siblings[0][0] ^= 1;

    assert_does_not_verify(check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        },
    ));
}

// ---------------------------------------------------------------------------
// The record's answer when it carries no proof
// ---------------------------------------------------------------------------

/// A record that has sealed at least as much history as the chain shows and has no leaf: the
/// permission was never granted, and repeating the request changes nothing.
#[test]
fn no_leaf_in_a_record_with_the_chains_history_is_terminal() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_value_account =
        EncryptedValueAccountFixture::allowing(live, Wallet::new(9).pubkey());

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::NotFound { leaf_count: 1 },
    )
    .expect_err("nobody allowed this key");

    assert!(matches!(
        failure,
        HandleBindingFailure::NoLeaf {
            record_leaf_count: 1,
            live_leaf_count: 1
        }
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

/// A record ahead of the chain — more history sealed than this observation shows — and still no
/// leaf is the same terminal answer: whatever the chain adds next, the record has already seen
/// past it.
#[test]
fn no_leaf_in_a_record_ahead_of_the_chain_is_terminal() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_value_account =
        EncryptedValueAccountFixture::allowing(live, Wallet::new(9).pubkey());

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::NotFound { leaf_count: 3 },
    )
    .expect_err("nobody allowed this key");

    assert!(matches!(failure, HandleBindingFailure::NoLeaf { .. }));
}

/// A record behind the chain with no leaf has not caught up to the append that may hold it. The
/// answer says nothing yet, and the request is worth repeating.
#[test]
fn no_leaf_in_a_record_behind_the_chain_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x32, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::NotFound { leaf_count: 0 },
    )
    .expect_err("a record behind the chain decides nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofRecordBehind {
            record_leaf_count: 0,
            live_leaf_count: 1
        }
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Retryable
    );
}

/// An account the chain has and the record has never seen is a record that has not indexed the
/// account's creation yet.
#[test]
fn an_account_unknown_to_the_record_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x33, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::UnknownAccount,
    )
    .expect_err("a record that does not know the account decides nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::AccountUnknownToProofRecord
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Retryable
    );
}

/// A record whose history for the account has a gap can answer nothing about it until rebuilt,
/// and no retry within a request's budget rebuilds it.
#[test]
fn an_incomplete_history_is_terminal() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x34, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_value_account),
        live,
        key,
        &LeafProofOutcome::HistoryIncomplete,
    )
    .expect_err("a broken record proves nothing");

    assert!(matches!(failure, HandleBindingFailure::HistoryIncomplete));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

// ---------------------------------------------------------------------------
// A proof and the chain's history
// ---------------------------------------------------------------------------

/// A proof built against fewer leaves than the chain shows still verifies when the appends that
/// followed left its peak alone — and it must be taken. Rejecting on age would fail every request
/// that raced any write.
#[test]
fn a_proof_from_a_record_behind_the_chain_still_verifies_when_its_peak_survived() {
    let key = Wallet::new(1).pubkey();
    let sealed = handle(0x40, FHE_TYPE_UINT64);
    let mut before = EncryptedValueAccountFixture::allowing(sealed, key);
    before.allow(Wallet::new(2).pubkey());
    assert_eq!(before.encrypted_value.leaf_count, 2);
    let proof_from_behind = answer(&before, sealed, key);
    let mut after = before.clone();
    after.allow(Wallet::new(3).pubkey());
    assert_eq!(after.encrypted_value.leaf_count, 3);

    check_handle_binding(&resolved(&after), sealed, key, &proof_from_behind)
        .expect("the third leaf is its own peak; the proof of the first still reaches the second");
}

/// The proof case that cannot be accepted: the append merged the proof's peak, so the sibling
/// path the record served no longer reaches any peak the chain holds. Retryable — the record will
/// serve a longer path once it catches up.
#[test]
fn a_proof_whose_peak_was_merged_does_not_verify_and_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let sealed = handle(0x41, FHE_TYPE_UINT64);
    let mut before = EncryptedValueAccountFixture::allowing(sealed, key);
    assert_eq!(before.encrypted_value.leaf_count, 1);
    let proof_from_behind = answer(&before, sealed, key);
    let mut after = before.clone();
    after.allow(Wallet::new(2).pubkey());
    assert_eq!(after.encrypted_value.leaf_count, 2);
    before.allow(Wallet::new(9).pubkey());

    let failure = check_handle_binding(&resolved(&after), sealed, key, &proof_from_behind)
        .expect_err("the lone-leaf peak was merged into a two-leaf mountain");

    assert!(matches!(
        failure,
        HandleBindingFailure::ProofDoesNotVerify {
            record_leaf_count: 1,
            live_leaf_count: 2
        }
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Retryable
    );
}

/// A record ahead of this observation may serve a leaf at a position the account does not have
/// yet. There is nothing for the proof to be a proof of, and it is refused before any hashing.
#[test]
fn a_leaf_position_the_account_does_not_have_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x42, FHE_TYPE_UINT64);
    let behind = EncryptedValueAccountFixture::new(live);
    let mut ahead = behind.clone();
    ahead.allow(key);

    let failure = check_handle_binding(&resolved(&behind), live, key, &answer(&ahead, live, key))
        .expect_err("the observation has no leaf zero yet");

    assert!(matches!(
        failure,
        HandleBindingFailure::LeafIndexOutOfRange {
            leaf_index: 0,
            leaf_count: 0
        }
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Retryable
    );
}

/// An account whose peak count does not match its leaf count is the host program's own
/// inconsistency. No proof can match it, and none is tried.
#[test]
fn an_inconsistent_mmr_state_is_terminal() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x43, FHE_TYPE_UINT64);
    let mut encrypted_value_account = EncryptedValueAccountFixture::allowing(live, key);
    let proof = answer(&encrypted_value_account, live, key);
    encrypted_value_account.encrypted_value.peaks.push([0; 32]);

    let failure = check_handle_binding(&resolved(&encrypted_value_account), live, key, &proof)
        .expect_err("two peaks for one leaf is not an MMR");

    assert!(matches!(
        failure,
        HandleBindingFailure::MmrStateInconsistent
    ));
    assert_eq!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

// ---------------------------------------------------------------------------
// Through the pipeline
// ---------------------------------------------------------------------------

/// The pipeline asks the record for exactly the leaf the entry claims — this account, this
/// handle, this key — and nothing broader. A record that answered a different query would be
/// answering a question nobody asked.
#[tokio::test]
async fn the_pipeline_asks_the_record_for_the_leaf_the_entry_claims() {
    let wallet = Wallet::new(1);
    let live = handle(0x50, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_value_account, live)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(
        &reader,
        &ServableKmsPair,
        &proofs,
        context(&deployment),
        &request,
    )
    .await
    .expect("the signer's own leaf authorizes");

    assert_eq!(
        proofs.calls(),
        vec![vec![LeafQuery {
            encrypted_value_account: encrypted_value_account.account_key,
            handle: live,
            kind: LeafKind::Allowed {
                key: wallet.pubkey()
            },
        }]]
    );
}

/// One batch for the request, one query per distinct leaf: two entries naming the same leaf cost
/// one query, and two entries on two accounts cost two, in request order.
#[tokio::test]
async fn the_pipeline_reads_one_batch_with_one_query_per_distinct_leaf() {
    let wallet = Wallet::new(1);
    let first = handle(0x51, FHE_TYPE_UINT64);
    let second = handle(0x52, FHE_TYPE_UINT64);
    let first_account = EncryptedValueAccountFixture::allowing(first, wallet.pubkey());
    let mut other_label = LABEL;
    other_label[0] = b'x';
    let mut second_account = EncryptedValueAccountFixture::in_application(
        APP_PROGRAM,
        AUTHORITY,
        SCOPE,
        other_label,
        second,
    );
    second_account.allow(wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&first_account, first)
        .direct(&second_account, second)
        .direct(&first_account, first)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&first_account)
        .with_encrypted_value_account(&second_account)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let authorized = authorize_request(
        &reader,
        &ServableKmsPair,
        &proofs,
        context(&deployment),
        &request,
    )
    .await
    .expect("every entry holds a leaf");

    assert_eq!(
        authorized.entries().len(),
        3,
        "the entry set is the request's"
    );
    let calls = proofs.calls();
    assert_eq!(calls.len(), 1, "one batch");
    assert_eq!(
        calls[0],
        vec![
            first_account.allowed_query(first, wallet.pubkey()),
            second_account.allowed_query(second, wallet.pubkey()),
        ],
        "distinct leaves in first-seen order, the repeat collapsed"
    );
}

/// The record's transport failing says nothing about any leaf: the request is rejected
/// transiently, with no verdict on any entry.
#[tokio::test]
async fn an_unreachable_record_rejects_transiently() {
    let wallet = Wallet::new(1);
    let live = handle(0x53, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_value_account, live)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        &UnavailableProofReader,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("no record, no verdict");

    assert!(matches!(failure, AuthorizationFailure::ProofRead(_)));
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// In a batch, the failure names the entry whose leaf is missing — in request coordinates.
#[tokio::test]
async fn a_batch_failure_names_the_entry_without_a_leaf() {
    let wallet = Wallet::new(1);
    let allowed = handle(0x54, FHE_TYPE_UINT64);
    let never_allowed = handle(0x55, FHE_TYPE_UINT64);
    let mut encrypted_value_account =
        EncryptedValueAccountFixture::allowing(allowed, wallet.pubkey());
    encrypted_value_account.update(never_allowed);
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_value_account, allowed)
        .direct(&encrypted_value_account, never_allowed)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        &proofs,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("the second entry was never allowed");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 1,
                source: HandleBindingFailure::NoLeaf { .. }
            }
        ),
        "the failure names the entry, got {failure}"
    );
}
