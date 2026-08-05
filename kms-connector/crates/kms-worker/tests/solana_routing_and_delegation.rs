//! Routing and delegation: which identity an entry authorizes as, and what a delegation record has
//! to say for a delegated entry to stand.
//!
//! Routing is per entry and decided by one comparison: the entry's owner against the permit's
//! signer. Equal means the signer is the subject whose access is proven; unequal means the owner is
//! that subject and, additionally, that the owner has delegated to the signer for the encrypted
//! value account's authority. One request mixes both freely, and different
//! delegators freely — there is no delegated mode and no delegated route.
//!
//! The load-bearing negative test is the substitution of the subject. In the delegated branch it
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
    delegation::DelegationFailure,
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::HandleBindingFailure,
    pipeline::{AuthorizationContext, AuthorizedRequest, authorize_request},
    request::SolanaUserDecryptRequest,
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
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
