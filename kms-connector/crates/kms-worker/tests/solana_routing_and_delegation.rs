//! Routing and delegation: which identity an entry authorizes as, and what a delegation record has
//! to say for a delegated entry to stand.
//!
//! Routing is per entry and decided by one comparison: the entry's owner against the permit's
//! signer. Equal means the signer is the subject whose access is proven; unequal means the owner is
//! that subject and, additionally, that the owner has delegated to the signer for the encrypted
//! value account's authority. One request mixes both freely, and different
//! delegators freely — there is no delegated mode and no delegated route.
//!
//! The negative test that matters most is the substitution of the subject. In the delegated branch it
//! would be easy, and wrong, to prove access for the signer: the signer is the one asking, and the
//! delegation record does name them. But the handle belongs to the delegator, and it is the
//! delegator's standing in the encrypted value account that the delegation extends. Proving the
//! signer's own standing instead would authorize a delegate against encrypted value accounts the
//! delegator never had access to, wherever the delegate happens to be a member. Two tests below pin
//! the direction from both sides: the delegator's membership authorizes, and the signer's own
//! membership does not.
//!
//! Freshness is evaluated against the observed slot, and the record's `delegation_counter` takes no
//! part in it. That absence is deliberate: pinning the counter in the request would let any
//! unrelated update to any delegation record invalidate requests already in flight, and would make
//! a mixed-delegator batch impossible to build.
//!
//! Two rows can carry one grant — the encrypted value account's authority, and the delegator's
//! wildcard row — and the last section pins that rule from both sides: either row alone authorizes,
//! neither vetoes the other, and revoking one leaves the other standing. That last property is the
//! price of wildcard scope and is asserted deliberately, not tolerated.

mod solana_support;

use kms_worker::core::solana::{
    delegation::{AuthorizedRow, DelegationFailure, check_delegation, wildcard_delegation_address},
    encrypted_value_account::EncryptedValueAccountFailure,
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::HandleBindingFailure,
    pipeline::{AuthorizationContext, AuthorizedRequest, authorize_request},
    request::SolanaUserDecryptRequest,
    snapshot::{SnapshotAccount, SnapshotError, SnapshotKeys},
};
use kms_worker::core::solana_acl::{SolanaPubkeyBytes, WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY};
use solana_support::*;

const OBSERVED_SLOT: u64 = 500;

/// Authorizes a request against a world, returning the outcome and how many reads it cost.
async fn authorize_in(
    world: World,
    request: &SolanaUserDecryptRequest,
) -> (Result<AuthorizedRequest, AuthorizationFailure>, usize) {
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();
    let context = AuthorizationContext {
        deployment: &deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    };
    let outcome = authorize_request(&reader, &ServableKmsPair, context, request).await;
    (outcome, reader.call_count())
}

/// A world holding an encrypted value account, the signer's zero watermark, and whatever else is
/// added.
fn world_with(
    encrypted_value_account: &EncryptedValueAccountFixture,
    signer: SolanaPubkeyBytes,
) -> World {
    World::at_slot(OBSERVED_SLOT)
        .with_encrypted_value_account(encrypted_value_account)
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
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[signer.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .direct_current(&encrypted_value_account, live)
        .typed();

    let (outcome, reads) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()),
        &request,
    )
    .await;

    let authorized = outcome.expect("the signer owns the handle and is a member");
    assert_eq!(authorized.entries()[0].subject, signer.pubkey());
    assert_eq!(reads, 1, "a direct entry needs no delegation record");
}

/// An entry owned by somebody else authorizes as *that* owner — the delegator — and the delegator's
/// membership is what has to hold. The encrypted value account here names only the delegator, so an
/// implementation proving the signer's standing instead would fail this test.
#[tokio::test]
async fn a_delegated_entry_authorizes_as_the_delegator() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x11, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, reads) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&delegation),
        &request,
    )
    .await;

    let authorized = outcome.expect("a live delegation lets the signer use the delegator's access");
    assert_eq!(
        authorized.entries()[0].subject,
        delegator.pubkey(),
        "the subject of a delegated entry is the delegator, not the delegate"
    );
    assert_eq!(reads, 2);
}

/// The mirror image, and the substitution this whole branch has to refuse: the signer is a member
/// of the encrypted value account in their own right, the delegator is not, and a live delegation
/// exists between them. Nothing about that combination gives the signer access to a handle the
/// delegator owns.
///
/// An implementation that proved the signer's membership would accept this, and would thereby let
/// any delegate reach any encrypted value account they happen to be a member of while attributing
/// the handle to somebody else.
#[tokio::test]
async fn a_delegated_entry_is_not_authorized_by_the_delegates_own_membership() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x12, FHE_TYPE_UINT64);
    // The signer is the member; the delegator is not.
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[signer.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&delegation),
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
                source: HandleBindingFailure::NotAMember { subject }
            } if subject == delegator.pubkey()
        ),
        "the subject that failed membership must be the delegator, got {failure}"
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
    let own_encrypted_value_account =
        EncryptedValueAccountFixture::in_domain(DOMAIN, AUTHORITY, LABEL, own, &[signer.pubkey()]);
    let mut first_label = LABEL;
    first_label[0] = b'1';
    let first_encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        AUTHORITY,
        first_label,
        first,
        &[first_delegator.pubkey()],
    );
    let mut second_label = LABEL;
    second_label[0] = b'2';
    let second_encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        AUTHORITY,
        second_label,
        second,
        &[second_delegator.pubkey()],
    );
    let first_delegation =
        DelegationFixture::live(first_delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let second_delegation =
        DelegationFixture::live(second_delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);

    let request = RequestBuilder::new(&signer)
        .direct_current(&own_encrypted_value_account, own)
        .delegated_current(
            &first_encrypted_value_account,
            first,
            first_delegator.pubkey(),
        )
        .delegated_current(
            &second_encrypted_value_account,
            second,
            second_delegator.pubkey(),
        )
        .typed();
    let world = World::at_slot(OBSERVED_SLOT)
        .with_encrypted_value_account(&own_encrypted_value_account)
        .with_encrypted_value_account(&first_encrypted_value_account)
        .with_encrypted_value_account(&second_encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&first_delegation)
        .with_delegation(&second_delegation);

    let (outcome, reads) = authorize_in(world, &request).await;

    let authorized = outcome.expect("a mixed batch is an ordinary request");
    let subjects: Vec<SolanaPubkeyBytes> = authorized.entries().iter().map(|e| e.subject).collect();
    assert_eq!(
        subjects,
        vec![
            signer.pubkey(),
            first_delegator.pubkey(),
            second_delegator.pubkey()
        ]
    );
    assert_eq!(
        reads, 2,
        "two delegators still cost one extra read, not two"
    );
}

// ---------------------------------------------------------------------------
// Freshness
// ---------------------------------------------------------------------------

/// A delegated scenario, parameterised by what the delegation record says.
async fn authorize_delegated(
    delegation: DelegationFixture,
) -> Result<AuthorizedRequest, AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x30, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let world = world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&delegation);
    authorize_in(world, &request).await.0
}

/// The delegation of the scenario above, live at the observed slot.
fn live_delegation() -> DelegationFixture {
    DelegationFixture::live(
        Wallet::new(2).pubkey(),
        Wallet::new(1).pubkey(),
        OBSERVED_SLOT,
    )
}

/// A revoked delegation stops the request immediately — while the signer's own permit is
/// untouched, which is the whole point of having two independent levers.
#[tokio::test]
async fn a_revoked_delegation_rejects_its_entry() {
    let mut revoked = live_delegation();
    revoked.revoked = true;

    let failure = authorize_delegated(revoked)
        .await
        .expect_err("a revoked delegation authorizes nothing");

    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Revoked
        }
    ));
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// Expiry is measured against the observed slot, not against a local clock.
#[tokio::test]
async fn a_delegation_expired_at_the_observation_rejects_its_entry() {
    let mut expired = live_delegation();
    expired.expiration_slot = OBSERVED_SLOT - 1;

    let failure = authorize_delegated(expired)
        .await
        .expect_err("an expired delegation authorizes nothing");

    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Expired {
                expiration_slot,
                observed_slot
            }
        } if expiration_slot == OBSERVED_SLOT - 1 && observed_slot == OBSERVED_SLOT
    ));
}

/// The expiration slot is inclusive: a delegation valid through slot N is valid at slot N.
#[tokio::test]
async fn a_delegation_expiring_at_the_observed_slot_is_still_live() {
    let mut boundary = live_delegation();
    boundary.expiration_slot = OBSERVED_SLOT;

    authorize_delegated(boundary)
        .await
        .expect("the expiration slot itself is inside the delegation's life");
}

/// A record written after the observation is not part of the state this authorization saw.
/// Accepting it would mean authorizing from a mixture of two points in time — the delegation from
/// one, everything else from another.
#[tokio::test]
async fn a_delegation_written_after_the_observation_rejects_its_entry() {
    let mut from_the_future = live_delegation();
    from_the_future.last_update_slot = OBSERVED_SLOT + 1;

    let failure = authorize_delegated(from_the_future)
        .await
        .expect_err("a record newer than the observation is not in it");

    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NewerThanObservation {
                last_update_slot,
                observed_slot
            }
        } if last_update_slot == OBSERVED_SLOT + 1 && observed_slot == OBSERVED_SLOT
    ));
    // Transient, and pinned as such: a coherent node cannot return a record written after its own
    // context slot, so this names a bad observation rather than a dead grant. Terminal would let
    // one bad RPC response kill a valid request.
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// A record written exactly at the observation is part of it.
#[tokio::test]
async fn a_delegation_written_at_the_observation_is_live() {
    let mut boundary = live_delegation();
    boundary.last_update_slot = OBSERVED_SLOT;

    authorize_delegated(boundary)
        .await
        .expect("a record written in the observed slot is observed");
}

// ---------------------------------------------------------------------------
// Wildcard scope
// ---------------------------------------------------------------------------

/// The same delegated scenario, parameterised by which rows the world holds.
async fn authorize_delegated_with_rows(
    rows: &[DelegationFixture],
) -> Result<AuthorizedRequest, AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x31, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let mut world = world_with(&encrypted_value_account, signer.pubkey());
    for row in rows {
        world = world.with_delegation(row);
    }
    authorize_in(world, &request).await.0
}

/// The wildcard row of the same pair, live at the observed slot.
fn live_wildcard() -> DelegationFixture {
    DelegationFixture::live_wildcard(
        Wallet::new(2).pubkey(),
        Wallet::new(1).pubkey(),
        OBSERVED_SLOT,
    )
}

/// A wildcard row covers an encrypted value account that has no row of its own — the EVM ACL's
/// wildcard delegation, with the sentinel standing where an encrypted value account authority
/// would.
#[tokio::test]
async fn a_wildcard_row_authorizes_a_encrypted_value_account_with_no_app_specific_row() {
    let authorized = authorize_delegated_with_rows(&[live_wildcard()])
        .await
        .expect("a wildcard row covers every app of its delegator");

    assert_eq!(
        authorized.entries()[0].subject,
        Wallet::new(2).pubkey(),
        "the wildcard row changes which record authorizes, not whose access is proven"
    );
}

/// The authority-specific row is tried first, so a dead wildcard row beside it changes nothing.
#[tokio::test]
async fn an_app_specific_row_authorizes_while_the_wildcard_row_is_dead() {
    let mut dead_wildcard = live_wildcard();
    dead_wildcard.revoked = true;

    authorize_delegated_with_rows(&[live_delegation(), dead_wildcard])
        .await
        .expect("one live row is the whole requirement");
}

/// The consequence of the rule, stated as a test rather than left to be discovered: revoking the
/// authority-specific row does not stop a delegate who also holds a wildcard row. Scope-by-app is a
/// property of a row, so narrowing one app takes revoking both rows — two transactions, because the
/// host program's revocation instruction takes one record per call.
#[tokio::test]
async fn revoking_the_app_specific_row_does_not_stop_a_wildcard_delegate() {
    let mut revoked = live_delegation();
    revoked.revoked = true;

    authorize_delegated_with_rows(&[revoked, live_wildcard()])
        .await
        .expect("the wildcard row still authorizes, which is what wildcard means");
}

/// Neither row can veto the other. An authority-specific row written after the observation is not part of
/// the state this authorization saw — and that says nothing about the wildcard row, which is.
#[tokio::test]
async fn an_app_specific_row_newer_than_the_observation_does_not_veto_the_wildcard_row() {
    let mut from_the_future = live_delegation();
    from_the_future.last_update_slot = OBSERVED_SLOT + 1;

    authorize_delegated_with_rows(&[from_the_future, live_wildcard()])
        .await
        .expect("a row outside the observation cannot invalidate one inside it");
}

/// The mirror: a wildcard row from the future does not reach a live authority-specific row.
#[tokio::test]
async fn a_wildcard_row_newer_than_the_observation_does_not_veto_the_app_specific_row() {
    let mut from_the_future = live_wildcard();
    from_the_future.last_update_slot = OBSERVED_SLOT + 1;

    authorize_delegated_with_rows(&[live_delegation(), from_the_future])
        .await
        .expect("a row outside the observation cannot invalidate one inside it");
}

/// When both rows exist and neither is live, both reasons are reported. Naming one would send the
/// delegate to fix a row that was not the only thing standing in the way.
#[tokio::test]
async fn two_dead_rows_report_both_reasons() {
    let mut revoked = live_delegation();
    revoked.revoked = true;
    let mut expired_wildcard = live_wildcard();
    expired_wildcard.expiration_slot = OBSERVED_SLOT - 1;

    let failure = authorize_delegated_with_rows(&[revoked, expired_wildcard])
        .await
        .expect_err("two dead rows authorize nothing");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveGrant { exact, wildcard },
        } => {
            assert!(matches!(**exact, DelegationFailure::Revoked));
            assert!(matches!(**wildcard, DelegationFailure::Expired { .. }));
        }
        other => panic!("expected both reasons, got {other}"),
    }
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// Holding no wildcard row is the ordinary case, and in it the authority-specific reason is reported as
/// itself: the rule gained a second row, not a second sentence in every diagnostic.
#[tokio::test]
async fn without_a_wildcard_row_the_app_specific_reason_is_reported_alone() {
    let mut revoked = live_delegation();
    revoked.revoked = true;

    let failure = authorize_delegated_with_rows(&[revoked])
        .await
        .expect_err("a revoked row authorizes nothing");

    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Revoked
        }
    ));
}

/// A wildcard row is per delegator: somebody else's covers nothing here, because its address is
/// derived from the delegator this entry names.
#[tokio::test]
async fn a_wildcard_row_of_another_delegator_authorizes_nothing() {
    let stranger = DelegationFixture::live_wildcard(
        Wallet::new(9).pubkey(),
        Wallet::new(1).pubkey(),
        OBSERVED_SLOT,
    );

    let failure = authorize_delegated_with_rows(&[stranger])
        .await
        .expect_err("a wildcard row of another delegator is not at this entry's address");

    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Absent { .. }
        }
    ));
}

/// The delegated scenario of this section, with an arbitrary account planted at the delegator's
/// wildcard address and no authority-specific row beside it.
async fn authorize_with_wildcard_account(
    account: SnapshotAccount,
) -> Result<AuthorizedRequest, AuthorizationFailure> {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x39, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let (wildcard_key, _) = live_wildcard().address();
    let world =
        world_with(&encrypted_value_account, signer.pubkey()).with_account(wildcard_key, account);
    authorize_in(world, &request).await.0
}

/// An account under another program's ownership at the wildcard address is reported as what it
/// is, beside the absent authority-specific row — not swallowed as "no wildcard row".
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
            source: DelegationFailure::NoLiveGrant { exact, wildcard },
        } => {
            assert!(matches!(**exact, DelegationFailure::Absent { .. }));
            assert!(matches!(**wildcard, DelegationFailure::ForeignOwner { .. }));
        }
        other => panic!("expected both reasons, got {other}"),
    }
}

/// A wildcard row storing a bump other than the canonical one for its address is not the record
/// this reader reads — the same rule the authority-specific row is already held to.
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
            source: DelegationFailure::NoLiveGrant { exact, wildcard },
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
    let stranger_row = DelegationFixture::live_wildcard(
        Wallet::new(9).pubkey(),
        Wallet::new(1).pubkey(),
        OBSERVED_SLOT,
    );

    let failure = authorize_with_wildcard_account(stranger_row.account())
        .await
        .expect_err("a wildcard row must name the tuple it was read for");

    match &failure {
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NoLiveGrant { exact, wildcard },
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
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (expected_key, _) = expected.address();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("an absent delegation authorizes nothing");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Absent { account_key }
        } if account_key == expected_key
    ));
    // Transient, and pinned as such: this reader and the relayer read through their own RPCs and
    // can sit at different confirmed slots, so "not here" can mean "not here yet". Terminal would
    // fail a valid delegated request permanently over ordinary replica lag.
    assert_eq!(failure.class(), FailureClass::Transient);
}

/// The other side of the same rule: a revoked, expired or mismatched record stays terminal,
/// because each describes what a read record says and no later observation changes that. Pinned
/// together so a future edit to the transient arms cannot quietly take the rest with them.
#[tokio::test]
async fn delegation_outcomes_about_a_record_that_exists_stay_terminal() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let stranger = Wallet::new(9);
    let live = handle(0x3f, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (expected_key, _) = expected.address();

    let mut revoked = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    revoked.revoked = true;
    let mut expired = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    expired.expiration_slot = OBSERVED_SLOT - 1;
    // Somebody else's tuple, planted at the address this request reads: changing a tuple field
    // moves the record's own address, so a mismatch only exists when the record is placed by
    // address rather than derived from itself.
    let mut mismatched = DelegationFixture::live(stranger.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    mismatched.encrypted_value_account_authority = AUTHORITY;

    for (what, delegation) in [
        ("revoked", revoked),
        ("expired", expired),
        ("tuple mismatch", mismatched),
    ] {
        let (outcome, _) = authorize_in(
            world_with(&encrypted_value_account, signer.pubkey())
                .with_account(expected_key, delegation.account()),
            &request,
        )
        .await;
        let failure = outcome.expect_err("none of these authorize");
        assert_eq!(
            failure.class(),
            FailureClass::Terminal,
            "a {what} delegation must stay terminal"
        );
    }
}

/// A delegation is scoped to an encrypted value account authority, and the encrypted value account
/// authority is the encrypted value account's. A delegation for another app is simply not the
/// record that gets read — the address derived from the encrypted value account's app is empty.
#[tokio::test]
async fn a_delegation_for_another_app_does_not_authorize() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let other_app: SolanaPubkeyBytes = [0x77; 32];
    let live = handle(0x32, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        AUTHORITY,
        LABEL,
        live,
        &[delegator.pubkey()],
    );
    let mut elsewhere = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    elsewhere.encrypted_value_account_authority = other_app;
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&elsewhere),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a delegation for another app is not this app's delegation");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::Absent { .. }
        }
    ));
}

/// A record sitting at the canonical address while naming a different tuple is rejected. The
/// address alone is not taken as proof of what the record says.
#[tokio::test]
async fn a_delegation_record_naming_another_tuple_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let stranger = Wallet::new(9);
    let live = handle(0x33, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (expected_key, _) = expected.address();
    // A record for a different delegator, planted at the address the request will read.
    let mut foreign = DelegationFixture::live(stranger.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    foreign.encrypted_value_account_authority = AUTHORITY;
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey())
            .with_account(expected_key, foreign.account()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a record must name the tuple it was read for");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::TupleMismatch { .. }
        }
    ));
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
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let expected = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (expected_key, _) = expected.address();
    // A record delegating to somebody else, planted at the address the request will read.
    let other_delegate =
        DelegationFixture::live(delegator.pubkey(), stranger.pubkey(), OBSERVED_SLOT);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey())
            .with_account(expected_key, other_delegate.account()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a record delegating to somebody else authorizes nothing");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::TupleMismatch { account_key }
        } if account_key == expected_key
    ));
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
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (key, canonical_bump) = delegation.address();
    let mut wrong_bump = delegation.account();
    let last = wrong_bump.data.len() - 1;
    assert_eq!(
        wrong_bump.data[last], canonical_bump,
        "the fixture writes the canonical bump, or this test proves nothing"
    );
    wrong_bump.data[last] = canonical_bump.wrapping_sub(1);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_account(key, wrong_bump),
        &request,
    )
    .await;

    let failure = outcome.expect_err("a non-canonical bump is not this record");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::NotADelegationRecord { .. }
        }
    ));
}

/// An account under another program's ownership is not a delegation, whatever it contains.
#[tokio::test]
async fn a_delegation_record_owned_by_another_program_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x34, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let (key, _) = delegation.address();
    let mut impostor = delegation.account();
    impostor.owner = [0xee; 32];
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_account(key, impostor),
        &request,
    )
    .await;

    let failure = outcome.expect_err("only the host program grants delegations");
    assert!(matches!(
        failure,
        AuthorizationFailure::Delegation {
            index: 0,
            source: DelegationFailure::ForeignOwner { .. }
        }
    ));
}

/// The same permit, twice, across a revocation: the first request authorizes and the second does
/// not. Reuse of a permit is not reuse of an authorization — there is no cached verdict, so the
/// delegator's revocation takes effect on the next request rather than on the next permit.
#[tokio::test]
async fn the_same_permit_stops_working_once_the_delegation_is_revoked() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x35, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let granted = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let mut revoked = granted;
    revoked.revoked = true;
    revoked.last_update_slot = OBSERVED_SLOT;

    let (first, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&granted),
        &request,
    )
    .await;
    first.expect("the first request is authorized");

    let (second, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&revoked),
        &request,
    )
    .await;
    assert!(
        matches!(
            second.expect_err("the second request is not"),
            AuthorizationFailure::Delegation {
                source: DelegationFailure::Revoked,
                ..
            }
        ),
        "the identical request must be re-authorized from scratch"
    );
}

// ---------------------------------------------------------------------------
// The authorizing row
// ---------------------------------------------------------------------------

/// `check_delegation` names the row that authorized. The distinction is invisible in the request's
/// outcome — either row authorizes identically — but an audit record of a delegated authorization
/// has to tell an authority-scoped grant from a wildcard one, and only this function knows which
/// row stood behind the entry.
#[test]
fn a_live_authority_specific_row_is_named_as_the_exact_row() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let exact = DelegationFixture::live(delegator, delegate, OBSERVED_SLOT);
    let (exact_key, _) = exact.address();
    let (wildcard_key, _) = wildcard_delegation_address(PROGRAM_ID, delegator, delegate);
    let snapshot = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&exact)
        .read(&SnapshotKeys::new([exact_key, wildcard_key]))
        .expect("both row addresses are in the planned key set");

    let row = check_delegation(
        &snapshot,
        PROGRAM_ID,
        delegator,
        delegate,
        exact.encrypted_value_account_authority,
    )
    .expect("a live authority-specific row authorizes");

    assert_eq!(row, AuthorizedRow::Exact);
}

/// The same grant carried by the wildcard row alone is named as such: the row that authorized is
/// reported, not merely the fact that one did.
#[test]
fn a_live_wildcard_row_is_named_as_the_wildcard_row() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let wildcard = DelegationFixture::live_wildcard(delegator, delegate, OBSERVED_SLOT);
    let (wildcard_key, _) = wildcard.address();
    let exact = DelegationFixture::live(delegator, delegate, OBSERVED_SLOT);
    let (exact_key, _) = exact.address();
    let snapshot = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&wildcard)
        .read(&SnapshotKeys::new([exact_key, wildcard_key]))
        .expect("both row addresses are in the planned key set");

    let row = check_delegation(
        &snapshot,
        PROGRAM_ID,
        delegator,
        delegate,
        exact.encrypted_value_account_authority,
    )
    .expect("a live wildcard row authorizes an authority with no row of its own");

    assert_eq!(row, AuthorizedRow::Wildcard);
}

/// With BOTH rows live, the authority-specific row is the one named. The request outcome is
/// identical either way, so nothing but this assertion notices a reordering of the two checks —
/// which would silently relabel every such authorization in the audit record as wildcard-carried.
#[test]
fn with_both_rows_live_the_authority_specific_row_is_the_one_named() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let exact = DelegationFixture::live(delegator, delegate, OBSERVED_SLOT);
    let wildcard = DelegationFixture::live_wildcard(delegator, delegate, OBSERVED_SLOT);
    let (exact_key, _) = exact.address();
    let (wildcard_key, _) = wildcard.address();
    let snapshot = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&exact)
        .with_delegation(&wildcard)
        .read(&SnapshotKeys::new([exact_key, wildcard_key]))
        .expect("both row addresses are in the planned key set");

    let row = check_delegation(
        &snapshot,
        PROGRAM_ID,
        delegator,
        delegate,
        exact.encrypted_value_account_authority,
    )
    .expect("two live rows authorize");

    assert_eq!(row, AuthorizedRow::Exact);
}

// ---------------------------------------------------------------------------
// Sentinel injection
// ---------------------------------------------------------------------------

/// An encrypted value account naming the wildcard sentinel as its authority is rejected at
/// resolution, before any delegation row is read. The address of the authority-specific row is
/// derived from that authority, and with the sentinel in it the derivation lands on the wildcard
/// row itself — the authority-specific check would be structurally a wildcard check. On-chain the
/// authority signs `fhe_execute`, so no legal encrypted value account carries the sentinel; one
/// that does is rejected, not interpreted.
///
/// The world here holds a live wildcard row — exactly the row a sentinel authority resolves to —
/// so an implementation without the guard authorizes this request.
#[tokio::test]
async fn a_sentinel_authority_in_the_encrypted_value_account_rejects_a_delegated_entry() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x36, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
        LABEL,
        live,
        &[delegator.pubkey()],
    );
    let wildcard =
        DelegationFixture::live_wildcard(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()).with_delegation(&wildcard),
        &request,
    )
    .await;

    let failure = outcome
        .expect_err("a sentinel authority must be rejected, not resolved to the wildcard row");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::EncryptedValueAccount {
                index: 0,
                source: EncryptedValueAccountFailure::SentinelAuthority { .. }
            }
        ),
        "the rejection belongs to the encrypted value account resolution, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// The guard lives in the resolution of the encrypted value account, so a direct entry under a
/// sentinel authority is rejected the same way. Deliberate: such an account is illegitimate
/// whether or not a delegation is in play, and one rule at the chokepoint beats a rule that only
/// the delegated branch remembers to apply.
#[tokio::test]
async fn a_sentinel_authority_in_the_encrypted_value_account_rejects_a_direct_entry_too() {
    let signer = Wallet::new(1);
    let live = handle(0x37, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        WILDCARD_ENCRYPTED_VALUE_ACCOUNT_AUTHORITY,
        LABEL,
        live,
        &[signer.pubkey()],
    );
    let request = RequestBuilder::new(&signer)
        .direct_current(&encrypted_value_account, live)
        .typed();

    let (outcome, _) = authorize_in(
        world_with(&encrypted_value_account, signer.pubkey()),
        &request,
    )
    .await;

    let failure = outcome.expect_err("the sentinel is not an authority any account may name");
    assert!(matches!(
        failure,
        AuthorizationFailure::EncryptedValueAccount {
            index: 0,
            source: EncryptedValueAccountFailure::SentinelAuthority { .. }
        }
    ));
    assert_eq!(failure.class(), FailureClass::Terminal);
}

// ---------------------------------------------------------------------------
// What the delegated branch reads, and what it refuses to read
// ---------------------------------------------------------------------------

/// A delegation key the snapshot never read is an error of key planning, not a verdict about the
/// delegation: it can never fold into "no live grant" and reach a client as a statement about the
/// state of the world.
#[test]
fn a_delegation_key_the_snapshot_never_read_is_an_error_not_a_verdict() {
    let delegate = Wallet::new(1).pubkey();
    let delegator = Wallet::new(2).pubkey();
    let mut revoked = DelegationFixture::live(delegator, delegate, OBSERVED_SLOT);
    revoked.revoked = true;
    let (exact_key, _) = revoked.address();
    // The authority-specific row is dead, so the rule proceeds to the wildcard row — whose key
    // was never planned.
    let snapshot = World::at_slot(OBSERVED_SLOT)
        .with_delegation(&revoked)
        .read(&SnapshotKeys::new([exact_key]))
        .expect("the planned key is readable");

    let failure = check_delegation(
        &snapshot,
        PROGRAM_ID,
        delegator,
        delegate,
        revoked.encrypted_value_account_authority,
    )
    .expect_err("a missing key cannot authorize");

    assert!(matches!(
        failure,
        DelegationFailure::Snapshot(SnapshotError::KeyNotInSnapshot { .. })
    ));
}

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
    let own_encrypted_value_account =
        EncryptedValueAccountFixture::in_domain(DOMAIN, AUTHORITY, LABEL, own, &[signer.pubkey()]);
    let mut first_label = LABEL;
    first_label[0] = b'3';
    let first_encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        AUTHORITY,
        first_label,
        first,
        &[first_delegator.pubkey()],
    );
    let mut second_label = LABEL;
    second_label[0] = b'4';
    let second_encrypted_value_account = EncryptedValueAccountFixture::in_domain(
        DOMAIN,
        AUTHORITY,
        second_label,
        second,
        &[second_delegator.pubkey()],
    );
    let first_delegation =
        DelegationFixture::live(first_delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let mut second_delegation =
        DelegationFixture::live(second_delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    second_delegation.revoked = true;

    let request = RequestBuilder::new(&signer)
        .direct_current(&own_encrypted_value_account, own)
        .delegated_current(
            &first_encrypted_value_account,
            first,
            first_delegator.pubkey(),
        )
        .delegated_current(
            &second_encrypted_value_account,
            second,
            second_delegator.pubkey(),
        )
        .typed();
    let world = World::at_slot(OBSERVED_SLOT)
        .with_encrypted_value_account(&own_encrypted_value_account)
        .with_encrypted_value_account(&first_encrypted_value_account)
        .with_encrypted_value_account(&second_encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_delegation(&first_delegation)
        .with_delegation(&second_delegation);

    let (outcome, _) = authorize_in(world, &request).await;

    let failure = outcome.expect_err("one dead delegation rejects the request");
    assert!(
        matches!(
            failure,
            AuthorizationFailure::Delegation {
                index: 2,
                source: DelegationFailure::Revoked
            }
        ),
        "the failure must name the entry whose delegation is dead, got {failure}"
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
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), OBSERVED_SLOT);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    // A watermark that would invalidate any permit — were it ever read for this request.
    let world = world_with(&encrypted_value_account, signer.pubkey())
        .with_delegation(&delegation)
        .with_watermark(delegator.pubkey(), u64::MAX);

    let (outcome, reads) = authorize_in(world, &request).await;

    outcome.expect("the delegator's permit watermark plays no part in a delegated request");
    assert_eq!(reads, 2, "no extra read fetches the delegator's watermark");
}
