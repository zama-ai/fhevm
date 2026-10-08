use crate::decryption::types::DecryptionType;
use alloy::primitives::U256;
use clap::{Args, Parser, Subcommand};
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

    /// Sets the initial value of the request counter (DB and HTTP paths only).
    ///
    /// The DB path uses it as the decryption id, the HTTP path as the `extraData` nonce that makes
    /// every request's content-derived decryption id unique. Accepts a decimal or `0x`-prefixed
    /// hexadecimal `U256`. Defaults to a very high value to avoid colliding with ids that could be
    /// used in the testing environment.
    #[arg(long)]
    #[clap(value_parser = U256::from_str)]
    pub id_counter_start: Option<U256>,

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

    /// Perform stress tests by sending decryption requests to the connectors' HTTP endpoints
    Http(HttpTestArgs),

    /// Perform decryption benchmark by sending decryption requests to the connectors' HTTP
    /// endpoints
    BenchHttp(HttpBenchmarkArgs),
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

#[derive(Args, Debug)]
pub struct HttpTestArgs {
    /// Sets the type of decryption for the test session
    #[arg(short = 't', long)]
    #[clap(value_parser = DecryptionType::from_str, default_value = "public")]
    pub decryption_type: DecryptionType,
}

#[derive(Args)]
pub struct HttpBenchmarkArgs {
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
