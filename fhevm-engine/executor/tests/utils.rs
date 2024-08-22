use clap::Parser;
use executor::{cli::Args, server};
use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};

pub struct TestInstance {
    pub keys: FhevmKeys,
    pub server_addr: String,
}

impl TestInstance {
    pub fn new() -> Self {
        // Get defaults by parsing a cmd line without any arguments.
        let args = Args::parse_from(&["test"]);

        let instance = TestInstance {
            keys: SerializedFhevmKeys::load_from_disk().into(),
            server_addr: format!("http://{}", args.server_addr),
        };

        std::thread::spawn(move || server::start(&args).expect("start server"));

        // TODO: a hacky way to wait for the server to start
        std::thread::sleep(std::time::Duration::from_millis(150));

        instance
    }
}
