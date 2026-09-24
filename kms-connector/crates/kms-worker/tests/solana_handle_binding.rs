//! Handle binding: which key may decrypt which handle of an encrypted store, and on what
//! evidence.
//!
//! The evidence is always the same: an `Allowed(key, handle)` leaf sealed in the account's MMR,
//! located by the coprocessors' leaf record and proven against the peaks this connector observed
//! on chain. There is no live membership set to consult and no current-handle rule: a leaf, once
//! sealed, names its handle and its key forever, and a later write to the account changes which
//! handle is current without touching what was allowed on the old one.
//!
//! The file is dominated by substitutions of the leaf commitment: another encrypted store,
//! another key, another handle, another position, another leaf kind. Each of them is a proof that
//! verifies against *something*, and the question is whether the code checks that it verifies
//! against the thing that was asked. A leaf commitment binds four values and a domain prefix, and
//! a test per value is the only way to know all of them are in the preimage.
//!
//! The record's answer is classified only once it carries no proof, and the classification is the
//! second half of this file: a record with the observed history and no leaf is an ACL denial to
//! retry; a record behind the chain, a record that does not know the account yet, and a proof from
//! a record ahead of this observation are disagreements to retry; a record whose history has a gap
//! is terminal.
//! Two accepts carry as much weight as the rejections: a proof from a record behind the chain still
//! verifies when the append that followed left its peak alone, and it must be taken.

mod solana_support;

use kms_worker::core::solana::{
    SolanaPubkeyBytes,
    encrypted_store::{ResolvedEncryptedStore, resolve_encrypted_store},
    failure::AuthorizationFailure,
    handle_binding::{HandleBindingFailure, check_handle_binding, verify_proofs},
    pipeline::authorize_request,
    proof::{LeafKind, LeafProofOutcome, LeafQuery, ProofReadError},
};
use rstest::rstest;
use solana_support::*;
use zama_solana_acl::{historical_access_leaf_commitment, public_decrypt_leaf_commitment};

/// Resolves an encrypted store the way the pipeline does, so the binding rules are
/// exercised against a validated account rather than a hand-made value.
fn resolved(encrypted_store: &EncryptedStoreFixture) -> ResolvedEncryptedStore {
    let world = World::at_slot(1).with_encrypted_store(encrypted_store);
    resolve_encrypted_store(
        world.account(&encrypted_store.account_key).as_ref(),
        PROGRAM_ID,
        encrypted_store.account_key,
    )
    .expect("the fixture encrypted store resolves")
}

/// The record's answer for `key` on `handle` when it has sealed exactly this account's leaves.
fn answer(
    encrypted_store: &EncryptedStoreFixture,
    handle: [u8; 32],
    key: SolanaPubkeyBytes,
) -> LeafProofOutcome {
    encrypted_store.outcome(&encrypted_store.allowed_query(handle, key))
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
    let encrypted_store = EncryptedStoreFixture::allowing(live, key);

    check_handle_binding(
        &resolved(&encrypted_store),
        live,
        key,
        &answer(&encrypted_store, live, key),
    )
    .expect("an owner address decrypts its handle");
}

/// A write replaces the current handle and seals nothing about the old one, and the old one stays
/// decryptable by whoever was allowed on it: the leaf names the handle, not the account's current
/// state. This is what makes "historical access" the ordinary case rather than a mode.
#[test]
fn a_leaf_outlives_the_handle_it_names_being_replaced() {
    let key = Wallet::new(1).pubkey();
    let sealed = handle(0x11, FHE_TYPE_UINT64);
    let mut encrypted_store = EncryptedStoreFixture::allowing(sealed, key);
    encrypted_store.update(handle(0x12, FHE_TYPE_UINT64));
    encrypted_store.allow(key);

    check_handle_binding(
        &resolved(&encrypted_store),
        sealed,
        key,
        &answer(&encrypted_store, sealed, key),
    )
    .expect("the old handle is still the owner address's to decrypt");
}

/// Being allowed on the account's current handle says nothing about a handle that was never
/// allowed to the key. Access is per handle, and the account is only where the handles live.
#[test]
fn a_leaf_on_one_handle_does_not_bind_another() {
    let key = Wallet::new(1).pubkey();
    let allowed = handle(0x13, FHE_TYPE_UINT64);
    let never_allowed = handle(0x14, FHE_TYPE_UINT64);
    let mut encrypted_store = EncryptedStoreFixture::allowing(allowed, key);
    encrypted_store.update(never_allowed);

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
        never_allowed,
        key,
        &answer(&encrypted_store, never_allowed, key),
    )
    .expect_err("the current handle was never allowed to the key");

    assert!(matches!(
        failure,
        HandleBindingFailure::NoLeaf {
            record_leaf_count: 1,
            live_leaf_count: 1
        }
    ));
}

/// Several keys allowed on one handle each hold their own leaf, and each is proven on its own.
#[test]
fn each_owner_address_holds_its_own_leaf() {
    let first = Wallet::new(1).pubkey();
    let second = Wallet::new(2).pubkey();
    let live = handle(0x15, FHE_TYPE_UINT64);
    let mut encrypted_store = EncryptedStoreFixture::allowing(live, first);
    encrypted_store.allow(second);
    let account = resolved(&encrypted_store);

    check_handle_binding(
        &account,
        live,
        first,
        &answer(&encrypted_store, live, first),
    )
    .expect("the first key's leaf verifies");
    check_handle_binding(
        &account,
        live,
        second,
        &answer(&encrypted_store, live, second),
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
    let mut encrypted_store = EncryptedStoreFixture::new(live);
    let commitment = substitute(encrypted_store.account_key, live, key);
    encrypted_store.append(encrypted_store.allowed_query(live, key), commitment);

    check_handle_binding(
        &resolved(&encrypted_store),
        live,
        key,
        &answer(&encrypted_store, live, key),
    )
}

fn assert_does_not_verify(verdict: Result<(), HandleBindingFailure>) {
    let failure = verdict.expect_err("a substituted commitment must not verify");
    assert!(
        matches!(failure, HandleBindingFailure::ProofDoesNotVerify { .. }),
        "expected a proof that does not verify, got {failure}"
    );
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
    );
}

/// A leaf sealed for another encrypted store authorizes nothing here.
#[test]
fn a_leaf_committing_to_another_encrypted_store_does_not_verify() {
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

/// A sibling path with one hash altered does not reach the peak. The record supplies the path
/// and the chain decides.
#[test]
fn a_tampered_sibling_path_does_not_verify() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x23, FHE_TYPE_UINT64);
    let mut encrypted_store = EncryptedStoreFixture::allowing(live, key);
    encrypted_store.allow(Wallet::new(2).pubkey());
    let LeafProofOutcome::Found {
        leaf_index,
        leaf_count,
        mut siblings,
    } = answer(&encrypted_store, live, key)
    else {
        panic!("the record holds the leaf");
    };
    assert!(!siblings.is_empty(), "two leaves give the first a sibling");
    siblings[0][0] ^= 1;

    assert_does_not_verify(check_handle_binding(
        &resolved(&encrypted_store),
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

/// A record that has sealed at least the observed history and has no leaf: nothing grants the key
/// yet. The node this connector reads can be behind the grant the user saw, so the request is
/// retried within the attempt budget, as an EVM ACL denial is.
#[test]
fn no_leaf_in_a_record_with_the_observed_history_is_retried() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, Wallet::new(9).pubkey());

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
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
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
    );
}

/// A record ahead of the chain — more history sealed than this observation shows — and still no
/// leaf is a missing leaf, not a record to wait for.
#[test]
fn no_leaf_in_a_record_ahead_of_the_chain_is_a_missing_leaf() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, Wallet::new(9).pubkey());

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
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
    let encrypted_store = EncryptedStoreFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
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
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
    );
}

/// An account the chain has and the record has never seen is a record that has not indexed the
/// account's creation yet.
#[test]
fn an_account_unknown_to_the_record_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x33, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
        live,
        key,
        &LeafProofOutcome::UnknownAccount,
    )
    .expect_err("a record that does not know the account decides nothing");

    assert!(matches!(
        failure,
        HandleBindingFailure::AccountUnknownToProofRecord
    ));
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
    );
}

/// A record whose history for the account has a gap can answer nothing about it until rebuilt,
/// and no retry within a request's budget rebuilds it.
#[test]
fn an_incomplete_history_is_terminal() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x34, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, key);

    let failure = check_handle_binding(
        &resolved(&encrypted_store),
        live,
        key,
        &LeafProofOutcome::HistoryIncomplete,
    )
    .expect_err("a broken record proves nothing");

    assert!(matches!(failure, HandleBindingFailure::HistoryIncomplete));
    assert!(
        !AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
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
    let mut before = EncryptedStoreFixture::allowing(sealed, key);
    before.allow(Wallet::new(2).pubkey());
    assert_eq!(before.encrypted_store.leaf_count, 2);
    let proof_from_behind = answer(&before, sealed, key);
    let mut after = before.clone();
    after.allow(Wallet::new(3).pubkey());
    assert_eq!(after.encrypted_store.leaf_count, 3);

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
    let mut before = EncryptedStoreFixture::allowing(sealed, key);
    assert_eq!(before.encrypted_store.leaf_count, 1);
    let proof_from_behind = answer(&before, sealed, key);
    let mut after = before.clone();
    after.allow(Wallet::new(2).pubkey());
    assert_eq!(after.encrypted_store.leaf_count, 2);
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
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
    );
}

/// A record ahead of this observation may serve a leaf at a position the account does not have
/// yet. There is nothing for the proof to be a proof of, and it is refused before any hashing.
#[test]
fn a_leaf_position_the_account_does_not_have_is_retryable() {
    let key = Wallet::new(1).pubkey();
    let live = handle(0x42, FHE_TYPE_UINT64);
    let behind = EncryptedStoreFixture::new(live);
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
    assert!(
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: failure
        }
        .is_recoverable()
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
    let encrypted_store = EncryptedStoreFixture::allowing(live, wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, live)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("the signer's own leaf authorizes");

    assert_eq!(
        proofs.calls(),
        vec![(
            0,
            vec![LeafQuery {
                encrypted_store: encrypted_store.account_key,
                handle: live,
                kind: LeafKind::Allowed {
                    key: wallet.pubkey()
                },
            }]
        )]
    );
}

/// One batch for the request, one query per entry in request order, so each result is its entry's.
#[tokio::test]
async fn the_pipeline_reads_one_batch_with_one_query_per_entry() {
    let wallet = Wallet::new(1);
    let first = handle(0x51, FHE_TYPE_UINT64);
    let second = handle(0x52, FHE_TYPE_UINT64);
    let first_account = EncryptedStoreFixture::allowing(first, wallet.pubkey());
    let mut other_authority = AUTHORITY;
    other_authority[0] ^= 1;
    let mut second_account =
        EncryptedStoreFixture::in_application(APP_PROGRAM, other_authority, SCOPE, LABEL, second);
    second_account.allow(wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&first_account, first)
        .direct(&second_account, second)
        .direct(&first_account, first)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&first_account)
        .with_encrypted_store(&second_account)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("every entry holds a leaf");

    let calls = proofs.calls();
    assert_eq!(calls.len(), 1, "one batch");
    assert_eq!(
        calls[0].1,
        vec![
            first_account.allowed_query(first, wallet.pubkey()),
            second_account.allowed_query(second, wallet.pubkey()),
            first_account.allowed_query(first, wallet.pubkey()),
        ]
    );
}

/// The record's transport failing says nothing about any leaf: the request is rejected
/// transiently, with no verdict on any entry.
#[tokio::test]
async fn an_unreachable_record_rejects_transiently() {
    let wallet = Wallet::new(1);
    let live = handle(0x53, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, live)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);

    let failure = authorize_request(&reader, &ScriptedProofReader::down(), CONTEXT, &request)
        .await
        .expect_err("no record, no verdict");

    assert!(matches!(failure, AuthorizationFailure::ProofRead(_)));
    assert!(failure.is_recoverable());
}

/// In a batch, the failure names the entry whose leaf is missing — in request coordinates.
#[tokio::test]
async fn a_batch_failure_names_the_entry_without_a_leaf() {
    let wallet = Wallet::new(1);
    let allowed = handle(0x54, FHE_TYPE_UINT64);
    let never_allowed = handle(0x55, FHE_TYPE_UINT64);
    let mut encrypted_store = EncryptedStoreFixture::allowing(allowed, wallet.pubkey());
    encrypted_store.update(never_allowed);
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, allowed)
        .direct(&encrypted_store, never_allowed)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    let failure = authorize_request(&reader, &proofs, CONTEXT, &request)
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

#[test]
fn ahead_paths_verify_against_every_earlier_mountain() {
    let key = Wallet::new(1).pubkey();
    let sealed = handle(0x61, FHE_TYPE_UINT64);
    let mut history = EncryptedStoreFixture::allowing(sealed, key);
    let mut snapshots = vec![history.clone()];
    for tag in 2..=16 {
        history.allow(Wallet::new(tag).pubkey());
        snapshots.push(history.clone());
    }
    for (index, snapshot) in snapshots.iter().enumerate() {
        let account = resolved(snapshot);
        for tag in 1..=index + 1 {
            let key = Wallet::new(tag as u8).pubkey();
            for ahead in &snapshots[index..] {
                check_handle_binding(&account, sealed, key, &answer(ahead, sealed, key))
                    .expect("ahead paths can be shortened to the observed peak");
            }
        }
    }
}

/// A proof built against an older leaf count still verifies, so it resolves the query at the first
/// coprocessor and the next one is not asked.
#[tokio::test]
async fn a_valid_older_proof_resolves_without_asking_the_next_coprocessor() {
    let wallet = Wallet::new(1);
    let sealed = handle(0x62, FHE_TYPE_UINT64);
    let mut before = EncryptedStoreFixture::allowing(sealed, wallet.pubkey());
    before.allow(Wallet::new(2).pubkey());
    let mut after = before.clone();
    after.allow(Wallet::new(3).pubkey());
    let request = RequestBuilder::new(&wallet).direct(&after, sealed).typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&after)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::in_order(vec![ProofRecord::of(&[&before]), world.record()]);
    authorize_request(&ScriptedReader::constant(world), &proofs, CONTEXT, &request)
        .await
        .expect("a proof against a surviving peak verifies");
    assert_eq!(proofs.call_count(), 1);
}

/// A store holding allow leaves for `key` on two handles, resolved, and the two queries.
fn two_allowed_queries() -> (
    EncryptedStoreFixture,
    ResolvedEncryptedStore,
    [(LeafQuery, [u8; 32]); 2],
) {
    let key = Wallet::new(1).pubkey();
    let first = handle(0x63, FHE_TYPE_UINT64);
    let second = handle(0x64, FHE_TYPE_UINT64);
    let mut fixture = EncryptedStoreFixture::allowing(first, key);
    fixture.allow_handle(second, key);
    let account = resolved(&fixture);
    let batch = [first, second].map(|handle| (fixture.allowed_query(handle, key), handle));
    (fixture, account, batch)
}

/// What the store's record answers for a query it does not hold.
fn not_found(fixture: &EncryptedStoreFixture) -> LeafProofOutcome {
    LeafProofOutcome::NotFound {
        leaf_count: fixture.encrypted_store.leaf_count,
    }
}

async fn verify_with(
    reader: &ScriptedProofReader,
    account: &ResolvedEncryptedStore,
    batch: &[(LeafQuery, [u8; 32])],
) -> Result<Vec<Result<(), HandleBindingFailure>>, ProofReadError> {
    let key = Wallet::new(1).pubkey();
    verify_proofs(reader, batch, |handle, outcome| {
        check_handle_binding(account, *handle, key, outcome)
    })
    .await
}

/// Coprocessor A holds only the first leaf and B holds both: both queries resolve, and B is asked
/// only for the query A left unresolved.
#[tokio::test]
async fn the_next_coprocessor_is_asked_only_for_the_unresolved_queries() {
    let (fixture, account, batch) = two_allowed_queries();
    let [(first, _), (second, _)] = batch;
    let only_first = ProofRecord::answering([
        (first, fixture.outcome(&first)),
        (second, not_found(&fixture)),
    ]);
    let reader = ScriptedProofReader::in_order(vec![only_first, ProofRecord::of(&[&fixture])]);

    let results = verify_with(&reader, &account, &batch).await.unwrap();

    assert_eq!(results, vec![Ok(()), Ok(())]);
    assert_eq!(
        reader.calls(),
        vec![(0, vec![first, second]), (1, vec![second])]
    );
}

/// When every coprocessor has sealed the history without the leaf, the entry is a recoverable
/// denial, and no coprocessor is asked twice.
#[tokio::test]
async fn every_coprocessor_missing_the_leaf_is_a_recoverable_denial() {
    let (fixture, account, [entry, _]) = two_allowed_queries();
    let missing = ProofRecord::answering([(entry.0, not_found(&fixture))]);
    let reader = ScriptedProofReader::in_order(vec![missing.clone(), missing]);

    let results = verify_with(&reader, &account, &[entry]).await.unwrap();

    assert!(matches!(
        &results[..],
        [Err(failure @ HandleBindingFailure::NoLeaf { .. })] if failure.is_recoverable()
    ));
    assert_eq!(reader.call_count(), 2);
}

/// A coprocessor that fails the read, or answers the wrong number of outcomes, says nothing about
/// any leaf: the batch moves on to the next.
#[tokio::test]
async fn a_failed_or_short_read_moves_on_to_the_next_coprocessor() {
    let (fixture, account, batch) = two_allowed_queries();
    let record = ProofRecord::of(&[&fixture]);
    let reader = ScriptedProofReader::coprocessors(vec![
        ProofSource::Down,
        ProofSource::Truncated(record.clone()),
        ProofSource::Serving(record),
    ]);

    let results = verify_with(&reader, &account, &batch).await.unwrap();

    assert_eq!(results, vec![Ok(()), Ok(())]);
    let queries: Vec<_> = batch.iter().map(|(query, _)| *query).collect();
    assert_eq!(
        reader.calls(),
        vec![(0, queries.clone()), (1, queries.clone()), (2, queries)]
    );
}

/// A query no coprocessor answered is a failed proof read, naming every coprocessor's failure.
#[tokio::test]
async fn no_answer_from_any_coprocessor_is_a_proof_read_error() {
    let (_, account, batch) = two_allowed_queries();
    let reader = ScriptedProofReader::coprocessors(vec![ProofSource::Down, ProofSource::Down]);

    let error = verify_with(&reader, &account, &batch).await.unwrap_err();

    let ProofReadError::Unavailable { reason } = error else {
        panic!("expected an unavailable read, got {error}");
    };
    assert!(reason.contains("coprocessor 0") && reason.contains("coprocessor 1"));
}

/// One coprocessor whose history for the store is incomplete cannot make the entry terminal while
/// another reports a recoverable miss, whichever answers first.
#[rstest]
#[case::incomplete_first(false)]
#[case::incomplete_last(true)]
#[tokio::test]
async fn a_terminal_answer_does_not_replace_a_recoverable_one(#[case] incomplete_last: bool) {
    let (fixture, account, [entry, _]) = two_allowed_queries();
    let mut records = vec![
        ProofRecord::answering([(entry.0, LeafProofOutcome::HistoryIncomplete)]),
        ProofRecord::answering([(entry.0, not_found(&fixture))]),
    ];
    if incomplete_last {
        records.reverse();
    }

    let results = verify_with(&ScriptedProofReader::in_order(records), &account, &[entry])
        .await
        .unwrap();

    assert!(matches!(
        &results[..],
        [Err(HandleBindingFailure::NoLeaf { .. })]
    ));
}

/// When every coprocessor answers that its history for the store is incomplete, the entry fails
/// terminally.
#[tokio::test]
async fn a_terminal_answer_from_every_coprocessor_stands() {
    let (_, account, [entry, _]) = two_allowed_queries();
    let incomplete = || ProofRecord::answering([(entry.0, LeafProofOutcome::HistoryIncomplete)]);
    let reader = ScriptedProofReader::in_order(vec![incomplete(), incomplete()]);

    let results = verify_with(&reader, &account, &[entry]).await.unwrap();

    assert!(matches!(
        &results[..],
        [Err(HandleBindingFailure::HistoryIncomplete)]
    ));
}

/// A terminal answer stands only when every coprocessor answered: one that could not be read may
/// still hold the history the other lacks, whichever is asked first.
#[rstest]
#[case::incomplete_first(false)]
#[case::incomplete_last(true)]
#[tokio::test]
async fn a_terminal_answer_beside_an_unread_coprocessor_is_a_proof_read_error(
    #[case] incomplete_last: bool,
) {
    let (_, account, [entry, _]) = two_allowed_queries();
    let mut sources = vec![
        ProofSource::Serving(ProofRecord::answering([(
            entry.0,
            LeafProofOutcome::HistoryIncomplete,
        )])),
        ProofSource::Down,
    ];
    if incomplete_last {
        sources.reverse();
    }

    let error = verify_with(
        &ScriptedProofReader::coprocessors(sources),
        &account,
        &[entry],
    )
    .await
    .unwrap_err();

    assert!(matches!(error, ProofReadError::Unavailable { .. }));
}

/// Through the production client: an unavailable coprocessor hands the batch to the next one.
#[tokio::test]
async fn an_unavailable_coprocessor_hands_the_batch_to_the_next() {
    use kms_worker::core::solana::proof::CoprocessorProofClient;
    use mocktail::{StatusCode, server::MockServer};
    let (fixture, account, [entry, _]) = two_allowed_queries();
    let mut unavailable = MockServer::new_http("unavailable-coprocessor");
    unavailable.mock(|when, then| {
        when.post();
        then.status(StatusCode::BAD_GATEWAY);
    });
    unavailable.start().await.unwrap();
    let mut serving = MockServer::new_http("serving-coprocessor");
    serve_proofs(&mut serving, &[(entry.0, fixture.outcome(&entry.0))]);
    serving.start().await.unwrap();
    let client = CoprocessorProofClient::new(
        &[
            unavailable.base_url().unwrap().clone(),
            serving.base_url().unwrap().clone(),
        ],
        "secret".into(),
        reqwest::Client::new(),
    );
    let key = Wallet::new(1).pubkey();

    let results = verify_proofs(&client, &[entry], |handle, outcome| {
        check_handle_binding(&account, *handle, key, outcome)
    })
    .await
    .unwrap();

    assert_eq!(results, vec![Ok(())]);
}

/// A duplicate handle is legal, as on the EVM path, where the Gateway does not deduplicate: both
/// occurrences of an authorized handle are authorized.
#[tokio::test]
async fn both_occurrences_of_a_duplicate_handle_are_authorized() {
    let wallet = Wallet::new(1);
    let repeated = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(repeated, wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, repeated)
        .direct(&encrypted_store, repeated)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a duplicate of an authorized handle is authorized");
}
