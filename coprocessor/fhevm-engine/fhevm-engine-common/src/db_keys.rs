use crate::key_material_policy::{KeyMaterialKind, KeyMaterialUnavailable};
use crate::utils::safe_deserialize_key;
use bytesize::ByteSize;
use sqlx::{
    postgres::{types::Oid, PgRow},
    PgConnection, PgPool, Row,
};
use std::{num::NonZeroUsize, ops::DerefMut, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

#[cfg(all(feature = "gpu", not(feature = "latency")))]
use tfhe::core_crypto::gpu::get_number_of_gpus;
use tfhe::xof_key_set::CompressedXofKeySet;

pub type DbKeyId = Vec<u8>;

/// Single row shape for both CPU and GPU builds. `server_key_blob` is
/// COALESCE'd in SQL to take `compressed_xof_keyset` when present and
/// fall back to legacy `sks_key` otherwise, so we cross the wire with
/// exactly one BYTEA per row (~400 MB XOF or ~329 MB legacy) instead
/// of both. `is_xof` tells the deserializer which encoding came back.
///
/// Single query shape across CPU and GPU keeps sqlx-prepare cacheable
/// without a CUDA toolchain.
struct DbKeyRow {
    key_id: DbKeyId,
    sequence_number: i64,
    pks_key: Vec<u8>,
    server_key_blob: Vec<u8>,
    is_xof: bool,
    cks_key: Option<Vec<u8>>,
}

/// Cache entries are keyed by `(key_id, material_kind)` so both byte
/// representations of the same key can coexist during the RFC-029
/// cutover window. The kind is an opaque second dimension here; all
/// cutover logic lives in `key_material_policy`.
#[derive(Clone)]
pub struct DbKeyCache {
    cache: Arc<RwLock<lru::LruCache<(DbKeyId, KeyMaterialKind), DbKey>>>,
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
                // Default (no-cutover) reads mirror the SQL COALESCE
                // preference: compressed material first, legacy second.
                for kind in [KeyMaterialKind::CompressedXof, KeyMaterialKind::Legacy] {
                    if let Some(key) = w.get(&(db_key_id.clone(), kind)) {
                        return Ok(key.clone());
                    }
                }
            }
            self.populate(vec![db_key_id.clone()], executor).await?;
        }
    }

    /// Fetches the latest key by sequence_number, loading whichever
    /// material the row carries (compressed preferred). Pre-cutover
    /// behavior; workers under an active cutover use
    /// [`Self::fetch_latest_pinned`] instead.
    pub async fn fetch_latest(&self, executor: &mut PgConnection) -> anyhow::Result<DbKey> {
        let row = sqlx::query!(
            "SELECT key_id, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;

        let key_id: DbKeyId = row.key_id;
        let sequence_number = row.sequence_number;

        // Check if already in cache under either material kind,
        // compressed first to mirror the SQL COALESCE preference. The
        // light query above deliberately touches only metadata columns.
        {
            let mut cache = self.cache.write().await;
            for kind in [KeyMaterialKind::CompressedXof, KeyMaterialKind::Legacy] {
                if let Some(key) = cache.get(&(key_id.clone(), kind)) {
                    if key.sequence_number == sequence_number {
                        return Ok(key.clone());
                    }
                }
            }
        }

        // Only fetch the heavy key blobs when the latest key is not already cached.
        let row = sqlx::query_as!(
            DbKeyRow,
            "SELECT key_id, sequence_number, pks_key, \
             COALESCE(compressed_xof_keyset, sks_key) AS \"server_key_blob!\", \
             (compressed_xof_keyset IS NOT NULL) AS \"is_xof!\", \
             cks_key \
             FROM keys WHERE sequence_number = $1",
            sequence_number
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("Latest key disappeared from database"))?;
        let result = Self::deserialize_db_key_row(row)?;
        self.cache_and_log(&result).await;
        Ok(result)
    }

    /// Fetches the latest key by sequence_number with the material
    /// kind pinned by the RFC-029 cutover policy. Loads exactly the
    /// requested material; if the row does not carry it yet, returns
    /// [`KeyMaterialUnavailable`] (retryable) — never the other kind.
    pub async fn fetch_latest_pinned(
        &self,
        executor: &mut PgConnection,
        kind: KeyMaterialKind,
    ) -> anyhow::Result<DbKey> {
        let row = sqlx::query!(
            "SELECT key_id, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;

        let key_id: DbKeyId = row.key_id;
        let sequence_number = row.sequence_number;

        {
            let mut cache = self.cache.write().await;
            if let Some(key) = cache.get(&(key_id.clone(), kind)) {
                if key.sequence_number == sequence_number {
                    return Ok(key.clone());
                }
            }
        }

        let row = match kind {
            KeyMaterialKind::CompressedXof => {
                sqlx::query_as!(
                    DbKeyRow,
                    "SELECT key_id, sequence_number, pks_key, \
                     compressed_xof_keyset AS \"server_key_blob!\", \
                     TRUE AS \"is_xof!\", cks_key \
                     FROM keys WHERE sequence_number = $1 \
                     AND compressed_xof_keyset IS NOT NULL",
                    sequence_number
                )
                .fetch_optional(&mut *executor)
                .await?
            }
            KeyMaterialKind::Legacy => {
                sqlx::query_as!(
                    DbKeyRow,
                    "SELECT key_id, sequence_number, pks_key, \
                     sks_key AS \"server_key_blob!\", \
                     FALSE AS \"is_xof!\", cks_key \
                     FROM keys WHERE sequence_number = $1 \
                     AND sks_key IS NOT NULL",
                    sequence_number
                )
                .fetch_optional(&mut *executor)
                .await?
            }
        };
        let row = row.ok_or_else(|| KeyMaterialUnavailable {
            key_id: hex::encode(&key_id),
            kind,
        })?;

        let result = Self::deserialize_db_key_row(row)?;
        self.cache_and_log(&result).await;
        Ok(result)
    }

    async fn cache_and_log(&self, result: &DbKey) {
        {
            let mut cache = self.cache.write().await;
            cache.put(
                (result.key_id.clone(), result.material_kind),
                result.clone(),
            );
        }
        info!(
            "Key cached: key_id={:?}, seq={}, kind={:?}",
            hex::encode(&result.key_id),
            result.sequence_number,
            result.material_kind
        );
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
            if db_key_ids_to_query.iter().all(|id| {
                key_cache
                    .get(&(id.clone(), KeyMaterialKind::CompressedXof))
                    .is_some()
                    || key_cache
                        .get(&(id.clone(), KeyMaterialKind::Legacy))
                        .is_some()
            }) {
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
                key_cache.put((key.key_id.clone(), key.material_kind), key);
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
            sqlx::query_as!(
                DbKeyRow,
                "SELECT key_id, sequence_number, pks_key, \
                 COALESCE(compressed_xof_keyset, sks_key) AS \"server_key_blob!\", \
                 (compressed_xof_keyset IS NOT NULL) AS \"is_xof!\", \
                 cks_key \
                 FROM keys WHERE key_id = ANY($1)",
                ids
            )
            .fetch_all(conn)
            .await?
        } else {
            sqlx::query_as!(
                DbKeyRow,
                "SELECT key_id, sequence_number, pks_key, \
                 COALESCE(compressed_xof_keyset, sks_key) AS \"server_key_blob!\", \
                 (compressed_xof_keyset IS NOT NULL) AS \"is_xof!\", \
                 cks_key \
                 FROM keys"
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
            server_key_blob,
            is_xof,
            cks_key,
        } = row;
        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
        let cks: Option<tfhe::ClientKey> = cks_key
            .as_ref()
            .map(|k| safe_deserialize_key(k))
            .transpose()?;
        let material_kind = if is_xof {
            KeyMaterialKind::CompressedXof
        } else {
            KeyMaterialKind::Legacy
        };

        #[cfg(not(feature = "gpu"))]
        {
            // Prefer the CompressedXofKeySet when present so CPU and
            // GPU readers share a single source of truth. Decompress
            // the whole keyset in one pass (the XOF stream is shared
            // across subkeys, so taking the embedded CSK out and
            // decompressing it alone would skip the public-key portion
            // of the stream), then strip NS material in memory to
            // match the legacy sks_key shape tfhe-worker expects.
            //
            // Legacy sks_key fallback is used only for rows that
            // predate XOF keygen (compressed_xof_keyset IS NULL).
            let sks: tfhe::ServerKey = if is_xof {
                let kxs: CompressedXofKeySet =
                    crate::utils::safe_deserialize_sns_key(&server_key_blob).map_err(|err| {
                        anyhow::anyhow!(
                            "failed to deserialize CompressedXofKeySet from compressed_xof_keyset: {err}"
                        )
                    })?;
                let (_xof_pks, server_key) = kxs
                    .decompress()
                    .map_err(|err| {
                        anyhow::anyhow!(
                            "failed to decompress CompressedXofKeySet to ServerKey: {err}"
                        )
                    })?
                    .into_raw_parts();
                strip_ns_from_server_key(server_key)
            } else {
                safe_deserialize_key(&server_key_blob)?
            };

            Ok(DbKey {
                key_id,
                sequence_number,
                material_kind,
                sks,
                pks,
                cks,
            })
        }
        #[cfg(feature = "gpu")]
        {
            if !is_xof {
                anyhow::bail!(
                    "GPU coprocessor requires keys.compressed_xof_keyset to be populated; \
                     rotate kms-core to publish CompressedXofKeySet so the host-listener can ingest it"
                );
            }

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

            #[cfg(feature = "latency")]
            let gpu_sks = vec![
                kxs.decompress_to_gpu()
                    .map_err(|err| {
                        anyhow::anyhow!(
                            "failed to decompress CompressedXofKeySet to CudaServerKey: {err}"
                        )
                    })?
                    .into_raw_parts()
                    .1,
            ];
            #[cfg(not(feature = "latency"))]
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
                material_kind,
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

    /// Which byte representation this entry was loaded from.
    pub material_kind: KeyMaterialKind,

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

/// Encoding of the server-key blob returned by
/// [`read_compressed_xof_keyset_by_sequence_number_with_fallback`].
///
/// `CompressedXof` blobs are `tfhe::xof_key_set::CompressedXofKeySet` —
/// the whole keyset must be deserialized in one pass to keep the XOF
/// state consistent across subkeys. `Legacy` blobs are plain
/// `tfhe::ServerKey` and can be deserialized directly. Reflects which
/// column in the `keys` table held the bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum CompressedXofKeysetEncoding {
    CompressedXof,
    Legacy,
}

/// Reads the server-key blob for `sequence_number`, preferring the
/// `compressed_xof_keyset` BYTEA column (the raw kms-core
/// CompressedXofKeySet) when present and falling back to the
/// decompressed `sns_pk` LOB for rows published before XOF keygen.
pub async fn read_compressed_xof_keyset_by_sequence_number_with_fallback(
    pool: &PgPool,
    sequence_number: i64,
    legacy_capacity: usize,
) -> anyhow::Result<(Vec<u8>, CompressedXofKeysetEncoding)> {
    let row = sqlx::query!(
        "SELECT compressed_xof_keyset, sns_pk FROM keys WHERE sequence_number = $1",
        sequence_number
    )
    .fetch_one(pool)
    .await?;

    if let Some(bytes) = row.compressed_xof_keyset {
        info!(
            bytes_len = bytes.len(),
            "Retrieved compressed_xof_keyset BYTEA"
        );
        return Ok((bytes, CompressedXofKeysetEncoding::CompressedXof));
    }

    // The activation upsert in host-listener::database gates on
    // key_content_sks_key IS NOT NULL but doesn't explicitly require
    // either server-key column; surface a clear error if a keys row
    // ever makes it here with both NULL rather than letting sqlx
    // produce an opaque ColumnDecode failure.
    let legacy = row.sns_pk.ok_or_else(|| {
        anyhow::anyhow!(
            "keys row for sequence_number has neither compressed_xof_keyset nor sns_pk populated"
        )
    })?;
    info!("Retrieved legacy sns_pk oid: {:?}", legacy);
    let bytes = read_large_object_in_chunks(pool, legacy, CHUNK_SIZE, legacy_capacity).await?;
    Ok((bytes, CompressedXofKeysetEncoding::Legacy))
}

/// Loads the RFC-029 compressed-key cutover schedule (the migration is
/// one-time: the contract enforces at most one schedule per key and
/// only one key is live). If several rows ever exist, the first by key
/// order is used — deterministic across coprocessors either way.
pub async fn load_active_compressed_key_cutover(
    conn: &mut PgConnection,
) -> anyhow::Result<Option<crate::key_material_policy::CompressedKeyCutover>> {
    let Some(key_id) =
        sqlx::query_scalar!("SELECT key_id FROM compressed_key_cutover ORDER BY key_id LIMIT 1")
            .fetch_optional(&mut *conn)
            .await?
    else {
        return Ok(None);
    };
    load_compressed_key_cutover(conn, &key_id).await
}

/// Loads the RFC-029 compressed-key cutover schedule for `key_id`, if
/// one has been ingested. `None` means no cutover is scheduled and all
/// key loading keeps its pre-feature behavior. The schedule is
/// immutable once present, so callers may cache a `Some` result.
pub async fn load_compressed_key_cutover(
    conn: &mut PgConnection,
    key_id: &[u8],
) -> anyhow::Result<Option<crate::key_material_policy::CompressedKeyCutover>> {
    let Some(gateway_cutover_block) = sqlx::query_scalar!(
        "SELECT gateway_cutover_block FROM compressed_key_cutover WHERE key_id = $1",
        key_id
    )
    .fetch_optional(&mut *conn)
    .await?
    else {
        return Ok(None);
    };

    let hosts = sqlx::query!(
        "SELECT chain_id, cutover_block FROM compressed_key_cutover_hosts WHERE key_id = $1",
        key_id
    )
    .fetch_all(&mut *conn)
    .await?;

    Ok(Some(crate::key_material_policy::CompressedKeyCutover {
        host_cutover_blocks: hosts
            .into_iter()
            .map(|row| (row.chain_id as u64, row.cutover_block as u64))
            .collect(),
        gateway_cutover_block: gateway_cutover_block as u64,
    }))
}

/// Reads the SnS server-key blob for `sequence_number` with the
/// material kind pinned by the RFC-029 cutover policy (SnS tasks pin
/// their source ciphertext's kind). `Legacy` reads the decompressed
/// `sns_pk` LOB; `CompressedXof` reads the `compressed_xof_keyset`
/// BYTEA. A missing column for the requested kind returns
/// [`KeyMaterialUnavailable`] (retryable) — never the other kind.
pub async fn read_sns_key_blob_by_sequence_number_pinned(
    pool: &PgPool,
    sequence_number: i64,
    kind: KeyMaterialKind,
    legacy_capacity: usize,
) -> anyhow::Result<(Vec<u8>, CompressedXofKeysetEncoding)> {
    let row = sqlx::query!(
        "SELECT key_id, compressed_xof_keyset, sns_pk FROM keys WHERE sequence_number = $1",
        sequence_number
    )
    .fetch_one(pool)
    .await?;

    let unavailable = || KeyMaterialUnavailable {
        key_id: hex::encode(&row.key_id),
        kind,
    };

    match kind {
        KeyMaterialKind::CompressedXof => {
            let bytes = row.compressed_xof_keyset.ok_or_else(unavailable)?;
            info!(
                bytes_len = bytes.len(),
                "Retrieved pinned compressed_xof_keyset BYTEA"
            );
            Ok((bytes, CompressedXofKeysetEncoding::CompressedXof))
        }
        KeyMaterialKind::Legacy => {
            let legacy = row.sns_pk.ok_or_else(unavailable)?;
            info!("Retrieved pinned legacy sns_pk oid: {:?}", legacy);
            let bytes =
                read_large_object_in_chunks(pool, legacy, CHUNK_SIZE, legacy_capacity).await?;
            Ok((bytes, CompressedXofKeysetEncoding::Legacy))
        }
    }
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
