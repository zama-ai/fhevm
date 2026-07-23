use crate::ExecutionError;
use alloy_primitives::{Address, B256, U256};
use ciphertext_attestation::manifest::{ManifestReference, ManifestVersion, SignedManifest};
use sqlx::{Postgres, Transaction};

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

#[derive(Clone, Debug)]
pub(crate) struct StoreResult {
    pub manifest: AuthenticatedManifest,
    pub outcome: StoreOutcome,
}

#[derive(Clone, Debug)]
struct ManifestIdentity {
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
        "manifests/v{}/{}/{}/{}/{}/{}",
        u8::from(manifest.payload.version),
        manifest.payload.coprocessor_context_id,
        manifest.payload.host_chain_id,
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
) -> Result<StoreResult, ExecutionError> {
    let manifest = authenticate_manifest_object(expected_publisher, object_key, signed_bytes)?;
    let identity = ManifestIdentity::from_manifest(&manifest.signed)?;
    let inserted = sqlx::query!(
        r#"
        INSERT INTO block_consensus_manifest (
            publisher,
            version,
            coprocessor_context_id,
            host_chain_id,
            publication_block_number,
            publication_block_hash,
            revision,
            manifest_digest,
            object_key,
            signed_manifest
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
        ON CONFLICT DO NOTHING
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
    )
    .execute(trx.as_mut())
    .await?
    .rows_affected()
        == 1;

    let row = sqlx::query!(
        r#"
        SELECT manifest_digest, object_key, signed_manifest
          FROM block_consensus_manifest
         WHERE publisher = $1
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

    Ok(StoreResult {
        manifest,
        outcome: if inserted {
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
          FROM block_consensus_manifest
         WHERE publisher = $1
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

#[allow(
    dead_code,
    reason = "archive tip selection is exercised by DB simulations before peer polling is wired"
)]
pub(crate) async fn load_tip_eligible_manifest(
    trx: &mut Transaction<'_, Postgres>,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
    publication_block_number: i64,
    publication_block_hash: B256,
) -> Result<Option<AuthenticatedManifest>, ExecutionError> {
    let context = coprocessor_context_id.to_be_bytes::<32>();
    let rows = sqlx::query!(
        r#"
        SELECT publisher, manifest_digest, object_key, signed_manifest
          FROM block_consensus_manifest
         WHERE version = $1
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
        .iter()
        .enumerate()
        .filter(|(_, manifest)| manifest.signed.payload.publisher == publisher)
        .filter(|(index, _)| has_complete_supersession_chain(*index, &manifests))
        .max_by_key(|(_, manifest)| manifest.signed.payload.revision)
        .map(|(_, manifest)| manifest.clone()))
}

fn has_complete_supersession_chain(mut index: usize, manifests: &[AuthenticatedManifest]) -> bool {
    loop {
        let candidate = &manifests[index];
        if candidate.signed.payload.revision == 0 {
            return candidate.signed.payload.supersedes.is_none();
        }
        let Some(reference) = candidate.signed.payload.supersedes.as_ref() else {
            return false;
        };
        let Some(previous_index) = manifests.iter().position(|previous| {
            previous.signed.payload.publisher == reference.publisher
                && previous.signed.payload.publication_block_number == reference.block_number
                && previous.signed.payload.publication_block_hash == reference.block_hash
                && previous.signed.payload.revision == reference.revision
                && previous.digest == reference.manifest_digest
                && supersedes_authenticated_predecessor(candidate, previous)
        }) else {
            return false;
        };
        index = previous_index;
    }
}

#[allow(
    dead_code,
    reason = "archive tip selection is exercised by DB simulations before peer polling is wired"
)]
fn supersedes_authenticated_predecessor(
    candidate: &AuthenticatedManifest,
    previous: &AuthenticatedManifest,
) -> bool {
    let payload = &candidate.signed.payload;
    let previous_payload = &previous.signed.payload;
    payload.revision == previous_payload.revision + 1
        && payload.supersedes.as_ref().is_some_and(|reference| {
            reference.publisher == previous_payload.publisher
                && reference.block_number == previous_payload.publication_block_number
                && reference.block_hash == previous_payload.publication_block_hash
                && reference.revision == previous_payload.revision
                && reference.manifest_digest == previous.digest
        })
}

fn matches_archive_scope(
    manifest: &AuthenticatedManifest,
    publisher: Address,
    version: ManifestVersion,
    coprocessor_context_id: U256,
    host_chain_id: i64,
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
