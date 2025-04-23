use tracing::{info, warn};

use crate::{
    core::config::{
        Config, default_decryption_domain_name, default_decryption_domain_version,
        default_gateway_config_domain_name, default_gateway_config_domain_version,
    },
    error::Result,
};

/// Validate connector configuration
pub fn validate_config(config: &Config) -> Result<()> {
    info!("Validating KMS Core Connector configuration...");

    // Check S3 configuration - warn but don't fail if missing
    let s3_config_complete = config.s3_config.is_some();

    if !s3_config_complete {
        warn!("Optional S3 configuration is not provided. Some functionality may be limited.");
    }

    // Validate other critical configuration
    if config.gateway_url.is_empty() {
        return Err(crate::error::Error::Config(
            "Gateway URL is not configured".to_string(),
        ));
    }

    if config.kms_core_endpoint.is_empty() {
        return Err(crate::error::Error::Config(
            "KMS Core endpoint is not configured".to_string(),
        ));
    }

    if config.decryption_address.is_empty() {
        return Err(crate::error::Error::Config(
            "Decryption address is not configured".to_string(),
        ));
    }

    if config.gateway_config_address.is_empty() {
        return Err(crate::error::Error::Config(
            "GatewayConfig address is not configured".to_string(),
        ));
    }

    // Validate domain name is not empty
    if config.decryption_domain_name.is_empty() {
        warn!(
            "Decryption domain name is empty, will use default '{}' at runtime",
            default_decryption_domain_name()
        );
    }

    if config.decryption_domain_version.is_empty() {
        warn!(
            "Decryption domain version is empty, will use default '{}' at runtime",
            default_decryption_domain_version()
        );
    }

    if config.gateway_config_domain_name.is_empty() {
        warn!(
            "GatewayConfig domain name is empty, will use default '{}' at runtime",
            default_gateway_config_domain_name()
        );
    }

    if config.gateway_config_domain_version.is_empty() {
        warn!(
            "GatewayConfig domain version is empty, will use default '{}' at runtime",
            default_gateway_config_domain_version()
        );
    }

    // Validate wallet configuration
    if config.mnemonic.is_empty() && config.signing_key_path.is_none() {
        return Err(crate::error::Error::Config(
            "Either mnemonic or signing key path must be configured".to_string(),
        ));
    }

    info!("Configuration validation successful");
    Ok(())
}
