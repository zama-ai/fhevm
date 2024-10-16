use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    #[arg(long, default_value_t = 4)]
    pub tokio_threads: usize,

    #[arg(long, default_value_t = 8)]
    pub fhe_compute_threads: usize,

    #[arg(long, default_value_t = 8)]
    pub fhe_operation_threads: usize,

    #[arg(long, default_value = "127.0.0.1:50051")]
    pub server_addr: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
