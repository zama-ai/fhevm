use executor::server::common::FheOperation;
use executor::server::executor::sync_compute_response::Resp;
use executor::server::executor::{
    fhevm_executor_client::FhevmExecutorClient, SyncComputation, SyncComputeRequest,
};
use executor::server::executor::{sync_input::Input, SyncInput};
use tfhe::CompactCiphertextListBuilder;
use utils::get_test;

mod utils;

#[tokio::test]
async fn get_input_ciphertexts() -> Result<(), Box<dyn std::error::Error>> {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone()).await?;
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = bincode::serialize(&builder.push(10_u8).build()).unwrap();
    // TODO: tests for all types and avoiding passing in 2 as an identifier for FheUint8.
    let input_handle = test.input_handle(&list, 0, 2);
    let sync_input = SyncInput {
        input: Some(Input::InputHandle(input_handle.to_vec())),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheGetInputCiphertext.into(),
        result_handles: vec![vec![0xaa]],
        inputs: vec![sync_input],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        input_lists: vec![list],
    };
    let response = client.sync_compute(req).await?;
    let sync_compute_response = response.get_ref();
    match &sync_compute_response.resp {
        Some(Resp::ResultCiphertexts(cts)) => {
            match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
                (Some(ct), 1) => {
                    if ct.handle != input_handle || ct.ciphertext.is_empty() {
                        assert!(false);
                    }
                }
                _ => assert!(false),
            }
        }
        _ => assert!(false),
    }
    Ok(())
}
