//! Encrypted value account resolution and where an entry's authority comes from: what a request
//! may name, and what it may never name.
//!
//! A handle entry names the encrypted value account that authorizes it, by address. That name is
//! unsigned, so every value read out of the named account has to be earned: the account must
//! exist under this deployment's program, carry the encrypted value account type, and live at
//! the address its own fields derive. Only then are its fields — the encrypted value account
//! authority, the `(program, scope)` pair, the MMR peaks — allowed to decide anything.
//!
//! The tests here come in two shapes. The first shape substitutes something for the encrypted value
//! account and demands a rejection: a foreign program's account, another account type of the same
//! program, an account whose own fields describe a different encrypted value account. The second
//! shape asserts the opposite direction — that the authority and the application of every entry
//! come from *its* encrypted value account, so a batch cannot smuggle a foreign-application handle
//! past a narrowly scoped permit, and a request has no field with which to name an authority at
//! all.
//!
//! One accept among the rejections deserves its own note: trailing bytes after the encrypted value
//! account body are legal. The account is grown to its high-water mark and never shrunk, so an
//! encrypted value account whose MMR once held more peaks than it holds now has a tail.
//! Rejecting it would deny service to exactly the accounts that have been used the most.

mod solana_support;

use kms_worker::core::solana::{
    encrypted_value_account::{
        EncryptedValueAccountFailure, ResolvedEncryptedValueAccount,
        resolve_encrypted_value_account,
    },
    failure::{AuthorizationFailure, FailureClass},
    handle_binding::HandleBindingFailure,
    pipeline::{AuthorizationContext, authorize_request},
    scope::{ScopeFailure, check_scope},
    snapshot::{SnapshotAccount, SnapshotKeys},
};
use kms_worker::core::solana_acl::SolanaPubkeyBytes;
use solana_support::*;
use zama_solana_acl::encrypted_value_discriminator;

/// Resolves the account at `account_key` from a world.
fn resolve_from(
    world: &World,
    account_key: SolanaPubkeyBytes,
) -> Result<ResolvedEncryptedValueAccount, EncryptedValueAccountFailure> {
    let snapshot = world
        .read(&SnapshotKeys::new([account_key]))
        .expect("the world reads");
    resolve_encrypted_value_account(&snapshot, PROGRAM_ID, account_key)
}

/// An encrypted value account placed in a world, resolved.
fn resolved(
    encrypted_value_account: &EncryptedValueAccountFixture,
) -> ResolvedEncryptedValueAccount {
    resolve_from(
        &World::running_at_slot(1).with_encrypted_value_account(encrypted_value_account),
        encrypted_value_account.account_key,
    )
    .expect("a well-formed encrypted value account resolves")
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
// Presence, ownership, type, address binding
// ---------------------------------------------------------------------------

/// The reference case: an account written by the host program at the address its own fields
/// derive, resolved through the same shared code the program runs.
#[test]
fn an_encrypted_value_account_named_by_its_address_resolves() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x10, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );

    let resolved = resolve_from(
        &World::running_at_slot(1).with_encrypted_value_account(&encrypted_value_account),
        encrypted_value_account.account_key,
    )
    .expect("a well-formed encrypted value account resolves");

    assert_eq!(resolved.account_key(), encrypted_value_account.account_key);
    assert_eq!(resolved.encrypted_value_account_authority(), AUTHORITY);
    assert_eq!(resolved.program(), APP_PROGRAM);
    assert_eq!(resolved.scope(), SCOPE);
}

/// An absent encrypted value account is a rejection that may resolve itself: the account may simply
/// not have reached the observed commitment yet. Calling it terminal would strand requests that a
/// later observation would authorize.
#[test]
fn an_encrypted_value_account_absent_at_the_observation_is_transient() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x11, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );

    let failure = resolve_from(
        &World::running_at_slot(1),
        encrypted_value_account.account_key,
    )
    .expect_err("an account that does not exist authorizes nothing");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::Absent { account_key } if account_key == encrypted_value_account.account_key
    ));
    assert_eq!(
        AuthorizationFailure::EncryptedValueAccount {
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
fn an_encrypted_value_account_owned_by_another_program_is_terminal() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x12, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );
    let mut impostor = encrypted_value_account.account();
    impostor.owner = [0xee; 32];

    let failure = resolve_from(
        &World::running_at_slot(1).with_account(encrypted_value_account.account_key, impostor),
        encrypted_value_account.account_key,
    )
    .expect_err("a foreign program's account is not an encrypted value account");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::ForeignOwner { owner, .. } if owner == [0xee; 32]
    ));
    assert_eq!(
        AuthorizationFailure::EncryptedValueAccount {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

/// A host-owned account of a different type is caught by the discriminator rather than by whatever
/// its bytes happen to mean when read as an encrypted value account. Here the substitute is a real
/// delegation record — the account type most likely to be confused with an encrypted value account,
/// since both are written by the same program and both hold identities in their first bytes.
#[test]
fn a_host_owned_account_of_another_type_is_rejected() {
    let signer = Wallet::new(1);
    let delegator = Wallet::new(2);
    let encrypted_value_account =
        EncryptedValueAccountFixture::allowing(handle(0x13, FHE_TYPE_UINT64), signer.pubkey());
    let delegation = DelegationFixture::live(delegator.pubkey(), signer.pubkey(), 100);

    let failure = resolve_from(
        &World::running_at_slot(1)
            .with_account(encrypted_value_account.account_key, delegation.account()),
        encrypted_value_account.account_key,
    )
    .expect_err("delegation-record bytes are not an encrypted value account");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::WrongAccountType { account_key } if account_key == encrypted_value_account.account_key
    ));
}

/// The address binding is the backstop of the chain: the account's own fields must derive the
/// address it was read at. Without it, a bug anywhere in account selection would let a request
/// read the authority, the application and the peaks out of somebody else's encrypted value
/// account by naming its own address.
#[test]
fn an_encrypted_value_account_whose_fields_derive_another_address_is_rejected() {
    let owner = Wallet::new(1).pubkey();
    let claimed = EncryptedValueAccountFixture::allowing(handle(0x14, FHE_TYPE_UINT64), owner);
    // An encrypted value account of another authority, placed at the claimed account's address.
    let mut foreign = EncryptedValueAccountFixture::in_application(
        APP_PROGRAM,
        [0x33; 32],
        SCOPE,
        LABEL,
        handle(0x14, FHE_TYPE_UINT64),
    );
    foreign.allow(owner);
    assert_ne!(foreign.account_key, claimed.account_key);

    let failure = resolve_from(
        &World::running_at_slot(1).with_account(claimed.account_key, foreign.account()),
        claimed.account_key,
    )
    .expect_err("an encrypted value account must live where its fields say");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::AddressMismatch { account_key, derived: Some(derived) }
            if account_key == claimed.account_key && derived == foreign.account_key
    ));
    assert_eq!(
        AuthorizationFailure::EncryptedValueAccount {
            index: 0,
            source: failure
        }
        .class(),
        FailureClass::Terminal
    );
}

/// The stored bump is part of the derivation. An account carrying another bump either derives
/// another address or no address at all, and both are the same rejection.
#[test]
fn an_encrypted_value_account_with_an_altered_bump_is_rejected() {
    let owner = Wallet::new(1).pubkey();
    let mut altered = EncryptedValueAccountFixture::allowing(handle(0x15, FHE_TYPE_UINT64), owner);
    let account_key = altered.account_key;
    altered.encrypted_value.bump = altered.encrypted_value.bump.wrapping_sub(1);

    let failure = resolve_from(
        &World::running_at_slot(1).with_account(account_key, altered.account()),
        account_key,
    )
    .expect_err("the bump is part of the address");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::AddressMismatch { account_key: key, derived }
            if key == account_key && derived != Some(account_key)
    ));
}

/// Trailing bytes are legal. An encrypted value account is realloc-grown to its high-water mark and
/// never shrunk, so the tail is the normal state of any account whose MMR has held more peaks
/// than it holds now.
#[test]
fn trailing_bytes_after_the_encrypted_value_account_body_are_accepted() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x16, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );
    let mut grown = encrypted_value_account.account();
    let body_len = grown.data.len();
    grown.data.extend_from_slice(&[0; 96]);

    let resolved = resolve_from(
        &World::running_at_slot(1).with_account(encrypted_value_account.account_key, grown),
        encrypted_value_account.account_key,
    )
    .expect("a realloc-grown account resolves");

    assert_eq!(resolved.encrypted_value_account_authority(), AUTHORITY);
    assert_eq!(
        8 + borsh::to_vec(resolved.encrypted_value())
            .expect("the encrypted value account serializes")
            .len(),
        body_len,
        "the decoded body ends where the account ended before the tail was appended, so the \
         accept really did ignore 96 surplus bytes"
    );
}

/// A body cut short is not the same thing as a body followed by extra bytes: the first is an
/// encrypted value account that cannot be read, the second is an encrypted value account with room
/// to spare.
#[test]
fn an_encrypted_value_account_with_a_truncated_body_is_rejected() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x17, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );
    let full = encrypted_value_account.account();
    let truncated = SnapshotAccount {
        owner: PROGRAM_ID,
        data: full.data[..full.data.len() - 8].to_vec(),
    };

    let failure = resolve_from(
        &World::running_at_slot(1).with_account(encrypted_value_account.account_key, truncated),
        encrypted_value_account.account_key,
    )
    .expect_err("a body that does not decode is not an encrypted value account");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::Malformed { .. }
    ));
}

/// An account holding only a discriminator is host-owned and of the right type, and still has no
/// encrypted value account in it. The type check and the decode are two checks because an account
/// can pass the first and fail the second.
#[test]
fn an_encrypted_value_account_holding_only_its_discriminator_is_rejected() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x18, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );
    let empty = SnapshotAccount {
        owner: PROGRAM_ID,
        data: encrypted_value_discriminator().to_vec(),
    };

    let failure = resolve_from(
        &World::running_at_slot(1).with_account(encrypted_value_account.account_key, empty),
        encrypted_value_account.account_key,
    )
    .expect_err("a discriminator alone is not an encrypted value account");

    assert!(matches!(
        failure,
        EncryptedValueAccountFailure::Malformed { .. }
    ));
}

// ---------------------------------------------------------------------------
// Authority and scope
// ---------------------------------------------------------------------------

/// Each entry's authority comes from its own encrypted value account. Two entries of the same
/// application and different authorities resolve to their own — there is no request-level
/// authority to share, and no first-entry value to inherit.
#[test]
fn each_entry_takes_its_authority_from_its_own_encrypted_value_account() {
    let first = EncryptedValueAccountFixture::in_application(
        APP_PROGRAM,
        [0x51; 32],
        SCOPE,
        LABEL,
        handle(0x19, FHE_TYPE_UINT64),
    );
    let second = EncryptedValueAccountFixture::in_application(
        APP_PROGRAM,
        [0x52; 32],
        SCOPE,
        LABEL,
        handle(0x1a, FHE_TYPE_UINT64),
    );

    assert_eq!(
        resolved(&first).encrypted_value_account_authority(),
        [0x51; 32]
    );
    assert_eq!(
        resolved(&second).encrypted_value_account_authority(),
        [0x52; 32]
    );
    assert_eq!(resolved(&first).program(), APP_PROGRAM);
    assert_eq!(resolved(&first).scope(), SCOPE);
}

/// A scoped permit admits the `(program, scope)` pairs it signed.
#[test]
fn a_scoped_permit_admits_an_encrypted_value_account_of_a_signed_application() {
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(
        handle(0x1b, FHE_TYPE_UINT64),
        Wallet::new(1).pubkey(),
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).scope(&[(APP_PROGRAM, SCOPE)]);

    check_scope(
        permit.typed().allowed_scopes(),
        &resolved(&encrypted_value_account),
    )
    .expect("a signed application is in scope");
}

/// A scope outside the signed set is rejected, and the pair that gets tested is the encrypted
/// value account's — the only place it exists.
#[test]
fn an_encrypted_value_account_outside_the_signed_scope_is_rejected() {
    let foreign_scope: SolanaPubkeyBytes = [0x61; 32];
    let encrypted_value_account = EncryptedValueAccountFixture::in_application(
        APP_PROGRAM,
        AUTHORITY,
        foreign_scope,
        LABEL,
        handle(0x1c, FHE_TYPE_UINT64),
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).scope(&[(APP_PROGRAM, SCOPE)]);

    let failure = check_scope(
        permit.typed().allowed_scopes(),
        &resolved(&encrypted_value_account),
    )
    .expect_err("an unsigned scope is out of scope");

    assert!(matches!(
        failure,
        ScopeFailure::ScopeNotAllowed { program, scope }
            if program == APP_PROGRAM && scope == foreign_scope
    ));
}

/// The scope is only meaningful as a pair. The same scope bytes under another program are another
/// application: a program cannot borrow a scope somebody signed for a different program.
#[test]
fn the_same_scope_under_another_program_is_rejected() {
    let other_program: SolanaPubkeyBytes = [0x62; 32];
    let encrypted_value_account = EncryptedValueAccountFixture::in_application(
        other_program,
        AUTHORITY,
        SCOPE,
        LABEL,
        handle(0x1d, FHE_TYPE_UINT64),
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).scope(&[(APP_PROGRAM, SCOPE)]);

    let failure = check_scope(
        permit.typed().allowed_scopes(),
        &resolved(&encrypted_value_account),
    )
    .expect_err("the pair is the identity, not the scope alone");

    assert!(matches!(
        failure,
        ScopeFailure::ScopeNotAllowed { program, scope }
            if program == other_program && scope == SCOPE
    ));
}

/// An empty signed list is permissive and the rule is skipped, which is parity with the EVM
/// path rather than an optimization.
#[test]
fn a_permissive_permit_admits_an_encrypted_value_account_of_any_application() {
    let encrypted_value_account = EncryptedValueAccountFixture::in_application(
        [0x71; 32],
        AUTHORITY,
        [0x72; 32],
        LABEL,
        handle(0x1e, FHE_TYPE_UINT64),
    );
    let permit = PermitBuilder::new(Wallet::new(1).pubkey()).permissive();

    assert!(
        permit.typed().allowed_scopes().is_permissive(),
        "the fixture really is permissive"
    );
    check_scope(
        permit.typed().allowed_scopes(),
        &resolved(&encrypted_value_account),
    )
    .expect("permissive skips the scope rule");
}

/// Scope is tested per handle, so a foreign-application handle mixed into a batch fails the whole
/// request. Checking the first entry only would let a narrowly scoped permit decrypt whatever
/// was appended after it, and there is no partial release: a request is authorized entirely or
/// not at all. The rejection lands before the proof read: a request outside its signed scope
/// costs no round trip to the record.
#[tokio::test]
async fn a_foreign_application_handle_later_in_the_batch_rejects_the_whole_request() {
    let wallet = Wallet::new(1);
    let in_scope_handle = handle(0x1f, FHE_TYPE_UINT64);
    let out_of_scope_handle = handle(0x20, FHE_TYPE_UINT64);
    let in_scope = EncryptedValueAccountFixture::allowing(in_scope_handle, wallet.pubkey());
    let mut out_of_scope = EncryptedValueAccountFixture::in_application(
        [0x81; 32],
        AUTHORITY,
        SCOPE,
        LABEL,
        out_of_scope_handle,
    );
    out_of_scope.allow(wallet.pubkey());
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).scope(&[(APP_PROGRAM, SCOPE)]))
        .direct(&in_scope, in_scope_handle)
        .direct(&out_of_scope, out_of_scope_handle)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&in_scope)
        .with_encrypted_value_account(&out_of_scope)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let proofs = ScriptedProofReader::unreachable();
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        &proofs,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("one out-of-scope entry rejects the request");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::Scope {
                index: 1,
                source: ScopeFailure::ScopeNotAllowed { .. }
            }
        ),
        "the rejection names the offending entry, got {failure}"
    );
    assert_eq!(failure.class(), FailureClass::Terminal);
    assert_eq!(
        proofs.call_count(),
        0,
        "scope is decided before the record is asked"
    );
}

/// Permissive widens the scope rule and nothing else. The allow leaf is unconditional, so a
/// permissive permit gets a key that was never allowed exactly as far as a scoped one does.
#[tokio::test]
async fn a_permissive_permit_does_not_widen_the_allow_leaf() {
    let wallet = Wallet::new(1);
    let stranger = Wallet::new(9);
    let live = handle(0x21, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::allowing(live, stranger.pubkey());
    let request = RequestBuilder::new(&wallet)
        .permit(PermitBuilder::new(wallet.pubkey()).permissive())
        .direct(&encrypted_value_account, live)
        .typed();
    let world = World::running_at_slot(100)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(wallet.pubkey(), 0);
    let proofs = ScriptedProofReader::constant(world.record());
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let failure = authorize_request(
        &reader,
        &ServableKmsPair,
        &proofs,
        context(&deployment),
        &request,
    )
    .await
    .expect_err("permissive does not allow a key nobody allowed");

    assert!(
        matches!(
            failure,
            AuthorizationFailure::HandleBinding {
                index: 0,
                source: HandleBindingFailure::NoLeaf { .. }
            }
        ),
        "expected a missing allow leaf, got {failure}"
    );
}
