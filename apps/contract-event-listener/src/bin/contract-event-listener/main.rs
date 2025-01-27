use alloy::primitives::Address;
use dotenv::from_path;
use fhevm_relayer::{EventRegistry, RealEventHandler};
use serde::{Deserialize, Serialize};
use std::env;
use std::path::Path;
use std::str::FromStr;
use std::sync::Arc;

#[derive(Debug, Serialize, Deserialize)]
pub struct ContractConfig {
    name: String,
    address: Address,
}

#[tokio::main]
async fn main() {
    // Handler
    let ws_url = "ws://localhost:8746";
    from_path(Path::new(".env")).ok();
    let oracle_address = env::var("DECRYPTION_ORACLE_ADDRESS").unwrap_or(
        String::from_str("0x6eeD9cadBB2c3d8338EB45745Ec32141cfE99aB1")
            .expect("Couln't parse string"),
    );
    let contract_address = Address::from_str(&oracle_address).expect("Invalid Ethereum address");
    let event_registry = EventRegistry::new();
    event_registry.register_contract(contract_address);
    let real_handler = RealEventHandler::new(ws_url, Arc::new(event_registry))
        .await
        .expect("Coulnd't properly create handler");
    let _ = real_handler.listen_for_contract_events().await;
}
