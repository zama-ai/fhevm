use std::{fs::read, path::Path};

use clap::{Parser, Subcommand};
use fhevm_engine_common::utils::{safe_deserialize_sns_key, safe_serialize_key};
use tfhe::ServerKey;
use tracing::{error, info};
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Debug, Subcommand)]
enum Commands {
    ExtractSksWithoutNoise {
        /// Server key with noise squashing enabled
        #[arg(long, default_value = "./sks_noise_squashing.bin")]
        src_path: String,

        /// Output server key with noise squashing disabled
        #[arg(long, default_value = "./sks_key.bin")]
        dst_path: String,
    },
}

/// Extracts the server key without noise squashing from the given path and saves it to the destination path.
pub fn extract_server_key_without_ns(src_path: String, dest_path: &String) -> bool {
    let dest_path = Path::new(dest_path);
    let src_path = Path::new(&src_path);
    info!("Reading server key from file {:?}", src_path);

    let server_key: ServerKey = safe_deserialize_sns_key(&read(src_path).expect("read server key"))
        .expect("deserialize server key");

    let (
        sks,
        kskm,
        compression_key,
        decompression_key,
        noise_squashing_key,
        _noise_squashing_compression_key,
        re_randomization_keyswitching_key,
        tag,
    ) = server_key.into_raw_parts();

    if noise_squashing_key.is_none() {
        error!("Server key does not have noise squashing");
        return false;
    }

    info!("Creating file {:?}", dest_path);

    let bytes: Vec<u8> = safe_serialize_key(&ServerKey::from_raw_parts(
        sks,
        kskm,
        compression_key,
        decompression_key,
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        re_randomization_keyswitching_key,
        tag,
    ));

    std::fs::write(dest_path, bytes).expect("write sks");

    true
}

fn main() {
    tracing_subscriber::fmt().with_level(true).init();
    let args = Args::parse();
    match args.command {
        Commands::ExtractSksWithoutNoise { src_path, dst_path } => {
            if extract_server_key_without_ns(src_path, &dst_path) {
                info!("Server key without noise squashing saved to {:?}", dst_path);
            }
        }
    }
}
