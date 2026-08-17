use super::*;

pub(super) async fn load_seeded_block(
    pool: &PgPool,
    host_chain_id: i64,
    block_hash: B256,
) -> PendingBlock {
    let row = sqlx::query(
        "SELECT generation, host_chain_id, block_number, block_hash, parent_block_hash,
                publication_cadence, block_content_digest, block_handle_count, manifest_revision,
                manifest_publisher,
                manifest_digest, manifest_published
           FROM block_manifest_state
          WHERE host_chain_id = $1 AND block_hash = $2",
    )
    .bind(host_chain_id)
    .bind(block_hash.as_slice())
    .fetch_one(pool)
    .await
    .expect("load seeded manifest block");
    PendingBlock {
        generation: row.get("generation"),
        host_chain_id: row.get("host_chain_id"),
        block_number: row.get("block_number"),
        block_hash: row.get("block_hash"),
        parent_block_hash: row.get("parent_block_hash"),
        publication_cadence: row.get("publication_cadence"),
        block_content_digest: row.get("block_content_digest"),
        block_handle_count: row.get("block_handle_count"),
        manifest_revision: row.get("manifest_revision"),
        manifest_publisher: row.get("manifest_publisher"),
        manifest_digest: row.get("manifest_digest"),
        manifest_published: row.get("manifest_published"),
    }
}

pub(super) async fn seal_seeded_block(
    pool: &PgPool,
    host_chain_id: i64,
    block_hash: B256,
    context: U256,
) {
    let mut trx = pool.begin().await.expect("begin seeded block seal");
    let block = load_seeded_block(pool, host_chain_id, block_hash).await;
    assert!(is_block_manifest_ready(&mut trx, &block)
        .await
        .expect("check seeded block readiness"));
    let descriptors = load_manifest_descriptors(&mut trx, &block)
        .await
        .expect("load seeded block descriptors");
    seal_block_content(&mut trx, &block, context, &descriptors)
        .await
        .expect("seal seeded block");
    trx.commit().await.expect("commit seeded block seal");
}

#[allow(clippy::too_many_arguments)]
pub(super) async fn seed_revision_publication_block(
    pool: &PgPool,
    host_chain_id: i64,
    block_number: i64,
    block_hash: B256,
    parent_hash: B256,
    handle: B256,
    _transaction_id: B256,
    _dependence_chain_id: B256,
    gateway_key_id: B256,
    keyset_id: B256,
    ct64_digest: B256,
    ct128_digest: B256,
) {
    sqlx::query(
        "INSERT INTO block_manifest_state
             (host_chain_id, block_number, block_hash, parent_block_hash, publication_cadence)
         VALUES ($1, $2, $3, $4, 1)",
    )
    .bind(host_chain_id)
    .bind(block_number)
    .bind(block_hash.as_slice())
    .bind(parent_hash.as_slice())
    .execute(pool)
    .await
    .expect("insert revision publication block");
    sqlx::query(
        "INSERT INTO handle_producer_block (
             host_chain_id, producer_block_number, producer_block_hash, handle
         ) VALUES ($1, $2, $3, $4)",
    )
    .bind(host_chain_id)
    .bind(block_number)
    .bind(block_hash.as_slice())
    .bind(handle.as_slice())
    .execute(pool)
    .await
    .expect("insert revision producer block");
    sqlx::query(
        "INSERT INTO keys (key_id_gw, key_id, pks_key, sks_key, chain_id, block_hash)
         VALUES ($1, $2, ''::BYTEA, ''::BYTEA, $3, $4)",
    )
    .bind(gateway_key_id.as_slice())
    .bind(keyset_id.as_slice())
    .bind(host_chain_id)
    .bind(block_hash.as_slice())
    .execute(pool)
    .await
    .expect("insert revision keyset");
    sqlx::query(
        "INSERT INTO ciphertext_digest (
             host_chain_id, key_id_gw, handle, ciphertext, ciphertext128,
             ciphertext128_format
         ) VALUES ($1, $2, $3, $4, $5, 11)",
    )
    .bind(host_chain_id)
    .bind(gateway_key_id.as_slice())
    .bind(handle.as_slice())
    .bind(ct64_digest.as_slice())
    .bind(ct128_digest.as_slice())
    .execute(pool)
    .await
    .expect("insert revision ciphertext digests");
}

pub(super) async fn load_local_revision(
    pool: &PgPool,
    publisher: Address,
    context: U256,
    host_chain_id: i64,
    block_number: i64,
    block_hash: B256,
    revision: i64,
) -> AuthenticatedManifest {
    let mut trx = pool.begin().await.expect("begin local revision load");
    let manifest = load_manifest_revision(
        &mut trx,
        publisher,
        ManifestVersion::V1,
        context,
        host_chain_id,
        0,
        block_number,
        block_hash,
        revision,
    )
    .await
    .expect("load local manifest revision")
    .expect("local manifest revision exists");
    trx.commit().await.expect("commit local revision load");
    manifest
}
