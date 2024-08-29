use anyhow::{anyhow, Result};
use executor::server::common::FheOperation;
use executor::server::executor::sync_compute_response::Resp;
use executor::server::executor::Ciphertext;
use executor::server::executor::{
    fhevm_executor_client::FhevmExecutorClient, SyncComputation, SyncComputeRequest,
};
use executor::server::executor::{sync_input::Input, SyncInput};
use fhevm_engine_common::types::{SupportedFheCiphertexts, HANDLE_LEN};
use tfhe::CompactCiphertextListBuilder;
use utils::get_test;

mod utils;

#[tokio::test]
async fn get_input_ciphertext() -> Result<()> {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone()).await?;
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = bincode::serialize(&builder.push(10_u8).build())?;
    // TODO: tests for all types and avoiding passing in 2 as an identifier for FheUint8.
    let input_handle = test.input_handle(&list, 0, 2);
    let sync_input = SyncInput {
        input: Some(Input::InputHandle(input_handle.clone())),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheGetCiphertext.into(),
        result_handles: vec![input_handle.clone()],
        inputs: vec![sync_input],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        input_lists: vec![list],
    };
    let response = client.sync_compute(req).await?;
    let sync_compute_response = response.get_ref();
    let resp = <Option<Resp> as Clone>::clone(&sync_compute_response.resp)
        .ok_or_else(|| anyhow!("resp is None"))?;
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
            (Some(ct), 1) => {
                if ct.handle != input_handle || ct.ciphertext.is_empty() {
                    return Err(anyhow!("response handle or ciphertext are unexpected"));
                }
                Ok(())
            }
            _ => Err(anyhow!("unexpected amount of result ciphertexts returned")),
        },
        Resp::Error(e) => Err(anyhow!(format!("error response: {}", e))),
    }
}

#[tokio::test]
async fn fhe_compute_two_ciphertexts() -> Result<()> {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone()).await?;
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = builder.push(10_u16).push(11_u16).build();
    let expander = list.expand_with_key(&test.keys.server_key)?;
    let ct1 = SupportedFheCiphertexts::FheUint16(
        expander
            .get(0)
            .ok_or(anyhow!("missing ciphertext at index 0"))??,
    );
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(
        expander
            .get(1)
            .ok_or(anyhow!("missing ciphertext at index 1"))??,
    );
    let ct2 = test.compress(ct2);
    let sync_input1 = SyncInput {
        input: Some(Input::Ciphertext(Ciphertext {
            handle: test.ciphertext_handle(&ct1, 3).to_vec(),
            ciphertext: ct1,
        })),
    };
    let sync_input2 = SyncInput {
        input: Some(Input::Ciphertext(Ciphertext {
            handle: test.ciphertext_handle(&ct2, 3).to_vec(),
            ciphertext: ct2,
        })),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1, sync_input2],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        input_lists: vec![],
    };
    let response = client.sync_compute(req).await?;
    let sync_compute_response = response.get_ref();
    let resp = <Option<Resp> as Clone>::clone(&sync_compute_response.resp)
        .ok_or_else(|| anyhow!("resp is None"))?;
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
            (Some(ct), 1) => {
                if ct.handle != vec![0xaa; HANDLE_LEN] || ct.ciphertext.is_empty() {
                    return Err(anyhow!("response handle or ciphertext are unexpected"));
                }
                Ok(())
            }
            _ => Err(anyhow!("unexpected amount of result ciphertexts returned")),
        },
        Resp::Error(e) => Err(anyhow!(format!("error response: {}", e))),
    }
}
