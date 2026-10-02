//! The Merkle proof reader. A store's MMR leaves live in the coprocessors' record of host program
//! events; the account holds only the peaks. Each coprocessor is a source of proofs, never of
//! decisions: every answer is verified against the observed peaks.

use alloy::primitives::B256;
use connector_utils::config::KmsWallet;
use request_authorization::KeyRegistry;
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::{
    future::Future,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
use url::Url;

/// The coprocessor route that answers Merkle proof queries. The same literal as the coprocessor's
/// `MERKLE_PROOFS_PATH`; the shared vectors pin the request and response shapes.
pub const MERKLE_PROOFS_PATH: &str = "/v1/solana/merkle-proofs";

/// The coprocessor's cap on queries per read. A validated request stays below it: it has at most
/// `MAX_REQUEST_HANDLES` entries.
const MAX_LEAVES_PER_READ: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LeafKind {
    /// `key` was allowed on the handle.
    Allowed {
        #[serde(with = "alloy::hex::serde::no_prefix")]
        key: Pubkey,
    },
    /// The handle was made public.
    Public,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeafQuery {
    #[serde(with = "alloy::hex::serde::no_prefix")]
    pub encrypted_store: Pubkey,
    #[serde(with = "alloy::hex::serde::no_prefix")]
    pub handle: B256,
    #[serde(flatten)]
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

    /// How many coprocessors can be asked.
    fn source_count(&self) -> usize;

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
// The HTTP transport. Field names mirror `coprocessor/fhevm-engine/solana-merkle-proof-service/src/server.rs`.

#[derive(Serialize)]
struct MerkleProofRequest<'a> {
    leaves: &'a [LeafQuery],
}

#[derive(Deserialize)]
struct MerkleProofResponse {
    proofs: Vec<MerkleProofOutcome>,
}

pub fn merkle_proof_request_body(queries: &[LeafQuery]) -> impl Serialize + '_ {
    MerkleProofRequest { leaves: queries }
}

fn decode_siblings<'de, D: serde::Deserializer<'de>>(
    deserializer: D,
) -> Result<Vec<[u8; 32]>, D::Error> {
    let siblings = Vec::<String>::deserialize(deserializer)?;
    if siblings.len() > zama_solana_acl::MAX_MMR_PEAKS {
        return Err(serde::de::Error::custom("too many proof siblings"));
    }
    siblings
        .iter()
        .map(|s| alloy::hex::decode_to_array(s).map_err(serde::de::Error::custom))
        .collect()
}

pub fn parse_merkle_proof_response(body: &str) -> Result<Vec<MerkleProofOutcome>, ProofReadError> {
    let response: MerkleProofResponse = serde_json::from_str(body)
        .map_err(|error| unavailable(format!("response does not decode: {error}")))?;
    Ok(response.proofs)
}

fn unavailable(reason: String) -> ProofReadError {
    ProofReadError::Unavailable { reason }
}

/// How long a signed batch stays valid: the reads of every coprocessor, plus clock skew with them.
const AUTHORIZATION_VALIDITY_SECS: u64 = 120;
const _: () = assert!(AUTHORIZATION_VALIDITY_SECS < request_authorization::MAX_VALIDITY_SECS);

/// The production reader: one signed `POST` per coprocessor. Every coprocessor of a batch receives
/// the same body and signature, so the wallet signs once per batch.
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

    async fn read_from(
        &self,
        url: &Url,
        batch: &SignedBatch,
    ) -> Result<Vec<MerkleProofOutcome>, ProofReadError> {
        let response = self
            .client
            .post(url.clone())
            .header("authorization", &batch.authorization)
            .header("content-type", "application/json")
            .body(batch.body.clone())
            .send()
            .await
            .map_err(|error| unavailable(format!("{url}: request failed: {error}")))?;
        let status = response.status();
        if !status.is_success() {
            return Err(unavailable(format!("{url}: HTTP {status}")));
        }
        // At most 64 queries, each with 64 hex siblings and bounded numeric metadata.
        const MAX_RESPONSE_BYTES: usize =
            MAX_LEAVES_PER_READ * (zama_solana_acl::MAX_MMR_PEAKS * 68 + 256);
        let mut response = response;
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| unavailable(format!("{url}: body could not be read: {error}")))?
        {
            if body.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(unavailable(format!(
                    "{url}: proof response exceeds {MAX_RESPONSE_BYTES} bytes"
                )));
            }
            body.extend_from_slice(&chunk);
        }
        let text = std::str::from_utf8(&body).map_err(|e| unavailable(format!("{url}: {e}")))?;
        parse_merkle_proof_response(text)
    }
}

impl HostProofReader for CoprocessorProofClient {
    type Batch = SignedBatch;

    fn source_count(&self) -> usize {
        self.urls.len()
    }

    async fn prepare(&self, queries: &[LeafQuery]) -> Result<SignedBatch, ProofReadError> {
        let body = serde_json::to_vec(&merkle_proof_request_body(queries))
            .expect("a Merkle proof request has no map keys or fallible fields");
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
        self.read_from(&self.urls[source], batch).await
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shared spelling of the wire; the coprocessor pins its own against the same file.
    const MERKLE_PROOFS_FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../solana/test-fixtures/merkle-proofs/merkle_proofs_v1.json"
    );

    fn fixture() -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(MERKLE_PROOFS_FIXTURE).expect("read fixture"))
            .expect("fixture is json")
    }

    #[test]
    fn request_body_matches_the_shared_fixture() {
        let fixture = fixture();
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
            serde_json::to_value(merkle_proof_request_body(&queries)).unwrap(),
            fixture["request"]
        );
    }

    #[test]
    fn every_fixture_answer_decodes() {
        let fixture = fixture();
        let body = serde_json::json!({ "proofs": fixture["proofs"] }).to_string();
        assert_eq!(
            parse_merkle_proof_response(&body).expect("the fixture answers decode"),
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
            reqwest::Client::new(),
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
        let found = |siblings: serde_json::Value| {
            serde_json::json!({
                "proofs": [{"status": "found", "leafIndex": 0, "leafCount": 1, "siblings": siblings}]
            })
            .to_string()
        };
        let invalid_sibling = found(serde_json::json!(["00"]));
        let too_many_siblings = found(serde_json::json!(vec!["00".repeat(32); 65]));
        for body in [
            "not JSON".to_owned(),
            invalid_sibling,
            too_many_siblings,
            " ".repeat(300_000),
        ] {
            let mut server = MockServer::new_http("proof-response");
            server.mock(move |when, then| {
                when.post().path(MERKLE_PROOFS_PATH);
                then.text(body.clone());
            });
            server.start().await.unwrap();
            let client = client(&[server.base_url().unwrap()], &wallet());
            let batch = client.prepare(&query()).await.unwrap();
            client
                .read_proofs(0, &batch)
                .await
                .expect_err("a malformed response is a failed read");
        }
    }

    /// Every coprocessor receives the same body and signature, which recovers to the wallet over
    /// that exact body.
    #[tokio::test]
    async fn every_coprocessor_receives_one_signature_by_the_wallet() {
        use mocktail::server::MockServer;
        let wallet = wallet();
        let batch = client(&[], &wallet).prepare(&query()).await.unwrap();
        let mut servers = Vec::new();
        for name in ["first", "second"] {
            let mut server = MockServer::new_http(name);
            let authorization = batch.authorization.clone();
            server.mock(move |when, then| {
                when.post()
                    .path(MERKLE_PROOFS_PATH)
                    .header("authorization", authorization.clone());
                then.text(r#"{"proofs":[{"status":"notFound","leafCount":0}]}"#);
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
            request_authorization::recover_signer(
                &REGISTRY,
                &batch.authorization,
                MERKLE_PROOFS_PATH,
                &batch.body,
                now,
            ),
            Ok(wallet.address())
        );
        assert_eq!(
            batch.body,
            serde_json::to_vec(&merkle_proof_request_body(&query())).unwrap()
        );
    }
}
