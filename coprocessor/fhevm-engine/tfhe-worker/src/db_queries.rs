use std::str::FromStr;
use std::sync::Arc;

use crate::server::GrpcTracer;
use crate::types::{CoprocessorError, TfheTenantKeys};
use fhevm_engine_common::utils::safe_deserialize_key;
use opentelemetry::trace::Span;
use opentelemetry::KeyValue;
use sqlx::{query, Postgres};

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;

/// Returns tenant id upon valid authorization request
pub async fn check_if_api_key_is_valid<T>(
    req: &tonic::Request<T>,
    pool: &sqlx::Pool<Postgres>,
    ctx: &GrpcTracer,
) -> Result<i32, CoprocessorError> {
    let mut outer_span = ctx.child_span("check_api_key_validity");
    match req.metadata().get("authorization") {
        Some(auth) => {
            let auth_header = String::from_utf8(auth.as_bytes().to_owned())
                .map_err(|_| CoprocessorError::Unauthorized)?
                .to_lowercase();

            let prefix = "bearer ";
            if !auth_header.starts_with(prefix) {
                return Err(CoprocessorError::Unauthorized);
            }

            let tail = &auth_header[prefix.len()..];
            let api_key = tail.trim();
            let api_key = match sqlx::types::Uuid::from_str(api_key) {
                Ok(uuid) => uuid,
                Err(_) => return Err(CoprocessorError::Unauthorized),
            };

            let mut span = ctx.child_span("db_query_api_key");
            let tenant = query!(
                "SELECT tenant_id FROM tenants WHERE tenant_api_key = $1",
                api_key
            )
            .fetch_all(pool)
            .await
            .map_err(Into::<CoprocessorError>::into)?;
            span.end();

            if tenant.is_empty() {
                return Err(CoprocessorError::Unauthorized);
            }

            let tenant_id = tenant[0].tenant_id;
            outer_span.set_attribute(KeyValue::new("tenant_id", tenant_id as i64));
            Ok(tenant_id)
        }
        None => Err(CoprocessorError::Unauthorized),
    }
}

pub struct FetchTenantKeyResult {
    pub chain_id: i32,
    pub verifying_contract_address: String,
    pub acl_contract_address: String,
    pub server_key: tfhe::ServerKey,
    #[cfg(feature = "gpu")]
    pub gpu_server_key: Vec<tfhe::CudaServerKey>,
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
}

/// Returns chain id and verifying contract address for EIP712 signature and tfhe server key
pub async fn fetch_tenant_server_key<'a, T>(
    tenant_id: i32,
    pool: T,
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
) -> Result<FetchTenantKeyResult, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a> + Copy,
{
    // try getting from cache until it succeeds with populating cache
    loop {
        {
            let mut w = tenant_key_cache.write().await;
            if let Some(key) = w.get(&tenant_id) {
                return Ok(FetchTenantKeyResult {
                    chain_id: key.chain_id,
                    verifying_contract_address: key.verifying_contract_address.clone(),
                    acl_contract_address: key.acl_contract_address.clone(),
                    server_key: key.sks.clone(),
                    #[cfg(feature = "gpu")]
                    gpu_server_key: key.gpu_sks.clone(),
                    public_params: key.public_params.clone(),
                });
            }
        }

        populate_cache_with_tenant_keys(vec![tenant_id], pool, tenant_key_cache).await?;
    }
}

pub async fn query_tenant_keys<'a, T>(
    tenants_to_query: Vec<i32>,
    conn: T,
) -> Result<Vec<TfheTenantKeys>, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let mut res = Vec::with_capacity(tenants_to_query.len());
    let keys = query!(
        "
            SELECT tenant_id, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params
            FROM tenants
            WHERE tenant_id = ANY($1::INT[])
        ",
        &tenants_to_query
    )
    .fetch_all(conn)
    .await?;
    for key in keys {
        #[cfg(not(feature = "gpu"))]
        {
            let sks: tfhe::ServerKey = safe_deserialize_key(&key.sks_key)
                .expect("We can't deserialize our own validated sks key");
            let pks: tfhe::CompactPublicKey = safe_deserialize_key(&key.pks_key)
                .expect("We can't deserialize our own validated pks key");
            let public_params: tfhe::zk::CompactPkeCrs = safe_deserialize_key(&key.public_params)
                .expect("We can't deserialize our own validated public params");
            res.push(TfheTenantKeys {
                tenant_id: key.tenant_id,
                sks,
                pks,
                public_params: Arc::new(public_params),
                chain_id: key.chain_id,
                acl_contract_address: key.acl_contract_address,
                verifying_contract_address: key.verifying_contract_address,
            });
        }
        #[cfg(feature = "gpu")]
        {
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&key.sks_key)
                .expect("We can't deserialize the gpu compressed sks key");
            let pks: tfhe::CompactPublicKey = safe_deserialize_key(&key.pks_key)
                .expect("We can't deserialize our own validated pks key");
            let public_params: tfhe::zk::CompactPkeCrs = safe_deserialize_key(&key.public_params)
                .expect("We can't deserialize our own validated public params");
            let num_gpus = get_number_of_gpus() as u64;
            res.push(TfheTenantKeys {
                tenant_id: key.tenant_id,
                pks,
                sks: csks.clone().decompress(),
                csks: csks.clone(),
                #[cfg(feature = "latency")]
                gpu_sks: vec![csks.decompress_to_gpu()],
                #[cfg(not(feature = "latency"))]
                gpu_sks: (0..num_gpus)
                    .map(|i| csks.decompress_to_specific_gpu(tfhe::GpuIndex::new(i as u32)))
                    .collect::<Vec<_>>(),
                public_params: Arc::new(public_params),
                chain_id: key.chain_id,
                acl_contract_address: key.acl_contract_address,
                verifying_contract_address: key.verifying_contract_address,
            });
        }
    }

    Ok(res)
}

pub async fn populate_cache_with_tenant_keys<'a, T>(
    tenants_to_query: Vec<i32>,
    conn: T,
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    if !tenants_to_query.is_empty() {
        let keys = query_tenant_keys(tenants_to_query, conn).await?;

        assert!(
            !keys.is_empty(),
            "We should have keys here, otherwise our database is corrupt"
        );

        let mut key_cache = tenant_key_cache.write().await;

        for key in keys {
            key_cache.put(key.tenant_id, key);
        }
    }

    Ok(())
}
