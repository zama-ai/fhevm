//! The public-decrypt proof carrier, pinned from outside the module that hosts it.
//!
//! Solana public decrypt has no live on-chain "is public" flag: public-ness is only provable by a
//! `PublicDecryptLeaf` MMR proof, and that proof travels in the version-`0x03` `extraData`
//! container. The container is temporary transport — it exists because the gateway interface has
//! no typed fields for a public-decrypt proof, and it is due for removal once such fields exist —
//! but while it exists, it is the only thing standing between a Solana public decrypt and a silent
//! fail-closed outage.
//!
//! These assertions were written from the outside on purpose. They used to hold inside the v0
//! user-decrypt module's own test module, which was deleted with that path — and a test deleted
//! together with the code it guarded guards nothing. The public-decrypt surface was re-homed to
//! `event_processor::solana_public_decrypt` instead, so the same properties are pinned here,
//! through the public entry point and a real RPC round-trip:
//! a mock `getAccountInfo` endpoint serves the encrypted value account bytes, and
//! [`check_solana_handles_public_decrypt`] does everything else it does in production.
//!
//! Wire bytes are pinned as literals, not as imports of the constants that produce them: if the
//! carrier version or the proof mode byte is renumbered, or the carrier is torn down without the
//! public-decrypt path being re-homed first, these tests fail by construction rather than
//! following the rename.

mod solana_support;

use base64::{Engine, engine::general_purpose::STANDARD as BASE64_STANDARD};
use connector_utils::types::solana_extra_data::{
    encode_solana_extra_data_context_only, encode_solana_extra_data_mmr_proof,
};
use kms_worker::core::event_processor::{
    ProcessingErrorKind,
    solana_public_decrypt::{SolanaHost, check_solana_handles_public_decrypt},
};
use kms_worker::core::solana::snapshot::RpcHostStateReader;
use kms_worker::core::solana_v2_fetcher::SolanaV2Fetcher;
use mocktail::server::MockServer;
use solana_pubkey::Pubkey;
use solana_support::{EncryptedValueAccountFixture, deployment};
use zama_solana_acl::MmrProof;

/// The `extraData` version byte that carries a public-decrypt proof. A literal, deliberately not
/// the production constant: this pin is what turns "the carrier was torn down without re-homing
/// public decrypt" into a readable test failure instead of a silent one.
const PROOF_CARRIER_VERSION: u8 = 0x03;

/// The mode byte of a public-decrypt proof blob, as the client writes it on the wire.
const PUBLIC_PROOF_MODE: u8 = 0x02;

/// The mode byte of a historical-access proof blob — the one mode the public path must refuse.
const HISTORICAL_PROOF_MODE: u8 = 0x01;

fn h(tag: u8) -> [u8; 32] {
    [tag; 32]
}

/// The full proof transport blob: 1-byte mode prefix ‖ Borsh(MmrProof).
fn proof_blob(mode: u8, proof: &MmrProof) -> Vec<u8> {
    let mut blob = vec![mode];
    blob.extend_from_slice(&borsh::to_vec(proof).expect("the fixture proof serializes"));
    blob
}

/// A carrier whose body is well-formed but whose version byte is not the proof-carrier version.
fn carrier_with_version(version: u8, valid_carrier: &[u8]) -> Vec<u8> {
    let mut blob = valid_carrier.to_vec();
    blob[0] = version;
    blob
}

/// Starts a mock Solana RPC serving exactly one account: the fixture's encrypted value account,
/// matched on the byte-exact `getAccountInfo` request the fetcher builds (which pins the account
/// key, base64 encoding, and confirmed commitment). Returns the server (kept alive by the caller)
/// and a host wired to it.
async fn host_serving(fixture: &EncryptedValueAccountFixture) -> (MockServer, SolanaHost) {
    let account = fixture.account();
    let response = serde_json::json!({
        "jsonrpc": "2.0",
        "id": 1,
        "result": {
            "context": { "slot": 1 },
            "value": {
                "owner": Pubkey::new_from_array(account.owner).to_string(),
                "data": [BASE64_STANDARD.encode(&account.data), "base64"],
                "lamports": 1,
                "executable": false,
                "rentEpoch": 0,
            },
        },
    });

    let mut server = MockServer::new_http("solana-rpc");
    let request_body = SolanaV2Fetcher::account_info_request_body(&fixture.account_key);
    server.mock(move |when, then| {
        when.post().json(request_body.clone());
        then.json(response.clone());
    });
    server.start().await.expect("the mock RPC starts");

    let host = host_bound_to(&server);
    (server, host)
}

/// A host whose RPC answers nothing: for requests that must be refused before any account is
/// read. If the path under test unexpectedly reaches the network, the fetch fails and the
/// assertions on the refusal shape catch it.
async fn host_without_accounts() -> (MockServer, SolanaHost) {
    let server = MockServer::new_http("solana-rpc-empty");
    server.start().await.expect("the mock RPC starts");
    let host = host_bound_to(&server);
    (server, host)
}

/// Builds a `SolanaHost` bound to a started mock RPC. Public decrypt only reads through the
/// single-account `fetcher`; the pipeline `reader` and `deployment` are wired for completeness.
fn host_bound_to(server: &MockServer) -> SolanaHost {
    let url = server.base_url().expect("the mock RPC has a URL").clone();
    SolanaHost {
        deployment: deployment(),
        reader: RpcHostStateReader::new(url.clone(), reqwest::Client::new()),
        fetcher: SolanaV2Fetcher::new(url, reqwest::Client::new()),
    }
}

/// An encrypted value account whose first leaf seals public-ness of `sealed`, after which the
/// value moved on to `successor` — the shape every exact-handle assertion needs.
fn public_then_updated(sealed: [u8; 32], successor: [u8; 32]) -> EncryptedValueAccountFixture {
    let mut fixture = EncryptedValueAccountFixture::new(sealed, &[[0x42; 32]]);
    fixture.mark_public();
    fixture.update(successor);
    fixture
}

fn public_carrier(fixture: &EncryptedValueAccountFixture, blob: Vec<u8>) -> Vec<u8> {
    encode_solana_extra_data_mmr_proof(
        [0u8; 32],
        fixture.encrypted_value_id(),
        fixture.encrypted_value.leaf_count,
        &blob,
    )
}

#[tokio::test]
async fn a_public_leaf_authorizes_exactly_its_sealed_handle() {
    let fixture = public_then_updated(h(20), h(21));
    let blob = proof_blob(PUBLIC_PROOF_MODE, &fixture.proof(0));
    let carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_serving(&fixture).await;

    check_solana_handles_public_decrypt(&host, &[h(20)], &carrier)
        .await
        .expect("a PublicDecryptLeaf sealed for this handle authorizes it");
}

#[tokio::test]
async fn a_public_leaf_does_not_authorize_the_handle_that_replaced_it() {
    let fixture = public_then_updated(h(20), h(21));
    let blob = proof_blob(PUBLIC_PROOF_MODE, &fixture.proof(0));
    let carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_serving(&fixture).await;

    let err = check_solana_handles_public_decrypt(&host, &[h(21)], &carrier)
        .await
        .expect_err("a PublicDecryptLeaf for the old handle must not authorize its successor");
    // The proof was built at the live leaf count, so the mismatch is classified as a view that
    // may still converge — retried, never granted.
    assert_eq!(err.kind, ProcessingErrorKind::Recoverable, "got: {err}");
}

#[tokio::test]
async fn a_historical_leaf_does_not_authorize_public_decrypt() {
    // No public leaf anywhere: the only sealed leaf is the historical-access one that `update`
    // wrote for the subject. Presenting it under the public mode byte must not verify.
    let mut fixture = EncryptedValueAccountFixture::new(h(30), &[[0x42; 32]]);
    fixture.update(h(31));
    let blob = proof_blob(PUBLIC_PROOF_MODE, &fixture.proof(0));
    let carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_serving(&fixture).await;

    let err = check_solana_handles_public_decrypt(&host, &[h(30)], &carrier)
        .await
        .expect_err("a historical-access leaf must not prove public-ness");
    assert_eq!(err.kind, ProcessingErrorKind::Recoverable, "got: {err}");
}

#[test]
fn the_proof_carrier_version_is_pinned_by_literal() {
    // If this fails, the carrier's version byte changed — or the carrier is gone — while the
    // public-decrypt path still depends on it. Re-home the proof before touching the container.
    let carrier = encode_solana_extra_data_mmr_proof([0u8; 32], [9u8; 32], 1, &[0xab]);
    assert_eq!(
        carrier[0], PROOF_CARRIER_VERSION,
        "the public-decrypt proof carrier must keep its version byte"
    );
}

#[tokio::test]
async fn a_carrier_of_another_version_yields_no_proof() {
    let fixture = public_then_updated(h(20), h(21));
    let blob = proof_blob(PUBLIC_PROOF_MODE, &fixture.proof(0));
    let valid_carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_without_accounts().await;

    // A context-only carrier, and a version this connector has never issued: both must refuse
    // explicitly before reading any account — never parse the body under the wrong layout.
    let context_only = encode_solana_extra_data_context_only([0u8; 32]);
    let unknown_version = carrier_with_version(0x09, &valid_carrier);

    for carrier in [context_only, unknown_version] {
        let err = check_solana_handles_public_decrypt(&host, &[h(20)], &carrier)
            .await
            .expect_err("a carrier of another version must not yield a proof");
        match err {
            err if err.kind == ProcessingErrorKind::Irrecoverable => assert!(
                err.source
                    .to_string()
                    .contains("requires a PublicDecryptLeaf MMR proof"),
                "got: {err}"
            ),
            other => panic!("a missing proof must be terminal, got: {other:?}"),
        }
    }
}

#[tokio::test]
async fn the_public_path_accepts_only_its_own_mode_byte() {
    // The same valid proof of the same public leaf, presented under the historical mode byte:
    // the public path must refuse on the mode alone, before any verification outcome.
    let fixture = public_then_updated(h(20), h(21));
    let blob = proof_blob(HISTORICAL_PROOF_MODE, &fixture.proof(0));
    let carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_serving(&fixture).await;

    let err = check_solana_handles_public_decrypt(&host, &[h(20)], &carrier)
        .await
        .expect_err("the public path must reject the historical mode byte");
    match err {
        err if err.kind == ProcessingErrorKind::Irrecoverable => {
            assert!(
                err.source.to_string().contains("requires MMR proof mode"),
                "got: {err}"
            )
        }
        other => panic!("a wrong mode byte must be terminal, got: {other:?}"),
    }
}

#[tokio::test]
async fn a_public_decrypt_request_without_a_proof_refuses_explicitly() {
    let fixture = public_then_updated(h(20), h(21));
    let (_server, host) = host_without_accounts().await;

    // A well-formed carrier whose proof section is empty: named value, no proof.
    let empty_proof = public_carrier(&fixture, Vec::new());

    for carrier in [Vec::new(), empty_proof] {
        let err = check_solana_handles_public_decrypt(&host, &[h(20)], &carrier)
            .await
            .expect_err("public decrypt without a proof must refuse, not consult a live flag");
        match err {
            err if err.kind == ProcessingErrorKind::Irrecoverable => assert!(
                err.source
                    .to_string()
                    .contains("requires a PublicDecryptLeaf MMR proof"),
                "got: {err}"
            ),
            other => panic!("a proofless public decrypt must be terminal, got: {other:?}"),
        }
    }
}

#[tokio::test]
async fn public_decrypt_authorizes_one_handle_per_request() {
    let fixture = public_then_updated(h(20), h(21));
    let blob = proof_blob(PUBLIC_PROOF_MODE, &fixture.proof(0));
    let carrier = public_carrier(&fixture, blob);
    let (_server, host) = host_without_accounts().await;

    let err = check_solana_handles_public_decrypt(&host, &[h(20), h(21)], &carrier)
        .await
        .expect_err("a public decrypt names exactly one handle");
    match err {
        err if err.kind == ProcessingErrorKind::Irrecoverable => {
            assert!(
                err.source.to_string().contains("exactly one handle"),
                "got: {err}"
            )
        }
        other => panic!("a multi-handle public decrypt must be terminal, got: {other:?}"),
    }
}
