//! The gateway's typed ACL-scope declaration: admitted only when it equals the signed list's
//! actual length.
//!
//! The gateway enforces its pre-fee scope bound (the EVM paths' `allowedContracts` rule) on the
//! event's typed `allowedAclDomainKeyCount` alone — it never reads the opaque `hostPayload`.
//! This suite pins the equality that turns that bound into a bound on the list the permit
//! actually signs, and that a lying declaration is refused before authorization spends a single
//! account read.

mod solana_support;

use kms_worker::core::solana::{
    failure::{AuthorizationFailure, FailureClass},
    pipeline::{AuthorizationContext, authorize_request},
    request::SolanaUserDecryptRequest,
};
use solana_support::*;

const OBSERVED_SLOT: u64 = 500;

/// Authorizes a request under a given declaration, returning the outcome and the reads it cost.
async fn authorize_with_declared(
    declared: u8,
    world: World,
    request: &SolanaUserDecryptRequest,
) -> (
    Result<
        kms_worker::core::solana::pipeline::AuthorizedRequest,
        AuthorizationFailure,
    >,
    usize,
) {
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();
    let context = AuthorizationContext {
        deployment: &deployment,
        now_unix_seconds: NOW_INSIDE_WINDOW,
        declared_acl_domain_key_count: declared,
    };
    let outcome = authorize_request(&reader, &ServableKmsPair, context, request).await;
    (outcome, reader.call_count())
}

fn scenario() -> (Wallet, EncryptedValueAccountFixture, [u8; 32]) {
    let signer = Wallet::new(1);
    let live = handle(0x10, FHE_TYPE_UINT64);
    let encrypted_value_account = EncryptedValueAccountFixture::new(live, &[signer.pubkey()]);
    (signer, encrypted_value_account, live)
}

/// The honest declaration authorizes: this rule adds nothing to an honest request.
#[tokio::test]
async fn the_honest_declaration_authorizes() {
    let (signer, encrypted_value_account, live) = scenario();
    let request = RequestBuilder::new(&signer)
        .direct_current(&encrypted_value_account, live)
        .typed();
    let world = World::at_slot(OBSERVED_SLOT)
        .with_encrypted_value_account(&encrypted_value_account)
        .with_watermark(signer.pubkey(), 0);

    let (outcome, _) =
        authorize_with_declared(declared_acl_domain_key_count(&request), world, &request).await;

    outcome.expect("an honest declaration changes nothing");
}

/// A declaration that is not the signed list's length is refused as itself — under- and
/// over-declaration alike — and before a single account read: the gateway's bound must bind the
/// signed list, not whatever the caller typed next to it.
#[tokio::test]
async fn a_lying_declaration_is_refused_before_any_read() {
    let (signer, encrypted_value_account, live) = scenario();
    // The fixture permit signs exactly one ACL domain key.
    let request = RequestBuilder::new(&signer)
        .direct_current(&encrypted_value_account, live)
        .typed();
    let actual = usize::from(declared_acl_domain_key_count(&request));

    for lying in [0u8, 2, 10] {
        assert_ne!(usize::from(lying), actual, "the lie must actually lie");
        let world = World::at_slot(OBSERVED_SLOT)
            .with_encrypted_value_account(&encrypted_value_account)
            .with_watermark(signer.pubkey(), 0);

        let (outcome, reads) = authorize_with_declared(lying, world, &request).await;

        let failure = outcome.expect_err("a lying declaration is refused");
        assert_eq!(
            failure,
            AuthorizationFailure::AclDomainKeyCountMismatch {
                declared: lying,
                actual,
            }
        );
        assert_eq!(reads, 0, "the lie is refused before any account read");
    }
}

/// The mismatch is terminal: no observation changes either side of the equality — the client's
/// move is a different request.
#[test]
fn the_mismatch_is_terminal() {
    assert_eq!(
        AuthorizationFailure::AclDomainKeyCountMismatch {
            declared: 3,
            actual: 1,
        }
        .class(),
        FailureClass::Terminal
    );
}
