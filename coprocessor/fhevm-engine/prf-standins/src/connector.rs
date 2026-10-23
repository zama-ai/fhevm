//! Stand-in for the KMS Connector side of RFC-038: the retrieval algorithm of one label, signed
//! with a KMS tx-sender key.

use alloy::{
    primitives::{keccak256, Address, B256},
    signers::local::PrivateKeySigner,
    sol_types::Eip712Domain,
};
use ciphertext_attestation::{
    prf::PRF_ATTESTATION_HEADER, ConsensusCheckError, ConsensusRound, CoprocessorEntry,
    PrfOutputAttestation, PrfOutputRef,
};
use fhevm_sig_auth::{FhevmSigHeader, PathAndQuery};
use rand::seq::SliceRandom;
use reqwest::{header::AUTHORIZATION, Method, StatusCode};
use serde::Serialize;
use std::{
    num::NonZeroUsize,
    sync::Arc,
    time::{Duration, Instant},
};
use tokio::{sync::mpsc, task::JoinSet};
use tracing::debug;

#[derive(Clone, Debug)]
pub struct Coprocessor {
    /// Base URL, without the `/v1/...` path.
    pub url: String,
    /// The registered signer: the audience of the requests, and the expected attestation signer.
    pub signer: Address,
}

pub struct Connector {
    pub key: PrivateKeySigner,
    pub domain: Eip712Domain,
    pub coprocessors: Vec<Coprocessor>,
    pub threshold: NonZeroUsize,
    pub http: reqwest::Client,
    pub header_validity: Duration,
    /// RFC-038 `PRF_RETRIEVAL_DEADLINE`.
    pub deadline: Duration,
    /// Wait before re-sending after a connection error or a retryable status without `Retry-After`.
    pub backoff: Duration,
}

pub struct Retrieval {
    pub prf: u16,
    pub label: B256,
    pub pad_length: u16,
    /// The coprocessor that gets the `GET`. Random when unset.
    pub get_from: Option<usize>,
    /// The `expires` of the first headers, to start from stale cached headers.
    pub first_expires: Option<u64>,
}

impl Retrieval {
    pub fn path(&self) -> String {
        format!(
            "/v1/prf/{}/labels/{}/{}",
            self.prf,
            hex::encode(self.label),
            self.pad_length
        )
    }
}

#[derive(Debug, Serialize)]
pub struct Report {
    pub connector: Address,
    pub label: B256,
    pub pad_length: u16,
    pub ok: bool,
    /// `no_quorum`, `deadline` or `no_matching_body`.
    pub error: Option<&'static str>,
    pub quorum_digest: Option<B256>,
    pub get_from: usize,
    pub body_from: Option<usize>,
    pub fallback: bool,
    pub resigns: u32,
    pub replies: Vec<ReplyReport>,
    pub elapsed_ms: u64,
}

#[derive(Debug, Clone, Serialize)]
pub struct ReplyReport {
    pub coprocessor: usize,
    pub method: &'static str,
    /// The last status, or `None` for a connection error.
    pub status: Option<u16>,
    pub code: Option<String>,
    pub attempts: u32,
    /// `valid`, `rejected` or `missing`, for a `200`.
    pub attestation: Option<&'static str>,
    pub digest: Option<B256>,
}

/// The end of one request loop to one coprocessor.
struct Answer {
    coprocessor: usize,
    report: ReplyReport,
    resigns: u32,
    attestation: Option<PrfOutputAttestation>,
    body: Option<bytes::Bytes>,
}

/// The RFC-033 error body.
#[derive(Debug, serde::Deserialize, Serialize)]
pub struct ErrorBody {
    pub code: String,
    pub retryable: bool,
}

impl Connector {
    /// Runs the RFC-038 retrieval algorithm for one label.
    pub async fn retrieve(self: &Arc<Self>, retrieval: &Retrieval) -> Report {
        let started = Instant::now();
        let deadline = tokio::time::Instant::now() + self.deadline;
        let get_from = retrieval
            .get_from
            .unwrap_or_else(|| rand::random_range(0..self.coprocessors.len()));
        let path = retrieval.path();
        let subject = PrfOutputRef::new(retrieval.prf, retrieval.label, retrieval.pad_length);
        let mut round = ConsensusRound::<PrfOutputAttestation>::open(
            subject,
            self.coprocessors.iter().map(|c| CoprocessorEntry {
                tx_sender: Address::ZERO,
                signer: c.signer,
                bucket: String::new(),
            }),
            self.threshold,
        );
        let mut report = Report {
            connector: self.key.address(),
            label: retrieval.label,
            pad_length: retrieval.pad_length,
            ok: false,
            error: None,
            quorum_digest: None,
            get_from,
            body_from: None,
            fallback: false,
            resigns: 0,
            replies: Vec::new(),
            elapsed_ms: 0,
        };

        let (tx, mut rx) = mpsc::unbounded_channel();
        let mut tasks = JoinSet::new();
        for i in 0..self.coprocessors.len() {
            let method = if i == get_from {
                Method::GET
            } else {
                Method::HEAD
            };
            let (connector, path, tx) = (self.clone(), path.clone(), tx.clone());
            let first_expires = retrieval.first_expires;
            tasks.spawn(async move {
                let _ = tx.send(connector.ask(i, method, &path, first_expires).await);
            });
        }
        drop(tx);

        let mut get_body = None;
        let outcome = loop {
            let answer = match tokio::time::timeout_at(deadline, rx.recv()).await {
                Ok(Some(answer)) => answer,
                Ok(None) => break round.clone().close(),
                Err(_) => {
                    report.error = Some("deadline");
                    break round.clone().close();
                }
            };
            report.resigns += answer.resigns;
            let signer = self.coprocessors[answer.coprocessor].signer;
            if answer.coprocessor == get_from {
                get_body = answer.body.clone();
            }
            let decided = match &answer.attestation {
                Some(attestation) => round.record_attestation(signer, attestation),
                None => round.record_no_reply(signer),
            };
            let mut reply = answer.report;
            if round.rejected().contains(&signer) {
                reply.attestation = Some("rejected");
            }
            report.replies.push(reply);
            if let Some(outcome) = decided {
                break outcome;
            }
        };

        let resolved = match outcome {
            Ok(resolved) => resolved,
            Err(err) => {
                report.error.get_or_insert(match err {
                    ConsensusCheckError::NotReachedThisRound(_)
                    | ConsensusCheckError::Unreachable(_) => "no_quorum",
                });
                report.elapsed_ms = started.elapsed().as_millis() as u64;
                return report;
            }
        };
        report.error = None;
        let digest = resolved.material;
        report.quorum_digest = Some(digest);

        if get_body
            .as_ref()
            .is_some_and(|body| keccak256(body) == digest)
        {
            report.body_from = Some(get_from);
        } else {
            // The GET target failed, is slow, or served another body: a quorum member answers at once.
            report.fallback = true;
            let mut members: Vec<usize> = resolved
                .winners
                .iter()
                .filter_map(|w| self.coprocessors.iter().position(|c| c.signer == w.signer))
                .filter(|&i| i != get_from)
                .collect();
            members.shuffle(&mut rand::rng());
            for i in members {
                let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
                let Ok(answer) =
                    tokio::time::timeout(remaining, self.ask(i, Method::GET, &path, None)).await
                else {
                    break;
                };
                report.resigns += answer.resigns;
                report.replies.push(answer.report);
                if answer.body.is_some_and(|body| keccak256(&body) == digest) {
                    report.body_from = Some(i);
                    break;
                }
            }
        }
        report.ok = report.body_from.is_some();
        if !report.ok {
            report.error = Some(if tokio::time::Instant::now() >= deadline {
                "deadline"
            } else {
                "no_matching_body"
            });
        }
        report.elapsed_ms = started.elapsed().as_millis() as u64;
        report
    }

    /// Sends `method` to coprocessor `i` until it answers `200` or an error that RFC-038 step 6
    /// does not retry. Retries connection errors, `429`, `502`, `504`, `401 auth_unknown_signer`,
    /// and `401 auth_expired` after re-signing. Like the RFC, it reads the status and `code` only.
    async fn ask(
        &self,
        i: usize,
        method: Method,
        path: &str,
        first_expires: Option<u64>,
    ) -> Answer {
        let coprocessor = &self.coprocessors[i];
        let mut expires = first_expires;
        let mut resigns = 0;
        let mut report = ReplyReport {
            coprocessor: i,
            method: if method == Method::GET { "GET" } else { "HEAD" },
            status: None,
            code: None,
            attempts: 0,
            attestation: None,
            digest: None,
        };
        loop {
            report.attempts += 1;
            let header = self.sign(coprocessor.signer, path, expires).await;
            let sent = self
                .http
                .request(method.clone(), format!("{}{path}", coprocessor.url))
                .header(AUTHORIZATION, header.to_string())
                .send()
                .await;
            let response = match sent {
                Ok(response) => response,
                Err(err) => {
                    debug!(coprocessor = i, error = %err, "request failed");
                    report.status = None;
                    report.code = Some("connection_error".into());
                    tokio::time::sleep(self.backoff).await;
                    continue;
                }
            };
            let status = response.status();
            report.status = Some(status.as_u16());
            if status == StatusCode::OK {
                let attestation = response
                    .headers()
                    .get(PRF_ATTESTATION_HEADER)
                    .and_then(|v| v.to_str().ok())
                    .and_then(|v| serde_json::from_str::<PrfOutputAttestation>(v).ok());
                report.code = None;
                report.attestation = Some(if attestation.is_some() {
                    "valid"
                } else {
                    "missing"
                });
                report.digest = attestation.as_ref().map(|a| a.sns_ciphertext_digest);
                let body = if method == Method::GET {
                    response.bytes().await.ok()
                } else {
                    None
                };
                return Answer {
                    coprocessor: i,
                    report,
                    resigns,
                    attestation,
                    body,
                };
            }
            let retry_after = response
                .headers()
                .get(reqwest::header::RETRY_AFTER)
                .and_then(|v| v.to_str().ok())
                .and_then(|v| v.parse::<u64>().ok())
                .map(Duration::from_secs);
            // A HEAD answer has no body: only its status tells what went wrong (RFC-038 gap).
            let error = response.json::<ErrorBody>().await.ok();
            report.code = error.as_ref().map(|e| e.code.clone());
            let expired = match &error {
                Some(e) => e.code == "auth_expired",
                // Re-sign once: a fresh header settles whether the 401 was an expiry.
                None => status == StatusCode::UNAUTHORIZED && expires.is_some(),
            };
            let retryable = matches!(
                status,
                StatusCode::TOO_MANY_REQUESTS
                    | StatusCode::BAD_GATEWAY
                    | StatusCode::GATEWAY_TIMEOUT
            ) || error
                .as_ref()
                .is_some_and(|e| e.code == "auth_unknown_signer");
            if expired {
                resigns += 1;
                expires = None;
            } else if retryable {
                tokio::time::sleep(retry_after.unwrap_or(self.backoff)).await;
            } else {
                return Answer {
                    coprocessor: i,
                    report,
                    resigns,
                    attestation: None,
                    body: None,
                };
            }
        }
    }

    async fn sign(&self, audience: Address, path: &str, expires: Option<u64>) -> FhevmSigHeader {
        let target = PathAndQuery::try_from(path).expect("a request path");
        let expires = expires.unwrap_or_else(|| {
            fhevm_sig_auth::unix_now().saturating_add(self.header_validity.as_secs())
        });
        fhevm_sig_auth::sign(&self.key, &self.domain, audience, &target, b"", expires)
            .await
            .expect("a local key signs")
    }

    /// One signed request, without retries: for the endpoint's error cases.
    pub async fn probe(
        &self,
        i: usize,
        method: Method,
        path: &str,
        expires: Option<u64>,
    ) -> anyhow::Result<(u16, Option<ErrorBody>, Option<String>)> {
        let coprocessor = &self.coprocessors[i];
        let header = self
            .sign(coprocessor.signer, path_without_query(path), expires)
            .await;
        let response = self
            .http
            .request(method, format!("{}{path}", coprocessor.url))
            .header(AUTHORIZATION, header.to_string())
            .send()
            .await?;
        let status = response.status().as_u16();
        let retry_after = response
            .headers()
            .get(reqwest::header::RETRY_AFTER)
            .and_then(|v| v.to_str().ok())
            .map(str::to_owned);
        let error = response.json::<ErrorBody>().await.ok();
        Ok((status, error, retry_after))
    }
}

/// A query string is not signed: the endpoint must refuse it.
fn path_without_query(path: &str) -> &str {
    path.split_once('?').map_or(path, |(path, _)| path)
}
