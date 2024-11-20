use anyhow::Result;
use executor::{
    fhevm_executor_server::{FhevmExecutor, FhevmExecutorServer},
    sync_compute_response::Resp,
    ResultCiphertexts, SyncComputeResponse, SyncInput,
};
pub use executor::{
    sync_input::Input, CompressedCiphertext, SyncComputation, SyncComputeError, SyncComputeRequest,
};
use fhevm_engine_common::{
    common::FheOperation,
    keys::{FhevmKeys, SerializedFhevmKeys},
    tfhe_ops::{current_ciphertext_version, perform_fhe_operation, try_expand_ciphertext_list},
    types::{get_ct_type, FhevmError, Handle, SupportedFheCiphertexts, HANDLE_LEN, SCALAR_LEN},
};
use sha3::{Digest, Keccak256};
use std::{borrow::Borrow, cell::Cell, collections::HashMap};
use tfhe::{set_server_key, zk::CompactPkePublicParams};
use tokio::task::spawn_blocking;
use tonic::{transport::Server, Code, Request, Response, Status};

use scheduler::dfg::{scheduler::Scheduler, types::DFGTaskInput, DFGraph};

pub use fhevm_engine_common::common;
pub mod executor {
    tonic::include_proto!("fhevm.executor");
}

thread_local! {
    pub static SERVER_KEY: Cell<Option<tfhe::ServerKey>> = const {Cell::new(None)};
    pub static LOCAL_RAYON_THREADS: Cell<usize> = const {Cell::new(8)};
}

pub fn start(args: &crate::cli::Args) -> Result<()> {
    let keys: FhevmKeys = SerializedFhevmKeys::load_from_disk(&args.fhe_keys_directory).into();
    SERVER_KEY.set(Some(keys.server_key.clone()));
    LOCAL_RAYON_THREADS.set(args.policy_fhe_compute_threads);
    let executor = FhevmExecutorService::new(keys.clone());
    let runtime = tokio::runtime::Builder::new_multi_thread()
        .worker_threads(args.tokio_threads)
        .max_blocking_threads(args.fhe_compute_threads)
        .on_thread_start(move || {
            set_server_key(keys.server_key.clone());
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
    keys: FhevmKeys,
}

#[tonic::async_trait]
impl FhevmExecutor for FhevmExecutorService {
    async fn sync_compute(
        &self,
        req: Request<SyncComputeRequest>,
    ) -> Result<Response<SyncComputeResponse>, Status> {
        let public_params = self.keys.public_params.clone();
        let resp = spawn_blocking(move || {
            let req = req.get_ref();
            let mut state = ComputationState::default();

            // Exapnd compact ciphertext lists for the whole request.
            if Self::expand_compact_lists(&req.compact_ciphertext_lists, &mut state, &public_params)
                .is_err()
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
                if let Err(e) = build_taskgraph_from_request(&mut graph, req, &state) {
                    return Some(Resp::Error((e as SyncComputeError).into()));
                }
                // Schedule computations in parallel as dependences allow
                let mut sched = Scheduler::new(&mut graph.graph, LOCAL_RAYON_THREADS.get());

                let now = std::time::SystemTime::now();
                let sks: tfhe::ServerKey = SERVER_KEY.borrow().take().expect("Server key missing");
                SERVER_KEY.set(Some(sks.clone()));
                if sched.schedule(sks).await.is_err() {
                    return Some(Resp::Error(SyncComputeError::ComputationFailed.into()));
                }
                println!(
                    "Execution time (sched): {}",
                    now.elapsed().unwrap().as_millis()
                );
                // Extract the results from the graph
                match graph.get_results() {
                    Ok(mut result_cts) => Some(Resp::ResultCiphertexts(ResultCiphertexts {
                        ciphertexts: result_cts
                            .iter_mut()
                            .map(|(h, ct)| CompressedCiphertext {
                                handle: h.clone(),
                                serialization: std::mem::take(&mut ct.2),
                            })
                            .collect(),
                    })),
                    Err(_) => Some(Resp::Error(SyncComputeError::ComputationFailed.into())),
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
    fn new(keys: FhevmKeys) -> Self {
        FhevmExecutorService {
            keys,
        }
    }

    #[allow(dead_code)]
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
        state: &mut ComputationState,
        public_params: &CompactPkePublicParams,
    ) -> Result<(), FhevmError> {
        for list in lists {
            let cts = try_expand_ciphertext_list(&list, &public_params)?;
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
                        compressed: ct.clone().compress().1,
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

    #[allow(dead_code)]
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

    #[allow(dead_code)]
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
                        Ok(SupportedFheCiphertexts::Scalar(s.clone()))
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
                    let (_, compressed) = result.clone().compress();
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

pub fn build_taskgraph_from_request(
    dfg: &mut DFGraph,
    req: &SyncComputeRequest,
    state: &ComputationState,
) -> Result<(), SyncComputeError> {
    let mut produced_handles: HashMap<&Handle, usize> = HashMap::new();
    // Add all computations as nodes in the graph.
    for computation in &req.computations {
        let inputs: Result<Vec<DFGTaskInput>> = computation
            .inputs
            .iter()
            .map(|input| match &input.input {
                Some(input) => match input {
                    Input::Handle(h) => {
                        if let Some(ct) = state.ciphertexts.get(h) {
                            Ok(DFGTaskInput::Value(ct.expanded.clone()))
                        } else {
                            Ok(DFGTaskInput::Dependence(None))
                        }
                    }
                    Input::Scalar(s) if s.len() == SCALAR_LEN => Ok(DFGTaskInput::Value(
                        SupportedFheCiphertexts::Scalar(s.clone()),
                    )),
                    _ => Err(FhevmError::BadInputs.into()),
                },
                None => Err(FhevmError::BadInputs.into()),
            })
            .collect();
        if let Ok(mut inputs) = inputs {
            let res_handle = computation
                .result_handles
                .first()
                .filter(|h| h.len() == HANDLE_LEN)
                .ok_or(SyncComputeError::BadResultHandles)?;
            let n = dfg
                .add_node(
                    res_handle.clone(),
                    computation.operation,
                    std::mem::take(&mut inputs),
                )
                .or_else(|_| Err(SyncComputeError::ComputationFailed))?;
            produced_handles.insert(res_handle, n.index());
        }
    }
    // Traverse computations and add dependences/edges as required
    for (index, computation) in req.computations.iter().enumerate() {
        for (input_idx, input) in computation.inputs.iter().enumerate() {
            if let Some(Input::Handle(input)) = &input.input {
                if !state.ciphertexts.contains_key(input) {
                    if let Some(producer_index) = produced_handles.get(input) {
                        dfg.add_dependence(*producer_index, index, input_idx)
                            .or_else(|_| Err(SyncComputeError::ComputationFailed))?;
                    } else {
                        return Err(SyncComputeError::ComputationFailed);
                    }
                }
            }
        }
    }
    Ok(())
}
