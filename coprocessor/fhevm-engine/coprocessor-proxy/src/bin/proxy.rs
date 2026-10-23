use clap::{Parser, Subcommand};
use coprocessor_proxy::core::{Config, Proxy};
use fhevm_engine_common::{healthz_server, metrics_server, telemetry};
use std::{path::PathBuf, process::ExitCode, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::error;

#[derive(Parser)]
#[command(version, about = "Public HTTPS entry point of a coprocessor")]
struct Cli {
    #[command(subcommand)]
    command: Subcommands,
}

#[derive(Subcommand)]
enum Subcommands {
    /// Start the proxy
    Start {
        /// Configuration file path (optional if using environment variables)
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,
    },

    /// Validate a configuration file
    Validate {
        /// Configuration file path
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,
    },
}

fn main() -> ExitCode {
    if let Err(err) = run() {
        error!("{err:#}");
        eprintln!("{err:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

// Unlike the other coprocessor services, the proxy does not run on `#[tokio::main]`: Pingora
// manages its own runtimes and blocks the main thread.
fn run() -> anyhow::Result<()> {
    fhevm_engine_common::handle_stack_version_flag();
    match Cli::parse().command {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Start { config } => {
            let config = Config::from_env_and_file(config.as_ref())?;

            // The OTLP exporter spawns on the entered runtime during setup. Keep it alive
            // alongside Pingora's runtimes until shutdown.
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()?;
            let _guard = runtime.enter();
            let _otel_guard = telemetry::init_tracing_otel_with_logs_only_fallback_and_filter(
                config.log_level,
                &config.service_name,
                "otlp-layer",
                telemetry::transport_dependency_filter,
            );

            // The monitoring servers run on the tokio runtime, next to Pingora's own.
            let healthcheck_port = config.healthcheck_port;
            let metrics_endpoint = config.metrics_endpoint.to_string();
            let (proxy, state) = Proxy::from_config(config)?;
            let cancel_token = CancellationToken::new();
            let health_server = healthz_server::HttpServer::new(
                Arc::new(state),
                healthcheck_port,
                cancel_token.child_token(),
            );
            let health_server_task = tokio::spawn(async move {
                if let Err(e) = health_server.start().await {
                    error!(error = %e, "Health check server failed");
                }
            });
            let metrics_server_task =
                metrics_server::spawn(Some(metrics_endpoint), cancel_token.child_token());

            proxy.run()?;

            cancel_token.cancel();
            runtime.block_on(async {
                let _ = health_server_task.await;
                if let Some(task) = metrics_server_task {
                    let _ = task.await;
                }
            });
        }
    }
    Ok(())
}
