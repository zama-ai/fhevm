use crate::utils::safe_deserialize_key;
use sqlx::query;
use std::sync::Arc;

pub struct TfheTenantKeys {
    pub tenant_id: i32,
    pub chain_id: i32,
    pub verifying_contract_address: String,
    pub acl_contract_address: String,
    pub sks: tfhe::ServerKey,

    // only used in tests, that's why we put dead_code
    #[allow(dead_code)]
    pub pks: tfhe::CompactPublicKey,
    #[allow(dead_code)]
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
}

pub struct FetchTenantKeyResult {
    pub chain_id: i32,
    pub verifying_contract_address: String,
    pub acl_contract_address: String,
    pub server_key: tfhe::ServerKey,
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
    pub pks: tfhe::CompactPublicKey,
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
                    public_params: key.public_params.clone(),
                    pks: key.pks.clone(),
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
