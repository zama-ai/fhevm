use crate::key_material_policy::{KeyMaterialSelection, MaterialVersion};
use crate::utils::safe_deserialize_key;
use bytesize::ByteSize;
use sqlx::{
    postgres::{types::Oid, PgRow},
    FromRow, PgConnection, PgPool, Row,
};
use std::{num::NonZeroUsize, ops::DerefMut, sync::Arc};
use tokio::sync::RwLock;
use tracing::info;

#[cfg(all(feature = "gpu", not(feature = "latency")))]
use tfhe::core_crypto::gpu::get_number_of_gpus;
use tfhe::xof_key_set::CompressedXofKeySet;

pub type DbKeyId = Vec<u8>;

/// Single row shape for both CPU and GPU builds. `stored_key_material` is
/// COALESCE'd in SQL to take `compressed_xof_keyset` when present and
/// fall back to legacy `sks_key` otherwise, so we cross the wire with
/// exactly one BYTEA per row (~400 MB XOF or ~329 MB legacy) instead
/// of both. `stored_as_xof` tells the deserializer which format came back.
///
/// Single query shape across CPU and GPU keeps sqlx-prepare cacheable
/// without a CUDA toolchain.
#[derive(FromRow)]
struct DbKeyRow {
    key_id: DbKeyId,
    sequence_number: i64,
    pks_key: Vec<u8>,
    stored_key_material: Vec<u8>,
    stored_as_xof: bool,
    cks_key: Option<Vec<u8>>,
}

#[derive(Clone)]
pub struct DbKeyCache {
    /// Internally split by persisted output label so the cutover boundary can
    /// hold both deserialized legacy and compressed material for the same key id.
    cache: Arc<RwLock<lru::LruCache<(DbKeyId, MaterialVersion), DbKey>>>,
}

impl DbKeyCache {
    pub fn new(capacity: usize) -> anyhow::Result<Self> {
        let capacity = NonZeroUsize::new(capacity)
            .ok_or_else(|| anyhow::anyhow!("Cache capacity must be greater than zero"))?;
        Ok(Self {
            cache: Arc::new(RwLock::new(lru::LruCache::new(capacity))),
        })
    }

    /// Fetches the latest key for normal startup reads. Native compressed rows
    /// (`compressed_key_state IS NULL`) use today's COALESCE path; rows
    /// participating in the migration force `sks_key` unless a cutover policy
    /// explicitly requests default material.
    pub async fn fetch_latest(&self, executor: &mut PgConnection) -> anyhow::Result<DbKey> {
        self.fetch_latest_default_material(executor).await
    }

    /// Fetches the latest key for normal startup/native reads.
    ///
    /// Native compressed rows use `COALESCE(compressed_xof_keyset, sks_key)`.
    /// Rows participating in the compressed-key migration force `sks_key` until
    /// an operation is explicitly selected for default material.
    pub async fn fetch_latest_default_material(
        &self,
        executor: &mut PgConnection,
    ) -> anyhow::Result<DbKey> {
        let row = sqlx::query_as::<_, DbKeyRow>(
            "SELECT key_id, sequence_number, pks_key, \
             CASE WHEN compressed_key_state IS NULL \
                  THEN COALESCE(compressed_xof_keyset, sks_key) \
                  ELSE sks_key END AS stored_key_material, \
             (compressed_key_state IS NULL AND compressed_xof_keyset IS NOT NULL) AS stored_as_xof, \
             cks_key \
             FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;

        Self::deserialize_db_key_row(row, MaterialVersion::LEGACY)
    }

    /// Fetches the latest worker key material for an execution group.
    pub async fn fetch_key_for(
        &self,
        selection: KeyMaterialSelection,
        executor: &mut PgConnection,
    ) -> anyhow::Result<DbKey> {
        self.fetch_key_with_label(selection.required_material_version(), executor)
            .await
    }

    /// Fetches the latest key by persisted RFC-029 output label.
    async fn fetch_key_with_label(
        &self,
        version: MaterialVersion,
        executor: &mut PgConnection,
    ) -> anyhow::Result<DbKey> {
        if version != MaterialVersion::LEGACY && version != MaterialVersion::MIGRATED_V1 {
            anyhow::bail!(
                "unsupported material version {}: only LEGACY (0) and MIGRATED_V1 (1) are defined",
                version.0
            );
        }
        // Light query first: identify the latest key row without pulling the
        // heavy key blobs, so a cache hit costs nothing.
        let row = sqlx::query!(
            "SELECT key_id, sequence_number FROM keys ORDER BY sequence_number DESC LIMIT 1",
        )
        .fetch_optional(&mut *executor)
        .await?
        .ok_or_else(|| anyhow::anyhow!("No keys found in database"))?;
        let key_id: DbKeyId = row.key_id;
        let sequence_number = row.sequence_number;

        let mut cache = self.cache.write().await;
        if let Some(key) = cache.get(&(key_id.clone(), version)) {
            if key.sequence_number == sequence_number {
                return Ok(key.clone());
            }
        }
        drop(cache);

        let row = if version == MaterialVersion::LEGACY {
            sqlx::query_as::<_, DbKeyRow>(
                "SELECT key_id, sequence_number, pks_key, \
                 sks_key AS stored_key_material, \
                 FALSE AS stored_as_xof, \
                 cks_key \
                 FROM keys WHERE sequence_number = $1",
            )
            .bind(sequence_number)
            .fetch_optional(&mut *executor)
            .await?
        } else {
            sqlx::query_as::<_, DbKeyRow>(
                "SELECT key_id, sequence_number, pks_key, \
                 compressed_xof_keyset AS stored_key_material, \
                 TRUE AS stored_as_xof, \
                 cks_key \
                 FROM keys WHERE sequence_number = $1 AND compressed_xof_keyset IS NOT NULL",
            )
            .bind(sequence_number)
            .fetch_optional(&mut *executor)
            .await?
        };
        let row = row.ok_or_else(|| {
            anyhow::anyhow!(
                "key material for version {} is not available yet (halt and retry)",
                version.0
            )
        })?;
        // A present-but-empty blob means a partial/in-flight publish; treat it as not-yet-available
        // (halt-and-retry) rather than feeding an empty blob into deserialization.
        if row.stored_key_material.is_empty() {
            anyhow::bail!(
                "key material for version {} is present but empty (not published yet, halt and retry)",
                version.0
            );
        }
        let result = Self::deserialize_db_key_row(row, version)?;

        let mut cache = self.cache.write().await;
        cache.put((result.key_id.clone(), version), result.clone());

        info!(
            "Key cached: key_id={:?}, seq={}, material_label={}",
            hex::encode(&result.key_id),
            result.sequence_number,
            version.0
        );
        Ok(result)
    }

    pub async fn fetch_latest_from_pool(&self, pool: &PgPool) -> anyhow::Result<DbKey> {
        let mut conn = pool.acquire().await?;
        self.fetch_latest(&mut conn).await
    }

    fn deserialize_db_key_row(
        row: DbKeyRow,
        material_version: MaterialVersion,
    ) -> anyhow::Result<DbKey> {
        let DbKeyRow {
            key_id,
            sequence_number,
            pks_key,
            stored_key_material,
            stored_as_xof,
            cks_key,
        } = row;
        let pks: tfhe::CompactPublicKey = safe_deserialize_key(&pks_key)?;
        let cks: Option<tfhe::ClientKey> = cks_key
            .as_ref()
            .map(|k| safe_deserialize_key(k))
            .transpose()?;

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
            let sks: tfhe::ServerKey = if stored_as_xof {
                let kxs: CompressedXofKeySet =
                    crate::utils::safe_deserialize_sns_key(&stored_key_material).map_err(|err| {
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
                safe_deserialize_key(&stored_key_material)?
            };

            Ok(DbKey {
                key_id,
                sequence_number,
                material_version,
                sks,
                pks,
                cks,
            })
        }
        #[cfg(feature = "gpu")]
        {
            if !stored_as_xof {
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
                crate::utils::safe_deserialize_sns_key(&stored_key_material).map_err(|err| {
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
                material_version,
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

    /// Which key-material version these bytes are (RFC-029).
    /// [`MaterialVersion::LEGACY`] for every key fetched via the
    /// pre-existing paths.
    pub material_version: MaterialVersion,

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

/// Stored format of key material read from `keys`.
///
/// `CompressedXof` blobs are `tfhe::xof_key_set::CompressedXofKeySet` —
/// the whole keyset must be deserialized in one pass to keep the XOF
/// state consistent across subkeys. `Legacy` blobs are plain
/// `tfhe::ServerKey` and can be deserialized directly. Reflects which
/// column in the `keys` table held the bytes.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum StoredServerKeyFormat {
    CompressedXof,
    Legacy,
}

/// Reads the normal SNS key material for the latest active key row.
///
/// Native compressed rows prefer `compressed_xof_keyset`. Rows participating
/// in the compressed-key migration force legacy material until the schedule
/// says callers should request compressed material explicitly.
pub async fn read_default_sns_key_material(
    pool: &PgPool,
    sequence_number: i64,
    legacy_capacity: usize,
) -> anyhow::Result<(Vec<u8>, StoredServerKeyFormat)> {
    let row = sqlx::query(
        "SELECT compressed_xof_keyset, sns_pk, compressed_key_state \
         FROM keys WHERE sequence_number = $1",
    )
    .bind(sequence_number)
    .fetch_one(pool)
    .await?;

    let compressed_key_state: Option<i16> = row.try_get("compressed_key_state")?;
    if compressed_key_state.is_none() {
        if let Some(bytes) = row.try_get::<Option<Vec<u8>>, _>("compressed_xof_keyset")? {
            info!(
                bytes_len = bytes.len(),
                "Retrieved compressed_xof_keyset BYTEA"
            );
            return Ok((bytes, StoredServerKeyFormat::CompressedXof));
        }
    }

    read_legacy_sns_key(row.try_get("sns_pk")?, pool, legacy_capacity).await
}

/// Reads compressed SNS key material for the latest active key row, if published.
///
/// This is the strict RFC-029 v1 read path: callers must not fall back to
/// legacy material after they resolved an operation to the compressed version.
pub async fn read_compressed_sns_key_material(
    pool: &PgPool,
    sequence_number: i64,
) -> anyhow::Result<Option<Vec<u8>>> {
    let row = sqlx::query("SELECT compressed_xof_keyset FROM keys WHERE sequence_number = $1")
        .bind(sequence_number)
        .fetch_one(pool)
        .await?;

    if let Some(bytes) = row.try_get::<Option<Vec<u8>>, _>("compressed_xof_keyset")? {
        info!(
            bytes_len = bytes.len(),
            "Retrieved compressed_xof_keyset BYTEA"
        );
        return Ok(Some(bytes));
    }
    Ok(None)
}

async fn read_legacy_sns_key(
    sns_pk: Option<Oid>,
    pool: &PgPool,
    legacy_capacity: usize,
) -> anyhow::Result<(Vec<u8>, StoredServerKeyFormat)> {
    // The activation upsert in host-listener::database gates on
    // key_content_sks_key IS NOT NULL but doesn't explicitly require
    // either server-key column; surface a clear error if a keys row
    // ever makes it here with both NULL rather than letting sqlx
    // produce an opaque ColumnDecode failure.
    let legacy = sns_pk.ok_or_else(|| {
        anyhow::anyhow!(
            "keys row for sequence_number has neither compressed_xof_keyset nor sns_pk populated"
        )
    })?;
    info!("Retrieved legacy sns_pk oid: {:?}", legacy);
    let bytes = read_large_object_in_chunks(pool, legacy, CHUNK_SIZE, legacy_capacity).await?;
    Ok((bytes, StoredServerKeyFormat::Legacy))
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
