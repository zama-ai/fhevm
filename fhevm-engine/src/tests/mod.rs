use std::str::FromStr;

use tonic::metadata::MetadataValue;
use utils::default_api_key;
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{AsyncComputation, AsyncComputeRequest, DebugDecryptRequest, DebugEncryptRequest, DebugEncryptRequestSingle, FheOperation};

mod utils;
mod operators;

#[tokio::test]
async fn test_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let app = utils::setup_test_app().await?;

    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let api_key_header = format!("Bearer {}", default_api_key());
    let ct_type = 4; // i32

    // encrypt two ciphertexts
    {
        let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
            values: vec![
                DebugEncryptRequestSingle {
                    handle: "0x0abc".to_string(),
                    le_value: vec![123],
                    output_type: ct_type,
                },
                DebugEncryptRequestSingle {
                    handle: "0x0abd".to_string(),
                    le_value: vec![124],
                    output_type: ct_type,
                },
            ],
        });
        encrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
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
        compute_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
        let resp = client.async_compute(compute_request).await?;
        println!("compute request: {:?}", resp);
    }

    println!("sleeping for computation to complete...");
    tokio::time::sleep(tokio::time::Duration::from_secs(3)).await;

    // decrypt values
    {
        let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
            handles: vec![
                "0x0abe".to_string(),
                "0x0abf".to_string(),
            ],
        });
        decrypt_request.metadata_mut().append("authorization", MetadataValue::from_str(&api_key_header).unwrap());
        let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;
        println!("decrypt request: {:?}", resp);
        assert_eq!(resp.get_ref().values.len(), 2);
        // first value
        assert_eq!(resp.get_ref().values[0].value, "247");
        assert_eq!(resp.get_ref().values[0].output_type, ct_type);
        // second value
        assert_eq!(resp.get_ref().values[1].value, "263");
        assert_eq!(resp.get_ref().values[1].output_type, ct_type);
    }

    Ok(())
}