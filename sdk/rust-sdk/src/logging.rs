use log::{LevelFilter, info};
use std::sync::Once;

static INIT: Once = Once::new();

/// Initialize the logger with the specified log level.
///
/// # Arguments
///
/// * `level` - The maximum log level to display.
///
/// # Example
///
/// ```
/// use fhevm_sdk::logging;
///
/// // Initialize logging with info level
/// logging::init(log::LevelFilter::Info);
///     
/// // Now you can use log macros
/// log::info!("Application started");
/// ```
pub fn init(level: LevelFilter) {
    INIT.call_once(|| {
        env_logger::Builder::new()
            .filter_level(level)
            .format_timestamp_millis()
            .init();

        info!("Logger initialized with level: {}", level);
    });
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
/// use fhevm_sdk::logging;
/// use log::LevelFilter;
///
/// // Initialize logging with Info as default level
/// logging::init_from_env(LevelFilter::Info);
/// ```
pub fn init_from_env(default_level: LevelFilter) {
    INIT.call_once(|| {
        env_logger::Builder::from_env(
            env_logger::Env::default().default_filter_or(default_level.as_str()),
        )
        .format_timestamp_millis()
        .init();

        info!(
            "Logger initialized from environment with default level: {}",
            default_level
        );
    });
}
