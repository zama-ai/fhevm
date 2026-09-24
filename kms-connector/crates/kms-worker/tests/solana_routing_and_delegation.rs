//! Routing and delegation: which identity an entry authorizes as, and what a delegation record has
//! to say for a delegated entry to stand.
//!
//! Routing is per entry and decided by one comparison: the entry's owner address against the
//! permit's signer. Equal means the signer's own allow leaf is what is proven; unequal means the
//! owner address is a delegator whose leaf is proven and who, additionally, has delegated to the
//! signer in the encrypted store's application `(program, scope)`. One request mixes both freely, and
//! different delegators freely — there is no delegated mode and no delegated route.
//!
//! The negative test that matters most is the substitution of the key. In the delegated branch it
//! would be easy, and wrong, to prove the signer's leaf: the signer is the one asking, and the
//! delegation record does name them. But the handle was allowed to the delegator, and it is the
//! delegator's allow leaf that the delegation extends. Proving the signer's own leaf instead would
//! authorize a delegate against handles the delegator was never allowed on, wherever the delegate
//! happens to hold a leaf. Two tests below pin the direction from both sides: the delegator's leaf
//! authorizes, and the signer's own leaf does not.
//!
//! Expiry is evaluated against the Clock of the deciding read, and the record's `delegation_counter`
//! takes no part in it. That absence is deliberate: pinning the counter in the request would let any
//! unrelated update to any delegation record invalidate requests already in flight, and would make
//! a mixed-delegator batch impossible to build.
//!
//! Two rows can carry one grant — the encrypted store's application, and the delegator's
//! wildcard row — and the last section pins that rule from both sides: either row alone authorizes,
//! neither vetoes the other, and revoking one leaves the other standing. That last property is the
//! price of wildcard scope and is asserted deliberately, not tolerated.
use connector_utils::types::solana_request::SolanaUserDecryptionRequestV1;

mod solana_support;

use kms_worker::core::solana::{
    SolanaPubkeyBytes,
    delegation::{AuthorizedRow, DelegationFailure, check_delegation},
    encrypted_store::EncryptedStoreFailure,
    failure::AuthorizationFailure,
    handle_binding::HandleBindingFailure,
    pipeline::authorize_request,
    snapshot::{SnapshotAccount, SnapshotError},
};
use solana_support::*;
use zama_solana_acl::WILDCARD_APP;

const OBSERVED_SLOT: u64 = 500;

/// Authorizes a request against a world, returning the outcome and how many reads it cost.
async fn authorize_in(
    world: World,
    request: &SolanaUserDecryptionRequestV1,
) -> (Result<(), AuthorizationFailure>, usize) {
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let context = CONTEXT;
    let outcome = authorize_request(&reader, &proofs, context, request).await;
    (outcome, reader.call_count())
}

/// A world holding an encrypted store, the signer's zero watermark, and whatever else is
/// added.
fn world_with(encrypted_store: &EncryptedStoreFixture, signer: SolanaPubkeyBytes) -> World {
    World::at_slot(OBSERVED_SLOT)
        .with_encrypted_store(encrypted_store)
        .with_watermark(signer, 0)
}

// ---------------------------------------------------------------------------
// Routing
// ---------------------------------------------------------------------------

/// An entry the signer owns authorizes as the signer.
#[tokio::test]
async fn a_direct_entry_authorizes_as_the_signer() {
    let signer = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .direct(&encrypted_store, live)
        .typed();

    let (outcome, reads) =
        authorize_in(world_with(&encrypted_store, signer.pubkey()), &request).await;

    outcome.expect("the signer holds the allow leaf on the handle");
    assert_eq!(reads, 1, "a direct entry needs no delegation record");
}

/// An entry allowed to somebody else authorizes as *that* key — the delegator — and the delegator's
/// allow leaf is what has to be proven. The account here holds only the delegator's leaf, so an
/// implementation proving the signer's leaf instead would fail this test.
#[tokio::test]
async fn a_delegated_entry_authorizes_as_the_delegator() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x11, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, reads) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&delegation),
        &request,
    )
    .await;

    outcome.expect("a live delegation lets the signer use the delegator's access");
    assert_eq!(reads, 2);
}

/// The mirror image, and the substitution this whole branch has to refuse: the signer holds an
/// allow leaf on the handle in their own right, the delegator does not, and a live delegation
/// exists between them. Nothing about that combination gives the signer access *as the delegator*
/// to a handle the delegator was never allowed on.
///
/// An implementation that proved the signer's leaf would accept this, and would thereby let any
/// delegate reach any handle they happen to hold a leaf on while attributing it to somebody else.
#[tokio::test]
async fn a_delegated_entry_is_not_authorized_by_the_delegates_own_leaf() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x12, FHE_TYPE_UINT64);
    // The signer holds the leaf; the delegator does not.
    let encrypted_store = EncryptedStoreFixture::allowing(live, signer.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&delegation),
        &request,
    )
    .await;

    let failure =
        outcome.expect_err("a delegation extends the delegator's access, not the delegate's");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::NoLeaf { .. }
            }
        ),
        "the leaf that is missing is the delegator's, got {failure}"
    );
}

/// One request mixes a direct entry with entries from two different delegators. There is no
/// per-request mode to pick, so nothing about the batch has to be uniform.
#[tokio::test]
async fn a_batch_mixes_a_direct_entry_and_two_delegators() {
    let signer = Wallet::new(1);
    let first_delegator = Wallet::new(2);
    let second_delegator = Wallet::new(3);
    let own = handle(0x20, FHE_TYPE_UINT64);
    let first = handle(0x21, FHE_TYPE_UINT64);
    let second = handle(0x22, FHE_TYPE_UINT64);
    let own_encrypted_store = EncryptedStoreFixture::allowing(own, signer.pubkey());
    let first_authority = [0xa1; 32];
    let mut first_encrypted_store =
        EncryptedStoreFixture::in_application(APP_PROGRAM, first_authority, SCOPE, LABEL, first);
    first_encrypted_store.allow(first_delegator.pubkey());
    let second_authority = [0xb1; 32];
    let mut second_encrypted_store =
        EncryptedStoreFixture::in_application(APP_PROGRAM, second_authority, SCOPE, LABEL, second);
    second_encrypted_store.allow(second_delegator.pubkey());
    let first_delegation = DelegationFixture::live(first_delegator.pubkey(), signer.pubkey())
        .in_application_of(&first_encrypted_store);
    let second_delegation = DelegationFixture::live(second_delegator.pubkey(), signer.pubkey())
        .in_application_of(&second_encrypted_store);

    let request = RequestBuilder::new(&signer)
        .direct(&own_encrypted_store, own)
        .delegated(&first_encrypted_store, first, first_delegator.pubkey())
        .delegated(&second_encrypted_store, second, second_delegator.pubkey())
        .typed();
    let world = World::at_slot(OBSERVED_SLOT)
        .with_encrypted_store(&own_encrypted_store)
        .with_encrypted_store(&first_encrypted_store)
        .with_encrypted_store(&second_encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&first_delegation)
        .with_delegation(&second_delegation);

    let (outcome, reads) = authorize_in(world, &request).await;

    outcome.expect("a mixed batch is an ordinary request");
    assert_eq!(
        reads, 2,
        "two delegators still cost one extra read, not two"
    );
}

// ---------------------------------------------------------------------------
// Freshness
// ---------------------------------------------------------------------------

/// A delegated scenario, parameterised by what the delegation record says.
async fn authorize_delegated(delegation: DelegationFixture) -> Result<(), AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let world = world_with(&encrypted_store, signer.pubkey()).with_delegation(&delegation);
    authorize_in(world, &request).await.0
}

/// The delegation of the scenario above, live at the host's Clock (`HOST_NOW`).
fn live_delegation() -> DelegationFixture {
    DelegationFixture::live(Wallet::new(2).pubkey(), Wallet::new(1).pubkey())
}

/// A revoked exact delegation denies this attempt; the missing wildcard may become visible later.
/// Revocation zeroes the expiry, so the row reads like one never granted, as on EVM.
#[tokio::test]
async fn a_revoked_delegation_rejects_its_entry() {
    let failure = authorize_delegated(live_delegation().revoked())
        .await
        .expect_err("a revoked delegation authorizes nothing");

    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::NotLive { expires_at: 0, now: HOST_NOW })
            && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// Expiry is measured against the Clock of the deciding read, not against the Connector's clock:
/// this world's Clock is past the expiry while the Connector's is not.
#[tokio::test]
async fn a_delegation_expired_at_the_hosts_clock_rejects_its_entry() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    assert!(delegation.expires_at > CONTEXT.now_unix_seconds);
    let world = world_with(&encrypted_store, signer.pubkey())
        .with_delegation(&delegation)
        .with_clock(delegation.expires_at + 1);

    let failure = authorize_in(world, &request)
        .await
        .0
        .expect_err("an expired delegation authorizes nothing");

    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::NotLive { .. })
            && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// The expiry is exclusive, as EVM's `expirationDate > block.timestamp`: a delegation ending at the
/// Clock's second is dead, and one ending a second later is live.
#[tokio::test]
async fn a_delegation_is_live_until_its_expiry_second() {
    let mut boundary = live_delegation();
    boundary.expires_at = HOST_NOW;
    let failure = authorize_delegated(boundary)
        .await
        .expect_err("the expiry second itself is outside the delegation's life");
    assert!(failure.is_recoverable());

    boundary.expires_at = HOST_NOW + 1;
    authorize_delegated(boundary)
        .await
        .expect("a delegation ending a second after the Clock is live");
}

/// A deciding read without a decodable Clock comes from a bad node, so the attempt is retried.
#[tokio::test]
async fn a_deciding_read_without_the_clock_is_retried() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let world = world_with(&encrypted_store, signer.pubkey())
        .with_delegation(&DelegationFixture::live(
            delegator.pubkey(),
            signer.pubkey(),
        ))
        .without_account(&zama_solana_acl::CLOCK_SYSVAR_ID);

    let failure = authorize_in(world, &request)
        .await
        .0
        .expect_err("no Clock, no expiry check");

    assert!(failure.is_recoverable());
    assert!(matches!(
        failure,
        AuthorizationFailure::Snapshot(SnapshotError::MalformedClock)
    ));
}

// ---------------------------------------------------------------------------
// Wildcard scope
// ---------------------------------------------------------------------------

/// The same delegated scenario, parameterised by which rows the world holds.
async fn authorize_delegated_with_rows(
    rows: &[DelegationFixture],
) -> Result<(), AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let mut world = world_with(&encrypted_store, signer.pubkey());
    for row in rows {
        world = world.with_delegation(row);
    }
    authorize_in(world, &request).await.0
}

/// The wildcard row of the same pair, live at the host's Clock (`HOST_NOW`).
fn live_wildcard() -> DelegationFixture {
    DelegationFixture::live_wildcard(Wallet::new(2).pubkey(), Wallet::new(1).pubkey())
}

/// A wildcard row covers an encrypted store that has no row of its own — the EVM ACL's
/// wildcard delegation, with `0xff×32` standing in both the program and the scope position.
#[tokio::test]
async fn a_wildcard_row_authorizes_a_encrypted_store_with_no_app_specific_row() {
    authorize_delegated_with_rows(&[live_wildcard()])
        .await
        .expect("a wildcard row covers every app of its delegator");
}

/// The application row is tried first, so a dead wildcard row beside it changes nothing.
#[tokio::test]
async fn an_app_specific_row_authorizes_while_the_wildcard_row_is_dead() {
    let dead_wildcard = live_wildcard().revoked();

    authorize_delegated_with_rows(&[live_delegation(), dead_wildcard])
        .await
        .expect("one live row is the whole requirement");
}

/// The consequence of the rule, stated as a test rather than left to be discovered: revoking the
/// application row does not stop a delegate who also holds a wildcard row. Scope-by-app is a
/// property of a row, so narrowing one app takes revoking both rows — two transactions, because the
/// host program's revocation instruction takes one record per call.
#[tokio::test]
async fn revoking_the_app_specific_row_does_not_stop_a_wildcard_delegate() {
    let revoked = live_delegation().revoked();

    authorize_delegated_with_rows(&[revoked, live_wildcard()])
        .await
        .expect("the wildcard row still authorizes, which is what wildcard means");
}

/// When both rows exist and neither is live, both reasons are reported. Naming one would send the
/// delegate to fix a row that was not the only thing standing in the way.
#[tokio::test]
async fn two_dead_rows_report_both_reasons() {
    let revoked = live_delegation().revoked();
    let mut expired_wildcard = live_wildcard();
    expired_wildcard.expires_at = HOST_NOW;

    let failure = authorize_delegated_with_rows(&[revoked, expired_wildcard])
        .await
        .expect_err("two dead rows authorize nothing");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveDelegation { exact, wildcard },
        } => {
            assert!(matches!(
                **exact,
                DelegationFailure::NotLive { expires_at: 0, .. }
            ));
            assert!(matches!(
                **wildcard,
                DelegationFailure::NotLive {
                    expires_at: HOST_NOW,
                    ..
                }
            ));
        }
        other => panic!("expected both reasons, got {other}"),
    }
    // As on EVM, a delegation that is not live is an ACL denial a later attempt may clear.
    assert!(failure.is_recoverable());
}

/// A revoked exact row cannot settle a wildcard grant that may not be visible yet.
#[tokio::test]
async fn a_missing_wildcard_preserves_the_exact_reason_and_retryability() {
    let revoked = live_delegation().revoked();

    let failure = authorize_delegated_with_rows(&[revoked])
        .await
        .expect_err("a revoked row authorizes nothing");

    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::NotLive { expires_at: 0, .. }) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// A wildcard row is per delegator: somebody else's covers nothing here, because its address is
/// derived from the delegator this entry names.
#[tokio::test]
async fn a_wildcard_row_of_another_delegator_authorizes_nothing() {
    let stranger =
        DelegationFixture::live_wildcard(Wallet::new(9).pubkey(), Wallet::new(1).pubkey());

    let failure = authorize_delegated_with_rows(&[stranger])
        .await
        .expect_err("a wildcard row of another delegator is not at this entry's address");

    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::Absent { .. })
            && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// The delegated scenario of this section, with an arbitrary account planted at the delegator's
/// wildcard address and no application row beside it.
async fn authorize_with_wildcard_account(
    account: SnapshotAccount,
) -> Result<(), AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x39, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let (wildcard_key, _) = live_wildcard().address();
    let world = world_with(&encrypted_store, signer.pubkey()).with_account(wildcard_key, account);
    authorize_in(world, &request).await.0
}

/// An account under another program's ownership at the wildcard address is reported as what it
/// is, beside the absent application row — not swallowed as "no wildcard row".
#[tokio::test]
async fn an_impostor_at_the_wildcard_address_is_named_beside_the_absent_exact_row() {
    let mut impostor = live_wildcard().account();
    impostor.owner = [0xee; 32];

    let failure = authorize_with_wildcard_account(impostor)
        .await
        .expect_err("an account of another program is not a wildcard row");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveDelegation { exact, wildcard },
        } => {
            assert!(matches!(**exact, DelegationFailure::Absent { .. }));
            assert!(matches!(**wildcard, DelegationFailure::ForeignOwner { .. }));
        }
        other => panic!("expected both reasons, got {other}"),
    }
}

/// A wildcard row storing a bump other than the canonical one for its address is not the record
/// this reader reads — the same rule the application row is already held to.
#[tokio::test]
async fn a_wildcard_row_storing_a_non_canonical_bump_is_not_a_wildcard_row() {
    let (_, canonical_bump) = live_wildcard().address();
    let mut wrong_bump = live_wildcard().account();
    let last = wrong_bump.data.len() - 1;
    assert_eq!(
        wrong_bump.data[last], canonical_bump,
        "the fixture writes the canonical bump, or this test proves nothing"
    );
    wrong_bump.data[last] = canonical_bump.wrapping_sub(1);

    let failure = authorize_with_wildcard_account(wrong_bump)
        .await
        .expect_err("a non-canonical bump is not this record");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveDelegation { exact, wildcard },
        } => {
            assert!(matches!(**exact, DelegationFailure::Absent { .. }));
            assert!(matches!(
                **wildcard,
                DelegationFailure::NotADelegationRecord { .. }
            ));
        }
        other => panic!("expected both reasons, got {other}"),
    }
}

/// A record naming another delegator, planted at *this* pair's wildcard address: it decodes, and
/// its own fields betray it. The address alone is not taken as proof for the wildcard row either.
#[tokio::test]
async fn a_wildcard_row_naming_another_tuple_is_rejected() {
    let stranger_row =
        DelegationFixture::live_wildcard(Wallet::new(9).pubkey(), Wallet::new(1).pubkey());

    let failure = authorize_with_wildcard_account(stranger_row.account())
        .await
        .expect_err("a wildcard row must name the tuple it was read for");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveDelegation { exact, wildcard },
        } => {
            assert!(matches!(**exact, DelegationFailure::Absent { .. }));
            assert!(matches!(
                **wildcard,
                DelegationFailure::TupleMismatch { .. }
            ));
        }
        other => panic!("expected both reasons, got {other}"),
    }
}

/// The counter is decoded and ignored. Two records that differ only in it behave identically —
/// which is what makes a permit reusable across unrelated delegation updates.
#[tokio::test]
async fn the_counter_does_not_affect_the_outcome() {
    let mut fresh_grant = live_delegation();
    fresh_grant.delegation_counter = 1;
    let mut regranted_many_times = live_delegation();
    regranted_many_times.delegation_counter = 4_294_967_296;

    authorize_delegated(fresh_grant)
        .await
        .expect("a first grant authorizes");
    authorize_delegated(regranted_many_times)
        .await
        .expect("a record regranted any number of times authorizes identically");
}

/// No record, no delegated access. The rejection names the canonical address that was read, so
/// the diagnosis does not require guessing which tuple was derived.
#[tokio::test]
async fn a_missing_delegation_rejects_its_entry() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (expected_key, _) = expected.address();

    let (outcome, _) = authorize_in(world_with(&encrypted_store, signer.pubkey()), &request).await;

    let failure = outcome.expect_err("an absent delegation authorizes nothing");
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::Absent { account_key } if account_key == expected_key)
            && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
    // Transient, and pinned as such: this reader and the relayer read through their own RPCs and
    // can sit at different confirmed slots, so "not here" can mean "not here yet". Terminal would
    // fail a valid delegated request permanently over ordinary replica lag.
    assert!(failure.is_recoverable());
}

/// A revoked, expired or mismatched exact row beside an absent wildcard row leaves the request
/// retryable: the wildcard row may not be visible to this node yet.
#[tokio::test]
async fn a_dead_exact_row_with_missing_wildcard_stays_retryable() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let stranger = Wallet::new(9);
    let live = handle(0x3f, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (expected_key, _) = expected.address();

    let revoked = DelegationFixture::live(delegator.pubkey(), signer.pubkey()).revoked();
    let mut expired = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    expired.expires_at = HOST_NOW;
    // Somebody else's tuple, planted at the address this request reads: changing a tuple field
    // moves the record's own address, so a mismatch only exists when the record is placed by
    // address rather than derived from itself.
    let mismatched = DelegationFixture::live(stranger.pubkey(), signer.pubkey());

    for (what, delegation) in [
        ("revoked", revoked),
        ("expired", expired),
        ("tuple mismatch", mismatched),
    ] {
        let (outcome, _) = authorize_in(
            world_with(&encrypted_store, signer.pubkey())
                .with_account(expected_key, delegation.account()),
            &request,
        )
        .await;
        let failure = outcome.expect_err("none of these authorize");
        assert!(
            failure.is_recoverable(),
            "a {what} exact row does not settle the missing wildcard"
        );
    }
}

/// A delegation is scoped to an application, and the application is the encrypted store's. A
/// delegation for another application is simply not the record that gets read — the address
/// derived from the encrypted store's application is empty.
#[tokio::test]
async fn a_delegation_for_another_application_does_not_authorize() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x32, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let mut elsewhere = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    elsewhere.scope = [0x77; 32];
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&elsewhere),
        &request,
    )
    .await;

    let failure =
        outcome.expect_err("a delegation for another application is not this application's");
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::Absent { .. })
            && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// A record sitting at the canonical address while naming a different tuple is rejected. The
/// address alone is not taken as proof of what the record says.
#[tokio::test]
async fn a_delegation_record_naming_another_tuple_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let stranger = Wallet::new(9);
    let live = handle(0x33, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (expected_key, _) = expected.address();
    // A record for a different delegator, planted at the address the request will read.
    let foreign = DelegationFixture::live(stranger.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_account(expected_key, foreign.account()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a record must name the tuple it was read for");
    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::TupleMismatch { .. }) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// The delegate half of the same rule: a record at the canonical address naming a different
/// delegate is refused too. Sitting at the address derived from the delegate is not the same as
/// naming them.
#[tokio::test]
async fn a_delegation_record_naming_another_delegate_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let stranger = Wallet::new(9);
    let live = handle(0x38, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (expected_key, _) = expected.address();
    // A record delegating to somebody else, planted at the address the request will read.
    let other_delegate = DelegationFixture::live(delegator.pubkey(), stranger.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey())
            .with_account(expected_key, other_delegate.account()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a record delegating to somebody else authorizes nothing");
    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::TupleMismatch { account_key } if account_key == expected_key) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// A record storing a bump other than the canonical one for its address is not the record this
/// reader reads. Nothing an attacker can arrange — only the host program writes program-owned bytes,
/// and the address was derived here — but a record written under another derivation is caught where
/// it is one comparison instead of surfacing later as a rule that stopped matching.
#[tokio::test]
async fn a_delegation_record_storing_a_non_canonical_bump_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x34, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (key, canonical_bump) = delegation.address();
    let mut wrong_bump = delegation.account();
    let last = wrong_bump.data.len() - 1;
    assert_eq!(
        wrong_bump.data[last], canonical_bump,
        "the fixture writes the canonical bump, or this test proves nothing"
    );
    wrong_bump.data[last] = canonical_bump.wrapping_sub(1);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_account(key, wrong_bump),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a non-canonical bump is not this record");
    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::NotADelegationRecord { .. }) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// An account under another program's ownership is not a delegation, whatever it contains.
#[tokio::test]
async fn a_delegation_record_owned_by_another_program_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x34, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (key, _) = delegation.address();
    let mut impostor = delegation.account();
    impostor.owner = [0xee; 32];
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_account(key, impostor),
        &request,
    )
    .await;

    let failure = outcome.expect_err("only the host program grants delegations");
    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::ForeignOwner { .. }) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// A host-owned account at the delegation address that does not decode as a delegation record
/// grants nothing.
#[tokio::test]
async fn a_host_account_that_is_not_a_delegation_record_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x35, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let (key, _) = delegation.address();
    let mut undecodable = delegation.account();
    undecodable.data.truncate(8);
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_account(key, undecodable),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a record that does not decode delegates nothing");
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { exact, .. } }
        if matches!(**exact, DelegationFailure::NotADelegationRecord { .. })),
        "{failure}"
    );
}

/// The same request across a revocation: the first authorization succeeds and the next fails.
/// Each attempt observes the delegation again, including polls of already-sent requests.
#[tokio::test]
async fn the_same_permit_stops_working_once_the_delegation_is_revoked() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x35, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    let granted = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let revoked = granted.revoked();

    let (first, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&granted),
        &request,
    )
    .await;
    first.expect("the first request is authorized");

    let (second, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&revoked),
        &request,
    )
    .await;
    let failure = second.expect_err("the second request is no longer authorized");
    assert!(failure.is_recoverable());
    assert!(
        matches!(failure, AuthorizationFailure::Delegation { source: DelegationFailure::NoLiveDelegation { exact, wildcard }, .. }
        if matches!(*exact, DelegationFailure::NotLive { expires_at: 0, .. }) && matches!(*wildcard, DelegationFailure::Absent { .. }))
    );
}

// ---------------------------------------------------------------------------
// The authorizing row
// ---------------------------------------------------------------------------

/// The default encrypted store, whose application the rows below are in.
fn store() -> zama_solana_acl::EncryptedStore {
    EncryptedStoreFixture::new(handle(0x3a, FHE_TYPE_UINT64)).encrypted_store
}

/// `check_delegation` names the row that authorized. The distinction is invisible in the request's
/// outcome — either row authorizes identically — but an audit record of a delegated authorization
/// has to tell an application-scoped grant from a wildcard one, and only this function knows which
/// row stood behind the entry.
#[test]
fn a_live_application_row_is_named_as_the_exact_row() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let exact = DelegationFixture::live(delegator, delegate);
    let rows = World::at_slot(OBSERVED_SLOT).with_delegation(&exact).rows(
        exact.address(),
        DelegationFixture::live_wildcard(delegator, delegate).address(),
    );

    let row = check_delegation(&rows, PROGRAM_ID, delegator, delegate, &store())
        .expect("a live application row authorizes");

    assert_eq!(row, AuthorizedRow::Exact);
}

/// The same grant carried by the wildcard row alone is named as such: the row that authorized is
/// reported, not merely the fact that one did.
#[test]
fn a_live_wildcard_row_is_named_as_the_wildcard_row() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let wildcard = DelegationFixture::live_wildcard(delegator, delegate);
    let exact = DelegationFixture::live(delegator, delegate);
    let rows = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&wildcard)
        .rows(exact.address(), wildcard.address());

    let row = check_delegation(&rows, PROGRAM_ID, delegator, delegate, &store())
        .expect("a live wildcard row authorizes an application with no row of its own");

    assert_eq!(row, AuthorizedRow::Wildcard);
}

/// With BOTH rows live, the application row is the one named. The request outcome is
/// identical either way, so nothing but this assertion notices a reordering of the two checks —
/// which would silently relabel every such authorization in the audit record as wildcard-carried.
#[test]
fn with_both_rows_live_the_application_row_is_the_one_named() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let exact = DelegationFixture::live(delegator, delegate);
    let wildcard = DelegationFixture::live_wildcard(delegator, delegate);
    let rows = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&exact)
        .with_delegation(&wildcard)
        .rows(exact.address(), wildcard.address());

    let row = check_delegation(&rows, PROGRAM_ID, delegator, delegate, &store())
        .expect("two live rows authorize");

    assert_eq!(row, AuthorizedRow::Exact);
}

// ---------------------------------------------------------------------------
// Sentinel injection
// ---------------------------------------------------------------------------

/// An encrypted store naming the wildcard sentinel as its program is rejected at resolution,
/// before any delegation row is read. With the sentinel as program, the application row's address
/// lands on the wildcard row itself. On-chain the store's authority must sign as a PDA of its
/// program, and no program exists at the sentinel, so no legal encrypted store carries it; one
/// that does is rejected, not interpreted.
///
/// The world here holds a live wildcard row — exactly the row a sentinel program resolves to —
/// so an implementation without the guard authorizes this request.
#[tokio::test]
async fn a_sentinel_program_in_the_encrypted_store_rejects_a_delegated_entry() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x36, FHE_TYPE_UINT64);
    let mut encrypted_store =
        EncryptedStoreFixture::in_application(WILDCARD_APP, AUTHORITY, WILDCARD_APP, LABEL, live);
    encrypted_store.allow(delegator.pubkey());
    let wildcard = DelegationFixture::live_wildcard(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_store, signer.pubkey()).with_delegation(&wildcard),
        &request,
    )
    .await;

    let failure =
        outcome.expect_err("a sentinel program must be rejected, not resolved to the wildcard row");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::EncryptedStore {
                index: 0,
                source: EncryptedStoreFailure::SentinelProgram { .. }
            }
        ),
        "the rejection belongs to the encrypted store resolution, got {failure}"
    );
    assert!(!failure.is_recoverable());
}

/// The guard lives in the resolution of the encrypted store, so a direct entry under a sentinel
/// program is rejected the same way. Deliberate: such an account is illegitimate whether or not a
/// delegation is in play, and one rule at the chokepoint beats a rule that only the delegated
/// branch remembers to apply.
#[tokio::test]
async fn a_sentinel_program_in_the_encrypted_store_rejects_a_direct_entry_too() {
    let signer = Wallet::new(1);
    let live = handle(0x37, FHE_TYPE_UINT64);
    let mut encrypted_store =
        EncryptedStoreFixture::in_application(WILDCARD_APP, AUTHORITY, SCOPE, LABEL, live);
    encrypted_store.allow(signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .direct(&encrypted_store, live)
        .typed();

    let (outcome, _) = authorize_in(world_with(&encrypted_store, signer.pubkey()), &request).await;

    let failure = outcome.expect_err("the sentinel is not a program any account may name");
    assert!(matches!(
        failure,
        AuthorizationFailure::EncryptedStore {
            index: 0,
            source: EncryptedStoreFailure::SentinelProgram { .. }
        }
    ));
    assert!(!failure.is_recoverable());
}

// ---------------------------------------------------------------------------
// What the delegated branch reads, and what it refuses to read
// ---------------------------------------------------------------------------

/// In a batch where delegated entries have different outcomes, the failure names the index of the
/// entry whose delegation is dead — in request coordinates, so the client can point at the
/// offending entry without re-deriving which entries were delegated.
#[tokio::test]
async fn a_mixed_batch_failure_names_the_entry_whose_delegation_is_dead() {
    let signer = Wallet::new(1);
    let first_delegator = Wallet::new(2);
    let second_delegator = Wallet::new(3);
    let own = handle(0x41, FHE_TYPE_UINT64);
    let first = handle(0x42, FHE_TYPE_UINT64);
    let second = handle(0x43, FHE_TYPE_UINT64);
    let own_encrypted_store = EncryptedStoreFixture::allowing(own, signer.pubkey());
    let first_authority = [0xa2; 32];
    let mut first_encrypted_store =
        EncryptedStoreFixture::in_application(APP_PROGRAM, first_authority, SCOPE, LABEL, first);
    first_encrypted_store.allow(first_delegator.pubkey());
    let second_authority = [0xb2; 32];
    let mut second_encrypted_store =
        EncryptedStoreFixture::in_application(APP_PROGRAM, second_authority, SCOPE, LABEL, second);
    second_encrypted_store.allow(second_delegator.pubkey());
    let first_delegation = DelegationFixture::live(first_delegator.pubkey(), signer.pubkey())
        .in_application_of(&first_encrypted_store);
    let second_delegation = DelegationFixture::live(second_delegator.pubkey(), signer.pubkey())
        .in_application_of(&second_encrypted_store)
        .revoked();

    let request = RequestBuilder::new(&signer)
        .direct(&own_encrypted_store, own)
        .delegated(&first_encrypted_store, first, first_delegator.pubkey())
        .delegated(&second_encrypted_store, second, second_delegator.pubkey())
        .typed();
    let world = World::at_slot(OBSERVED_SLOT)
        .with_encrypted_store(&own_encrypted_store)
        .with_encrypted_store(&first_encrypted_store)
        .with_encrypted_store(&second_encrypted_store)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&first_delegation)
        .with_delegation(&second_delegation);

    let (outcome, _) = authorize_in(world, &request).await;

    let failure = outcome.expect_err("one dead delegation rejects the request");
    assert!(failure.is_recoverable());
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 2,
        source: DelegationFailure::NoLiveDelegation { exact, wildcard } }
        if matches!(**exact, DelegationFailure::NotLive { expires_at: 0, .. }) && matches!(**wildcard, DelegationFailure::Absent { .. })),
        "{failure}"
    );
}

/// The delegator's own permit watermark is not read. `revoke_permits` is the delegate-side lever
/// — it invalidates permits the delegator signed as a *requester* — and the delegator's lever
/// over delegated access is delegation revocation. A delegator who has revoked all their own
/// permits has said nothing about their delegations.
#[tokio::test]
async fn the_delegators_permit_watermark_is_not_read() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x44, FHE_TYPE_UINT64);
    let encrypted_store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&encrypted_store, live, delegator.pubkey())
        .typed();
    // A watermark that would invalidate any permit — were it ever read for this request.
    let world = world_with(&encrypted_store, signer.pubkey())
        .with_delegation(&delegation)
        .with_watermark(delegator.pubkey(), u64::MAX);

    let (outcome, reads) = authorize_in(world, &request).await;

    outcome.expect("the delegator's permit watermark plays no part in a delegated request");
    assert_eq!(reads, 2, "no extra read fetches the delegator's watermark");
}

#[tokio::test]
async fn prefunded_delegations_remain_absent_until_initialized() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x3f, FHE_TYPE_UINT64);
    let store = EncryptedStoreFixture::allowing(live, delegator.pubkey());
    let request = RequestBuilder::new(&signer)
        .delegated(&store, live, delegator.pubkey())
        .typed();
    let exact = live_delegation().revoked();
    let wildcard = live_wildcard();
    let empty = SnapshotAccount {
        owner: [0; 32],
        data: vec![],
    };
    let base = world_with(&store, signer.pubkey());
    for exact_account in [exact.account(), empty.clone()] {
        let world = base
            .clone()
            .with_account(exact.address().0, exact_account)
            .with_account(wildcard.address().0, empty.clone());
        let failure = authorize_in(world, &request).await.0.unwrap_err();
        assert!(
            failure.is_recoverable(),
            "prefunding must not make a missing grant terminal: {failure}"
        );
    }
    let invalid = base.clone().with_delegation(&exact).with_account(
        wildcard.address().0,
        SnapshotAccount {
            owner: [0; 32],
            data: vec![1],
        },
    );
    let failure = authorize_in(invalid, &request).await.0.unwrap_err();
    assert!(
        matches!(&failure, AuthorizationFailure::Delegation { index: 0,
        source: DelegationFailure::NoLiveDelegation { wildcard, .. } }
        if matches!(**wildcard, DelegationFailure::ForeignOwner { .. })),
        "a nonempty foreign account is not read as absent: {failure}"
    );
    authorize_in(
        base.with_delegation(&exact).with_delegation(&wildcard),
        &request,
    )
    .await
    .0
    .expect("a later initialized grant authorizes the same request");
}
