//! The leaf-proof reader. A store's MMR leaves live in the coprocessors' record of host program
//! events; the account holds only the peaks. Each coprocessor is a source of proofs, never of
//! decisions: every answer is verified against the observed peaks.

use crate::core::config::{ApiKey, ProofRoute};
use alloy::primitives::B256;
use serde::{Deserialize, Serialize};
use solana_pubkey::Pubkey;
use std::future::Future;
use url::Url;

/// The coprocessor route that answers leaf-proof queries. The same literal as the coprocessor's
/// `LEAF_PROOFS_PATH`; the shared vectors pin the request and response shapes.
pub const LEAF_PROOFS_PATH: &str = "/v1/solana/leaf-proofs";

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

/// Implemented by [`CoprocessorProofClient`]; tests drive authorization with canned proofs.
pub trait HostProofReader: Send + Sync {
    /// How many coprocessors can be asked, in configured order.
    fn source_count(&self) -> usize;

    /// What coprocessor `source` answers, one outcome per query in query order. No outcome is
    /// trusted here.
    fn read_proofs(
        &self,
        source: usize,
        queries: &[LeafQuery],
    ) -> impl Future<Output = Result<Vec<LeafProofOutcome>, ProofReadError>> + Send;
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
    #[error("leaf proof read failed: {reason}")]
    Unavailable { reason: String },
    #[error("leaf proof read returned {returned} outcomes for {requested} queries")]
    ResponseLengthMismatch { requested: usize, returned: usize },
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

pub fn parse_leaf_proof_response(body: &str) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
    let response: LeafProofResponse = serde_json::from_str(body)
        .map_err(|error| unavailable(format!("response does not decode: {error}")))?;
    Ok(response.proofs)
}

fn unavailable(reason: String) -> ProofReadError {
    ProofReadError::Unavailable { reason }
}

/// The production reader: one authenticated `POST` to one coprocessor per read. Each
/// coprocessor receives only the key it issued.
#[derive(Clone, Debug)]
pub struct CoprocessorProofClient {
    routes: Vec<(Url, ApiKey)>,
    client: reqwest::Client,
}

impl CoprocessorProofClient {
    pub fn new(routes: &[ProofRoute], client: reqwest::Client) -> Self {
        let routes = routes
            .iter()
            .map(|route| {
                let mut url = route.url.clone();
                url.set_path(LEAF_PROOFS_PATH);
                (url, route.api_key.clone())
            })
            .collect();
        Self { routes, client }
    }

    async fn read_from(
        &self,
        (route, api_key): &(Url, ApiKey),
        body: &[u8],
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        let response = self
            .client
            .post(route.clone())
            .bearer_auth(api_key.expose())
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
        parse_leaf_proof_response(text)
    }
}

impl HostProofReader for CoprocessorProofClient {
    fn source_count(&self) -> usize {
        self.routes.len()
    }

    async fn read_proofs(
        &self,
        source: usize,
        queries: &[LeafQuery],
    ) -> Result<Vec<LeafProofOutcome>, ProofReadError> {
        let body = serde_json::to_vec(&leaf_proof_request_body(queries))
            .expect("a leaf proof request has no map keys or fallible fields");
        self.read_from(&self.routes[source], &body).await
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
            serde_json::to_value(leaf_proof_request_body(&queries)).unwrap(),
            fixture["request"]
        );
    }

    #[test]
    fn every_fixture_answer_decodes() {
        let fixture = fixture();
        let body = serde_json::json!({ "proofs": fixture["proofs"] }).to_string();
        assert_eq!(
            parse_leaf_proof_response(&body).expect("the fixture answers decode"),
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
                when.post()
                    .path(LEAF_PROOFS_PATH)
                    .header("authorization", "Bearer secret");
                then.text(body.clone());
            });
            server.start().await.unwrap();
            let client = CoprocessorProofClient::new(
                &[route(server.base_url().unwrap(), "secret")],
                reqwest::Client::new(),
            );
            client
                .read_proofs(
                    0,
                    &[LeafQuery {
                        encrypted_store: Pubkey::new_from_array([1; 32]),
                        handle: B256::new([2; 32]),
                        kind: LeafKind::Public,
                    }],
                )
                .await
                .expect_err("a malformed response is a failed read");
        }
    }

    fn route(url: &Url, api_key: &str) -> ProofRoute {
        ProofRoute {
            url: url.clone(),
            api_key: ApiKey::from(api_key.to_owned()),
        }
    }

    /// Each coprocessor issues its own key, so a route sends only the key configured with it, and
    /// neither key reaches the client's debug output.
    #[tokio::test]
    async fn each_coprocessor_receives_only_its_own_key() {
        use mocktail::server::MockServer;
        let mut servers = Vec::new();
        for key in ["first-key", "second-key"] {
            let mut server = MockServer::new_http(key);
            server.mock(move |when, then| {
                when.post()
                    .path(LEAF_PROOFS_PATH)
                    .header("authorization", format!("Bearer {key}"));
                then.text(r#"{"proofs":[{"status":"notFound","leafCount":0}]}"#);
            });
            server.start().await.unwrap();
            servers.push(server);
        }
        let client = CoprocessorProofClient::new(
            &[
                route(servers[0].base_url().unwrap(), "first-key"),
                route(servers[1].base_url().unwrap(), "second-key"),
            ],
            reqwest::Client::new(),
        );
        let query = [LeafQuery {
            encrypted_store: Pubkey::new_from_array([1; 32]),
            handle: B256::new([2; 32]),
            kind: LeafKind::Public,
        }];

        for source in 0..2 {
            client
                .read_proofs(source, &query)
                .await
                .expect("each coprocessor accepts its own key");
        }
        let debug = format!("{client:?}");
        assert!(
            !debug.contains("first-key") && !debug.contains("second-key"),
            "{debug}"
        );
    }
}
