//! The leaf-proof reader. A store's MMR leaves live in the coprocessors' record of host program
//! events; the account holds only the peaks. The record is a source of proofs, never of
//! decisions: every candidate is verified against the observed peaks.

use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::future::Future;
use url::Url;

/// The coprocessor route that answers leaf-proof queries. The same literal as the coprocessor's
/// `LEAF_PROOFS_PATH`; the shared vectors pin the request and response shapes.
pub const LEAF_PROOFS_PATH: &str = "/v1/solana/leaf-proofs";

/// The coprocessor's cap on queries per read.
const MAX_LEAVES_PER_READ: usize = 64;

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum LeafKind {
    /// `key` was allowed on the handle.
    Allowed {
        #[serde(with = "alloy::hex::serde::no_prefix")]
        key: SolanaPubkeyBytes,
    },
    /// The handle was made public.
    Public,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LeafQuery {
    #[serde(with = "alloy::hex::serde::no_prefix")]
    pub encrypted_store: SolanaPubkeyBytes,
    #[serde(with = "alloy::hex::serde::no_prefix")]
    pub handle: HandleBytes,
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
pub enum LeafProofOutcome {
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
    /// The record's history for this account has a gap it cannot close until it is rebuilt.
    HistoryIncomplete,
}

/// Per-query candidates plus any peers that could not answer the batch.
#[derive(Debug)]
pub struct ProofResponses {
    pub candidates: Vec<Vec<LeafProofOutcome>>,
    pub unavailable: Option<ProofReadError>,
}

/// Implemented by [`CoprocessorProofClient`]; tests drive authorization with canned proofs.
pub trait HostProofReader: Send + Sync {
    /// One list of peer candidates per query, in query order. No candidate is trusted here.
    fn read_proofs(
        &self,
        queries: &[LeafQuery],
    ) -> impl Future<Output = Result<ProofResponses, ProofReadError>> + Send;
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

/// Why a batch could not be read at all. Every variant says nothing about any leaf.
#[derive(Clone, PartialEq, Eq, Debug, thiserror::Error)]
pub enum ProofReadError {
    #[error("leaf proof request serialization failed: {reason}")]
    RequestEncoding { reason: String },
    /// No coprocessor answered.
    #[error("leaf proof read failed: {reason}")]
    Unavailable { reason: String },
    #[error("leaf proof read returned {returned} outcomes for {requested} queries")]
    ResponseLengthMismatch { requested: usize, returned: usize },
    /// Unreachable from a validated request, which has at most `MAX_REQUEST_HANDLES` entries.
    #[error("leaf proof batch of {count} queries exceeds the {MAX_LEAVES_PER_READ}-query cap")]
    TooManyQueries { count: usize },
}

// ------------------------------------------------------------------------------------------
// The HTTP transport. Field names mirror the coprocessor's `http_server.rs`.

#[derive(Serialize)]
struct LeafProofRequest<'a> {
    leaves: &'a [LeafQuery],
}

#[derive(Deserialize)]
struct LeafProofResponse {
    proofs: Vec<LeafProofOutcome>,
}

pub fn leaf_proof_request_body(queries: &[LeafQuery]) -> impl Serialize + '_ {
    LeafProofRequest { leaves: queries }
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

pub fn parse_leaf_proof_response(
    body: &str,
    requested: usize,
) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
    let response: LeafProofResponse = serde_json::from_str(body)
        .map_err(|error| unavailable(format!("response does not decode: {error}")))?;
    check_length(requested, response.proofs.len())?;
    Ok(response.proofs)
}

fn unavailable(reason: String) -> ProofReadError {
    ProofReadError::Unavailable { reason }
}

/// The production reader: one authenticated `POST` per coprocessor per read, grouped per query.
#[derive(Clone, Debug)]
pub struct CoprocessorProofClient {
    routes: Vec<Url>,
    api_key: String,
    client: reqwest::Client,
}

impl CoprocessorProofClient {
    pub fn new(endpoints: &[Url], api_key: String, client: reqwest::Client) -> Self {
        let routes = endpoints
            .iter()
            .map(|endpoint| {
                let mut route = endpoint.clone();
                route.set_path(LEAF_PROOFS_PATH);
                route
            })
            .collect();
        Self {
            routes,
            api_key,
            client,
        }
    }

    async fn read_from(
        &self,
        route: &Url,
        body: &[u8],
        requested: usize,
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        let response = self
            .client
            .post(route.clone())
            .bearer_auth(&self.api_key)
            .header("content-type", "application/json")
            .body(body.to_vec())
            .send()
            .await
            .map_err(|error| unavailable(format!("{route}: request failed: {error}")))?;
        let status = response.status();
        if !status.is_success() {
            return Err(unavailable(format!("{route}: HTTP {status}")));
        }
        // At most 64 queries, each with 64 hex siblings and bounded numeric metadata.
        const MAX_RESPONSE_BYTES: usize =
            MAX_LEAVES_PER_READ * (zama_solana_acl::MAX_MMR_PEAKS * 68 + 256);
        let mut response = response;
        let mut body = Vec::new();
        while let Some(chunk) = response
            .chunk()
            .await
            .map_err(|error| unavailable(format!("{route}: body could not be read: {error}")))?
        {
            if body.len() + chunk.len() > MAX_RESPONSE_BYTES {
                return Err(unavailable(format!(
                    "{route}: proof response exceeds {MAX_RESPONSE_BYTES} bytes"
                )));
            }
            body.extend_from_slice(&chunk);
        }
        let text = std::str::from_utf8(&body).map_err(|e| unavailable(format!("{route}: {e}")))?;
        parse_leaf_proof_response(text, requested)
    }
}

impl HostProofReader for CoprocessorProofClient {
    async fn read_proofs(&self, queries: &[LeafQuery]) -> Result<ProofResponses, ProofReadError> {
        if queries.len() > MAX_LEAVES_PER_READ {
            return Err(ProofReadError::TooManyQueries {
                count: queries.len(),
            });
        }
        let body = serde_json::to_vec(&leaf_proof_request_body(queries)).map_err(|error| {
            ProofReadError::RequestEncoding {
                reason: error.to_string(),
            }
        })?;
        let answers = join_all(
            self.routes
                .iter()
                .map(|route| self.read_from(route, &body, queries.len())),
        )
        .await;

        let mut candidates = vec![Vec::new(); queries.len()];
        let mut answered = false;
        let mut failures = Vec::new();
        for answer in answers {
            match answer {
                Ok(outcomes) => {
                    answered = true;
                    for (query_candidates, outcome) in candidates.iter_mut().zip(outcomes) {
                        query_candidates.push(outcome);
                    }
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        if answered {
            Ok(ProofResponses {
                candidates,
                unavailable: (!failures.is_empty()).then(|| unavailable(failures.join("; "))),
            })
        } else {
            Err(unavailable(if failures.is_empty() {
                "no leaf proof endpoint is configured".to_string()
            } else {
                failures.join("; ")
            }))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// The shared spelling of the wire; the coprocessor pins its own against the same file.
    const LEAF_PROOFS_FIXTURE: &str = concat!(
        env!("CARGO_MANIFEST_DIR"),
        "/../../../solana/test-fixtures/leaf-proofs/leaf_proofs_v1.json"
    );

    fn fixture() -> serde_json::Value {
        serde_json::from_str(&std::fs::read_to_string(LEAF_PROOFS_FIXTURE).expect("read fixture"))
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
                encrypted_store: [0xAC; 32],
                handle: [0x10; 32],
                kind: LeafKind::Allowed { key: [0xA1; 32] },
            },
            LeafQuery {
                encrypted_store: [0xAC; 32],
                handle: [0x11; 32],
                kind: LeafKind::Public,
            },
        ];
        assert_eq!(
            serde_json::to_value(leaf_proof_request_body(&queries)).unwrap(),
            fixture["request"]
        );
    }

    #[test]
    fn every_fixture_answer_decodes() {
        let fixture = fixture();
        let body = serde_json::json!({ "proofs": fixture["proofs"] }).to_string();
        assert_eq!(
            parse_leaf_proof_response(&body, 4).expect("the fixture answers decode"),
            vec![
                LeafProofOutcome::Found {
                    leaf_index: 1,
                    leaf_count: 3,
                    siblings: vec![[0x5B; 32]],
                },
                LeafProofOutcome::NotFound { leaf_count: 3 },
                LeafProofOutcome::UnknownAccount,
                LeafProofOutcome::HistoryIncomplete,
            ]
        );
    }

    #[tokio::test]
    async fn malformed_and_oversized_proof_responses_are_recoverable_read_errors() {
        use mocktail::server::MockServer;
        let invalid_sibling = serde_json::json!({"proofs":[{"status":"found","leafIndex":0,"leafCount":1,"siblings":["00"]}]}).to_string();
        let too_many_siblings = serde_json::json!({"proofs":[{"status":"found","leafIndex":0,"leafCount":1,"siblings":vec!["00".repeat(32);65]}]}).to_string();
        for body in [
            "not JSON".to_owned(),
            "{\"proofs\":[]}".to_owned(),
            invalid_sibling,
            too_many_siblings,
            " ".repeat(300_000),
        ] {
            let mut server = MockServer::new_http("proof-response");
            server.mock(move |when, then| {
                when.post()
                    .path(LEAF_PROOFS_PATH)
                    .header("authorization", "Bearer secret");
                then.text(body.clone());
            });
            server.start().await.unwrap();
            let client = CoprocessorProofClient::new(
                &[server.base_url().unwrap().clone()],
                "secret".into(),
                reqwest::Client::new(),
            );
            let error = client
                .read_proofs(&[LeafQuery {
                    encrypted_store: [1; 32],
                    handle: [2; 32],
                    kind: LeafKind::Public,
                }])
                .await
                .unwrap_err();
            assert!(error.is_recoverable());
        }
    }
}
