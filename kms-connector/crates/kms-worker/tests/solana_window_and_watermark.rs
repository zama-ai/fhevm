//! The validity window and the invalidation watermark: the two bounds on when a permit is
//! usable.
//!
//! Together they pin the start of any usable permit into `[last revocation, now]` at the moment
//! of evaluation — the window from above, the watermark from below. Neither bound is a permit
//! field: the window is signed but evaluated against a clock the signer does not control, and
//! the watermark is host state the signer writes with one transaction.
//!
//! That one transaction is the point of the design, and it works because of the window: a permit
//! signed before a revocation necessarily started before it, so raising the watermark to the
//! moment of revocation kills every outstanding permit at once. Its documented gap is a permit
//! pre-signed with a future start, which is unusable when the revocation happens and becomes
//! usable afterwards. The gap is accepted deliberately, in parity with the EVM path, and the
//! test below states the behaviour rather than wishing it away.
//!
//! The watermark's key is the request signer. In a delegated flow that is the delegate: the
//! delegate's lever kills the delegate's permits, and the delegator's lever is the delegation
//! record, not this one.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    pipeline::{AuthorizationContext, authorize_request},
    snapshot::{SYSTEM_PROGRAM_ID, SnapshotAccount, SnapshotKeys},
    watermark::{
        WatermarkFailure, WindowFailure, check_not_invalidated, check_window,
        permit_invalidation_address, read_watermark,
    },
};
use solana_support::*;

/// Reads the watermark of `user` out of a world.
fn watermark_in(world: &World, user: [u8; 32]) -> Result<u64, WatermarkFailure> {
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let snapshot = world
        .read(&SnapshotKeys::new([key]))
        .expect("the world reads");
    read_watermark(&snapshot, PROGRAM_ID, user)
}

fn context_at<'a>(
    deployment: &'a kms_worker::core::solana::deployment::DeploymentIdentity,
    now: u64,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: now,
    }
}

// ---------------------------------------------------------------------------
// The window
// ---------------------------------------------------------------------------

/// Inside the window, the rule says nothing.
#[test]
fn a_permit_inside_its_window_is_accepted() {
    check_window(DEFAULT_START, DEFAULT_DURATION, NOW_INSIDE_WINDOW)
        .expect("a permit in its window is usable");
}

/// The window is open at its exact start: the rule rejects a start *later* than now, not one
/// equal to it, so a permit is usable the second it names.
#[test]
fn the_window_is_open_at_its_exact_start() {
    check_window(DEFAULT_START, DEFAULT_DURATION, DEFAULT_START)
        .expect("the start second is inside the window");
}

/// The window is open at its exact end and closed one second later: `[start, start + duration]`
/// is the closed interval the Gateway applies on chain (`start + duration < now` rejects), and
/// this second line has to agree with it second for second.
#[test]
fn the_window_is_open_at_its_exact_end() {
    let end = DEFAULT_START + DEFAULT_DURATION;

    check_window(DEFAULT_START, DEFAULT_DURATION, end)
        .expect("the end second is inside the window");

    let failure = check_window(DEFAULT_START, DEFAULT_DURATION, end + 1)
        .expect_err("the second after the end is outside the window");

    assert!(matches!(
        failure,
        WindowFailure::Expired { end: e, now } if e == end && now == end + 1
    ));
    check_window(DEFAULT_START, 1, DEFAULT_START + 1)
        .expect("a one-second permit is usable at the second it names as its end");
    assert!(check_window(DEFAULT_START, 1, DEFAULT_START + 2).is_err());
}

/// An expired permit is terminal: no later observation makes it younger.
#[test]
fn an_expired_permit_is_terminal() {
    let now = DEFAULT_START + DEFAULT_DURATION + 1;

    let failure = check_window(DEFAULT_START, DEFAULT_DURATION, now).expect_err("expired");

    assert_eq!(
        AuthorizationFailure::Window(failure).class(),
        FailureClass::Terminal
    );
}

/// A permit whose window has not opened is rejected — without this rule the duration cap is
/// bypassed by pushing the start forward, and the watermark by pushing it past a future
/// revocation.
///
/// The rejection is transient, because it is the one rejection that time itself repairs: the
/// same request at a later observation is inside the window. That is also exactly the
/// documented gap — a permit pre-signed with a future start survives a revocation performed
/// before its window opens. This test pins the behaviour as accepted, not as desirable.
#[test]
fn a_permit_whose_window_has_not_opened_is_transient() {
    let now = DEFAULT_START - 1;

    let failure = check_window(DEFAULT_START, DEFAULT_DURATION, now)
        .expect_err("a permit is not usable before it starts");

    assert!(matches!(
        failure,
        WindowFailure::NotYetValid { start_timestamp, now: n }
            if start_timestamp == DEFAULT_START && n == now
    ));
    assert_eq!(
        AuthorizationFailure::Window(failure).class(),
        FailureClass::Transient
    );
}

// ---------------------------------------------------------------------------
// The watermark
// ---------------------------------------------------------------------------

/// A user who has never revoked has no record, and that reads as zero rather than as an error or
/// a missing initialisation step.
#[test]
fn an_absent_invalidation_record_reads_as_zero() {
    let user = Wallet::new(1).pubkey();

    let watermark = watermark_in(&World::at_slot(1), user).expect("an absent record is a zero");

    assert_eq!(watermark, 0);
}

/// A never-revoked user whose address someone else pre-funded reads as zero too. The two
/// observations say the same thing — the host program has never written here — and which of them a
/// user is presented as is not the user's choice: the address is derivable by anyone. Reading the
/// pre-funded one as a failure would sell a terminal denial of every request of any user for the
/// price of one transfer.
#[test]
fn a_prefunded_invalidation_address_reads_as_zero() {
    let user = Wallet::new(1).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let world = World::at_slot(1).with_account(key, prefunded_account());

    let watermark = watermark_in(&world, user).expect("a pre-funded address is a zero");

    assert_eq!(watermark, 0);
}

/// The exception is the empty account, not the System program: an account it owns that does carry
/// data is a layout this reader is not reading, and stays a rejection.
#[test]
fn a_system_owned_invalidation_account_carrying_data_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let mut impostor = invalidation_account(user, DEFAULT_START + 5);
    impostor.owner = SYSTEM_PROGRAM_ID;
    let world = World::at_slot(1).with_account(key, impostor);

    let failure =
        watermark_in(&world, user).expect_err("only an empty account reads as never written");

    assert!(matches!(failure, WatermarkFailure::ForeignOwner { .. }));
}

/// A stored watermark is read as written.
#[test]
fn a_stored_invalidation_record_reads_its_watermark() {
    let user = Wallet::new(1).pubkey();
    let world = World::at_slot(1).with_watermark(user, DEFAULT_START + 10);

    assert_eq!(
        watermark_in(&world, user).expect("a stored record reads"),
        DEFAULT_START + 10
    );
}

/// A permit that started before its signer's last revocation is dead, permanently: this is the
/// mechanism behind revoking every outstanding signature with one transaction.
#[test]
fn a_permit_starting_below_the_watermark_is_dead() {
    let failure = check_not_invalidated(DEFAULT_START, DEFAULT_START + 1)
        .expect_err("a revocation kills the permits that predate it");

    assert!(matches!(
        failure,
        WatermarkFailure::Invalidated {
            start_timestamp,
            watermark
        } if start_timestamp == DEFAULT_START && watermark == DEFAULT_START + 1
    ));
    assert_eq!(
        AuthorizationFailure::Watermark(failure).class(),
        FailureClass::Terminal,
        "no later observation resurrects it"
    );
}

/// A permit signed after a revocation starts at or above the watermark and is unaffected. The
/// boundary is inclusive on purpose: a permit signed in the same second as the revocation must
/// survive, or a user who revokes and immediately re-signs would be locked out for a second.
#[test]
fn a_permit_signed_at_or_after_the_revocation_is_unaffected() {
    check_not_invalidated(DEFAULT_START, DEFAULT_START).expect("the boundary second survives");
    check_not_invalidated(DEFAULT_START + 1, DEFAULT_START).expect("a later start survives");
}

/// The record's contents are checked against the address it was read from. Reading a watermark
/// out of some other user's record would let one user's revocation kill another's permits, and
/// reading zero out of a foreign layout would resurrect revoked ones.
#[test]
fn an_invalidation_record_naming_another_user_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let other = Wallet::new(2).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    // A record for another user, placed at this user's address.
    let world = World::at_slot(1).with_account(key, invalidation_account(other, DEFAULT_START));

    let failure =
        watermark_in(&world, user).expect_err("a record must name the user it was read for");

    assert!(matches!(
        failure,
        WatermarkFailure::RecordNamesAnotherUser { account_key } if account_key == key
    ));
}

/// An account of another type at the invalidation address is a rejection, not a zero.
#[test]
fn an_account_that_is_not_an_invalidation_record_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let encrypted_value_account =
        EncryptedValueAccountFixture::new(handle(0x10, FHE_TYPE_UINT64), &[user]);
    let world = World::at_slot(1).with_account(key, encrypted_value_account.account());

    let failure =
        watermark_in(&world, user).expect_err("encrypted value account bytes are not a watermark");

    assert!(matches!(
        failure,
        WatermarkFailure::NotAnInvalidationRecord { account_key } if account_key == key
    ));
}

/// An account under another program's ownership proves nothing, whatever its bytes say.
#[test]
fn an_invalidation_record_owned_by_another_program_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let mut impostor = invalidation_account(user, DEFAULT_START + 5);
    impostor.owner = [0xee; 32];
    let world = World::at_slot(1).with_account(key, impostor);

    let failure = watermark_in(&world, user).expect_err("a foreign program cannot set a watermark");

    assert!(matches!(failure, WatermarkFailure::ForeignOwner { .. }));
}

/// A record storing a bump other than the canonical one for its address is not the record this
/// reader reads. Nothing an attacker can arrange — only the host program writes these bytes — but a
/// record the program wrote under another derivation is a layout nobody promised, and reading a
/// watermark out of it would be a guess.
#[test]
fn an_invalidation_record_storing_a_non_canonical_bump_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let (key, canonical_bump) = permit_invalidation_address(PROGRAM_ID, user);
    let mut wrong_bump = invalidation_account(user, DEFAULT_START);
    let last = wrong_bump.data.len() - 1;
    assert_eq!(
        wrong_bump.data[last], canonical_bump,
        "the fixture writes the canonical bump, or this test proves nothing"
    );
    wrong_bump.data[last] = canonical_bump.wrapping_sub(1);
    let world = World::at_slot(1).with_account(key, wrong_bump);

    let failure = watermark_in(&world, user).expect_err("a non-canonical bump is not this record");

    assert!(matches!(
        failure,
        WatermarkFailure::NotAnInvalidationRecord { account_key } if account_key == key
    ));
}

/// A record too short to hold the layout is a rejection rather than a partial read.
#[test]
fn a_truncated_invalidation_record_is_rejected() {
    let user = Wallet::new(1).pubkey();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, user);
    let full = invalidation_account(user, DEFAULT_START);
    let truncated = SnapshotAccount {
        owner: PROGRAM_ID,
        data: full.data[..full.data.len() - 1].to_vec(),
    };
    let world = World::at_slot(1).with_account(key, truncated);

    assert!(watermark_in(&world, user).is_err());
}

/// The eight bytes this Connector looks for, pinned twice: as the literal a foreign
/// implementation can be compared against, and as the preimage it comes from.
///
/// The host program pins the same pair on its side. Neither side computes it through the other's
/// framework, which is the point — a rename or a derivation change has to fail in both places
/// rather than move them together.
#[test]
fn the_invalidation_record_discriminator_is_the_hash_of_the_account_name() {
    assert_eq!(
        PERMIT_INVALIDATION_DISCRIMINATOR,
        permit_invalidation_discriminator(),
        "the literal and its preimage have diverged"
    );
}

// ---------------------------------------------------------------------------
// Whose watermark
// ---------------------------------------------------------------------------

/// The watermark is keyed by the request signer, not by the owner of the handles. A delegator
/// who revokes their own permits does not thereby stop a delegate from using theirs — that is
/// what revoking the delegation is for.
#[tokio::test]
async fn the_watermark_is_keyed_by_the_signer_not_the_handle_owner() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x20, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    // The delegator revoked everything they ever signed; the delegate revoked nothing.
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0)
        .with_watermark(delegator.pubkey(), DEFAULT_START + DEFAULT_DURATION)
        .with_delegation(&delegation);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(
        &reader,
        &ServableKmsPair,
        context_at(&deployment, NOW_INSIDE_WINDOW),
        &request,
    )
    .await
    .expect("the delegator's revocation does not reach the delegate's permit");
}

/// The signer's own revocation does stop the request, delegated or not.
#[tokio::test]
async fn a_revocation_by_the_signer_stops_a_delegated_request() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let live = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[delegator.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);
    let request = RequestBuilder::new(&signer)
        .delegated_current(&encrypted_value_account, live, delegator.pubkey())
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), DEFAULT_START + 1)
        .with_delegation(&delegation);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        context_at(&deployment, NOW_INSIDE_WINDOW),
        &request,
    )
    .await
    .expect_err("the signer's own revocation kills their permit");

    assert!(matches!(
        failure,
        AuthorizationFailure::Watermark(WatermarkFailure::Invalidated { .. })
    ));
}

/// End to end: pre-funding a user's invalidation address does not deny them service. The whole
/// attack is one transfer to an address anyone can derive, so the request has to survive it — and
/// it has to survive it in the pipeline, not only in the reader, because the watermark is the one
/// rule where an absent account is the permissive reading.
#[tokio::test]
async fn a_prefunded_invalidation_address_does_not_deny_service() {
    let wallet = Wallet::new(1);
    let live = handle(0x23, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, live)
        .typed();
    let (key, _) = permit_invalidation_address(PROGRAM_ID, wallet.pubkey());
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_account(key, prefunded_account());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    authorize_request(
        &reader,
        &ServableKmsPair,
        context_at(&deployment, NOW_INSIDE_WINDOW),
        &request,
    )
    .await
    .expect("a donated lamport is not a revocation");
}

/// The window is evaluated against the time handed to authorization, so a permit that expired
/// between acceptance elsewhere and processing here is refused here. This is the second of the
/// two lines that check the window, and it is self-sufficient.
#[tokio::test]
async fn a_permit_that_expired_before_processing_is_refused() {
    let wallet = Wallet::new(1);
    let live = handle(0x22, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&encrypted_value_account, live)
        .typed();
    let world = World::at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        context_at(&deployment, DEFAULT_START + DEFAULT_DURATION + 1),
        &request,
    )
    .await
    .expect_err("an expired permit is refused whatever accepted it earlier");

    assert!(matches!(
        failure,
        AuthorizationFailure::Window(WindowFailure::Expired { .. })
    ));
    assert_eq!(
        reader.call_count(),
        0,
        "a permit outside its window costs no account read"
    );
}
