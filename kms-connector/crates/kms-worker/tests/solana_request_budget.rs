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
//! What the Connector does require of the list's size is a non-empty lower bound and a
//! snapshot-shaped upper bound, and neither is a mirror of the Gateway's bit budget. The empty
//! list is a precondition of the module's own output: a request with no entries would authorize
//! nothing and still be accepted. The handle cap is a precondition of its only observation:
//! every rule is evaluated against one atomic `getMultipleAccounts` snapshot, and a standard RPC
//! node serves at most 100 accounts per call — three per entry plus the signer's invalidation
//! record in the worst case. The Gateway enforces the same cap at admission, before the fee, so
//! the Connector's arm is unreachable through it and exists to keep the invariant local.
//!
//! The property this file exists to pin is the one that survives the removal — the list is passed
//! through verbatim. Nothing here trims, deduplicates or reorders it, because the response binds
//! every occurrence at its position, and a client whose list was silently changed can no longer
//! reconstruct the binding of its own request.

mod solana_support;

use kms_worker::core::solana::{
    pipeline::{AuthorizationContext, authorize_request},
    request::{
        MAX_REQUEST_HANDLES, RequestFormError, SolanaUserDecryptRequest,
        SolanaUserDecryptRequestWire,
    },
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

/// A list at the handle cap decodes as itself, even though its summed bit width is past the
/// on-chain budget for this type (33 × 64 bits > 2048): the Connector holds no copy of the
/// bit-width table, and the test states that absence by decoding what the table would refuse.
#[test]
fn a_request_larger_than_the_on_chain_bit_budget_still_decodes_here() {
    let wallet = Wallet::new(1);
    let handles: Vec<[u8; 32]> = (0..MAX_REQUEST_HANDLES as u16)
        .map(|index| distinct_handle(index, FHE_TYPE_UINT64))
        .collect();
    let built = request_naming(&wallet, &handles);

    let request = SolanaUserDecryptRequest::decode(&built)
        .expect("the bit budget is enforced on chain, before the request exists");

    assert_eq!(
        request.handles().len(),
        handles.len(),
        "the list is neither trimmed nor rejected for its width"
    );
}

/// One entry past the cap is rejected at decode: a longer list could never be read in one
/// atomic snapshot, so no observation could ever authorize it. Terminal — the client's move is
/// to split the request.
#[test]
fn a_request_past_the_handle_cap_is_rejected() {
    let wallet = Wallet::new(1);
    let handles: Vec<[u8; 32]> = (0..(MAX_REQUEST_HANDLES as u16 + 1))
        .map(|index| distinct_handle(index, FHE_TYPE_UINT64))
        .collect();
    let built = request_naming(&wallet, &handles);

    let failure = SolanaUserDecryptRequest::decode(&built)
        .expect_err("a list past the cap cannot be read in one snapshot");

    assert!(matches!(
        failure,
        RequestFormError::TooManyHandles { handles } if handles == MAX_REQUEST_HANDLES + 1
    ));
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
/// This is the Connector's own precondition, the lower half of its two size-shaped rules.
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
