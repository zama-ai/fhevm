use executor::server::common::FheOperation;
use executor::server::executor::sync_compute_response::Resp;
use executor::server::executor::CompressedCiphertext;
use executor::server::executor::{
    fhevm_executor_client::FhevmExecutorClient, SyncComputation, SyncComputeRequest,
};
use executor::server::executor::{sync_input::Input, SyncInput};
use executor::server::SyncComputeError;
use fhevm_engine_common::types::{SupportedFheCiphertexts, HANDLE_LEN};
use tfhe::prelude::CiphertextList;
use tfhe::zk::ZkComputeLoad;
use tfhe::ProvenCompactCiphertextList;
use utils::get_test;

mod utils;

#[tokio::test]
async fn schedule_circular_dependence() {
    let test = get_test().await;
    test.keys.set_server_key_for_current_thread();
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(vec![0xaa; HANDLE_LEN])),
    };
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(vec![0xbb; HANDLE_LEN])),
    };
    let sync_input3 = SyncInput {
        input: Some(Input::Handle(vec![0xcc; HANDLE_LEN])),
    };
    let computation1 = SyncComputation {
        operation: FheOperation::FheNeg.into(),
        result_handles: vec![vec![0xbb; HANDLE_LEN]],
        inputs: vec![sync_input1],
    };
    let computation2 = SyncComputation {
        operation: FheOperation::FheNeg.into(),
        result_handles: vec![vec![0xcc; HANDLE_LEN]],
        inputs: vec![sync_input2],
    };
    let computation3 = SyncComputation {
        operation: FheOperation::FheNeg.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input3],
    };
    let req = SyncComputeRequest {
        computations: vec![computation1, computation2, computation3],
        compact_ciphertext_lists: vec![],
        compressed_ciphertexts: vec![],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(_cts) => {
            panic!("Received ciphertext outputs despite circular dependence.")
        }
        Resp::Error(e) => assert!(
            e == SyncComputeError::UnsatisfiedDependence as i32,
            "Error response should be UnsatisfiedDependence but is {}",
            e
        ),
    }
}

#[tokio::test]
async fn schedule_dependent_computations() {
    let test = get_test().await;
    test.keys.set_server_key_for_current_thread();
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = ProvenCompactCiphertextList::builder(&test.keys.compact_public_key);
    let list = builder
        .push(3_u16)
        .push(5_u16)
        .push(7_u16)
        .push(11_u16)
        .push(13_u16)
        .build_with_proof_packed(
            &test.keys.public_params,
            &[],
            tfhe::zk::ZkComputeLoad::Proof,
        )
        .unwrap();
    let expander = list.expand_without_verification().unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(expander.get(1).unwrap().unwrap());
    let ct2 = test.compress(ct2);
    let ct3 = SupportedFheCiphertexts::FheUint16(expander.get(2).unwrap().unwrap());
    let ct3 = test.compress(ct3);
    let ct4 = SupportedFheCiphertexts::FheUint16(expander.get(3).unwrap().unwrap());
    let ct4 = test.compress(ct4);
    let ct5 = SupportedFheCiphertexts::FheUint16(expander.get(4).unwrap().unwrap());
    let ct5 = test.compress(ct5);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.ciphertext_handle(&ct2, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let handle3 = test.ciphertext_handle(&ct3, 3);
    let sync_input3 = SyncInput {
        input: Some(Input::Handle(handle3.clone())),
    };
    let handle4 = test.ciphertext_handle(&ct4, 3);
    let sync_input4 = SyncInput {
        input: Some(Input::Handle(handle4.clone())),
    };
    let handle5 = test.ciphertext_handle(&ct5, 3);
    let sync_input5 = SyncInput {
        input: Some(Input::Handle(handle5.clone())),
    };
    let sync_input6 = SyncInput {
        input: Some(Input::Handle(vec![0xaa; HANDLE_LEN])),
    };
    let sync_input7 = SyncInput {
        input: Some(Input::Handle(vec![0xbb; HANDLE_LEN])),
    };
    let sync_input8 = SyncInput {
        input: Some(Input::Handle(vec![0xcc; HANDLE_LEN])),
    };
    let computation1 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1, sync_input2],
    };
    let computation2 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xbb; HANDLE_LEN]],
        inputs: vec![sync_input3, sync_input4],
    };
    let computation3 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xcc; HANDLE_LEN]],
        inputs: vec![sync_input6, sync_input7],
    };
    let computation4 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xdd; HANDLE_LEN]],
        inputs: vec![sync_input5, sync_input8],
    };
    let req = SyncComputeRequest {
        computations: vec![computation4, computation3, computation2, computation1],
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
            CompressedCiphertext {
                handle: handle3,
                serialization: ct3,
            },
            CompressedCiphertext {
                handle: handle4,
                serialization: ct4,
            },
            CompressedCiphertext {
                handle: handle5,
                serialization: ct5,
            },
        ],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => {
            assert!(
                cts.ciphertexts.len() == 4,
                "wrong number of output ciphertexts {} instead of {}",
                cts.ciphertexts.len(),
                4
            );
            let aa: Vec<u8> = vec![0xaa; HANDLE_LEN];
            let bb: Vec<u8> = vec![0xbb; HANDLE_LEN];
            let cc: Vec<u8> = vec![0xcc; HANDLE_LEN];
            let dd: Vec<u8> = vec![0xdd; HANDLE_LEN];
            for ct in cts.ciphertexts.iter() {
                match &ct.handle {
                    a if *a == aa => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "8" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    b if *b == bb => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "18" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    c if *c == cc => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "26" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    d if *d == dd => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "39" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    _ => panic!("unexpected handle 0x{:x}", ct.handle[0]),
                }
            }
        }
        Resp::Error(e) => panic!("error response: {}", e),
    }
}

#[tokio::test]
async fn schedule_y_patterns() {
    let test = get_test().await;
    test.keys.set_server_key_for_current_thread();
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = ProvenCompactCiphertextList::builder(&test.keys.compact_public_key);
    let list = builder
        .push(1_u16)
        .push(2_u16)
        .push(3_u16)
        .push(4_u16)
        .push(5_u16)
        .build_with_proof_packed(
            &(*test.keys.public_params).clone(),
            &[],
            ZkComputeLoad::Proof,
        )
        .unwrap();
    let expander = list.expand_without_verification().unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(expander.get(1).unwrap().unwrap());
    let ct2 = test.compress(ct2);
    let ct3 = SupportedFheCiphertexts::FheUint16(expander.get(2).unwrap().unwrap());
    let ct3 = test.compress(ct3);
    let ct4 = SupportedFheCiphertexts::FheUint16(expander.get(3).unwrap().unwrap());
    let ct4 = test.compress(ct4);
    let ct5 = SupportedFheCiphertexts::FheUint16(expander.get(4).unwrap().unwrap());
    let ct5 = test.compress(ct5);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.ciphertext_handle(&ct2, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let handle3 = test.ciphertext_handle(&ct3, 3);
    let sync_input3 = SyncInput {
        input: Some(Input::Handle(handle3.clone())),
    };
    let handle4 = test.ciphertext_handle(&ct4, 3);
    let _sync_input4 = SyncInput {
        input: Some(Input::Handle(handle4.clone())),
    };
    let handle5 = test.ciphertext_handle(&ct5, 3);
    let sync_input5 = SyncInput {
        input: Some(Input::Handle(handle5.clone())),
    };
    let sync_input_aa = SyncInput {
        input: Some(Input::Handle(vec![0xaa; HANDLE_LEN])),
    };
    let sync_input_bb = SyncInput {
        input: Some(Input::Handle(vec![0xbb; HANDLE_LEN])),
    };
    let sync_input_cc = SyncInput {
        input: Some(Input::Handle(vec![0xcc; HANDLE_LEN])),
    };
    let _sync_input_dd = SyncInput {
        input: Some(Input::Handle(vec![0xdd; HANDLE_LEN])),
    };
    let sync_input_ee = SyncInput {
        input: Some(Input::Handle(vec![0xee; HANDLE_LEN])),
    };
    let sync_input_ff = SyncInput {
        input: Some(Input::Handle(vec![0xff; HANDLE_LEN])),
    };
    let _sync_input_99 = SyncInput {
        input: Some(Input::Handle(vec![0x99; HANDLE_LEN])),
    };
    // Pattern Y
    let computation1 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1.clone(), sync_input1.clone()],
    }; // Compute 1 + 1
    let computation2 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xbb; HANDLE_LEN]],
        inputs: vec![sync_input2.clone(), sync_input2.clone()],
    }; // 2 + 2
    let computation3 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xcc; HANDLE_LEN]],
        inputs: vec![sync_input_aa.clone(), sync_input_bb.clone()],
    }; // 2 + 4
    let computation4 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xdd; HANDLE_LEN]],
        inputs: vec![sync_input_cc.clone(), sync_input3.clone()],
    }; // 6 + 3
       // Pattern reverse Y
    let computation5 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xee; HANDLE_LEN]],
        inputs: vec![sync_input1.clone(), sync_input1.clone()],
    }; // 1 + 1
    let computation6 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xff; HANDLE_LEN]],
        inputs: vec![sync_input_ee.clone(), sync_input2.clone()],
    }; // 2 + 2
    let computation7 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0x99; HANDLE_LEN]],
        inputs: vec![sync_input5.clone(), sync_input_ff.clone()],
    }; // 5 + 4
    let computation8 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0x88; HANDLE_LEN]],
        inputs: vec![sync_input3.clone(), sync_input_ff.clone()],
    }; // 3 + 4
    let req = SyncComputeRequest {
        computations: vec![
            computation4,
            computation3,
            computation2,
            computation1,
            computation5,
            computation6,
            computation7,
            computation8,
        ],
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
            CompressedCiphertext {
                handle: handle3,
                serialization: ct3,
            },
            CompressedCiphertext {
                handle: handle4,
                serialization: ct4,
            },
            CompressedCiphertext {
                handle: handle5,
                serialization: ct5,
            },
        ],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => {
            assert!(
                cts.ciphertexts.len() == 8,
                "wrong number of output ciphertexts {} instead of {}",
                cts.ciphertexts.len(),
                8
            );
            let aa: Vec<u8> = vec![0xaa; HANDLE_LEN];
            let bb: Vec<u8> = vec![0xbb; HANDLE_LEN];
            let cc: Vec<u8> = vec![0xcc; HANDLE_LEN];
            let dd: Vec<u8> = vec![0xdd; HANDLE_LEN];
            let ee: Vec<u8> = vec![0xee; HANDLE_LEN];
            let ff: Vec<u8> = vec![0xff; HANDLE_LEN];
            let x88: Vec<u8> = vec![0x88; HANDLE_LEN];
            let x99: Vec<u8> = vec![0x99; HANDLE_LEN];
            for ct in cts.ciphertexts.iter() {
                match &ct.handle {
                    a if *a == aa => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "2" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    b if *b == bb => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "4" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    c if *c == cc => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "6" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    d if *d == dd => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "9" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    e if *e == ee => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "2" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    f if *f == ff => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "4" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    x if *x == x99 => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "9" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    x if *x == x88 => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "7" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    _ => panic!("unexpected handle 0x{:x}", ct.handle[0]),
                }
            }
        }
        Resp::Error(e) => panic!("error response: {}", e),
    }
}

#[tokio::test]
async fn schedule_diamond_reduction_dependence_pattern() {
    let test = get_test().await;
    test.keys.set_server_key_for_current_thread();
    let mut client = FhevmExecutorClient::connect(test.server_addr.clone())
        .await
        .unwrap();
    let mut builder = ProvenCompactCiphertextList::builder(&test.keys.compact_public_key);
    let list = builder
        .push(1_u16)
        .push(2_u16)
        .push(3_u16)
        .push(4_u16)
        .push(5_u16)
        .build_with_proof_packed(
            &test.keys.public_params,
            &[],
            tfhe::zk::ZkComputeLoad::Proof,
        )
        .unwrap();
    let expander = list.expand_without_verification().unwrap();
    let ct1 = SupportedFheCiphertexts::FheUint16(expander.get(0).unwrap().unwrap());
    let ct1 = test.compress(ct1);
    let ct2 = SupportedFheCiphertexts::FheUint16(expander.get(1).unwrap().unwrap());
    let ct2 = test.compress(ct2);
    let ct3 = SupportedFheCiphertexts::FheUint16(expander.get(2).unwrap().unwrap());
    let ct3 = test.compress(ct3);
    let ct4 = SupportedFheCiphertexts::FheUint16(expander.get(3).unwrap().unwrap());
    let ct4 = test.compress(ct4);
    let ct5 = SupportedFheCiphertexts::FheUint16(expander.get(4).unwrap().unwrap());
    let ct5 = test.compress(ct5);
    let handle1 = test.ciphertext_handle(&ct1, 3);
    let sync_input1 = SyncInput {
        input: Some(Input::Handle(handle1.clone())),
    };
    let handle2 = test.ciphertext_handle(&ct2, 3);
    let sync_input2 = SyncInput {
        input: Some(Input::Handle(handle2.clone())),
    };
    let handle3 = test.ciphertext_handle(&ct3, 3);
    let sync_input3 = SyncInput {
        input: Some(Input::Handle(handle3.clone())),
    };
    let handle4 = test.ciphertext_handle(&ct4, 3);
    let sync_input4 = SyncInput {
        input: Some(Input::Handle(handle4.clone())),
    };
    let handle5 = test.ciphertext_handle(&ct5, 3);
    let sync_input5 = SyncInput {
        input: Some(Input::Handle(handle5.clone())),
    };
    let sync_input_aa = SyncInput {
        input: Some(Input::Handle(vec![0xaa; HANDLE_LEN])),
    };
    let sync_input_bb = SyncInput {
        input: Some(Input::Handle(vec![0xbb; HANDLE_LEN])),
    };
    let sync_input_cc = SyncInput {
        input: Some(Input::Handle(vec![0xcc; HANDLE_LEN])),
    };
    let sync_input_dd = SyncInput {
        input: Some(Input::Handle(vec![0xdd; HANDLE_LEN])),
    };
    let sync_input_ee = SyncInput {
        input: Some(Input::Handle(vec![0xee; HANDLE_LEN])),
    };
    let sync_input_ff = SyncInput {
        input: Some(Input::Handle(vec![0xff; HANDLE_LEN])),
    };
    let sync_input_99 = SyncInput {
        input: Some(Input::Handle(vec![0x99; HANDLE_LEN])),
    };
    let computation1 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xaa; HANDLE_LEN]],
        inputs: vec![sync_input1.clone(), sync_input1],
    }; // Compute 1 + 1
    let computation2 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xbb; HANDLE_LEN]],
        inputs: vec![sync_input2, sync_input_aa.clone()],
    }; // 2 + 2
    let computation3 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xcc; HANDLE_LEN]],
        inputs: vec![sync_input3, sync_input_aa.clone()],
    }; // 2 + 3
    let computation4 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xdd; HANDLE_LEN]],
        inputs: vec![sync_input4, sync_input_aa.clone()],
    }; // 2 + 4
    let computation5 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xee; HANDLE_LEN]],
        inputs: vec![sync_input5, sync_input_aa.clone()],
    }; // 2 + 5
    let computation6 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0xff; HANDLE_LEN]],
        inputs: vec![sync_input_bb, sync_input_cc],
    }; // 4 + 5
    let computation7 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0x99; HANDLE_LEN]],
        inputs: vec![sync_input_dd, sync_input_ee],
    }; // 6 + 7
    let computation8 = SyncComputation {
        operation: FheOperation::FheAdd.into(),
        result_handles: vec![vec![0x88; HANDLE_LEN]],
        inputs: vec![sync_input_ff, sync_input_99],
    }; // 9 + 13
    let req = SyncComputeRequest {
        computations: vec![
            computation4,
            computation3,
            computation2,
            computation1,
            computation5,
            computation6,
            computation7,
            computation8,
        ],
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
            CompressedCiphertext {
                handle: handle3,
                serialization: ct3,
            },
            CompressedCiphertext {
                handle: handle4,
                serialization: ct4,
            },
            CompressedCiphertext {
                handle: handle5,
                serialization: ct5,
            },
        ],
    };
    let response = client.sync_compute(req).await.unwrap();
    let sync_compute_response = response.get_ref();
    let resp = sync_compute_response.resp.clone().unwrap();
    match resp {
        Resp::ResultCiphertexts(cts) => {
            assert!(
                cts.ciphertexts.len() == 8,
                "wrong number of output ciphertexts {} instead of {}",
                cts.ciphertexts.len(),
                8
            );
            let aa: Vec<u8> = vec![0xaa; HANDLE_LEN];
            let bb: Vec<u8> = vec![0xbb; HANDLE_LEN];
            let cc: Vec<u8> = vec![0xcc; HANDLE_LEN];
            let dd: Vec<u8> = vec![0xdd; HANDLE_LEN];
            let ee: Vec<u8> = vec![0xee; HANDLE_LEN];
            let ff: Vec<u8> = vec![0xff; HANDLE_LEN];
            let x88: Vec<u8> = vec![0x88; HANDLE_LEN];
            let x99: Vec<u8> = vec![0x99; HANDLE_LEN];
            for ct in cts.ciphertexts.iter() {
                match &ct.handle {
                    a if *a == aa => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "2" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    b if *b == bb => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "4" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    c if *c == cc => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "5" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    d if *d == dd => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "6" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    e if *e == ee => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "7" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    f if *f == ff => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "9" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    x if *x == x99 => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "13" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    x if *x == x88 => {
                        let ctd =
                            SupportedFheCiphertexts::decompress(3, &ct.serialization).unwrap();
                        match ctd
                            .decrypt(&test.as_ref().keys.client_key.clone().unwrap())
                            .as_str()
                        {
                            "22" => (),
                            s => panic!("unexpected result: {} for handle 0x{:x}", s, ct.handle[0]),
                        }
                    }
                    _ => panic!("unexpected handle 0x{:x}", ct.handle[0]),
                }
            }
        }
        Resp::Error(e) => panic!("error response: {}", e),
    }
}
