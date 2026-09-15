pub mod metrics;

use std::str::FromStr;

use alloy::primitives::Address;
use anyhow::{anyhow, Result};
use tracing::{info, warn};

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
/// Warns when the role and `protocol_config_address` disagree, since either mismatch silently disables decoding.
pub fn resolve_protocol_config_listener(
    canonical_protocol_config_chain_id: Option<u64>,
    chain_id: u64,
    protocol_config_address: Option<Address>,
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
        protocol_config_address = ?protocol_config_address,
        "Resolved ProtocolConfig listener role",
    );
    match (is_listener, protocol_config_address) {
        (true, None) => warn!(
            chain_id,
            "ProtocolConfig listener has no --protocol-config-address; \
             ProtocolConfig.CoprocessorUpgradeProposed events will not be decoded"
        ),
        (false, Some(address)) => warn!(
            chain_id,
            canonical_protocol_config_chain_id = ?canonical_protocol_config_chain_id,
            protocol_config_address = %address,
            "--protocol-config-address is set but this listener is not the ProtocolConfig listener; \
             ProtocolConfig.CoprocessorUpgradeProposed events will not be decoded. \
             Set --canonical-protocol-config-chain-id to this chain id or drop the address"
        ),
        (true, Some(_)) | (false, None) => {}
    }
    Ok(is_listener)
}

#[cfg(test)]
mod tests {
    use super::*;

    const NOT_DECODED: &str = "events will not be decoded";
    const NO_ADDRESS: &str = "has no --protocol-config-address";
    const NOT_LISTENER: &str = "not the ProtocolConfig listener";

    #[test]
    #[tracing_test::traced_test]
    fn listener_without_address_warns() {
        let is_listener =
            resolve_protocol_config_listener(Some(1), 1, None).unwrap();
        assert!(is_listener);
        assert!(logs_contain(NO_ADDRESS));
        assert!(!logs_contain(NOT_LISTENER));
    }

    #[test]
    #[tracing_test::traced_test]
    fn address_without_canonical_chain_warns() {
        let address = Some(Address::repeat_byte(0x11));
        let is_listener =
            resolve_protocol_config_listener(None, 1, address).unwrap();
        assert!(!is_listener);
        assert!(logs_contain(NOT_LISTENER));
        assert!(!logs_contain(NO_ADDRESS));
    }

    #[test]
    #[tracing_test::traced_test]
    fn non_listener_without_address_is_quiet() {
        let is_listener =
            resolve_protocol_config_listener(None, 1, None).unwrap();
        assert!(!is_listener);
        assert!(!logs_contain(NOT_DECODED));
    }
}
