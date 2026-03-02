use crate::utils::safe_deserialize_key;
use bytesize::ByteSize;
use sqlx::{
    postgres::{types::Oid, PgRow},
    PgPool, Row,
};
use std::{num::NonZeroUsize, ops::DerefMut, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;

pub type DbKeyId = Vec<u8>;

#[derive(Clone)]
pub struct DbKeyCache {
    cache: Arc<RwLock<lru::LruCache<DbKeyId, DbKey>>>,
}

impl DbKeyCache {
    pub fn new(capacity: usize) -> anyhow::Result<Self> {
        let capacity = NonZeroUsize::new(capacity)
            .ok_or_else(|| anyhow::anyhow!("Cache capacity must be greater than zero"))?;
        Ok(Self {
            cache: Arc::new(RwLock::new(lru::LruCache::new(capacity))),
        })
    }

    pub async fn fetch<'a, T>(&self, db_key_id: &DbKeyId, executor: T) -> anyhow::Result<DbKey>
    where
        T: sqlx::PgExecutor<'a> + Copy,
    {
        // try getting from cache until it succeeds with populating cache
        loop {
            {
                let mut w = self.cache.write().await;
                if let Some(key) = w.get(db_key_id) {
                    return Ok(key.clone());
                }
            }
            self.populate(vec![db_key_id.clone()], executor).await?;
        }
    }

    /// Fetches the latest key by sequence_number.
    pub async fn fetch_latest<'a, T>(&self, executor: T) -> anyhow::Result<DbKey>
    where
        T: sqlx::PgExecutor<'a>,
    {
        let row = sqlx::query(
            "SELECT key_id, sequence_number, pks_key, sks_key, cks_key FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;

        let key_id: DbKeyId = row.try_get("key_id")?;
        let sequence_number: i64 = row.try_get("sequence_number")?;

        // Check if already in cache
        {
            let mut cache = self.cache.write().await;
            if let Some(key) = cache.get(&key_id) {
                return Ok(key.clone());
            }
        }

        // Not in cache, deserialize and cache it
        let pks_key: Vec<u8> = row.try_get("pks_key")?;
        let sks_key: Vec<u8> = row.try_get("sks_key")?;
        let cks_key: Option<Vec<u8>> = row.try_get("cks_key")?;

        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
        let cks: Option<tfhe::ClientKey> = cks_key
            .as_ref()
            .map(|k| safe_deserialize_key(k))
            .transpose()?;

        let result;
        #[cfg(not(feature = "gpu"))]
        {
            let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;

            result = DbKey {
                key_id: key_id.clone(),
                sequence_number,
                sks,
                pks,
                cks,
            }
        }
        #[cfg(feature = "gpu")]
        {
            let num_gpus = get_number_of_gpus() as u64;
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)?;

            result = DbKey {
                key_id: key_id.clone(),
                sequence_number,
                sks: csks.clone().decompress(),
                csks: csks.clone(),
                #[cfg(feature = "latency")]
                gpu_sks: vec![csks.decompress_to_gpu()],
                #[cfg(not(feature = "latency"))]
                gpu_sks: (0..num_gpus)
                    .map(|i| csks.decompress_to_specific_gpu(tfhe::GpuIndex::new(i as u32)))
                    .collect::<Vec<_>>(),
                pks,
                cks,
            };
        }

        // Insert into cache
        {
            let mut cache = self.cache.write().await;
            cache.put(key_id.clone(), result.clone());
        }

        info!(
            "Latest key cached: key_id={:?}, seq={}",
            hex::encode(&key_id),
            sequence_number
        );
        Ok(result)
    }

    pub async fn populate<'a, T>(
        &self,
        db_key_ids_to_query: Vec<DbKeyId>,
        executor: T,
    ) -> anyhow::Result<()>
    where
        T: sqlx::PgExecutor<'a>,
    {
        if !db_key_ids_to_query.is_empty() {
            let mut key_cache = self.cache.write().await;
            if db_key_ids_to_query
                .iter()
                .all(|id| key_cache.get(id).is_some())
            {
                return Ok(());
            }

            tracing::info!(
                message = "query keys",
                db_key_ids_to_query = format!("{:?}", db_key_ids_to_query),
            );

            let keys = Self::query_db_keys(Some(db_key_ids_to_query.clone()), executor).await?;
            if keys.is_empty() {
                anyhow::bail!(
                    "No keys found for {:?}; database may be corrupt",
                    db_key_ids_to_query
                );
            }

            for key in keys {
                key_cache.put(key.key_id.clone(), key);
            }
        }

        Ok(())
    }

    /// If `db_key_ids_to_query` is `None`, fetch all keys from the database.
    /// Else, fetch only the keys with the specified IDs.
    async fn query_db_keys<'a, T>(
        db_key_ids_to_query: Option<Vec<DbKeyId>>,
        conn: T,
    ) -> anyhow::Result<Vec<DbKey>>
    where
        T: sqlx::PgExecutor<'a>,
    {
        let rows = if let Some(ref ids) = db_key_ids_to_query {
            sqlx::query(
                "SELECT key_id, sequence_number, pks_key, sks_key, cks_key FROM keys WHERE key_id = ANY($1)",
            )
            .bind(ids)
            .fetch_all(conn)
            .await?
        } else {
            sqlx::query("SELECT key_id, sequence_number, pks_key, sks_key, cks_key FROM keys")
                .fetch_all(conn)
                .await?
        };

        let mut res = Vec::with_capacity(rows.len());

        for row in rows {
            let key_id = row.try_get("key_id")?;
            let sequence_number: i64 = row.try_get("sequence_number")?;
            let pks_key: Vec<u8> = row.try_get("pks_key")?;
            let sks_key: Vec<u8> = row.try_get("sks_key")?;
            let cks_key: Option<Vec<u8>> = row.try_get("cks_key")?;

            let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
            let cks: Option<tfhe::ClientKey> = cks_key
                .as_ref()
                .map(|k| safe_deserialize_key(k))
                .transpose()?;

            #[cfg(not(feature = "gpu"))]
            {
                let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;

                res.push(DbKey {
                    key_id,
                    sequence_number,
                    sks,
                    pks,
                    cks,
                });
            }
            #[cfg(feature = "gpu")]
            {
                let num_gpus = get_number_of_gpus() as u64;
                let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)?;

                res.push(DbKey {
                    key_id,
                    sequence_number,
                    sks: csks.clone().decompress(),
                    csks: csks.clone(),
                    #[cfg(feature = "latency")]
                    gpu_sks: vec![csks.decompress_to_gpu()],
                    #[cfg(not(feature = "latency"))]
                    gpu_sks: (0..num_gpus)
                        .map(|i| csks.decompress_to_specific_gpu(tfhe::GpuIndex::new(i as u32)))
                        .collect::<Vec<_>>(),
                    pks,
                    cks,
                });
            }
        }

        Ok(res)
    }
}

#[derive(Clone)]
pub struct DbKey {
    pub key_id: DbKeyId,
    pub sequence_number: i64,

    pub sks: tfhe::ServerKey,

    #[cfg(feature = "gpu")]
    pub csks: tfhe::CompressedServerKey,
    #[cfg(feature = "gpu")]
    pub gpu_sks: Vec<tfhe::CudaServerKey>,

    pub pks: tfhe::CompactPublicKey,

    pub cks: Option<tfhe::ClientKey>,
}

const CHUNK_SIZE: i32 = 64 * 1024; // 64KiB

pub async fn read_keys_from_large_object_by_key_id_gw(
    pool: &PgPool,
    key_id_gw: DbKeyId,
    keys_column_name: &str,
    capacity: usize,
) -> anyhow::Result<Vec<u8>> {
    let query = format!("SELECT {} FROM keys WHERE key_id_gw = $1", keys_column_name);

    let row: PgRow = sqlx::query(&query).bind(key_id_gw).fetch_one(pool).await?;

    let oid: Oid = row.try_get(0)?;
    info!("Retrieved oid: {:?}, column: {}", oid, keys_column_name);

    read_large_object_in_chunks(pool, oid, CHUNK_SIZE, capacity).await
}

// Read a large object by Oid from the database in chunks
async fn read_large_object_in_chunks(
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

    let mut timestamp = std::time::Instant::now();
    let started_at = std::time::Instant::now();

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

        // Log progress every 10 seconds
        if timestamp.elapsed().as_secs() > 10 {
            // calculate the bandwidth of the read operation
            let elapsed = started_at.elapsed().as_secs();
            let bandwidth = if elapsed > 0 {
                bytes.len() as u64 / elapsed
            } else {
                bytes.len() as u64
            };

            info!(
                "Read {} bytes so far from large object (Oid: {:?}), bandwidth: {}/s",
                ByteSize::b(bytes.len() as u64),
                large_object_oid,
                ByteSize::b(bandwidth)
            );

            timestamp = std::time::Instant::now();
        }
    }

    info!(
        "End of large object ({:?}) reached, result length: {}, elapsed: {}",
        large_object_oid,
        ByteSize::b(bytes.len() as u64),
        started_at.elapsed().as_secs()
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
    let mut tx: sqlx::Transaction<'_, sqlx::Postgres> = pool.begin().await?;
    let oid = write_large_object_in_chunks_tx(&mut tx, data, chunk_size).await?;
    tx.commit().await?;
    Ok(oid)
}

pub async fn write_large_object_in_chunks_tx(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    data: &[u8],
    chunk_size: usize,
) -> anyhow::Result<Oid> {
    const INV_WRITE: i32 = 131072;

    // Create new LO
    let row = sqlx::query("SELECT lo_create(0)")
        .fetch_one(tx.deref_mut())
        .await?;
    let oid: Oid = row.try_get(0)?;

    info!("Created large object with Oid: {:?}", oid);

    // Open LO for writing
    let row = sqlx::query("SELECT lo_open($1, $2)")
        .bind(oid)
        .bind(INV_WRITE)
        .fetch_one(tx.deref_mut())
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
            .execute(tx.deref_mut())
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
        .fetch_one(tx.deref_mut())
        .await?;

    Ok(oid)
}
