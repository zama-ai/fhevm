//! `ciphertext-attestation`'s tests check the rules for counting attestations: which ones are
//! valid, how they are grouped, and when a group is big enough.
//!
//! These tests check what happens around those rules. The attestations are read from the
//! Coprocessor buckets over HTTP, and when too few of them agree, the caller must get an error
//! back. They also check which error it is, since `Unavailable` is retried and `NoConsensus` is
//! not, and which buckets are named when the check does succeed.
//!
//! Each Coprocessor gets its own `wiremock` server, so every bucket is a real, separate URL. The
//! registry is built in the test, so there is no chain, no S3 and no Docker.

use alloy::{
    primitives::{Address, B256, U256},
    transports::http::Client,
};
use alloy_signer_local::PrivateKeySigner;
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat,
    S3_METADATA_ATTESTATION_HEADER, Version,
};
use ciphertext_attestation_client::{
    ConsensusCheckError, CoprocessorRegistrySnapshot, ResolvedConsensus,
    fetch_attestations_and_check_consensus,
};
use std::{
    collections::{HashMap, HashSet},
    num::NonZeroUsize,
    time::Duration,
};
use wiremock::{
    Mock, MockServer, ResponseTemplate,
    matchers::{method, path_regex},
};

/// The handle under test, and the context the attestations are signed under. Both are bound by the
/// signature, so a mismatch on either surfaces as a signature failure.
const HANDLE: B256 = B256::repeat_byte(0xAA);
const CONTEXT_ID: U256 = U256::ONE;

const KEY_ID: U256 = U256::ONE;
const CT_DIGEST: B256 = B256::repeat_byte(0xBB);
const SNS_DIGEST: B256 = B256::repeat_byte(0xCC);
const FORMAT: CiphertextFormat = CiphertextFormat::UncompressedOnCpu;

/// Generous enough that a bucket which answers is never mistaken for a slow one.
const HEAD_TIMEOUT: Duration = Duration::from_secs(5);

/// One mocked Coprocessor bucket: its own listener, its own signing key, its own tx sender.
struct Bucket {
    server: MockServer,
    signer: PrivateKeySigner,
    tx_sender: Address,
}

impl Bucket {
    async fn start(index: u8) -> Self {
        Self {
            server: MockServer::start().await,
            signer: PrivateKeySigner::random(),
            // Deliberately distinct from the signer address: the registry maps tx senders to
            // buckets while consensus counts signers, and conflating the two hides bugs.
            tx_sender: Address::repeat_byte(0xA0 + index),
        }
    }

    fn url(&self) -> String {
        self.server.uri()
    }

    /// Serves `attestation` in the S3 metadata header, as a real bucket would.
    async fn serve(&self, attestation: &CiphertextAttestation) {
        let header = serde_json::to_string(attestation).expect("attestation serialization failed");
        self.mount(
            ResponseTemplate::new(200)
                .insert_header(S3_METADATA_ATTESTATION_HEADER, header.as_str()),
        )
        .await;
    }

    /// The object is not there yet — the normal state while Coprocessors are still uploading.
    async fn serve_missing(&self) {
        self.mount(ResponseTemplate::new(404)).await;
    }

    /// Answers, but later than the caller is willing to wait.
    async fn stall(&self, delay: Duration) {
        self.mount(ResponseTemplate::new(200).set_delay(delay))
            .await;
    }

    /// Matches whatever object path the client builds, so no test has to spell the URL out.
    async fn mount(&self, response: ResponseTemplate) {
        Mock::given(method("HEAD"))
            .and(path_regex(r"^/[0-9a-fA-F]{64}/\d+$"))
            .respond_with(response)
            .mount(&self.server)
            .await;
    }
}

async fn start_buckets(count: u8) -> Vec<Bucket> {
    let mut buckets = Vec::with_capacity(count as usize);
    for index in 0..count {
        buckets.push(Bucket::start(index).await);
    }
    buckets
}

/// Registry snapshot over `buckets`, authorizing exactly `signers`.
fn snapshot(
    buckets: &[Bucket],
    threshold: usize,
    signers: HashSet<Address>,
) -> CoprocessorRegistrySnapshot {
    let tx_sender_to_bucket: HashMap<Address, String> = buckets
        .iter()
        .map(|bucket| (bucket.tx_sender, bucket.url()))
        .collect();
    CoprocessorRegistrySnapshot::new(
        signers,
        tx_sender_to_bucket,
        NonZeroUsize::new(threshold).expect("threshold must be non-zero"),
    )
}

/// The usual case: the registry authorizes each bucket's own signing key.
fn snapshot_trusting_buckets(buckets: &[Bucket], threshold: usize) -> CoprocessorRegistrySnapshot {
    let signers = buckets.iter().map(|b| b.signer.address()).collect();
    snapshot(buckets, threshold, signers)
}

/// Attestation over `handle` and `ct_digest`, signed with the bucket's key.
///
/// `ciphertext_digest` is what consensus groups on: give every bucket the same one and they agree,
/// vary it and they do not.
async fn attestation(bucket: &Bucket, handle: B256, ct_digest: B256) -> CiphertextAttestation {
    CiphertextAttestationPayload::new(
        Version::V1,
        handle,
        KEY_ID,
        CONTEXT_ID,
        ct_digest,
        SNS_DIGEST,
        FORMAT,
    )
    .sign(&bucket.signer)
    .await
    .expect("attestation signing failed")
}

async fn check(
    registry: &CoprocessorRegistrySnapshot,
    head_timeout: Duration,
) -> Result<ResolvedConsensus, ConsensusCheckError> {
    fetch_attestations_and_check_consensus(
        &Client::new(),
        HANDLE,
        registry,
        head_timeout,
        CONTEXT_ID,
    )
    .await
}

/// Not enough buckets have published yet. Retryable: the missing ones may still upload.
#[tokio::test]
async fn under_threshold_is_unavailable() {
    let buckets = start_buckets(3).await;
    buckets[0]
        .serve(&attestation(&buckets[0], HANDLE, CT_DIGEST).await)
        .await;
    buckets[1].serve_missing().await;
    buckets[2].serve_missing().await;

    let err = check(&snapshot_trusting_buckets(&buckets, 2), HEAD_TIMEOUT)
        .await
        .expect_err("one attestation must not satisfy a threshold of two");
    assert!(
        matches!(
            err,
            ConsensusCheckError::Unavailable {
                fetched: 1,
                required: 2
            }
        ),
        "expected Unavailable {{ fetched: 1, required: 2 }}, got {err:?}"
    );
}

/// Every bucket answers with a valid signature over different material, so no group reaches the
/// threshold. Terminal: re-reading returns the same disagreement.
#[tokio::test]
async fn cross_bucket_disagreement_is_no_consensus() {
    let buckets = start_buckets(3).await;
    for (index, bucket) in buckets.iter().enumerate() {
        let divergent_digest = B256::repeat_byte(0xB0 + index as u8);
        bucket
            .serve(&attestation(bucket, HANDLE, divergent_digest).await)
            .await;
    }

    let err = check(&snapshot_trusting_buckets(&buckets, 2), HEAD_TIMEOUT)
        .await
        .expect_err("three disagreeing buckets must not reach consensus");
    assert!(
        matches!(err, ConsensusCheckError::NoConsensus(_)),
        "expected NoConsensus, got {err:?}"
    );
}

/// Signatures are valid but recover to keys the registry does not authorize, so every attestation
/// is discarded before counting.
#[tokio::test]
async fn unregistered_signers_are_no_consensus() {
    let buckets = start_buckets(2).await;
    for bucket in &buckets {
        bucket
            .serve(&attestation(bucket, HANDLE, CT_DIGEST).await)
            .await;
    }
    // Both attestations are fetched, so this is not an availability problem: the fetch reached the
    // threshold and consensus rejected what it read.
    let strangers = HashSet::from([Address::repeat_byte(0x11), Address::repeat_byte(0x22)]);

    let err = check(&snapshot(&buckets, 2, strangers), HEAD_TIMEOUT)
        .await
        .expect_err("attestations from unregistered signers must not reach consensus");
    assert!(
        matches!(err, ConsensusCheckError::NoConsensus(_)),
        "expected NoConsensus, got {err:?}"
    );
}

/// Buckets that answer too late count as not having answered.
#[tokio::test]
async fn head_timeout_is_unavailable() {
    let buckets = start_buckets(3).await;
    for bucket in &buckets {
        bucket.stall(Duration::from_secs(30)).await;
    }

    let err = check(
        &snapshot_trusting_buckets(&buckets, 2),
        Duration::from_millis(50),
    )
    .await
    .expect_err("stalled buckets must not yield a consensus");
    assert!(
        matches!(
            err,
            ConsensusCheckError::Unavailable {
                fetched: 0,
                required: 2
            }
        ),
        "expected Unavailable {{ fetched: 0, required: 2 }}, got {err:?}"
    );
}

/// A bucket serving a validly-signed attestation for a *different* handle must not vouch for the
/// handle being probed — the handle is bound by the signature, so it fails verification.
#[tokio::test]
async fn attestation_for_another_handle_is_rejected() {
    let other_handle = B256::repeat_byte(0xEE);
    assert_ne!(other_handle, HANDLE);

    let buckets = start_buckets(3).await;
    for bucket in &buckets {
        bucket
            .serve(&attestation(bucket, other_handle, CT_DIGEST).await)
            .await;
    }

    let err = check(&snapshot_trusting_buckets(&buckets, 2), HEAD_TIMEOUT)
        .await
        .expect_err("attestations over another handle must not reach consensus");
    assert!(
        matches!(err, ConsensusCheckError::NoConsensus(_)),
        "expected NoConsensus, got {err:?}"
    );
}

/// Positive control: without it, a check that rejects everything would pass every test above.
///
/// Threshold-many buckets agree and the third has nothing, so the winning group is exactly the two
/// agreeing buckets — no early-exit ambiguity about which buckets are named.
#[tokio::test]
async fn agreeing_buckets_reach_consensus_and_name_their_buckets() {
    let buckets = start_buckets(3).await;
    for bucket in &buckets[..2] {
        bucket
            .serve(&attestation(bucket, HANDLE, CT_DIGEST).await)
            .await;
    }
    buckets[2].serve_missing().await;

    let resolved = check(&snapshot_trusting_buckets(&buckets, 2), HEAD_TIMEOUT)
        .await
        .expect("two agreeing buckets must reach a threshold of two");

    assert_eq!(resolved.consensus.material.ciphertext_digest, CT_DIGEST);
    assert_eq!(
        resolved.consensus.material.sns_ciphertext_digest,
        SNS_DIGEST
    );
    assert_eq!(resolved.consensus.signers.len(), 2);

    let mut winning = resolved.winning_buckets;
    winning.sort();
    let mut expected = vec![buckets[0].url(), buckets[1].url()];
    expected.sort();
    assert_eq!(winning, expected);
}
