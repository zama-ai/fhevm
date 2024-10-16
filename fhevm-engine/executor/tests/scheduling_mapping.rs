use executor::server::common::FheOperation;
use executor::server::executor::sync_compute_response::Resp;
use executor::server::executor::CompressedCiphertext;
use executor::server::executor::{
    fhevm_executor_client::FhevmExecutorClient, SyncComputation, SyncComputeRequest,
};
use executor::server::executor::{sync_input::Input, SyncInput};
use fhevm_engine_common::types::{SupportedFheCiphertexts, HANDLE_LEN};
use std::time::SystemTime;
use tfhe::prelude::CiphertextList;
use tfhe::zk::ZkComputeLoad;
use tfhe::ProvenCompactCiphertextList;
use utils::get_test;
mod utils;

fn get_handle(h: u32) -> Vec<u8> {
    let mut res: Vec<u8> = Vec::with_capacity(HANDLE_LEN);
    let slice: [u8; 4] = h.to_be_bytes();
    for _i in 0..HANDLE_LEN / 4 {
        res.extend_from_slice(&slice);
    }
    res.to_vec()
}

#[tokio::test]
async fn schedule_multi_erc20() {
    let mut num_samples: usize = 2;
    let samples = std::env::var("FHEVM_TEST_NUM_SAMPLES");
    if let Ok(samples) = samples {
        num_samples = samples.parse::<usize>().unwrap();
    }
    let test = get_test().await;
    test.keys.set_server_key_for_current_thread();
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap()
        .max_decoding_message_size(usize::MAX);
    let mut builder = ProvenCompactCiphertextList::builder(&test.keys.compact_public_key);
    let list = builder
        .push(100_u64) // Balance source
        .push(10_u64) // Transfer amount
        .push(20_u64) // Balance destination
        .push(0_u64) // 0
        .build_with_proof_packed(&test.keys.public_params, &[], ZkComputeLoad::Proof)
        .unwrap();
    let expander = list.expand_without_verification().unwrap();
    let bals = SupportedFheCiphertexts::FheUint64(expander.get(0).unwrap().unwrap());
    let bals = test.compress(bals);
    let trxa = SupportedFheCiphertexts::FheUint64(expander.get(1).unwrap().unwrap());
    let trxa = test.compress(trxa);
    let bald = SupportedFheCiphertexts::FheUint64(expander.get(2).unwrap().unwrap());
    let bald = test.compress(bald);
    let zero = SupportedFheCiphertexts::FheUint64(expander.get(3).unwrap().unwrap());
    let zero = test.compress(zero);
    let handle_bals = test.ciphertext_handle(&bals, 5);
    let sync_input_bals = SyncInput {
        input: Some(Input::Handle(handle_bals.clone())),
    };
    let handle_trxa = test.ciphertext_handle(&trxa, 5);
    let sync_input_trxa = SyncInput {
        input: Some(Input::Handle(handle_trxa.clone())),
    };
    let handle_bald = test.ciphertext_handle(&bald, 5);
    let sync_input_bald = SyncInput {
        input: Some(Input::Handle(handle_bald.clone())),
    };
    let handle_zero = test.ciphertext_handle(&zero, 5);
    let sync_input_zero = SyncInput {
        input: Some(Input::Handle(handle_zero.clone())),
    };

    let mut computed_handles = vec![];
    for i in 0..=(num_samples * 4 - 1) as u32 {
        let input = Some(Input::Handle(get_handle(i)));
        computed_handles.push(SyncInput { input });
    }

    let mut computations = vec![];
    for i in 0..=(num_samples - 1) as u32 {
        computations.push(SyncComputation {
            operation: FheOperation::FheLe.into(),
            result_handles: vec![get_handle(i * 4)],
            inputs: vec![sync_input_trxa.clone(), sync_input_bals.clone()],
        }); // Compare trxa <= bals
        computations.push(SyncComputation {
            operation: FheOperation::FheIfThenElse.into(),
            result_handles: vec![get_handle(i * 4 + 1)],
            inputs: vec![
                computed_handles[(i * 4) as usize].clone(),
                sync_input_trxa.clone(),
                sync_input_zero.clone(),
            ],
        }); // if trxa <= bals then trxa else zero
        computations.push(SyncComputation {
            operation: FheOperation::FheSub.into(),
            result_handles: vec![get_handle(i * 4 + 2)],
            inputs: vec![
                sync_input_bals.clone(),
                computed_handles[(i * 4 + 1) as usize].clone(),
            ],
        }); // bals - trxa/zero
        computations.push(SyncComputation {
            operation: FheOperation::FheAdd.into(),
            result_handles: vec![get_handle(i * 4 + 3)],
            inputs: vec![
                sync_input_bald.clone(),
                computed_handles[(i * 4 + 1) as usize].clone(),
            ],
        }); // bald + trxa/zero
    }
    let req = SyncComputeRequest {
        computations,
        compact_ciphertext_lists: vec![],
        compressed_ciphertexts: vec![
            CompressedCiphertext {
                handle: handle_bals,
                serialization: bals,
            },
            CompressedCiphertext {
                handle: handle_trxa,
                serialization: trxa,
            },
            CompressedCiphertext {
                handle: handle_bald,
                serialization: bald,
            },
            CompressedCiphertext {
                handle: handle_zero,
                serialization: zero,
            },
        ],
    };
    let now = SystemTime::now();
    let response = client.sync_compute(req).await.unwrap();
    println!("Execution time: {}", now.elapsed().unwrap().as_millis());
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => {
            assert!(
                cts.ciphertexts.len() == num_samples * 4,
                "wrong number of output ciphertexts {} instead of {}",
                cts.ciphertexts.len(),
                num_samples * 4
            );
            for i in 0..=(num_samples * 4 - 1) as u32 {
                match &cts.ciphertexts[i as usize].handle {
                    a if *a == get_handle(i) => {
                        let mut tt = 0;
                        if i % 4 != 0 {
                            tt = 5;
                        }
                        let ctd = SupportedFheCiphertexts::decompress(
                            tt,
                            &cts.ciphertexts[i as usize].serialization,
                        )
                        .unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "true" if i % 4 == 0 => (), // trxa <= bals true
                            "10" if i % 4 == 1 => (),   // select trxa
                            "90" if i % 4 == 2 => (),   // bals - trxa
                            "30" if i % 4 == 3 => (),   // bald + trxa
                            s => assert!(
                                false,
                                "unexpected result: {} for handle 0x{:x}",
                                s, cts.ciphertexts[i as usize].handle[0]
                            ),
                        }
                    }
                    _ => assert!(
                        false,
                        "unexpected handle 0x{:x}",
                        cts.ciphertexts[i as usize].handle[0]
                    ),
                }
            }
        }
        Resp::Error(e) => assert!(false, "error response: {}", e),
    }
}
