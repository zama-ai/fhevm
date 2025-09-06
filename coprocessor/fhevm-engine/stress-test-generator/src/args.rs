use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[command(version, about, long_about = None)]
pub struct Args {
    /// Run the API server
    #[arg(long)]
    pub run_server: bool,

    /// The address to listen on
    #[arg(long, default_value = "0.0.0.0:3000")]
    pub listen_address: String,

    /// The channel to notify for ZK proof events
    #[arg(long, default_value = "event_zkpok_new_work")]
    pub zkproof_notify_channel: String,

    /// The log level for the application
    #[arg(long, default_value = "info")]
    pub log_level: String,
}

pub fn parse_args() -> Args {
    Args::parse()
}
