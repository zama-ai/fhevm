use clap::{Args, Parser, Subcommand, command};
use std::path::PathBuf;

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The path to the testing configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable sequential sending of burst requests
    #[arg(short, long, default_value_t = false)]
    pub sequential: bool,

    /// Sets the number of parallel requests in one burst
    #[arg(short, long)]
    pub parallel: Option<u32>,

    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Perform tests with public decryptions only
    Public,

    /// Perform tests with user decryptions only
    User,

    /// Perform tests with mixed decryptions (both public and user)
    Mixed,
    
    /// Test database connectors with direct insertions
    DbConnector(DbConnectorArgs),
}

#[derive(Args, Debug)]
pub struct DbConnectorArgs {
    /// Path to configuration file (overrides default location)
    #[arg(short = 'c', long)]
    pub config: Option<PathBuf>,
    
    /// Override request type (public, user, or mixed)
    /// Default comes from config file
    #[arg(short = 't', long = "request-type")]
    pub request_type: Option<String>,
    
    /// Override test duration (e.g., "30s", "5m", "1h")
    /// Default comes from config file
    #[arg(long)]
    pub duration: Option<String>,
    
    /// Override batch size for CI/load testing scenarios
    /// Default comes from config file
    #[arg(short = 'b', long)]
    pub batch_size: Option<usize>,
    
    /// Override batch interval (e.g., "1s", "500ms")
    /// Default comes from config file
    #[arg(short = 'i', long)]
    pub interval: Option<String>,
    
    /// Number of database URLs to use from config (default: all)
    #[arg(short = 'n', long)]
    pub num_connectors: Option<usize>,
    
    /// Enable response tracking (useful for CI verification)
    #[arg(long)]
    pub track_responses: bool,
    
    /// Clear database tables before starting test
    #[arg(long)]
    pub clear_db: bool,
}
