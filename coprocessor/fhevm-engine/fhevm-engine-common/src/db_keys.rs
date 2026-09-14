use crate::utils::safe_deserialize_key;
use bytesize::ByteSize;
use sqlx::{
    postgres::{types::Oid, PgRow},
    PgConnection, PgPool, Row,
};
use std::{num::NonZeroUsize, ops::DerefMut, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

#[cfg(feature = "gpu")]
use tfhe::core_crypto::gpu::get_number_of_gpus;
use tfhe::xof_key_set::CompressedXofKeySet;

pub type DbKeyId = Vec<u8>;

pub const FORCE_LEGACY_SERVER_KEY_ENV: &str = "FORCE_LEGACY_SERVER_KEY";

pub fn reject_legacy_server_key_override() -> anyhow::Result<()> {
    let force_legacy = match std::env::var(FORCE_LEGACY_SERVER_KEY_ENV) {
        Ok(value) => value.parse::<bool>().map_err(|_| {
            anyhow::anyhow!(
                "invalid {FORCE_LEGACY_SERVER_KEY_ENV}={value:?}; expected true or false"
            )
        }),
        Err(std::env::VarError::NotPresent) => Ok(false),
        Err(error) => Err(error.into()),
    }?;
    if force_legacy {
        anyhow::bail!(
            "{FORCE_LEGACY_SERVER_KEY_ENV}=true is not supported after compressed-key adoption"
        );
    }
    Ok(())
}

struct DbKeyRow {
    key_id: DbKeyId,
    sequence_number: i64,
    pks_key: Vec<u8>,
    compressed_xof_keyset: Option<Vec<u8>>,
    cks_key: Option<Vec<u8>>,
}

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

    pub fn new_from_env(capacity: usize) -> anyhow::Result<Self> {
        reject_legacy_server_key_override()?;
        Self::new(capacity)
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
    pub async fn fetch_latest(&self, executor: &mut PgConnection) -> anyhow::Result<DbKey> {
        let row = sqlx::query!(
            "SELECT key_id, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;

        let key_id: DbKeyId = row.key_id;
        let sequence_number = row.sequence_number;

        // Check if already in cache
        {
            let mut cache = self.cache.write().await;
            if let Some(key) = cache.get(&key_id) {
                if key.sequence_number == sequence_number {
                    return Ok(key.clone());
                }
            }
        }

        // Only fetch the heavy key blobs when the latest key is not already cached.
        let row = sqlx::query_as!(
            DbKeyRow,
            r#"SELECT key_id, sequence_number, pks_key, compressed_xof_keyset,
               cks_key
               FROM keys WHERE sequence_number = $1"#,
            sequence_number,
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Latest key disappeared from database"))?;
        let result = Self::deserialize_db_key_row(row)?;

        // Insert into cache
        {
            let mut cache = self.cache.write().await;
            cache.put(result.key_id.clone(), result.clone());
        }

        info!(
            "Latest key cached: key_id={:?}, seq={}",
            hex::encode(&result.key_id),
            result.sequence_number
        );
        Ok(result)
    }

    pub async fn fetch_latest_from_pool(&self, pool: &PgPool) -> anyhow::Result<DbKey> {
        let mut conn = pool.acquire().await?;
        self.fetch_latest(&mut conn).await
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

            let keys = self
                .query_db_keys(Some(db_key_ids_to_query.clone()), executor)
                .await?;
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
        &self,
        db_key_ids_to_query: Option<Vec<DbKeyId>>,
        conn: T,
    ) -> anyhow::Result<Vec<DbKey>>
    where
        T: sqlx::PgExecutor<'a>,
    {
        let rows = if let Some(ref ids) = db_key_ids_to_query {
            sqlx::query_as!(
                DbKeyRow,
                r#"SELECT key_id, sequence_number, pks_key, compressed_xof_keyset,
                   cks_key
                   FROM keys WHERE key_id = ANY($1)"#,
                ids,
            )
            .fetch_all(conn)
            .await?
        } else {
            sqlx::query_as!(
                DbKeyRow,
                r#"SELECT key_id, sequence_number, pks_key, compressed_xof_keyset, cks_key
                   FROM keys"#,
            )
            .fetch_all(conn)
            .await?
        };

        let mut res = Vec::with_capacity(rows.len());

        for row in rows {
            res.push(Self::deserialize_db_key_row(row)?);
        }

        Ok(res)
    }

    fn deserialize_db_key_row(row: DbKeyRow) -> anyhow::Result<DbKey> {
        let DbKeyRow {
            key_id,
            sequence_number,
            pks_key,
            compressed_xof_keyset,
            cks_key,
        } = row;
        let server_key_blob = compressed_xof_keyset.ok_or_else(|| {
            anyhow::anyhow!(
                "compressed_xof_keyset is missing for key {}",
                hex::encode(&key_id)
            )
        })?;
        info!(
            server_key_representation = "compressed-xof",
            key_id = hex::encode(&key_id),
            sequence_number,
            "Loading server-key material"
        );
        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
        let cks: Option<tfhe::ClientKey> = cks_key
            .as_ref()
            .map(|k| safe_deserialize_key(k))
            .transpose()?;

        #[cfg(not(feature = "gpu"))]
        {
            let kxs: CompressedXofKeySet =
                crate::utils::safe_deserialize_sns_key(&server_key_blob).map_err(|err| {
                    anyhow::anyhow!(
                        "failed to deserialize CompressedXofKeySet from compressed_xof_keyset: {err}"
                    )
                })?;
            let (_xof_pks, server_key) = kxs
                .decompress()
                .map_err(|err| {
                    anyhow::anyhow!("failed to decompress CompressedXofKeySet to ServerKey: {err}")
                })?
                .into_raw_parts();
            let sks = strip_ns_from_server_key(server_key);

            Ok(DbKey {
                key_id,
                sequence_number,
                sks,
                pks,
                cks,
            })
        }
        #[cfg(feature = "gpu")]
        {
            // The whole CompressedXofKeySet must be decompressed before
            // we extract the server key. The XOF stream is shared across
            // subkeys, so taking the embedded CompressedServerKey out of
            // the wrapper and decompressing it alone would skip the
            // public-key portion of the stream.
            let kxs: CompressedXofKeySet =
                crate::utils::safe_deserialize_sns_key(&server_key_blob).map_err(|err| {
                    anyhow::anyhow!(
                        "failed to deserialize CompressedXofKeySet from compressed_xof_keyset: {err}"
                    )
                })?;
            let (_xof_pks, sks) = kxs
                .decompress()
                .map_err(|err| {
                    anyhow::anyhow!("failed to decompress CompressedXofKeySet to ServerKey: {err}")
                })?
                .into_raw_parts();

            let gpu_sks = {
                let num_gpus = get_number_of_gpus() as u64;
                (0..num_gpus)
                    .map(|i| {
                        kxs.decompress_to_specific_gpu(tfhe::GpuIndex::new(i as u32))
                            .map(|keyset| keyset.into_raw_parts().1)
                            .map_err(|err| {
                                anyhow::anyhow!(
                                    "failed to decompress CompressedXofKeySet to GPU {i}: {err}"
                                )
                            })
                    })
                    .collect::<Result<Vec<_>, _>>()?
            };

            Ok(DbKey {
                key_id,
                sequence_number,
                sks,
                gpu_sks,
                pks,
                cks,
            })
        }
    }
}

/// Returns the input `ServerKey` with noise-squashing material
/// removed. CPU readers don't use NS slots and carrying them ~triples
/// the per-key memory footprint, so we strip after the whole-keyset
/// XOF decompression (post-decompress is safe — the shared XOF stream
/// has already been consumed in order).
#[cfg(not(feature = "gpu"))]
fn strip_ns_from_server_key(server_key: tfhe::ServerKey) -> tfhe::ServerKey {
    let (
        sks,
        kskm,
        compression_key,
        decompression_key,
        _noise_squashing_key,
        _noise_squashing_compression_key,
        re_randomization_keyswitching_key,
        oprf_key,
        tag,
    ) = server_key.into_raw_parts();
    tfhe::ServerKey::from_raw_parts(
        sks,
        kskm,
        compression_key,
        decompression_key,
        None, // noise squashing key excluded
        None, // noise squashing compression key excluded
        re_randomization_keyswitching_key,
        oprf_key,
        tag,
    )
}

#[derive(Clone)]
pub struct DbKey {
    pub key_id: DbKeyId,
    pub sequence_number: i64,

    pub sks: tfhe::ServerKey,

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

pub async fn read_compressed_server_key_by_sequence_number(
    pool: &PgPool,
    sequence_number: i64,
) -> anyhow::Result<Vec<u8>> {
    let bytes = sqlx::query_scalar::<_, Option<Vec<u8>>>(
        "SELECT compressed_xof_keyset FROM keys WHERE sequence_number = $1",
    )
    .bind(sequence_number)
    .fetch_one(pool)
    .await?
    .ok_or_else(|| {
        anyhow::anyhow!("compressed_xof_keyset is missing for sequence_number {sequence_number}")
    })?;
    info!(
        bytes_len = bytes.len(),
        "Retrieved compressed_xof_keyset BYTEA"
    );
    Ok(bytes)
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
            let total_bytes = bytes.len() as u64;
            let bandwidth = total_bytes.checked_div(elapsed).unwrap_or(total_bytes);

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
