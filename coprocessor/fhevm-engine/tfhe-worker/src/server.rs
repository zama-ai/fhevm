use std::collections::{BTreeMap, BTreeSet, HashMap};
use std::error::Error;
use std::num::NonZeroUsize;
use std::str::FromStr;

use crate::db_queries::{check_if_api_key_is_valid, fetch_tenant_server_key};
use crate::server::tfhe_worker::GenericResponse;
use crate::types::{CoprocessorError, TfheTenantKeys};
use crate::utils::sort_computations_by_dependencies;
use alloy::signers::local::PrivateKeySigner;
use alloy::signers::SignerSync;
use alloy::sol_types::{Eip712Domain, SolStruct};
pub use fhevm_engine_common::common;
use fhevm_engine_common::tfhe_ops::{
    check_fhe_operand_types, current_ciphertext_version, trivial_encrypt_be_bytes,
    try_expand_ciphertext_list, validate_fhe_type,
};
use fhevm_engine_common::types::{FhevmError, SupportedFheCiphertexts, SupportedFheOperations};
use lazy_static::lazy_static;
use opentelemetry::global::{BoxedSpan, BoxedTracer};
use opentelemetry::trace::{Span, TraceContextExt, Tracer};
use opentelemetry::KeyValue;
use prometheus::{register_int_counter, IntCounter};
use sha3::{Digest, Keccak256};
use sqlx::{query, Acquire};
use tfhe_worker::async_computation_input::Input;
use tfhe_worker::{
    FetchedCiphertext, GetCiphertextSingleResponse, InputCiphertextResponse,
    InputCiphertextResponseHandle, InputUploadBatch, InputUploadResponse,
};
use tokio::task::spawn_blocking;
use tonic::transport::Server;
use tracing::{error, info};
pub mod tfhe_worker {
    tonic::include_proto!("fhevm.tfhe_worker");
}

lazy_static! {
    static ref UPLOAD_INPUTS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_upload_inputs_count",
        "grpc calls for inputs upload endpoint"
    )
    .unwrap();
    static ref UPLOAD_INPUTS_ERRORS: IntCounter = register_int_counter!(
        "coprocessor_upload_inputs_errors",
        "grpc errors while calling upload inputs"
    )
    .unwrap();
    static ref ASYNC_COMPUTE_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_async_compute_count",
        "grpc calls for async compute endpoint"
    )
    .unwrap();
    static ref ASYNC_COMPUTE_ERRORS: IntCounter = register_int_counter!(
        "coprocessor_async_compute_errors",
        "grpc errors while calling async compute"
    )
    .unwrap();
    static ref TRIVIAL_ENCRYPT_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_trivial_encrypt_count",
        "grpc calls for trivial encrypt endpoint"
    )
    .unwrap();
    static ref TRIVIAL_ENCRYPT_ERRORS: IntCounter = register_int_counter!(
        "coprocessor_trivial_encrypt_errors",
        "grpc errors while calling trivial encrypt"
    )
    .unwrap();
    static ref GET_CIPHERTEXTS_COUNTER: IntCounter = register_int_counter!(
        "coprocessor_get_ciphertexts_count",
        "grpc calls for get ciphertexts endpoint"
    )
    .unwrap();
    static ref GET_CIPHERTEXTS_ERRORS: IntCounter = register_int_counter!(
        "coprocessor_get_ciphertexts_errors",
        "grpc errors while calling get ciphertexts"
    )
    .unwrap();
}

struct CoprocessorService {
    pool: sqlx::Pool<sqlx::Postgres>,
    args: crate::daemon_cli::Args,
    tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
    signer: PrivateKeySigner,
    get_ciphertext_eip712_domain: Eip712Domain,
}

pub async fn run_server(
    args: crate::daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    loop {
        if let Err(e) = run_server_iteration(args.clone()).await {
            error!(target: "grpc_server", { error = e }, "Error running server, retrying shortly");
        }

        tokio::time::sleep(tokio::time::Duration::from_millis(5000)).await;
    }
}

pub async fn run_server_iteration(
    args: crate::daemon_cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = args
        .server_addr
        .parse()
        .expect("Can't parse server address");
    let db_url = crate::utils::db_url(&args);

    let coprocessor_key_file = tokio::fs::read_to_string(&args.coprocessor_private_key).await?;

    let signer = PrivateKeySigner::from_str(coprocessor_key_file.trim())?;
    info!(target: "grpc_server", { address = signer.address().to_string() }, "Coprocessor signer initiated");

    info!("Coprocessor listening on {}", addr);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.pg_pool_max_connections)
        .connect(&db_url)
        .await?;

    let tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>> =
        std::sync::Arc::new(tokio::sync::RwLock::new(lru::LruCache::new(
            NonZeroUsize::new(args.tenant_key_cache_size as usize).unwrap(),
        )));

    let service = CoprocessorService::new(pool, args, tenant_key_cache, signer);

    Server::builder()
        .add_service(
            crate::server::tfhe_worker::fhevm_coprocessor_server::FhevmCoprocessorServer::new(
                service,
            ),
        )
        .serve(addr)
        .await?;

    Ok(())
}

// for EIP712 input signature
alloy::sol! {
    struct CiphertextVerificationForCopro {
        address aclAddress;
        bytes32 hashOfCiphertext;
        uint256[] handlesList;
        address userAddress;
        address contractAddress;
    }
}

// copied from fhevm-go coprocessor
//   const types = {
//     CiphertextVerificationForCopro: [
//       {
//         name: 'aclAddress',
//         type: 'address',
//       },
//       {
//         name: 'hashOfCiphertext',
//         type: 'bytes32',
//       },
//       {
//         name: 'handlesList',
//         type: 'uint256[]',
//       },
//       {
//         name: 'userAddress',
//         type: 'address',
//       },
//       {
//         name: 'contractAddress',
//         type: 'address',
//       },
//     ],
//   };

alloy::sol! {
    struct GetCiphertextResponseSignatureData {
        uint256 handle;
        bytes ciphertext_digest;
    }
}

pub struct GrpcTracer {
    ctx: opentelemetry::Context,
    name: &'static str,
    tracer: BoxedTracer,
}

impl GrpcTracer {
    pub fn child_span(&self, name: &'static str) -> BoxedSpan {
        self.tracer.start_with_context(name, &self.ctx)
    }

    pub fn set_error(&mut self, e: impl Error) {
        self.ctx
            .span()
            .set_status(opentelemetry::trace::Status::Error {
                description: e.to_string().into(),
            });
    }
}

impl Clone for GrpcTracer {
    fn clone(&self) -> Self {
        GrpcTracer {
            ctx: self.ctx.clone(),
            name: self.name,
            tracer: opentelemetry::global::tracer(self.name),
        }
    }
}

fn grpc_tracer(function_name: &'static str) -> GrpcTracer {
    let name = "grpc_service";
    let tracer = opentelemetry::global::tracer(name);
    let span = tracer.start(function_name);
    let ctx = opentelemetry::Context::current_with_span(span);
    GrpcTracer { ctx, tracer, name }
}

#[tonic::async_trait]
impl tfhe_worker::fhevm_coprocessor_server::FhevmCoprocessor for CoprocessorService {
    async fn upload_inputs(
        &self,
        request: tonic::Request<InputUploadBatch>,
    ) -> std::result::Result<tonic::Response<InputUploadResponse>, tonic::Status> {
        UPLOAD_INPUTS_COUNTER.inc();
        let mut tracer = grpc_tracer("upload_inputs");
        self.upload_inputs_impl(request, &tracer)
            .await
            .inspect_err(|e| {
                tracer.set_error(e);
                UPLOAD_INPUTS_ERRORS.inc();
            })
    }

    async fn async_compute(
        &self,
        request: tonic::Request<tfhe_worker::AsyncComputeRequest>,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GenericResponse>, tonic::Status> {
        ASYNC_COMPUTE_COUNTER.inc();
        let mut tracer = grpc_tracer("async_compute");
        self.async_compute_impl(request, &tracer)
            .await
            .inspect_err(|e| {
                tracer.set_error(e);
                ASYNC_COMPUTE_ERRORS.inc();
            })
    }

    async fn trivial_encrypt_ciphertexts(
        &self,
        request: tonic::Request<tfhe_worker::TrivialEncryptBatch>,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GenericResponse>, tonic::Status> {
        TRIVIAL_ENCRYPT_COUNTER.inc();
        let mut tracer = grpc_tracer("trivial_encrypt_ciphertexts");
        self.trivial_encrypt_ciphertexts_impl(request, &tracer)
            .await
            .inspect_err(|e| {
                tracer.set_error(e);
                TRIVIAL_ENCRYPT_ERRORS.inc();
            })
    }

    async fn get_ciphertexts(
        &self,
        request: tonic::Request<tfhe_worker::GetCiphertextBatch>,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GetCiphertextResponse>, tonic::Status>
    {
        GET_CIPHERTEXTS_COUNTER.inc();
        let mut tracer = grpc_tracer("get_ciphertexts");
        self.get_ciphertexts_impl(request, &tracer)
            .await
            .inspect_err(|e| {
                tracer.set_error(e);
                GET_CIPHERTEXTS_ERRORS.inc();
            })
    }
}

impl CoprocessorService {
    fn new(
        pool: sqlx::Pool<sqlx::Postgres>,
        args: crate::daemon_cli::Args,
        tenant_key_cache: std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
        signer: PrivateKeySigner,
    ) -> Self {
        let get_ciphertext_eip712_domain = alloy::sol_types::eip712_domain! {
            name: "GetCiphertextResponse",
            version: "1",
        };
        CoprocessorService {
            pool,
            args,
            tenant_key_cache,
            signer,
            get_ciphertext_eip712_domain,
        }
    }

    async fn upload_inputs_impl(
        &self,
        request: tonic::Request<InputUploadBatch>,
        tracer: &GrpcTracer,
    ) -> std::result::Result<tonic::Response<InputUploadResponse>, tonic::Status> {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool, tracer).await?;

        let req = request.get_ref();
        if req.input_ciphertexts.len() > self.args.maximimum_compact_inputs_upload {
            return Err(tonic::Status::from_error(Box::new(
                CoprocessorError::MoreThanMaximumCompactInputCiphertextsUploaded {
                    input_count: req.input_ciphertexts.len(),
                    maximum_allowed: self.args.maximimum_compact_inputs_upload,
                },
            )));
        }

        let mut response = InputUploadResponse {
            upload_responses: Vec::with_capacity(req.input_ciphertexts.len()),
        };
        if req.input_ciphertexts.is_empty() {
            return Ok(tonic::Response::new(response));
        }

        let fetch_key_response = {
            fetch_tenant_server_key(tenant_id, &self.pool, &self.tenant_key_cache)
                .await
                .map_err(tonic::Status::from_error)?
        };
        let chain_id = fetch_key_response.chain_id;
        let chain_id_be = (chain_id as u64).to_be_bytes();
        let server_key = fetch_key_response.server_key;
        let verifying_contract_address = fetch_key_response.verifying_contract_address;
        let verifying_contract_address =
            alloy::primitives::Address::from_str(&verifying_contract_address).map_err(|e| {
                tonic::Status::from_error(Box::new(
                    CoprocessorError::CannotParseTenantEthereumAddress {
                        bad_address: verifying_contract_address.clone(),
                        parsing_error: e.to_string(),
                    },
                ))
            })?;
        let acl_contract_address =
            alloy::primitives::Address::from_str(&fetch_key_response.acl_contract_address)
                .map_err(|e| {
                    tonic::Status::from_error(Box::new(
                        CoprocessorError::CannotParseTenantEthereumAddress {
                            bad_address: fetch_key_response.acl_contract_address.clone(),
                            parsing_error: e.to_string(),
                        },
                    ))
                })?;

        let eip_712_domain = alloy::sol_types::eip712_domain! {
            name: "InputVerifier",
            version: "1",
            chain_id: chain_id as u64,
            verifying_contract: verifying_contract_address,
        };

        let mut tfhe_work_set = tokio::task::JoinSet::new();

        let mut contract_addresses = Vec::with_capacity(req.input_ciphertexts.len());
        let mut user_addresses = Vec::with_capacity(req.input_ciphertexts.len());
        for ci in &req.input_ciphertexts {
            // parse addresses
            contract_addresses.push(
                alloy::primitives::Address::from_str(&ci.contract_address).map_err(|e| {
                    CoprocessorError::CannotParseEthereumAddress {
                        bad_address: ci.contract_address.clone(),
                        parsing_error: e.to_string(),
                    }
                })?,
            );
            user_addresses.push(
                alloy::primitives::Address::from_str(&ci.user_address).map_err(|e| {
                    CoprocessorError::CannotParseEthereumAddress {
                        bad_address: ci.contract_address.clone(),
                        parsing_error: e.to_string(),
                    }
                })?,
            );
        }

        for (idx, ci) in req.input_ciphertexts.iter().enumerate() {
            let cloned_input = ci.clone();
            let server_key = server_key.clone();
            let tracer = tracer.clone();
            let public_params = fetch_key_response.public_params.clone();

            let mut blocking_span = tracer.child_span("blocking_ciphertext_list_expand");
            blocking_span.set_attributes(vec![KeyValue::new("idx", idx as i64)]);
            tfhe_work_set.spawn_blocking(
                move || -> Result<_, (Box<dyn std::error::Error + Send + Sync>, usize)> {
                    let mut span = tracer.child_span("set_server_key");
                    tfhe::set_server_key(server_key.clone());
                    span.end();

                    let mut span = tracer.child_span("keccak_256_hash");
                    let mut state = Keccak256::new();
                    state.update(&cloned_input.input_payload);
                    let blob_hash = state.finalize().to_vec();
                    assert_eq!(blob_hash.len(), 32, "should be 32 bytes");
                    span.end();

                    let mut span = tracer.child_span("expand_ciphertext_list");
                    let expanded =
                        try_expand_ciphertext_list(&cloned_input.input_payload, &public_params)
                            .map_err(|e| {
                                let err: Box<dyn std::error::Error + Send + Sync> = Box::new(e);
                                (err, idx)
                            })?;

                    span.set_attributes(vec![
                        KeyValue::new("idx", idx as i64),
                        KeyValue::new("count", expanded.len() as i64),
                        KeyValue::new("input_hash", format!("0x{}", hex::encode(&blob_hash))),
                    ]);
                    span.end();

                    blocking_span.end();

                    Ok((expanded, idx, blob_hash))
                },
            );
        }

        let mut span = tracer.child_span("ciphertext_list_expand_wait");
        let mut results: BTreeMap<usize, (Vec<SupportedFheCiphertexts>, Vec<u8>)> = BTreeMap::new();
        while let Some(output) = tfhe_work_set.join_next().await {
            let (cts, idx, hash) = output
                .map_err(|e| {
                    let err: Box<dyn std::error::Error + Sync + Send> = Box::new(e);
                    tonic::Status::from_error(err)
                })?
                .map_err(|e| tonic::Status::from_error(e.0))?;

            if cts.len() > self.args.maximum_handles_per_input as usize {
                return Err(tonic::Status::from_error(Box::new(
                    CoprocessorError::CompactInputCiphertextHasMoreCiphertextThanLimitAllows {
                        input_blob_index: idx,
                        input_ciphertexts_in_blob: cts.len(),
                        input_maximum_ciphertexts_allowed: self.args.maximum_handles_per_input
                            as usize,
                    },
                )));
            }

            assert!(
                results.insert(idx, (cts, hash)).is_none(),
                "fresh map, we passed vector ordered by indexes before"
            );
        }
        span.end();

        assert_eq!(
            results.len(),
            req.input_ciphertexts.len(),
            "We should have all the ciphertexts now"
        );

        let mut span = tracer.child_span("db_input_ciphertexts_insert");
        let mut trx = self
            .pool
            .begin()
            .await
            .map_err(Into::<CoprocessorError>::into)?;
        for (idx, input_blob) in req.input_ciphertexts.iter().enumerate() {
            let (corresponding_unpacked, blob_hash) = results
                .remove(&idx)
                .expect("we should have all results computed now");

            let mut span = tracer.child_span("db_insert_input_blob");
            span.set_attributes(vec![KeyValue::new("idx", idx as i64)]);
            // save blob for audits and historical reference
            let _ = sqlx::query!(
                "
              INSERT INTO input_blobs(tenant_id, blob_hash, blob_data, blob_ciphertext_count)
              VALUES($1, $2, $3, $4)
              ON CONFLICT (tenant_id, blob_hash) DO NOTHING
            ",
                tenant_id,
                &blob_hash,
                &input_blob.input_payload,
                corresponding_unpacked.len() as i32
            )
            .execute(trx.as_mut())
            .await
            .map_err(Into::<CoprocessorError>::into)?;
            span.end();

            let mut hash_of_ciphertext: [u8; 32] = [0; 32];
            hash_of_ciphertext.copy_from_slice(&blob_hash);

            let mut ct_verification = CiphertextVerificationForCopro {
                hashOfCiphertext: alloy::primitives::FixedBytes(hash_of_ciphertext),
                aclAddress: acl_contract_address,
                contractAddress: contract_addresses[idx],
                userAddress: user_addresses[idx],
                handlesList: Vec::with_capacity(corresponding_unpacked.len()),
            };

            let mut ct_resp = InputCiphertextResponse {
                acl_address: fetch_key_response.acl_contract_address.clone(),
                hash_of_ciphertext: hash_of_ciphertext.to_vec(),
                input_handles: Vec::with_capacity(corresponding_unpacked.len()),
                eip712_signature: Vec::new(),
                contract_address: contract_addresses[idx].to_string(),
                user_address: user_addresses[idx].to_string(),
                signer_address: self.signer.address().to_string(),
            };

            let ciphertext_version = current_ciphertext_version();
            for (ct_idx, the_ct) in corresponding_unpacked.into_iter().enumerate() {
                // TODO: simplify compress and hash computation async handling
                let blob_hash_clone = blob_hash.clone();
                let server_key_clone = server_key.clone();
                let (handle, serialized_ct, serialized_type) = spawn_blocking(move || {
                    tfhe::set_server_key(server_key_clone);
                    let (serialized_type, serialized_ct) = the_ct.compress();
                    let mut handle_hash = Keccak256::new();
                    handle_hash.update(&blob_hash_clone);
                    handle_hash.update([ct_idx as u8]);
                    handle_hash.update(acl_contract_address.as_slice());
                    handle_hash.update(chain_id_be);
                    let mut handle = handle_hash.finalize().to_vec();
                    assert_eq!(handle.len(), 32);
                    // idx cast to u8 must succeed because we don't allow
                    // more handles than u8 size
                    handle[29] = ct_idx as u8;
                    handle[30] = serialized_type as u8;
                    handle[31] = ciphertext_version as u8;

                    (handle, serialized_ct, serialized_type)
                })
                .await
                .map_err(|e| tonic::Status::from_error(Box::new(e)))?;

                let mut span = tracer.child_span("db_insert_ciphertext");
                span.set_attributes(vec![
                    KeyValue::new("blob_idx", idx as i64),
                    KeyValue::new("ct_idx", ct_idx as i64),
                    KeyValue::new("handle", format!("0x{}", hex::encode(&handle))),
                ]);
                let _ = sqlx::query!(
                    "
                    INSERT INTO ciphertexts(
                        tenant_id,
                        handle,
                        ciphertext,
                        ciphertext_version,
                        ciphertext_type,
                        input_blob_hash,
                        input_blob_index
                    )
                    VALUES($1, $2, $3, $4, $5, $6, $7)
                    ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
                ",
                    tenant_id,
                    &handle,
                    &serialized_ct,
                    ciphertext_version,
                    serialized_type,
                    &blob_hash,
                    ct_idx as i32
                )
                .execute(trx.as_mut())
                .await
                .map_err(Into::<CoprocessorError>::into)?;

                ct_verification
                    .handlesList
                    .push(alloy::primitives::U256::from_be_slice(&handle));
                ct_resp.input_handles.push(InputCiphertextResponseHandle {
                    handle: handle.to_vec(),
                    ciphertext_type: serialized_type as i32,
                });
            }

            let mut span = tracer.child_span("eip_712_signature");
            span.set_attributes(vec![KeyValue::new("blob_idx", idx as i64)]);
            let signing_hash = ct_verification.eip712_signing_hash(&eip_712_domain);
            let eip_712_signature = self.signer.sign_hash_sync(&signing_hash).map_err(|e| {
                CoprocessorError::Eip712SigningFailure {
                    error: e.to_string(),
                }
            })?;
            span.end();

            ct_resp.eip712_signature = eip_712_signature.as_bytes().to_vec();

            response.upload_responses.push(ct_resp);
        }

        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;
        span.end();

        Ok(tonic::Response::new(response))
    }

    async fn async_compute_impl(
        &self,
        request: tonic::Request<tfhe_worker::AsyncComputeRequest>,
        tracer: &GrpcTracer,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GenericResponse>, tonic::Status> {
        let req = request.get_ref();
        if req.computations.len() > self.args.server_maximum_ciphertexts_to_schedule {
            return Err(tonic::Status::from_error(Box::new(
                CoprocessorError::TooManyCiphertextsInBatch {
                    maximum_allowed: self.args.server_maximum_ciphertexts_to_schedule,
                    got: req.computations.len(),
                },
            )));
        }

        let tenant_id = check_if_api_key_is_valid(&request, &self.pool, tracer).await?;

        if req.computations.is_empty() {
            return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
        }

        let mut span = tracer.child_span("sort_computations_by_dependencies");
        // computations are now sorted based on dependencies or error should have
        // been returned if there's circular dependency
        let (sorted_computations, _handles_to_check_in_db) =
            sort_computations_by_dependencies(&req.computations)?;
        span.end();

        // to insert to db
        let mut computations_inputs: Vec<Vec<Vec<u8>>> =
            Vec::with_capacity(sorted_computations.len());
        let mut computations_outputs: Vec<Vec<u8>> = Vec::with_capacity(sorted_computations.len());
        let mut are_comps_scalar: Vec<bool> = Vec::with_capacity(sorted_computations.len());
        for comp in &sorted_computations {
            computations_outputs.push(comp.output_handle.clone());
            let mut is_computation_scalar = false;
            let mut this_comp_inputs: Vec<Vec<u8>> = Vec::with_capacity(comp.inputs.len());
            let mut is_scalar_op_vec: Vec<bool> = Vec::with_capacity(comp.inputs.len());
            for (idx, ih) in comp.inputs.iter().enumerate() {
                let fhe_op: SupportedFheOperations = comp
                    .operation
                    .try_into()
                    .map_err(CoprocessorError::FhevmError)?;
                if let Some(input) = &ih.input {
                    match input {
                        Input::InputHandle(ih) => {
                            this_comp_inputs.push(ih.clone());
                            is_scalar_op_vec.push(false);
                        }
                        Input::Scalar(sc) => {
                            is_computation_scalar = true;
                            this_comp_inputs.push(sc.clone());
                            is_scalar_op_vec.push(true);
                            assert!(idx == 1 || fhe_op.does_have_more_than_one_scalar(), "we should have checked earlier that only second operand can be scalar");
                        }
                    }
                }
            }

            // check before we insert computation that it has
            // to succeed according to the type system
            check_fhe_operand_types(comp.operation, &this_comp_inputs, &is_scalar_op_vec)
                .map_err(CoprocessorError::FhevmError)?;

            computations_inputs.push(this_comp_inputs);
            are_comps_scalar.push(is_computation_scalar);
        }

        let computation_buckets: Vec<Bucket> =
            sort_computations_into_bucket(&sorted_computations).await;
        let mut tx_span = tracer.child_span("db_transaction");
        let mut trx = self
            .pool
            .begin()
            .await
            .map_err(Into::<CoprocessorError>::into)?;

        for (idx, comp) in sorted_computations.iter().enumerate() {
            let fhe_operation: i16 = comp.operation.try_into().map_err(|_| {
                CoprocessorError::FhevmError(FhevmError::UnknownFheOperation(comp.operation))
            })?;
            let mut span = tracer.child_span("insert_computation");
            span.set_attributes(vec![KeyValue::new(
                "handle",
                format!("0x{}", hex::encode(&comp.output_handle)),
            )]);
            let _ = query!(
                "
                    INSERT INTO computations(
                        tenant_id,
                        output_handle,
                        dependencies,
                        fhe_operation,
                        is_completed,
                        is_scalar,
                        dependence_chain_id,
                        transaction_id
                    )
                    VALUES($1, $2, $3, $4, false, $5, $6, $7)
                    ON CONFLICT (tenant_id, output_handle, transaction_id) DO NOTHING
                ",
                tenant_id,
                comp.output_handle,
                &computations_inputs[idx],
                fhe_operation,
                are_comps_scalar[idx],
                computation_buckets[idx],
                comp.transaction_id
            )
            .execute(trx.as_mut())
            .await
            .map_err(Into::<CoprocessorError>::into)?;
            span.end();
        }
        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;
        tx_span.end();
        Ok(tonic::Response::new(GenericResponse { response_code: 0 }))
    }

    async fn trivial_encrypt_ciphertexts_impl(
        &self,
        request: tonic::Request<tfhe_worker::TrivialEncryptBatch>,
        tracer: &GrpcTracer,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GenericResponse>, tonic::Status> {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool, tracer).await?;
        let req = request.get_ref();

        let mut unique_handles: BTreeSet<&[u8]> = BTreeSet::new();
        for val in &req.values {
            validate_fhe_type(val.output_type).map_err(CoprocessorError::FhevmError)?;
            if !unique_handles.insert(&val.handle) {
                return Err(CoprocessorError::DuplicateOutputHandleInBatch(format!(
                    "0x{}",
                    hex::encode(&val.handle)
                ))
                .into());
            }
        }

        let mut span = tracer.child_span("db_query_server_key");
        let fetch_key_response = {
            fetch_tenant_server_key(tenant_id, &self.pool, &self.tenant_key_cache)
                .await
                .map_err(tonic::Status::from_error)?
        };
        let server_key = fetch_key_response.server_key;
        span.end();

        let cloned = req.values.clone();
        let inner_tracer = tracer.clone();
        let mut outer_span = tracer.child_span("blocking_trivial_encrypt");
        let out_cts = tokio::task::spawn_blocking(move || {
            let mut span = inner_tracer.child_span("set_sks");
            tfhe::set_server_key(server_key.clone());
            span.end();

            // single threaded implementation, we can optimize later
            let mut res: Vec<(Vec<u8>, i16, Vec<u8>)> = Vec::with_capacity(cloned.len());
            for v in cloned {
                let mut span = inner_tracer.child_span("trivial_encrypt");
                let ct = trivial_encrypt_be_bytes(v.output_type as i16, &v.be_value);
                span.end();
                let mut span = inner_tracer.child_span("compress_ciphertext");
                let (ct_type, ct_bytes) = ct.compress();
                span.end();
                res.push((v.handle, ct_type, ct_bytes));
            }

            res
        })
        .await
        .unwrap();
        outer_span.end();

        let mut tx_span = tracer.child_span("db_transaction_insert_ciphertexts");
        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(Into::<CoprocessorError>::into)?;
        let mut trx = conn.begin().await.map_err(Into::<CoprocessorError>::into)?;

        for (handle, db_type, db_bytes) in out_cts {
            let mut span = tracer.child_span("db_insert_ciphertext");
            span.set_attributes(vec![
                KeyValue::new("handle", format!("0x{}", hex::encode(&handle))),
                KeyValue::new("ciphertext_type", db_type as i64),
            ]);
            sqlx::query!("
                    INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                    VALUES ($1, $2, $3, $4, $5)
                    ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
                ",
                tenant_id, handle, db_bytes, current_ciphertext_version(), db_type as i16
            )
            .execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
            span.end();
        }

        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;
        tx_span.end();

        Ok(tonic::Response::new(GenericResponse { response_code: 0 }))
    }

    async fn get_ciphertexts_impl(
        &self,
        request: tonic::Request<tfhe_worker::GetCiphertextBatch>,
        tracer: &GrpcTracer,
    ) -> std::result::Result<tonic::Response<tfhe_worker::GetCiphertextResponse>, tonic::Status>
    {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool, tracer).await?;
        let req = request.get_ref();

        if req.handles.len() > self.args.server_maximum_ciphertexts_to_get {
            return Err(tonic::Status::from_error(Box::new(
                CoprocessorError::MoreThanMaximumCiphertextsAttemptedToDownload {
                    input_count: req.handles.len(),
                    maximum_allowed: self.args.server_maximum_ciphertexts_to_get,
                },
            )));
        }

        let mut result = tfhe_worker::GetCiphertextResponse {
            responses: Vec::new(),
        };
        let mut set = BTreeSet::new();

        for h in &req.handles {
            let _ = set.insert(h.clone());
        }

        let cts: Vec<Vec<u8>> = set.into_iter().collect();

        let mut span = tracer.child_span("query_ciphertexts");
        span.set_attribute(KeyValue::new("count", cts.len() as i64));
        let db_cts = query!(
            "
                SELECT handle, ciphertext_type, ciphertext_version, ciphertext
                FROM ciphertexts
                WHERE tenant_id = $1
                AND handle = ANY($2::BYTEA[])
            ",
            tenant_id,
            &cts
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::<CoprocessorError>::into)?;
        span.end();

        let mut the_map: BTreeMap<Vec<u8>, _> = BTreeMap::new();
        for ct in db_cts {
            let _ = the_map.insert(ct.handle.clone(), ct);
        }

        for h in &req.handles {
            let ciphertext: Result<Option<FetchedCiphertext>, tonic::Status> = the_map
                .get(h)
                .map(|res| {
                    let signature_data = GetCiphertextResponseSignatureData {
                        handle: alloy::primitives::U256::from_be_slice(h),
                        ciphertext_digest: Keccak256::digest(&the_map.get(h).unwrap().ciphertext)
                            .to_vec()
                            .into(),
                    };
                    let signing_hash =
                        signature_data.eip712_signing_hash(&self.get_ciphertext_eip712_domain);
                    let signature = self.signer.sign_hash_sync(&signing_hash).map_err(|e| {
                        CoprocessorError::Eip712SigningFailure {
                            error: e.to_string(),
                        }
                    })?;
                    Ok(FetchedCiphertext {
                        ciphertext_bytes: res.ciphertext.clone(),
                        ciphertext_type: res.ciphertext_type as i32,
                        ciphertext_version: res.ciphertext_version as i32,
                        signature: signature.into(),
                    })
                })
                .transpose();
            result.responses.push(GetCiphertextSingleResponse {
                handle: h.clone(),
                ciphertext: ciphertext?,
            });
        }

        Ok(tonic::Response::new(result))
    }
}

type Handle = Vec<u8>;
type Bucket = Vec<u8>;

async fn sort_computations_into_bucket(
    computations: &[&crate::server::tfhe_worker::AsyncComputation],
) -> Vec<Bucket> {
    let mut res: Vec<Bucket> = vec![vec![0]; computations.len()];
    let mut bucket_map: HashMap<Handle, Bucket> = HashMap::with_capacity(computations.len());
    'comps: for (idx, comp) in computations.iter().enumerate() {
        let output = &comp.output_handle;
        for ih in comp.inputs.iter() {
            let Some(Input::InputHandle(input)) = &ih.input else {
                continue;
            };
            let Some(ce) = bucket_map.get(input).cloned() else {
                continue;
            };
            bucket_map.insert(output.to_owned(), ce.to_owned());
            res[idx] = ce;
            continue 'comps;
        }
        // If this computation is not linked to any others, assign it
        // to a new bucket
        res[idx] = output.to_owned();
        bucket_map.insert(output.to_owned(), res[idx].to_owned());
    }
    res
}
