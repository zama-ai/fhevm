use std::num::NonZeroUsize;
use std::sync::Arc;

use crate::types::TfheTenantKeys;
use fhevm_engine_common::utils::safe_deserialize_key;
use sqlx::query;

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;
use tracing::warn;

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

        if keys.len() > key_cache.cap().into() {
            warn!(target: "tfhe_worker",
                  { key_cache_size = key_cache.cap(), fetched_keys = keys.len()},
                "TFHE worker key cache size insufficient, increasing"
            );
            key_cache.resize(NonZeroUsize::new(keys.len()).unwrap());
        }
        for key in keys {
            key_cache.put(key.tenant_id, key);
        }
    }

    Ok(())
}
