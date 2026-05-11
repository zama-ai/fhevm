//! Tracing initialization module
//!
//! This module provides unified tracing setup for both main application and tests,
//! with support for different formats, Chrome tracing, and proper configuration.

use crate::config::settings::LogConfig;
use std::sync::Once;
use tracing::info;
use tracing_subscriber::prelude::*;
use tracing_subscriber::EnvFilter;

#[cfg(feature = "tracing-chrome")]
use tracing_chrome::{ChromeLayerBuilder, FlushGuard};

#[cfg(not(feature = "tracing-chrome"))]
pub struct FlushGuard {}

#[cfg(not(feature = "tracing-chrome"))]
impl FlushGuard {
    pub fn flush(&self) {}
}

#[cfg(not(feature = "tracing-chrome"))]
impl Drop for FlushGuard {
    fn drop(&mut self) {}
}

/// Initialize tracing for the main application with full configuration support.
///
/// # Arguments
/// * `log_config` - The [`LogConfig`] containing logging preferences
///
/// # Returns
/// * `Ok(Some(FlushGuard))` - Chrome tracing guard if feature enabled
/// * `Ok(None)` - If Chrome tracing is disabled
/// * `Err(`[`anyhow::Error`]`)` - If initialization failed
///
/// # Configuration Options
/// - Log level (trace, debug, info, warn, error)
/// - Log format (compact, pretty, json)
/// - File and line number display
/// - Thread ID display
/// - Chrome tracing support (with feature flag)
pub fn init_tracing(log_config: &LogConfig) -> anyhow::Result<Option<FlushGuard>> {
    // Default: WARN for dependencies, INFO for fhevm_relayer. Override with RUST_LOG env var.
    // Examples: RUST_LOG=debug | RUST_LOG=warn,fhevm_relayer=debug | RUST_LOG=warn,reqwest=debug
    let env_filter = EnvFilter::try_from_default_env()
        .or_else(|_| EnvFilter::try_new("warn,fhevm_relayer=info,ethereum_rpc_mock=info"))
        .unwrap();

    // Apply format configuration and box the layer
    let fmt_layer = match log_config.format.as_str() {
        "json" => {
            let layer = tracing_subscriber::fmt::layer()
                .with_file(log_config.show_file_line)
                .with_line_number(log_config.show_file_line)
                .with_thread_ids(log_config.show_thread_ids)
                .with_target(log_config.show_target)
                .json();
            if !log_config.show_timestamp {
                layer.without_time().boxed()
            } else {
                layer.boxed()
            }
        }
        "pretty" => {
            let layer = tracing_subscriber::fmt::layer()
                .with_file(log_config.show_file_line)
                .with_line_number(log_config.show_file_line)
                .with_thread_ids(log_config.show_thread_ids)
                .with_target(log_config.show_target)
                .pretty();
            if !log_config.show_timestamp {
                layer.without_time().boxed()
            } else {
                layer.boxed()
            }
        }
        "compact" => {
            // Compact format
            let layer = tracing_subscriber::fmt::layer()
                .with_file(log_config.show_file_line)
                .with_line_number(log_config.show_file_line)
                .with_thread_ids(log_config.show_thread_ids)
                .with_target(log_config.show_target)
                .compact();
            if !log_config.show_timestamp {
                layer.without_time().boxed()
            } else {
                layer.boxed()
            }
        }
        _ => {
            // Default to compact format
            let layer = tracing_subscriber::fmt::layer()
                .with_file(log_config.show_file_line)
                .with_line_number(log_config.show_file_line)
                .with_thread_ids(log_config.show_thread_ids)
                .with_target(log_config.show_target)
                .compact();
            if !log_config.show_timestamp {
                layer.without_time().boxed()
            } else {
                layer.boxed()
            }
        }
    };

    let tracing_subscriber_builder = tracing_subscriber::registry()
        .with(env_filter)
        .with(fmt_layer);

    let optional_chrome_guard: Option<FlushGuard>;
    #[cfg(feature = "tracing-chrome")]
    {
        let (chrome_layer, chrome_tracing_guard) = ChromeLayerBuilder::new()
            .trace_style(tracing_chrome::TraceStyle::Async)
            .build();
        tracing_subscriber_builder.with(chrome_layer).init();
        optional_chrome_guard = Some(chrome_tracing_guard);
    }
    #[cfg(not(feature = "tracing-chrome"))]
    {
        tracing_subscriber_builder.init();
        optional_chrome_guard = None;
    }

    info!(
        format = ?log_config.format,
        show_file_line = ?log_config.show_file_line,
        show_thread_ids = ?log_config.show_thread_ids,
        show_target = ?log_config.show_target,
        "Tracing initialized successfully"
    );

    Ok(optional_chrome_guard)
}

/// Initialize tracing once for tests using the full-featured init_tracing.
/// Provides full feature parity with main application including Chrome tracing support.
/// Safe to call multiple times - initialization only happens once.
///
/// # Arguments
/// * `config` - LogConfig from Settings containing format and display options
///
/// # Environment Variables
/// Tests can override config via environment variables:
/// - `APP_LOG__FORMAT` - Override format (compact/pretty/json)
/// - `APP_LOG__SHOW_FILE_LINE` - Show file/line numbers (true/false)  
/// - `APP_LOG__SHOW_THREAD_IDS` - Show thread IDs (true/false)
/// - `RUST_LOG` - Override log levels (warn for deps, info for fhevm_relayer)
pub fn init_tracing_once(config: &LogConfig) {
    static INIT: Once = Once::new();
    INIT.call_once(|| {
        match init_tracing(config) {
            Ok(guard) => {
                // For tests, we need to ensure Chrome traces are properly flushed
                // Store the guard in a static with proper cleanup
                use std::sync::Mutex;
                use std::thread;

                static GUARD: Mutex<Option<FlushGuard>> = Mutex::new(None);

                if let Ok(mut g) = GUARD.lock() {
                    *g = guard;

                    // Spawn a background thread that will flush the guard after a short delay
                    // This ensures traces are written even if tests exit quickly
                    thread::spawn(|| {
                        thread::sleep(std::time::Duration::from_millis(100));
                        if let Ok(mut guard_opt) = GUARD.lock() {
                            if let Some(guard) = guard_opt.take() {
                                guard.flush();
                                drop(guard);
                            }
                        }
                    });
                }
            }
            Err(e) => eprintln!("Failed to initialize tracing: {}", e),
        }
    });
}
