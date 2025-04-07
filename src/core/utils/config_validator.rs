use tracing::{info, warn};

use crate::{
    core::config::{
        default_decryption_manager_domain_name, default_decryption_manager_domain_version,
        default_httpz_domain_name, default_httpz_domain_version, Config,
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
    if config.gwl2_url.is_empty() {
        return Err(crate::error::Error::Config(
            "Gateway L2 URL is not configured".to_string(),
        ));
    }

    if config.kms_core_endpoint.is_empty() {
        return Err(crate::error::Error::Config(
            "KMS Core endpoint is not configured".to_string(),
        ));
    }

    if config.decryption_manager_address.is_empty() {
        return Err(crate::error::Error::Config(
            "Decryption manager address is not configured".to_string(),
        ));
    }

    if config.httpz_address.is_empty() {
        return Err(crate::error::Error::Config(
            "HTTPZ address is not configured".to_string(),
        ));
    }

    // Validate domain name is not empty
    if config.decryption_manager_domain_name.is_empty() {
        warn!(
            "Decryption manager domain name is empty, will use default '{}' at runtime",
            default_decryption_manager_domain_name()
        );
    }

    if config.decryption_manager_domain_version.is_empty() {
        warn!(
            "Decryption manager domain version is empty, will use default '{}' at runtime",
            default_decryption_manager_domain_version()
        );
    }

    if config.httpz_domain_name.is_empty() {
        warn!(
            "HTTPZ domain name is empty, will use default '{}' at runtime",
            default_httpz_domain_name()
        );
    }

    if config.httpz_domain_version.is_empty() {
        warn!(
            "HTTPZ domain version is empty, will use default '{}' at runtime",
            default_httpz_domain_version()
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
