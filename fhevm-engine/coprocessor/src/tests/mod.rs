use std::str::FromStr;

use crate::server::common::FheOperation;
use crate::server::coprocessor::async_computation_input::Input;
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{
    AsyncComputation, AsyncComputationInput, AsyncComputeRequest, DebugDecryptRequest,
    DebugEncryptRequest, DebugEncryptRequestSingle,
};
use tonic::metadata::MetadataValue;
use utils::{default_api_key, random_handle, wait_until_all_ciphertexts_computed};

mod errors;
mod inputs;
mod operators;
mod utils;

#[tokio::test]
async fn test_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let app = utils::setup_test_app().await?;

    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let api_key_header = format!("bearer {}", default_api_key());
    let ct_type = 4; // i32

    let h1 = random_handle().to_be_bytes();
    let h2 = random_handle().to_be_bytes();
    let h3 = random_handle().to_be_bytes();
    let h4 = random_handle().to_be_bytes();

    // encrypt two ciphertexts
    {
        let mut encrypt_request = tonic::Request::new(DebugEncryptRequest {
            values: vec![
                DebugEncryptRequestSingle {
                    handle: h1.to_vec(),
                    be_value: vec![123],
                    output_type: ct_type,
                },
                DebugEncryptRequestSingle {
                    handle: h2.to_vec(),
                    be_value: vec![124],
                    output_type: ct_type,
                },
            ],
        });
        encrypt_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.debug_encrypt_ciphertext(encrypt_request).await?;
        println!("encryption request: {:?}", resp);
    }

    // compute
    {
        let mut compute_request = tonic::Request::new(AsyncComputeRequest {
            computations: vec![
                AsyncComputation {
                    operation: FheOperation::FheAdd.into(),
                    output_handle: h3.to_vec(),
                    inputs: vec![
                        AsyncComputationInput {
                            input: Some(Input::InputHandle(h4.to_vec())),
                        },
                        AsyncComputationInput {
                            input: Some(Input::Scalar(vec![0x00, 0x10])),
                        },
                    ],
                },
                AsyncComputation {
                    operation: FheOperation::FheAdd.into(),
                    output_handle: h4.to_vec(),
                    inputs: vec![
                        AsyncComputationInput {
                            input: Some(Input::InputHandle(h1.to_vec())),
                        },
                        AsyncComputationInput {
                            input: Some(Input::InputHandle(h2.to_vec())),
                        },
                    ],
                },
            ],
        });
        compute_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.async_compute(compute_request).await?;
        println!("compute request: {:?}", resp);
    }

    println!("sleeping for computation to complete...");
    wait_until_all_ciphertexts_computed(&app).await?;

    // decrypt values
    {
        let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
            handles: vec![h4.to_vec(), h3.to_vec()],
        });
        decrypt_request.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
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
