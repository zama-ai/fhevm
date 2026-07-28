pub mod metrics;

use std::str::FromStr;

use alloy::primitives::Address;
use anyhow::{anyhow, Result};
use tracing::info;

/// CLI fragment shared by all host-listener binaries (main, poller, consumer).
/// Groups the two ProtocolConfig-related flags so they're declared and
/// documented once. `address` is kept as `String` (rather than
/// `Option<Address>`) to preserve the existing "empty string = not provided"
/// semantics used by the main host-listener, which the helm chart relies on
/// when `PROTOCOL_CONFIG_ADDRESS` is unset.
#[derive(clap::Args, Debug, Clone)]
pub struct ProtocolConfigArgs {
    #[arg(
        long = "protocol-config-address",
        env = "PROTOCOL_CONFIG_ADDRESS",
        default_value = "",
        help = "ProtocolConfig contract address to monitor"
    )]
    pub address: String,

    #[arg(
        id = "canonical_protocol_config_chain_id",
        long = "canonical-protocol-config-chain-id",
        value_name = "CANONICAL_PROTOCOL_CONFIG_CHAIN_ID",
        env = "CANONICAL_PROTOCOL_CONFIG_CHAIN_ID",
        help = "Chain id of the canonical chain hosting the ProtocolConfig contract. \
                The listener decodes ProtocolConfig.CoprocessorUpgradeProposed only when its \
                own chain id matches. Omit on listeners that don't run against the canonical chain."
    )]
    pub chain_id: Option<u64>,
}

impl ProtocolConfigArgs {
    /// Parses `address` into an `Address`. Returns `Ok(None)` when the flag
    /// was omitted (or passed empty by the helm template), `Err` when a
    /// non-empty value fails to parse as an EVM address.
    pub fn parsed_address(&self) -> Result<Option<Address>> {
        if self.address.is_empty() {
            return Ok(None);
        }
        Address::from_str(&self.address).map(Some).map_err(|err| {
            anyhow!("Invalid ProtocolConfig contract address: {err}")
        })
    }
}

/// True iff `canonical_protocol_config_chain_id == Some(chain_id)`. Rejects `Some(0)`; logs the resolved role.
pub fn resolve_protocol_config_listener(
    canonical_protocol_config_chain_id: Option<u64>,
    chain_id: u64,
) -> Result<bool> {
    if matches!(canonical_protocol_config_chain_id, Some(0)) {
        return Err(anyhow!(
            "--canonical-protocol-config-chain-id=0 is not a valid chain id; omit the flag to disable ProtocolConfig decoding"
        ));
    }
    let is_listener = canonical_protocol_config_chain_id == Some(chain_id);
    info!(
        is_protocol_config_listener = is_listener,
        chain_id,
        canonical_protocol_config_chain_id = ?canonical_protocol_config_chain_id,
        "Resolved ProtocolConfig listener role",
    );
    Ok(is_listener)
}
