//! Global tracing subscriber, the current relayer's setup: `RUST_LOG` filter, `json` | `pretty` | `compact`
//! format. Called once from `main`; the library never installs one.

use tracing::info;
use tracing_subscriber::{EnvFilter, Layer, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::LogConfig;

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
}
