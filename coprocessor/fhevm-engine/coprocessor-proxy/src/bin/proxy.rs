use clap::{Parser, Subcommand};
use coprocessor_proxy::{config::Config, health::State};
use fhevm_engine_common::{healthz_server, metrics_server};
use fhevm_sig_proxy::Proxy;
use std::{path::PathBuf, process::ExitCode, sync::Arc};
use tokio_util::sync::CancellationToken;
use tracing::{debug, error};
use url::Url;

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

    /// Check the health of a running proxy
    Health {
        /// Healthcheck endpoint to query, e.g. `http://127.0.0.1:8080/healthz`
        #[arg(short, long, value_name = "URL")]
        endpoint: Url,
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

// The proxy does not run on `#[tokio::main]`: Pingora manages its own runtimes and blocks the main
// thread.
fn run() -> anyhow::Result<()> {
    fhevm_engine_common::handle_stack_version_flag();
    match Cli::parse().command {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(&config))?
                .proxy_config()?
                .validate()?;
        }
        Subcommands::Health { endpoint } => {
            let runtime = tokio::runtime::Runtime::new()?;
            let response = runtime.block_on(reqwest::get(endpoint))?;
            anyhow::ensure!(
                response.status().is_success(),
                "Healthcheck failed: {}",
                response.status()
            );
        }
        Subcommands::Start { config } => {
            let config = Config::from_env_and_file(config.as_deref())?;
            fhevm_engine_common::telemetry::init_json_subscriber(
                config.log_level,
                &config.service_name,
                "",
            )
            .map_err(|e| anyhow::anyhow!(e))?;
            debug!("{config:?}");

            // The monitoring servers run on this tokio runtime, next to Pingora's own.
            let runtime = tokio::runtime::Builder::new_multi_thread()
                .worker_threads(1)
                .enable_all()
                .build()?;
            let (proxy, state) = Proxy::from_config(config.proxy_config()?)?;
            let cancel_token = CancellationToken::new();
            let health_server = healthz_server::HttpServer::new(
                Arc::new(State::new(state)),
                config.healthcheck_port,
                cancel_token.child_token(),
            );
            let health_server_task = runtime.spawn(async move { health_server.start().await });
            let metrics_server_task = runtime.block_on(async {
                metrics_server::spawn(
                    Some(config.metrics_endpoint.to_string()),
                    cancel_token.child_token(),
                )
            });

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
