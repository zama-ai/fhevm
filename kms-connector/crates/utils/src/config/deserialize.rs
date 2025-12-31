//! Module exposing a trait to deserialize raw configurations of KMS Connector's subcomponents.
//!
//! These raw configurations can then be parsed properly.

use super::Result;
use crate::{config::Error, monitoring::otlp::default_dispatcher};
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::Path;
use tracing::{error, info};

pub trait DeserializeConfig {
    /// Loads the configuration from environment variables and optionally from a TOML file.
    ///
    /// Environment variables take precedence over file configuration.
    /// Environment variables are prefixed with KMS_CONNECTOR_.
    fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self>
    where
        for<'a> Self: Sized + Deserialize<'a>,
    {
        // We use a temporary dispatcher to display the traces as the global dispatcher is not set
        // while parsing config. This is because the global dispatcher uses the `service_name`
        // field of the config.
        tracing::dispatcher::with_default(&default_dispatcher(), || {
            if let Some(config_path) = &path {
                info!("Loading config from: {}", config_path.as_ref().display());
            } else {
                info!("Loading config using environment variables only");
            }

            let mut builder = ConfigBuilder::builder();

            // If path is provided, add it as a config source
            if let Some(path) = path {
                let path_str = path
                    .as_ref()
                    .to_str()
                    .ok_or_else(|| Error::InvalidConfig("Invalid config path".to_string()))
                    .inspect_err(|e| error!("{e}"))?;
                builder = builder.add_source(File::with_name(path_str).format(FileFormat::Toml));
            }

            // Add environment variables last so they take precedence
            info!("Adding environment variables with prefix KMS_CONNECTOR_");
            builder = builder.add_source(
                Environment::with_prefix("KMS_CONNECTOR")
                    .prefix_separator("_")
                    .separator("__")
                    .list_separator(",")
                    .with_list_parse_key("kms_core_endpoints")
                    .try_parsing(true),
            );

            let settings = builder.build().inspect_err(|e| error!("{e}"))?;
            let config = settings.try_deserialize().inspect_err(|e| error!("{e}"))?;
            Ok(config)
        })
    }
}
