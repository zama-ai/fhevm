//! Global tracing subscriber. Called once from `main`; the library never installs one.

use std::io::IsTerminal;

use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

use crate::settings::LogConfig;

/// Text or JSON lines on stdout, filtered by `RUST_LOG` or `log.level`. A second call is a no-op.
pub fn init(cfg: &LogConfig) {
    let filter = EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new(&cfg.level));
    let registry = tracing_subscriber::registry().with(filter);
    let result = if cfg.json {
        registry
            .with(fmt::layer().json().flatten_event(true))
            .try_init()
    } else {
        // No colour codes in pod logs.
        registry
            .with(fmt::layer().with_ansi(std::io::stdout().is_terminal()))
            .try_init()
    };
    if let Err(e) = result {
        eprintln!("logging already initialised: {e}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn init_twice_does_not_panic() {
        let text = LogConfig {
            level: "debug".into(),
            json: false,
        };
        let json = LogConfig {
            level: "not a directive !!".into(),
            json: true,
        };
        init(&text);
        init(&json);
        tracing::info!("still alive");
    }
}
