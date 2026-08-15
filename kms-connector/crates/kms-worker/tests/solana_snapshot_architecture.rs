//! The observation point: how many times authorization reads host state, what it reads, and
//! what it is allowed to conclude from two reads.
//!
//! This file is the architectural half of the suite. Every other group asserts what a rule
//! decides; this one asserts the shape of the machine the rules run in, because that shape is
//! what makes the rest of the properties true:
//!
//! * authorization cannot be assembled from states that never coexisted — every rule is
//!   evaluated against one read, and when a delegated entry forces a second one, the earlier
//!   read produces addresses rather than decisions;
//! * a request accepted at its observation point is not re-evaluated, because after the reads
//!   nothing in the path can read anything;
//! * a permit is reusable, but no authorization result is cached — every request pays for its
//!   own observation.
//!
//! The instrument is a reader that answers from a scripted world and counts calls. Without it,
//! "reads state once" is a claim about code that no test can hold to account: the difference
//! between one read and two is invisible in the outcome and very visible in a race.

mod solana_support;

use kms_worker::core::solana::{
    delegation::{DelegationFailure, check_delegation},
    deployment::DeploymentIdentity,
    encrypted_value_account::{
        EncryptedValueAccountFailure, ResolvedEncryptedValueAccount,
        resolve_encrypted_value_account,
    },
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::{HandleBindingFailure, check_handle_binding},
    pipeline::{AuthorizationContext, authorize_request},
    request::AccessEvidence,
    scope::{ScopeFailure, check_scope},
    snapshot::{
        HostSnapshot, SnapshotAccount, SnapshotError, SnapshotKeys, multiple_accounts_request_body,
        parse_multiple_accounts_response,
    },
    watermark::{WatermarkFailure, read_watermark},
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_pubkey::Pubkey;
use solana_support::*;

/// A direct request under a valid permit, against a world that authorizes it.
fn direct_scenario() -> (Wallet, EncryptedValueAccountFixture, [u8; 32]) {
    let wallet = Wallet::new(1);
    let handle = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[wallet.pubkey()]);
    (wallet, encrypted_value_account, handle)
}

fn context<'a>(
    deployment: &'a DeploymentIdentity,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    }
}

/// Every key the direct branch will ever look at is derivable from the request and the
/// deployment alone, so authorizing a direct request costs exactly one account read.
#[tokio::test]
async fn authorizing_a_direct_request_reads_host_state_once() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a live handle owned by the signer authorizes");

    assert_eq!(
        reader.call_count(),
        1,
        "a direct-only request must be authorized from a single account read"
    );
}

/// A delegated entry's delegation record lives at a PDA seeded by the encrypted value account
/// authority, and that authority is a field of the account — so the record's address is not
/// computable until the encrypted value account has been read. That costs a second read and nothing
/// beyond it: no rule after the deciding observation reads state at all.
#[tokio::test]
async fn authorizing_a_delegated_request_reads_host_state_twice_and_never_more() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x20, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    // Three worlds are scripted although two reads are expected: a third read would find a
    // world and fail the count assertion below, rather than panicking inside the reader with a
    // less specific message.
    let reader = ScriptedReader::scripted(vec![world.clone(), world.clone(), world]);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a live delegation authorizes a delegated entry");

    assert_eq!(
        reader.call_count(),
        2,
        "a delegated request is authorized from exactly two reads"
    );
}

/// The second read re-reads everything the first read saw. That is what makes it a complete
/// observation on its own: the rules are evaluated against it alone, so an encrypted value account
/// or an invalidation record missing from it would have to be taken from the discarded read.
#[tokio::test]
async fn the_second_read_carries_over_every_key_of_the_first() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a live delegation authorizes a delegated entry");

    let first = reader.call(0);
    let second = reader.call(1);
    for key in first.as_slice() {
        assert!(
            second.contains(key),
            "the second read dropped a key the first read observed"
        );
    }
    let (delegation_key, _) = delegation.address();
    assert!(
        second.contains(&delegation_key) && !first.contains(&delegation_key),
        "the delegation record is what the second read adds"
    );
}

/// A delegated entry plans both rows that could carry its grant — the encrypted value account's
/// encrypted value account authority and the delegator's wildcard row — in the same read. Fetching
/// the wildcard row only when the authority-specific one is missing would be a third read, and nothing in
/// this pipeline reads state after the deciding observation.
///
/// Two entries in two apps under one delegator show how the two kinds of row scale: an app row per
/// app, and one wildcard row however many apps there are, because its address does not mention an
/// app at all. The batch is also mixed by construction — the first entry is authorized by its app
/// row, the second only by the wildcard row.
#[tokio::test]
async fn a_delegated_entry_plans_both_of_its_delegation_rows() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let first_handle = handle(0x27, FHE_TYPE_UINT64);
    let second_handle = handle(0x28, FHE_TYPE_UINT64);
    let other_app: SolanaPubkeyBytes = [0x5a; 32];
    let mut other_label = LABEL;
    other_label[0] = b'a';
    let first_encrypted_value_account =
        EncryptedValueAccountFixture::new(first_handle, &[delegator.pubkey()]);
    let second_encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        other_app,
        other_label,
        second_handle,
        &[delegator.pubkey()],
    );
    let app_row = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let wildcard_row = DelegationFixture::live_wildcard(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(
            &first_encrypted_value_account,
            first_handle,
            delegator.pubkey(),
        )
        .delegated_current(
            &second_encrypted_value_account,
            second_handle,
            delegator.pubkey(),
        )
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&first_encrypted_value_account)
        .with_encrypted_value_account(&second_encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&app_row)
        .with_delegation(&wildcard_row);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("one entry stands on its app row, the other on the wildcard row");

    let second = reader.call(1);
    let (app_key, _) = app_row.address();
    let (wildcard_key, _) = wildcard_row.address();
    assert!(
        second.contains(&app_key) && second.contains(&wildcard_key),
        "both rows that could authorize an entry have to be in the deciding read"
    );
    assert_eq!(
        second.len(),
        // the invalidation record, two encrypted value accounts, one app row per app, one wildcard
        // row for both
        1 + 2 + 2 + 1,
        "the wildcard row is per delegator, so a second app adds an app row and no second wildcard"
    );
    assert_eq!(reader.call_count(), 2, "still two reads, never a third");
}

/// There is no scan in the authorization path: the first read's key set is a pure function of the
/// request and the deployment, which is what "known before the first read" means operationally. The
/// set is exactly the signer's invalidation record plus one encrypted value account per named
/// encrypted value account.
#[tokio::test]
async fn every_account_key_is_planned_before_the_first_read() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let planned = kms_worker::core::solana::snapshot::plan_first_read(&request, &deployment);

    authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a live handle owned by the signer authorizes");

    assert_eq!(
        reader.call(0),
        planned,
        "the first read must ask for exactly the planned key set"
    );
    let (watermark_key, _) = invalidation_address(wallet.pubkey());
    assert!(
        planned.contains(&watermark_key) && planned.contains(&encrypted_value_account.account_key),
        "the plan covers the signer's invalidation record and the named encrypted value account"
    );
}

/// Two entries naming the same encrypted value account — including the same handle twice, which is
/// legal — are one account. Reading it twice would be a second chance for the two copies to
/// disagree.
#[tokio::test]
async fn repeated_encrypted_value_accounts_are_read_once() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let deployment = deployment();

    let planned = kms_worker::core::solana::snapshot::plan_first_read(&request, &deployment);

    assert_eq!(
        planned.len(),
        2,
        "a request naming one encrypted value account twice plans the encrypted value account once, beside the watermark"
    );
}

/// The chain advancing between the two reads is not a failure. Requiring the reads to agree
/// would reject delegated requests at whatever rate slots advance between two round trips — a
/// slot is about 400ms — and it would prove nothing, because the second read is one
/// `getMultipleAccounts` at one context slot and is therefore already one observation point.
/// The first read only produced the delegation addresses.
#[tokio::test]
async fn a_slot_change_between_the_two_reads_does_not_fail_the_request() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x22, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::scripted(vec![world.clone(), world.at(101)]);
    let deployment = deployment();

    let authorized = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("the deciding observation is the second read, not an agreement of the two");

    assert_eq!(
        authorized.observed_slot(),
        101,
        "the recorded observation point is the read the rules were evaluated against"
    );
}

/// The chain going *backwards* between the two reads is a failure, and a transient one. Behind a
/// load balancer this is a second node that has fallen behind, not a later state: judging the
/// request on it would report the delegation the discovery read just saw as absent, which is
/// terminal. A retry that lands on a node which has caught up authorizes the same request.
#[tokio::test]
async fn a_deciding_read_older_than_the_discovery_read_is_refused_transiently() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x25, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    // The same state throughout: the only difference between the reads is which node answered.
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::scripted(vec![world.clone(), world.at(99)]);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("a deciding read behind the discovery read decides nothing");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::Snapshot(SnapshotError::DecidingReadOlderThanDiscovery {
                discovery_slot: 100,
                deciding_slot: 99,
            })
        ),
        "expected the ordering failure, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// Equal slots are not a regression: two reads of one slot are the ordinary case when the chain
/// has not advanced between the round trips, and the second is still the deciding one.
#[tokio::test]
async fn two_reads_at_the_same_slot_authorize() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x26, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let deployment = deployment();

    let authorized = authorize_request(
        &ScriptedReader::constant(world),
        &ServableKmsPair,
        context(&deployment),
        &request,
    )
    .await
    .expect("ordering is not agreement: one slot twice is in order");

    assert_eq!(authorized.observed_slot(), 100);
}

/// The gate is on the pair of reads, not on any absolute slot: a direct request reads once, so
/// there is no earlier read for its observation to be older than.
#[test]
fn the_ordering_gate_compares_the_two_reads_and_nothing_else() {
    let keys = SnapshotKeys::new([[7; 32]]);
    let discovery = World::at_slot(100).read(&keys).expect("the world reads");
    let ahead = World::at_slot(101).read(&keys).expect("the world reads");
    let level = World::at_slot(100).read(&keys).expect("the world reads");
    let behind = World::at_slot(99).read(&keys).expect("the world reads");

    assert_eq!(
        ahead
            .deciding_after(&discovery)
            .expect("advancing is the expected case")
            .observed_slot(),
        101
    );
    assert_eq!(
        level
            .deciding_after(&discovery)
            .expect("the same slot is in order")
            .observed_slot(),
        100
    );
    assert!(matches!(
        behind.deciding_after(&discovery),
        Err(SnapshotError::DecidingReadOlderThanDiscovery {
            discovery_slot: 100,
            deciding_slot: 99,
        })
    ));
}

/// The state a delegated request is judged against is the deciding read's, not the discovery
/// read's. Here the handle is live in the first read and replaced in the second: the entry
/// claims current access, and it is refused — the earlier, more favorable bytes are gone and
/// were never a candidate.
#[tokio::test]
async fn the_deciding_state_of_a_delegated_request_is_the_second_reads() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x23, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(handle, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, handle, delegator.pubkey())
        .typed();

    let mut replaced = encrypted_value_account.clone();
    replaced.update(handle_on_chain(0x24, FHE_TYPE_UINT64, CHAIN_ID));

    let first = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let second = World::at_slot(101)
        .with_encrypted_value_account(&replaced)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::scripted(vec![first, second]);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("the handle is no longer current at the deciding observation");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::NotCurrentHandle { .. }
            }
        ),
        "expected the deciding read's update, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// Every authorization read asks for `confirmed`. A grant observed on a supermajority-confirmed
/// fork is sufficient authorization here, and the level is pinned in the request body rather
/// than left to whatever the endpoint defaults to.
#[test]
fn the_account_read_pins_confirmed_commitment() {
    let keys = SnapshotKeys::new([[3; 32], [4; 32]]);

    let body = multiple_accounts_request_body(&keys);

    assert_eq!(body["method"], "getMultipleAccounts");
    let params = &body["params"];
    assert_eq!(
        params[0][0].as_str().unwrap(),
        Pubkey::new_from_array([3; 32]).to_string(),
        "keys are sent in the planned order, so the response can be zipped back onto it"
    );
    assert_eq!(
        params[0][1].as_str().unwrap(),
        Pubkey::new_from_array([4; 32]).to_string()
    );
    assert_eq!(params[1]["commitment"], "confirmed");
    assert_eq!(params[1]["encoding"], "base64");
}

/// The observation point is the state's own account of when it was observed — the response's
/// context slot — never a locally chosen number. Every slot comparison in the delegation rules
/// is against this value, so sourcing it locally would silently decide those rules.
#[test]
fn the_observed_slot_is_the_context_slot_of_the_response() {
    let key = [5; 32];
    let keys = SnapshotKeys::new([key]);
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "context": { "slot": 4_242 },
            "value": [{
                "owner": Pubkey::new_from_array(PROGRAM_ID).to_string(),
                "data": ["AQID", "base64"],
                "lamports": 1,
                "executable": false,
                "rentEpoch": 0,
            }]
        }
    })
    .to_string();

    let snapshot = parse_multiple_accounts_response(&body, &keys).expect("the response parses");

    assert_eq!(snapshot.observed_slot(), 4_242);
    assert_eq!(
        snapshot.account(&key).unwrap(),
        Some(&SnapshotAccount {
            owner: PROGRAM_ID,
            data: vec![1, 2, 3],
        })
    );
}

/// A key that has no account is an absence *inside* the snapshot, not a failure of the read. The
/// distinction decides the outcome: an absent encrypted value account is a rule outcome that may
/// resolve itself at a later observation, while a failed read tells us nothing about any account.
#[test]
fn a_missing_account_is_an_absence_in_the_snapshot_not_a_read_failure() {
    let key = [6; 32];
    let keys = SnapshotKeys::new([key]);
    let body = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": { "context": { "slot": 7 }, "value": [null] }
    })
    .to_string();

    let snapshot = parse_multiple_accounts_response(&body, &keys).expect("the response parses");

    assert_eq!(snapshot.observed_slot(), 7);
    assert_eq!(
        snapshot.account(&key).expect("the key was read"),
        None,
        "an account that does not exist is a known absence"
    );
}

/// Asking the snapshot for an account nobody planned is a defect in key planning, and it is
/// reported as one. Answering "absent" would turn a missing plan entry into a transient
/// rejection that looks like ordinary commitment lag and would survive review.
#[test]
fn an_account_that_was_never_planned_cannot_be_read_from_the_snapshot() {
    let planned = [7; 32];
    let never_planned: SolanaPubkeyBytes = [8; 32];
    let snapshot = World::at_slot(1)
        .with_account(
            planned,
            SnapshotAccount {
                owner: PROGRAM_ID,
                data: vec![],
            },
        )
        .read(&SnapshotKeys::new([planned]))
        .expect("the world reads");

    let error = snapshot
        .account(&never_planned)
        .expect_err("an unplanned key is not a legitimate question");

    assert!(matches!(
        error,
        SnapshotError::KeyNotInSnapshot { key } if key == never_planned
    ));
}

/// Every state-dependent check takes the observation and no way to reach the network. This is
/// the type-level half of "a request accepted at its observation point is not re-evaluated":
/// there is no check that *could* re-read, whatever a future caller wanted.
///
/// The delegation signature is also where the delegation counter used to be. It takes a
/// snapshot, a program id and three identities — the counter is not a parameter, because
/// pinning it would invalidate in-flight requests on every unrelated delegation update.
#[test]
fn authorization_checks_take_the_observation_and_never_a_reader() {
    let _resolve_encrypted_value_account: fn(
        &HostSnapshot,
        SolanaPubkeyBytes,
        [u8; 32],
    ) -> Result<
        ResolvedEncryptedValueAccount,
        EncryptedValueAccountFailure,
    > = resolve_encrypted_value_account;

    let _read_watermark: fn(
        &HostSnapshot,
        SolanaPubkeyBytes,
        SolanaPubkeyBytes,
    ) -> Result<u64, WatermarkFailure> = read_watermark;

    let _check_delegation: fn(
        &HostSnapshot,
        SolanaPubkeyBytes,
        SolanaPubkeyBytes,
        SolanaPubkeyBytes,
        SolanaPubkeyBytes,
    ) -> Result<(), DelegationFailure> = check_delegation;

    let _check_handle_binding: fn(
        &ResolvedEncryptedValueAccount,
        [u8; 32],
        SolanaPubkeyBytes,
        &AccessEvidence,
    ) -> Result<(), HandleBindingFailure> = check_handle_binding;

    let _check_scope: fn(
        &zama_solana_permit::AclDomainKeys,
        &ResolvedEncryptedValueAccount,
    ) -> Result<(), ScopeFailure> = check_scope;
}

/// An accepted request carries the point it was accepted at. Recording it is what lets the
/// rest of the system state which observation the handle set belongs to, instead of inferring
/// it from a later read — which is the failure mode this whole file exists to prevent.
#[tokio::test]
async fn an_accepted_request_records_its_observation_point() {
    let (wallet, encrypted_value_account, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, handle)
        .typed();
    let world = World::at_slot(9_000)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let authorized = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a live handle owned by the signer authorizes");

    assert_eq!(authorized.observed_slot(), 9_000);
    assert_eq!(
        authorized.entries().len(),
        1,
        "the accepted entry set is what the response will bind"
    );
}
