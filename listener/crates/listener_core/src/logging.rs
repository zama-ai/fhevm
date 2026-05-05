use crate::config::{LogConfig, LogFormat};
use tracing_subscriber::layer::SubscriberExt;
use tracing_subscriber::util::SubscriberInitExt;
use tracing_subscriber::{EnvFilter, fmt};

/// Initialize the tracing subscriber and optionally enter a root span
/// that injects constant fields (`name`, `network`, `chain_id`) into every log line.
///
/// # EnvFilter behaviour
/// - If `RUST_LOG` is set, it takes full precedence over `config.level`.
/// - Otherwise, the default filter is `warn,listener_core={level}` where
///   `{level}` comes from `config.level`.
///
/// # Returns
/// An `Option<tracing::span::EnteredSpan>` that **must** be held alive for the
/// lifetime of `main()`. Dropping it removes the constant fields from subsequent logs.
pub fn init_logging(
    config: &LogConfig,
    name: &str,
    network: &str,
    chain_id: u64,
) -> Option<tracing::span::EnteredSpan> {
    let filter = build_env_filter(config);

    // Each format × timestamp combination produces a different concrete type,
    // so we call .init() in each branch rather than boxing.
    match (&config.format, config.show_timestamp) {
        (LogFormat::Json, true) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .json()
                        .flatten_event(true),
                )
                .init();
        }
        (LogFormat::Json, false) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .json()
                        .flatten_event(true)
                        .without_time(),
                )
                .init();
        }
        (LogFormat::Compact, true) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .compact(),
                )
                .init();
        }
        (LogFormat::Compact, false) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .compact()
                        .without_time(),
                )
                .init();
        }
        (LogFormat::Pretty, true) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .pretty(),
                )
                .init();
        }
        (LogFormat::Pretty, false) => {
            tracing_subscriber::registry()
                .with(filter)
                .with(
                    fmt::layer()
                        .with_file(config.show_file_line)
                        .with_line_number(config.show_file_line)
                        .with_thread_ids(config.show_thread_ids)
                        .with_target(config.show_target)
                        .pretty()
                        .without_time(),
                )
                .init();
        }
    }

    if config.show_constants {
        let span = tracing::info_span!(
            "listener",
            name = %name,
            network = %network,
            chain_id = chain_id,
        );
        Some(span.entered())
    } else {
        None
    }
}

fn build_env_filter(config: &LogConfig) -> EnvFilter {
    match std::env::var("RUST_LOG") {
        Ok(_) => EnvFilter::from_default_env(),
        Err(_) => EnvFilter::new(format!("warn,listener_core={}", config.level)),
    }
}
