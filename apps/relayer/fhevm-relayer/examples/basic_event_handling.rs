use eyre::Result;
use fhevm_relayer::{
    config::settings::Settings, event::registry::EventRegistry, service::handler::RealEventHandler,
};
use std::sync::Arc;
use tracing::{info, Level};
use tracing_subscriber::FmtSubscriber;

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize basic logging
    FmtSubscriber::builder().with_max_level(Level::DEBUG).init();

    info!("Starting basic event handling example");

    // Load test configuration
    let settings = Settings::new()?;

    // Create registry and handler
    let registry = Arc::new(EventRegistry::new());
    let handler =
        Arc::new(RealEventHandler::new(&settings.network.ws_url, registry.clone()).await?);
    // Register a test contract and events
    let test_address = "0x742d35Cc6634C0532925a3b844Bc454e4438f44e"
        .parse()
        .expect("Invalid test address");

    registry.register_contract(test_address);

    // Example of registering a custom event processor
    registry.register_event(
        test_address,
        "Transfer(address,address,uint256)",
        handler.clone(),
    );

    // Start listening for events
    info!("Starting event listener...");
    handler.listen_for_contract_events().await?;

    Ok(())
}

// Example of running:
// cargo run --example basic_event_handling
