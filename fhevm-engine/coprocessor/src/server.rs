use crate::db_queries::{check_if_api_key_is_valid, check_if_ciphertexts_exist_in_db};
use crate::server::coprocessor::GenericResponse;
use crate::tfhe_ops::{
    self, check_fhe_operand_types, current_ciphertext_version, debug_trivial_encrypt_be_bytes,
};
use crate::types::CoprocessorError;
use crate::utils::sort_computations_by_dependencies;
use coprocessor::async_computation_input::Input;
use coprocessor::{DebugDecryptResponse, DebugDecryptResponseSingle};
use sqlx::{query, Acquire};
use tonic::transport::Server;

pub mod coprocessor {
    tonic::include_proto!("coprocessor");
}

pub struct CoprocessorService {
    pool: sqlx::Pool<sqlx::Postgres>,
    args: crate::cli::Args,
}

pub async fn run_server(
    args: crate::cli::Args,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = args
        .server_addr
        .parse()
        .expect("Can't parse server address");
    let db_url = crate::utils::db_url(&args);

    println!("Coprocessor listening on {}", addr);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.pg_pool_max_connections)
        .connect(&db_url)
        .await?;

    let service = CoprocessorService { pool, args };

    Server::builder()
        .add_service(
            crate::server::coprocessor::fhevm_coprocessor_server::FhevmCoprocessorServer::new(
                service,
            ),
        )
        .serve(addr)
        .await?;

    Ok(())
}

#[tonic::async_trait]
impl coprocessor::fhevm_coprocessor_server::FhevmCoprocessor for CoprocessorService {
    async fn debug_encrypt_ciphertext(
        &self,
        request: tonic::Request<coprocessor::DebugEncryptRequest>,
    ) -> std::result::Result<tonic::Response<coprocessor::GenericResponse>, tonic::Status> {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;
        let req = request.get_ref();

        let mut public_key = sqlx::query!(
            "
          SELECT sks_key
          FROM tenants
          WHERE tenant_id = $1
        ",
            tenant_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::<CoprocessorError>::into)?;

        assert_eq!(public_key.len(), 1);

        let public_key = public_key.pop().unwrap();

        let cloned = req.values.clone();
        let out_cts = tokio::task::spawn_blocking(move || {
            let server_key: tfhe::ServerKey = bincode::deserialize(&public_key.sks_key).unwrap();
            tfhe::set_server_key(server_key);

            // single threaded implementation as this is debug function and it is simple to implement
            let mut res: Vec<(Vec<u8>, i16, Vec<u8>)> = Vec::with_capacity(cloned.len());
            for v in cloned {
                let ct = debug_trivial_encrypt_be_bytes(v.output_type as i16, &v.le_value);
                let (ct_type, ct_bytes) = ct.serialize();
                res.push((v.handle, ct_type, ct_bytes));
            }

            res
        })
        .await
        .unwrap();

        let mut conn = self
            .pool
            .acquire()
            .await
            .map_err(Into::<CoprocessorError>::into)?;
        let mut trx = conn.begin().await.map_err(Into::<CoprocessorError>::into)?;

        for (handle, db_type, db_bytes) in out_cts {
            sqlx::query!("
                    INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
                    VALUES ($1, $2, $3, $4, $5)
                ",
                tenant_id, handle, db_bytes, current_ciphertext_version(), db_type as i16
            )
            .execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
        }

        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;

        return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
    }

    async fn debug_decrypt_ciphertext(
        &self,
        request: tonic::Request<coprocessor::DebugDecryptRequest>,
    ) -> std::result::Result<tonic::Response<coprocessor::DebugDecryptResponse>, tonic::Status>
    {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;
        let req = request.get_ref();

        let mut priv_key = sqlx::query!(
            "
          SELECT cks_key
          FROM tenants
          WHERE tenant_id = $1
        ",
            tenant_id
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::<CoprocessorError>::into)?;

        if priv_key.is_empty() || priv_key[0].cks_key.is_none() {
            return Err(tonic::Status::not_found("tenant private key not found"));
        }

        assert_eq!(priv_key.len(), 1);

        let cts = sqlx::query!(
            "
          SELECT ciphertext, ciphertext_type, handle
          FROM ciphertexts
          WHERE tenant_id = $1
          AND handle = ANY($2::BYTEA[])
          AND ciphertext_version = $3
        ",
            tenant_id,
            &req.handles,
            current_ciphertext_version()
        )
        .fetch_all(&self.pool)
        .await
        .map_err(Into::<CoprocessorError>::into)?;

        if cts.is_empty() {
            return Err(tonic::Status::not_found("ciphertext not found"));
        }

        let priv_key = priv_key.pop().unwrap().cks_key.unwrap();

        let values = tokio::task::spawn_blocking(move || {
            let client_key: tfhe::ClientKey = bincode::deserialize(&priv_key).unwrap();

            let mut decrypted: Vec<DebugDecryptResponseSingle> = Vec::with_capacity(cts.len());
            for ct in cts {
                let deserialized =
                    tfhe_ops::deserialize_fhe_ciphertext(ct.ciphertext_type, &ct.ciphertext)
                        .unwrap();
                decrypted.push(DebugDecryptResponseSingle {
                    output_type: ct.ciphertext_type as i32,
                    value: deserialized.decrypt(&client_key),
                });
            }

            decrypted
        })
        .await
        .unwrap();

        return Ok(tonic::Response::new(DebugDecryptResponse { values }));
    }

    async fn upload_ciphertexts(
        &self,
        request: tonic::Request<coprocessor::CiphertextUploadBatch>,
    ) -> std::result::Result<tonic::Response<coprocessor::GenericResponse>, tonic::Status> {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;

        let req = request.get_ref();

        // TODO: check if ciphertext deserializes into type correctly
        // TODO: check for duplicate handles in the input
        // TODO: check if ciphertext doesn't exist already
        // TODO: if ciphertexts exists check that it is equal to the one being uploaded

        let mut trx = self
            .pool
            .begin()
            .await
            .map_err(Into::<CoprocessorError>::into)?;
        for i_ct in &req.input_ciphertexts {
            let ciphertext_type: i16 = i_ct
                .ciphertext_type
                .try_into()
                .map_err(|_e| CoprocessorError::UnknownFheType(i_ct.ciphertext_type))?;
            let _ = sqlx::query!("
              INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
              VALUES($1, $2, $3, $4, $5)
              ON CONFLICT (tenant_id, handle, ciphertext_version) DO NOTHING
            ", tenant_id, i_ct.ciphertext_handle, i_ct.ciphertext_bytes, current_ciphertext_version(), ciphertext_type)
            .execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
        }

        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;

        return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
    }

    async fn async_compute(
        &self,
        request: tonic::Request<coprocessor::AsyncComputeRequest>,
    ) -> std::result::Result<tonic::Response<coprocessor::GenericResponse>, tonic::Status> {
        let req = request.get_ref();
        if req.computations.len() > self.args.server_maximum_ciphertexts_to_schedule {
            return Err(tonic::Status::from_error(Box::new(
                CoprocessorError::TooManyCiphertextsInBatch {
                    maximum_allowed: self.args.server_maximum_ciphertexts_to_schedule,
                    got: req.computations.len(),
                },
            )));
        }

        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;

        if req.computations.is_empty() {
            return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
        }

        // computations are now sorted based on dependencies or error should have
        // been returned if there's circular dependency
        let (sorted_computations, handles_to_check_in_db) =
            sort_computations_by_dependencies(&req.computations)?;

        // to insert to db
        let mut ct_types =
            check_if_ciphertexts_exist_in_db(handles_to_check_in_db, tenant_id, &self.pool).await?;
        let mut computations_inputs: Vec<Vec<Vec<u8>>> =
            Vec::with_capacity(sorted_computations.len());
        let mut are_comps_scalar: Vec<bool> = Vec::with_capacity(sorted_computations.len());
        for comp in &sorted_computations {
            let mut handle_types = Vec::with_capacity(comp.inputs.len());
            let mut is_computation_scalar = false;
            let mut this_comp_inputs: Vec<Vec<u8>> = Vec::with_capacity(comp.inputs.len());
            for (idx, ih) in comp.inputs.iter().enumerate() {
                if let Some(input) = &ih.input {
                    match input {
                        Input::InputHandle(ih) => {
                            let ct_type = ct_types
                                .get(ih)
                                .expect("this must be found if operand is non scalar");
                            handle_types.push(*ct_type);
                            this_comp_inputs.push(ih.clone());
                        }
                        Input::Scalar(sc) => {
                            is_computation_scalar = true;
                            handle_types.push(-1);
                            this_comp_inputs.push(sc.clone());
                            assert!(idx == 1, "we should have checked earlier that only second operand can be scalar");
                        }
                    }
                }
            }

            computations_inputs.push(this_comp_inputs);
            are_comps_scalar.push(is_computation_scalar);
            // check before we insert computation that it has
            // to succeed according to the type system
            let output_type = check_fhe_operand_types(
                comp.operation,
                &handle_types,
                is_computation_scalar,
                &comp.inputs,
            )?;
            // fill in types with output handles that are computed as we go
            assert!(ct_types
                .insert(comp.output_handle.clone(), output_type)
                .is_none());
        }

        let mut trx = self
            .pool
            .begin()
            .await
            .map_err(Into::<CoprocessorError>::into)?;
        let mut new_work_available = false;
        for (idx, comp) in sorted_computations.iter().enumerate() {
            let fhe_operation: i16 = comp
                .operation
                .try_into()
                .map_err(|_| CoprocessorError::UnknownFheOperation(comp.operation))?;
            let res = query!(
                "
                    INSERT INTO computations(tenant_id, output_handle, dependencies, fhe_operation, is_completed, is_scalar)
                    VALUES($1, $2, $3, $4, false, $5)
                    ON CONFLICT (tenant_id, output_handle) DO NOTHING
                ",
                tenant_id, comp.output_handle, &computations_inputs[idx], fhe_operation, are_comps_scalar[idx]
            ).execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
            if res.rows_affected() > 0 {
                new_work_available = true;
            }
        }
        if new_work_available {
            query!("NOTIFY work_available")
                .execute(trx.as_mut())
                .await
                .map_err(Into::<CoprocessorError>::into)?;
        }
        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;
        return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
    }

    async fn wait_computations(
        &self,
        _request: tonic::Request<coprocessor::AsyncComputeRequest>,
    ) -> std::result::Result<tonic::Response<coprocessor::FhevmResponses>, tonic::Status> {
        return Err(tonic::Status::unimplemented("not implemented"));
    }
}
