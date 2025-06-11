//! Module exposing a trait to deserialize raw configurations of KMS Connector's subcomponents.
//!
//! These raw configurations can then be parsed properly.

use super::Result;
use config::{Config as ConfigBuilder, Environment, File, FileFormat};
use serde::Deserialize;
use std::path::Path;
use tracing::info;

pub trait DeserializeRawConfig {
    fn from_env_and_file<P: AsRef<Path>>(path: Option<P>) -> Result<Self>
    where
        for<'a> Self: Sized + Deserialize<'a>,
    {
        let mut builder = ConfigBuilder::builder();

        // If path is provided, add it as a config source
        if let Some(path) = path {
            info!(
                "Loading configuration from file: {}",
                path.as_ref().display()
            );
            builder = builder.add_source(
                File::with_name(path.as_ref().to_str().unwrap()).format(FileFormat::Toml),
            );
        }

        // Add environment variables last so they take precedence
        info!("Adding environment variables with prefix KMS_CONNECTOR_");
        builder = builder.add_source(
            Environment::with_prefix("KMS_CONNECTOR")
                .prefix_separator("_")
                .separator("__"),
        );

        let settings = builder.build()?;
        let config = settings.try_deserialize()?;
        Ok(config)
    }
}
