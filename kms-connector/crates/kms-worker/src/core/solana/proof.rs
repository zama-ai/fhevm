//! The leaf-proof reader: the one place in the authorization path that talks to the
//! coprocessors' leaf record.
//!
//! An encrypted state stores nothing about who may decrypt it; every decrypt permission
//! ever sealed on it is a leaf of the account's MMR, and the account holds only the peaks. The
//! leaf and its sibling path live in the coprocessors' record of the host program's events, so
//! the connector fetches them from there and verifies them against the peaks it observed on
//! chain. The record is a source of proofs, never of decisions: a proof either verifies against
//! the account's own peaks or it does not, and nothing the service says about a leaf is trusted
//! on its own.
//!
//! Each query retains all responding coprocessors' candidates. The binding layer verifies them
//! against its chain observation before deciding whether another read is necessary.

use crate::core::solana_acl::{HandleBytes, SolanaPubkeyBytes};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use std::future::Future;
use url::Url;

/// The coprocessor route that answers leaf-proof queries. The same literal as the coprocessor's
/// `LEAF_PROOFS_PATH`; the shared vectors pin the request and response shapes.
pub const LEAF_PROOFS_PATH: &str = "/v1/solana/leaf-proofs";

/// Upper bound on queries per read, the coprocessor's cap. A request's batch never reaches it:
/// one query per entry and at most `MAX_REQUEST_HANDLES` entries.
const MAX_LEAVES_PER_READ: usize = 64;

/// Which leaf a query asks for.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub enum LeafKind {
    /// A historical-access leaf: `key` was allowed on the handle.
    Allowed {
        /// The key the leaf names.
        key: SolanaPubkeyBytes,
    },
    /// A public-decrypt leaf: the handle was made public.
    Public,
}

/// One leaf to prove.
#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct LeafQuery {
    /// The encrypted state whose MMR holds the leaf.
    pub encrypted_state: SolanaPubkeyBytes,
    /// The handle the leaf names.
    pub handle: HandleBytes,
    /// Which leaf.
    pub kind: LeafKind,
}

/// What the record said about one query.
#[derive(Clone, PartialEq, Eq, Debug)]
pub enum LeafProofOutcome {
    /// The leaf is recorded. `leaf_count` is the history the record had sealed when it built the
    /// proof; the verifier checks the sibling path against the on-chain peaks, not this number.
    Found {
        /// The leaf's position.
        leaf_index: u64,
        /// How many leaves the record had sealed on this account.
        leaf_count: u64,
        /// The sibling path.
        siblings: Vec<[u8; 32]>,
    },
    /// The record knows the account and has no such leaf in the history it has sealed.
    NotFound {
        /// How many leaves the record had sealed on this account.
        leaf_count: u64,
    },
    /// The record has never seen this account.
    UnknownAccount,
    /// The record's history for this account has a gap it cannot close, so it can answer nothing
    /// about the account until it is rebuilt.
    HistoryIncomplete,
}

/// The single proof-reading abstraction of the authorization path.
///
/// Authorization is generic over it for the same reason it is generic over the state reader: a
/// test drives the pipeline against canned proofs and counts the reads. Production has exactly one
/// implementation ([`HttpHostProofReader`]).
pub trait HostProofReader: Send + Sync {
    /// One list of peer candidates per query, in query order. No candidate is trusted here.
    fn read_proofs(
        &self,
        queries: &[LeafQuery],
    ) -> impl Future<Output = Result<Vec<Vec<LeafProofOutcome>>, ProofReadError>> + Send;
}

/// An ordered, duplicate-free batch of queries and their verification context.
///
/// Repeated queries retain the first context. Callers derive that context from the same
/// validated snapshot, so deduplication cannot change what authorizes the leaf.
#[derive(Clone, PartialEq, Eq, Debug)]
pub struct ProofBatch<T>(Vec<(LeafQuery, T)>);

impl<T> ProofBatch<T> {
    /// Collects queries with their context, preserving first-seen order and dropping repeats.
    pub fn new(queries: impl IntoIterator<Item = (LeafQuery, T)>) -> Self {
        let mut ordered: Vec<(LeafQuery, T)> = Vec::new();
        for (query, context) in queries {
            if !ordered.iter().any(|(planned, _)| *planned == query) {
                ordered.push((query, context));
            }
        }
        Self(ordered)
    }

    /// The queries, in read order.
    pub fn queries(&self) -> Vec<LeafQuery> {
        self.0.iter().map(|(query, _)| *query).collect()
    }

    /// Verification contexts, in the same order as the queries.
    pub fn contexts(&self) -> impl Iterator<Item = &T> {
        self.0.iter().map(|(_, context)| context)
    }

    /// Where `query` sits in the batch, if it was planned.
    pub fn position(&self, query: &LeafQuery) -> Option<usize> {
        self.0.iter().position(|(planned, _)| planned == query)
    }
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
    /// The locally constructed request could not be encoded; retrying cannot repair it.
    #[error("leaf proof request serialization failed: {reason}")]
    RequestEncoding {
        /// Serialization diagnostic.
        reason: String,
    },
    /// No coprocessor answered.
    #[error("leaf proof read failed: {reason}")]
    Unavailable {
        /// What went wrong, for the log.
        reason: String,
    },
    /// The answer did not carry one outcome per query.
    #[error("leaf proof read returned {returned} outcomes for {requested} queries")]
    ResponseLengthMismatch {
        /// Queries sent.
        requested: usize,
        /// Outcomes returned.
        returned: usize,
    },
    /// The batch exceeds the coprocessor's cap. Unreachable from a validated request; a defect in
    /// batch planning, not a property of the record.
    #[error("leaf proof batch of {count} queries exceeds the {MAX_LEAVES_PER_READ}-query cap")]
    TooManyQueries {
        /// Queries planned.
        count: usize,
    },
}

// ------------------------------------------------------------------------------------------
// The HTTP transport. Field names mirror the coprocessor's `http_server.rs`.

#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LeafQueryWire {
    encrypted_state: String,
    handle: String,
    kind: LeafKindWire,
    #[serde(skip_serializing_if = "Option::is_none")]
    key: Option<String>,
}

#[derive(Serialize)]
#[serde(rename_all = "snake_case")]
enum LeafKindWire {
    Allowed,
    Public,
}

#[derive(Serialize)]
struct LeafProofRequestWire {
    leaves: Vec<LeafQueryWire>,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase", tag = "status")]
enum LeafProofWire {
    #[serde(rename_all = "camelCase")]
    Found {
        leaf_index: u64,
        leaf_count: u64,
        siblings: Vec<String>,
    },
    #[serde(rename_all = "camelCase")]
    NotFound {
        leaf_count: u64,
    },
    UnknownAccount,
    HistoryIncomplete,
}

#[derive(Deserialize)]
struct LeafProofResponseWire {
    proofs: Vec<LeafProofWire>,
}

/// Builds the request body the coprocessor route expects. Split out so the shape is assertable
/// against the shared vectors without a live service.
pub fn leaf_proof_request_body(queries: &[LeafQuery]) -> impl Serialize {
    let leaves = queries
        .iter()
        .map(|query| LeafQueryWire {
            encrypted_state: alloy::hex::encode(query.encrypted_state),
            handle: alloy::hex::encode(query.handle),
            kind: match query.kind {
                LeafKind::Allowed { .. } => LeafKindWire::Allowed,
                LeafKind::Public => LeafKindWire::Public,
            },
            key: match query.kind {
                LeafKind::Allowed { key } => Some(alloy::hex::encode(key)),
                LeafKind::Public => None,
            },
        })
        .collect();
    LeafProofRequestWire { leaves }
}

/// Parses one coprocessor's answer to a batch.
pub fn parse_leaf_proof_response(
    body: &str,
    requested: usize,
) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
    let response: LeafProofResponseWire = serde_json::from_str(body)
        .map_err(|error| unavailable(format!("response does not decode: {error}")))?;
    check_length(requested, response.proofs.len())?;
    response
        .proofs
        .into_iter()
        .map(|proof| match proof {
            LeafProofWire::Found {
                leaf_index,
                leaf_count,
                siblings,
            } => Ok(LeafProofOutcome::Found {
                leaf_index,
                leaf_count,
                siblings: siblings
                    .iter()
                    .map(|sibling| decode_hex32(sibling))
                    .collect::<Result<_, _>>()?,
            }),
            LeafProofWire::NotFound { leaf_count } => Ok(LeafProofOutcome::NotFound { leaf_count }),
            LeafProofWire::UnknownAccount => Ok(LeafProofOutcome::UnknownAccount),
            LeafProofWire::HistoryIncomplete => Ok(LeafProofOutcome::HistoryIncomplete),
        })
        .collect()
}

fn decode_hex32(text: &str) -> Result<[u8; 32], ProofReadError> {
    let bytes = alloy::hex::decode(text)
        .map_err(|error| unavailable(format!("sibling '{text}' is not hex: {error}")))?;
    bytes.try_into().map_err(|bytes: Vec<u8>| {
        unavailable(format!(
            "sibling '{text}' is {} bytes, expected 32",
            bytes.len()
        ))
    })
}

fn unavailable(reason: String) -> ProofReadError {
    ProofReadError::Unavailable { reason }
}

/// The production reader: one authenticated `POST` per coprocessor per read, grouped per query.
#[derive(Clone, Debug)]
pub struct HttpHostProofReader {
    routes: Vec<Url>,
    api_key: String,
    client: reqwest::Client,
}

impl HttpHostProofReader {
    /// Binds the reader to the coprocessors' base URLs and the bearer key they expect.
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
        let text = response
            .text()
            .await
            .map_err(|error| unavailable(format!("{route}: body could not be read: {error}")))?;
        if !status.is_success() {
            return Err(unavailable(format!("{route}: HTTP {status}: {text}")));
        }
        parse_leaf_proof_response(&text, requested)
    }
}

impl HostProofReader for HttpHostProofReader {
    async fn read_proofs(
        &self,
        queries: &[LeafQuery],
    ) -> Result<Vec<Vec<LeafProofOutcome>>, ProofReadError> {
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
            Ok(candidates)
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
                encrypted_state: [0xAC; 32],
                handle: [0x10; 32],
                kind: LeafKind::Allowed { key: [0xA1; 32] },
            },
            LeafQuery {
                encrypted_state: [0xAC; 32],
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
}
