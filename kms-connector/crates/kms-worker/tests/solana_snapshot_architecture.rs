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
//! * each invocation uses one deciding snapshot; worker retries authorize again;
//! * a permit is reusable, but no authorization result is cached — every request pays for its
//!   own observation.
//!
//! The instrument is a reader that answers from a scripted world and counts calls. Without it,
//! "reads state once" is a claim about code that no test can hold to account: the difference
//! between one read and two is invisible in the outcome and very visible in a race. The leaf
//! record has its own counted reader, for the same reason: the proof read is one batch per
//! request, repeated once for unresolved retryable proofs, and never a third time.

mod solana_support;

use kms_worker::core::solana::{
    delegation::{AuthorizedRow, DelegationFailure, check_delegation},
    encrypted_store::{EncryptedStoreFailure, ResolvedEncryptedStore, resolve_encrypted_store},
    failure::AuthorizationFailure,
    handle_binding::{HandleBindingFailure, check_handle_binding},
    pipeline::authorize_request,
    proof::LeafProofOutcome,
    scope::{ScopeFailure, check_scope},
    snapshot::{
        HostSnapshot, HostStateReader, SnapshotAccount, SnapshotError, SnapshotKeys,
        SolanaRpcClient,
    },
    watermark::{WatermarkFailure, read_watermark},
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_pubkey::Pubkey;
use solana_support::*;

/// A direct request under a valid permit, against a world that authorizes it.
fn direct_scenario() -> (Wallet, EncryptedStoreFixture, [u8; 32]) {
    let wallet = Wallet::new(1);
    let handle = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, wallet.pubkey());
    (wallet, encrypted_store, handle)
}

/// Every key the direct branch will ever look at is derivable from the request and the
/// deployment alone, so authorizing a direct request costs exactly one account read.
#[tokio::test]
async fn authorizing_a_direct_request_reads_host_state_once() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live handle owned by the signer authorizes");

    assert_eq!(
        reader.call_count(),
        1,
        "a direct-only request must be authorized from a single account read"
    );
}

/// A delegated entry's delegation record lives at a PDA seeded by the encrypted store
/// authority, and that authority is a field of the account — so the record's address is not
/// computable until the encrypted store has been read. That costs a second read and nothing
/// beyond it: no rule after the deciding observation reads state at all.
#[tokio::test]
async fn authorizing_a_delegated_request_reads_host_state_twice_and_never_more() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x20, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    // Three worlds are scripted although two reads are expected: a third read would find a
    // world and fail the count assertion below, rather than panicking inside the reader with a
    // less specific message.
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::scripted(vec![world.clone(), world.clone(), world]);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live delegation authorizes a delegated entry");

    assert_eq!(
        reader.call_count(),
        2,
        "a delegated request is authorized from exactly two reads"
    );
}

/// The second read re-reads every key the rules are evaluated against. That is what makes it a
/// complete observation on its own: an encrypted store or an invalidation record missing
/// from it would have to be taken from the discarded read.
///
/// The one key it does not carry is the config singleton, and it is not an exception to that: the
/// pause switch is decided on the first read and no rule below the reads looks at it. Its own
/// property is pinned by `the_deciding_read_drops_the_config_singleton`.
#[tokio::test]
async fn the_second_read_carries_over_every_key_of_the_first() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live delegation authorizes a delegated entry");

    let first = reader.call(0);
    let second = reader.call(1);
    let (host_config_key, _) = host_config_address();
    for key in first
        .as_slice()
        .iter()
        .filter(|key| *key != &host_config_key)
    {
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

/// A delegated entry plans both rows that could carry its grant — the encrypted store's
/// encrypted store authority and the delegator's wildcard row — in the same read. Fetching
/// the wildcard row only when the authority-specific one is missing would be a third read, and nothing in
/// this pipeline reads state after the deciding observation.
///
/// Two entries under two authorities of one delegator show how the two kinds of row scale: an
/// authority row per authority, and one wildcard row however many authorities there are, because
/// its address does not mention an authority at all. The batch is also mixed by construction — the
/// first entry is authorized by its authority row, the second only by the wildcard row.
#[tokio::test]
async fn a_delegated_entry_plans_both_of_its_delegation_rows() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let first_handle = handle(0x27, FHE_TYPE_UINT64);
    let second_handle = handle(0x28, FHE_TYPE_UINT64);
    let other_authority: SolanaPubkeyBytes = [0x5a; 32];
    let mut other_label = LABEL;
    other_label[0] = b'a';
    let first_encrypted_store = EncryptedStoreFixture::allowing(first_handle, delegator.pubkey());
    let mut second_encrypted_store = EncryptedStoreFixture::in_application(
        APP_PROGRAM,
        other_authority,
        SCOPE,
        other_label,
        second_handle,
    );
    second_encrypted_store.allow(delegator.pubkey());
    let app_row = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let wildcard_row = DelegationFixture::live_wildcard(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&first_encrypted_store, first_handle, delegator.pubkey())
        .delegated(&second_encrypted_store, second_handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&first_encrypted_store)
        .with_encrypted_store(&second_encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&app_row)
        .with_delegation(&wildcard_row);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
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
        // the invalidation record, two encrypted stores, one row per authority, one
        // wildcard row for both — and not the config singleton, which the deciding read drops
        1 + 2 + 2 + 1,
        "the wildcard row is per delegator, so a second authority adds its row and no second wildcard"
    );
    assert_eq!(reader.call_count(), 2, "still two reads, never a third");
}

/// There is no scan in the authorization path: the first read's key set is a pure function of the
/// request and the deployment, which is what "known before the first read" means operationally. The
/// set is exactly the deployment's config singleton, the signer's invalidation record and one
/// encrypted store per named encrypted store.
#[tokio::test]
async fn every_account_key_is_planned_before_the_first_read() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    let planned = kms_worker::core::solana::snapshot::plan_first_read(&request, PROGRAM_ID);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live handle owned by the signer authorizes");

    assert_eq!(
        reader.call(0),
        planned,
        "the first read must ask for exactly the planned key set"
    );
    let (watermark_key, _) = invalidation_address(wallet.pubkey());
    let (host_config_key, _) = host_config_address();
    assert!(
        planned.contains(&watermark_key)
            && planned.contains(&encrypted_store.account_key)
            && planned.contains(&host_config_key),
        "the plan covers the config singleton, the signer's invalidation record and the named \
         encrypted store"
    );
    assert_eq!(
        reader.call_count(),
        1,
        "the pause switch rides the read the request already makes"
    );
}

/// The config singleton is the one key the deciding read drops: the pause switch was spent on the
/// first read, and carrying the account into the second would cost the read an account it no
/// longer uses. That single key is the difference between the worst-case delegated read
/// saturating the RPC's hundred-account limit and exceeding it.
#[tokio::test]
async fn the_deciding_read_drops_the_config_singleton() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x29, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live delegation authorizes a delegated entry");

    let (host_config_key, _) = host_config_address();
    assert!(
        reader.call(0).contains(&host_config_key),
        "the switch is read on the first read"
    );
    assert!(
        !reader.call(1).contains(&host_config_key),
        "and is not carried into the deciding read, which is sized without it"
    );
}

/// Two entries naming the same encrypted store — including the same handle twice, which is
/// legal — are one account. Reading it twice would be a second chance for the two copies to
/// disagree.
#[tokio::test]
async fn repeated_encrypted_stores_are_read_once() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .direct(&encrypted_store, handle)
        .typed();

    let planned = kms_worker::core::solana::snapshot::plan_first_read(&request, PROGRAM_ID);

    assert_eq!(
        planned.len(),
        3,
        "a request naming one encrypted store twice plans the encrypted store once, beside the config singleton and the watermark"
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
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::scripted(vec![world.clone(), world.at(101)]);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("the deciding observation is the second read, not an agreement of the two");

    assert_eq!(reader.call_count(), 2);
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
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    // The same state throughout: the only difference between the reads is which node answered.
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::scripted(vec![world.clone(), world.at(99)]);
    let proofs = ScriptedProofReader::unreachable();

    let failure = authorize_request(&reader, &proofs, CONTEXT, &request)
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
    assert!(failure.is_recoverable());
}

/// Equal slots are not a regression: two reads of one slot are the ordinary case when the chain
/// has not advanced between the round trips, and the second is still the deciding one.
#[tokio::test]
async fn two_reads_at_the_same_slot_authorize() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x26, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);

    authorize_request(
        &ScriptedReader::constant(world.clone()),
        &ScriptedProofReader::constant(world.record()),
        CONTEXT,
        &request,
    )
    .await
    .expect("ordering is not agreement: one slot twice is in order");
}

/// The gate is on the pair of reads, not on any absolute slot: a direct request reads once, so
/// there is no earlier read for its observation to be older than.
#[test]
fn the_ordering_gate_compares_the_two_reads_and_nothing_else() {
    let keys = SnapshotKeys::new([[7; 32]]);
    let discovery = World::running_at_slot(100)
        .read(&keys)
        .expect("the world reads");
    let ahead = World::running_at_slot(101)
        .read(&keys)
        .expect("the world reads");
    let level = World::running_at_slot(100)
        .read(&keys)
        .expect("the world reads");
    let behind = World::running_at_slot(99)
        .read(&keys)
        .expect("the world reads");

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
/// read's. Here the first read shows the delegator's allow leaf on the handle and the second read
/// shows an account on which that leaf was never sealed: the entry is refused, because the
/// earlier, more favorable peaks are gone and were never a candidate.
#[tokio::test]
async fn the_deciding_state_of_a_delegated_request_is_the_second_reads() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x23, FHE_TYPE_UINT64);
    let allowed = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let mut never_allowed = EncryptedStoreFixture::new(handle);
    never_allowed.allow(Wallet::new(9).pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated(&allowed, handle, delegator.pubkey())
        .typed();

    let first = World::running_at_slot(100)
        .with_encrypted_store(&allowed)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let second = World::running_at_slot(101)
        .with_encrypted_store(&never_allowed)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    // The record agrees with the deciding read.
    let proofs = ScriptedProofReader::constant(second.record());
    let reader = ScriptedReader::scripted(vec![first, second]);

    let failure = authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect_err("the delegator holds no allow leaf at the deciding observation");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::NoLeaf { .. }
            }
        ),
        "expected the deciding read's peaks to decide, got {failure}"
    );
    assert!(!failure.is_recoverable());
}

/// Missing leaves from a lagging record are fetched once more. If still missing, the request
/// is rejected retryably and the ordinary attempt budget decides.
#[tokio::test]
async fn the_leaf_record_is_retried_only_when_a_required_leaf_is_unavailable() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);

    let in_step = ScriptedProofReader::constant(world.record());
    authorize_request(
        &ScriptedReader::constant(world.clone()),
        &in_step,
        CONTEXT,
        &request,
    )
    .await
    .expect("a record in step authorizes");
    assert_eq!(in_step.call_count(), 1, "a record in step is read once");

    // A record that has not yet sealed the allow leaf, then catches up.
    let behind = ProofRecord::of(&[&EncryptedStoreFixture::new(handle)]);
    let catches_up = ScriptedProofReader::scripted(vec![behind.clone(), world.record()]);
    authorize_request(
        &ScriptedReader::constant(world.clone()),
        &catches_up,
        CONTEXT,
        &request,
    )
    .await
    .expect("the second read finds the leaf");
    assert_eq!(
        catches_up.call_count(),
        2,
        "a record behind is read once more"
    );

    // A record that stays behind: two reads, then a retryable rejection, never a third read.
    let stays_behind = ScriptedProofReader::scripted(vec![behind.clone(), behind]);
    let failure = authorize_request(
        &ScriptedReader::constant(world),
        &stays_behind,
        CONTEXT,
        &request,
    )
    .await
    .expect_err("a record still behind after the retry decides nothing");
    assert_eq!(
        stays_behind.call_count(),
        2,
        "the retry policy is one repeat, not a loop"
    );
    assert!(matches!(
        failure,
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::ProofRecordBehind { .. }
        }
    ));
    assert!(failure.is_recoverable());
}

/// Exercise the actual SDK transport; the mock matches commitment, encoding and ordered keys.
async fn rpc_read(
    keys: &SnapshotKeys,
    value: serde_json::Value,
) -> Result<HostSnapshot, SnapshotError> {
    let expected = serde_json::json!({"jsonrpc":"2.0","id":0,"method":"getMultipleAccounts",
        "params":[keys.as_slice().iter().map(|key| Pubkey::new_from_array(*key).to_string()).collect::<Vec<_>>(),
        {"encoding":"base64","commitment":"confirmed","dataSlice":null,"minContextSlot":null}]});
    let response = serde_json::json!({"jsonrpc":"2.0","id":0,"result":{"context":{"slot":4242},"value":value}});
    let mut server = mocktail::server::MockServer::new_http("solana-accounts");
    server.mock(move |when, then| {
        when.post().json(expected.clone());
        then.json(response.clone());
    });
    server.start().await.unwrap();
    SolanaRpcClient::new(
        server.base_url().unwrap().clone(),
        std::time::Duration::from_secs(1),
        std::num::NonZeroUsize::new(1).unwrap(),
    )
    .read_accounts(keys)
    .await
}

fn rpc_account() -> serde_json::Value {
    serde_json::json!({"owner":Pubkey::new_from_array(PROGRAM_ID).to_string(),
        "data":["AQID","base64"],"lamports":1,"executable":false,"rentEpoch":0})
}

#[tokio::test]
async fn confirmed_rpc_preserves_order_null_accounts_and_context_slot() {
    let keys = SnapshotKeys::new([[5; 32], [6; 32]]);
    let snapshot = rpc_read(&keys, serde_json::json!([rpc_account(), null]))
        .await
        .unwrap();
    assert_eq!(snapshot.observed_slot(), 4242);
    assert_eq!(
        snapshot.account(&[5; 32]).unwrap(),
        Some(&SnapshotAccount {
            owner: PROGRAM_ID,
            data: vec![1, 2, 3]
        })
    );
    assert_eq!(snapshot.account(&[6; 32]).unwrap(), None);
}

#[tokio::test]
async fn malformed_rpc_accounts_are_errors_never_missing_accounts() {
    let mut bad_base64 = rpc_account();
    bad_base64["data"][0] = "not base64!".into();
    let mut wrong_encoding = rpc_account();
    wrong_encoding["data"][1] = "base64+zstd".into();
    let mut bad_owner = rpc_account();
    bad_owner["owner"] = "not a pubkey".into();
    let mut missing_owner = rpc_account();
    missing_owner.as_object_mut().unwrap().remove("owner");
    let keys = SnapshotKeys::new([[5; 32]]);
    for account in [bad_base64, wrong_encoding, bad_owner, missing_owner] {
        assert!(matches!(
            rpc_read(&keys, serde_json::json!([account])).await,
            Err(SnapshotError::Unavailable { .. })
        ));
    }
    assert!(matches!(
        rpc_read(&keys, serde_json::json!([])).await,
        Err(SnapshotError::ResponseLengthMismatch {
            requested: 1,
            returned: 0
        })
    ));
}

#[tokio::test]
async fn a_full_hundred_account_snapshot_is_one_rpc_call() {
    let keys = SnapshotKeys::new((0..100).map(|i| [i; 32]));
    let snapshot = rpc_read(&keys, serde_json::json!(vec![None::<()>; 100]))
        .await
        .unwrap();
    for key in keys.as_slice() {
        assert_eq!(snapshot.account(key).unwrap(), None);
    }
}

/// Asking the snapshot for an account nobody planned is a defect in key planning, and it is
/// reported as one. Answering "absent" would turn a missing plan entry into a transient
/// rejection that looks like ordinary commitment lag and would survive review.
#[test]
fn an_account_that_was_never_planned_cannot_be_read_from_the_snapshot() {
    let planned = [7; 32];
    let never_planned: SolanaPubkeyBytes = [8; 32];
    let snapshot = World::running_at_slot(1)
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

/// Individual checks use the deciding snapshot. A later worker attempt obtains a fresh one.
#[test]
fn authorization_checks_take_the_observation_and_never_a_reader() {
    let _resolve_encrypted_store: fn(
        &HostSnapshot,
        SolanaPubkeyBytes,
        [u8; 32],
    ) -> Result<ResolvedEncryptedStore, EncryptedStoreFailure> = resolve_encrypted_store;

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
    ) -> Result<AuthorizedRow, DelegationFailure> = check_delegation;

    let _check_handle_binding: fn(
        &ResolvedEncryptedStore,
        [u8; 32],
        SolanaPubkeyBytes,
        &LeafProofOutcome,
    ) -> Result<(), HandleBindingFailure> = check_handle_binding;

    let _check_scope: fn(
        &zama_solana_permit::AllowedScopes,
        &ResolvedEncryptedStore,
    ) -> Result<(), ScopeFailure> = check_scope;
}

#[tokio::test]
async fn rpc_throttling_retries_fit_inside_the_configured_deadline() {
    let mut server = mocktail::server::MockServer::new_http("throttled-rpc");
    server.mock(|when, then| {
        when.post();
        then.status(mocktail::StatusCode::TOO_MANY_REQUESTS);
    });
    server.start().await.unwrap();
    let client = SolanaRpcClient::new(
        server.base_url().unwrap().clone(),
        std::time::Duration::from_millis(50),
        std::num::NonZeroUsize::new(1).unwrap(),
    );
    let result = tokio::time::timeout(
        std::time::Duration::from_millis(250),
        client.read_accounts(&SnapshotKeys::new([[1; 32]])),
    )
    .await;
    assert!(
        result
            .expect("SDK retries must fit inside the request deadline")
            .is_err()
    );
}

#[tokio::test]
async fn rpc_limit_is_shared_across_clones_and_cancellation_releases_it() {
    use std::{num::NonZeroUsize, time::Duration};
    use tokio::{
        io::{AsyncReadExt, AsyncWriteExt},
        net::TcpListener,
        time::timeout,
    };

    let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
    let client = SolanaRpcClient::new(
        format!("http://{}", listener.local_addr().unwrap())
            .parse()
            .unwrap(),
        Duration::from_secs(5),
        NonZeroUsize::new(1).unwrap(),
    );
    let first_client = client.clone();
    let first = tokio::spawn(async move {
        first_client
            .read_accounts(&SnapshotKeys::new([[1; 32]]))
            .await
    });
    let (_held, _) = timeout(Duration::from_secs(1), listener.accept())
        .await
        .unwrap()
        .unwrap();
    let second =
        tokio::spawn(async move { client.read_accounts(&SnapshotKeys::new([[2; 32]])).await });
    assert!(
        timeout(Duration::from_millis(100), listener.accept())
            .await
            .is_err()
    );
    first.abort();
    assert!(first.await.unwrap_err().is_cancelled());
    let (mut stream, _) = timeout(Duration::from_secs(1), listener.accept())
        .await
        .unwrap()
        .unwrap();
    stream.read_exact(&mut [0]).await.unwrap();
    let body = r#"{"jsonrpc":"2.0","id":1,"result":{"context":{"slot":42},"value":[null]}}"#;
    stream.write_all(format!("HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\n\r\n{}", body.len(), body).as_bytes()).await.unwrap();
    assert_eq!(second.await.unwrap().unwrap().observed_slot(), 42);
}
