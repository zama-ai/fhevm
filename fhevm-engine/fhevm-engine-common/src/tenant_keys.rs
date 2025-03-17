use crate::utils::safe_deserialize_key;
use sqlx::Row;
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
    pub tenant_id: i32,
    pub chain_id: i32,
    pub verifying_contract_address: String,
    pub acl_contract_address: String,
    pub server_key: tfhe::ServerKey,
    pub public_params: Arc<tfhe::zk::CompactPkeCrs>,
    pub pks: tfhe::CompactPublicKey,
}

/// Returns chain id and verifying contract address for EIP712 signature and tfhe server key
pub async fn fetch_tenant_server_key<'a, T>(
    id: i32,
    pool: T,
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
    is_tenant_id: bool,
) -> Result<FetchTenantKeyResult, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a> + Copy,
{
    // try getting from cache until it succeeds with populating cache
    loop {
        {
            let mut w = tenant_key_cache.write().await;
            if let Some(key) = w.get(&id) {
                return Ok(FetchTenantKeyResult {
                    tenant_id: key.tenant_id,
                    chain_id: key.chain_id,
                    verifying_contract_address: key.verifying_contract_address.clone(),
                    acl_contract_address: key.acl_contract_address.clone(),
                    server_key: key.sks.clone(),
                    public_params: key.public_params.clone(),
                    pks: key.pks.clone(),
                });
            }
        }

        populate_cache_with_tenant_keys(vec![id], pool, tenant_key_cache, is_tenant_id).await?;
    }
}
pub async fn query_tenant_keys<'a, T>(
    ids_to_query: Vec<i32>,
    conn: T,
    is_tenant_id: bool,
) -> Result<Vec<TfheTenantKeys>, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let column = if is_tenant_id {
        "tenant_id"
    } else {
        "chain_id"
    };

    let query_str = format!(
        "
            SELECT tenant_id, chain_id, acl_contract_address, verifying_contract_address, pks_key, sks_key, public_params
            FROM tenants
            WHERE {} = ANY($1::INT[])
        ",
        column
    );

    let rows = sqlx::query(&query_str)
        .bind(&ids_to_query)
        .fetch_all(conn)
        .await?;

    let mut res = Vec::with_capacity(rows.len());

    for row in rows {
        let tenant_id: i32 = row.try_get("tenant_id")?;
        let chain_id: i32 = row.try_get("chain_id")?;
        let acl_contract_address: String = row.try_get("acl_contract_address")?;
        let verifying_contract_address: String = row.try_get("verifying_contract_address")?;
        let pks_key: Vec<u8> = row.try_get("pks_key")?;
        let sks_key: Vec<u8> = row.try_get("sks_key")?;
        let public_params_key: Vec<u8> = row.try_get("public_params")?;

        // Deserialize binary keys properly
        let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;
        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
        let public_params: tfhe::zk::CompactPkeCrs = safe_deserialize_key(&public_params_key)?;

        res.push(TfheTenantKeys {
            tenant_id,
            chain_id,
            acl_contract_address,
            verifying_contract_address,
            sks,
            pks,
            public_params: Arc::new(public_params),
        });
    }

    Ok(res)
}

pub async fn populate_cache_with_tenant_keys<'a, T>(
    tenants_to_query: Vec<i32>,
    conn: T,
    tenant_key_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<i32, TfheTenantKeys>>>,
    is_tenant_id: bool,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    if !tenants_to_query.is_empty() {
        let mut key_cache = tenant_key_cache.write().await;
        if tenants_to_query
            .iter()
            .all(|id| key_cache.get(id).is_some())
        {
            // All IDs are already in the key cache, no need to re-query the keys
            return Ok(());
        }

        tracing::info!(
            message = "query tenants",
            tenants = format!("{:?}", tenants_to_query),
            is_tenant_id
        );

        let keys = query_tenant_keys(tenants_to_query, conn, is_tenant_id).await?;

        assert!(
            !keys.is_empty(),
            "We should have keys here, otherwise our database is corrupt"
        );

        for key in keys {
            let id = if is_tenant_id {
                key.tenant_id
            } else {
                key.chain_id
            };

            key_cache.put(id, key);
        }
    }

    Ok(())
}

pub struct TenantInfo {
    pub key_id: i32,
    pub chain_id: i32,
}

/// Returns the key_id, chain_id for a given tenant_id
pub async fn query_tenant_info<'a, T>(
    conn: T,
    tenant_id: i32,
) -> Result<TenantInfo, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let row = sqlx::query(
        "SELECT key_id, chain_id FROM tenants
            WHERE tenant_id = $1::INT",
    )
    .bind(tenant_id)
    .fetch_one(conn)
    .await?;

    let res: TenantInfo = TenantInfo {
        key_id: row.try_get("key_id")?,
        chain_id: row.try_get("chain_id")?,
    };

    Ok(res)
}
