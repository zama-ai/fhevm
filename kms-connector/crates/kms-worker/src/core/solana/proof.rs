//! The Merkle proof reader. A store's MMR leaves live in the coprocessors' record of host program
//! events; the account holds only the peaks. Each coprocessor is a source of proofs, never of
//! decisions: every answer is verified against the observed peaks.

use alloy::primitives::B256;
use connector_utils::config::KmsWallet;
use rand::seq::SliceRandom;
use request_authorization::KeyRegistry;
use serde::{Deserialize, Serialize};
use serde_bytes::ByteArray;
use solana_pubkey::Pubkey;
use std::{
    future::Future,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use url::Url;

/// The coprocessor route that answers Merkle proof queries. The same literal as the coprocessor's
/// `MERKLE_PROOFS_PATH`; the shared vectors pin the request and response bytes.
pub const MERKLE_PROOFS_PATH: &str = "/v1/solana/merkle-proofs";

/// The coprocessor's cap on queries per read. A validated request stays below it: it has at most
/// `MAX_REQUEST_HANDLES` entries.
const MAX_LEAVES_PER_READ: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LeafKind {
    /// `key` was allowed on the handle.
    Allowed { key: Pubkey },
    /// The handle was made public.
    Public,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LeafQuery {
    pub encrypted_store: Pubkey,
    pub handle: B256,
    pub kind: LeafKind,
}

/// What the record said about one query.
#[derive(Clone, PartialEq, Eq, Debug, Deserialize)]
#[serde(
    rename_all = "camelCase",
    tag = "status",
    rename_all_fields = "camelCase"
)]
pub enum MerkleProofOutcome {
    /// `leaf_count` is how many leaves the record had sealed when it built the proof. The
    /// verifier checks the siblings against the on-chain peaks, not this number.
    Found {
        leaf_index: u64,
        leaf_count: u64,
        #[serde(deserialize_with = "decode_siblings")]
        siblings: Vec<[u8; 32]>,
    },
    /// The record knows the account and has no such leaf in the history it has sealed.
    NotFound { leaf_count: u64 },
    /// The record has never seen this account.
    UnknownAccount,
}

/// Implemented by [`CoprocessorProofClient`]; tests drive authorization with canned proofs.
pub trait HostProofReader: Send + Sync {
    /// A batch of queries made ready once and sent to any coprocessor.
    type Batch: Sync;

    /// The coprocessors in the order to ask them.
    fn hedge_order(&self) -> Vec<usize>;

    fn prepare(
        &self,
        queries: &[LeafQuery],
    ) -> impl Future<Output = Result<Self::Batch, ProofReadError>> + Send;

    /// What coprocessor `source` answers, one outcome per query in query order. No outcome is
    /// trusted here.
    fn read_proofs(
        &self,
        source: usize,
        batch: &Self::Batch,
    ) -> impl Future<Output = Result<Vec<MerkleProofOutcome>, ProofReadError>> + Send;
}

pub(super) fn check_length(requested: usize, returned: usize) -> Result<(), ProofReadError> {
    if requested == returned {
        Ok(())
    } else {
        Err(ProofReadError::ResponseLengthMismatch {
            requested,
            returned,
        })
    }
}

/// Why a batch could not be read from a coprocessor. Every variant says nothing about any leaf,
/// and a later read may succeed.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ProofReadError {
    /// A coprocessor could not be read; from [`verify_proofs`], some query has no answer that
    /// decides it.
    ///
    /// [`verify_proofs`]: super::handle_binding::verify_proofs
    #[error("Merkle proof read failed: {reason}")]
    Unavailable { reason: String },
    #[error("Merkle proof read returned {returned} outcomes for {requested} queries")]
    ResponseLengthMismatch { requested: usize, returned: usize },
}

// ------------------------------------------------------------------------------------------
// The HTTP transport: CBOR bodies over HTTP/2. Field names mirror
// `coprocessor/fhevm-engine/solana-merkle-proof-service/src/server.rs`.

#[derive(Serialize)]
struct MerkleProofRequest {
    leaves: Vec<WireLeaf>,
}

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct WireLeaf {
    #[serde(with = "serde_bytes")]
    encrypted_store: [u8; 32],
    #[serde(with = "serde_bytes")]
    handle: [u8; 32],
    kind: WireLeafKind,
    #[serde(skip_serializing_if = "Option::is_none", with = "serde_bytes")]
    key: Option<[u8; 32]>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum WireLeafKind {
    Allowed,
    Public,
}

impl From<&LeafQuery> for WireLeaf {
    fn from(query: &LeafQuery) -> Self {
        let (kind, key) = match query.kind {
            LeafKind::Allowed { key } => (WireLeafKind::Allowed, Some(key.to_bytes())),
            LeafKind::Public => (WireLeafKind::Public, None),
        };
        Self {
            encrypted_store: query.encrypted_store.to_bytes(),
            handle: query.handle.0,
            kind,
            key,
        }
    }
}

#[derive(Deserialize)]
struct MerkleProofResponse {
    proofs: Vec<MerkleProofOutcome>,
}

/// The coprocessor's error body (RFC 033), read only to report it.
#[derive(Deserialize)]
struct ErrorResponse {
    code: String,
    message: String,
}

/// The CBOR body asking for `queries`.
pub fn encode_merkle_proof_request(queries: &[LeafQuery]) -> Vec<u8> {
    let request = MerkleProofRequest {
        leaves: queries.iter().map(WireLeaf::from).collect(),
    };
    let mut body = Vec::new();
    ciborium::into_writer(&request, &mut body)
        .expect("a Merkle proof request has no fallible fields");
    body
}

fn decode_siblings<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<[u8; 32]>, D::Error> {
    let siblings = Vec::<ByteArray<32>>::deserialize(deserializer)?;
    if siblings.len() > zama_solana_acl::MAX_MMR_PEAKS {
        return Err(serde::de::Error::custom("too many proof siblings"));
    }
    Ok(siblings.into_iter().map(ByteArray::into_array).collect())
}

/// Decodes exactly one CBOR item filling `body`.
fn decode_cbor<T: serde::de::DeserializeOwned>(body: &[u8]) -> Result<T, String> {
    let mut rest = body;
    let value = ciborium::from_reader(&mut rest)
        .map_err(|error| format!("body does not decode: {error}"))?;
    if !rest.is_empty() {
        return Err(format!("{} bytes after the body", rest.len()));
    }
    Ok(value)
}

/// The client the proof reads share: HTTP/2 without TLS negotiation (prior knowledge), so the
/// requests to one coprocessor share one connection.
pub fn proof_http_client(timeout: Duration) -> reqwest::Result<reqwest::Client> {
    reqwest::Client::builder()
        .http2_prior_knowledge()
        .connect_timeout(timeout)
        .timeout(timeout)
        .build()
}

fn unavailable(reason: String) -> ProofReadError {
    ProofReadError::Unavailable { reason }
}

/// How long a signed batch stays valid: the reads of every coprocessor, plus clock skew with them.
const AUTHORIZATION_VALIDITY_SECS: u64 = 120;
const _: () = assert!(AUTHORIZATION_VALIDITY_SECS < request_authorization::MAX_VALIDITY_SECS);

/// The production reader: one signed `POST` per coprocessor asked. Every coprocessor of a batch
/// receives the same body and signature, so the wallet signs once per batch.
#[derive(Clone, Debug)]
pub struct CoprocessorProofClient {
    urls: Vec<Url>,
    client: reqwest::Client,
    /// The connector's tx-sender wallet.
    wallet: KmsWallet,
    /// The canonical `ProtocolConfig`, which the signature's EIP-712 domain names.
    registry: KeyRegistry,
    /// Bounds the wallet's signature, an AWS KMS call in production.
    signing_timeout: Duration,
}

/// A batch signed for every coprocessor.
pub struct SignedBatch {
    body: Vec<u8>,
    authorization: String,
}

impl CoprocessorProofClient {
    pub fn new(
        urls: &[Url],
        client: reqwest::Client,
        wallet: KmsWallet,
        registry: KeyRegistry,
        signing_timeout: Duration,
    ) -> Self {
        let urls = urls
            .iter()
            .map(|url| {
                let mut url = url.clone();
                url.set_path(MERKLE_PROOFS_PATH);
                url
            })
            .collect();
        Self {
            urls,
            client,
            wallet,
            registry,
            signing_timeout,
        }
    }
}

impl HostProofReader for CoprocessorProofClient {
    type Batch = SignedBatch;

    /// A fresh random order per batch spreads the reads over the coprocessors.
    fn hedge_order(&self) -> Vec<usize> {
        let mut order: Vec<usize> = (0..self.urls.len()).collect();
        order.shuffle(&mut rand::rng());
        order
    }

    async fn prepare(&self, queries: &[LeafQuery]) -> Result<SignedBatch, ProofReadError> {
        let body = encode_merkle_proof_request(queries);
        let expires = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .map_or(0, |elapsed| elapsed.as_secs())
            + AUTHORIZATION_VALIDITY_SECS;
        let signing = request_authorization::authorize(
            &self.wallet,
            &self.registry,
            MERKLE_PROOFS_PATH,
            &body,
            expires,
        );
        let authorization = tokio::time::timeout(self.signing_timeout, signing)
            .await
            .map_err(|_| {
                unavailable(format!(
                    "the proof request was not signed within {:?}",
                    self.signing_timeout
                ))
            })?
            .map_err(|error| {
                unavailable(format!("the proof request could not be signed: {error}"))
            })?;
        Ok(SignedBatch {
            body,
            authorization,
        })
    }

    async fn read_proofs(
        &self,
        source: usize,
        batch: &SignedBatch,
    ) -> Result<Vec<MerkleProofOutcome>, ProofReadError> {
        let url = &self.urls[source];
        let failed = |reason: String| unavailable(format!("{url}: {reason}"));
        let mut response = self
            .client
            .post(url.clone())
            .header("authorization", &batch.authorization)
            .header("content-type", "application/cbor")
            .body(batch.body.clone())
            .send()
            .await
            .map_err(|error| failed(format!("request failed: {error}")))?;
        // At most 64 answers, each with at most 64 siblings of 34 bytes and under 128 bytes of
        // other fields.
        const MAX_RESPONSE_BYTES: usize =
            MAX_LEAVES_PER_READ * (zama_solana_acl::MAX_MMR_PEAKS * 34 + 128);
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| failed(format!("body could not be read: {error}")))?
        {
            if body.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(failed(format!("body exceeds {MAX_RESPONSE_BYTES} bytes")));
            }
            body.extend_from_slice(&chunk);
        }
        let status = response.status();
        if !status.is_success() {
            let error = decode_cbor::<ErrorResponse>(&body).map_or_else(
                |_| String::new(),
                |error| format!(" {}: {}", error.code, error.message),
            );
            return Err(failed(format!("HTTP {status}{error}")));
        }
        decode_cbor::<MerkleProofResponse>(&body)
            .map(|response| response.proofs)
            .map_err(failed)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use ciborium::cbor;

    /// The shared spelling of the wire; the coprocessor pins its own against the same file.
    const MERKLE_PROOFS_FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../solana/test-fixtures/merkle-proofs/merkle_proofs_v1.json"
    );

    fn fixture() -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(MERKLE_PROOFS_FIXTURE).expect("read fixture"))
            .expect("fixture is json")
    }

    fn cbor(value: ciborium::Value) -> Vec<u8> {
        let mut body = Vec::new();
        ciborium::into_writer(&value, &mut body).unwrap();
        body
    }

    #[test]
    fn request_body_matches_the_shared_fixture() {
        let fixture = fixture();
        assert_eq!(fixture["path"], MERKLE_PROOFS_PATH);
        assert_eq!(
            fixture["maxLeavesPerRequest"],
            serde_json::json!(MAX_LEAVES_PER_READ)
        );
        let queries = [
            LeafQuery {
                encrypted_store: Pubkey::new_from_array([0xAC; 32]),
                handle: B256::new([0x10; 32]),
                kind: LeafKind::Allowed {
                    key: Pubkey::new_from_array([0xA1; 32]),
                },
            },
            LeafQuery {
                encrypted_store: Pubkey::new_from_array([0xAC; 32]),
                handle: B256::new([0x11; 32]),
                kind: LeafKind::Public,
            },
        ];
        assert_eq!(
            alloy::hex::encode(encode_merkle_proof_request(&queries)),
            fixture["request"].as_str().unwrap()
        );
    }

    #[test]
    fn every_fixture_answer_decodes() {
        let body = alloy::hex::decode(fixture()["response"].as_str().unwrap()).unwrap();
        assert_eq!(
            decode_cbor::<MerkleProofResponse>(&body)
                .expect("the fixture answers decode")
                .proofs,
            vec![
                MerkleProofOutcome::Found {
                    leaf_index: 1,
                    leaf_count: 3,
                    siblings: vec![[0x5B; 32]],
                },
                MerkleProofOutcome::NotFound { leaf_count: 3 },
                MerkleProofOutcome::UnknownAccount,
            ]
        );
    }

    const REGISTRY: KeyRegistry = KeyRegistry {
        chain_id: 12345,
        contract: alloy::primitives::Address::repeat_byte(0xC0),
    };

    fn client(urls: &[&Url], wallet: &KmsWallet) -> CoprocessorProofClient {
        let urls: Vec<Url> = urls.iter().map(|url| (*url).clone()).collect();
        CoprocessorProofClient::new(
            &urls,
            proof_http_client(Duration::from_secs(5)).unwrap(),
            wallet.clone(),
            REGISTRY,
            Duration::from_secs(5),
        )
    }

    fn query() -> [LeafQuery; 1] {
        [LeafQuery {
            encrypted_store: Pubkey::new_from_array([1; 32]),
            handle: B256::new([2; 32]),
            kind: LeafKind::Public,
        }]
    }

    fn wallet() -> KmsWallet {
        KmsWallet::from_private_key_str(
            "0x3f45b129a7fd099146e9fe63851a71646231f7743c712695f3b2d2bf0e41c774",
            None,
        )
        .unwrap()
    }

    #[tokio::test]
    async fn malformed_and_oversized_proof_responses_are_recoverable_read_errors() {
        use mocktail::server::MockServer;
        let found = |siblings: Vec<Vec<u8>>| {
            cbor!({
                "proofs" => [{
                    "status" => "found",
                    "leafIndex" => 0,
                    "leafCount" => 1,
                    "siblings" => siblings
                        .into_iter()
                        .map(ciborium::Value::Bytes)
                        .collect::<Vec<_>>(),
                }]
            })
            .unwrap()
        };
        let not_found =
            cbor(cbor!({ "proofs" => [{ "status" => "notFound", "leafCount" => 0 }] }).unwrap());
        let mut trailing = not_found.clone();
        trailing.push(0);
        for body in [
            b"{\"proofs\":[]}".to_vec(),
            cbor(found(vec![vec![0]])),
            cbor(found(vec![vec![0; 32]; 65])),
            trailing,
            vec![0; 300_000],
        ] {
            let mut server = MockServer::new_http("proof-response");
            server.mock(move |when, then| {
                when.post().path(MERKLE_PROOFS_PATH);
                then.bytes(body.clone());
            });
            server.start().await.unwrap();
            let client = client(&[server.base_url().unwrap()], &wallet());
            let batch = client.prepare(&query()).await.unwrap();
            client
                .read_proofs(0, &batch)
                .await
                .expect_err("a malformed response is a failed read");
        }

        let mut server = MockServer::new_http("proof-response");
        server.mock(move |when, then| {
            when.post().path(MERKLE_PROOFS_PATH);
            then.bytes(not_found.clone());
        });
        server.start().await.unwrap();
        let client = client(&[server.base_url().unwrap()], &wallet());
        let batch = client.prepare(&query()).await.unwrap();
        assert_eq!(
            client.read_proofs(0, &batch).await,
            Ok(vec![MerkleProofOutcome::NotFound { leaf_count: 0 }]),
            "the same answer without trailing bytes decodes"
        );
    }

    /// A refusal carries the coprocessor's error code and message into the read error.
    #[tokio::test]
    async fn a_refusal_reports_the_coprocessor_error() {
        use mocktail::{StatusCode, server::MockServer};
        let refusal = cbor(
            cbor!({
                "code" => "rate_limited",
                "message" => "no database connection free within 200ms",
                "retryable" => true,
            })
            .unwrap(),
        );
        let mut server = MockServer::new_http("refusal");
        server.mock(move |when, then| {
            when.post().path(MERKLE_PROOFS_PATH);
            then.status(StatusCode::TOO_MANY_REQUESTS)
                .bytes(refusal.clone());
        });
        server.start().await.unwrap();
        let url = server.base_url().unwrap();
        let client = client(&[url], &wallet());
        let batch = client.prepare(&query()).await.unwrap();
        let mut url = url.clone();
        url.set_path(MERKLE_PROOFS_PATH);
        assert_eq!(
            client.read_proofs(0, &batch).await,
            Err(ProofReadError::Unavailable {
                reason: format!(
                    "{url}: HTTP 429 Too Many Requests rate_limited: \
                     no database connection free within 200ms"
                ),
            })
        );
    }

    /// Every coprocessor receives the same body and signature, which recovers to the wallet over
    /// that exact body.
    #[tokio::test]
    async fn every_coprocessor_receives_one_signature_by_the_wallet() {
        use mocktail::server::MockServer;
        let wallet = wallet();
        let batch = client(&[], &wallet).prepare(&query()).await.unwrap();
        let mut servers = Vec::new();
        let answer =
            cbor(cbor!({ "proofs" => [{ "status" => "notFound", "leafCount" => 0 }] }).unwrap());
        for name in ["first", "second"] {
            let mut server = MockServer::new_http(name);
            let authorization = batch.authorization.clone();
            let answer = answer.clone();
            server.mock(move |when, then| {
                when.post()
                    .path(MERKLE_PROOFS_PATH)
                    .header("authorization", authorization.clone())
                    .header("content-type", "application/cbor");
                then.bytes(answer.clone());
            });
            server.start().await.unwrap();
            servers.push(server);
        }
        let client = client(
            &[
                servers[0].base_url().unwrap(),
                servers[1].base_url().unwrap(),
            ],
            &wallet,
        );
        for source in 0..2 {
            client
                .read_proofs(source, &batch)
                .await
                .expect("each coprocessor receives the batch's signature");
        }

        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        assert_eq!(
            request_authorization::recover_authorization(
                &REGISTRY,
                &batch.authorization,
                MERKLE_PROOFS_PATH,
                &batch.body,
                now,
            )
            .map(|authorization| authorization.signer),
            Ok(wallet.address())
        );
        assert_eq!(batch.body, encode_merkle_proof_request(&query()));
    }
}
