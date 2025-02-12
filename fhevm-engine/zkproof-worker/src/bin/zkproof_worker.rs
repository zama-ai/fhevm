use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(short, long)]
    database_url: Option<String>,

    #[arg(long, default_value = "10")]
    database_pool_size: u32,

    #[arg(long, default_value = "5")]
    database_polling_interval_secs: u16,

    #[arg(short, long, default_value = "verify_proof_resquests")]
    verify_proof_req_database_channel: String,

    #[arg(short, long, default_value = "16")]
    tokio_blocking_threads: usize,

    #[arg(long, default_value = "1")]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value = "10")]
    error_sleep_max_secs: u16,
}

fn main() {
    let _conf = Conf::parse();
}
