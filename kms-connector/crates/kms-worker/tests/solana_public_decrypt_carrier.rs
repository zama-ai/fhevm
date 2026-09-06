//! The public-decrypt path, pinned from outside the module that hosts it.
//!
//! Solana public decrypt has no live on-chain "is public" flag: public-ness is a `PublicDecryptLeaf`
//! sealed in the encrypted value account's MMR, and the account holds only the peaks. The request
//! supplies one thing — which account the handle lives in, carried in the version-`0x03`
//! `extraData` — and the connector does the rest: it reads the account at `confirmed`, asks the
//! coprocessors' leaf record for the leaf, and verifies the sibling path against the peaks it
//! observed. Nothing the requester hands in is a proof, and nothing the record says is trusted
//! without verifying.
//!
//! These assertions run the public entry point over two real HTTP round-trips — a mock
//! `getMultipleAccounts` endpoint serving the account bytes and a mock leaf-proof route serving
//! the record's answer — so the transport, the carrier and the rule are exercised together. Wire
//! bytes are pinned as literals, not as imports of the constants that produce them: if the carrier
//! version is renumbered, or the leaf-proof route moves, these tests fail by construction rather
//! than following the rename.

mod solana_support;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use kms_worker::core::event_processor::{
    ProcessingErrorKind,
    solana_public_decrypt::{SolanaHost, check_solana_handles_public_decrypt},
};
use kms_worker::core::solana::proof::{HttpHostProofReader, LeafProofOutcome, LeafQuery};
use kms_worker::core::solana::snapshot::{
    RpcHostStateReader, SnapshotKeys, multiple_accounts_request_body,
};
use mocktail::server::MockServer;
use solana_pubkey::Pubkey;
use solana_support::{EncryptedValueAccountFixture, deployment, handle};

/// The `extraData` version byte of the public-decrypt carrier. A literal, deliberately not the
/// production constant.
const CARRIER_VERSION: u8 = 0x03;

/// The coprocessor route the connector reads leaf proofs from. A literal, for the same reason.
const LEAF_PROOFS_ROUTE: &str = "/v1/solana/leaf-proofs";

/// The bearer key the mock coprocessor is configured with.
const API_KEY: &str = "test-key";

/// The FHE type byte of the fixture handles: any type will do, public-ness is per handle.
const FHE_TYPE_UINT64: u8 = 5;

/// The carrier as the client builds it: version, context id, the handle's encrypted value account.
fn carrier(fixture: &EncryptedValueAccountFixture) -> Vec<u8> {
    let mut blob = vec![CARRIER_VERSION];
    blob.extend_from_slice(&[0x11; 32]);
    blob.extend_from_slice(&fixture.account_key);
    blob
}

/// An account whose current handle was made public, then replaced: the public leaf survives
/// the update because it names the handle, not the slot the handle occupied.
fn public_then_updated(public: [u8; 32], replacement: [u8; 32]) -> EncryptedValueAccountFixture {
    let mut fixture = EncryptedValueAccountFixture::new(public);
    fixture.mark_public();
    fixture.update(replacement);
    fixture
}

/// What the record answers for one query, as the coprocessor route serializes it.
fn wire_outcome(outcome: &LeafProofOutcome) -> serde_json::Value {
    match outcome {
        LeafProofOutcome::Found {
            leaf_index,
            leaf_count,
            siblings,
        } => serde_json::json!({
            "status": "found",
            "leafIndex": leaf_index,
            "leafCount": leaf_count,
            "siblings": siblings.iter().map(alloy::hex::encode).collect::<Vec<_>>(),
        }),
        LeafProofOutcome::NotFound { leaf_count } => {
            serde_json::json!({ "status": "notFound", "leafCount": leaf_count })
        }
        LeafProofOutcome::UnknownAccount => serde_json::json!({ "status": "unknownAccount" }),
        LeafProofOutcome::HistoryIncomplete => {
            serde_json::json!({ "status": "historyIncomplete" })
        }
    }
}

/// A host whose readers answer nothing: for requests that must be refused before any read. If the
/// path under test unexpectedly reaches the network, the read fails and the assertions on the
/// refusal shape catch it.
async fn host_without_state() -> (MockServer, MockServer, SolanaHost) {
    let rpc = MockServer::new_http("solana-rpc-empty");
    rpc.start().await.expect("the mock RPC starts");
    let coprocessor = MockServer::new_http("coprocessor-empty");
    coprocessor
        .start()
        .await
        .expect("the mock coprocessor starts");
    let host = host_bound_to(&rpc, &coprocessor);
    (rpc, coprocessor, host)
}

fn host_bound_to(rpc: &MockServer, coprocessor: &MockServer) -> SolanaHost {
    let client = reqwest::Client::new();
    SolanaHost {
        deployment: deployment(),
        reader: RpcHostStateReader::new(
            rpc.base_url().expect("the mock RPC has a URL").clone(),
            client.clone(),
        ),
        proofs: HttpHostProofReader::new(
            &[coprocessor
                .base_url()
                .expect("the mock coprocessor has a URL")
                .clone()],
            API_KEY.to_owned(),
            client,
        ),
    }
}

/// Starts a mock Solana RPC serving exactly one account, matched on the byte-exact
/// `getMultipleAccounts` request the reader builds (which pins the key, base64 encoding and
/// confirmed commitment), and a mock coprocessor answering `query` with `outcome` exactly,
/// matched on the byte-exact leaf-proof request. Returns both servers (kept alive by the caller)
/// and a host wired to them.
async fn host_answering(
    fixture: &EncryptedValueAccountFixture,
    query: LeafQuery,
    outcome: LeafProofOutcome,
) -> (MockServer, MockServer, SolanaHost) {
    let account = fixture.account();
    let rpc_response = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "context": { "slot": 1 },
            "value": [{
                "owner": Pubkey::new_from_array(account.owner).to_string(),
                "data": [BASE64_STANDARD.encode(&account.data), "base64"],
                "lamports": 1,
                "executable": false,
                "rentEpoch": 0,
            }],
        },
    });
    let mut rpc = MockServer::new_http("solana-rpc");
    let rpc_request = multiple_accounts_request_body(&SnapshotKeys::new([fixture.account_key]));
    rpc.mock(move |when, then| {
        when.post().json(rpc_request.clone());
        then.json(rpc_response.clone());
    });
    rpc.start().await.expect("the mock RPC starts");

    let proof_request =
        kms_worker::core::solana::proof::leaf_proof_request_body(std::slice::from_ref(&query));
    let proof_response = serde_json::json!({ "proofs": [wire_outcome(&outcome)] });
    let mut coprocessor = MockServer::new_http("coprocessor-leaf-proofs");
    coprocessor.mock(move |when, then| {
        when.post()
            .path(LEAF_PROOFS_ROUTE)
            .json(proof_request.clone());
        then.json(proof_response.clone());
    });
    coprocessor
        .start()
        .await
        .expect("the mock coprocessor starts");

    let host = host_bound_to(&rpc, &coprocessor);
    (rpc, coprocessor, host)
}

fn irrecoverable_containing(err: kms_worker::core::event_processor::ProcessingError, needle: &str) {
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Irrecoverable,
        "a wrong request is terminal, got: {err}"
    );
    assert!(err.source.to_string().contains(needle), "got: {err}");
}

#[tokio::test]
async fn a_public_leaf_the_record_serves_authorizes_the_handle() {
    let public = handle(0x20, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x21, FHE_TYPE_UINT64));
    let query = fixture.public_query(public);
    let (_rpc, _coprocessor, host) = host_answering(&fixture, query, fixture.outcome(&query)).await;

    check_solana_handles_public_decrypt(&host, &[public], &carrier(&fixture))
        .await
        .expect("a public leaf proven against the account's own peaks authorizes");
}

/// The record's answer is verified, not believed: a well-formed proof of another leaf — here the
/// allow leaf a key holds on the same handle — does not make the handle public.
#[tokio::test]
async fn an_allow_leaf_does_not_prove_public_ness() {
    let allowed = handle(0x30, FHE_TYPE_UINT64);
    let mut fixture = EncryptedValueAccountFixture::allowing(allowed, [0x42; 32]);
    fixture.update(handle(0x31, FHE_TYPE_UINT64));
    let allow_query = fixture.allowed_query(allowed, [0x42; 32]);
    // The record answers the public query with the allow leaf's proof.
    let (_rpc, _coprocessor, host) = host_answering(
        &fixture,
        fixture.public_query(allowed),
        fixture.outcome(&allow_query),
    )
    .await;

    let err = check_solana_handles_public_decrypt(&host, &[allowed], &carrier(&fixture))
        .await
        .expect_err("an allow leaf must not prove public-ness");
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Recoverable,
        "a proof that does not verify is a disagreement to retry, got: {err}"
    );
}

/// No public leaf in a record that has sealed as much history as the chain shows: the handle was
/// never made public, and no retry changes that.
#[tokio::test]
async fn a_handle_never_made_public_is_refused_terminally() {
    let private = handle(0x40, FHE_TYPE_UINT64);
    let fixture = EncryptedValueAccountFixture::allowing(private, [0x42; 32]);
    let query = fixture.public_query(private);
    let (_rpc, _coprocessor, host) = host_answering(&fixture, query, fixture.outcome(&query)).await;

    let err = check_solana_handles_public_decrypt(&host, &[private], &carrier(&fixture))
        .await
        .expect_err("a handle nobody made public is not public");
    irrecoverable_containing(err, "no leaf");
}

/// A record behind the chain says nothing yet: the read is retried once against the same mock,
/// and then the request is left to the ordinary attempt budget.
#[tokio::test]
async fn a_record_behind_the_chain_is_retried_not_refused() {
    let public = handle(0x50, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x51, FHE_TYPE_UINT64));
    let query = fixture.public_query(public);
    let (_rpc, _coprocessor, host) = host_answering(
        &fixture,
        query,
        LeafProofOutcome::NotFound { leaf_count: 0 },
    )
    .await;

    let err = check_solana_handles_public_decrypt(&host, &[public], &carrier(&fixture))
        .await
        .expect_err("a record that has not sealed the leaf yet authorizes nothing yet");
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Recoverable,
        "a record behind the chain is retried, got: {err}"
    );
}

#[test]
fn the_carrier_version_is_pinned_by_literal() {
    // If this fails, the carrier's version byte changed while the public-decrypt path still
    // depends on it.
    let blob = connector_utils::types::solana_extra_data::encode_solana_public_decrypt_extra_data(
        [0u8; 32], [9u8; 32],
    );
    assert_eq!(blob[0], CARRIER_VERSION);
    assert_eq!(
        blob.len(),
        65,
        "version, context id, account — and nothing else"
    );
}

/// A carrier of another version, a carrier of another length, and no carrier at all: each is
/// refused explicitly before reading any account, and never parsed under the wrong layout.
#[tokio::test]
async fn a_malformed_carrier_refuses_before_any_read() {
    let fixture = public_then_updated(handle(0x60, FHE_TYPE_UINT64), handle(0x61, FHE_TYPE_UINT64));
    let (_rpc, _coprocessor, host) = host_without_state().await;
    let valid = carrier(&fixture);

    let mut other_version = valid.clone();
    other_version[0] = 0x09;
    let mut context_only = vec![0x01];
    context_only.extend_from_slice(&[0x11; 32]);
    let mut trailing = valid.clone();
    trailing.push(0);
    let truncated = valid[..valid.len() - 1].to_vec();

    for blob in [Vec::new(), other_version, context_only, trailing, truncated] {
        let err =
            check_solana_handles_public_decrypt(&host, &[handle(0x60, FHE_TYPE_UINT64)], &blob)
                .await
                .expect_err("a carrier that is not the version-3 layout names no account");
        irrecoverable_containing(err, "requires the version-3 extraData");
    }
}

#[tokio::test]
async fn public_decrypt_authorizes_one_handle_per_request() {
    let fixture = public_then_updated(handle(0x70, FHE_TYPE_UINT64), handle(0x71, FHE_TYPE_UINT64));
    let (_rpc, _coprocessor, host) = host_without_state().await;

    let err = check_solana_handles_public_decrypt(
        &host,
        &[handle(0x70, FHE_TYPE_UINT64), handle(0x71, FHE_TYPE_UINT64)],
        &carrier(&fixture),
    )
    .await
    .expect_err("a public decrypt names exactly one handle");
    irrecoverable_containing(err, "exactly one handle");
}

/// The carrier names the account; the account still has to be the host program's. A foreign
/// account at that address proves nothing, however well-formed.
#[tokio::test]
async fn a_carrier_naming_a_foreign_account_is_refused() {
    let public = handle(0x80, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x81, FHE_TYPE_UINT64));
    let mut impostor = fixture.account();
    impostor.owner = [0xee; 32];
    let rpc_response = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "context": { "slot": 1 },
            "value": [{
                "owner": Pubkey::new_from_array(impostor.owner).to_string(),
                "data": [BASE64_STANDARD.encode(&impostor.data), "base64"],
                "lamports": 1,
                "executable": false,
                "rentEpoch": 0,
            }],
        },
    });
    let mut rpc = MockServer::new_http("solana-rpc");
    let rpc_request = multiple_accounts_request_body(&SnapshotKeys::new([fixture.account_key]));
    rpc.mock(move |when, then| {
        when.post().json(rpc_request.clone());
        then.json(rpc_response.clone());
    });
    rpc.start().await.expect("the mock RPC starts");
    let coprocessor = MockServer::new_http("coprocessor-unreached");
    coprocessor
        .start()
        .await
        .expect("the mock coprocessor starts");
    let host = host_bound_to(&rpc, &coprocessor);

    let err = check_solana_handles_public_decrypt(&host, &[public], &carrier(&fixture))
        .await
        .expect_err("a foreign program's account is not an encrypted value account");
    irrecoverable_containing(err, "is owned by");
}
