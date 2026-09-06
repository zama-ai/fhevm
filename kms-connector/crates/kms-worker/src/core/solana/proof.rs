//! The leaf-proof reader: the one place in the authorization path that talks to the
//! coprocessors' leaf record.
//!
//! An encrypted value account stores nothing about who may decrypt it; every decrypt permission
//! ever sealed on it is a leaf of the account's MMR, and the account holds only the peaks. The
//! leaf and its sibling path live in the coprocessors' record of the host program's events, so
//! the connector fetches them from there and verifies them against the peaks it observed on
//! chain. The record is a source of proofs, never of decisions: a proof either verifies against
//! the account's own peaks or it does not, and nothing the service says about a leaf is trusted
//! on its own.
//!
//! ## Fan-out
//!
//! Every configured coprocessor is asked, concurrently, and the answers are merged per query
//! ([`merge_outcomes`]): a proof beats no proof, more history beats less. A coprocessor that is
//! behind cannot sink a request that another can serve, and one that is unreachable is ignored
//! for as long as any other answers. Only when none does is the read a failure, and it is a
//! transient one.
//!
//! ## One read, one retry
//!
//! A request's proofs are planned as one batch and read once. The account's own `leaf_count`
//! says how much history a proof must have seen, so a record that answers with less history than
//! the chain has is known to be behind, and the batch is read once more before any verdict
//! ([`read_proofs_with_one_retry`]). That is the whole of the retry policy inside a request;
//! beyond it the request is rejected retryably and the ordinary attempt budget decides.

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
pub const MAX_LEAVES_PER_READ: usize = 64;

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
    /// The encrypted value account whose MMR holds the leaf.
    pub encrypted_value_account: SolanaPubkeyBytes,
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

impl LeafProofOutcome {
    /// The history the record had sealed, where it said.
    fn leaf_count(&self) -> Option<u64> {
        match self {
            Self::Found { leaf_count, .. } | Self::NotFound { leaf_count } => Some(*leaf_count),
            Self::UnknownAccount | Self::HistoryIncomplete => None,
        }
    }

    /// Whether the record had sealed less history than the chain shows. A proof from such a
    /// record may still verify — an append merges only some peaks — but an absence from it says
    /// nothing, and the read is repeated once before either is judged.
    pub fn is_behind(&self, live_leaf_count: u64) -> bool {
        match self {
            Self::Found { leaf_count, .. } | Self::NotFound { leaf_count } => {
                *leaf_count < live_leaf_count
            }
            Self::UnknownAccount => true,
            Self::HistoryIncomplete => false,
        }
    }

    /// How much this outcome is worth against another for the same query: a proof beats no
    /// proof, more history beats less, and an answer beats a record that cannot answer.
    fn rank(&self) -> (u8, u64) {
        let tier = match self {
            Self::Found { .. } => 3,
            Self::NotFound { .. } => 2,
            Self::UnknownAccount => 1,
            Self::HistoryIncomplete => 0,
        };
        (tier, self.leaf_count().unwrap_or(0))
    }
}

/// Picks the better of two answers to one query. Stated once and used for both merges — across
/// coprocessors and across the retry — so the two cannot prefer differently.
pub fn merge_outcomes(current: LeafProofOutcome, other: LeafProofOutcome) -> LeafProofOutcome {
    if other.rank() > current.rank() {
        other
    } else {
        current
    }
}

/// The single proof-reading abstraction of the authorization path.
///
/// Authorization is generic over it for the same reason it is generic over the state reader: a
/// test drives the pipeline against canned proofs and counts the reads. Production has exactly one
/// implementation ([`HttpHostProofReader`]).
pub trait HostProofReader: Send + Sync {
    /// Answers every query, in order.
    fn read_proofs(
        &self,
        queries: &[LeafQuery],
    ) -> impl Future<Output = Result<Vec<LeafProofOutcome>, ProofReadError>> + Send;
}

/// An ordered, duplicate-free batch of queries, with the position of each.
///
/// Duplicates collapse so a request naming one handle twice costs one query, and the position
/// lookup is what lets the per-entry rules find their outcome again.
#[derive(Clone, PartialEq, Eq, Debug, Default)]
pub struct ProofBatch(Vec<LeafQuery>);

impl ProofBatch {
    /// Collects queries, preserving first-seen order and dropping repeats.
    pub fn new(queries: impl IntoIterator<Item = LeafQuery>) -> Self {
        let mut ordered: Vec<LeafQuery> = Vec::new();
        for query in queries {
            if !ordered.contains(&query) {
                ordered.push(query);
            }
        }
        Self(ordered)
    }

    /// The queries, in read order.
    pub fn queries(&self) -> &[LeafQuery] {
        &self.0
    }

    /// Where `query` sits in the batch, if it was planned.
    pub fn position(&self, query: &LeafQuery) -> Option<usize> {
        self.0.iter().position(|planned| planned == query)
    }
}

/// Reads a batch, and reads it once more if any answer is behind the chain.
///
/// `live_leaf_count` gives, per query position, the leaf count the on-chain account showed at the
/// deciding observation. The second read replaces an answer only where it is the better one
/// ([`merge_outcomes`]), so a repeat that lands on a coprocessor further behind cannot make things
/// worse.
pub async fn read_proofs_with_one_retry<P: HostProofReader>(
    reader: &P,
    batch: &ProofBatch,
    live_leaf_count: impl Fn(usize) -> u64,
) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
    let mut outcomes = reader.read_proofs(batch.queries()).await?;
    check_length(batch.queries().len(), outcomes.len())?;
    let behind = outcomes
        .iter()
        .enumerate()
        .any(|(position, outcome)| outcome.is_behind(live_leaf_count(position)));
    if behind {
        let again = reader.read_proofs(batch.queries()).await?;
        check_length(batch.queries().len(), again.len())?;
        outcomes = outcomes
            .into_iter()
            .zip(again)
            .map(|(current, other)| merge_outcomes(current, other))
            .collect();
    }
    Ok(outcomes)
}

fn check_length(requested: usize, returned: usize) -> Result<(), ProofReadError> {
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
    encrypted_value_account: String,
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
pub fn leaf_proof_request_body(queries: &[LeafQuery]) -> serde_json::Value {
    let leaves = queries
        .iter()
        .map(|query| LeafQueryWire {
            encrypted_value_account: alloy::hex::encode(query.encrypted_value_account),
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
    serde_json::to_value(LeafProofRequestWire { leaves }).expect("plain strings serialize")
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

/// The production reader: one authenticated `POST` per coprocessor per read, merged per query.
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
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        if queries.len() > MAX_LEAVES_PER_READ {
            return Err(ProofReadError::TooManyQueries {
                count: queries.len(),
            });
        }
        let body = serde_json::to_vec(&leaf_proof_request_body(queries))
            .map_err(|error| unavailable(format!("request body does not serialize: {error}")))?;
        let answers = join_all(
            self.routes
                .iter()
                .map(|route| self.read_from(route, &body, queries.len())),
        )
        .await;

        let mut merged: Option<Vec<LeafProofOutcome>> = None;
        let mut failures = Vec::new();
        for answer in answers {
            match answer {
                Ok(outcomes) => {
                    merged = Some(match merged {
                        None => outcomes,
                        Some(current) => current
                            .into_iter()
                            .zip(outcomes)
                            .map(|(current, other)| merge_outcomes(current, other))
                            .collect(),
                    })
                }
                Err(error) => failures.push(error.to_string()),
            }
        }
        merged.ok_or_else(|| {
            unavailable(if failures.is_empty() {
                "no leaf proof endpoint is configured".to_string()
            } else {
                failures.join("; ")
            })
        })
    }
}
