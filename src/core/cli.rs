use crate::error::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, warn};

/// Environment variable to override the default config directory
const CONFIG_DIR_ENV: &str = "KMS_CONNECTOR_CONFIG_DIR";

#[derive(Parser)]
#[command(name = "kms-connector")]
#[command(about = "KMS Connector CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Start a KMS connector instance
    Start {
        /// Configuration file path (optional if using environment variables)
        #[arg(short, long, value_name = "FILE")]
        config: Option<PathBuf>,

        /// Optional instance name (overrides service_name in config)
        #[arg(short, long)]
        name: Option<String>,
    },

    /// List available configurations
    List {
        /// Show full configuration paths
        #[arg(short, long)]
        full_path: bool,
    },

    /// Validate a configuration file
    Validate {
        /// Configuration file path
        #[arg(short, long, value_name = "FILE")]
        config: PathBuf,
    },
}

impl Commands {
    /// Get the configuration directory path
    ///
    /// This function will:
    /// 1. Check for KMS_CONNECTOR_CONFIG_DIR environment variable
    /// 2. Fall back to default location relative to CARGO_MANIFEST_DIR
    ///
    /// The default location is: $CARGO_MANIFEST_DIR/config/environments
    pub fn get_config_dir() -> PathBuf {
        // First check if there's an override via environment variable
        if let Ok(config_dir) = std::env::var(CONFIG_DIR_ENV) {
            info!(
                "Using config directory from {}: {}",
                CONFIG_DIR_ENV, config_dir
            );
            PathBuf::from(config_dir)
        } else {
            let mut dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
            dir.push("config");
            dir.push("environments");
            info!("Using default config directory: {}", dir.display());
            dir
        }
    }

    /// List all available configuration files
    pub fn list_configs(full_path: bool) -> Result<Vec<PathBuf>> {
        let config_dir = Self::get_config_dir();

        if !config_dir.exists() {
            warn!("Config directory does not exist: {}", config_dir.display());
            return Ok(Vec::new());
        }

        let mut configs = Vec::new();
        for entry in std::fs::read_dir(config_dir)? {
            let entry = entry?;
            let path = entry.path();
            if path.is_file() && path.extension().is_some_and(|ext| ext == "toml") {
                if full_path {
                    configs.push(path);
                } else {
                    configs.push(PathBuf::from(path.file_name().unwrap()));
                }
            }
        }

        configs.sort();
        Ok(configs)
    }

    /// Validate a configuration file
    pub fn validate_config(config_path: &PathBuf) -> Result<()> {
        // Leverage existing Config parsing for validation
        super::config::Config::from_file(config_path)?;
        Ok(())
    }
}
