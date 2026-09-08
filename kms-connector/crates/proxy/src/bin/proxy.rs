use connector_utils::{
    cli::{Cli, Subcommands},
    config::DeserializeConfig,
    monitoring::otlp::init_otlp_setup,
};
use proxy::core::{Config, Proxy};
use std::process::ExitCode;
use tracing::{debug, error};

fn main() -> ExitCode {
    if let Err(err) = run() {
        error!("{err}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}

// Unlike the other connector services, the proxy does not run on `#[tokio::main]`: Pingora
// manages its own runtimes and blocks the main thread.
fn run() -> anyhow::Result<()> {
    match Cli::new("Proxy").parse() {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Health { endpoint: _ } => {
            todo!("Proxy healthcheck is not implemented yet")
        }
        Subcommands::Start { config } => {
            let config = Config::from_env_and_file(config.as_ref())?;
            debug!("{config:?}");

            // The tonic OTLP exporter spawns its channel worker with `tokio::spawn` at build time,
            // so a runtime must be entered here. The worker then runs on this runtime's threads,
            // so it must stay alive for the whole process lifetime.
            let runtime = tokio::runtime::Runtime::new()?;
            let _guard = runtime.enter();
            init_otlp_setup(config.service_name.clone())?;

            Proxy::from_config(config)?.run()?;
        }
    }
    Ok(())
}
