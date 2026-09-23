//! The public-decrypt path, pinned from outside the module that hosts it.
//!
//! Solana public decrypt has no live on-chain "is public" flag: public-ness is a `PublicDecryptLeaf`
//! sealed in the encrypted store's MMR, and the account holds only the peaks. The request
//! supplies one thing — which account the handle lives in, carried in the version-`0x04`
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

use kms_worker::core::event_processor::{ProcessingError, ProcessingErrorKind, RequestCheckError};
use kms_worker::core::solana::{
    SolanaHost,
    proof::{CoprocessorProofClient, LeafProofOutcome, LeafQuery},
    public_decrypt::check_public_decrypt,
    snapshot::SolanaRpcClient,
};
use mocktail::{StatusCode, server::MockServer};
use solana_support::{
    EncryptedStoreFixture, FHE_TYPE_UINT64, HttpHost, LEAF_PROOFS_ROUTE, PROGRAM_ID, handle,
    serve_proofs, solana_host,
};

/// The `extraData` version byte of the public-decrypt carrier. A literal, deliberately not the
/// production constant.
const CARRIER_VERSION: u8 = 0x04;

/// The carrier as the client builds it: version, context id, the handle's encrypted store.
fn carrier(fixture: &EncryptedStoreFixture) -> Vec<u8> {
    let mut blob = vec![CARRIER_VERSION];
    blob.extend_from_slice(&[0x11; 32]);
    blob.extend_from_slice(&fixture.account_key);
    blob
}

/// An account whose current handle was made public, then replaced: the public leaf survives
/// the update because it names the handle, not the slot the handle occupied.
fn public_then_updated(public: [u8; 32], replacement: [u8; 32]) -> EncryptedStoreFixture {
    let mut fixture = EncryptedStoreFixture::new(public);
    fixture.mark_public();
    fixture.update(replacement);
    fixture
}

/// A host serving `fixture`'s account and answering `query` with `outcome`.
async fn host_answering(
    fixture: &EncryptedStoreFixture,
    query: LeafQuery,
    outcome: LeafProofOutcome,
) -> HttpHost {
    let mut host = HttpHost::start().await;
    host.serve_accounts(&[(fixture.account_key, Some(fixture.account()))]);
    host.serve_proofs(&[(query, outcome)]);
    host
}

/// Authorizes `handle` and classifies a refusal as the worker records it.
async fn check(
    host: &SolanaHost,
    handle: [u8; 32],
    extra_data: &[u8],
) -> Result<(), ProcessingError> {
    check_public_decrypt(host, handle, extra_data)
        .await
        .map_err(|failure| RequestCheckError::from(failure).record())
}

fn irrecoverable_containing(err: ProcessingError, needle: &str) {
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
    let host = host_answering(&fixture, query, fixture.outcome(&query)).await;

    check(&host.host(), public, &carrier(&fixture))
        .await
        .expect("a public leaf proven against the account's own peaks authorizes");
}

/// The record's answer is verified, not believed: a well-formed proof of another leaf — here the
/// allow leaf a key holds on the same handle — does not make the handle public.
#[tokio::test]
async fn an_allow_leaf_does_not_prove_public_ness() {
    let allowed = handle(0x30, FHE_TYPE_UINT64);
    let mut fixture = EncryptedStoreFixture::allowing(allowed, [0x42; 32]);
    fixture.update(handle(0x31, FHE_TYPE_UINT64));
    let allow_query = fixture.allowed_query(allowed, [0x42; 32]);
    // The record answers the public query with the allow leaf's proof.
    let host = host_answering(
        &fixture,
        fixture.public_query(allowed),
        fixture.outcome(&allow_query),
    )
    .await;

    let err = check(&host.host(), allowed, &carrier(&fixture))
        .await
        .expect_err("an allow leaf must not prove public-ness");
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Recoverable,
        "a proof that does not verify is a disagreement to retry, got: {err}"
    );
}

/// No public leaf in a record that has sealed the observed history: the handle is not public yet,
/// which is retried as an EVM public decryption of a handle not yet public is.
#[tokio::test]
async fn a_handle_not_made_public_is_retried() {
    let private = handle(0x40, FHE_TYPE_UINT64);
    let fixture = EncryptedStoreFixture::allowing(private, [0x42; 32]);
    let query = fixture.public_query(private);
    let host = host_answering(&fixture, query, fixture.outcome(&query)).await;

    let err = check(&host.host(), private, &carrier(&fixture))
        .await
        .expect_err("a handle nobody made public is not public");
    assert_eq!(err.kind, ProcessingErrorKind::Recoverable, "got: {err}");
    assert_eq!(err.code, kms_connector_api::ErrorCode::AclDenied);
}

/// Every configured coprocessor is asked and the answers are merged: one that fails the read
/// and one that has not sealed the leaf yet cannot sink a request a third can serve.
#[tokio::test]
async fn one_serving_coprocessor_carries_a_request_the_others_cannot() {
    let public = handle(0x55, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x56, FHE_TYPE_UINT64));
    let query = fixture.public_query(public);
    let serving = host_answering(&fixture, query, fixture.outcome(&query)).await;

    let mut failing = MockServer::new_http("coprocessor-failing");
    failing.mock(|when, then| {
        when.post().path(LEAF_PROOFS_ROUTE);
        then.error(StatusCode::INTERNAL_SERVER_ERROR, "leaf record unavailable");
    });
    failing.start().await.expect("the failing mock starts");
    let mut behind = MockServer::new_http("coprocessor-behind");
    serve_proofs(
        &mut behind,
        &[(query, LeafProofOutcome::NotFound { leaf_count: 0 })],
    );
    behind.start().await.expect("the behind mock starts");
    let host = solana_host(&serving.rpc, &[&failing, &behind, &serving.coprocessor]);

    check(&host, public, &carrier(&fixture))
        .await
        .expect("the one coprocessor that serves the proof authorizes the handle");
}

/// A record behind the chain says nothing yet: the read is retried once against the same mock,
/// and then the request is left to the ordinary attempt budget.
#[tokio::test]
async fn a_record_behind_the_chain_is_retried_not_refused() {
    let public = handle(0x50, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x51, FHE_TYPE_UINT64));
    let query = fixture.public_query(public);
    let host = host_answering(
        &fixture,
        query,
        LeafProofOutcome::NotFound { leaf_count: 0 },
    )
    .await;

    let err = check(&host.host(), public, &carrier(&fixture))
        .await
        .expect_err("a record that has not sealed the leaf yet authorizes nothing yet");
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Recoverable,
        "a record behind the chain is retried, got: {err}"
    );
}

/// A carrier of another version, a carrier of another length, and no carrier at all: each is
/// refused explicitly before reading any account, and never parsed under the wrong layout.
#[tokio::test]
async fn a_malformed_carrier_refuses_before_any_read() {
    let fixture = public_then_updated(handle(0x60, FHE_TYPE_UINT64), handle(0x61, FHE_TYPE_UINT64));
    let host = HttpHost::start().await;
    let valid = carrier(&fixture);

    let mut other_version = valid.clone();
    other_version[0] = 0x09;
    let mut obsolete_value_account_version = valid.clone();
    obsolete_value_account_version[0] = 0x03;
    let mut context_only = vec![0x01];
    context_only.extend_from_slice(&[0x11; 32]);
    let mut trailing = valid.clone();
    trailing.push(0);
    let truncated = valid[..valid.len() - 1].to_vec();

    for blob in [
        Vec::new(),
        other_version,
        obsolete_value_account_version,
        context_only,
        trailing,
        truncated,
    ] {
        let err = check(&host.host(), handle(0x60, FHE_TYPE_UINT64), &blob)
            .await
            .expect_err("a carrier that is not the version-4 layout names no state");
        irrecoverable_containing(err, "requires the version-4 extraData");
    }
}

/// The carrier names the account; the account still has to be the host program's. A foreign
/// account at that address proves nothing, however well-formed.
#[tokio::test]
async fn a_carrier_naming_a_foreign_account_is_refused() {
    let public = handle(0x80, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x81, FHE_TYPE_UINT64));
    let mut impostor = fixture.account();
    impostor.owner = [0xee; 32];
    let mut host = HttpHost::start().await;
    host.serve_accounts(&[(fixture.account_key, Some(impostor))]);

    let err = check(&host.host(), public, &carrier(&fixture))
        .await
        .expect_err("a foreign program's account is not an encrypted store");
    irrecoverable_containing(err, "is owned by");
}

/// A peer can stall before headers or halfway through its body. Neither may trap a healthy
/// proof behind join_all; the same bounded client also protects the deciding RPC read.
#[tokio::test]
async fn stalled_http_does_not_block_healthy_proofs_or_rpc_failure() {
    use std::time::Duration;
    use tokio::{io::AsyncWriteExt, net::TcpListener, time::timeout};
    let public = handle(0x91, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x92, FHE_TYPE_UINT64));
    let query = fixture.public_query(public);
    let good = host_answering(&fixture, query, fixture.outcome(&query)).await;
    for send_headers in [false, true] {
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let stalled: url::Url = format!("http://{}", listener.local_addr().unwrap())
            .parse()
            .unwrap();
        let server = tokio::spawn(async move {
            loop {
                let (mut stream, _) = listener.accept().await.unwrap();
                tokio::spawn(async move {
                    if send_headers {
                        stream
                            .write_all(b"HTTP/1.1 200 OK\r\nContent-Length: 1000\r\n\r\n{")
                            .await
                            .unwrap();
                    }
                    std::future::pending::<()>().await;
                    drop(stream);
                });
            }
        });
        let client = reqwest::Client::builder()
            .connect_timeout(Duration::from_millis(250))
            .timeout(Duration::from_millis(250))
            .build()
            .unwrap();
        let mut host = SolanaHost {
            program_id: PROGRAM_ID,
            reader: SolanaRpcClient::new(
                good.rpc.base_url().unwrap().clone(),
                Duration::from_millis(100),
                std::num::NonZeroUsize::MIN,
            ),
            proofs: CoprocessorProofClient::new(
                &[
                    stalled.clone(),
                    good.coprocessor.base_url().unwrap().clone(),
                ],
                "test-key".into(),
                client.clone(),
            ),
        };
        timeout(
            Duration::from_secs(3),
            check(&host, public, &carrier(&fixture)),
        )
        .await
        .expect("fanout must finish")
        .expect("healthy peer still authorizes");
        host.reader = SolanaRpcClient::new(
            stalled,
            Duration::from_millis(100),
            std::num::NonZeroUsize::MIN,
        );
        let error = timeout(
            Duration::from_secs(3),
            check(&host, public, &carrier(&fixture)),
        )
        .await
        .expect("RPC must finish")
        .expect_err("stalled RPC has no observation");
        assert_eq!(error.kind, ProcessingErrorKind::Recoverable);
        server.abort();
    }
}
