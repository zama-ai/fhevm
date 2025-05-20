use crate::utils::safe_deserialize_key;
use sqlx::{
    postgres::{types::Oid, PgRow},
    PgPool, Row,
};
use std::sync::Arc;
use tracing::info;

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
        #[cfg(not(feature = "gpu"))]
        let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;
        #[cfg(feature = "gpu")]
        let sks = {
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)?;
            csks.decompress()
        };
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
    /// The key_id of the tenant
    pub key_id: [u8; 32],
    /// The chain id of the tenant
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

    let key_id_vec: Vec<u8> = row.try_get("key_id")?;

    let key_id: [u8; 32] = key_id_vec
        .try_into()
        .map_err(|_| "Failed to convert key_id to [u8; 32]")?;

    let res: TenantInfo = TenantInfo {
        key_id,
        chain_id: row.try_get("chain_id")?,
    };

    Ok(res)
}

const CHUNK_SIZE: i32 = 64 * 1024; // 64KiB
pub async fn read_keys_from_large_object(
    pool: &PgPool,
    tenant_api_key: &String,
    keys_column_name: &str,
    capacity: usize,
) -> anyhow::Result<Vec<u8>> {
    let query = format!(
        "SELECT {} FROM tenants WHERE tenant_api_key = $1::uuid",
        keys_column_name
    );

    // Read the Oid of the large object
    let row: PgRow = sqlx::query(&query)
        .bind(tenant_api_key)
        .fetch_one(pool)
        .await?;

    let oid: Oid = row.try_get(0)?;
    info!("Retrieved oid: {:?}, column: {}", oid, keys_column_name);

    read_large_object_in_chunks(pool, oid, CHUNK_SIZE, capacity).await
}

/// Read a large object by Oid from the database in chunks
pub async fn read_large_object_in_chunks(
    pool: &PgPool,
    large_object_oid: Oid,
    chunk_size: i32,
    capacity: usize,
) -> anyhow::Result<Vec<u8>> {
    const INV_READ: i32 = 262144;
    // DB transaction must be kept open until the large object is being read
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    let row = sqlx::query("SELECT lo_open($1, $2)")
        .bind(large_object_oid)
        .bind(INV_READ)
        .fetch_one(&mut *tx)
        .await?;

    let fd: i32 = row.try_get(0)?;
    info!(
        "Large Object oid: {:?}, fd: {}, chunk size: {}",
        large_object_oid, fd, chunk_size
    );

    let mut bytes = Vec::with_capacity(capacity);

    loop {
        let chunk = sqlx::query("SELECT loread($1, $2)")
            .bind(fd)
            .bind(chunk_size)
            .fetch_optional(&mut *tx)
            .await?;

        match chunk {
            Some(row) => {
                let data: Vec<u8> = row.try_get(0)?;
                if data.is_empty() {
                    // No more data to read
                    break;
                }
                bytes.extend_from_slice(&data);
            }
            _ => {
                break;
            }
        }
    }

    info!(
        "End of large object ({:?}) reached, result length: {}",
        large_object_oid,
        bytes.len()
    );

    let _ = sqlx::query("SELECT lo_close($1)")
        .bind(fd)
        .fetch_one(&mut *tx)
        .await?;

    Ok(bytes)
}

/// Write a large object to the database in chunks
pub async fn write_large_object_in_chunks(
    pool: &PgPool,
    data: &[u8],
    chunk_size: usize,
) -> anyhow::Result<Oid> {
    const INV_WRITE: i32 = 131072;

    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;

    // Create new LO
    let row = sqlx::query("SELECT lo_create(0)")
        .fetch_one(&mut *tx)
        .await?;
    let oid: Oid = row.try_get(0)?;

    info!("Created large object with Oid: {:?}", oid);

    // Open LO for writing
    let row = sqlx::query("SELECT lo_open($1, $2)")
        .bind(oid)
        .bind(INV_WRITE)
        .fetch_one(&mut *tx)
        .await?;
    let fd: i32 = row.try_get(0)?;

    info!(
        "Large Object oid: {:?}, fd: {}, chunk size: {}",
        oid, fd, chunk_size
    );

    // Write chunks
    for chunk in data.chunks(chunk_size) {
        sqlx::query("SELECT lowrite($1, $2)")
            .bind(fd)
            .bind(chunk)
            .execute(&mut *tx)
            .await?;
    }

    info!(
        "End of large object ({:?}) reached, result length: {}",
        oid,
        data.len()
    );

    // Close LO
    let _ = sqlx::query("SELECT lo_close($1)")
        .bind(fd)
        .fetch_one(&mut *tx)
        .await?;

    tx.commit().await?;
    Ok(oid)
}
