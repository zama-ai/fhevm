use std::{sync::Arc, time::Duration};

use clap::Parser;
use executor::{cli::Args, server};
use fhevm_engine_common::{
    keys::{FhevmKeys, SerializedFhevmKeys},
    tfhe_ops::current_ciphertext_version,
    types::{Handle, SupportedFheCiphertexts},
};
use sha3::{Digest, Keccak256};
use tfhe::set_server_key;
use tokio::{sync::OnceCell, time::sleep};

pub struct TestInstance {
    pub keys: FhevmKeys,
    pub server_addr: String,
}

impl TestInstance {
    pub async fn new() -> Self {
        // Get defaults by parsing a cmd line without any arguments.
        let args = Args::parse_from(&["test"]);

        let instance = TestInstance {
            keys: SerializedFhevmKeys::load_from_disk().into(),
            server_addr: format!("http://{}", args.server_addr),
        };

        std::thread::spawn(move || server::start(&args).expect("start server"));

        // TODO: a hacky way to wait for the server to start
        sleep(Duration::from_secs(6)).await;

        instance
    }

    #[allow(dead_code)]
    pub fn input_handle(&self, list: &[u8], index: u8, ct_type: u8) -> Handle {
        let mut handle: Handle = Keccak256::digest(list).to_vec();
        handle[29] = index;
        handle[30] = ct_type;
        handle[31] = current_ciphertext_version() as u8;
        handle
    }

    pub fn ciphertext_handle(&self, ciphertext: &[u8], ct_type: u8) -> Handle {
        let mut handle: Handle = Keccak256::digest(&ciphertext).to_vec();
        handle[30] = ct_type;
        handle[31] = current_ciphertext_version() as u8;
        handle
    }

    pub fn compress(&self, ct: SupportedFheCiphertexts) -> Vec<u8> {
        set_server_key(self.keys.server_key.clone());
        ct.compress().1
    }
}

static TEST: OnceCell<Arc<TestInstance>> = OnceCell::const_new();

pub async fn get_test() -> Arc<TestInstance> {
    TEST.get_or_init(|| async { Arc::new(TestInstance::new().await) })
        .await
        .clone()
}
