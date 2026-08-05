//! Lineage resolution and the sourcing of app context: what a request may name, and what it
//! may never name.
//!
//! A handle entry names the lineage that authorizes it. That name is unsigned, so every value
//! read out of the named account has to be earned: the account must exist under this
//! deployment's program, carry the lineage type, and reproduce the identity that was claimed.
//! Only then are its fields — the app account, the ACL domain, the current handle, the subject
//! set — allowed to decide anything.
//!
//! The tests here come in two shapes. The first shape substitutes something for the lineage
//! and demands a rejection: a foreign program's account, another account type of the same
//! program, an account whose own fields describe a different lineage. The second shape asserts
//! the opposite direction — that the app context and the domain of every entry come from *its*
//! lineage, so a batch cannot smuggle a foreign-domain handle past a narrowly scoped permit,
//! and a request has no field with which to name an app context at all.
//!
//! One accept among the rejections deserves its own note: trailing bytes after the lineage
//! body are legal. The account is grown to its high-water mark and never shrunk, so a lineage
//! that once held more subjects than it holds now has a tail. Rejecting it would deny service
//! to exactly the accounts that have been used the most. The access proof takes the opposite
//! rule, and that asymmetry is deliberate.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    lineage::{LineageFailure, ResolvedLineage, resolve_lineage},
    pipeline::{AuthorizationContext, authorize_request},
    scope::{ScopeFailure, check_scope},
    snapshot::{SnapshotAccount, SnapshotKeys},
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_support::*;
use zama_solana_acl::encrypted_value_discriminator;

/// Resolves a lineage from a world holding exactly the account under test.
fn resolve_from(world: &World, value_key: [u8; 32]) -> Result<ResolvedLineage, LineageFailure> {
    let (account_key, _) =
        kms_worker::core::solana::lineage::lineage_address(PROGRAM_ID, value_key);
    let snapshot = world
        .read(&SnapshotKeys::new([account_key]))
        .expect("the world reads");
    resolve_lineage(&snapshot, PROGRAM_ID, value_key)
}

/// A lineage placed in a world, resolved.
fn resolved(lineage: &LineageFixture) -> ResolvedLineage {
    resolve_from(
        &World::at_slot(1).with_lineage(lineage),
        lineage.value_key(),
    )
    .expect("a well-formed lineage resolves")
}

fn context<'a>(
    deployment: &'a kms_worker::core::solana::deployment::DeploymentIdentity,
) -> AuthorizationContext<'a> {
    AuthorizationContext {
        deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
    }
}

// ---------------------------------------------------------------------------
// Presence, ownership, type, identity binding
// ---------------------------------------------------------------------------

/// The reference case: an account written by the host program at the address its own fields
/// derive, resolved through the same shared code the program runs.
#[test]
fn a_lineage_named_by_its_value_key_resolves() {
    let owner = Wallet::new(1).pubkey();
    let lineage = LineageFixture::new(handle(0x10, FHE_TYPE_UINT64), &[owner]);

    let resolved = resolve_from(
        &World::at_slot(1).with_lineage(&lineage),
        lineage.value_key(),
    )
    .expect("a well-formed lineage resolves");

    assert_eq!(resolved.account_key(), lineage.account_key);
    assert_eq!(resolved.app_account(), APP);
    assert_eq!(resolved.acl_domain_key(), DOMAIN);
}

/// An absent lineage is a rejection that may resolve itself: the account may simply not have
/// reached the observed commitment yet. Calling it terminal would strand requests that a later
/// observation would authorize.
#[test]
fn a_lineage_absent_at_the_observation_is_transient() {
    let lineage = LineageFixture::new(handle(0x11, FHE_TYPE_UINT64), &[Wallet::new(1).pubkey()]);

    let failure = resolve_from(&World::at_slot(1), lineage.value_key())
        .expect_err("an account that does not exist authorizes nothing");

    assert!(matches!(
        failure,
        LineageFailure::Absent { account_key } if account_key == lineage.account_key
    ));
    assert_eq!(
        AuthorizationFailure::Lineage {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Transient
    );
}

/// Program ownership is the sole trust anchor of the whole chain: nobody but the host program
/// can produce data in an account it owns. An account with impeccable contents under another
/// program's ownership proves nothing at all.
#[test]
fn a_lineage_account_owned_by_another_program_is_terminal() {
    let owner = Wallet::new(1).pubkey();
    let lineage = LineageFixture::new(handle(0x12, FHE_TYPE_UINT64), &[owner]);
    let mut impostor = lineage.account();
    impostor.owner = [0xee; 32];

    let failure = resolve_from(
        &World::at_slot(1).with_account(lineage.account_key, impostor),
        lineage.value_key(),
    )
    .expect_err("a foreign program's account is not a lineage");

    assert!(matches!(
        failure,
        LineageFailure::ForeignOwner { owner, .. } if owner == [0xee; 32]
    ));
    assert_eq!(
        AuthorizationFailure::Lineage {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

/// A host-owned account of a different type is caught by the discriminator rather than by
/// whatever its bytes happen to mean when read as a lineage. Here the substitute is a real
/// delegation record — the account type most likely to be confused with a lineage, since both
/// are written by the same program and both hold identities in their first bytes.
#[test]
fn a_host_owned_account_of_another_type_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let lineage = LineageFixture::new(handle(0x13, FHE_TYPE_UINT64), &[signer.pubkey()]);
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);

    let failure = resolve_from(
        &World::at_slot(1).with_account(lineage.account_key, delegation.account()),
        lineage.value_key(),
    )
    .expect_err("delegation-record bytes are not a lineage");

    assert!(matches!(
        failure,
        LineageFailure::WrongAccountType { account_key } if account_key == lineage.account_key
    ));
}

/// The identity binding is the backstop of the chain: the account's own fields must reproduce
/// the `valueKey` that was claimed. Without it, a bug anywhere in address derivation or account
/// selection would let a request read app context and subjects out of somebody else's lineage.
#[test]
fn a_lineage_whose_fields_derive_another_value_key_is_rejected() {
    let owner = Wallet::new(1).pubkey();
    let claimed = LineageFixture::new(handle(0x14, FHE_TYPE_UINT64), &[owner]);
    // A lineage of another app, placed at the claimed lineage's address.
    let foreign = LineageFixture::in_domain(
        DOMAIN,
        [0x33; 32],
        LABEL,
        handle(0x14, FHE_TYPE_UINT64),
        &[owner],
    );
    assert_ne!(foreign.value_key(), claimed.value_key());

    let failure = resolve_from(
        &World::at_slot(1).with_account(claimed.account_key, foreign.account()),
        claimed.value_key(),
    )
    .expect_err("a lineage must reproduce the identity it was named by");

    assert!(matches!(
        failure,
        LineageFailure::ValueKeyMismatch { claimed: c, derived: d, .. }
            if c == claimed.value_key() && d == foreign.value_key()
    ));
}

/// The address is derived from the claimed identity, never supplied. A perfectly valid lineage
/// sitting at another address is not consulted, and naming it changes nothing about which
/// account is read.
#[test]
fn a_valid_lineage_at_another_address_is_never_consulted() {
    let owner = Wallet::new(1).pubkey();
    let named = LineageFixture::new(handle(0x15, FHE_TYPE_UINT64), &[owner]);
    let elsewhere = LineageFixture::in_domain(
        [0x44; 32],
        APP,
        LABEL,
        handle(0x15, FHE_TYPE_UINT64),
        &[owner],
    );

    let failure = resolve_from(
        &World::at_slot(1).with_lineage(&elsewhere),
        named.value_key(),
    )
    .expect_err("only the address derived from the claimed identity is read");

    assert!(matches!(failure, LineageFailure::Absent { .. }));
}

/// Trailing bytes are legal. A lineage account is realloc-grown to its high-water mark and
/// never shrunk, so the tail is the normal state of any account that has held more subjects
/// than it holds now.
#[test]
fn trailing_bytes_after_the_lineage_body_are_accepted() {
    let owner = Wallet::new(1).pubkey();
    let lineage = LineageFixture::new(handle(0x16, FHE_TYPE_UINT64), &[owner]);
    let mut grown = lineage.account();
    let body_len = grown.data.len();
    grown.data.extend_from_slice(&[0; 96]);

    let resolved = resolve_from(
        &World::at_slot(1).with_account(lineage.account_key, grown),
        lineage.value_key(),
    )
    .expect("a realloc-grown account resolves");

    assert_eq!(resolved.app_account(), APP);
    assert_eq!(
        8 + borsh::to_vec(resolved.lineage())
            .expect("the lineage serializes")
            .len(),
        body_len,
        "the decoded body ends where the account ended before the tail was appended, so the \
         accept really did ignore 96 surplus bytes"
    );
}

/// A body cut short is not the same thing as a body followed by extra bytes: the first is a
/// lineage that cannot be read, the second is a lineage with room to spare.
#[test]
fn a_lineage_with_a_truncated_body_is_rejected() {
    let owner = Wallet::new(1).pubkey();
    let lineage = LineageFixture::new(handle(0x17, FHE_TYPE_UINT64), &[owner]);
    let full = lineage.account();
    let truncated = SnapshotAccount {
        owner: PROGRAM_ID,
        data: full.data[..full.data.len() - 8].to_vec(),
    };

    let failure = resolve_from(
        &World::at_slot(1).with_account(lineage.account_key, truncated),
        lineage.value_key(),
    )
    .expect_err("a body that does not decode is not a lineage");

    assert!(matches!(failure, LineageFailure::Malformed { .. }));
}

/// An account holding only a discriminator is host-owned and of the right type, and still has
/// no lineage in it. The type check and the decode are two checks because an account can pass
/// the first and fail the second.
#[test]
fn a_lineage_account_holding_only_its_discriminator_is_rejected() {
    let lineage = LineageFixture::new(handle(0x18, FHE_TYPE_UINT64), &[Wallet::new(1).pubkey()]);
    let empty = SnapshotAccount {
        owner: PROGRAM_ID,
        data: encrypted_value_discriminator().to_vec(),
    };

    let failure = resolve_from(
        &World::at_slot(1).with_account(lineage.account_key, empty),
        lineage.value_key(),
    )
    .expect_err("a discriminator alone is not a lineage");

    assert!(matches!(failure, LineageFailure::Malformed { .. }));
}

// ---------------------------------------------------------------------------
// App context and scope
// ---------------------------------------------------------------------------

/// Each entry's app context comes from its own lineage. Two entries of the same domain and
/// different apps resolve to their own app accounts — there is no request-level app context to
/// share, and no first-entry value to inherit.
#[test]
fn each_entry_takes_its_app_context_from_its_own_lineage() {
    let owner = Wallet::new(1).pubkey();
    let first = LineageFixture::in_domain(
        DOMAIN,
        [0x51; 32],
        LABEL,
        handle(0x19, FHE_TYPE_UINT64),
        &[owner],
    );
    let second = LineageFixture::in_domain(
        DOMAIN,
        [0x52; 32],
        LABEL,
        handle(0x1a, FHE_TYPE_UINT64),
        &[owner],
    );

    assert_eq!(resolved(&first).app_account(), [0x51; 32]);
    assert_eq!(resolved(&second).app_account(), [0x52; 32]);
    assert_eq!(resolved(&first).acl_domain_key(), DOMAIN);
}

/// A scoped permit admits the domains it signed.
#[test]
fn a_scoped_permit_admits_a_lineage_of_a_signed_domain() {
    let lineage = LineageFixture::new(handle(0x1b, FHE_TYPE_UINT64), &[Wallet::new(1).pubkey()]);
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).scope(&[DOMAIN]);

    check_scope(
        permit.typed().allowed_acl_domain_keys(),
        &resolved(&lineage),
    )
    .expect("a signed domain is in scope");
}

/// A domain outside the signed set is rejected, and the domain that gets tested is the
/// lineage's — the only place it exists.
#[test]
fn a_lineage_outside_the_signed_scope_is_rejected() {
    let foreign_domain: SolanaPubkeyBytes = [0x61; 32];
    let lineage = LineageFixture::in_domain(
        foreign_domain,
        APP,
        LABEL,
        handle(0x1c, FHE_TYPE_UINT64),
        &[Wallet::new(1).pubkey()],
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).scope(&[DOMAIN]);

    let failure = check_scope(
        permit.typed().allowed_acl_domain_keys(),
        &resolved(&lineage),
    )
    .expect_err("an unsigned domain is out of scope");

    assert!(matches!(
        failure,
        ScopeFailure::DomainNotAllowed { domain } if domain == foreign_domain
    ));
}

/// An empty signed list is permissive and the rule is skipped, which is parity with the EVM
/// path rather than an optimization.
#[test]
fn a_permissive_permit_admits_a_lineage_of_any_domain() {
    let lineage = LineageFixture::in_domain(
        [0x71; 32],
        APP,
        LABEL,
        handle(0x1d, FHE_TYPE_UINT64),
        &[Wallet::new(1).pubkey()],
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).permissive();

    assert!(
        permit.typed().allowed_acl_domain_keys().is_permissive(),
        "the fixture really is permissive"
    );
    check_scope(
        permit.typed().allowed_acl_domain_keys(),
        &resolved(&lineage),
    )
    .expect("permissive skips the domain rule");
}

/// Scope is tested per handle, so a foreign-domain handle mixed into a batch fails the whole
/// request. Checking the first entry only would let a narrowly scoped permit decrypt whatever
/// was appended after it, and there is no partial release: a request is authorized entirely or
/// not at all.
#[tokio::test]
async fn a_foreign_domain_handle_later_in_the_batch_rejects_the_whole_request() {
    let wallet = Wallet::new(1);
    let in_scope_handle = handle(0x1e, FHE_TYPE_UINT64);
    let out_of_scope_handle = handle(0x1f, FHE_TYPE_UINT64);
    let in_scope = LineageFixture::new(in_scope_handle, &[wallet.pubkey()]);
    let out_of_scope = LineageFixture::in_domain(
        [0x81; 32],
        APP,
        LABEL,
        out_of_scope_handle,
        &[wallet.pubkey()],
    );
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).scope(&[DOMAIN]))
        .direct_current(&in_scope, in_scope_handle)
        .direct_current(&out_of_scope, out_of_scope_handle)
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&in_scope)
        .with_lineage(&out_of_scope)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("one out-of-scope entry rejects the request");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::Scope {
                index: 1,
                source: ScopeFailure::DomainNotAllowed { .. }
            }
        ),
        "the rejection names the offending entry, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
}

/// Permissive widens the domain rule and nothing else. Membership is unconditional, so a
/// permissive permit gets a non-member exactly as far as a scoped one does.
#[tokio::test]
async fn a_permissive_permit_does_not_widen_membership() {
    let wallet = Wallet::new(1);
    let stranger = Wallet::new(9);
    let live = handle(0x20, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(live, &[stranger.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).permissive())
        .direct_current(&lineage, live)
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&lineage)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect_err("permissive does not make a non-member a member");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding { index: 0, .. }
        ),
        "expected a membership failure, got {failure}"
    );
}
