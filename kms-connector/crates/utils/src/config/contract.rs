use crate::config::{Error, Result};
use alloy::primitives::Address;
use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};
use tracing::{info, warn};

/// Struct containing the information required to interact with a Gateway's contract.
#[derive(Clone, Debug)]
pub struct ContractConfig {
    pub contract_name: String,
    pub address: Address,
    pub domain_name: String,
    pub domain_version: String,
}

impl ContractConfig {
    /// Parses the `RawContractConfig` data from the configuration file.
    pub fn parse(contract_name: &str, raw_config: RawContractConfig) -> Result<Self> {
        if raw_config.address.is_empty() {
            return Err(Error::EmptyField(format!("{contract_name} address")));
        }

        if !raw_config.address.starts_with("0x") {
            return Err(Error::InvalidConfig(format!(
                "{contract_name} address must start with 0x"
            )));
        }
        let address = Address::from_str(&raw_config.address)
            .map_err(|e| Error::InvalidConfig(format!("Invalid {contract_name} address: {e}")))?;

        let domain_name = raw_config.domain_name.unwrap_or_else(|| {
            warn!("{contract_name} domain name is empty, will use default '{contract_name}'");
            contract_name.to_string()
        });

        // Check for characters that might cause issues in EIP-712 domain messages
        if domain_name.chars().any(|c| c.is_control()) {
            warn!(
                "  {contract_name} domain name contains control characters, may cause EIP-712 encoding issues"
            );
        } else if !domain_name.is_ascii() {
            warn!(
                "  {contract_name} domain name contains non-ASCII characters, may cause EIP-712 compatibility issues"
            );
        } else {
            info!("  {contract_name} domain name EIP-712 compatibility check: OK");
        }

        let domain_version = raw_config.domain_version.unwrap_or_else(|| {
            warn!("{contract_name} domain version is empty, will use default '1'");
            "1".to_string()
        });

        Ok(Self {
            contract_name: contract_name.to_string(),
            address,
            domain_name,
            domain_version,
        })
    }
}

impl Display for ContractConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(
            f,
            "{} contract address: {}",
            self.contract_name, self.address
        )?;
        writeln!(
            f,
            "{} Domain Name: {}",
            self.contract_name, self.domain_name
        )?;
        write!(
            f,
            "{} Domain Version: {}",
            self.contract_name, self.domain_version,
        )
    }
}

/// Struct used to deserialize the Gateway's contracts information from the configuration file.
#[derive(Clone, Debug, Default, Deserialize, Serialize)]
pub struct RawContractConfig {
    pub address: String,
    pub domain_name: Option<String>,
    pub domain_version: Option<String>,
}
