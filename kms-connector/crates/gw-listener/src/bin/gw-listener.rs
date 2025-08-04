use gw_listener::core::{Config, DbEventPublisher, GatewayListener};

use connector_utils::{
    cli::{Cli, Subcommands},
    conn::{connect_to_db, connect_to_gateway},
    signal::install_signal_handlers,
};
use tokio_util::sync::CancellationToken;
use tracing::{error, info};
use tracing_subscriber::{EnvFilter, fmt, layer::SubscriberExt, util::SubscriberInitExt};

#[tokio::main]
async fn main() {
    if let Err(err) = run().await {
        error!("{err}");
    }
}

async fn run() -> anyhow::Result<()> {
    rustls::crypto::aws_lc_rs::default_provider()
        .install_default()
        .expect("Failed to install AWS-LC crypto provider");
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let subcommand = Cli::new("GatewayListener").parse();
    match subcommand {
        Subcommands::Validate { config } => {
            Config::from_env_and_file(Some(config))?;
        }
        Subcommands::Start { config, name } => {
            // Load config and potentially override service name
            let mut config = Config::from_env_and_file(config.as_ref())?;
            if let Some(name) = name {
                config.service_name = name;
                info!("Using custom service name: {}", config.service_name);
            }

            let db_pool = connect_to_db(&config.database_url, config.database_pool_size).await?;
            let publisher = DbEventPublisher::new(db_pool);

            let provider = connect_to_gateway(&config.gateway_url).await?;
            let gw_listener = GatewayListener::new(&config, provider, publisher);

            let cancel_token = CancellationToken::new();
            install_signal_handlers(cancel_token.clone())?;

            info!("Starting GatewayListener with config:\n{}", config);
            gw_listener.start(cancel_token).await
        }
    }
    Ok(())
}
