use std::{cell::Cell, collections::HashMap, error::Error, sync::Arc};

use common::FheOperation;
use executor::{
    fhevm_executor_server::{FhevmExecutor, FhevmExecutorServer},
    sync_compute_response::Resp,
    sync_input::Input,
    Ciphertext, ResultCiphertexts, SyncComputation, SyncComputeError, SyncComputeRequest,
    SyncComputeResponse, SyncInput,
};
use fhevm_engine_common::{
    keys::{FhevmKeys, SerializedFhevmKeys},
    tfhe_ops::{current_ciphertext_version, try_expand_ciphertext_list},
    types::{FhevmError, Handle, SupportedFheCiphertexts},
};
use sha3::{Digest, Keccak256};
use tfhe::set_server_key;
use tokio::task::spawn_blocking;
use tonic::{transport::Server, Code, Request, Response, Status};

pub mod common {
    tonic::include_proto!("fhevm.common");
}

pub mod executor {
    tonic::include_proto!("fhevm.executor");
}

pub fn start(args: &crate::cli::Args) -> Result<(), Box<dyn Error>> {
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        .max_blocking_threads(args.fhe_compute_threads)
        .enable_all()
        .build()?;

    let executor = FhevmExecutorService::new();
    let addr = args.server_addr.parse().expect("server address");

    runtime.block_on(async {
        Server::builder()
            .add_service(FhevmExecutorServer::new(executor))
            .serve(addr)
            .await?;
        Ok::<(), Box<dyn Error>>(())
    })?;
    Ok(())
}

struct InMemoryCiphertext {
    expanded: SupportedFheCiphertexts,
    compressed: Vec<u8>,
}

#[derive(Default)]
struct ComputationState {
    ciphertexts: HashMap<Handle, InMemoryCiphertext>,
}

fn error_response(error: SyncComputeError) -> SyncComputeResponse {
    SyncComputeResponse {
        resp: Some(Resp::Error(error.into())),
    }
}

fn success_response(cts: Vec<Ciphertext>) -> SyncComputeResponse {
    SyncComputeResponse {
        resp: Some(Resp::ResultCiphertexts(ResultCiphertexts {
            ciphertexts: cts,
        })),
    }
}

struct FhevmExecutorService {
    keys: Arc<FhevmKeys>,
}

#[tonic::async_trait]
impl FhevmExecutor for FhevmExecutorService {
    async fn sync_compute(
        &self,
        req: Request<SyncComputeRequest>,
    ) -> Result<Response<SyncComputeResponse>, Status> {
        let keys = self.keys.clone();
        let resp = spawn_blocking(move || {
            // Make sure we only clone the server key if needed.
            thread_local! {
                static SERVER_KEY_IS_SET: Cell<bool> = Cell::new(false);
            }
            if !SERVER_KEY_IS_SET.get() {
                set_server_key(keys.server_key.clone());
                SERVER_KEY_IS_SET.set(true);
            }

            // Exapnd inputs that are global to the whole request.
            let req = req.get_ref();
            let mut state = ComputationState::default();
            if Self::expand_inputs(&req.input_lists, &keys, &mut state).is_err() {
                return error_response(SyncComputeError::BadInputList);
            }

            // Execute all computations.
            let mut result_cts = Vec::new();
            for computation in &req.computations {
                let outcome = Self::process_computation(computation, &mut state);
                // Either all succeed or we return on the first failure.
                match outcome.resp.unwrap() {
                    Resp::Error(error) => {
                        return error_response(
                            SyncComputeError::try_from(error).expect("correct error value"),
                        );
                    }
                    Resp::ResultCiphertexts(cts) => result_cts.extend(cts.ciphertexts),
                }
            }
            success_response(result_cts)
        })
        .await;
        match resp {
            Ok(resp) => Ok(Response::new(resp)),
            Err(_) => Err(Status::new(
                Code::Unknown,
                "failed to execute computation via spawn_blocking",
            )),
        }
    }
}

impl FhevmExecutorService {
    fn new() -> Self {
        FhevmExecutorService {
            keys: Arc::new(SerializedFhevmKeys::load_from_disk().into()),
        }
    }

    fn process_computation(
        comp: &SyncComputation,
        state: &mut ComputationState,
    ) -> SyncComputeResponse {
        let op = FheOperation::try_from(comp.operation);
        match op {
            Ok(FheOperation::FheGetInputCiphertext) => Self::get_input_ciphertext(comp, &state),
            Ok(_) => error_response(SyncComputeError::UnsupportedOperation),
            _ => error_response(SyncComputeError::InvalidOperation),
        }
    }

    fn expand_inputs(
        lists: &Vec<Vec<u8>>,
        keys: &FhevmKeys,
        state: &mut ComputationState,
    ) -> Result<(), FhevmError> {
        for list in lists {
            let cts = try_expand_ciphertext_list(&list, &keys.server_key)?;
            let list_hash: Handle = Keccak256::digest(list).into();
            for (i, ct) in cts.iter().enumerate() {
                let mut handle = list_hash;
                handle[29] = i as u8;
                handle[30] = ct.type_num() as u8;
                handle[31] = current_ciphertext_version() as u8;
                state.ciphertexts.insert(
                    handle,
                    InMemoryCiphertext {
                        expanded: ct.clone(),
                        compressed: ct.clone().compress(),
                    },
                );
            }
        }
        Ok(())
    }

    fn get_input_ciphertext(
        comp: &SyncComputation,
        state: &ComputationState,
    ) -> SyncComputeResponse {
        match (comp.inputs.first(), comp.inputs.len()) {
            (
                Some(SyncInput {
                    input: Some(Input::InputHandle(handle)),
                }),
                1,
            ) => {
                if let Ok(handle) = (handle as &[u8]).try_into() as Result<Handle, _> {
                    if let Some(in_mem_ciphertext) = state.ciphertexts.get(&handle) {
                        success_response(vec![Ciphertext {
                            handle: handle.to_vec(),
                            ciphertext: in_mem_ciphertext.compressed.clone(),
                        }])
                    } else {
                        error_response(SyncComputeError::UnknownHandle)
                    }
                } else {
                    error_response(SyncComputeError::BadInputs)
                }
            }
            _ => error_response(SyncComputeError::BadInputs),
        }
    }
}
