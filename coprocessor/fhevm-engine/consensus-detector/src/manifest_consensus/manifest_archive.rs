use crate::manifest_consensus::ExecutionError;
use alloy_primitives::{Address, B256, U256};
use block_manifest::{ManifestVersion, SignedManifest};
use sqlx::{Postgres, Transaction};

#[allow(dead_code)]
const MAX_COVERING_MANIFEST_CANDIDATES: i64 = 64;
#[allow(dead_code)]
pub(crate) const MAX_HISTORY_PREDECESSORS: usize = 5;

/// Internal database locator. It is not part of the signed manifest format.
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct ManifestReference {
    pub(crate) generation: u64,
    pub(crate) publisher: Address,
    pub(crate) block_number: U256,
    pub(crate) block_hash: B256,
    pub(crate) revision: u64,
    pub(crate) manifest_digest: B256,
}

#[derive(Clone, Debug)]
pub(crate) struct AuthenticatedManifest {
    pub signed: SignedManifest,
    pub digest: B256,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum StoreOutcome {
    Inserted,
    AlreadyPresent,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum ManifestSource {
    Local,
    #[allow(dead_code)]
    Peer,
}

impl ManifestSource {
    const fn as_db_str(self) -> &'static str {
        match self {
            Self::Local => "local",
            Self::Peer => "peer",
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) struct StoreResult {
    #[allow(dead_code)]
    pub id: i64,
    pub manifest: AuthenticatedManifest,
    pub outcome: StoreOutcome,
}

#[derive(Clone, Debug)]
struct ManifestIdentity {
    generation: i64,
    publisher: Address,
    version: i16,
    coprocessor_context_id: [u8; 32],
    host_chain_id: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
    revision: i64,
}

impl ManifestIdentity {
    fn from_manifest(manifest: &SignedManifest) -> Result<Self, ExecutionError> {
        Ok(Self {
            generation: i64::try_from(manifest.payload.generation)
                .map_err(|_| internal("manifest generation exceeds BIGINT"))?,
            publisher: manifest.payload.publisher,
            version: i16::from(u8::from(manifest.payload.version)),
            coprocessor_context_id: manifest.payload.coprocessor_context_id.to_be_bytes(),
            host_chain_id: i64_from_u256("manifest host chain id", manifest.payload.host_chain_id)?,
            publication_block_number: i64_from_u256(
                "manifest publication block number",
                manifest.payload.publication_block_number,
            )?,
            publication_block_hash: manifest.payload.publication_block_hash,
            revision: i64::try_from(manifest.payload.revision)
                .map_err(|_| internal("manifest revision exceeds BIGINT"))?,
        })
    }
}

pub(crate) fn manifest_object_key(manifest: &SignedManifest) -> String {
    format!(
        "manifests/v_{}/context_{}/chain_{}/generation_{}/block_{}/hash_{}/revision_{}",
        u8::from(manifest.payload.version),
        manifest.payload.coprocessor_context_id,
        manifest.payload.host_chain_id,
        manifest.payload.generation,
        manifest.payload.publication_block_number,
        hex::encode(manifest.payload.publication_block_hash),
        manifest.payload.revision,
    )
}

pub(crate) fn authenticate_manifest_object(
    expected_publisher: Address,
    object_key: &str,
    signed_bytes: &[u8],
) -> Result<AuthenticatedManifest, ExecutionError> {
    let signed: SignedManifest = serde_json::from_slice(signed_bytes)
        .map_err(|err| ExecutionError::DeserializationError(err.to_string()))?;
    signed
        .verify()
        .map_err(|err| internal(format!("manifest signature or payload is invalid: {err}")))?;
    if signed.payload.publisher != expected_publisher {
        return Err(internal(format!(
            "manifest publisher {} does not match expected publisher {}",
            signed.payload.publisher, expected_publisher,
        )));
    }

    let canonical_key = manifest_object_key(&signed);
    if object_key != canonical_key {
        return Err(internal(format!(
            "manifest object key {object_key} does not match signed identity {canonical_key}",
        )));
    }

    let digest = signed
        .digest()
        .map_err(|err| internal(format!("manifest digest is invalid: {err}")))?;
    Ok(AuthenticatedManifest { signed, digest })
}

pub(crate) async fn store_authenticated_manifest(
    trx: &mut Transaction<'_, Postgres>,
    expected_publisher: Address,
    object_key: &str,
    signed_bytes: &[u8],
    source: ManifestSource,
) -> Result<StoreResult, ExecutionError> {
    let manifest = authenticate_manifest_object(expected_publisher, object_key, signed_bytes)?;
    let identity = ManifestIdentity::from_manifest(&manifest.signed)?;
    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_manifest (
            generation,
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            manifest_digest,
            object_key,
            signed_manifest,
            manifest_source
        )
        VALUES ($12, $1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
        ON CONFLICT DO NOTHING
        RETURNING id
        "#,
        identity.publisher.as_slice(),
        identity.version,
        identity.coprocessor_context_id.as_slice(),
        identity.host_chain_id,
        identity.publication_block_number,
        identity.publication_block_hash.as_slice(),
        identity.revision,
        manifest.digest.as_slice(),
        object_key,
        signed_bytes,
        source.as_db_str(),
        identity.generation,
    )
    .fetch_optional(trx.as_mut())
    .await?;

    if inserted.is_none() && matches!(source, ManifestSource::Local) {
        sqlx::query!(
            r#"
            UPDATE block_manifest
               SET manifest_source = 'local'
             WHERE publisher = $1
               AND generation = $8
               AND version = $2
               AND coprocessor_context_id = $3
               AND host_chain_id = $4
               AND publication_block_number = $5
               AND publication_block_hash = $6
               AND revision = $7
               AND manifest_source = 'peer'
            "#,
            identity.publisher.as_slice(),
            identity.version,
            identity.coprocessor_context_id.as_slice(),
            identity.host_chain_id,
            identity.publication_block_number,
            identity.publication_block_hash.as_slice(),
            identity.revision,
            identity.generation,
        )
        .execute(trx.as_mut())
        .await?;
    }

    let row = sqlx::query!(
        r#"
        SELECT id, manifest_digest, object_key, signed_manifest
          FROM block_manifest
         WHERE publisher = $1
           AND generation = $8
           AND version = $2
           AND coprocessor_context_id = $3
           AND host_chain_id = $4
           AND publication_block_number = $5
           AND publication_block_hash = $6
           AND revision = $7
        "#,
        identity.publisher.as_slice(),
        identity.version,
        identity.coprocessor_context_id.as_slice(),
        identity.host_chain_id,
        identity.publication_block_number,
        identity.publication_block_hash.as_slice(),
        identity.revision,
        identity.generation,
    )
    .fetch_one(trx.as_mut())
    .await?;

    let stored_digest = b256("stored manifest digest", &row.manifest_digest)?;
    let stored_key = row.object_key;
    let stored_bytes = row.signed_manifest;
    if stored_digest != manifest.digest {
        return Err(internal(format!(
            "manifest equivocation for publisher {} key {}: stored digest {}, observed digest {}",
            expected_publisher, object_key, stored_digest, manifest.digest,
        )));
    }
    if stored_key != object_key {
        return Err(internal(format!(
            "immutable manifest identity has conflicting object keys {stored_key} and {object_key}",
        )));
    }
    if stored_bytes != signed_bytes {
        return Err(internal(format!(
            "immutable manifest object {object_key} has different signed wire bytes for the same digest",
        )));
    }

    let id = inserted.as_ref().map_or(row.id, |inserted| inserted.id);
    let outcome = inserted.is_some();
    Ok(StoreResult {
        id,
        manifest,
        outcome: if outcome {
            StoreOutcome::Inserted
        } else {
            StoreOutcome::AlreadyPresent
        },
    })
}

pub(crate) async fn load_manifest_by_reference(
    trx: &mut Transaction<'_, Postgres>,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    reference: &ManifestReference,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    let publication_block_number =
        i64_from_u256("manifest publication block number", reference.block_number)?;
    let revision = i64::try_from(reference.revision)
        .map_err(|_| internal("manifest revision exceeds BIGINT"))?;
    let manifest = load_manifest_revision(
        trx,
        reference.publisher,
        version,
        coprocessor_context_id,
        host_chain_id,
        i64::try_from(reference.generation)
            .map_err(|_| internal("manifest generation exceeds BIGINT"))?,
        publication_block_number,
        reference.block_hash,
        revision,
    )
    .await?;
    let Some(manifest) = manifest else {
        return Ok(None);
    };
    if manifest.digest != reference.manifest_digest {
        return Err(internal(format!(
            "stored manifest digest does not match reference for publisher {} revision {}",
            reference.publisher, reference.revision,
        )));
    }
    Ok(Some(manifest))
}

#[allow(clippy::too_many_arguments)]
pub(crate) async fn load_manifest_revision(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    generation: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
    revision: i64,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    if revision < 0 {
        return Err(internal("manifest revision is negative"));
    }
    let context = coprocessor_context_id.to_be_bytes::<32>();
    let row = sqlx::query!(
        r#"
        SELECT manifest_digest, object_key, signed_manifest
          FROM block_manifest
         WHERE publisher = $1
           AND generation = $8
           AND version = $2
           AND coprocessor_context_id = $3
           AND host_chain_id = $4
           AND publication_block_number = $5
           AND publication_block_hash = $6
           AND revision = $7
        "#,
        publisher.as_slice(),
        i16::from(u8::from(version)),
        context.as_slice(),
        host_chain_id,
        publication_block_number,
        publication_block_hash.as_slice(),
        revision,
        generation,
    )
    .fetch_optional(trx.as_mut())
    .await?;
    let Some(row) = row else {
        return Ok(None);
    };

    let object_key = row.object_key;
    let signed_bytes = row.signed_manifest;
    let manifest = authenticate_manifest_object(publisher, &object_key, &signed_bytes)?;
    if !matches_archive_scope(
        &manifest,
        publisher,
        version,
        coprocessor_context_id,
        host_chain_id,
        generation,
        publication_block_number,
        publication_block_hash,
    ) || manifest.signed.payload.revision
        != u64::try_from(revision).expect("checked non-negative")
    {
        return Err(internal(format!(
            "stored manifest body does not match its archive identity for publisher {} key {}",
            publisher, object_key,
        )));
    }
    let stored_digest = b256("stored manifest digest", &row.manifest_digest)?;
    if manifest.digest != stored_digest {
        return Err(internal(format!(
            "stored manifest digest does not match body for publisher {} key {}",
            publisher, object_key,
        )));
    }
    Ok(Some(manifest))
}

/// Finds the nearest archived local manifest whose direct range contains the
/// requested block. The caller supplies the block hash as well, so competing
/// lineages at the same height cannot be confused.
///
/// This is the history scanner's jump primitive: a differing historical range
/// selects the local manifest that directly covers that range's newest block,
/// rather than requiring every intermediate publication point.
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub(crate) async fn load_local_manifest_covering_block(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    generation: i64,
    target_block_number: U256,
    target_block_hash: B256,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    let target_block = target_block_number;
    let target_block_number = i64_from_u256("covered block number", target_block_number)?;
    let context = coprocessor_context_id.to_be_bytes::<32>();
    let candidates = sqlx::query!(
        r#"
        SELECT DISTINCT publication_block_number, publication_block_hash
          FROM block_manifest
         WHERE publisher = $1
           AND generation = $7
           AND version = $2
           AND coprocessor_context_id = $3
           AND host_chain_id = $4
           AND publication_block_number >= $5
           AND manifest_source = 'local'
         ORDER BY publication_block_number, publication_block_hash
         LIMIT $6
        "#,
        publisher.as_slice(),
        i16::from(u8::from(version)),
        context.as_slice(),
        host_chain_id,
        target_block_number,
        MAX_COVERING_MANIFEST_CANDIDATES,
        generation,
    )
    .fetch_all(trx.as_mut())
    .await?;

    for candidate in candidates {
        let publication_block_hash = b256(
            "covering manifest publication block hash",
            &candidate.publication_block_hash,
        )?;
        let Some(manifest) = load_tip_eligible_manifest(
            trx,
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            generation,
            candidate.publication_block_number,
            publication_block_hash,
        )
        .await?
        else {
            continue;
        };
        if manifest
            .signed
            .payload
            .detailed_range
            .blocks
            .iter()
            .any(|block| {
                block.block_number == target_block && block.block_hash == target_block_hash
            })
        {
            return Ok(Some(manifest));
        }
    }
    Ok(None)
}

/// Loads the closest earlier local publication points on the same generation
/// and lineage namespace. Callers use these signed manifests only to refine
/// history when a peer did not publish the corresponding covering manifest.
#[allow(clippy::too_many_arguments)]
#[allow(dead_code)]
pub(crate) async fn load_previous_local_manifests(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    starting_manifest: &AuthenticatedManifest,
    limit: usize,
) -> Result<Vec<AuthenticatedManifest>, ExecutionError> {
    let payload = &starting_manifest.signed.payload;
    let version = payload.version;
    let coprocessor_context_id = payload.coprocessor_context_id;
    let host_chain_id = i64_from_u256("manifest host chain id", payload.host_chain_id)?;
    let generation = i64::try_from(payload.generation)
        .map_err(|_| internal("manifest generation exceeds BIGINT"))?;
    let mut current = starting_manifest.clone();
    let mut manifests = Vec::with_capacity(limit);
    for _ in 0..limit {
        let Some(first_detailed_block) = current.signed.payload.detailed_range.blocks.first()
        else {
            break;
        };
        let Some(previous_block_number) = first_detailed_block.block_number.checked_sub(U256::ONE)
        else {
            break;
        };
        let previous_block_number =
            i64_from_u256("previous manifest block number", previous_block_number)?;
        let Some(previous) = load_tip_eligible_manifest(
            trx,
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            generation,
            previous_block_number,
            first_detailed_block.parent_block_hash,
        )
        .await?
        else {
            break;
        };
        manifests.push(previous.clone());
        current = previous;
    }
    Ok(manifests)
}

#[allow(dead_code)]
#[allow(clippy::too_many_arguments)]
pub(crate) async fn load_tip_eligible_manifest(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    generation: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    let context = coprocessor_context_id.to_be_bytes::<32>();
    let rows = sqlx::query!(
        r#"
        SELECT publisher, manifest_digest, object_key, signed_manifest
          FROM block_manifest
         WHERE version = $1
           AND generation = $6
           AND coprocessor_context_id = $2
           AND host_chain_id = $3
           AND publication_block_number = $4
           AND publication_block_hash = $5
         ORDER BY revision, publisher
        "#,
        i16::from(u8::from(version)),
        context.as_slice(),
        host_chain_id,
        publication_block_number,
        publication_block_hash.as_slice(),
        generation,
    )
    .fetch_all(trx.as_mut())
    .await?;

    let mut manifests = Vec::with_capacity(rows.len());
    for row in rows {
        let stored_publisher = address("stored manifest publisher", &row.publisher)?;
        let object_key = row.object_key;
        let signed_bytes = row.signed_manifest;
        let candidate = authenticate_manifest_object(stored_publisher, &object_key, &signed_bytes)?;
        if !matches_archive_scope(
            &candidate,
            stored_publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            generation,
            publication_block_number,
            publication_block_hash,
        ) {
            return Err(internal(format!(
                "stored manifest body does not match its archive identity for publisher {} key {}",
                stored_publisher, object_key,
            )));
        }
        let stored_digest = b256("stored manifest digest", &row.manifest_digest)?;
        if candidate.digest != stored_digest {
            return Err(internal(format!(
                "stored manifest digest is corrupt for publisher {} key {}",
                stored_publisher, object_key,
            )));
        }
        manifests.push(candidate);
    }

    Ok(manifests
        .into_iter()
        .filter(|manifest| manifest.signed.payload.publisher == publisher)
        .max_by_key(|manifest| manifest.signed.payload.revision))
}

#[allow(clippy::too_many_arguments)]
fn matches_archive_scope(
    manifest: &AuthenticatedManifest,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    generation: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
) -> bool {
    let Ok(host_chain_id) = u64::try_from(host_chain_id) else {
        return false;
    };
    let Ok(publication_block_number) = u64::try_from(publication_block_number) else {
        return false;
    };
    let payload = &manifest.signed.payload;
    payload.publisher == publisher
        && payload.version == version
        && payload.coprocessor_context_id == coprocessor_context_id
        && payload.host_chain_id == U256::from(host_chain_id)
        && payload.generation == u64::try_from(generation).unwrap_or(u64::MAX)
        && payload.publication_block_number == U256::from(publication_block_number)
        && payload.publication_block_hash == publication_block_hash
}

fn i64_from_u256(field: &str, value: U256) -> Result<i64, ExecutionError> {
    i64::try_from(value).map_err(|_| internal(format!("{field} exceeds BIGINT")))
}

fn b256(field: &str, value: &[u8]) -> Result<B256, ExecutionError> {
    let value: [u8; 32] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 32 bytes, got {}", value.len())))?;
    Ok(B256::from(value))
}

#[allow(dead_code)]
fn address(field: &str, value: &[u8]) -> Result<Address, ExecutionError> {
    let value: [u8; 20] = value
        .try_into()
        .map_err(|_| internal(format!("{field} must be 20 bytes, got {}", value.len())))?;
    Ok(Address::from(value))
}

fn internal(message: impl Into<String>) -> ExecutionError {
    ExecutionError::InternalError(message.into())
}

#[cfg(test)]
#[path = "manifest_archive_tests.rs"]
mod tests;
