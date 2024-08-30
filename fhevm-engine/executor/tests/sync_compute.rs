use executor::server::common::FheOperation;
use executor::server::executor::sync_compute_response::Resp;
use executor::server::executor::CompressedCiphertext;
use executor::server::executor::{
    fhevm_executor_client::FhevmExecutorClient, SyncComputation, SyncComputeRequest,
};
use executor::server::executor::{sync_input::Input, SyncInput};
use fhevm_engine_common::types::{SupportedFheCiphertexts, HANDLE_LEN};
use tfhe::CompactCiphertextListBuilder;
use utils::get_test;

mod utils;

#[tokio::test]
async fn get_input_ciphertext() {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = bincode::serialize(&builder.push(10_u8).build()).unwrap();
    // TODO: tests for all types and avoiding passing in 2 as an identifier for FheUint8.
    let input_handle = test.input_handle(&list, 0, 2);
    let sync_input = SyncInput {
        input: Some(Input::Handle(input_handle.clone())),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheGetCiphertext.into(),
        result_handles: vec![input_handle.clone()],
        inputs: vec![sync_input],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        compact_ciphertext_lists: vec![list],
        compressed_ciphertexts: vec![],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
            (Some(ct), 1) => {
                if ct.handle != input_handle || ct.serialization.is_empty() {
                    assert!(false, "response handle or ciphertext are unexpected");
                }
            }
            _ => assert!(false, "no response"),
        },
        Resp::Error(e) => assert!(false, "error: {}", e),
    }
}

#[tokio::test]
async fn compute_on_two_serialized_ciphertexts() {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = builder.push(10_u16).push(11_u16).build();
    let expander = list.expand_with_key(&test.keys.server_key).unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(expander.get(1).unwrap().unwrap());
    let ct2 = test.compress(ct2);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.ciphertext_handle(&ct2, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1, sync_input2],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        compact_ciphertext_lists: vec![],
        compressed_ciphertexts: vec![
            CompressedCiphertext {
                handle: handle1,
                serialization: ct1,
            },
            CompressedCiphertext {
                handle: handle2,
                serialization: ct2,
            },
        ],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
            (Some(ct), 1) => {
                if ct.handle != vec![0xaa; HANDLE_LEN] {
                    assert!(false, "response handle is unexpected");
                }
                let ct = SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                match ct
                    .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                    .as_str()
                {
                    "21" => (),
                    s => assert!(false, "unexpected result: {}", s),
                }
            }
            _ => assert!(false, "unexpected amount of result ciphertexts returned"),
        },
        Resp::Error(e) => assert!(false, "error response: {}", e),
    }
}

#[tokio::test]
async fn compute_on_compact_and_serialized_ciphertexts() {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder_input = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let compact_list = bincode::serialize(&builder_input.push(10_u16).build()).unwrap();
    let mut builder_cts = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = builder_cts.push(11_u16).build();
    let expander = list.expand_with_key(&test.keys.server_key).unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.input_handle(&compact_list, 0, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let computation = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1, sync_input2],
    };
    let req = SyncComputeRequest {
        computations: vec![computation],
        compact_ciphertext_lists: vec![compact_list],
        compressed_ciphertexts: vec![CompressedCiphertext {
            handle: handle1,
            serialization: ct1,
        }],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.first(), cts.ciphertexts.len()) {
            (Some(ct), 1) => {
                if ct.handle != vec![0xaa; HANDLE_LEN] {
                    assert!(false, "response handle is unexpected");
                }
                let ct = SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                match ct
                    .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                    .as_str()
                {
                    "21" => (),
                    s => assert!(false, "unexpected result: {}", s),
                }
            }
            _ => assert!(false, "unexpected amount of result ciphertexts returned"),
        },
        Resp::Error(e) => assert!(false, "error response: {}", e),
    }
}

#[tokio::test]
async fn compute_on_result_ciphertext() {
    let test = get_test().await;
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = CompactCiphertextListBuilder::new(&test.keys.compact_public_key);
    let list = builder.push(10_u16).push(11_u16).build();
    let expander = list.expand_with_key(&test.keys.server_key).unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(expander.get(1).unwrap().unwrap());
    let ct2 = test.compress(ct2);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.ciphertext_handle(&ct2, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let computation1 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1, sync_input2.clone()],
    };
    let sync_input3 = SyncInput {
        input: Some(Input::Handle(vec![0xaa; HANDLE_LEN])),
    };
    // 10 + 11 = 21. Then, add the 21 result to 11 and expect 32.
    let computation2 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xbb; HANDLE_LEN]],
        inputs: vec![sync_input3, sync_input2],
    };
    let req = SyncComputeRequest {
        computations: vec![computation1, computation2],
        compact_ciphertext_lists: vec![],
        compressed_ciphertexts: vec![
            CompressedCiphertext {
                handle: handle1,
                serialization: ct1,
            },
            CompressedCiphertext {
                handle: handle2,
                serialization: ct2,
            },
        ],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => match (cts.ciphertexts.get(1), cts.ciphertexts.len()) {
            (Some(ct), 2) => {
                if ct.handle != vec![0xbb; HANDLE_LEN] {
                    assert!(false, "response handle is unexpected");
                }
                let ct = SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                match ct
                    .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                    .as_str()
                {
                    "32" => (),
                    s => assert!(false, "unexpected result: {}", s),
                }
            }
            _ => assert!(false, "unexpected amount of result ciphertexts returned"),
        },
        Resp::Error(e) => assert!(false, "error response: {}", e),
    }
}
