//! Logging: the global tracing subscriber (the current relayer's setup: `RUST_LOG` filter, `json` | `pretty` |
//! `compact` format, installed once by `main`) and [`Log`], the identifiers every line about one request carries.

use alloy::primitives::B256;
use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LogConfig;

/// Level filter when `RUST_LOG` is not set: `warn` for dependencies, `info` for this crate.
pub const DEFAULT_FILTER: &str = "warn,relayer_http=info";

/// Installs the subscriber described by `cfg`. Unknown formats fall back to `compact`. A second call is a no-op.
pub fn init(cfg: &LogConfig) {
    let filter =
        EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(DEFAULT_FILTER));
    let layer = fmt::layer()
        .with_file(cfg.show_file_line)
        .with_line_number(cfg.show_file_line)
        .with_thread_ids(cfg.show_thread_ids)
        .with_target(cfg.show_target);
    let layer = match (cfg.format.as_str(), cfg.show_timestamp) {
        ("json", true) => layer.json().boxed(),
        ("json", false) => layer.json().without_time().boxed(),
        ("pretty", true) => layer.pretty().boxed(),
        ("pretty", false) => layer.pretty().without_time().boxed(),
        (_, true) => layer.compact().boxed(),
        (_, false) => layer.compact().without_time().boxed(),
    };
    if let Err(e) = tracing_subscriber::registry()
        .with(filter)
        .with(layer)
        .try_init()
    {
        eprintln!("logging already initialised: {e}");
        return;
    }
    info!(
        format = %cfg.format,
        show_file_line = cfg.show_file_line,
        show_thread_ids = cfg.show_thread_ids,
        show_timestamp = cfg.show_timestamp,
        show_target = cfg.show_target,
        "tracing initialized"
    );
}

/// The log of one unit of work (one HTTP request today): the identifiers that must be on every line, `none` until
/// known, and one method per known event. Any other line goes through [`log!`] with the same identifiers. Built by
/// the handler and handed down; meant to reach the aggregator in a later iteration.
#[derive(Debug, Clone)]
pub struct Log {
    /// The relayer's correlation id (UUIDv7), minted by `new`.
    pub request_id: String,
    /// `user_decrypt`, `public_decrypt`; `none` on the router fallbacks.
    pub flow: &'static str,
    /// The ciphertext handles, known once the body is parsed.
    pub handles: Option<Vec<B256>>,
    /// The connector's content hash of the request, known once the connector request is built.
    pub decryption_id: Option<B256>,
}

impl Log {
    /// A new request id, nothing else known yet.
    pub fn new(flow: &'static str) -> Self {
        Self {
            request_id: uuid::Uuid::now_v7().to_string(),
            flow,
            handles: None,
            decryption_id: None,
        }
    }

    /// `[0x…, 0x…]`, or `none` until the body is parsed.
    pub fn handles(&self) -> String {
        self.handles
            .as_ref()
            .map_or_else(|| "none".to_owned(), |handles| format!("{handles:?}"))
    }

    /// `0x…`, or `none` until the connector request is built.
    pub fn decryption_id(&self) -> String {
        self.decryption_id
            .map_or_else(|| "none".to_owned(), |id| id.to_string())
    }

    /// The body is parsed: the handles become known.
    pub fn received(&mut self, handles: Vec<B256>) {
        self.handles = Some(handles);
        log!(info, self, "request received");
    }

    /// The body could not be parsed (invalid JSON, unknown field, wrong content type, oversized): the client's.
    pub fn body_rejected(&self, reason: &str) {
        log!(info, self, reason, "request body rejected");
    }

    /// A validation rule failed: the client's, named by field and issue.
    pub fn validation_failed(&self, field: &str, issue: &str) {
        log!(info, self, field, issue, "request validation failed");
    }

    /// The connector request is built and goes to the aggregator: the decryption id becomes known.
    pub fn forwarded(&mut self, decryption_id: B256) {
        self.decryption_id = Some(decryption_id);
        log!(info, self, "request forwarded");
    }

    /// The client gets a 2xx.
    pub fn succeeded(&self, status: u16) {
        log!(info, self, status, "request succeeded");
    }

    /// The client gets a 4xx: its request, not the relayer, is at fault.
    pub fn rejected(&self, status: u16, code: &str, reason: &str) {
        log!(info, self, status, code, reason, "request rejected");
    }

    /// The client gets a 5xx: the relayer or the nodes are at fault.
    pub fn failed(&self, status: u16, code: &str) {
        log!(warn, self, status, code, "request failed");
    }
}

/// One line with the identifiers of a [`Log`] first, then the caller's own fields and message:
/// `log!(debug, log, attempts = 2, "retrying")`.
macro_rules! log {
    ($level:ident, $log:expr, $($rest:tt)*) => {
        tracing::$level!(
            request_id = %$log.request_id,
            flow = $log.flow,
            handles = %$log.handles(),
            decryption_id = %$log.decryption_id(),
            $($rest)*
        )
    };
}
pub(crate) use log;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn every_format_installs_once_and_never_panics() {
        for (format, show_timestamp) in [
            ("json", true),
            ("json", false),
            ("pretty", true),
            ("pretty", false),
            ("compact", true),
            ("compact", false),
            ("not a format", true),
        ] {
            init(&LogConfig {
                format: format.to_owned(),
                show_timestamp,
                ..LogConfig::default()
            });
        }
        tracing::info!("still alive");
    }

    #[test]
    fn default_filter_parses() {
        assert!(EnvFilter::try_new(DEFAULT_FILTER).is_ok());
    }

    #[test]
    fn log_starts_with_a_unique_id_and_none_identifiers() {
        let (a, b) = (Log::new("user_decrypt"), Log::new("user_decrypt"));
        assert_ne!(a.request_id, b.request_id);
        assert_eq!(a.request_id.len(), 36);
        assert_eq!(a.flow, "user_decrypt");
        assert_eq!(
            (a.handles(), a.decryption_id()),
            ("none".into(), "none".into())
        );
    }

    #[test]
    fn identifiers_render_once_known() {
        let mut log = Log::new("public_decrypt");
        log.received(vec![B256::repeat_byte(0x11), B256::repeat_byte(0x22)]);
        log.forwarded(B256::repeat_byte(0xaa));
        assert_eq!(
            log.handles(),
            "[0x1111111111111111111111111111111111111111111111111111111111111111, \
             0x2222222222222222222222222222222222222222222222222222222222222222]"
        );
        assert_eq!(
            log.decryption_id(),
            "0xaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa"
        );
        // Every event method and the macro compile with custom fields and never panic.
        log.body_rejected("bad json");
        log.validation_failed("payload.publicKey", "must not be empty");
        log.succeeded(200);
        log.rejected(400, "malformed", "x");
        log.failed(502, "upstream_transient");
        log!(debug, log, attempts = 2u32, "custom line");
    }
}
