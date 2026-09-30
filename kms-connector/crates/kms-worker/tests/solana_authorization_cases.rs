//! The Connector's verdict on user decryptions, written to a fixture the SDK's cleartext client is
//! held to (`sdk/js-sdk/src/solana/cleartext/authorization.test.ts`). Each case runs the real
//! [`authorize_request`] over one world and one leaf record, and the committed file must be what
//! this run renders. `ZAMA_UPDATE_AUTHORIZATION_CASES=1` rewrites it.
//!
//! Every failure variant is either produced by a case or listed in [`RUST_ONLY`] with the reason
//! the cleartext client cannot produce it.

mod solana_support;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;
use kms_worker::core::solana::{
    delegation::DelegationFailure,
    encrypted_store::EncryptedStoreFailure,
    failure::AuthorizationFailure,
    handle_binding::HandleBindingFailure,
    pipeline::authorize_request,
    proof::{LeafKind, ProofReadError},
    snapshot::{SnapshotAccount, SnapshotError},
    watermark::{WatermarkFailure, WindowFailure},
};
use serde_json::{Value, json};
use solana_pubkey::Pubkey;
use solana_support::*;
use zama_solana_acl::WILDCARD_APP;
use zama_solana_permit::PermitWireFields;
use zama_solana_request::HandleEntry;

const FIXTURE: &str = concat!(
    env!("CARGO_MANIFEST_DIR"),
    "/../../../solana/test-fixtures/authorization/user_decrypt_cases_v1.json"
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
    "HandleBinding::HistoryIncomplete",
    "HandleBinding::ProofDoesNotVerify",
    "HandleBinding::LeafIndexOutOfRange",
    "Delegation::NoLiveDelegation",
    "Delegation::InvalidHostRecord",
];

const NODE: &str = "The cleartext client reads the chain through the RPC it is given; a read that fails \
                    or lags throws instead of judging the request.";
const COPROCESSORS: &str = "The cleartext client asks no coprocessor: it rebuilds each store's leaves \
                            from the store's own transactions.";

/// The failures no cleartext case can produce, with the reason.
const RUST_ONLY: [(&str, &str); 9] = [
    ("Snapshot::Unavailable", NODE),
    ("Snapshot::ResponseLengthMismatch", NODE),
    ("Snapshot::MalformedClock", NODE),
    ("Snapshot::NodeBehind", NODE),
    ("ProofRead::Unavailable", COPROCESSORS),
    ("ProofRead::ResponseLengthMismatch", COPROCESSORS),
    (
        "HandleBinding::AccountUnknownToProofRecord",
        "A history rebuilt from the store's transactions always knows the store.",
    ),
    (
        "HandleBinding::HistoryIncomplete",
        "The cleartext history reader never keeps a history with a gap.",
    ),
    (
        "HandleBinding::LeafIndexOutOfRange",
        "The cleartext client proves a leaf over the observed store's leaf count only.",
    ),
];

/// The variant a failure is, and the entry it names.
fn failure_name(failure: &AuthorizationFailure) -> (&'static str, Option<usize>) {
    use AuthorizationFailure as F;
    match failure {
        F::Signature(_) => ("Signature", None),
        F::Window(WindowFailure::NotYetValid { .. }) => ("Window::NotYetValid", None),
        F::Window(WindowFailure::Expired { .. }) => ("Window::Expired", None),
        F::ProgramIdMismatch { .. } => ("ProgramIdMismatch", None),
        F::Snapshot(source) => (
            match source {
                SnapshotError::Unavailable { .. } => "Snapshot::Unavailable",
                SnapshotError::ResponseLengthMismatch { .. } => "Snapshot::ResponseLengthMismatch",
                SnapshotError::MalformedClock => "Snapshot::MalformedClock",
                SnapshotError::NodeBehind => "Snapshot::NodeBehind",
            },
            None,
        ),
        F::Watermark(WatermarkFailure::Invalidated { .. }) => ("Watermark::Invalidated", None),
        F::Watermark(WatermarkFailure::InvalidHostRecord(_)) => {
            ("Watermark::InvalidHostRecord", None)
        }
        F::EncryptedStore { index, source } => (
            match source {
                EncryptedStoreFailure::Absent { .. } => "EncryptedStore::Absent",
                EncryptedStoreFailure::ForeignOwner { .. } => "EncryptedStore::ForeignOwner",
                EncryptedStoreFailure::NotAnEncryptedStore { .. } => {
                    "EncryptedStore::NotAnEncryptedStore"
                }
                EncryptedStoreFailure::AddressMismatch { .. } => "EncryptedStore::AddressMismatch",
                EncryptedStoreFailure::InvalidHostRecord(_) => "EncryptedStore::InvalidHostRecord",
            },
            Some(*index),
        ),
        F::ScopeNotAllowed { index, .. } => ("ScopeNotAllowed", Some(*index)),
        F::ProofRead(source) => (
            match source {
                ProofReadError::Unavailable { .. } => "ProofRead::Unavailable",
                ProofReadError::ResponseLengthMismatch { .. } => {
                    "ProofRead::ResponseLengthMismatch"
                }
            },
            None,
        ),
        F::HandleBinding { index, source } => (
            match source {
                HandleBindingFailure::NoLeaf { .. } => "HandleBinding::NoLeaf",
                HandleBindingFailure::ProofRecordBehind { .. } => {
                    "HandleBinding::ProofRecordBehind"
                }
                HandleBindingFailure::AccountUnknownToProofRecord => {
                    "HandleBinding::AccountUnknownToProofRecord"
                }
                HandleBindingFailure::HistoryIncomplete => "HandleBinding::HistoryIncomplete",
                HandleBindingFailure::ProofDoesNotVerify { .. } => {
                    "HandleBinding::ProofDoesNotVerify"
                }
                HandleBindingFailure::LeafIndexOutOfRange { .. } => {
                    "HandleBinding::LeafIndexOutOfRange"
                }
            },
            Some(*index),
        ),
        F::Delegation { index, source } => (
            match source {
                DelegationFailure::NoLiveDelegation { .. } => "Delegation::NoLiveDelegation",
                DelegationFailure::InvalidHostRecord(_) => "Delegation::InvalidHostRecord",
            },
            Some(*index),
        ),
    }
}

struct Case {
    name: &'static str,
    permit: PermitWireFields,
    signature: [u8; 64],
    entries: Vec<HandleEntry>,
    world: World,
    /// The stores as the coprocessors' leaf record, and the cleartext history, hold them.
    record: Vec<EncryptedStoreFixture>,
    expected: Option<(&'static str, Option<usize>)>,
    request: SolanaUserDecryptionRequestV1,
}

fn case(
    name: &'static str,
    request: RequestBuilder<'_>,
    world: World,
    record: &[&EncryptedStoreFixture],
    expected: Option<(&'static str, Option<usize>)>,
) -> Case {
    let (permit, signature) = request.signed_permit();
    Case {
        name,
        permit,
        signature: *signature.as_bytes(),
        entries: request.entries().to_vec(),
        world,
        record: record.iter().map(|store| (*store).clone()).collect(),
        expected,
        request: request.typed(),
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

    let mine = EncryptedStoreFixture::allowing(h1, signer);
    let theirs = EncryptedStoreFixture::allowing(h1, delegator);
    let strangers = EncryptedStoreFixture::allowing(h1, stranger);
    let delegation = DelegationFixture::live(delegator, signer);
    let wildcard = DelegationFixture::live_wildcard(delegator, signer);
    let outside = EncryptedStoreFixture::in_application(other_app, AUTHORITY, SCOPE, LABEL, h1);
    let mut outside_mine = outside.clone();
    outside_mine.allow(signer);

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
            &[&mine],
            None,
        ),
        case(
            "an allow leaf sealed after the handle was made public",
            request().direct(&public, h1),
            world().with_encrypted_store(&public),
            &[&public],
            None,
        ),
        case(
            "an allow leaf on a handle the store no longer holds",
            request().direct(&updated, h1),
            world().with_encrypted_store(&updated),
            &[&updated],
            None,
        ),
        case(
            "a delegated entry through the application's row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation),
            &[&theirs],
            None,
        ),
        case(
            "a delegated entry through the wildcard row beside a revoked application row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation.revoked())
                .with_delegation(&wildcard),
            &[&theirs],
            None,
        ),
        case(
            "a permit signed by another wallet",
            RequestBuilder::new(&other_wallet)
                .permit(PermitBuilder::new(signer))
                .entry(h1, signer, mine.account_key),
            world().with_encrypted_store(&mine),
            &[&mine],
            Some(("Signature", None)),
        ),
        case(
            "a permit that starts after now",
            request()
                .permit(PermitBuilder::new(signer).window(NOW_INSIDE_WINDOW + 1, DEFAULT_DURATION))
                .direct(&mine, h1),
            world().with_encrypted_store(&mine),
            &[&mine],
            Some(("Window::NotYetValid", None)),
        ),
        case(
            "an expired permit",
            request()
                .permit(PermitBuilder::new(signer).window(DEFAULT_START - 7_200, DEFAULT_DURATION))
                .direct(&mine, h1),
            world().with_encrypted_store(&mine),
            &[&mine],
            Some(("Window::Expired", None)),
        ),
        case(
            "a permit for another host program",
            request()
                .permit(PermitBuilder::new(signer).verifying_program(pubkey(8)))
                .direct(&mine, h1),
            world().with_encrypted_store(&mine),
            &[&mine],
            Some(("ProgramIdMismatch", None)),
        ),
        case(
            "a permit that started before its signer revoked permits",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_watermark(signer, DEFAULT_START + 1),
            &[&mine],
            Some(("Watermark::Invalidated", None)),
        ),
        case(
            "a permit that started in the second its signer revoked permits",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_watermark(signer, DEFAULT_START),
            &[&mine],
            None,
        ),
        case(
            "a prefunded invalidation address",
            request().direct(&mine, h1),
            world()
                .with_encrypted_store(&mine)
                .with_account(invalidation_address(signer).0, prefunded_account()),
            &[&mine],
            None,
        ),
        case(
            "an invalidation address holding another program's account",
            request().direct(&mine, h1),
            world().with_encrypted_store(&mine).with_account(
                invalidation_address(signer).0,
                foreign(invalidation_account(signer, 0)),
            ),
            &[&mine],
            Some(("Watermark::InvalidHostRecord", None)),
        ),
        case(
            "an entry naming no account",
            request().direct(&mine, h1),
            world(),
            &[],
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "an entry naming a prefunded address",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, prefunded_account()),
            &[],
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "an entry naming another program's account",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, foreign(mine.account())),
            &[],
            Some(("EncryptedStore::ForeignOwner", Some(0))),
        ),
        case(
            "an entry naming another host account",
            request().direct(&mine, h1),
            world().with_account(mine.account_key, delegation.account()),
            &[],
            Some(("EncryptedStore::NotAnEncryptedStore", Some(0))),
        ),
        case(
            "an entry naming a store copied to another address",
            request().entry(h1, signer, copied_to),
            world().with_account(copied_to, mine.account()),
            &[],
            Some(("EncryptedStore::AddressMismatch", Some(0))),
        ),
        case(
            "a store whose peaks do not match its leaf count",
            request().direct(&inconsistent, h1),
            world().with_encrypted_store(&inconsistent),
            &[&mine],
            Some(("EncryptedStore::InvalidHostRecord", Some(0))),
        ),
        case(
            "a store naming the wildcard application",
            request()
                .permit(PermitBuilder::new(signer).permissive())
                .direct(&sentinel, h1),
            world().with_encrypted_store(&sentinel),
            &[&sentinel],
            Some(("EncryptedStore::InvalidHostRecord", Some(0))),
        ),
        case(
            "a store outside the permit's scopes",
            request().direct(&outside_mine, h1),
            world().with_encrypted_store(&outside_mine),
            &[&outside_mine],
            Some(("ScopeNotAllowed", Some(0))),
        ),
        case(
            "a permissive permit and a store of any application",
            request()
                .permit(PermitBuilder::new(signer).permissive())
                .direct(&outside_mine, h1),
            world().with_encrypted_store(&outside_mine),
            &[&outside_mine],
            None,
        ),
        case(
            "a delegated entry with no delegation row",
            request().delegated(&theirs, h1, delegator),
            world().with_encrypted_store(&theirs),
            &[&theirs],
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
            &[&theirs],
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
        case(
            "a revoked delegation",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_delegation(&delegation.revoked()),
            &[&theirs],
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
        case(
            "a delegation row held by another program beside a live wildcard row",
            request().delegated(&theirs, h1, delegator),
            world()
                .with_encrypted_store(&theirs)
                .with_account(delegation.address().0, foreign(delegation.account()))
                .with_delegation(&wildcard),
            &[&theirs],
            Some(("Delegation::InvalidHostRecord", Some(0))),
        ),
        case(
            "no allow leaf for the signer",
            request().direct(&strangers, h1),
            world().with_encrypted_store(&strangers),
            &[&strangers],
            Some(("HandleBinding::NoLeaf", Some(0))),
        ),
        case(
            "a delegated entry backed only by the delegate's own leaf",
            request().delegated(&mine, h1, delegator),
            world()
                .with_encrypted_store(&mine)
                .with_delegation(&delegation),
            &[&mine],
            Some(("HandleBinding::NoLeaf", Some(0))),
        ),
        case(
            "a leaf record behind the store",
            request().direct(&ahead, h1),
            world().with_encrypted_store(&ahead),
            &[&strangers],
            Some(("HandleBinding::ProofRecordBehind", Some(0))),
        ),
        case(
            "a leaf record whose leaves disagree with the store's peaks",
            request().direct(&ahead, h1),
            world().with_encrypted_store(&ahead),
            &[&reordered],
            Some(("HandleBinding::ProofDoesNotVerify", Some(0))),
        ),
        case(
            "every entry's store is judged before any leaf",
            request().direct(&strangers, h1).direct(&second, h2),
            world()
                .with_encrypted_store(&strangers)
                .with_account(second.account_key, foreign(second.account())),
            &[&strangers],
            Some(("EncryptedStore::ForeignOwner", Some(1))),
        ),
        case(
            "a delegated entry's store is resolved before the watermark",
            request().delegated(&theirs, h1, delegator),
            world().with_watermark(signer, DEFAULT_START + 1),
            &[],
            Some(("EncryptedStore::Absent", Some(0))),
        ),
        case(
            "a direct entry's store is resolved after the watermark",
            request().direct(&mine, h1),
            world().with_watermark(signer, DEFAULT_START + 1),
            &[],
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
            &[&theirs, &outside_mine],
            Some(("Delegation::NoLiveDelegation", Some(0))),
        ),
    ]
}

fn hex(bytes: &[u8]) -> String {
    alloy::hex::encode(bytes)
}

fn render(cases: &[(Case, Option<AuthorizationFailure>)]) -> String {
    let rendered: Vec<Value> = cases
        .iter()
        .map(|(case, failure)| {
            let permit = &case.permit;
            let accounts: Vec<Value> = case
                .world
                .accounts()
                .map(|(address, account)| {
                    json!({
                        "address": address.to_string(),
                        "owner": account.owner.to_string(),
                        "data_base64": BASE64_STANDARD.encode(&account.data),
                    })
                })
                .collect();
            let records: Vec<Value> = case
                .record
                .iter()
                .map(|store| {
                    let leaves: Vec<Value> = store
                        .leaves
                        .iter()
                        .map(|leaf| match leaf.query.kind {
                            LeafKind::Allowed { key } => json!({
                                "kind": "allowed",
                                "handle": hex(leaf.query.handle.as_slice()),
                                "key": hex(key.as_ref()),
                            }),
                            LeafKind::Public => json!({
                                "kind": "public",
                                "handle": hex(leaf.query.handle.as_slice()),
                            }),
                        })
                        .collect();
                    json!({ "encrypted_store": store.account_key.to_string(), "leaves": leaves })
                })
                .collect();
            let verdict = match failure {
                None => json!({ "authorized": true }),
                Some(failure) => {
                    let (name, entry) = failure_name(failure);
                    let class = failure.class();
                    json!({
                        "authorized": false,
                        "failure": name,
                        "entry": entry,
                        "code": class.code.as_str(),
                        "recoverable": class.recoverable,
                    })
                }
            };
            json!({
                "name": case.name,
                "now": NOW_INSIDE_WINDOW.to_string(),
                "permit": {
                    "user_address": hex(&permit.user_address),
                    "transport_key": hex(&permit.transport_key),
                    "allowed_scopes": permit.allowed_scopes.iter().map(|scope| hex(scope)).collect::<Vec<_>>(),
                    "start_timestamp": permit.start_timestamp.to_string(),
                    "duration_seconds": permit.duration_seconds.to_string(),
                    "verifying_program_id": hex(&permit.verifying_program_id),
                    "chain_id": permit.chain_id.to_string(),
                    "extra_data": hex(&permit.extra_data),
                },
                "signature": hex(&case.signature),
                "entries": case.entries.iter().map(|entry| json!({
                    "handle": hex(&entry.handle),
                    "owner_address": hex(&entry.owner_address),
                    "encrypted_store": hex(&entry.encrypted_store),
                })).collect::<Vec<_>>(),
                "accounts": accounts,
                "records": records,
                "verdict": verdict,
            })
        })
        .collect();
    let file = json!({
        "schema": "zama-solana-user-decrypt-authorization-cases/v1",
        "description": "The KMS Connector's verdict on user decryptions over one host state and one leaf record each. Bytes are hex, 64-bit numbers decimal strings, addresses base58, account data base64. A missing account is absent. The SDK's cleartext client must reach the same verdict.",
        "regenerate_with": "ZAMA_UPDATE_AUTHORIZATION_CASES=1 cargo test -p kms-worker --test solana_authorization_cases",
        "host_program": PROGRAM_ID.to_string(),
        "rust_only": RUST_ONLY.iter().map(|(failure, reason)| json!({ "failure": failure, "reason": reason })).collect::<Vec<_>>(),
        "cases": rendered,
    });
    serde_json::to_string_pretty(&file).expect("the fixture serializes") + "\n"
}

#[tokio::test]
async fn the_committed_cases_are_the_connectors_verdicts() {
    let mut judged = Vec::new();
    for case in cases() {
        let reader = ScriptedReader::constant(case.world.clone());
        let proofs =
            ScriptedProofReader::constant(ProofRecord::of(&case.record.iter().collect::<Vec<_>>()));
        let failure = authorize_request(&reader, &proofs, CONTEXT, &case.request)
            .await
            .err();
        assert_eq!(
            failure.as_ref().map(failure_name),
            case.expected,
            "{}: {failure:?}",
            case.name,
        );
        judged.push((case, failure));
    }

    let produced: Vec<&str> = judged
        .iter()
        .filter_map(|(case, _)| case.expected.map(|(name, _)| name))
        .collect();
    for name in FAILURES {
        let rust_only = RUST_ONLY.iter().any(|(listed, _)| *listed == name);
        assert!(
            produced.contains(&name) != rust_only,
            "{name} must be produced by a case or listed as Rust-only, not both or neither"
        );
    }

    let rendered = render(&judged);
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
