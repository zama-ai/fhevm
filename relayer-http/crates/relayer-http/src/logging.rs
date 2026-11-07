//! Logging: the global tracing subscriber (the current relayer's setup: `RUST_LOG` filter, `json` | `pretty` |
//! `compact` format, installed once by `main`), the bridge from `log`-crate libraries, and the panic hook. What is
//! logged is governed by `OBSERVABILITY.md`.

use tracing::{error, info};
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::config::LogConfig;

/// Level filter when `RUST_LOG` is not set: `warn` for dependencies, `debug` for this crate (the per-node lines of
/// every request).
pub const DEFAULT_FILTER: &str = "warn,relayer_http=debug";

/// Installs the subscriber described by `cfg`, and the bridge that turns the records of `log`-crate libraries
/// (reqwest, rustls-platform-verifier) into events under the same filter. Unknown formats fall back to `compact`. A
/// second call is a no-op.
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

/// Replaces the default panic hook, whose plain-text stderr line would break the JSON stream. A panic inside a
/// request is left to that request's `internal error` line, and one in a KMS node task to its `node task panicked`
/// line, both carrying its location; any other is one ERROR line `panicked` with the `location`. Never the payload:
/// it may hold request data.
pub fn init_panic_hook() {
    std::panic::set_hook(Box::new(|info| {
        let location = info.location().map(ToString::to_string);
        if !crate::endpoint::note_panic(location.clone())
            && !crate::kms_aggregator::note_node_panic(location.clone())
        {
            error!(
                location = location.as_deref().unwrap_or("unknown"),
                "panicked"
            );
        }
    }));
}

/// Test only: collects the JSON log lines emitted on the current thread.
#[cfg(test)]
pub(crate) mod capture {
    use std::io::Write;
    use std::sync::{Arc, Mutex};

    use serde_json::Value;
    use tracing_subscriber::fmt::MakeWriter;
    use tracing_subscriber::layer::SubscriberExt;

    /// The captured bytes, shared with the subscriber.
    #[derive(Clone, Default)]
    pub struct Sink(Arc<Mutex<Vec<u8>>>);

    impl Write for Sink {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            self.0.lock().unwrap().extend_from_slice(buf);
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    impl<'a> MakeWriter<'a> for Sink {
        type Writer = Sink;
        fn make_writer(&'a self) -> Sink {
            self.clone()
        }
    }

    impl Sink {
        pub fn clear(&self) {
            self.0.lock().unwrap().clear();
        }

        /// The lines whose message starts with `prefix`, parsed, in order.
        pub fn lines(&self, prefix: &str) -> Vec<Value> {
            let bytes = self.0.lock().unwrap().clone();
            String::from_utf8(bytes)
                .unwrap()
                .lines()
                .map(|line| serde_json::from_str::<Value>(line).unwrap())
                .filter(|line| {
                    line["fields"]["message"]
                        .as_str()
                        .is_some_and(|m| m.starts_with(prefix))
                })
                .collect()
        }
    }

    /// Collects the JSON lines of `run`: it is repeated, under a JSON subscriber for this thread, until every message
    /// in `expected` was logged, and the lines of that last run are returned.
    ///
    /// `tracing` caches per call site whether any subscriber wants it. A call site first hit by a concurrent test
    /// while it had no subscriber is cached as disabled, and the rebuild triggered by installing this one can miss a
    /// call site still being registered. The cache is rebuilt between tries, so the assertions never depend on test
    /// scheduling.
    pub async fn capture_runs<F: Future>(expected: &[&str], run: impl Fn() -> F) -> Vec<Value> {
        let sink = Sink::default();
        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(sink.clone()),
        );
        let _guard = tracing::subscriber::set_default(subscriber);
        for _ in 0..1000 {
            sink.clear();
            run().await;
            let lines = sink.lines("");
            let all_seen = expected.iter().all(|wanted| {
                lines
                    .iter()
                    .any(|line| line["fields"]["message"] == *wanted)
            });
            if all_seen {
                return lines;
            }
            tracing::callsite::rebuild_interest_cache();
            tokio::task::yield_now().await;
        }
        panic!("the call sites of {expected:?} never became enabled");
    }
}

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
    fn log_records_go_through_the_same_filter_and_a_second_init_is_harmless() {
        init(&LogConfig::default());
        init(&LogConfig::default());
        let log_enabled = |target: &str, level: log::Level| {
            log::logger().enabled(&log::Metadata::builder().target(target).level(level).build())
        };
        // Without the bridge, `log` has its no-op logger and every record is disabled.
        assert_eq!(
            log_enabled("reqwest::connect", log::Level::Warn),
            tracing::enabled!(target: "reqwest::connect", tracing::Level::WARN)
        );
        assert_eq!(
            log_enabled("reqwest::connect", log::Level::Debug),
            tracing::enabled!(target: "reqwest::connect", tracing::Level::DEBUG)
        );
        assert_eq!(
            log_enabled("relayer_http::x", log::Level::Debug),
            tracing::enabled!(target: "relayer_http::x", tracing::Level::DEBUG)
        );
        if std::env::var_os("RUST_LOG").is_none() {
            assert!(log_enabled("reqwest::connect", log::Level::Warn));
            assert!(!log_enabled("reqwest::connect", log::Level::Debug));
        }
    }

    #[test]
    fn the_panic_hook_logs_the_location_never_the_payload() {
        init_panic_hook();
        let sink = capture::Sink::default();
        let subscriber = tracing_subscriber::registry().with(
            tracing_subscriber::fmt::layer()
                .json()
                .with_writer(sink.clone()),
        );
        tracing::subscriber::with_default(subscriber, || {
            let _ = std::panic::catch_unwind(|| panic!("SECRET-PAYLOAD"));
        });
        let _ = std::panic::take_hook();
        let lines = sink.lines("");
        assert_eq!(lines.len(), 1, "{lines:?}");
        assert_eq!(lines[0]["level"], "ERROR");
        assert_eq!(lines[0]["fields"]["message"], "panicked");
        assert!(
            lines[0]["fields"]["location"]
                .as_str()
                .unwrap()
                .contains("logging.rs")
        );
        assert!(!lines[0].to_string().contains("SECRET-PAYLOAD"));
    }

    #[test]
    fn default_filter_parses() {
        assert!(EnvFilter::try_new(DEFAULT_FILTER).is_ok());
    }
}
