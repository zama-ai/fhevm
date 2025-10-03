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

    /// Perform decryption benchmark
    Benchmark(BenchmarkArgs),
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
