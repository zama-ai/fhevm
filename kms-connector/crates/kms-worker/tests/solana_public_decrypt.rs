//! The public-decrypt path, pinned from outside the module that hosts it.
//!
//! Solana public decrypt has no live on-chain "is public" flag: public-ness is a `PublicDecryptLeaf`
//! sealed in the encrypted store's MMR, and the account holds only the peaks. The request
//! supplies one thing per handle — which encrypted store the handle lives in — and the connector
//! does the rest: it reads every named store at `confirmed` in one snapshot, asks the
//! coprocessors' leaf record for the leaves, and verifies each sibling path against the peaks it
//! observed. Nothing the requester hands in is a proof, and nothing the record says is trusted
//! without verifying.
//!
//! These assertions run the public entry point over two real HTTP round-trips — a mock
//! `getMultipleAccounts` endpoint serving the account bytes and a mock leaf-proof route serving
//! the record's answer — so the transport and the rule are exercised together. The leaf-proof
//! route is pinned as a literal, not as an import of the constant that produces it: if it moves,
//! these tests fail by construction rather than following the rename.

mod solana_support;

use alloy::primitives::{B256, U256};
use connector_utils::types::solana_request::SolanaPublicDecryptionRequest;
use kms_worker::core::event_processor::{ProcessingError, ProcessingErrorKind, RequestCheckError};
use kms_worker::core::solana::{
    SolanaHost,
    proof::{CoprocessorProofClient, LeafProofOutcome, LeafQuery},
    public_decrypt::check_public_decrypt,
    snapshot::SolanaRpcClient,
};
use mocktail::{StatusCode, server::MockServer};
use solana_pubkey::Pubkey;
use solana_support::{
    APP_PROGRAM, AUTHORITY, EncryptedStoreFixture, FHE_TYPE_UINT64, HttpHost, LABEL,
    LEAF_PROOFS_ROUTE, PROGRAM_ID, handle, proof_route, pubkey, serve_proofs, solana_host,
};

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

/// Authorizes each handle against the store named beside it and classifies a refusal as the worker
/// records it.
async fn check_entries(
    host: &SolanaHost,
    entries: &[([u8; 32], Pubkey)],
) -> Result<(), ProcessingError> {
    let handles: Vec<_> = entries.iter().map(|(h, _)| B256::new(*h)).collect();
    let stores: Vec<_> = entries
        .iter()
        .map(|(_, s)| B256::new(s.to_bytes()))
        .collect();
    let request = SolanaPublicDecryptionRequest::new(U256::ONE, &handles, &stores, vec![0x00])
        .expect("a well-formed request");
    check_public_decrypt(host, &request)
        .await
        .map_err(|failure| RequestCheckError::from(failure).record())
}

/// Authorizes `handle` against `fixture`'s store.
async fn check(
    host: &SolanaHost,
    handle: [u8; 32],
    fixture: &EncryptedStoreFixture,
) -> Result<(), ProcessingError> {
    check_entries(host, &[(handle, fixture.account_key)]).await
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

    check(&host.host(), public, &fixture)
        .await
        .expect("a public leaf proven against the account's own peaks authorizes");
}

/// The record's answer is verified, not believed: a well-formed proof of another leaf — here the
/// allow leaf a key holds on the same handle — does not make the handle public.
#[tokio::test]
async fn an_allow_leaf_does_not_prove_public_ness() {
    let allowed = handle(0x30, FHE_TYPE_UINT64);
    let mut fixture = EncryptedStoreFixture::allowing(allowed, pubkey(0x42));
    fixture.update(handle(0x31, FHE_TYPE_UINT64));
    let allow_query = fixture.allowed_query(allowed, pubkey(0x42));
    // The record answers the public query with the allow leaf's proof.
    let host = host_answering(
        &fixture,
        fixture.public_query(allowed),
        fixture.outcome(&allow_query),
    )
    .await;

    let err = check(&host.host(), allowed, &fixture)
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
    let fixture = EncryptedStoreFixture::allowing(private, pubkey(0x42));
    let query = fixture.public_query(private);
    let host = host_answering(&fixture, query, fixture.outcome(&query)).await;

    let err = check(&host.host(), private, &fixture)
        .await
        .expect_err("a handle nobody made public is not public");
    assert_eq!(err.kind, ProcessingErrorKind::Recoverable, "got: {err}");
    assert_eq!(err.code, kms_connector_api::ErrorCode::AclDenied);
}

/// A proof that verifies from any coprocessor carries the request: one that fails the read and one
/// that has not sealed the leaf yet cannot sink a request a third can serve.
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

    check(&host, public, &fixture)
        .await
        .expect("the one coprocessor that serves the proof authorizes the handle");
}

/// A record behind the chain says nothing yet: the request is refused recoverably and left to the
/// ordinary attempt budget.
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

    let err = check(&host.host(), public, &fixture)
        .await
        .expect_err("a record that has not sealed the leaf yet authorizes nothing yet");
    assert_eq!(
        err.kind,
        ProcessingErrorKind::Recoverable,
        "a record behind the chain is retried, got: {err}"
    );
}

/// Every handle is proven against its own store, from one read of all of them and one proof batch.
#[tokio::test]
async fn each_handle_is_proven_against_the_store_it_names() {
    let first = handle(0x70, FHE_TYPE_UINT64);
    let second = handle(0x71, FHE_TYPE_UINT64);
    let first_store = public_then_updated(first, handle(0x72, FHE_TYPE_UINT64));
    let mut second_store =
        EncryptedStoreFixture::in_application(APP_PROGRAM, AUTHORITY, pubkey(0x33), LABEL, second);
    second_store.mark_public();
    let queries = [
        first_store.public_query(first),
        second_store.public_query(second),
    ];
    let mut host = HttpHost::start().await;
    host.serve_accounts(&[
        (first_store.account_key, Some(first_store.account())),
        (second_store.account_key, Some(second_store.account())),
    ]);
    host.serve_proofs(&[
        (queries[0], first_store.outcome(&queries[0])),
        (queries[1], second_store.outcome(&queries[1])),
    ]);

    check_entries(
        &host.host(),
        &[
            (first, first_store.account_key),
            (second, second_store.account_key),
        ],
    )
    .await
    .expect("each handle is public in the store it names");
}

/// A store that is the host program's but does not hold the handle's public leaf proves nothing:
/// the request is retried, as an EVM public decryption of a handle not yet public is, and never
/// authorized by another store's leaf.
#[tokio::test]
async fn a_store_that_does_not_hold_the_handle_authorizes_nothing() {
    let public = handle(0x75, FHE_TYPE_UINT64);
    let wrong = EncryptedStoreFixture::allowing(handle(0x76, FHE_TYPE_UINT64), pubkey(0x42));
    let query = wrong.public_query(public);
    let host = host_answering(&wrong, query, wrong.outcome(&query)).await;

    let err = check(&host.host(), public, &wrong)
        .await
        .expect_err("the named store holds no public leaf for the handle");
    assert_eq!(err.kind, ProcessingErrorKind::Recoverable, "got: {err}");
    assert_eq!(err.code, kms_connector_api::ErrorCode::AclDenied);
    assert!(err.source.to_string().contains("entry 0"), "got: {err}");
}

/// The request names the account; the account still has to be the host program's. A foreign
/// account at that address proves nothing, however well-formed.
#[tokio::test]
async fn a_request_naming_a_foreign_account_is_refused() {
    let public = handle(0x80, FHE_TYPE_UINT64);
    let fixture = public_then_updated(public, handle(0x81, FHE_TYPE_UINT64));
    let mut impostor = fixture.account();
    impostor.owner = pubkey(0xee);
    let mut host = HttpHost::start().await;
    host.serve_accounts(&[(fixture.account_key, Some(impostor))]);

    let err = check(&host.host(), public, &fixture)
        .await
        .expect_err("a foreign program's account is not an encrypted store");
    irrecoverable_containing(err, "is owned by");
}

/// A coprocessor can stall before headers or halfway through its body. The healthy coprocessor's
/// proof decides without waiting on it; the same bounded client also protects the RPC read.
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
                    proof_route(&stalled),
                    proof_route(good.coprocessor.base_url().unwrap()),
                ],
                client.clone(),
            ),
        };
        timeout(Duration::from_secs(3), check(&host, public, &fixture))
            .await
            .expect("the stalled coprocessor does not hold the request")
            .expect("the healthy coprocessor authorizes");
        host.reader = SolanaRpcClient::new(
            stalled,
            Duration::from_millis(100),
            std::num::NonZeroUsize::MIN,
        );
        let error = timeout(Duration::from_secs(3), check(&host, public, &fixture))
            .await
            .expect("RPC must finish")
            .expect_err("stalled RPC has no observation");
        assert_eq!(error.kind, ProcessingErrorKind::Recoverable);
        server.abort();
    }
}
