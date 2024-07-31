use server::coprocessor::AsyncComputeRequest;
use tokio::task::JoinSet;
use tonic::metadata::MetadataValue;

mod server;
mod db_queries;
mod cli;
mod types;
mod utils;
mod tfhe_worker;
mod tfhe_ops;

fn main() {
    let args = crate::cli::parse_args();
    assert!(args.work_items_batch_size < args.tenant_key_cache_size, "Work items batch size must be less than tenant key cache size");

    // TODO: check that computation has uniform input types
    tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        // not using tokio main to specify max blocking threads
        .max_blocking_threads(args.coprocessor_fhe_threads)
        .enable_all()
        .build()
        .unwrap()
        .block_on(async {
            if let Err(e) = async_main(args).await {
                eprintln!("Runtime error: {:?}", e);
            }
        })
}

async fn async_main(args: crate::cli::Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    if args.run_custom_function {
        custom_function().await?;
        return Ok(());
    }

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

async fn custom_function() -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let gen_keys = false;
    if gen_keys {
        let (client_key, server_key) = tfhe::generate_keys(tfhe::ConfigBuilder::default().build());
        let compact_key = tfhe::CompactPublicKey::new(&client_key);
        let client_key = bincode::serialize(&client_key).unwrap();
        let server_key = bincode::serialize(&server_key).unwrap();
        let compact_key = bincode::serialize(&compact_key).unwrap();
        std::fs::create_dir_all("fhevm-keys").unwrap();
        std::fs::write("fhevm-keys/cks", client_key).unwrap();
        std::fs::write("fhevm-keys/pks", compact_key).unwrap();
        std::fs::write("fhevm-keys/sks", server_key).unwrap();
    }

    use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
    use crate::server::coprocessor::{DebugEncryptRequest, DebugDecryptRequest, AsyncComputation};
    let mut client = FhevmCoprocessorClient::connect(
        "http://127.0.0.1:50051"
    ).await?;

    let api_key = "Bearer a1503fb6-d79b-4e9e-826d-44cf262f3e05";

    // ciphertext A
    {
        let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
            handle: "0x0abc".to_string(),
            original_value: 123,
        });
        encrypt_request.metadata_mut().append("authorization", MetadataValue::from_static(api_key));
        let resp = client.debug_encrypt_ciphertext(encrypt_request).await?;
        println!("encryption request: {:?}", resp);
    }

    // ciphertext B
    {
        let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
            handle: "0x0abd".to_string(),
            original_value: 124,
        });
        encrypt_request.metadata_mut().append("authorization", MetadataValue::from_static(api_key));
        let resp = client.debug_encrypt_ciphertext(encrypt_request).await?;
        println!("encryption request: {:?}", resp);
    }

    // compute
    {
        let mut compute_request = tonic::Request::new(AsyncComputeRequest {
            computations: vec![
                AsyncComputation {
                    operation: 1,
                    is_scalar: false,
                    output_handle: "0x0abe".to_string(),
                    input_handles: vec![
                        "0x0abc".to_string(),
                        "0x0abd".to_string(),
                    ]
                },
                AsyncComputation {
                    operation: 1,
                    is_scalar: true,
                    output_handle: "0x0abf".to_string(),
                    input_handles: vec![
                        "0x0abe".to_string(),
                        "0x0010".to_string(),
                    ]
                },
            ]
        });
        compute_request.metadata_mut().append("authorization", MetadataValue::from_static(api_key));
        let resp = client.async_compute(compute_request).await?;
        println!("compute request: {:?}", resp);
    }

    println!("sleeping for computation to complete...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // decrypt first
    {
        let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
            handle: "0x0abe".to_string()
        });
        decrypt_request.metadata_mut().append("authorization", MetadataValue::from_static(api_key));
        let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;
        println!("decrypt request: {:?}", resp);
        assert_eq!(resp.get_ref().value, "247");
    }

    // decrypt second
    {
        let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
            handle: "0x0abf".to_string()
        });
        decrypt_request.metadata_mut().append("authorization", MetadataValue::from_static(api_key));
        let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;
        println!("decrypt request: {:?}", resp);
        assert_eq!(resp.get_ref().value, "263");
    }

    Ok(())
}