//! The Connector's verdict on user and public decryptions, written to a fixture the SDK's cleartext
//! client is held to (`sdk/js-sdk/src/solana/cleartext/authorization.test.ts`). Each case runs the
//! real [`authorize_request`] or [`check_public_decrypt`] over one world and one leaf record, and
//! the committed file must be what this run renders: the host accounts, the Merkle proof batch the
//! Connector asked for with the record's answers, and the verdict.
//! `ZAMA_UPDATE_AUTHORIZATION_CASES=1` rewrites it.
//!
//! Every failure variant is either produced by a user-decrypt case or listed in [`RUST_ONLY`] with
//! the reason the cleartext client cannot produce it.

mod solana_support;

use alloy::primitives::{B256, U256};
use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use connector_utils::types::solana_request::{
    SolanaPublicDecryptionRequest, SolanaUserDecryptionRequestV1,
};
use kms_worker::core::solana::{
    delegation::DelegationFailure,
    encrypted_store::EncryptedStoreFailure,
    failure::AuthorizationFailure,
    handle_binding::HandleBindingFailure,
    pipeline::authorize_request,
    proof::{LeafKind, LeafQuery, MerkleProofOutcome, ProofReadError},
    public_decrypt::{PublicDecryptFailure, check_public_decrypt},
    snapshot::{SnapshotAccount, SnapshotError},
    watermark::{WatermarkFailure, WindowFailure},
};
use serde_json::{Value, json};
use solana_pubkey::Pubkey;
use solana_support::*;
use zama_solana_acl::WILDCARD_APP;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../solana/test-fixtures/authorization/decrypt_cases_v1.json"
);
const UPDATE_ENV: &str = "ZAMA_UPDATE_AUTHORIZATION_CASES";
const SLOT: u64 = 100;

/// Every failure [`failure_name`] can return.
const FAILURES: [&str; 26] = [
    "Signature",
    "Window::NotYetValid",
    "Window::Expired",
    "ProgramIdMismatch",
    "Snapshot::Unavailable",
    "Snapshot::ResponseLengthMismatch",
    "Snapshot::MalformedClock",
    "Snapshot::NodeBehind",
    "Watermark::Invalidated",
    "Watermark::InvalidHostRecord",
    "EncryptedStore::Absent",
    "EncryptedStore::ForeignOwner",
    "EncryptedStore::NotAnEncryptedStore",
    "EncryptedStore::AddressMismatch",
    "EncryptedStore::InvalidHostRecord",
    "ScopeNotAllowed",
    "ProofRead::Unavailable",
    "ProofRead::ResponseLengthMismatch",
    "HandleBinding::NoLeaf",
    "HandleBinding::ProofRecordBehind",
    "HandleBinding::AccountUnknownToProofRecord",
    "HandleBinding::ProofRecordInconsistent",
    "HandleBinding::ProofDoesNotVerify",
    "HandleBinding::LeafIndexOutOfRange",
    "Delegation::NoLiveDelegation",
    "Delegation::InvalidHostRecord",
];

const NODE: &str = "The cleartext client throws when an account read fails, lags or answers the \
                    wrong number of accounts, instead of judging the request.";
const LEAF_RECORD: &str = "The cleartext client throws when a Merkle proof read fails or answers the \
                           wrong number of proofs, instead of judging the request.";
const OWN_RECORD: &str = "The cleartext client builds its proofs from its own leaf record, which \
                          has no store check, so it never answers inconsistent.";

/// The failures no cleartext case can produce, with the reason.
const RUST_ONLY: [(&str, &str); 7] = [
    ("Snapshot::Unavailable", NODE),
    ("Snapshot::ResponseLengthMismatch", NODE),
    ("Snapshot::MalformedClock", NODE),
    ("Snapshot::NodeBehind", NODE),
    ("ProofRead::Unavailable", LEAF_RECORD),
    ("ProofRead::ResponseLengthMismatch", LEAF_RECORD),
    ("HandleBinding::ProofRecordInconsistent", OWN_RECORD),
];

fn snapshot_name(source: &SnapshotError) -> &'static str {
    match source {
        SnapshotError::Unavailable { .. } => "Snapshot::Unavailable",
        SnapshotError::ResponseLengthMismatch { .. } => "Snapshot::ResponseLengthMismatch",
        SnapshotError::MalformedClock => "Snapshot::MalformedClock",
        SnapshotError::NodeBehind => "Snapshot::NodeBehind",
    }
}

fn encrypted_store_name(source: &EncryptedStoreFailure) -> &'static str {
    match source {
        EncryptedStoreFailure::Absent { .. } => "EncryptedStore::Absent",
        EncryptedStoreFailure::ForeignOwner { .. } => "EncryptedStore::ForeignOwner",
        EncryptedStoreFailure::NotAnEncryptedStore { .. } => "EncryptedStore::NotAnEncryptedStore",
        EncryptedStoreFailure::AddressMismatch { .. } => "EncryptedStore::AddressMismatch",
        EncryptedStoreFailure::InvalidHostRecord(_) => "EncryptedStore::InvalidHostRecord",
    }
}

fn proof_read_name(source: &ProofReadError) -> &'static str {
    match source {
        ProofReadError::Unavailable { .. } => "ProofRead::Unavailable",
        ProofReadError::ResponseLengthMismatch { .. } => "ProofRead::ResponseLengthMismatch",
    }
}

fn handle_binding_name(source: &HandleBindingFailure) -> &'static str {
    match source {
        HandleBindingFailure::NoLeaf { .. } => "HandleBinding::NoLeaf",
        HandleBindingFailure::ProofRecordBehind { .. } => "HandleBinding::ProofRecordBehind",
        HandleBindingFailure::AccountUnknownToProofRecord => {
            "HandleBinding::AccountUnknownToProofRecord"
        }
        HandleBindingFailure::ProofRecordInconsistent => "HandleBinding::ProofRecordInconsistent",
        HandleBindingFailure::ProofDoesNotVerify { .. } => "HandleBinding::ProofDoesNotVerify",
        HandleBindingFailure::LeafIndexOutOfRange { .. } => "HandleBinding::LeafIndexOutOfRange",
    }
}

/// The variant a user-decrypt failure is, and the entry it names.
fn failure_name(failure: &AuthorizationFailure) -> (&'static str, Option<usize>) {
    use AuthorizationFailure as F;
    match failure {
        F::Signature(_) => ("Signature", None),
        F::Window(WindowFailure::NotYetValid { .. }) => ("Window::NotYetValid", None),
        F::Window(WindowFailure::Expired { .. }) => ("Window::Expired", None),
        F::ProgramIdMismatch { .. } => ("ProgramIdMismatch", None),
        F::Snapshot(source) => (snapshot_name(source), None),
        F::Watermark(WatermarkFailure::Invalidated { .. }) => ("Watermark::Invalidated", None),
        F::Watermark(WatermarkFailure::InvalidHostRecord(_)) => {
            ("Watermark::InvalidHostRecord", None)
        }
        F::EncryptedStore { index, source } => (encrypted_store_name(source), Some(*index)),
        F::ScopeNotAllowed { index, .. } => ("ScopeNotAllowed", Some(*index)),
        F::ProofRead(source) => (proof_read_name(source), None),
        F::HandleBinding { index, source } => (handle_binding_name(source), Some(*index)),
        F::Delegation { index, source } => (
            match source {
                DelegationFailure::NoLiveDelegation { .. } => "Delegation::NoLiveDelegation",
                DelegationFailure::InvalidHostRecord(_) => "Delegation::InvalidHostRecord",
            },
            Some(*index),
        ),
    }
}

/// The variant a public-decrypt failure is, and the entry it names.
fn public_failure_name(failure: &PublicDecryptFailure) -> (&'static str, Option<usize>) {
    use PublicDecryptFailure as F;
    match failure {
        F::Snapshot(source) => (snapshot_name(source), None),
        F::EncryptedStore { index, source } => (encrypted_store_name(source), Some(*index)),
        F::ProofRead(source) => (proof_read_name(source), None),
        F::HandleBinding { index, source } => (handle_binding_name(source), Some(*index)),
    }
}

type Expected = Option<(&'static str, Option<usize>)>;

struct Case {
    name: &'static str,
    now: u64,
    request: SolanaUserDecryptionRequestV1,
    world: World,
    record: ProofRecord,
    expected: Expected,
}

/// A user decryption judged at [`NOW_INSIDE_WINDOW`] against a leaf record in step with `world`.
fn case(name: &'static str, request: RequestBuilder<'_>, world: World, expected: Expected) -> Case {
    Case {
        name,
        now: NOW_INSIDE_WINDOW,
        request: request.typed(),
        record: world.record(),
        world,
        expected,
    }
}

impl Case {
    /// The same case judged at `now` by the Connector's clock.
    fn at(self, now: u64) -> Self {
        Self { now, ..self }
    }

    /// The same case against another leaf record.
    fn record(self, record: ProofRecord) -> Self {
        Self { record, ..self }
    }
}

struct PublicCase {
    name: &'static str,
    /// Each handle with the encrypted store it is proven against.
    handles: Vec<([u8; 32], Pubkey)>,
    world: World,
    record: ProofRecord,
    expected: Expected,
}

/// A public decryption against a leaf record in step with `world`.
fn public_case(
    name: &'static str,
    handles: &[([u8; 32], Pubkey)],
    world: World,
    expected: Expected,
) -> PublicCase {
    PublicCase {
        name,
        handles: handles.to_vec(),
        record: world.record(),
        world,
        expected,
    }
}

fn foreign(account: SnapshotAccount) -> SnapshotAccount {
    SnapshotAccount {
        owner: pubkey(9),
        ..account
    }
}

fn cases() -> Vec<Case> {
    let user = Wallet::new(0x21);
    let other_wallet = Wallet::new(0x23);
    let signer = user.pubkey();
    let delegator = pubkey(0x22);
    let stranger = pubkey(0x55);
    let h1 = handle(0xa1, FHE_TYPE_UINT64);
    let h2 = handle(0xa2, FHE_TYPE_UINT64);
    let other_app = pubkey(0x31);
    let world = || World::at_slot(SLOT);
    let request = || RequestBuilder::new(&user);
    let end = DEFAULT_START + DEFAULT_DURATION;

    let mine = EncryptedStoreFixture::allowing(h1, signer);
    let theirs = EncryptedStoreFixture::allowing(h1, delegator);
    let strangers = EncryptedStoreFixture::allowing(h1, stranger);
    let delegation = DelegationFixture::live(delegator, signer);
    let wildcard = DelegationFixture::live_wildcard(delegator, signer);
    let outside = EncryptedStoreFixture::in_application(other_app, AUTHORITY, SCOPE, LABEL, h1);
    let mut outside_mine = outside.clone();
    outside_mine.allow(signer);
    let mut outside_theirs = outside.clone();
    outside_theirs.allow(delegator);

    let mut public = EncryptedStoreFixture::new(h1);
    public.mark_public();
    public.allow(signer);
    let mut updated = mine.clone();
    updated.update(h2);
    let mut ahead = strangers.clone();
    ahead.allow(signer);
    let mut reordered = mine.clone();
    reordered.allow(stranger);
    let mut inconsistent = mine.clone();
    inconsistent.encrypted_store.peaks.clear();
    let mut sentinel = EncryptedStoreFixture::in_application(
        Pubkey::new_from_array(WILDCARD_APP),
        AUTHORITY,
        SCOPE,
        LABEL,
        h1,
    );
    sentinel.allow(signer);
    let second = EncryptedStoreFixture::in_application(other_app, AUTHORITY, SCOPE, LABEL, h2);
    let copied_to = pubkey(0x44);

    vec![
        case(
            "a direct entry with the signer's allow leaf",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            None,
        ),
        case(
            "an allow leaf sealed after the handle was made public",
            request().direct(&public, h1),
            world().with_encrypted_store(&public),
            None,
        ),
        case(
            "an allow leaf on a handle the store no longer holds",
            request().direct(&updated, h1),
            world().with_encrypted_store(&updated),
            None,
        ),
        case(
            "a delegated entry through the application's row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation),
            None,
        ),
        case(
            "a delegated entry through the wildcard row beside a revoked application row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation.revoked())
                .with_delegation(&wildcard),
            None,
        ),
        case(
            "a permit signed by another wallet",
            RequestBuilder::new(&other_wallet)
                .permit(PermitBuilder::new(signer))
                .entry(h1, signer, mine.account_key),
            world().with_encrypted_store(&mine),
            Some(("Signature", None)),
        ),
        case(
            "a permit signed by another wallet, past its window",
            RequestBuilder::new(&other_wallet)
                .permit(PermitBuilder::new(signer))
                .entry(h1, signer, mine.account_key),
            world().with_encrypted_store(&mine),
            Some(("Signature", None)),
        )
        .at(end + 1),
        case(
            "a permit one second before its window",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            Some(("Window::NotYetValid", None)),
        )
        .at(DEFAULT_START - 1),
        case(
            "a permit in the first second of its window",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            None,
        )
        .at(DEFAULT_START),
        case(
            "a permit in the last second of its window",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            None,
        )
        .at(end),
        case(
            "a permit one second past its window",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            Some(("Window::Expired", None)),
        )
        .at(end + 1),
        case(
            "a permit for another host program",
            request()
                .permit(PermitBuilder::new(signer).verifying_program(pubkey(8)))
                .direct(&mine, h1),
            world().with_encrypted_store(&mine),
            Some(("ProgramIdMismatch", None)),
        ),
        case(
            "a permit for another host program, started before its signer revoked permits",
            request()
                .permit(PermitBuilder::new(signer).verifying_program(pubkey(8)))
                .direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_watermark(signer, DEFAULT_START + 1),
            Some(("ProgramIdMismatch", None)),
        ),
        case(
            "a permit that started before its signer revoked permits",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_watermark(signer, DEFAULT_START + 1),
            Some(("Watermark::Invalidated", None)),
        ),
        case(
            "a permit that started in the second its signer revoked permits",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_watermark(signer, DEFAULT_START),
            None,
        ),
        case(
            "a prefunded invalidation address",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_account(invalidation_address(signer).0, prefunded_account()),
            None,
        ),
        case(
            "an invalidation address holding another program's account",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine).with_account(
                invalidation_address(signer).0,
                foreign(invalidation_account(signer, 0)),
            ),
            Some(("Watermark::InvalidHostRecord", None)),
        ),
        case(
            "an entry naming no account",
            request().direct(&mine, h1),
            world(),
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "an entry naming a prefunded address",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, prefunded_account()),
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "an entry naming another program's account",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, foreign(mine.account())),
            Some(("EncryptedStore::ForeignOwner", Some(0))),
        ),
        case(
            "an entry naming another host account",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, delegation.account()),
            Some(("EncryptedStore::NotAnEncryptedStore", Some(0))),
        ),
        case(
            "an entry naming a store copied to another address",
            request().entry(h1, signer, copied_to),
            world().with_account(copied_to, mine.account()),
            Some(("EncryptedStore::AddressMismatch", Some(0))),
        ),
        case(
            "a store whose peaks do not match its leaf count",
            request().direct(&inconsistent, h1),
            world().with_encrypted_store(&inconsistent),
            Some(("EncryptedStore::InvalidHostRecord", Some(0))),
        ),
        case(
            "a store naming the wildcard application",
            request()
                .permit(PermitBuilder::new(signer).permissive())
                .direct(&sentinel, h1),
            world().with_encrypted_store(&sentinel),
            Some(("EncryptedStore::InvalidHostRecord", Some(0))),
        ),
        case(
            "a store outside the permit's scopes",
            request().direct(&outside_mine, h1),
            world().with_encrypted_store(&outside_mine),
            Some(("ScopeNotAllowed", Some(0))),
        ),
        case(
            "a permissive permit and a store of any application",
            request()
                .permit(PermitBuilder::new(signer).permissive())
                .direct(&outside_mine, h1),
            world().with_encrypted_store(&outside_mine),
            None,
        ),
        case(
            "a delegated entry outside the permit's scopes, with no delegation row",
            request().delegated(&outside_theirs, h1, delegator),
            world().with_encrypted_store(&outside_theirs),
            Some(("ScopeNotAllowed", Some(0))),
        ),
        case(
            "a delegated entry with no delegation row",
            request().delegated(&theirs, h1, delegator),
            world().with_encrypted_store(&theirs),
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
        case(
            "a delegation that has ended by the host's Clock but not by the local clock",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_clock(NOW_INSIDE_WINDOW + 100)
                .with_encrypted_store(&theirs)
                .with_delegation(&DelegationFixture {
                    expires_at: NOW_INSIDE_WINDOW + 50,
                    ..delegation
                }),
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
        case(
            "a revoked delegation",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation.revoked()),
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
        case(
            "a delegation row held by another program beside a live wildcard row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_account(delegation.address().0, foreign(delegation.account()))
                .with_delegation(&wildcard),
            Some(("Delegation::InvalidHostRecord", Some(0))),
        ),
        case(
            "a wildcard row held by another program beside a live application row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation)
                .with_account(wildcard.address().0, foreign(wildcard.account())),
            Some(("Delegation::InvalidHostRecord", Some(0))),
        ),
        case(
            "no allow leaf for the signer",
            request().direct(&strangers, h1),
            world().with_encrypted_store(&strangers),
            Some(("HandleBinding::NoLeaf", Some(0))),
        ),
        case(
            "a delegated entry backed only by the delegate's own leaf",
            request().delegated(&mine, h1, delegator),
            world()
                .with_encrypted_store(&mine)
                .with_delegation(&delegation),
            Some(("HandleBinding::NoLeaf", Some(0))),
        ),
        case(
            "a leaf record behind the store",
            request().direct(&ahead, h1),
            world().with_encrypted_store(&ahead),
            Some(("HandleBinding::ProofRecordBehind", Some(0))),
        )
        .record(ProofRecord::of(&[&strangers])),
        case(
            "a leaf record whose leaves disagree with the store's peaks",
            request().direct(&ahead, h1),
            world().with_encrypted_store(&ahead),
            Some(("HandleBinding::ProofDoesNotVerify", Some(0))),
        )
        .record(ProofRecord::of(&[&reordered])),
        case(
            "a leaf record that has never seen the store",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            Some(("HandleBinding::AccountUnknownToProofRecord", Some(0))),
        )
        .record(ProofRecord::default()),
        case(
            "a leaf record ahead of the store, whose leaf the store has not sealed yet",
            request().direct(&strangers, h1),
            world().with_encrypted_store(&strangers),
            Some(("HandleBinding::LeafIndexOutOfRange", Some(0))),
        )
        .record(ProofRecord::of(&[&ahead])),
        case(
            "a proof from a leaf record ahead of the store, of a leaf the store has sealed",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine),
            None,
        )
        .record(ProofRecord::of(&[&reordered])),
        case(
            "every entry's store is judged before any leaf",
            request().direct(&strangers, h1).direct(&second, h2),
            world()
                .with_encrypted_store(&strangers)
                .with_account(second.account_key, foreign(second.account())),
            Some(("EncryptedStore::ForeignOwner", Some(1))),
        ),
        case(
            "a delegated entry's store is resolved before the watermark",
            request().delegated(&theirs, h1, delegator),
            world().with_watermark(signer, DEFAULT_START + 1),
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "a direct entry's store is resolved after the watermark",
            request().direct(&mine, h1),
            world().with_watermark(signer, DEFAULT_START + 1),
            Some(("Watermark::Invalidated", None)),
        ),
        case(
            "entries are judged in request order",
            request()
                .delegated(&theirs, h1, delegator)
                .direct(&outside_mine, h1),
            world()
                .with_encrypted_store(&theirs)
                .with_encrypted_store(&outside_mine),
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
    ]
}

fn public_cases() -> Vec<PublicCase> {
    let world = || World::at_slot(SLOT);
    let h1 = handle(0xb1, FHE_TYPE_UINT64);
    let h2 = handle(0xb2, FHE_TYPE_UINT64);
    let key = pubkey(0x42);

    let allowed = EncryptedStoreFixture::allowing(h1, key);
    let mut public = allowed.clone();
    public.mark_public();
    let mut replaced = public.clone();
    replaced.update(h2);
    let mut sentinel = EncryptedStoreFixture::in_application(
        Pubkey::new_from_array(WILDCARD_APP),
        AUTHORITY,
        SCOPE,
        LABEL,
        h1,
    );
    sentinel.mark_public();
    let second = EncryptedStoreFixture::in_application(pubkey(0x31), AUTHORITY, SCOPE, LABEL, h2);

    vec![
        public_case(
            "a handle made public, then replaced in its slot",
            &[(h1, replaced.account_key)],
            world().with_encrypted_store(&replaced),
            None,
        ),
        public_case(
            "a handle the store never made public",
            &[(h1, allowed.account_key)],
            world().with_encrypted_store(&allowed),
            Some(("HandleBinding::NoLeaf", Some(0))),
        ),
        PublicCase {
            record: ProofRecord::of(&[&allowed]),
            ..public_case(
                "a leaf record behind the store's public leaf",
                &[(h1, public.account_key)],
                world().with_encrypted_store(&public),
                Some(("HandleBinding::ProofRecordBehind", Some(0))),
            )
        },
        PublicCase {
            record: ProofRecord::answering([(
                public.public_query(h1),
                allowed.outcome(&allowed.allowed_query(h1, key)),
            )]),
            ..public_case(
                "an allow leaf's proof offered for the public leaf",
                &[(h1, public.account_key)],
                world().with_encrypted_store(&public),
                Some(("HandleBinding::ProofDoesNotVerify", Some(0))),
            )
        },
        public_case(
            "no account at the named store",
            &[(h1, public.account_key)],
            world(),
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        public_case(
            "a store naming the wildcard application",
            &[(h1, sentinel.account_key)],
            world().with_encrypted_store(&sentinel),
            Some(("EncryptedStore::InvalidHostRecord", Some(0))),
        ),
        public_case(
            "every store is judged before any leaf",
            &[(h1, allowed.account_key), (h2, second.account_key)],
            world()
                .with_encrypted_store(&allowed)
                .with_account(second.account_key, foreign(second.account())),
            Some(("EncryptedStore::ForeignOwner", Some(1))),
        ),
    ]
}

/// A public decryption checked over real HTTP: the node serves the stores and the coprocessor
/// answers the batch of public queries from `record`. The batch, when the Connector reaches it, is
/// one query per handle in request order; a different one would miss the mock and fail the read.
async fn judge_public(case: &PublicCase) -> (Option<PublicDecryptFailure>, Vec<LeafQuery>) {
    let mut host = HttpHost::start().await;
    let stores: Vec<_> = case
        .handles
        .iter()
        .map(|(_, store)| (*store, case.world.account(store)))
        .collect();
    host.serve_accounts(&stores);
    let batch: Vec<_> = case
        .handles
        .iter()
        .map(|(handle, store)| LeafQuery {
            encrypted_store: *store,
            handle: B256::new(*handle),
            kind: LeafKind::Public,
        })
        .collect();
    let answers: Vec<_> = batch
        .iter()
        .map(|query| (*query, case.record.answer(query)))
        .collect();
    host.serve_proofs(&answers);
    let handles: Vec<_> = case.handles.iter().map(|(h, _)| B256::new(*h)).collect();
    let store_keys: Vec<_> = case
        .handles
        .iter()
        .map(|(_, s)| B256::new(s.to_bytes()))
        .collect();
    let request = SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &store_keys, vec![0x00])
        .expect("a well-formed request");
    let failure = check_public_decrypt(&host.host(), &request).await.err();
    // Every store is judged before the leaves are read.
    let asked = !matches!(
        failure,
        Some(PublicDecryptFailure::EncryptedStore { .. } | PublicDecryptFailure::Snapshot(_))
    );
    (failure, if asked { batch } else { Vec::new() })
}

fn hex(bytes: &[u8]) -> String {
    alloy::hex::encode(bytes)
}

fn render_accounts(world: &World) -> Vec<Value> {
    world
        .accounts()
        .map(|(address, account)| {
            json!({
                "address": address.to_string(),
                "owner": account.owner.to_string(),
                "data_base64": BASE64_STANDARD.encode(&account.data),
            })
        })
        .collect()
}

/// The batch the Connector asked for, each query with the record's answer, with the fields of
/// `POST /v1/solana/merkle-proofs`; `null` when it asked for none.
fn render_leaf_read(batch: &[LeafQuery], record: &ProofRecord) -> Value {
    if batch.is_empty() {
        return Value::Null;
    }
    batch
        .iter()
        .map(|query| {
            json!({
                "query": render_query(query),
                "outcome": render_outcome(&record.answer(query)),
            })
        })
        .collect()
}

fn render_query(query: &LeafQuery) -> Value {
    let mut rendered = json!({
        "encryptedStore": hex(query.encrypted_store.as_ref()),
        "handle": hex(query.handle.as_slice()),
    });
    match query.kind {
        LeafKind::Allowed { key } => {
            rendered["kind"] = json!("allowed");
            rendered["key"] = json!(hex(key.as_ref()));
        }
        LeafKind::Public => rendered["kind"] = json!("public"),
    }
    rendered
}

fn render_outcome(outcome: &MerkleProofOutcome) -> Value {
    match outcome {
        MerkleProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => json!({
            "status": "found",
            "leafIndex": leaf_index,
            "leafCount": leaf_count,
            "siblings": siblings.iter().map(|sibling| hex(&sibling[..])).collect::<Vec<_>>(),
        }),
        MerkleProofOutcome::NotFound { leaf_count } => {
            json!({ "status": "notFound", "leafCount": leaf_count })
        }
        MerkleProofOutcome::UnknownAccount => json!({ "status": "unknownAccount" }),
        MerkleProofOutcome::Inconsistent => json!({ "status": "inconsistent" }),
    }
}

fn render_verdict((name, entry): (&str, Option<usize>), recoverable: bool) -> Value {
    json!({ "authorized": false, "failure": name, "entry": entry, "recoverable": recoverable })
}

fn render_case(case: &Case, failure: Option<&AuthorizationFailure>, batch: &[LeafQuery]) -> Value {
    let request = &case.request.request;
    let permit = request.permit();
    assert_eq!(
        permit.transport_key().as_bytes(),
        &TRANSPORT_KEY,
        "{}: every case signs the fixture's transport key",
        case.name
    );
    let verdict = failure.map_or(json!({ "authorized": true }), |failure| {
        render_verdict(failure_name(failure), failure.is_recoverable())
    });
    json!({
        "name": case.name,
        "now": case.now.to_string(),
        "permit": {
            "user_address": hex(permit.user_address().as_bytes()),
            "allowed_scopes": permit.allowed_scopes().as_slice().iter().map(|scope| hex(scope.as_bytes())).collect::<Vec<_>>(),
            "start_timestamp": permit.start_timestamp().to_string(),
            "duration_seconds": permit.duration_seconds().to_string(),
            "verifying_program_id": hex(permit.verifying_program_id().as_bytes()),
            "chain_id": permit.chain_id().to_string(),
            "extra_data": hex(&permit.extra_data().to_extra_data()),
        },
        "signature": hex(request.signature().as_bytes()),
        "entries": request.entries().iter().map(|entry| json!({
            "handle": hex(&entry.handle),
            "owner_address": hex(&entry.owner_address),
            "encrypted_store": hex(&entry.encrypted_store),
        })).collect::<Vec<_>>(),
        "accounts": render_accounts(&case.world),
        "leaf_read": render_leaf_read(batch, &case.record),
        "verdict": verdict,
    })
}

fn render_public_case(
    case: &PublicCase,
    failure: Option<&PublicDecryptFailure>,
    batch: &[LeafQuery],
) -> Value {
    json!({
        "name": case.name,
        "handles": case.handles.iter().map(|(handle, store)| json!({
            "handle": hex(handle),
            "encrypted_store": hex(store.as_ref()),
        })).collect::<Vec<_>>(),
        "accounts": render_accounts(&case.world),
        "leaf_read": render_leaf_read(batch, &case.record),
        "verdict": failure.map_or(json!({ "authorized": true }), |failure| {
            render_verdict(public_failure_name(failure), failure.is_recoverable())
        }),
    })
}

#[tokio::test]
async fn the_committed_cases_are_the_connectors_verdicts() {
    let mut rendered = Vec::new();
    let mut produced = Vec::new();
    for case in cases() {
        let reader = ScriptedReader::constant(case.world.clone());
        let proofs = ScriptedProofReader::constant(case.record.clone());
        let failure = authorize_request(&reader, &proofs, context_at(case.now), &case.request)
            .await
            .err();
        assert_eq!(
            failure.as_ref().map(failure_name),
            case.expected,
            "{}: {failure:?}",
            case.name,
        );
        let calls = proofs.calls();
        assert!(
            calls.len() <= 1,
            "{}: one Merkle proof batch at most",
            case.name
        );
        let batch = calls
            .into_iter()
            .next()
            .map_or_else(Vec::new, |(_, batch)| batch);
        rendered.push(render_case(&case, failure.as_ref(), &batch));
        produced.extend(case.expected.map(|(name, _)| name));
    }
    for name in FAILURES {
        let rust_only = RUST_ONLY.iter().any(|(listed, _)| *listed == name);
        assert!(
            produced.contains(&name) != rust_only,
            "{name} must be produced by a case or listed as Rust-only, not both or neither"
        );
    }

    let mut rendered_public = Vec::new();
    for case in public_cases() {
        let (failure, batch) = judge_public(&case).await;
        assert_eq!(
            failure.as_ref().map(public_failure_name),
            case.expected,
            "{}: {failure:?}",
            case.name,
        );
        rendered_public.push(render_public_case(&case, failure.as_ref(), &batch));
    }

    let file = json!({
        "schema": "zama-solana-decrypt-authorization-cases/v1",
        "description": "The KMS Connector's verdicts on user and public decryptions, each over one host state and one leaf record. `leaf_read` is the Merkle proof batch the Connector asked for, with the record's answers, with the fields of POST /v1/solana/merkle-proofs; null when it asked for none. Bytes are hex, 64-bit numbers decimal strings, addresses base58, account data base64. A missing account is absent. Every read of a case's accounts reports `slot`. The SDK's cleartext client must reach the same verdicts.",
        "generator": "ZAMA_UPDATE_AUTHORIZATION_CASES=1 cargo test -p kms-worker --test solana_authorization_cases",
        "host_program": PROGRAM_ID.to_string(),
        "slot": SLOT.to_string(),
        "transport_key": hex(&TRANSPORT_KEY),
        "rust_only": RUST_ONLY.iter().map(|(failure, reason)| json!({ "failure": failure, "reason": reason })).collect::<Vec<_>>(),
        "user_decrypt_cases": rendered,
        "public_decrypt_cases": rendered_public,
    });
    let rendered = serde_json::to_string_pretty(&file).expect("the fixture serializes") + "\n";
    if std::env::var_os(UPDATE_ENV).is_some() {
        std::fs::write(FIXTURE, &rendered).expect("the fixture is writable");
        return;
    }
    let committed = std::fs::read_to_string(FIXTURE).unwrap_or_default();
    assert!(
        committed == rendered,
        "{FIXTURE} is out of date; rewrite it with {UPDATE_ENV}=1 and commit it"
    );
}
