//! [`Summary`]: what one request accumulates as it is handled, logged once as the `request completed` line when
//! the response is sent, and the one definition of that line's fields, [`FIELDS`].

use std::fmt::Display;
use std::sync::Arc;

use alloy::primitives::{Address, B256};
use tokio::time::Instant;
use tracing::{error, info, warn};

use super::validate::decode_handle;
use crate::config::HttpConfig;
use crate::kms_aggregator::Tally;
use crate::metrics::{FlowName, INTERNAL_CODE, Metrics, Outcome};

/// One field of the summary line, as published in `monitoring/summary_log.md`.
pub struct Field {
    pub name: &'static str,
    /// What the field holds, and its values.
    pub meaning: &'static str,
}

/// Every field of the summary line. [`Summary::emit`] must log exactly these: a test compares.
pub const FIELDS: &[Field] = &[
    Field {
        name: "request_id",
        meaning: "random UUIDv4 of the request; the `x-request-id` header and the `requestId` of the answer",
    },
    Field {
        name: "flow",
        meaning: "`user_decrypt`, `public_decrypt`, or `none` when answered before a flow is known",
    },
    Field {
        name: "outcome",
        meaning: "`succeeded` (2xx), `rejected` (4xx), `failed` (5xx, a panic included) or `cancelled` (the client closed the connection before the answer)",
    },
    Field {
        name: "status",
        meaning: "HTTP status of the answer, as text; `none` when the client closed the connection first; `500` for a panic",
    },
    Field {
        name: "code",
        meaning: "`ok`, the API error code of the answer, or `client_closed`; `internal` is a relayer defect (an internal error or a panic)",
    },
    Field {
        name: "elapsed_ms",
        meaning: "milliseconds from receipt to the answer",
    },
    Field {
        name: "handles",
        meaning: "ciphertext handles, comma-separated; at most `http.max_handles_per_decryption` (the validation limit), then `+N more`, so only a request that failed validation is cut; `none` if the body was unreadable",
    },
    Field {
        name: "contract_addresses",
        meaning: "contract addresses of the handles, comma-separated, each once, cut like `handles`; `none` if the body was unreadable or the flow has none (public decrypt)",
    },
    Field {
        name: "allowed_contracts",
        meaning: "the `allowedContracts` of the request (user decrypt), comma-separated, at most `http.max_allowed_contracts` (the validation limit), then `+N more`; `none` otherwise",
    },
    Field {
        name: "user_address",
        meaning: "the user's address (user decrypt); `none` otherwise",
    },
    Field {
        name: "chain_id",
        meaning: "chain id the handles carry; `none` if unreadable",
    },
    Field {
        name: "body_len",
        meaning: "bytes of the request body; 0 when it was not received (unreadable, over the size limit, or refused for its content type)",
    },
    Field {
        name: "decryption_id",
        meaning: "EIP-712 signing hash of the connector request, the id KMS operators know; `none` if the request was never forwarded",
    },
    Field {
        name: "counted",
        meaning: "KMS responses that counted toward the threshold; 0 if the request was never forwarded",
    },
    Field {
        name: "threshold",
        meaning: "KMS responses the flow needs; 0 if the request was never forwarded",
    },
    Field {
        name: "threshold_reached_ms",
        meaning: "milliseconds from receipt until the counted responses reached the threshold, as text; the answer waited for the remaining nodes after it; `none` if the threshold was never reached or the request was never forwarded",
    },
    Field {
        name: "failed_kms_nodes",
        meaning: "KMS nodes whose call failed, comma-separated, including a call that failed to sign (the `signing failed` line says so); `none` if none",
    },
    Field {
        name: "rejected_kms_nodes",
        meaning: "KMS nodes whose response a check refused, comma-separated; `none` if none",
    },
    Field {
        name: "cancelled_kms_nodes",
        meaning: "KMS nodes whose call was cut (deadline, fail fast, client hangup), comma-separated; `none` if none",
    },
    Field {
        name: "reason",
        meaning: "why the request did not succeed: the validation field and issue, the body error with its field path, the aggregation verdict, `client closed` or `handler panicked`; `none` on success",
    },
];

/// The message of the summary line.
pub const MESSAGE: &str = "request completed";

/// The `reason` of a request whose handler panicked; also the `message` the client gets.
pub const PANIC_REASON: &str = "handler panicked";

/// One request, from receipt to answer. Built by the handler, carried by the answer in its extensions, logged and
/// counted by `endpoint::record`.
#[derive(Debug, Clone)]
pub struct Summary {
    pub request_id: String,
    pub flow: FlowName,
    handles: Vec<B256>,
    contract_addresses: Vec<Address>,
    allowed_contracts: Vec<Address>,
    user_address: Option<Address>,
    chain_id: Option<u64>,
    pub body_len: usize,
    decryption_id: Option<B256>,
    pub tally: Tally,
    pub(super) code: &'static str,
    reason: Option<String>,
    /// Where the handler panicked (`file:line:column`), for the `internal error` line; never the panic's payload.
    pub(super) panic_location: Option<String>,
    /// Most entries listed: validation's limits (`http.max_handles_per_decryption`, `http.max_allowed_contracts`).
    max_handles: usize,
    max_allowed_contracts: usize,
}

impl Summary {
    /// A new random request id, nothing else known yet.
    pub fn new(flow: FlowName) -> Self {
        Self::with_id(flow, uuid::Uuid::new_v4().to_string())
    }

    /// The request `request_id`, nothing else known yet.
    fn with_id(flow: FlowName, request_id: String) -> Self {
        Self {
            request_id,
            flow,
            handles: Vec::new(),
            contract_addresses: Vec::new(),
            allowed_contracts: Vec::new(),
            user_address: None,
            chain_id: None,
            body_len: 0,
            decryption_id: None,
            tally: Tally::default(),
            code: "ok",
            reason: None,
            panic_location: None,
            max_handles: usize::MAX,
            max_allowed_contracts: usize::MAX,
        }
    }

    /// Moves the summary out, leaving its request id and flow: what a panic after this point still logs.
    pub fn take(&mut self) -> Self {
        let rest = Self::with_id(self.flow, self.request_id.clone());
        std::mem::replace(self, rest)
    }

    /// The body is parsed: the public fields of the request become known, listed up to `http`'s request limits.
    pub fn received(
        &mut self,
        http: &HttpConfig,
        handles: Vec<B256>,
        contract_addresses: Vec<Address>,
        allowed_contracts: Vec<Address>,
        user_address: Option<Address>,
    ) {
        self.max_handles = http.max_handles_per_decryption;
        self.max_allowed_contracts = http.max_allowed_contracts;
        self.chain_id = handles
            .first()
            .and_then(|handle| decode_handle(handle).ok())
            .map(|(chain_id, _)| chain_id);
        self.handles = handles;
        self.contract_addresses = contract_addresses;
        self.allowed_contracts = allowed_contracts;
        self.user_address = user_address;
    }

    /// The connector request is built and goes to the aggregator.
    pub fn forwarded(&mut self, decryption_id: B256) {
        self.decryption_id = Some(decryption_id);
    }

    /// A copy that ended with `code`, for `reason`.
    pub(super) fn ended(&self, code: &'static str, reason: &str) -> Self {
        Self {
            code,
            reason: Some(reason.to_owned()),
            ..self.clone()
        }
    }

    /// The request, received at `started`, is over: the one line `request completed`, and the request metrics, from
    /// this one value. `status` is `None` when the client closed the connection before the answer. A relayer defect
    /// (code `internal`) is also an ERROR line of its own: the summary line is INFO for every outcome.
    pub fn complete(&self, status: Option<u16>, started: Instant, metrics: &Metrics) {
        let outcome = status.map_or(Outcome::Cancelled, Outcome::of_status);
        let elapsed = started.elapsed();
        let threshold_reached_ms = self
            .tally
            .threshold_reached
            .map(|reached| reached.saturating_duration_since(started).as_millis());
        info!(
            request_id = %self.request_id,
            flow = self.flow.as_str(),
            outcome = outcome.as_str(),
            status = %or_none(status),
            code = self.code,
            elapsed_ms = elapsed.as_millis() as u64,
            handles = %list_at_most(&self.handles, self.max_handles),
            contract_addresses = %list_at_most(&self.contract_addresses, self.max_handles),
            allowed_contracts = %list_at_most(&self.allowed_contracts, self.max_allowed_contracts),
            user_address = %or_none(self.user_address),
            chain_id = %or_none(self.chain_id),
            body_len = self.body_len,
            decryption_id = %or_none(self.decryption_id),
            counted = self.tally.counted,
            threshold = self.tally.threshold,
            threshold_reached_ms = %or_none(threshold_reached_ms),
            failed_kms_nodes = %list(&self.tally.failed),
            rejected_kms_nodes = %list(&self.tally.rejected),
            cancelled_kms_nodes = %list(&self.tally.cancelled),
            reason = self.reason.as_deref().unwrap_or("none"),
            "request completed"
        );
        if self.code == INTERNAL_CODE {
            error!(
                request_id = %self.request_id,
                reason = self.reason.as_deref().unwrap_or("none"),
                location = self.panic_location.as_deref(),
                "internal error"
            );
        }
        metrics.request(self.flow, outcome, self.code, elapsed);
    }
}

/// A request in progress. If the handler's future is dropped (the client closed the connection), dropping the guard
/// completes the request as `cancelled`, with whatever is known by then, so every request has its line. If the
/// handler panics, the unwinding drops the guard: the request is completed as a `failed` `internal` (status 500,
/// the answer `endpoint::answer_panic` sends), with its ERROR line carrying the panic's location.
pub struct Guard {
    pub summary: Summary,
    metrics: Arc<Metrics>,
    started: Instant,
    armed: bool,
}

impl Guard {
    pub fn new(flow: FlowName, metrics: Arc<Metrics>) -> Self {
        Self {
            summary: Summary::new(flow),
            metrics,
            started: Instant::now(),
            armed: true,
        }
    }

    /// The handler answered: the middleware completes the request, not this guard.
    pub fn disarm(&mut self) {
        self.armed = false;
    }
}

impl Drop for Guard {
    fn drop(&mut self) {
        if !self.armed {
            return;
        }
        let panicked = std::thread::panicking();
        let (mut summary, cause) = if panicked {
            let mut summary = self.summary.ended(INTERNAL_CODE, PANIC_REASON);
            summary.panic_location = super::panic_logged(&summary.request_id);
            (summary, "panic")
        } else {
            (
                self.summary.ended("client_closed", "client closed"),
                "client_closed",
            )
        };
        // The node calls dropped with the handler are cut.
        let pending = std::mem::take(&mut summary.tally.pending);
        for kms_node in &pending {
            warn!(request_id = %summary.request_id, kms_node, cause, "call cut");
        }
        summary.tally.cancelled.extend(pending);
        summary.complete(panicked.then_some(500), self.started, &self.metrics);
    }
}

/// `a,b,c`, or `none` when empty.
fn list(items: &[impl Display]) -> String {
    list_at_most(items, usize::MAX)
}

/// [`list`], cut after `limit` entries: `a,b,+3 more`. The limits are validation's, so only a request that failed
/// validation is cut: nothing else bounds its lists but the body limit.
fn list_at_most(items: &[impl Display], limit: usize) -> String {
    if items.is_empty() {
        return "none".to_owned();
    }
    let shown = limit.min(items.len());
    let mut parts: Vec<String> = items.iter().take(shown).map(ToString::to_string).collect();
    if shown < items.len() {
        parts.push(format!("+{} more", items.len() - shown));
    }
    parts.join(",")
}

fn or_none(value: Option<impl Display>) -> String {
    value.map_or_else(|| "none".to_owned(), |v| v.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_new_summary_has_a_unique_v4_id_and_knows_nothing() {
        let (a, b) = (
            Summary::new(FlowName::UserDecrypt),
            Summary::new(FlowName::None),
        );
        assert_ne!(a.request_id, b.request_id);
        let id = uuid::Uuid::parse_str(&a.request_id).unwrap();
        assert_eq!(id.get_version_num(), 4);
        assert_eq!(list(&Vec::<B256>::new()), "none");
        assert_eq!(or_none(None::<u64>), "none");
    }

    #[test]
    fn lists_are_comma_separated_and_whole() {
        let handles = [B256::repeat_byte(0x11), B256::repeat_byte(0x22)];
        assert_eq!(list(&handles), format!("{},{}", handles[0], handles[1]),);
        assert_eq!(list(&["kms_00".to_owned()]), "kms_00");
    }

    #[test]
    fn a_bounded_list_names_how_many_entries_it_left_out() {
        let items = [1, 2, 3, 4];
        assert_eq!(list_at_most(&items, 2), "1,2,+2 more");
        assert_eq!(list_at_most(&items, 4), "1,2,3,4");
        assert_eq!(list_at_most(&Vec::<u8>::new(), 2), "none");
    }

    #[test]
    fn a_taken_summary_leaves_its_request_id_and_flow() {
        let mut summary = Summary::new(FlowName::PublicDecrypt);
        summary.received(
            &HttpConfig::for_tests(&[1]),
            vec![B256::repeat_byte(0x11)],
            vec![],
            vec![],
            None,
        );
        let taken = summary.take();
        assert_eq!(taken.handles.len(), 1);
        assert_eq!(summary.request_id, taken.request_id);
        assert_eq!(summary.flow, FlowName::PublicDecrypt);
        assert!(summary.handles.is_empty());
    }

    #[test]
    fn the_chain_id_comes_from_the_first_handle() {
        let mut summary = Summary::new(FlowName::PublicDecrypt);
        summary.received(
            &HttpConfig::for_tests(&[1]),
            vec![crate::endpoint::validate::tests::handle(137, 5)],
            vec![],
            vec![],
            None,
        );
        assert_eq!(summary.chain_id, Some(137));
        summary.received(
            &HttpConfig::for_tests(&[1]),
            vec![B256::repeat_byte(0xff)],
            vec![],
            vec![],
            None,
        );
        assert_eq!(summary.chain_id, None);
    }

    #[test]
    fn field_names_are_unique() {
        let mut names: Vec<&str> = FIELDS.iter().map(|f| f.name).collect();
        names.sort_unstable();
        names.dedup();
        assert_eq!(names.len(), FIELDS.len());
    }
}
