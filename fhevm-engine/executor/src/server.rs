use std::{cell::Cell, collections::HashMap, sync::Arc};

use anyhow::Result;
pub use common::FheOperation;
use executor::{
    fhevm_executor_server::{FhevmExecutor, FhevmExecutorServer},
    sync_compute_response::Resp,
    ResultCiphertexts, SyncComputeResponse, SyncInput,
};
pub use executor::{
    sync_input::Input, CompressedCiphertext, SyncComputation, SyncComputeError, SyncComputeRequest,
};
use fhevm_engine_common::{
    keys::{FhevmKeys, SerializedFhevmKeys},
    tfhe_ops::{current_ciphertext_version, perform_fhe_operation, try_expand_ciphertext_list},
    types::{get_ct_type, FhevmError, Handle, SupportedFheCiphertexts, HANDLE_LEN, SCALAR_LEN},
};
use sha3::{Digest, Keccak256};
use tfhe::{integer::U256, set_server_key};
use tokio::task::spawn_blocking;
use tonic::{transport::Server, Code, Request, Response, Status};

use crate::dfg::{scheduler::Scheduler, DFGraph};

pub mod common {
    tonic::include_proto!("fhevm.common");
}

pub mod executor {
    tonic::include_proto!("fhevm.executor");
}

pub fn start(args: &crate::cli::Args) -> Result<()> {
    let keys: Arc<FhevmKeys> = Arc::new(SerializedFhevmKeys::load_from_disk().into());
    let executor = FhevmExecutorService::new(keys.clone());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        .max_blocking_threads(args.fhe_compute_threads)
        .on_thread_start(move || {
            thread_local! {
                static SERVER_KEY_IS_SET: Cell<bool> = const {Cell::new(false)};
            }
            if !SERVER_KEY_IS_SET.get() {
                set_server_key(keys.server_key.clone());
                SERVER_KEY_IS_SET.set(true);
            }
        })
        .enable_all()
        .build()?;

    let addr = args.server_addr.parse().expect("server address");

    runtime.block_on(async {
        Server::builder()
            .add_service(FhevmExecutorServer::new(executor))
            .serve(addr)
            .await?;
        Ok::<(), anyhow::Error>(())
    })?;
    Ok(())
}

pub struct InMemoryCiphertext {
    pub expanded: SupportedFheCiphertexts,
    pub compressed: Vec<u8>,
}

#[derive(Default)]
pub struct ComputationState {
    pub ciphertexts: HashMap<Handle, InMemoryCiphertext>,
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
            let req = req.get_ref();
            let mut state = ComputationState::default();

            // Exapnd compact ciphertext lists for the whole request.
            if Self::expand_compact_lists(&req.compact_ciphertext_lists, &keys, &mut state).is_err()
            {
                return SyncComputeResponse {
                    resp: Some(Resp::Error(SyncComputeError::BadInputList.into())),
                };
            }

            // Decompress compressed ciphertexts for the whole request.
            if Self::decompress_compressed_ciphertexts(&req.compressed_ciphertexts, &mut state)
                .is_err()
            {
                return SyncComputeResponse {
                    resp: Some(Resp::Error(SyncComputeError::BadInputCiphertext.into())),
                };
            }

            // Run the request's computations in an async block
            let handle = tokio::runtime::Handle::current();
            let _ = handle.enter();
            let resp = handle.block_on(async {
                // Build the dataflow graph for this request
                let mut graph = DFGraph::default();
                if let Err(e) = graph.build_from_request(req, &state) {
                    return Some(Resp::Error((e as SyncComputeError).into()));
                }
                // Schedule computations in parallel as dependences allow
                let mut sched = Scheduler::new(&mut graph.graph);
                if sched.schedule().await.is_err() {
                    return Some(Resp::Error(SyncComputeError::ComputationFailed.into()));
                }
                // Extract the results from the graph
                match graph.get_results() {
                    Ok(result_cts) => Some(Resp::ResultCiphertexts(ResultCiphertexts {
                        ciphertexts: result_cts,
                    })),
                    Err(e) => Some(Resp::Error(e.into())),
                }
            });
            SyncComputeResponse { resp }
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
    fn new(keys: Arc<FhevmKeys>) -> Self {
        FhevmExecutorService { keys }
    }

    fn process_computation(
        comp: &SyncComputation,
        state: &mut ComputationState,
    ) -> Result<Vec<CompressedCiphertext>, SyncComputeError> {
        // For now, assume only one result handle.
        let result_handle = comp
            .result_handles
            .first()
            .filter(|h| h.len() == HANDLE_LEN)
            .ok_or_else(|| SyncComputeError::BadResultHandles)?
            .clone();
        let op = FheOperation::try_from(comp.operation);
        match op {
            Ok(FheOperation::FheGetCiphertext) => {
                Self::get_ciphertext(comp, &result_handle, &state)
            }
            Ok(_) => Self::compute(comp, result_handle, state),
            _ => Err(SyncComputeError::InvalidOperation),
        }
    }

    fn expand_compact_lists(
        lists: &Vec<Vec<u8>>,
        keys: &FhevmKeys,
        state: &mut ComputationState,
    ) -> Result<(), FhevmError> {
        for list in lists {
            let cts = try_expand_ciphertext_list(&list, &keys.server_key)?;
            let list_hash: Handle = Keccak256::digest(list).to_vec();
            for (i, ct) in cts.iter().enumerate() {
                let mut handle = list_hash.clone();
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

    fn decompress_compressed_ciphertexts(
        cts: &Vec<CompressedCiphertext>,
        state: &mut ComputationState,
    ) -> Result<()> {
        for ct in cts.iter() {
            let ct_type = get_ct_type(&ct.handle)?;
            let supported_ct = SupportedFheCiphertexts::decompress(ct_type, &ct.serialization)?;
            state.ciphertexts.insert(
                ct.handle.clone(),
                InMemoryCiphertext {
                    expanded: supported_ct,
                    compressed: ct.serialization.clone(),
                },
            );
        }
        Ok(())
    }

    fn get_ciphertext(
        comp: &SyncComputation,
        result_handle: &Handle,
        state: &ComputationState,
    ) -> Result<Vec<CompressedCiphertext>, SyncComputeError> {
        match (comp.inputs.first(), comp.inputs.len()) {
            (
                Some(SyncInput {
                    input: Some(Input::Handle(handle)),
                }),
                1,
            ) => {
                if let Some(in_mem_ciphertext) = state.ciphertexts.get(handle) {
                    if *handle != *result_handle {
                        Err(SyncComputeError::BadInputs)
                    } else {
                        Ok(vec![CompressedCiphertext {
                            handle: result_handle.to_vec(),
                            serialization: in_mem_ciphertext.compressed.clone(),
                        }])
                    }
                } else {
                    Err(SyncComputeError::UnknownHandle)
                }
            }
            _ => Err(SyncComputeError::BadInputs),
        }
    }

    fn compute(
        comp: &SyncComputation,
        result_handle: Handle,
        state: &mut ComputationState,
    ) -> Result<Vec<CompressedCiphertext>, SyncComputeError> {
        // Collect computation inputs.
        let inputs: Result<Vec<SupportedFheCiphertexts>> = comp
            .inputs
            .iter()
            .map(|sync_input| match &sync_input.input {
                Some(input) => match input {
                    Input::Handle(h) => {
                        let ct = state.ciphertexts.get(h).ok_or(FhevmError::BadInputs)?;
                        Ok(ct.expanded.clone())
                    }
                    Input::Scalar(s) if s.len() == SCALAR_LEN => {
                        let mut scalar = U256::default();
                        scalar.copy_from_be_byte_slice(&s);
                        Ok(SupportedFheCiphertexts::Scalar(scalar))
                    }
                    _ => Err(FhevmError::BadInputs.into()),
                },
                None => Err(FhevmError::BadInputs.into()),
            })
            .collect();

        // Do the computation on the inputs.
        match inputs {
            Ok(inputs) => match perform_fhe_operation(comp.operation as i16, &inputs) {
                Ok(result) => {
                    let compressed = result.clone().compress();
                    state.ciphertexts.insert(
                        result_handle.clone(),
                        InMemoryCiphertext {
                            expanded: result,
                            compressed: compressed.clone(),
                        },
                    );
                    Ok(vec![CompressedCiphertext {
                        handle: result_handle,
                        serialization: compressed,
                    }])
                }
                Err(_) => Err(SyncComputeError::ComputationFailed),
            },
            Err(_) => Err(SyncComputeError::BadInputs),
        }
    }
}

pub fn run_computation(
    operation: i32,
    inputs: Result<Vec<SupportedFheCiphertexts>, SyncComputeError>,
    graph_node_index: usize,
) -> Result<(usize, InMemoryCiphertext), SyncComputeError> {
    let op = FheOperation::try_from(operation);
    match inputs {
        Ok(inputs) => match op {
            Ok(FheOperation::FheGetCiphertext) => {
                let res = InMemoryCiphertext {
                    expanded: inputs[0].clone(),
                    compressed: inputs[0].clone().compress(),
                };
                Ok((graph_node_index, res))
            }
            Ok(_) => match perform_fhe_operation(operation as i16, &inputs) {
                Ok(result) => {
                    let res = InMemoryCiphertext {
                        expanded: result.clone(),
                        compressed: result.compress(),
                    };
                    Ok((graph_node_index, res))
                }
                Err(_) => Err::<(usize, InMemoryCiphertext), SyncComputeError>(
                    SyncComputeError::ComputationFailed,
                ),
            },
            _ => Err::<(usize, InMemoryCiphertext), SyncComputeError>(
                SyncComputeError::InvalidOperation,
            ),
        },
        Err(_) => Err(SyncComputeError::ComputationFailed),
    }
}
