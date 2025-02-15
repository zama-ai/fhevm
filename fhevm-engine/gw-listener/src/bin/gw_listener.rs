use alloy::{primitives::Address, transports::http::reqwest::Url};
use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
struct Conf {
    #[arg(long)]
    database_url: Option<String>,

    #[arg(long, default_value = "16")]
    database_pool_size: u32,

    #[arg(long)]
    gw_url: Url,

    #[arg(short, long)]
    zkpok_manager_address: Address,

    #[arg(long, default_value = "1")]
    error_sleep_initial_secs: u16,

    #[arg(long, default_value = "10")]
    error_sleep_max_secs: u16,
}

fn main() {
    let _conf = Conf::parse();
}
