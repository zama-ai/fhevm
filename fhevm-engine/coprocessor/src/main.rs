use fhevm_engine_common::keys::{FhevmKeys, SerializedFhevmKeys};
use tokio::task::JoinSet;

mod cli;
mod db_queries;
mod server;
#[cfg(test)]
mod tests;
mod tfhe_worker;
mod types;
mod utils;

fn main() {
    let args = crate::cli::parse_args();
    assert!(
        args.work_items_batch_size < args.tenant_key_cache_size,
        "Work items batch size must be less than tenant key cache size"
    );

    if args.generate_fhe_keys {
        generate_dump_fhe_keys();
    } else {
        start_runtime(args, None);
    }
}

// separate function for testing
pub fn start_runtime(
    args: crate::cli::Args,
    close_recv: Option<tokio::sync::watch::Receiver<bool>>,
) {
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        // not using tokio main to specify max blocking threads
        .max_blocking_threads(args.coprocessor_fhe_threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Some(mut close_recv) = close_recv {
                tokio::select! {
                    main = async_main(args) => {
                        if let Err(e) = main {
                            eprintln!("Runtime error: {:?}", e);
                        }
                    }
                    _ = close_recv.changed() => {
                        eprintln!("Service stopped voluntarily");
                    }
                }
            } else {
                if let Err(e) = async_main(args).await {
                    eprintln!("Runtime error: {:?}", e);
                }
            }
        })
}

async fn async_main(
    args: crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let mut set = JoinSet::new();
    if args.run_server {
        println!("Initializing api server");
        set.spawn(crate::server::run_server(args.clone()));
    }

    if args.run_bg_worker {
        println!("Initializing background worker");
        set.spawn(crate::tfhe_worker::run_tfhe_worker(args.clone()));
    }

    if set.is_empty() {
        panic!("No tasks specified to run");
    }

    while let Some(res) = set.join_next().await {
        let _ = res?;
    }

    Ok(())
}

fn generate_dump_fhe_keys() {
    let keys = FhevmKeys::new();
    let ser_keys: SerializedFhevmKeys = keys.into();
    ser_keys.save_to_disk();
}
