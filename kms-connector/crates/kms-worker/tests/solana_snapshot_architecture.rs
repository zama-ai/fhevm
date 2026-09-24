//! The observation point: how many times authorization reads host state, what it reads, and
//! what it is allowed to conclude from two reads.
//!
//! This file is the architectural half of the suite. Every other group asserts what a rule
//! decides; this one asserts the shape of the machine the rules run in, because that shape is
//! what makes the rest of the properties true:
//!
//! * authorization cannot be assembled from states that never coexisted — every rule is
//!   evaluated against one read, and when a delegated entry forces a second one, the earlier
//!   read produces addresses rather than decisions, and the second is no older than the first;
//! * each invocation uses one deciding snapshot; worker retries authorize again;
//! * a permit is reusable, but no authorization result is cached — every request pays for its
//!   own observation.
//!
//! The instrument is a reader that answers from a scripted world and counts calls. Without it,
//! "reads state once" is a claim about code that no test can hold to account: the difference
//! between one read and two is invisible in the outcome and very visible in a race. The leaf
//! record has its own recording reader, for the same reason: the proof read asks each coprocessor
//! at most once, in configured order, and only for the queries still unresolved.

mod solana_support;

use kms_worker::core::solana::{
    failure::AuthorizationFailure,
    handle_binding::HandleBindingFailure,
    pipeline::authorize_request,
    snapshot::{
        AccountsRead, HostStateReader, SnapshotAccount, SnapshotError, SolanaRpcClient,
        read_positional,
    },
};
use solana_pubkey::Pubkey;
use solana_support::*;
use zama_solana_request::MAX_REQUEST_HANDLES;

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
    let world = World::at_slot(100)
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
    assert_eq!(reader.call(0).min_context_slot, None);
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
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
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
    assert_eq!(
        (
            reader.call(0).min_context_slot,
            reader.call(1).min_context_slot
        ),
        (None, Some(100)),
        "the second read asks for a node that has reached the first read's slot"
    );
}

/// The second read re-reads every key the rules are evaluated against. That is what makes it a
/// complete observation on its own: an encrypted store or an invalidation record missing
/// from it would have to be taken from the discarded read.
#[tokio::test]
async fn the_second_read_carries_over_every_key_of_the_first() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
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
    for key in &first.keys {
        assert!(
            second.keys.contains(key),
            "the second read dropped a key the first read observed"
        );
    }
    let (delegation_key, _) = delegation.address();
    assert!(
        second.keys.contains(&delegation_key) && !first.keys.contains(&delegation_key),
        "the delegation record is what the second read adds"
    );
}

/// A delegated entry plans both rows that could carry its grant — the row of the encrypted
/// store's application and the delegator's wildcard row — in the same read. Fetching the wildcard
/// row only when the application row is missing would be a third read, and nothing in this
/// pipeline reads state after the deciding observation.
///
/// Two entries in two applications of one delegator: each entry is read with its own two rows, so
/// the wildcard row, whose address names no application, is read once per entry. The batch is
/// also mixed by construction — the first entry is authorized by its application row, the second
/// only by the wildcard row.
#[tokio::test]
async fn a_delegated_entry_plans_both_of_its_delegation_rows() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let first_handle = handle(0x27, FHE_TYPE_UINT64);
    let second_handle = handle(0x28, FHE_TYPE_UINT64);
    let other_scope: Pubkey = Pubkey::new_from_array([0x5a; 32]);
    let first_encrypted_store = EncryptedStoreFixture::allowing(first_handle, delegator.pubkey());
    let mut second_encrypted_store = EncryptedStoreFixture::in_application(
        APP_PROGRAM,
        AUTHORITY,
        other_scope,
        LABEL,
        second_handle,
    );
    second_encrypted_store.allow(delegator.pubkey());
    let app_row = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let wildcard_row = DelegationFixture::live_wildcard(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .permit(PermitBuilder::new(signer.pubkey()).permissive())
        .delegated(&first_encrypted_store, first_handle, delegator.pubkey())
        .delegated(&second_encrypted_store, second_handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
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

    let second = reader.call(1).keys;
    let (app_key, _) = app_row.address();
    let (wildcard_key, _) = wildcard_row.address();
    assert!(
        second.contains(&app_key) && second.contains(&wildcard_key),
        "both rows that could authorize an entry have to be in the deciding read"
    );
    assert_eq!(
        second.len(),
        // the invalidation record, the Clock, two encrypted stores, two rows per entry
        1 + 1 + 2 + 2 * 2,
    );
    assert_eq!(reader.call_count(), 2, "still two reads, never a third");
}

/// The largest request the cap admits still fits one `getMultipleAccounts`: every entry
/// delegated, each in its own application and from its own delegator, so no key is shared.
#[tokio::test]
async fn the_largest_delegated_request_fits_one_account_read() {
    const RPC_MAX_MULTIPLE_ACCOUNTS: usize = 100;
    let signer = Wallet::new(1);
    let mut request =
        RequestBuilder::new(&signer).permit(PermitBuilder::new(signer.pubkey()).permissive());
    let mut world = World::at_slot(100).with_watermark(signer.pubkey(), 0);
    for index in 0..MAX_REQUEST_HANDLES as u8 {
        let delegator = Wallet::new(index + 2);
        let live = handle(index, FHE_TYPE_UINT64);
        let mut store = EncryptedStoreFixture::in_application(
            APP_PROGRAM,
            AUTHORITY,
            Pubkey::new_from_array([index; 32]),
            LABEL,
            live,
        );
        store.allow(delegator.pubkey());
        world = world.with_encrypted_store(&store);
        request = request.delegated(&store, live, delegator.pubkey());
    }
    let request = request.typed();
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect_err("no delegation row exists");

    let deciding = reader.call(1).keys;
    // the invalidation record, the Clock, and per entry its store and two rows
    assert_eq!(deciding.len(), 1 + 1 + 3 * MAX_REQUEST_HANDLES);
    assert!(deciding.len() <= RPC_MAX_MULTIPLE_ACCOUNTS);
}

/// There is no scan in the authorization path: the first read's keys are a pure function of the
/// request and the deployment. They are the signer's invalidation record and each entry's named
/// store, in entry order; a store named twice is read at both positions, in the same read, so the
/// two cannot disagree.
#[tokio::test]
async fn the_first_read_is_the_watermark_and_each_named_store_in_order() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);

    authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect("a live handle owned by the signer authorizes, however often it is named");

    let (watermark_key, _) = invalidation_address(wallet.pubkey());
    assert_eq!(
        reader.calls(),
        [ReadCall {
            keys: vec![
                watermark_key,
                encrypted_store.account_key,
                encrypted_store.account_key
            ],
            min_context_slot: None,
        }]
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
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
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

/// A node behind the first read refuses the second, and the refusal is transient. Behind a load
/// balancer this is a second node that has fallen behind, not a later state: judging the request
/// on it would report the delegation the first read just saw as absent. A retry that lands on a
/// node which has caught up authorizes the same request.
#[tokio::test]
async fn a_second_read_from_a_node_behind_the_first_is_refused_transiently() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x25, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    // The same state throughout: the only difference between the reads is which node answered.
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let reader = ScriptedReader::scripted(vec![world.clone(), world.at(99)]);
    let proofs = ScriptedProofReader::unreachable();

    let failure = authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect_err("a node behind the first read decides nothing");

    assert_eq!(
        failure,
        AuthorizationFailure::Snapshot(SnapshotError::NodeBehind)
    );
    assert!(failure.is_recoverable());
    assert_eq!(reader.call(1).min_context_slot, Some(100));
}

/// A revocation that lands between the two reads decides the request: the second read is the
/// deciding one, so the live row the first read saw is never a candidate.
#[tokio::test]
async fn a_delegation_revoked_between_the_two_reads_refuses_the_entry() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x29, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let live = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let first = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&live);
    let second = first.clone().at(101).with_delegation(&live.revoked());
    let proofs = ScriptedProofReader::constant(second.record());
    let reader = ScriptedReader::scripted(vec![first, second]);

    let failure = authorize_request(&reader, &proofs, CONTEXT, &request)
        .await
        .expect_err("the row is revoked at the deciding read");

    assert!(
        matches!(failure, AuthorizationFailure::Delegation { index: 0, .. }),
        "expected the deciding read's revoked row to refuse, got {failure}"
    );
}

/// Equal slots are not a regression: two reads of one slot are the ordinary case when the chain
/// has not advanced between the round trips, and the second is still the deciding one.
#[tokio::test]
async fn two_reads_at_the_same_slot_authorize() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let handle = handle(0x26, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(handle, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, handle, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
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

/// The state a delegated request is judged against is the second read's, not the first
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
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&allowed, handle, delegator.pubkey())
        .typed();

    let first = World::at_slot(100)
        .with_encrypted_store(&allowed)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&delegation);
    let second = World::at_slot(101)
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
}

/// A coprocessor that has sealed the observed history and holds no leaf is asked once; with no
/// other coprocessor configured, the entry is refused recoverably. Whether a later grant lands is
/// left to the attempt budget.
#[tokio::test]
async fn a_leaf_the_record_does_not_hold_is_read_once() {
    let (wallet, _, handle) = direct_scenario();
    let encrypted_store = EncryptedStoreFixture::allowing(handle, Wallet::new(9).pubkey());
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_store(&encrypted_store)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());

    let failure = authorize_request(&ScriptedReader::constant(world), &proofs, CONTEXT, &request)
        .await
        .expect_err("nobody allowed the signer");

    assert!(matches!(
        failure,
        AuthorizationFailure::HandleBinding {
            index: 0,
            source: HandleBindingFailure::NoLeaf { .. }
        }
    ));
    assert!(failure.is_recoverable());
    assert_eq!(proofs.call_count(), 1);
}

/// A coprocessor behind the chain hands the query to the next one. When every coprocessor is
/// behind, the request is rejected retryably and the ordinary attempt budget decides.
#[tokio::test]
async fn a_coprocessor_behind_the_chain_hands_the_query_to_the_next() {
    let (wallet, encrypted_store, handle) = direct_scenario();
    let request = RequestBuilder::new(&wallet)
        .direct(&encrypted_store, handle)
        .typed();
    let world = World::at_slot(100)
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

    // The first coprocessor has not yet sealed the allow leaf; the second has.
    let behind = ProofRecord::of(&[&EncryptedStoreFixture::new(handle)]);
    let catches_up = ScriptedProofReader::in_order(vec![behind.clone(), world.record()]);
    authorize_request(
        &ScriptedReader::constant(world.clone()),
        &catches_up,
        CONTEXT,
        &request,
    )
    .await
    .expect("the second coprocessor finds the leaf");
    assert_eq!(catches_up.call_count(), 2, "each coprocessor is asked once");

    // Every coprocessor behind: each asked once, then a retryable rejection.
    let stays_behind = ScriptedProofReader::in_order(vec![behind.clone(), behind]);
    let failure = authorize_request(
        &ScriptedReader::constant(world),
        &stays_behind,
        CONTEXT,
        &request,
    )
    .await
    .expect_err("no coprocessor has sealed the leaf");
    assert_eq!(
        stays_behind.call_count(),
        2,
        "no coprocessor is asked twice"
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

/// A node answering `getMultipleAccounts` with `result` for exactly this body.
async fn node_answering(
    keys: &[Pubkey],
    min_context_slot: Option<u64>,
    answer: serde_json::Value,
) -> (mocktail::server::MockServer, SolanaRpcClient) {
    let expected = multiple_accounts_request(keys, min_context_slot);
    let mut server = mocktail::server::MockServer::new_http("solana-accounts");
    server.mock(move |when, then| {
        when.post().json(expected.clone());
        then.json(answer.clone());
    });
    server.start().await.unwrap();
    let client = SolanaRpcClient::new(
        server.base_url().unwrap().clone(),
        std::time::Duration::from_secs(1),
        std::num::NonZeroUsize::new(1).unwrap(),
    );
    (server, client)
}

/// Exercise the actual SDK transport; the mock matches commitment, encoding, `minContextSlot` and
/// ordered keys.
async fn rpc_read(
    keys: &[Pubkey],
    value: serde_json::Value,
) -> Result<AccountsRead, SnapshotError> {
    let answer = serde_json::json!({"jsonrpc":"2.0","id":0,"result":{"context":{"slot":4242},"value":value}});
    let (_server, client) = node_answering(keys, None, answer).await;
    read_positional(&client, keys, None).await
}

fn rpc_account() -> serde_json::Value {
    serde_json::json!({"owner":PROGRAM_ID.to_string(),
        "data":["AQID","base64"],"lamports":1,"executable":false,"rentEpoch":0})
}

#[tokio::test]
async fn confirmed_rpc_preserves_order_null_accounts_and_context_slot() {
    let read = rpc_read(
        &[
            Pubkey::new_from_array([5; 32]),
            Pubkey::new_from_array([6; 32]),
        ],
        serde_json::json!([rpc_account(), null]),
    )
    .await
    .unwrap();
    assert_eq!(
        read,
        AccountsRead {
            slot: 4242,
            accounts: vec![
                Some(SnapshotAccount {
                    owner: PROGRAM_ID,
                    data: vec![1, 2, 3]
                }),
                None
            ],
        }
    );
}

/// The second read of a delegated request carries the first read's slot as `minContextSlot`, and
/// a node that has not reached it answers JSON-RPC -32016, which is a node behind, not a failed
/// read.
#[tokio::test]
async fn a_node_below_the_minimum_context_slot_is_reported_as_behind() {
    let keys = [Pubkey::new_from_array([5; 32])];
    let behind = serde_json::json!({"jsonrpc":"2.0","id":0,"error":{
        "code":-32016,"message":"Minimum context slot has not been reached","data":{"contextSlot":99}}});
    let (_server, client) = node_answering(&keys, Some(100), behind).await;
    assert_eq!(
        client.read_accounts(&keys, Some(100)).await,
        Err(SnapshotError::NodeBehind)
    );

    let other = serde_json::json!({"jsonrpc":"2.0","id":0,"error":{"code":-32005,"message":"Node is unhealthy"}});
    let (_server, client) = node_answering(&keys, Some(100), other).await;
    assert!(matches!(
        client.read_accounts(&keys, Some(100)).await,
        Err(SnapshotError::Unavailable { reason }) if reason.contains("Node is unhealthy")
    ));
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
    for account in [bad_base64, wrong_encoding, bad_owner, missing_owner] {
        assert!(matches!(
            rpc_read(
                &[Pubkey::new_from_array([5; 32])],
                serde_json::json!([account])
            )
            .await,
            Err(SnapshotError::Unavailable { .. })
        ));
    }
}

/// A node that answers with fewer or more accounts than it was asked for has lost the pairing of
/// accounts to keys.
#[rstest::rstest]
#[case::short(serde_json::json!([]), 0)]
#[case::oversized(serde_json::json!([null, null]), 2)]
#[tokio::test]
async fn an_rpc_answer_of_the_wrong_length_is_an_error(
    #[case] value: serde_json::Value,
    #[case] returned: usize,
) {
    assert_eq!(
        rpc_read(&[Pubkey::new_from_array([5; 32])], value).await,
        Err(SnapshotError::ResponseLengthMismatch {
            requested: 1,
            returned,
        })
    );
}

#[tokio::test]
async fn a_full_hundred_account_snapshot_is_one_rpc_call() {
    let keys: Vec<_> = (0..100).map(|i| Pubkey::new_from_array([i; 32])).collect();
    let read = rpc_read(&keys, serde_json::json!(vec![None::<()>; 100]))
        .await
        .unwrap();
    assert_eq!(read.accounts, vec![None; 100]);
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
        client.read_accounts(&[Pubkey::new_from_array([1; 32])], None),
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
            .read_accounts(&[Pubkey::new_from_array([1; 32])], None)
            .await
    });
    let (_held, _) = timeout(Duration::from_secs(1), listener.accept())
        .await
        .unwrap()
        .unwrap();
    let second = tokio::spawn(async move {
        client
            .read_accounts(&[Pubkey::new_from_array([2; 32])], None)
            .await
    });
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
    assert_eq!(second.await.unwrap().unwrap().slot, 42);
}
