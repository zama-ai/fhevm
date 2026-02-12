use std::ops::DerefMut;

use sqlx::{Postgres, Transaction};
use tracing::info;

use tokio_util::bytes::Bytes;

use fhevm_engine_common::db_keys::{write_large_object_in_chunks_tx, DbKeyId};

const CHUNK_SIZE: usize = 128 * 1024 * 1024; // 128MB

#[derive(Debug, Default)]
pub(crate) struct KeyRecord {
    pub key_id_gw: DbKeyId,
    pub pks_key: Bytes,
    pub sks_key: Bytes,
    pub sns_pk: Bytes,
}

impl KeyRecord {
    pub fn is_valid(&self) -> bool {
        !self.key_id_gw.is_empty()
            && !self.pks_key.is_empty()
            && !self.sks_key.is_empty()
            && !self.sns_pk.is_empty()
    }
}

pub async fn insert_key(
    tx: &mut Transaction<'_, Postgres>,
    key_record: &KeyRecord,
) -> anyhow::Result<()> {
    // TODO: we should extract the key_id from the server key
    // TODO: think about what happens to the written object if the SQL query fails
    let oid = write_large_object_in_chunks_tx(tx, &key_record.sns_pk, CHUNK_SIZE).await?;
    let query = sqlx::query!(
        "INSERT INTO keys (key_id, key_id_gw, pks_key, sks_key, sns_pk)
        VALUES ('', $1, $2, $3, $4)
        ON CONFLICT (key_id_gw) DO UPDATE SET
            key_id = '',
            pks_key = EXCLUDED.pks_key,
            sks_key = EXCLUDED.sks_key,
            sns_pk = EXCLUDED.sns_pk",
        &key_record.key_id_gw,
        key_record.pks_key.as_ref(),
        key_record.sks_key.as_ref(),
        oid,
    );
    query.execute(tx.deref_mut()).await?;
    Ok(())
}

// Inserts or updates the CRS associated with the given key ID.
pub async fn insert_crs(
    tx: &mut Transaction<'_, Postgres>,
    id: &[u8],
    crs: &[u8],
) -> anyhow::Result<()> {
    info!(id, "Inserting crs");
    let query = sqlx::query!(
        "INSERT INTO crs (crs_id, crs)
        VALUES ($1, $2)
        ON CONFLICT (crs_id) DO NOTHING",
        id,
        crs
    );
    query.execute(tx.deref_mut()).await?;
    Ok(())
}
