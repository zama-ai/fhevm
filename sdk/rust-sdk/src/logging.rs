use std::sync::Once;
use tracing::{Level, info};
use tracing_subscriber::Layer;
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};
static INIT: Once = Once::new();

/// Configuration for the logging system
#[derive(Debug, Clone)]
pub struct LogConfig {
    /// The log level to use
    pub level: Level,
    /// Whether to show file names and line numbers
    pub show_file_line: bool,
    /// Whether to show thread IDs
    pub show_thread_ids: bool,
    /// Log format (compact, pretty, json)
    pub format: LogFormat,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LogFormat {
    Compact,
    Pretty,
    Json,
}

impl Default for LogConfig {
    fn default() -> Self {
        Self {
            level: Level::INFO,
            show_file_line: false,
            show_thread_ids: false,
            format: LogFormat::Compact,
        }
    }
}

/// Initialize the logger with the specified log level.
///
/// This is a simplified version that uses sensible defaults.
/// For more control, use `init_with_config`.
///
/// # Arguments
///
/// * `level` - The maximum log level to display.
///
/// # Example
///
/// ```
/// use gateway_sdk::logging;
/// use tracing::Level;
///
/// // Initialize logging with info level
/// logging::init(Level::INFO);
///     
/// // Now you can use tracing macros
/// tracing::info!("Application started");
/// ```
pub fn init(level: Level) {
    let config = LogConfig {
        level,
        ..Default::default()
    };
    init_with_config(config);
}

/// Initialize the logger with the level specified in the RUST_LOG environment variable.
/// If RUST_LOG is not set, defaults to the provided level.
///
/// # Arguments
///
/// * `default_level` - The default log level to use if RUST_LOG is not set.
///
/// # Example
///
/// ```
/// use gateway_sdk::logging;
/// use tracing::Level;
///
/// // Initialize logging with Info as default level
/// logging::init_from_env(Level::INFO);
/// ```
pub fn init_from_env(default_level: Level) {
    INIT.call_once(|| {
        // Create env filter with default
        let env_filter = EnvFilter::try_from_default_env()
            .unwrap_or_else(|_| EnvFilter::new(level_to_string(default_level)));

        let subscriber = tracing_subscriber::registry().with(env_filter).with(
            fmt::layer()
                .with_target(false)
                .with_ansi(true)
                .with_timer(fmt::time::SystemTime),
        );

        subscriber.init();

        info!(
            "Logger initialized from environment with default level: {}",
            level_to_string(default_level)
        );
    });
}

/// Initialize logging with full configuration control
///
/// # Arguments
/// * `config` - The [`LogConfig`] containing logging preferences
///
/// # Example
///
/// ```
/// use gateway_sdk::logging::{LogConfig, LogFormat};
/// use tracing::Level;
/// use gateway_sdk::logging;
///
/// let config = LogConfig {
///     level: Level::DEBUG,
///     show_file_line: true,
///     show_thread_ids: false,
///     format: LogFormat::Pretty,
/// };
///
/// logging::init_with_config(config);
/// ```
pub fn init_with_config(config: LogConfig) {
    INIT.call_once(|| {
        if let Err(e) = init_tracing(&config) {
            eprintln!("Failed to initialize tracing: {}", e);
        }
    });
}

/// Initialize tracing based on configuration settings.
///
/// This provides full control over the logging output format and filtering.
fn init_tracing(log_config: &LogConfig) -> Result<(), Box<dyn std::error::Error>> {
    // Create env filter with fallback to configured level
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(level_to_string(log_config.level)));

    // Build the formatting layer based on config
    let fmt_layer = match log_config.format {
        LogFormat::Compact => fmt::layer()
            .compact()
            .with_ansi(true)
            .with_file(log_config.show_file_line)
            .with_line_number(log_config.show_file_line)
            .with_thread_ids(log_config.show_thread_ids)
            .with_target(false)
            .with_timer(fmt::time::SystemTime)
            .boxed(),
        LogFormat::Pretty => fmt::layer()
            .pretty()
            .with_ansi(true)
            .with_file(log_config.show_file_line)
            .with_line_number(log_config.show_file_line)
            .with_thread_ids(log_config.show_thread_ids)
            .with_target(false)
            .with_timer(fmt::time::SystemTime)
            .boxed(),
        LogFormat::Json => fmt::layer()
            .json()
            .with_file(log_config.show_file_line)
            .with_line_number(log_config.show_file_line)
            .with_thread_ids(log_config.show_thread_ids)
            .with_target(false)
            .with_timer(fmt::time::SystemTime)
            .boxed(),
    };

    // Build and initialize the subscriber
    let subscriber = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    subscriber.init();

    info!(
        format = ?log_config.format,
        show_file_line = log_config.show_file_line,
        show_thread_ids = log_config.show_thread_ids,
        "Tracing initialized successfully"
    );

    Ok(())
}

/// Convert tracing Level to string for env filter
fn level_to_string(level: Level) -> &'static str {
    match level {
        Level::TRACE => "trace",
        Level::DEBUG => "debug",
        Level::INFO => "info",
        Level::WARN => "warn",
        Level::ERROR => "error",
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_level_to_string() {
        assert_eq!(level_to_string(Level::TRACE), "trace");
        assert_eq!(level_to_string(Level::DEBUG), "debug");
        assert_eq!(level_to_string(Level::INFO), "info");
        assert_eq!(level_to_string(Level::WARN), "warn");
        assert_eq!(level_to_string(Level::ERROR), "error");
    }

    #[test]
    fn test_default_config() {
        let config = LogConfig::default();
        assert_eq!(config.level, Level::INFO);
        assert!(!config.show_file_line);
        assert!(!config.show_thread_ids);
        assert_eq!(config.format, LogFormat::Compact);
    }
}
