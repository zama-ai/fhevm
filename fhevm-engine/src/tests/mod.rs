use tonic::metadata::MetadataValue;
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{AsyncComputeRequest, FheOperation, DebugEncryptRequest, DebugDecryptRequest, AsyncComputation};

mod utils;

#[tokio::test]
async fn test_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let app = utils::setup_test_app().await?;

    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

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
                    operation: FheOperation::FheAdd.into(),
                    is_scalar: true,
                    output_handle: "0x0abf".to_string(),
                    input_handles: vec![
                        "0x0abe".to_string(),
                        "0x0010".to_string(),
                    ]
                },
                AsyncComputation {
                    operation: FheOperation::FheAdd.into(),
                    is_scalar: false,
                    output_handle: "0x0abe".to_string(),
                    input_handles: vec![
                        "0x0abc".to_string(),
                        "0x0abd".to_string(),
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