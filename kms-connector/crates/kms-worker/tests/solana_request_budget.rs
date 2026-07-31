//! The request handle list: what the Connector requires of it, and what it deliberately leaves
//! to the layer above.
//!
//! The system-wide decryption budget — the summed FHE-type bit widths of the requested handles —
//! is enforced on chain, by the Gateway entry point every request comes through: it sums the
//! widths per handle and reverts past the budget or on a handle whose type has no width. A
//! request that exists as an event has already passed both, and the EVM path in this Connector
//! does not re-adjudicate either. Neither does this one: a second copy of that table here could
//! only ever reject, terminally and after the fee was paid, a request the Gateway accepted.
//!
//! What the Connector does require is a non-empty list, and that is not a mirror of the Gateway's
//! rule but a precondition of its own output: a request with no entries would authorize nothing
//! and still be accepted.
//!
//! The property this file exists to pin is the one that survives the removal — the list is passed
//! through verbatim. Nothing here trims, deduplicates or reorders it, because the response binds
//! every occurrence at its position, and a client whose list was silently changed can no longer
//! reconstruct the binding of its own request.

mod solana_support;

use kms_worker::core::solana::{
    pipeline::{AuthorizationContext, authorize_request},
    request::{RequestFormError, SolanaUserDecryptRequest, SolanaUserDecryptRequestWire},
};
use solana_support::*;

/// A handle distinguished by an index rather than a repeated byte, so a long list is a list of
/// different handles.
fn distinct_handle(index: u16, fhe_type: u8) -> [u8; 32] {
    let mut bytes = handle(0x10, fhe_type);
    bytes[0..2].copy_from_slice(&index.to_be_bytes());
    bytes
}

/// A request naming the given handles, each against its own lineage.
fn request_naming(wallet: &Wallet, handles: &[[u8; 32]]) -> SolanaUserDecryptRequestWire {
    let mut builder = RequestBuilder::new(wallet);
    for (index, handle) in handles.iter().enumerate() {
        let mut label = LABEL;
        label[0..2].copy_from_slice(&(index as u16).to_be_bytes());
        let lineage = LineageFixture::in_domain(DOMAIN, APP, label, *handle, &[wallet.pubkey()]);
        builder = builder.direct_current(&lineage, *handle);
    }
    builder.wire()
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
// The list is passed through verbatim
// ---------------------------------------------------------------------------

/// A long list of distinct handles decodes as itself. The count is deliberately larger than the
/// on-chain budget admits for this type: this Connector has no opinion about the size, so the
/// test states that absence rather than a limit.
#[test]
fn a_request_larger_than_the_on_chain_budget_still_decodes_here() {
    let wallet = Wallet::new(1);
    // Well past the budget at this width, and the point is that nothing here notices.
    let handles: Vec<[u8; 32]> = (0..64)
        .map(|index| distinct_handle(index, FHE_TYPE_UINT64))
        .collect();
    let built = request_naming(&wallet, &handles);

    let request = SolanaUserDecryptRequest::decode(&built)
        .expect("the request budget is enforced on chain, before the request exists");

    assert_eq!(
        request.handles().len(),
        handles.len(),
        "the list is neither trimmed nor rejected for its size"
    );
}

/// The list is passed through in order, entry for entry. A layer that reordered or deduplicated it
/// would break the response binding, which is positional.
#[test]
fn the_handle_list_survives_decoding_in_order() {
    let wallet = Wallet::new(1);
    let handles: Vec<[u8; 32]> = (0..4)
        .map(|index| distinct_handle(index, FHE_TYPE_UINT64))
        .collect();

    let request =
        SolanaUserDecryptRequest::decode(&request_naming(&wallet, &handles)).expect("well formed");

    let decoded: Vec<[u8; 32]> = request
        .handles()
        .iter()
        .map(|entry| entry.handle())
        .collect();
    assert_eq!(decoded, handles);
}

/// A handle of a type this system does not support is not this Connector's rejection to make. The
/// Gateway reverts on it before the request exists; re-deciding it here would make the Connector
/// the authority on which ciphertext types exist, which it is not on the EVM path either.
#[test]
fn a_handle_of_an_exotic_type_is_not_refused_here() {
    let wallet = Wallet::new(1);
    let exotic_type = 200;
    let exotic = distinct_handle(0, exotic_type);

    SolanaUserDecryptRequest::decode(&request_naming(&wallet, &[exotic]))
        .expect("type support is decided upstream, not in the authorization path");
}

/// An empty list is rejected: there is nothing to authorize and nothing for a response to bind.
/// This is the Connector's own precondition, and it is the one size-shaped rule that stays.
#[test]
fn an_empty_handle_list_is_rejected() {
    let wallet = Wallet::new(1);
    let wire = RequestBuilder::new(&wallet).wire();

    let failure =
        SolanaUserDecryptRequest::decode(&wire).expect_err("a request must name a handle");

    assert!(matches!(failure, RequestFormError::EmptyHandles));
}

// ---------------------------------------------------------------------------
// Duplicates
// ---------------------------------------------------------------------------

/// A duplicate handle is legal, in parity with the EVM path, where the Gateway performs no
/// deduplication. Both occurrences survive decoding.
#[test]
fn a_duplicate_handle_is_legal() {
    let wallet = Wallet::new(1);
    let repeated = handle(0x20, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(repeated, &[wallet.pubkey()]);
    let wire = RequestBuilder::new(&wallet)
        .direct_current(&lineage, repeated)
        .direct_current(&lineage, repeated)
        .wire();

    let request = SolanaUserDecryptRequest::decode(&wire).expect("duplicates are legal");

    assert_eq!(
        request.handles().len(),
        2,
        "both occurrences survive decoding"
    );
}

/// Both occurrences are authorized, and both appear in the accepted entry set — in order, so the
/// response can bind each at its position.
#[tokio::test]
async fn both_occurrences_of_a_duplicate_handle_are_authorized() {
    let wallet = Wallet::new(1);
    let repeated = handle(0x21, FHE_TYPE_UINT64);
    let lineage = LineageFixture::new(repeated, &[wallet.pubkey()]);
    let request = RequestBuilder::new(&wallet)
        .direct_current(&lineage, repeated)
        .direct_current(&lineage, repeated)
        .typed();
    let world = World::at_slot(100)
        .with_lineage(&lineage)
        .with_watermark(wallet.pubkey(), 0);
    let reader = ScriptedReader::constant(world);
    let deployment = deployment();

    let authorized = authorize_request(&reader, &ServableKmsPair, context(&deployment), &request)
        .await
        .expect("a duplicate of an authorized handle is authorized");

    assert_eq!(authorized.entries().len(), 2);
    assert_eq!(authorized.entries()[0].handle, repeated);
    assert_eq!(authorized.entries()[1].handle, repeated);
}
