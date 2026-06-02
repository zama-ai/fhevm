pub mod metrics;

use anyhow::{anyhow, Result};
use tracing::info;

/// True iff `ethereum_chain_id == Some(chain_id)`. Rejects `Some(0)`; logs the resolved role.
pub fn resolve_protocol_config_listener(
    ethereum_chain_id: Option<u64>,
    chain_id: u64,
) -> Result<bool> {
    if matches!(ethereum_chain_id, Some(0)) {
        return Err(anyhow!(
            "--ethereum-chain-id=0 is not a valid chain id; omit the flag to disable ProtocolConfig decoding"
        ));
    }
    let is_listener = ethereum_chain_id == Some(chain_id);
    info!(
        is_protocol_config_listener = is_listener,
        chain_id,
        ethereum_chain_id = ?ethereum_chain_id,
        "Resolved ProtocolConfig listener role",
    );
    Ok(is_listener)
}
