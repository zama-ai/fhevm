use std::str::FromStr;

use crate::server::common::FheOperation;
use crate::server::coprocessor::async_computation_input::Input;
use crate::server::coprocessor::fhevm_coprocessor_client::FhevmCoprocessorClient;
use crate::server::coprocessor::{
    AsyncComputation, AsyncComputationInput, AsyncComputeRequest, GetCiphertextBatch,
    TrivialEncryptBatch, TrivialEncryptRequestSingle,
};
use fhevm_engine_common::tfhe_ops::current_ciphertext_version;
use tonic::metadata::MetadataValue;
use utils::{
    decrypt_ciphertexts, default_api_key, random_handle, wait_until_all_ciphertexts_computed,
};

mod errors;
mod inputs;
mod operators;
mod random;
mod utils;

#[tokio::test]
async fn test_smoke() -> Result<(), Box<dyn std::error::Error>> {
    let app = utils::setup_test_app().await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;

    let api_key_header = format!("bearer {}", default_api_key());
    let ct_type = 4; // i32

    let h1 = random_handle().to_be_bytes();
    let h2 = random_handle().to_be_bytes();
    let h3 = random_handle().to_be_bytes();
    let h4 = random_handle().to_be_bytes();
    // unused, non existing
    let h5 = random_handle().to_be_bytes();

    // encrypt two ciphertexts
    {
        let mut encrypt_request = tonic::Request::new(TrivialEncryptBatch {
            values: vec![
                TrivialEncryptRequestSingle {
                    handle: h1.to_vec(),
                    be_value: vec![123],
                    output_type: ct_type,
                },
                TrivialEncryptRequestSingle {
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
        let resp = client.trivial_encrypt_ciphertexts(encrypt_request).await?;
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
        let decrypt_request = vec![h4.to_vec(), h3.to_vec()];
        let resp = decrypt_ciphertexts(&pool, 1, decrypt_request).await?;
        println!("decrypt request: {:?}", resp);
        assert_eq!(resp.len(), 2);
        // first value
        assert_eq!(resp[0].value, "247");
        assert_eq!(resp[0].output_type, ct_type as i16);
        // second value
        assert_eq!(resp[1].value, "263");
        assert_eq!(resp[1].output_type, ct_type as i16);
    }

    // compute
    {
        let mut get_cts_req = tonic::Request::new(GetCiphertextBatch {
            handles: vec![
                h1.to_vec(),
                h2.to_vec(),
                h3.to_vec(),
                h4.to_vec(),
                h5.to_vec(),
            ],
        });
        get_cts_req.metadata_mut().append(
            "authorization",
            MetadataValue::from_str(&api_key_header).unwrap(),
        );
        let resp = client.get_ciphertexts(get_cts_req).await?;
        let output = resp.get_ref();
        assert_eq!(output.responses.len(), 5);

        assert_eq!(output.responses[0].handle, h1);
        assert_eq!(output.responses[1].handle, h2);
        assert_eq!(output.responses[2].handle, h3);
        assert_eq!(output.responses[3].handle, h4);
        assert_eq!(output.responses[4].handle, h5);

        assert!(output.responses[0].ciphertext.is_some());
        assert!(output.responses[1].ciphertext.is_some());
        assert!(output.responses[2].ciphertext.is_some());
        assert!(output.responses[3].ciphertext.is_some());
        assert!(output.responses[4].ciphertext.is_none());

        let ct1 = output.responses[0].ciphertext.clone().unwrap();
        let ct2 = output.responses[1].ciphertext.clone().unwrap();
        let ct3 = output.responses[2].ciphertext.clone().unwrap();
        let ct4 = output.responses[3].ciphertext.clone().unwrap();

        assert_eq!(ct1.ciphertext_type, ct_type);
        assert_eq!(ct2.ciphertext_type, ct_type);
        assert_eq!(ct3.ciphertext_type, ct_type);
        assert_eq!(ct4.ciphertext_type, ct_type);

        assert_eq!(ct1.ciphertext_version, current_ciphertext_version() as i32);
        assert_eq!(ct2.ciphertext_version, current_ciphertext_version() as i32);
        assert_eq!(ct3.ciphertext_version, current_ciphertext_version() as i32);
        assert_eq!(ct4.ciphertext_version, current_ciphertext_version() as i32);

        assert_eq!(ct1.signature.len(), 65);
        assert_eq!(ct2.signature.len(), 65);
        assert_eq!(ct3.signature.len(), 65);
        assert_eq!(ct4.signature.len(), 65);
    }

    Ok(())
}

#[tokio::test]
#[ignore]
// custom test to run against local instance for decrypting custom ciphertexts
async fn test_custom_function() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect("postgresql://postgres:postgres@127.0.0.1:5432/coprocessor")
        .await?;

    let res = utils::decrypt_ciphertexts(
        &pool,
        1,
        vec![
            hex::decode("de2c33227b24ca797f7ad88495648446c70612c17f416d27513c77f2d0810200")
                .unwrap(),
            hex::decode("51d1d882d1e5ce54f15523558edd2746766c14cd5177faeb659418c57cec0200")
                .unwrap(),
            hex::decode("e3935354c48514fdfb0cbd965ad506d8865a2c88efffffca94dc9e0cecec0300")
                .unwrap(),
            hex::decode("3eed1ad1d1aa030b3bb3d3587ece4661a56945affcdee6bbdc02e28779380200")
                .unwrap(),
            hex::decode("55fe0c4283fbad83dc6fab91c3f85c098ada7a70ca8089e3076043efc9c60200")
                .unwrap(),
            hex::decode("3b42e61e197b88c083b4a2ab4b0ec542775e2282bebcc574e45d09f9779a0200")
                .unwrap(),
            hex::decode("9718b490a41e20fecaa90a7ab75e74de0c4105213ac3e5d8b5368ab813160200")
                .unwrap(),
            hex::decode("164c6d678ddf95f12bfa6b0fee7fd8b12e6221bd0c587640ae61dfc624f20200")
                .unwrap(),
            hex::decode("52a01af58c3d2b8ed1d04cd846706c1d214b72e079bd0930827628cb69180200")
                .unwrap(),
            hex::decode("d8d493764d46b62187b6a42917d58e297922d9ebab0dee306324e8c78a130200")
                .unwrap(),
        ],
    )
    .await
    .unwrap();

    println!("{:#?}", res);

    Ok(())
}

#[tokio::test]
#[ignore]
/// setup test data with keys
async fn setup_test_user() -> Result<(), Box<dyn std::error::Error>> {
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&std::env::var("DATABASE_URL").expect("expected to get db url"))
        .await?;

    utils::setup_test_user(&pool).await?;

    Ok(())
}
