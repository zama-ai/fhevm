use crate::decryption::types::DecryptionType;
use clap::{Args, Parser, Subcommand, command};
use std::{path::PathBuf, str::FromStr, time::Duration};

#[derive(Parser)]
#[command(version, about, long_about = None)]
pub struct Cli {
    /// The path to the testing configuration file
    #[arg(short, long, value_name = "FILE")]
    pub config: PathBuf,

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
    /// Perform decryption stress tests using the Gateway chain
    Gw(GwTestArgs),

    /// Perform decryption benchmark using the Gateway chain
    BenchGw(GwBenchmarkArgs),

    /// Perform stress tests by inserting decryption requests directly in connectors' DB
    Db(DbTestArgs),

    /// Perform decryption benchmark by inserting decryption requests directly in connectors' DB
    BenchDb(DbBenchmarkArgs),
}

#[derive(Args, Debug)]
pub struct GwTestArgs {
    /// Sets the type of decryption for the test session
    #[arg(short = 't', long)]
    #[clap(value_parser = DecryptionType::from_str, default_value = "public")]
    pub decryption_type: DecryptionType,
}

#[derive(Args)]
pub struct GwBenchmarkArgs {
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
    /// Skip the database tables' clear before and after running the tests
    #[arg(long, default_value = "false")]
    pub skip_clear_db: bool,

    /// Sets the type of decryption for the test session
    #[arg(short = 't', long)]
    #[clap(value_parser = DecryptionType::from_str, default_value = "public")]
    pub decryption_type: DecryptionType,
}

#[derive(Args)]
pub struct DbBenchmarkArgs {
    /// Skip the database tables' clear before and after running the tests
    #[arg(long, default_value = "false")]
    pub skip_clear_db: bool,

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
