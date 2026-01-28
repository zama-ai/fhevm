use crate::utils::safe_deserialize_key;
use bytesize::ByteSize;
use sqlx::{
    postgres::{types::Oid, PgRow},
    PgPool, Row,
};
use std::ops::DerefMut;
use tracing::info;

pub type DbKeyId = Vec<u8>;

#[derive(Clone)]
pub struct DbKey {
    pub key_id: DbKeyId,
    pub sequence_number: i64,

    #[cfg(not(feature = "gpu"))]
    pub sks: tfhe::ServerKey,

    #[cfg(feature = "gpu")]
    pub sks: tfhe::CudaServerKey,

    pub pks: tfhe::CompactPublicKey,
}

pub async fn fetch_db_key<'a, T>(
    db_key_id: &DbKeyId,
    pool: T,
    db_keys_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<DbKeyId, DbKey>>>,
) -> Result<DbKey, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a> + Copy,
{
    // try getting from cache until it succeeds with populating cache
    loop {
        {
            let mut w = db_keys_cache.write().await;
            if let Some(key) = w.get(db_key_id) {
                return Ok(key.clone());
            }
        }
        populate_cache_with_db_keys(vec![db_key_id.clone()], pool, db_keys_cache).await?;
    }
}

/// Fetches the latest key by sequence_number and ensures it's in the cache.
/// Returns the key result.
pub async fn fetch_latest_db_key<'a, T>(
    pool: T,
    db_keys_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<DbKeyId, DbKey>>>,
) -> Result<DbKey, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let row = sqlx::query(
        "SELECT key_id, sequence_number, pks_key, sks_key FROM keys ORDER BY sequence_number DESC LIMIT 1",
    )
    .fetch_optional(pool)
    .await?
    .ok_or("No keys found in database")?;

    let key_id: DbKeyId = row.try_get("key_id")?;
    let sequence_number: i64 = row.try_get("sequence_number")?;

    // Check if already in cache
    {
        let mut cache = db_keys_cache.write().await;
        if let Some(key) = cache.get(&key_id) {
            return Ok(key.clone());
        }
    }

    // Not in cache, deserialize and cache it
    let pks_key: Vec<u8> = row.try_get("pks_key")?;
    let sks_key: Vec<u8> = row.try_get("sks_key")?;

    #[cfg(not(feature = "gpu"))]
    let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;
    #[cfg(feature = "gpu")]
    let sks = {
        let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)?;
        csks.decompress_to_gpu()
    };
    let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;

    let result = DbKey {
        key_id: key_id.clone(),
        sequence_number,
        sks: sks.clone(),
        pks: pks.clone(),
    };

    // Insert into cache
    {
        let mut cache = db_keys_cache.write().await;
        cache.put(
            key_id.clone(),
            DbKey {
                key_id: key_id.clone(),
                sequence_number,
                sks,
                pks,
            },
        );
    }

    info!(
        "Latest key cached: key_id={:?}, seq={}",
        hex::encode(&key_id),
        sequence_number
    );
    Ok(result)
}

/// If `db_key_ids_to_query` is `None`, fetch all keys from the database.
/// Else, fetch only the keys with the specified IDs.
pub async fn query_db_keys<'a, T>(
    db_key_ids_to_query: Option<Vec<DbKeyId>>,
    conn: T,
) -> Result<Vec<DbKey>, Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    let rows = if let Some(ref ids) = db_key_ids_to_query {
        sqlx::query(
            "SELECT key_id, sequence_number, pks_key, sks_key FROM keys WHERE key_id = ANY($1)",
        )
        .bind(ids)
        .fetch_all(conn)
        .await?
    } else {
        sqlx::query("SELECT key_id, sequence_number, pks_key, sks_key FROM keys")
            .fetch_all(conn)
            .await?
    };

    let mut res = Vec::with_capacity(rows.len());

    for row in rows {
        let key_id = row.try_get("key_id")?;
        let sequence_number: i64 = row.try_get("sequence_number")?;
        let pks_key: Vec<u8> = row.try_get("pks_key")?;
        let sks_key: Vec<u8> = row.try_get("sks_key")?;

        // Deserialize binary keys properly
        #[cfg(not(feature = "gpu"))]
        let sks: tfhe::ServerKey = safe_deserialize_key(&sks_key)?;
        #[cfg(feature = "gpu")]
        let sks = {
            let csks: tfhe::CompressedServerKey = safe_deserialize_key(&sks_key)?;
            csks.decompress_to_gpu()
        };
        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;

        res.push(DbKey {
            key_id,
            sequence_number,
            sks,
            pks,
        });
    }

    Ok(res)
}

async fn populate_cache_with_db_keys<'a, T>(
    db_key_ids_to_query: Vec<DbKeyId>,
    conn: T,
    db_keys_cache: &std::sync::Arc<tokio::sync::RwLock<lru::LruCache<DbKeyId, DbKey>>>,
) -> Result<(), Box<dyn std::error::Error + Send + Sync>>
where
    T: sqlx::PgExecutor<'a>,
{
    if !db_key_ids_to_query.is_empty() {
        let mut key_cache = db_keys_cache.write().await;
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

        let keys = query_db_keys(Some(db_key_ids_to_query), conn).await?;

        assert!(
            !keys.is_empty(),
            "We should have keys here, otherwise our database is corrupt"
        );

        for key in keys {
            key_cache.put(key.key_id.clone(), key);
        }
    }

    Ok(())
}

const CHUNK_SIZE: i32 = 64 * 1024; // 64KiB
pub async fn read_keys_from_large_object(
    pool: &PgPool,
    db_key_id: DbKeyId,
    keys_column_name: &str,
    capacity: usize,
) -> anyhow::Result<Vec<u8>> {
    let query = format!("SELECT {} FROM keys WHERE key_id = $1", keys_column_name);

    // Read the Oid of the large object
    let row: PgRow = sqlx::query(&query).bind(db_key_id).fetch_one(pool).await?;

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
