use std::str::FromStr;

use tfhe::integer::{bigint::StaticUnsignedBigInt, U256};
use tonic::metadata::MetadataValue;

use crate::{db_queries::query_tenant_keys, server::coprocessor::{fhevm_coprocessor_client::FhevmCoprocessorClient, DebugDecryptRequest, InputToUpload, InputUploadBatch}, tests::utils::{default_api_key, default_tenant_id, setup_test_app}};


#[tokio::test]
async fn test_fhe_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let app = setup_test_app().await?;
    let mut client = FhevmCoprocessorClient::connect(app.app_url().to_string()).await?;
    let api_key_header = format!("bearer {}", default_api_key());
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(app.db_url())
        .await?;

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool).await.map_err(|e| {
        let e: Box<dyn std::error::Error> = e;
        e
    })?;
    let keys = &keys[0];

    let mut builder = tfhe::CompactCiphertextListBuilder::new(&keys.pks);
    let the_list = builder
        .push(false)
        .push(1u8)
        .push(2u16)
        .push(3u32)
        .push(4u64)
        .push(5u128)
        // TODO: 160 bits test
        .push(U256::from(7u32))
        .push(StaticUnsignedBigInt::<8>::from(8u32))
        .push(StaticUnsignedBigInt::<16>::from(9u32))
        .push(StaticUnsignedBigInt::<32>::from(10u32))
        .build();

    let serialized = bincode::serialize(&the_list).unwrap();

    println!("Encrypting inputs...");
    let mut input_request = tonic::Request::new(InputUploadBatch {
        input_ciphertexts: vec![
            InputToUpload {
                input_payload: serialized,
                signature: Vec::new(),
            }
        ]
    });
    input_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let resp = client.upload_inputs(input_request).await?;
    let resp = resp.get_ref();
    assert_eq!(resp.upload_responses.len(), 1);

    let first_resp = &resp.upload_responses[0];

    assert_eq!(first_resp.input_handles.len(), 10);

    let mut decr_handles: Vec<Vec<u8>> = Vec::new();
    for handle in &first_resp.input_handles {
        decr_handles.push(handle.handle.clone());
    }

    let mut decrypt_request = tonic::Request::new(DebugDecryptRequest {
        handles: decr_handles,
    });
    decrypt_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );
    let resp = client.debug_decrypt_ciphertext(decrypt_request).await?;
    let resp = resp.get_ref();
    assert_eq!(resp.values.len(), 10);

    assert_eq!(resp.values[0].output_type, 0);
    assert_eq!(resp.values[0].value, "false");
    assert_eq!(resp.values[1].output_type, 2);
    assert_eq!(resp.values[1].value, "1");
    assert_eq!(resp.values[2].output_type, 3);
    assert_eq!(resp.values[2].value, "2");
    assert_eq!(resp.values[3].output_type, 4);
    assert_eq!(resp.values[3].value, "3");
    assert_eq!(resp.values[4].output_type, 5);
    assert_eq!(resp.values[4].value, "4");
    assert_eq!(resp.values[5].output_type, 6);
    assert_eq!(resp.values[5].value, "5");
    assert_eq!(resp.values[6].output_type, 8);
    assert_eq!(resp.values[6].value, "7");
    assert_eq!(resp.values[7].output_type, 9);
    assert_eq!(resp.values[7].value, "8");
    assert_eq!(resp.values[8].output_type, 10);
    assert_eq!(resp.values[8].value, "9");
    assert_eq!(resp.values[9].output_type, 11);
    assert_eq!(resp.values[9].value, "10");

    Ok(())
}

#[ignore]
#[tokio::test]
// custom function for integration testing in development environment
// uploads ciphertext batch from inputs to local coprocessor database
async fn custom_insert_inputs() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_url = "http://127.0.0.1:50051";
    let db_url = "postgres://postgres:postgres@localhost/coprocessor";
    let api_key_header = format!("bearer {}", default_api_key());

    let mut client = FhevmCoprocessorClient::connect(grpc_url).await?;
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(2)
        .connect(&db_url)
        .await?;

    let keys = query_tenant_keys(vec![default_tenant_id()], &pool).await.map_err(|e| {
        let e: Box<dyn std::error::Error> = e;
        e
    })?;
    let keys = &keys[0];

    let mut builder = tfhe::CompactCiphertextListBuilder::new(&keys.pks);
    let the_list = builder
        .push(false)
        .push(1u8)
        .push(2u16)
        .push(3u32)
        .push(4u64)
        .push(5u64)
        .build();

    let serialized = bincode::serialize(&the_list).unwrap();

    println!("Encrypting inputs...");
    let mut input_request = tonic::Request::new(InputUploadBatch {
        input_ciphertexts: vec![
            InputToUpload {
                input_payload: serialized,
                signature: Vec::new(),
            }
        ]
    });
    input_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );

    let uploaded = client.upload_inputs(input_request).await?;
    let response = uploaded.get_ref();

    for (idx, ur) in response.upload_responses.iter().enumerate() {
        println!("request {idx}");
        for (idx, h) in ur.input_handles.iter().enumerate() {
            println!(" ct {idx} 0x{}", hex::encode(&h.handle));
        }
    }

    Ok(())
}

#[ignore]
#[tokio::test]
// custom function to decrypt the ciphertext from grpc
// ct_to_decrypt should be changed to your environment
async fn custom_decrypt_ct() -> Result<(), Box<dyn std::error::Error>> {
    let grpc_url = "http://127.0.0.1:50051";
    let api_key_header = format!("bearer {}", default_api_key());
    let ct_to_decrypt = "5bcaeef7d5bee3b5dffff3dfbfafcfb73cf57ddbbff73f777ffdfe677ebc0500";

    let mut client = FhevmCoprocessorClient::connect(grpc_url).await?;
    println!("Encrypting inputs...");
    let mut input_request = tonic::Request::new(DebugDecryptRequest {
        handles: vec![
            hex::decode(ct_to_decrypt).unwrap()
        ]
    });
    input_request.metadata_mut().append(
        "authorization",
        MetadataValue::from_str(&api_key_header).unwrap(),
    );

    let uploaded = client.debug_decrypt_ciphertext(input_request).await?;
    let response = uploaded.get_ref();

    println!("{:#?}", response);

    Ok(())
}