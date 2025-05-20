use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    #[arg(long, default_value_t = 8)]
    pub fhe_compute_threads: usize,

    #[arg(long, default_value_t = 8)]
    pub policy_fhe_compute_threads: usize,

    #[arg(long, default_value = "127.0.0.1:50051")]
    pub server_addr: String,

    /// directory for fhe keys, target directory expected to contain files named:
    /// sks (server evaluation key), pks (compact public key), pp (public key params)
    #[arg(long)]
    pub fhe_keys_directory: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
