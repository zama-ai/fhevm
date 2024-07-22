use std::collections::{BTreeSet, HashSet};

use coprocessor::DebugDecryptResponse;
use sqlx::query;
use tfhe::prelude::FheTryTrivialEncrypt;
use tfhe::FheUint32;
use tonic::transport::Server;
use crate::db_queries::{check_if_api_key_is_valid, check_if_ciphertexts_exist_in_db};
use crate::utils::check_valid_ciphertext_handle;
use crate::types::{CoprocessorError, SupportedFheCiphertexts};
use crate::tfhe_ops::{self, current_ciphertext_version};
use crate::server::coprocessor::GenericResponse;

pub mod coprocessor {
    tonic::include_proto!("coprocessor");
}

pub struct CoprocessorService {
    pool: sqlx::Pool<sqlx::Postgres>,
}

pub async fn run_server(args: crate::cli::Args) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
    let addr = args.server_addr.parse().expect("Can't parse server address");
    let db_url = crate::utils::db_url();

    println!("Coprocessor listening on {}", addr);
    let pool = sqlx::postgres::PgPoolOptions::new()
        .max_connections(args.pg_pool_max_connections)
        .connect(&db_url)
        .await?;

    let service = CoprocessorService {
        pool,
    };

    Server::builder()
        .add_service(crate::server::coprocessor::fhevm_coprocessor_server::FhevmCoprocessorServer::new(service))
        .serve(addr)
        .await?;

    Ok(())
}

#[tonic::async_trait]
impl coprocessor::fhevm_coprocessor_server::FhevmCoprocessor for CoprocessorService {
    async fn debug_encrypt_ciphertext(
          &self,
          request: tonic::Request<coprocessor::DebugEncryptRequest>,
    ) -> std::result::Result<
        tonic::Response<coprocessor::GenericResponse>,
        tonic::Status,
    > {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;
        let req = request.get_ref();

        let mut public_key = sqlx::query!("
          SELECT sks_key
          FROM tenants
          WHERE tenant_id = $1
        ", tenant_id).fetch_all(&self.pool).await.map_err(Into::<CoprocessorError>::into)?;

        assert_eq!(public_key.len(), 1);

        let public_key = public_key.pop().unwrap();

        let value_to_encrypt = req.original_value as u32;
        let handle = req.handle.clone();
        let (db_type, db_bytes) = tokio::task::spawn_blocking(move || {
            let server_key: tfhe::ServerKey = bincode::deserialize(&public_key.sks_key).unwrap();
            tfhe::set_server_key(server_key);
            let encrypted = FheUint32::try_encrypt_trivial(value_to_encrypt).unwrap();
            SupportedFheCiphertexts::FheUint32(encrypted).serialize()
        }).await.unwrap();

        sqlx::query!("
          INSERT INTO ciphertexts(tenant_id, handle, ciphertext, ciphertext_version, ciphertext_type)
          VALUES ($1, $2, $3, $4, $5)
        ", tenant_id, handle, db_bytes, current_ciphertext_version(), db_type)
        .execute(&self.pool).await.map_err(Into::<CoprocessorError>::into)?;

        return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
    }

    async fn debug_decrypt_ciphertext(
          &self,
          request: tonic::Request<coprocessor::DebugDecryptRequest>,
    ) -> std::result::Result<
        tonic::Response<coprocessor::DebugDecryptResponse>,
        tonic::Status,
    > {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;
        let req = request.get_ref();

        let mut priv_key = sqlx::query!("
          SELECT cks_key
          FROM tenants
          WHERE tenant_id = $1
        ", tenant_id).fetch_all(&self.pool).await.map_err(Into::<CoprocessorError>::into)?;

        if priv_key.is_empty() || priv_key[0].cks_key.is_none() {
            return Err(tonic::Status::not_found("tenant private key not found"));
        }

        assert_eq!(priv_key.len(), 1);

        let mut cts = sqlx::query!("
          SELECT ciphertext, ciphertext_type
          FROM ciphertexts
          WHERE tenant_id = $1
          AND handle = $2
          AND ciphertext_version = $3
        ", tenant_id, &req.handle, current_ciphertext_version())
        .fetch_all(&self.pool)
        .await.map_err(Into::<CoprocessorError>::into)?;

        if cts.is_empty() {
            return Err(tonic::Status::not_found("ciphertext not found"));
        }

        assert_eq!(cts.len(), 1);

        let priv_key = priv_key.pop().unwrap().cks_key.unwrap();
        let ciphertext = cts.pop().unwrap();

        let value = tokio::task::spawn_blocking(move || {
            let client_key: tfhe::ClientKey = bincode::deserialize(&priv_key).unwrap();
            let deserialized = tfhe_ops::deserialize_fhe_ciphertext(ciphertext.ciphertext_type, &ciphertext.ciphertext).unwrap();
            deserialized.decrypt(&client_key)
        }).await.unwrap();

        return Ok(tonic::Response::new(DebugDecryptResponse { value }));
    }

    async fn upload_ciphertexts(
          &self,
          request: tonic::Request<coprocessor::CiphertextUploadBatch>,
    ) -> std::result::Result<
        tonic::Response<coprocessor::GenericResponse>,
        tonic::Status,
    > {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;

        let req = request.get_ref();

        // TODO: check if ciphertext deserializes into type correctly
        // TODO: check for duplicate handles in the input
        // TODO: check if ciphertext doesn't exist already
        // TODO: if ciphertexts exists check that it is equal to the one being uploaded

        let mut trx = self.pool.begin().await.map_err(Into::<CoprocessorError>::into)?;
        for i_ct in &req.input_ciphertexts {
            let ciphertext_type: i16 = i_ct.ciphertext_type.try_into()
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

    async fn schedule_computations(
          &self,
          request: tonic::Request<coprocessor::SchedulerBatch>,
    ) -> std::result::Result<
        tonic::Response<coprocessor::GenericResponse>,
        tonic::Status,
    > {
        let tenant_id = check_if_api_key_is_valid(&request, &self.pool).await?;

        let req = request.get_ref();
        let tenant =
            query!("SELECT tenant_id FROM tenants WHERE tenant_id = $1", tenant_id)
                .fetch_all(&self.pool)
                .await
                .map_err(Into::<CoprocessorError>::into)?;

        if tenant.is_empty() {
            return Err(tonic::Status::not_found("tenant not found"));
        }

        if req.input_handles.is_empty() {
            return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
        }

        let mut output_handles: HashSet<&str> = HashSet::new();
        for ct in &req.input_handles {
            check_valid_ciphertext_handle(ct.output_handle.as_str())?;
            if !output_handles.insert(ct.output_handle.as_str()) {
                return Err(
                    tonic::Status::from_error(
                        Box::new(CoprocessorError::DuplicateOutputHandleInBatch(ct.output_handle.clone()))
                    )
                );
            }
        }

        let mut handles_to_check_in_db: BTreeSet<String> = BTreeSet::new();
        for ct in &req.input_handles {
            for ih in &ct.input_handles {
                check_valid_ciphertext_handle(ih.as_str())?;
                if ih == &ct.output_handle {
                    return Err(tonic::Status::from_error(
                        Box::new(CoprocessorError::OutputHandleIsAlsoInputHandle(ih.clone()))
                    ));
                }

                if !output_handles.contains(ih.as_str()) {
                    handles_to_check_in_db.insert(ih.clone());
                }
            }
        }

        check_if_ciphertexts_exist_in_db(handles_to_check_in_db, tenant_id, &self.pool).await?;
        // TODO: check that input handles for operation are of uniform type and correct amount of inputs for fhe operation
        
        let mut trx = self.pool.begin().await.map_err(Into::<CoprocessorError>::into)?;
        let mut new_work_available = false;
        for ct in &req.input_handles {
            let fhe_operation: i16 =
                ct.fhe_operation.try_into().map_err(|_| CoprocessorError::UnknownFheOperation(ct.fhe_operation))?;
            let res = query!(
                "
                    INSERT INTO computations(tenant_id, output_handle, dependencies_handles, fhe_operation, is_completed)
                    VALUES($1, $2, $3, $4, false)
                    ON CONFLICT (tenant_id, output_handle) DO NOTHING
                ",
                tenant_id, ct.output_handle, &ct.input_handles, fhe_operation
            ).execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
            if res.rows_affected() > 0 {
                new_work_available = true;
            }
        }
        if new_work_available {
            query!("NOTIFY work_available").execute(trx.as_mut()).await.map_err(Into::<CoprocessorError>::into)?;
        }
        trx.commit().await.map_err(Into::<CoprocessorError>::into)?;
        return Ok(tonic::Response::new(GenericResponse { response_code: 0 }));
    }

    async fn wait_computations(
          &self,
          _request: tonic::Request<coprocessor::SchedulerBatch>,
    ) -> std::result::Result<
        tonic::Response<coprocessor::FhevmResponses>,
        tonic::Status,
    > {
        return Err(tonic::Status::unimplemented("not implemented"));
    }
}