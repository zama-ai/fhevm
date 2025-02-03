use alloy::primitives::Address;
use fhevm_relayer::event::{processors::transfer::TransferProcessor, registry::EventRegistry};
use std::{str::FromStr, sync::Arc};

#[tokio::main]
async fn main() -> eyre::Result<()> {
    // Create a new event registry
    let registry = Arc::new(EventRegistry::new());

    // Create the test address
    let test_address = Address::from_str("0x0000000000000000000000000000000000000000")?;

    // Register the contract
    registry.register_contract(test_address);

    // Create and register the transfer processor
    let transfer_processor = TransferProcessor::new();
    registry.register_event(test_address, transfer_processor);

    println!("Successfully registered Transfer event handler");
    Ok(())
}
