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
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
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
    /// The fallback policy. `None`, as RFC-038 step 5 reads: the GET target counts once its body
    /// is in, and the connector falls back if quorum forms before. `Some(bound)`: the target counts
    /// from its headers, and when it is in the quorum group the connector waits up to `bound` for
    /// its body (the optimistic fetch).
    pub body_wait: Option<Duration>,
    /// Test knob (QA row R7): the first fallback GET counts as failed, as after a restart.
    pub fail_first_fallback: bool,
}

/// What a request loop of `retrieve` sends back.
enum Event {
    /// The GET target's attestation, when its headers arrive and before its body.
    Headers {
        coprocessor: usize,
        attestation: Option<PrfOutputAttestation>,
    },
    Done(Answer),
}

/// Options of [`Connector::probe`], to build requests a connector would not send.
#[derive(Default)]
pub struct ProbeOptions {
    pub expires: Option<u64>,
    /// Sign for this audience instead of the coprocessor's signer.
    pub audience: Option<Address>,
    /// Sign with this key instead of the connector's.
    pub key: Option<PrivateKeySigner>,
    /// Sign for this body digest.
    pub signed_body: Vec<u8>,
    /// Send this body.
    pub body: Option<Vec<u8>>,
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
    /// How long the optimistic policy waited for the GET target's body.
    pub body_wait_ms: Option<u64>,
    /// Body bytes received, including the part of a GET abandoned when the retrieval returned.
    pub bytes: usize,
    /// GETs sent: the target's, then one per fallback member tried.
    pub gets: u32,
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
    /// The answers that were retried, in order: `<status> <code>`, or `connection_error`.
    pub retried: Vec<String>,
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
            body_wait_ms: None,
            bytes: 0,
            gets: 1,
            resigns: 0,
            replies: Vec::new(),
            elapsed_ms: 0,
        };

        let (tx, mut rx) = mpsc::unbounded_channel();
        let received = Arc::new(AtomicUsize::new(0));
        let mut tasks = JoinSet::new();
        for i in 0..self.coprocessors.len() {
            let method = if i == get_from {
                Method::GET
            } else {
                Method::HEAD
            };
            let (connector, path, tx) = (self.clone(), path.clone(), tx.clone());
            let first_expires = retrieval.first_expires;
            let received = received.clone();
            tasks.spawn(async move {
                let headers = (i == get_from && connector.body_wait.is_some()).then_some(&tx);
                let answer = connector
                    .ask(i, method, &path, first_expires, headers, &received)
                    .await;
                let _ = tx.send(Event::Done(answer));
            });
        }
        drop(tx);

        let mut get_body = None;
        // The GET target's attestation counted from its headers, before its body.
        let mut target_counted = false;
        let outcome = loop {
            let event = match tokio::time::timeout_at(deadline, rx.recv()).await {
                Ok(Some(event)) => event,
                Ok(None) => break round.clone().close(),
                Err(_) => {
                    report.error = Some("deadline");
                    break round.clone().close();
                }
            };
            let decided = match event {
                Event::Headers {
                    coprocessor,
                    attestation,
                } => {
                    target_counted = true;
                    let signer = self.coprocessors[coprocessor].signer;
                    match &attestation {
                        Some(attestation) => round.record_attestation(signer, attestation),
                        None => round.record_no_reply(signer),
                    }
                }
                Event::Done(answer) => {
                    report.resigns += answer.resigns;
                    let signer = self.coprocessors[answer.coprocessor].signer;
                    let counted = answer.coprocessor == get_from && target_counted;
                    if answer.coprocessor == get_from {
                        get_body = answer.body.clone();
                    }
                    let decided = match (&answer.attestation, counted) {
                        (_, true) => None,
                        (Some(attestation), false) => round.record_attestation(signer, attestation),
                        (None, false) => round.record_no_reply(signer),
                    };
                    let mut reply = answer.report;
                    if round.rejected().contains(&signer) {
                        reply.attestation = Some("rejected");
                    }
                    report.replies.push(reply);
                    decided
                }
            };
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
                report.bytes = received.load(Ordering::Relaxed);
                report.elapsed_ms = started.elapsed().as_millis() as u64;
                return report;
            }
        };
        report.error = None;
        let digest = resolved.material;
        report.quorum_digest = Some(digest);

        let target_signer = self.coprocessors[get_from].signer;
        let target_in_quorum =
            target_counted && resolved.winners.iter().any(|w| w.signer == target_signer);
        if let (None, true, Some(bound)) = (&get_body, target_in_quorum, self.body_wait) {
            let waited = tokio::time::Instant::now();
            let until = deadline.min(waited + bound);
            while let Ok(Some(event)) = tokio::time::timeout_at(until, rx.recv()).await {
                if let Event::Done(answer) = event {
                    let target = answer.coprocessor == get_from;
                    if target {
                        get_body = answer.body;
                    }
                    report.replies.push(answer.report);
                    if target {
                        break;
                    }
                }
            }
            report.body_wait_ms = Some(waited.elapsed().as_millis() as u64);
        }

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
            for (n, i) in members.into_iter().enumerate() {
                report.gets += 1;
                if n == 0 && self.fail_first_fallback {
                    report.replies.push(ReplyReport {
                        coprocessor: i,
                        method: "GET",
                        status: None,
                        code: Some("simulated_failure".into()),
                        attempts: 0,
                        retried: Vec::new(),
                        attestation: None,
                        digest: None,
                    });
                    continue;
                }
                let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
                let Ok(answer) = tokio::time::timeout(
                    remaining,
                    self.ask(i, Method::GET, &path, None, None, &received),
                )
                .await
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
        report.bytes = received.load(Ordering::Relaxed);
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
        headers: Option<&mpsc::UnboundedSender<Event>>,
        received: &AtomicUsize,
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
            retried: Vec::new(),
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
                    report.retried.push("connection_error".into());
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
                if let Some(headers) = headers {
                    let _ = headers.send(Event::Headers {
                        coprocessor: i,
                        attestation: attestation.clone(),
                    });
                }
                let body = if method == Method::GET {
                    read_body(response, received).await.map(Into::into)
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
            let next = after_error(status, report.code.as_deref(), expires.is_some());
            if next != Next::Stop {
                report.retried.push(
                    format!(
                        "{} {}",
                        status.as_u16(),
                        report.code.as_deref().unwrap_or("")
                    )
                    .trim()
                    .to_owned(),
                );
            }
            match next {
                Next::Resign => {
                    resigns += 1;
                    expires = None;
                }
                Next::Retry => tokio::time::sleep(retry_after.unwrap_or(self.backoff)).await,
                Next::Stop => {
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

    /// The request loop of the retrieval to one coprocessor, alone, until the deadline.
    pub async fn ask_one(&self, i: usize, method: Method, path: &str) -> Option<ReplyReport> {
        let received = AtomicUsize::new(0);
        tokio::time::timeout(
            self.deadline,
            self.ask(i, method, path, None, None, &received),
        )
        .await
        .ok()
        .map(|answer| answer.report)
    }

    /// One signed request, without retries: for the endpoint's error cases. The path is signed and
    /// sent as given, without its query string in the signature.
    pub async fn probe(
        &self,
        i: usize,
        method: Method,
        path: &str,
        options: &ProbeOptions,
    ) -> anyhow::Result<(u16, Option<ErrorBody>, Option<String>)> {
        let coprocessor = &self.coprocessors[i];
        let target = PathAndQuery::try_from(path_without_query(path))?;
        let expires = options.expires.unwrap_or_else(|| {
            fhevm_sig_auth::unix_now().saturating_add(self.header_validity.as_secs())
        });
        let header = fhevm_sig_auth::sign(
            options.key.as_ref().unwrap_or(&self.key),
            &self.domain,
            options.audience.unwrap_or(coprocessor.signer),
            &target,
            &options.signed_body,
            expires,
        )
        .await?;
        let mut request = self
            .http
            .request(method, format!("{}{path}", coprocessor.url))
            .header(AUTHORIZATION, header.to_string());
        if let Some(body) = &options.body {
            request = request.body(body.clone());
        }
        let response = request.send().await?;
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

/// Reads a body and counts its bytes as they arrive, so that an abandoned transfer still counts.
async fn read_body(mut response: reqwest::Response, received: &AtomicUsize) -> Option<Vec<u8>> {
    let mut body = Vec::new();
    while let Some(chunk) = response.chunk().await.ok()? {
        received.fetch_add(chunk.len(), Ordering::Relaxed);
        body.extend_from_slice(&chunk);
    }
    Some(body)
}

/// A query string is not signed: the endpoint must refuse it.
fn path_without_query(path: &str) -> &str {
    path.split_once('?').map_or(path, |(path, _)| path)
}

#[derive(Debug, PartialEq, Eq)]
enum Next {
    Resign,
    Retry,
    Stop,
}

/// RFC-038 step 6 for an error answer: `code` is `None` for a HEAD, and `stale` says whether the
/// header was a cached one that a fresh signature could fix.
fn after_error(status: StatusCode, code: Option<&str>, stale: bool) -> Next {
    match (status, code) {
        (StatusCode::UNAUTHORIZED, Some("auth_expired")) => Next::Resign,
        // Re-sign once: a fresh header settles whether a bodyless 401 was an expiry.
        (StatusCode::UNAUTHORIZED, None) if stale => Next::Resign,
        (StatusCode::UNAUTHORIZED, Some("auth_unknown_signer")) => Next::Retry,
        (
            StatusCode::TOO_MANY_REQUESTS | StatusCode::BAD_GATEWAY | StatusCode::GATEWAY_TIMEOUT,
            _,
        ) => Next::Retry,
        _ => Next::Stop,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn follows_rfc_038_step_6() {
        for (status, code, stale, next) in [
            (500, Some("prf_evaluation_failed"), false, Next::Stop),
            (500, None, false, Next::Stop),
            (502, Some("upstream_transient"), false, Next::Retry),
            (502, None, false, Next::Retry),
            (504, Some("timeout"), false, Next::Retry),
            (504, None, false, Next::Retry),
            (429, Some("concurrency_limited"), false, Next::Retry),
            (401, Some("auth_expired"), false, Next::Resign),
            (401, Some("auth_unknown_signer"), false, Next::Retry),
            (401, Some("auth_malformed"), true, Next::Stop),
            (401, None, true, Next::Resign),
            (401, None, false, Next::Stop),
            (503, Some("unavailable"), false, Next::Stop),
            (400, Some("non_canonical_path"), false, Next::Stop),
        ] {
            let status = StatusCode::from_u16(status).unwrap();
            assert_eq!(
                after_error(status, code, stale),
                next,
                "{status} {code:?} {stale}"
            );
        }
    }
}
