use std::str::FromStr;

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

    assert_eq!(first_resp.input_handles.len(), 5);

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
    assert_eq!(resp.values.len(), 5);

    assert_eq!(resp.values[0].output_type, 1);
    assert_eq!(resp.values[0].value, "false");
    assert_eq!(resp.values[1].output_type, 2);
    assert_eq!(resp.values[1].value, "1");
    assert_eq!(resp.values[2].output_type, 3);
    assert_eq!(resp.values[2].value, "2");
    assert_eq!(resp.values[3].output_type, 4);
    assert_eq!(resp.values[3].value, "3");

    Ok(())
}