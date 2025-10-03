use crate::decryption::types::DecryptionType;
use clap::{Args, Parser, Subcommand, command};
use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The path to the testing configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: Option<PathBuf>,

    /// Enable sequential sending of request burst
    #[arg(short, long, default_value_t = false)]
    pub sequential: bool,

    /// Sets the number of parallel requests in one burst
    #[arg(short, long)]
    pub parallel: Option<u32>,

    /// Sets the duration of the test session
    #[arg(short, long)]
    #[clap(value_parser = humantime::parse_duration)]
    pub duration: Option<Duration>,

    /// Sets the time to wait between each request burst
    #[arg(short, long)]
    #[clap(value_parser = humantime::parse_duration)]
    pub interval: Option<Duration>,

    #[command(subcommand)]
    pub subcommand: Subcommands,
}

#[derive(Subcommand)]
pub enum Subcommands {
    /// Perform tests with public decryptions only via the Gateway chain
    Public,

    /// Perform tests with user decryptions only via the Gateway chain
    User,

    /// Perform tests with mixed decryptions (both public and user) via the Gateway chain
    Mixed,

    /// Perform decryption benchmark using the Gateway chain
    Benchmark(BenchmarkArgs),

    /// Perform stress tests by inserting decryption request directly in connectors' DB
    Db(DbTestArgs),
}

#[derive(Args)]
pub struct BenchmarkArgs {
    /// CSV input file describing the benchmarks to run
    #[arg(short, long)]
    pub input: PathBuf,

    /// CSV output file containing the benchmarks results summary
    #[arg(short, long)]
    pub output: PathBuf,

    /// Optional CSV output file containing the full benchmarks results
    #[arg(short, long)]
    pub results: Option<PathBuf>,
}

#[derive(Args, Debug)]
pub struct DbTestArgs {
    /// Clear database tables before starting test
    #[arg(long)]
    pub clear_db: bool,

    /// Sets the type of decryption for the test session
    #[arg(short = 't', long)]
    #[clap(value_parser = DecryptionType::from_str, default_value = "public")]
    pub decryption_type: DecryptionType,
}
