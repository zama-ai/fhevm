use std::{collections::HashMap, panic::AssertUnwindSafe, time::Duration, time::Instant};

use alloy_primitives::{Address, B256, U256};
use aws_sdk_s3::{error::SdkError, primitives::ByteStream, types::MetadataDirective, Client};
use ciphertext_attestation::{
    CiphertextAttestation, CiphertextAttestationPayload, CiphertextFormat, Version,
    S3_METADATA_ATTESTATION_KEY,
};
use fhevm_engine_common::{types::CoproSigner, utils::to_hex};
use futures::{stream::FuturesUnordered, FutureExt, StreamExt};
use sqlx::PgPool;
use tokio_util::sync::CancellationToken;
use tracing::{error, info, warn};

use crate::aws_upload::{
    check_is_ready, compute_digest, s3_ciphertext_key, COPROCESSOR_CONTEXT_ID_1,
};
use crate::{
    Ciphertext128Format, ExecutionError, S3Config, CLEAN_OLD_S3_FORMAT_VERSION,
    S3_FORMAT_VERSION_V1,
};

pub const DEFAULT_S3_MIGRATION_MAX_RETRIES: i32 = 100;
const NO_SNS_CIPHERTEXT_DIGEST: [u8; 32] = [0; 32];

#[derive(Debug, Clone, Copy)]
pub(crate) enum CiphertextKind {
    Ct64,
    Ct128,
}

#[derive(Debug)]
pub(crate) struct MigrationRow {
    pub(crate) handle: Vec<u8>,
    pub(crate) key_id_gw: Vec<u8>,
    pub(crate) transaction_id: Option<Vec<u8>>,
    pub(crate) ciphertext: Option<Vec<u8>>,
    pub(crate) ciphertext128: Option<Vec<u8>>,
    pub(crate) ciphertext128_format: i16,
}

#[derive(Debug, Clone)]
pub(crate) struct CopySourceCandidate {
    pub(crate) key: String,
}

struct ExistingS3Object {
    metadata: HashMap<String, String>,
    bytes: Vec<u8>,
}

pub(crate) struct MigrationMaterial {
    pub(crate) handle: Vec<u8>,
    key_id_gw: Vec<u8>,
    row_ct64_digest: Option<Vec<u8>>,
    row_ct128_digest: Option<Vec<u8>>,
    pub(crate) ct64_digest: Vec<u8>,
    pub(crate) ct128_digest: Vec<u8>,
    pub(crate) has_ct64: bool,
    pub(crate) has_ct128: bool,
    pub(crate) ct128_format: Ciphertext128Format,
    signer: Address,
    metadata: S3MigrationMetadata,
}

#[derive(Clone)]
struct S3MigrationMetadata {
    attestation_json: String,
    key_id: String,
    transaction_id: String,
    signer: String,
}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub enum S3MigrationMode {
    #[default]
    No,
    Before,
    BeforeAndQuit,
    Concurrent,
    DryRun,
}

impl std::str::FromStr for S3MigrationMode {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "no" => Ok(Self::No),
            "before" => Ok(Self::Before),
            "before-and-quit" => Ok(Self::BeforeAndQuit),
            "concurrent" => Ok(Self::Concurrent),
            "dry-run" => Ok(Self::DryRun),
            other => Err(format!(
                "invalid S3 migration mode {other:?}, expected no, before, before-and-quit, concurrent, or dry-run"
            )),
        }
    }
}

impl std::fmt::Display for S3MigrationMode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::No => write!(f, "no"),
            Self::Before => write!(f, "before"),
            Self::BeforeAndQuit => write!(f, "before-and-quit"),
            Self::Concurrent => write!(f, "concurrent"),
            Self::DryRun => write!(f, "dry-run"),
        }
    }
}

#[derive(Clone)]
pub struct S3MigrationConfig {
    pub batch_size: u64,
    pub signer: CoproSigner,
    pub s3: S3Config,
    pub mode: S3MigrationMode,
    pub sleep_duration: Duration,
    pub max_retries: i32,
}

const PANIC_RETRY_DELAY: Duration = Duration::from_secs(10);
const CLEAR_PANIC_WINDOW: Duration = Duration::from_mins(3);
const MAX_PANIC_PER_WINDOW: u64 = 10;
const NOT_READY_DELAY: Duration = Duration::from_secs(30);
const MAX_NOT_READY: u64 = 10;

pub(crate) async fn run_startup_migrations(
    config: &S3MigrationConfig,
    token: &CancellationToken,
    pool: &PgPool,
    client: &Client,
) -> Result<(), ExecutionError> {
    let mut last_panic_time = Instant::now();
    let mut successive_panics = 0;
    for try_count in 1..=MAX_NOT_READY {
        let (is_ready, _) = check_is_ready(client, &config.s3).await;
        if is_ready {
            break;
        }
        error!(
            try_count,
            MAX_NOT_READY,
            ?NOT_READY_DELAY,
            "S3 is not ready yet"
        );
        tokio::time::sleep(NOT_READY_DELAY).await;
    }
    loop {
        let token = token.clone();
        match AssertUnwindSafe(migrate_s3_format_0_to_1(config, token, pool, client))
            .catch_unwind()
            .await
        {
            Ok(result) => return result,
            Err(payload) => {
                let now = Instant::now();
                if now.duration_since(last_panic_time) > CLEAR_PANIC_WINDOW {
                    successive_panics = 0;
                }
                successive_panics += 1;
                last_panic_time = now;
                if successive_panics > MAX_PANIC_PER_WINDOW {
                    return Err(ExecutionError::InternalError(
                        "Lots of panics without apparent progress.".into(),
                    ));
                }
                error!(
                    panic = %panic_payload_to_string(payload.as_ref()),
                    ?PANIC_RETRY_DELAY,
                    successive_panics,
                    "S3 format migration panicked; retrying"
                );
                tokio::time::sleep(PANIC_RETRY_DELAY).await;
            }
        }
    }
}

fn panic_payload_to_string(payload: &(dyn std::any::Any + Send)) -> String {
    if let Some(message) = payload.downcast_ref::<&'static str>() {
        (*message).to_owned()
    } else if let Some(message) = payload.downcast_ref::<String>() {
        message.clone()
    } else {
        "non-string panic payload".to_owned()
    }
}

async fn migrate_s3_format_0_to_1(
    config: &S3MigrationConfig,
    token: CancellationToken,
    pool: &PgPool,
    client: &Client,
) -> Result<(), ExecutionError> {
    let mut total_migrated = 0_u64;
    let mut worked_since_idle_log = false;
    loop {
        let already_failed = count_failed_old_format_handles(pool).await?;
        let mut total_failed = already_failed as u64;
        let remainings = count_pending_old_format_handles(pool).await?;

        info!(
            handles_to_process = remainings,
            handles_with_recorded_failures = already_failed,
            from_s3_format_version = CLEAN_OLD_S3_FORMAT_VERSION,
            to_s3_format_version = S3_FORMAT_VERSION_V1,
            "S3 format migration global status"
        );
        if token.is_cancelled() {
            return Ok(());
        }
        let new_handles = fetch_old_format_handles(config, pool, false).await?;
        if !new_handles.is_empty() {
            let (migrated, failed) =
                migrate_handle_batch(config, pool, client, &new_handles).await?;
            total_migrated += migrated;
            total_failed += failed;
            worked_since_idle_log = true;
            info!(total_failed, total_migrated, "S3 Migration");
        }

        // global retry part, on smaller non zero retry count
        let mut retry_handles = Vec::new();
        if total_failed > 0 {
            retry_handles = fetch_old_format_handles(config, pool, true).await?;
            if !retry_handles.is_empty() {
                let (migrated, failed) =
                    migrate_handle_batch(config, pool, client, &retry_handles).await?;
                total_migrated += migrated;
                total_failed = total_failed.saturating_sub(migrated);
                worked_since_idle_log = true;
                if failed > 0 {
                    if migrated == 0 {
                        error!(
                            total_failed,
                            total_migrated, migrated, failed, "S3 Migration retry"
                        );
                    } else {
                        warn!(
                            total_failed,
                            total_migrated, migrated, failed, "S3 Migration retry"
                        );
                    }
                }
            }
        }

        if new_handles.is_empty() && retry_handles.is_empty() {
            if config.mode != S3MigrationMode::Concurrent {
                let remaining_failed = count_failed_old_format_handles(pool).await? as u64;
                if remaining_failed > 0 {
                    return Err(migration_failed_error(remaining_failed, config.max_retries));
                }
                break;
            }
            if worked_since_idle_log {
                info!(
                    total_failed,
                    total_migrated, "S3 Migration done, go to sleep"
                );
                worked_since_idle_log = false;
            }
            tokio::time::sleep(config.sleep_duration).await;
        }
    }

    info!(
        handles_migrated = total_migrated,
        from_s3_format_version = CLEAN_OLD_S3_FORMAT_VERSION,
        to_s3_format_version = S3_FORMAT_VERSION_V1,
        "Finished S3 format migration"
    );

    Ok(())
}

fn migration_failed_error(total_failed: u64, max_retries: i32) -> ExecutionError {
    ExecutionError::S3TransientError(format!(
        "S3 migration failed for {total_failed} handle(s) after reaching max retry count {max_retries}"
    ))
}

pub(crate) async fn count_pending_old_format_handles(pool: &PgPool) -> Result<i64, ExecutionError> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)::BIGINT AS "count!"
         FROM ciphertext_digest
         WHERE s3_format_version = $1
           AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
           AND s3_migration_failure_count = 0
        "#,
        CLEAN_OLD_S3_FORMAT_VERSION,
    )
    .fetch_one(pool)
    .await?;

    Ok(count)
}

pub(crate) async fn count_failed_old_format_handles(pool: &PgPool) -> Result<i64, ExecutionError> {
    let count = sqlx::query_scalar!(
        r#"
        SELECT COUNT(*)::BIGINT AS "count!"
         FROM ciphertext_digest
         WHERE s3_format_version = $1
           AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
           AND s3_migration_failure_count > 0
        "#,
        CLEAN_OLD_S3_FORMAT_VERSION,
    )
    .fetch_one(pool)
    .await?;

    Ok(count)
}

async fn fetch_old_format_handles(
    config: &S3MigrationConfig,
    pool: &PgPool,
    focus_on_retry: bool,
) -> Result<Vec<Vec<u8>>, ExecutionError> {
    let handles = sqlx::query_scalar!(
        r#"
        SELECT handle AS "handle!"
         FROM ciphertext_digest
         WHERE s3_format_version = $1
           AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
           AND (
               ($2 = FALSE AND s3_migration_failure_count = 0)
               OR (
                   $2 = TRUE
                   AND s3_migration_failure_count > 0
                   AND s3_migration_failure_count < $3
                   AND s3_migration_failure_count = (
                       SELECT MIN(s3_migration_failure_count)
                        FROM ciphertext_digest
                        WHERE s3_format_version = $1
                          AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
                          AND s3_migration_failure_count > 0
                          AND s3_migration_failure_count < $3
                   )
               )
           )
         ORDER BY s3_migration_failure_count, handle
         LIMIT $4
        "#,
        CLEAN_OLD_S3_FORMAT_VERSION,
        focus_on_retry,
        config.max_retries,
        config.batch_size as i64,
    )
    .fetch_all(pool)
    .await?;

    Ok(handles)
}

#[allow(clippy::too_many_arguments)]
async fn migrate_handle_batch(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
    handles: &[Vec<u8>],
) -> Result<(u64, u64), ExecutionError> {
    let mut migrated = 0;
    let mut failed = 0;
    let mut tasks = FuturesUnordered::new();
    for handle in handles.iter() {
        tasks.push(async move {
            (
                handle.clone(),
                migrate_handle_0_to_1(config, pool, client, handle).await,
            )
        });
    }
    while let Some((handle, result)) = tasks.next().await {
        match result {
            Ok(true) => migrated += 1,
            Ok(false) => {}
            Err(err) => {
                failed += 1;
                let handle_hex = to_hex(&handle);
                error!(
                    handle = handle_hex,
                    error = %err,
                    "S3 migration, failed for handle"
                );
                if let Err(err) = record_migration_failure(pool, &handle, &err).await {
                    error!(?err, "S3 migration, cannot record failure on DB");
                }
            }
        }
    }
    Ok((migrated, failed))
}

async fn record_migration_failure(
    pool: &PgPool,
    handle: &[u8],
    error: &ExecutionError,
) -> Result<(), ExecutionError> {
    sqlx::query!(
        r#"
        UPDATE ciphertext_digest
         SET s3_migration_failure_count = s3_migration_failure_count + 1,
             s3_migration_last_error = $1,
             s3_migration_last_error_at = NOW()
         WHERE handle = $2
           AND s3_format_version = $3
        "#,
        error.to_string(),
        handle,
        CLEAN_OLD_S3_FORMAT_VERSION,
    )
    .execute(pool)
    .await?;

    Ok(())
}

async fn migrate_handle_0_to_1(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
    handle: &[u8],
) -> Result<bool, ExecutionError> {
    let row = sqlx::query!(
        r#"
        SELECT handle,
               key_id_gw,
               transaction_id,
               ciphertext,
               ciphertext128,
               ciphertext128_format
         FROM ciphertext_digest
         WHERE handle = $1
           AND s3_format_version = $2
           AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
        "#,
        handle,
        CLEAN_OLD_S3_FORMAT_VERSION,
    )
    .fetch_optional(pool)
    .await?;

    let Some(row) = row else {
        return Ok(false);
    };

    let row = MigrationRow {
        handle: row.handle,
        key_id_gw: row.key_id_gw,
        transaction_id: row.transaction_id,
        ciphertext: row.ciphertext,
        ciphertext128: row.ciphertext128,
        ciphertext128_format: row.ciphertext128_format,
    };

    let material = prepare_migration_material(pool, row, &config.signer).await?;
    migrate_ct64_object(pool, client, &config.s3, &material).await?;
    migrate_ct128_object(pool, client, &config.s3, &material).await?;

    if !migrated_objects_are_current(client, &config.s3, &material).await? {
        return Err(ExecutionError::S3TransientError(format!(
            "S3 migration did not publish all current-format objects for handle {}",
            to_hex(&material.handle)
        )));
    }

    let update_result = sqlx::query!(
        r#"
        UPDATE ciphertext_digest
         SET s3_format_version = $1,
             s3_migration_failure_count = 0,
             s3_migration_last_error = NULL,
             s3_migration_last_error_at = NULL
         WHERE handle = $2
           AND s3_format_version = $3
           AND ciphertext IS NOT DISTINCT FROM $4::bytea
           AND ciphertext128 IS NOT DISTINCT FROM $5::bytea
        "#,
        S3_FORMAT_VERSION_V1,
        &material.handle,
        CLEAN_OLD_S3_FORMAT_VERSION,
        material.row_ct64_digest.as_deref(),
        material.row_ct128_digest.as_deref(),
    )
    .execute(pool)
    .await?;

    if update_result.rows_affected() == 0 {
        info!(
            handle = to_hex(&material.handle),
            "Ciphertext handle was already migrated or changed while S3 migration was running"
        );
        return Ok(false);
    }

    if update_result.rows_affected() > 1 {
        return Err(ExecutionError::InternalError(format!(
            "expected to mark at most one ciphertext_digest row as migrated for handle {}, updated {} rows",
            to_hex(&material.handle),
            update_result.rows_affected()
        )));
    }

    info!(
        handle = to_hex(&material.handle),
        s3_format_version = S3_FORMAT_VERSION_V1,
        "Migrated ciphertext handle to new S3 format"
    );

    Ok(true)
}

pub(crate) async fn prepare_migration_material(
    pool: &PgPool,
    row: MigrationRow,
    signer: &CoproSigner,
) -> Result<MigrationMaterial, ExecutionError> {
    let row_ct64_digest = row.ciphertext.clone();
    let row_ct128_digest = row.ciphertext128.clone();
    let mut has_ct64 = row_ct64_digest.is_some();
    let has_ct128 = row_ct128_digest.is_some();

    let ct64_digest = match row.ciphertext {
        Some(digest) => digest,
        None => {
            let bytes = fetch_ct64_bytes_from_db(pool, &row.handle, None)
                .await?
                .ok_or_else(|| {
                    ExecutionError::MissingCiphertext64(format!(
                        "missing ct64 digest and DB ciphertext for handle {}",
                        to_hex(&row.handle),
                    ))
                })?;
            let digest = compute_digest(&bytes);
            has_ct64 = true;
            digest
        }
    };

    let ct128_digest = row
        .ciphertext128
        .unwrap_or_else(|| NO_SNS_CIPHERTEXT_DIGEST.to_vec());

    let ct128_format =
        Ciphertext128Format::from_i16(row.ciphertext128_format).ok_or_else(|| {
            ExecutionError::InvalidCiphertext128Format(format!(
                "invalid ciphertext128_format {} for handle {}",
                row.ciphertext128_format,
                to_hex(&row.handle),
            ))
        })?;

    let attestation = build_attestation(
        &row.handle,
        &row.key_id_gw,
        &ct64_digest,
        &ct128_digest,
        attestation_format(ct128_format)?,
        signer,
    )
    .await?;

    let metadata = S3MigrationMetadata {
        attestation_json: serde_json::to_string(&attestation)
            .map_err(|err| ExecutionError::ConversionError(err.into()))?,
        key_id: hex::encode(&row.key_id_gw),
        transaction_id: hex::encode(row.transaction_id.as_deref().unwrap_or_default()),
        signer: signer.address().to_string(),
    };

    Ok(MigrationMaterial {
        handle: row.handle,
        key_id_gw: row.key_id_gw,
        row_ct64_digest,
        row_ct128_digest,
        ct64_digest,
        ct128_digest,
        has_ct64,
        has_ct128,
        ct128_format,
        signer: signer.address(),
        metadata,
    })
}

async fn migrate_ct64_object(
    pool: &PgPool,
    client: &Client,
    s3: &S3Config,
    material: &MigrationMaterial,
) -> Result<(), ExecutionError> {
    if !material.has_ct64 {
        return Ok(());
    }

    let key = current_s3_ciphertext_key(&material.handle);
    if object_has_current_attestation(
        client,
        &s3.bucket_ct64,
        &key,
        CiphertextKind::Ct64,
        material,
    )
    .await?
    {
        return Ok(());
    }

    let source = CopySourceCandidate {
        key: legacy_s3_ciphertext_key(&material.ct64_digest),
    };

    if try_copy_existing_object(
        client,
        &s3.bucket_ct64,
        &source,
        &key,
        "ct64_compressed",
        CiphertextKind::Ct64,
        &material.ct64_digest,
        material,
    )
    .await?
    {
        return Ok(());
    }

    let bytes = fetch_ct64_bytes_from_db(pool, &material.handle, Some(&material.ct64_digest))
        .await?
        .unwrap_or_default();

    let bytes = if bytes.is_empty() {
        download_existing_object(
            client,
            &s3.bucket_ct64,
            &[source],
            CiphertextKind::Ct64,
            &material.ct64_digest,
        )
        .await?
        .ok_or_else(|| {
            ExecutionError::MissingCiphertext64(format!(
                "missing ct64 object for handle {}",
                to_hex(&material.handle)
            ))
        })?
    } else {
        bytes
    };

    put_object_with_metadata(
        client,
        &s3.bucket_ct64,
        &key,
        "ct64_compressed",
        material,
        bytes,
    )
    .await
}

async fn migrate_ct128_object(
    pool: &PgPool,
    client: &Client,
    s3: &S3Config,
    material: &MigrationMaterial,
) -> Result<(), ExecutionError> {
    if !material.has_ct128 {
        return Ok(());
    }

    let key = current_s3_ciphertext_key(&material.handle);
    let legacy_key = legacy_s3_ciphertext_key(&material.ct128_digest);
    let digest_key = hex::encode(&material.ct128_digest);
    let ct_format = material.ct128_format.to_string();

    let key_is_current = object_has_current_attestation(
        client,
        &s3.bucket_ct128,
        &key,
        CiphertextKind::Ct128,
        material,
    )
    .await?;
    let digest_key_is_current = object_has_current_attestation(
        client,
        &s3.bucket_ct128,
        &digest_key,
        CiphertextKind::Ct128,
        material,
    )
    .await?;

    if key_is_current && digest_key_is_current {
        return Ok(());
    }

    if !key_is_current {
        let sources = vec![
            CopySourceCandidate {
                key: digest_key.clone(),
            },
            CopySourceCandidate { key: legacy_key },
        ];

        if !try_copy_any_existing_object(
            client,
            &s3.bucket_ct128,
            &sources,
            &key,
            &ct_format,
            CiphertextKind::Ct128,
            &material.ct128_digest,
            material,
        )
        .await?
        {
            let bytes =
                fetch_ct128_bytes_from_db(pool, &material.handle, Some(&material.ct128_digest))
                    .await?
                    .ok_or_else(|| {
                        ExecutionError::MissingCiphertext128(format!(
                            "missing ct128 object for handle {}",
                            to_hex(&material.handle)
                        ))
                    })?;

            put_object_with_metadata(client, &s3.bucket_ct128, &key, &ct_format, material, bytes)
                .await?;
        }
    }

    if !digest_key_is_current {
        let source = CopySourceCandidate { key };
        if !try_copy_existing_object(
            client,
            &s3.bucket_ct128,
            &source,
            &digest_key,
            &ct_format,
            CiphertextKind::Ct128,
            &material.ct128_digest,
            material,
        )
        .await?
        {
            return Err(ExecutionError::S3TransientError(format!(
                "failed to create ct128 digest-key object for handle {}",
                to_hex(&material.handle)
            )));
        }
    }

    Ok(())
}

async fn migrated_objects_are_current(
    client: &Client,
    s3: &S3Config,
    material: &MigrationMaterial,
) -> Result<bool, ExecutionError> {
    let key = current_s3_ciphertext_key(&material.handle);

    if material.has_ct64
        && !object_has_current_attestation(
            client,
            &s3.bucket_ct64,
            &key,
            CiphertextKind::Ct64,
            material,
        )
        .await?
    {
        return Ok(false);
    }

    if material.has_ct128 {
        let digest_key = hex::encode(&material.ct128_digest);
        let key_is_current = object_has_current_attestation(
            client,
            &s3.bucket_ct128,
            &key,
            CiphertextKind::Ct128,
            material,
        )
        .await?;
        let digest_key_is_current = object_has_current_attestation(
            client,
            &s3.bucket_ct128,
            &digest_key,
            CiphertextKind::Ct128,
            material,
        )
        .await?;

        if !key_is_current || !digest_key_is_current {
            return Ok(false);
        }
    }

    Ok(true)
}

#[allow(clippy::too_many_arguments)]
async fn try_copy_any_existing_object(
    client: &Client,
    bucket: &str,
    sources: &[CopySourceCandidate],
    destination_key: &str,
    ct_format: &str,
    kind: CiphertextKind,
    expected_digest: &[u8],
    material: &MigrationMaterial,
) -> Result<bool, ExecutionError> {
    for source in sources {
        if try_copy_existing_object(
            client,
            bucket,
            source,
            destination_key,
            ct_format,
            kind,
            expected_digest,
            material,
        )
        .await?
        {
            return Ok(true);
        }
    }

    Ok(false)
}

#[allow(clippy::too_many_arguments)]
async fn try_copy_existing_object(
    client: &Client,
    bucket: &str,
    source: &CopySourceCandidate,
    destination_key: &str,
    ct_format: &str,
    kind: CiphertextKind,
    expected_digest: &[u8],
    material: &MigrationMaterial,
) -> Result<bool, ExecutionError> {
    if !object_body_matches_expected(client, bucket, &source.key, kind, expected_digest).await? {
        warn!(
            bucket,
            source_key = source.key,
            destination_key,
            "Skipping direct S3 copy because source bytes do not match expected digest"
        );
        return Ok(false);
    }

    copy_object_with_metadata(
        client,
        bucket,
        &source.key,
        destination_key,
        ct_format,
        material,
    )
    .await?;

    Ok(true)
}

pub(crate) async fn download_existing_object(
    client: &Client,
    bucket: &str,
    sources: &[CopySourceCandidate],
    kind: CiphertextKind,
    expected_digest: &[u8],
) -> Result<Option<Vec<u8>>, ExecutionError> {
    for source in sources {
        let Some(object) = get_object_if_exists(client, bucket, &source.key).await? else {
            continue;
        };

        let digest = compute_digest(&object.bytes);
        if digest == expected_digest {
            return Ok(Some(object.bytes));
        }

        warn!(
            bucket,
            source_key = source.key,
            ?kind,
            expected_digest = hex::encode(expected_digest),
            actual_digest = hex::encode(digest),
            "Skipping downloaded S3 object because digest does not match"
        );
    }

    Ok(None)
}

pub(crate) async fn object_has_current_attestation(
    client: &Client,
    bucket: &str,
    key: &str,
    kind: CiphertextKind,
    material: &MigrationMaterial,
) -> Result<bool, ExecutionError> {
    let Some(object) = get_object_if_exists(client, bucket, key).await? else {
        return Ok(false);
    };

    if !metadata_matches_expected(&object.metadata, kind, material) {
        return Ok(false);
    }

    Ok(object_bytes_match_expected_digest(
        &object.bytes,
        bucket,
        key,
        kind,
        expected_digest_for_kind(kind, material),
    ))
}

pub(crate) async fn object_body_matches_expected(
    client: &Client,
    bucket: &str,
    key: &str,
    kind: CiphertextKind,
    expected_digest: &[u8],
) -> Result<bool, ExecutionError> {
    let Some(object) = get_object_if_exists(client, bucket, key).await? else {
        return Ok(false);
    };

    Ok(object_bytes_match_expected_digest(
        &object.bytes,
        bucket,
        key,
        kind,
        expected_digest,
    ))
}

fn object_bytes_match_expected_digest(
    bytes: &[u8],
    bucket: &str,
    key: &str,
    kind: CiphertextKind,
    expected_digest: &[u8],
) -> bool {
    let digest = compute_digest(bytes);
    if digest == expected_digest {
        return true;
    }

    warn!(
        bucket,
        key,
        ?kind,
        expected_digest = hex::encode(expected_digest),
        actual_digest = hex::encode(digest),
        "S3 object bytes do not match expected digest"
    );
    false
}

fn expected_digest_for_kind(kind: CiphertextKind, material: &MigrationMaterial) -> &[u8] {
    match kind {
        CiphertextKind::Ct64 => &material.ct64_digest,
        CiphertextKind::Ct128 => &material.ct128_digest,
    }
}

fn metadata_matches_expected(
    metadata: &HashMap<String, String>,
    kind: CiphertextKind,
    material: &MigrationMaterial,
) -> bool {
    let Some(attestation_json) = metadata_get(metadata, S3_METADATA_ATTESTATION_KEY) else {
        return false;
    };
    let Ok(attestation) = serde_json::from_str::<CiphertextAttestation>(attestation_json) else {
        return false;
    };

    let Ok(ct64_digest) = b256_from_bytes("ciphertext digest", &material.ct64_digest) else {
        return false;
    };
    let Ok(ct128_digest) = b256_from_bytes("sns ciphertext digest", &material.ct128_digest) else {
        return false;
    };
    let Ok(handle) = b256_from_bytes("handle", &material.handle) else {
        return false;
    };
    let Ok(key_id) = u256_from_bytes("key_id_gw", &material.key_id_gw) else {
        return false;
    };

    if attestation.version != Version::V1
        || attestation.key_id != key_id
        || attestation.ciphertext_digest != ct64_digest
        || attestation.sns_ciphertext_digest != ct128_digest
        || attestation.signer != material.signer
    {
        return false;
    }

    if attestation
        .verify(handle, COPROCESSOR_CONTEXT_ID_1)
        .is_err()
    {
        return false;
    }

    if matches!(kind, CiphertextKind::Ct128) {
        let Ok(format) = attestation_format(material.ct128_format) else {
            return false;
        };
        if attestation.format != format {
            return false;
        }

        let expected_format = material.ct128_format.to_string();
        if metadata_get(metadata, "Ct-Format").map(String::as_str) != Some(expected_format.as_str())
        {
            return false;
        }
    }

    true
}

async fn get_object_if_exists(
    client: &Client,
    bucket: &str,
    key: &str,
) -> Result<Option<ExistingS3Object>, ExecutionError> {
    let output = match client.get_object().bucket(bucket).key(key).send().await {
        Ok(output) => output,
        Err(SdkError::ServiceError(err)) if err.err().is_no_such_key() => {
            return Ok(None);
        }
        Err(err) => return Err(ExecutionError::S3TransientError(err.to_string())),
    };

    let metadata = output.metadata().cloned().unwrap_or_default();
    let bytes = output
        .body
        .collect()
        .await
        .map_err(|err| ExecutionError::S3TransientError(err.to_string()))?;

    Ok(Some(ExistingS3Object {
        metadata,
        bytes: bytes.into_bytes().to_vec(),
    }))
}

async fn copy_object_with_metadata(
    client: &Client,
    bucket: &str,
    source_key: &str,
    destination_key: &str,
    ct_format: &str,
    material: &MigrationMaterial,
) -> Result<(), ExecutionError> {
    let copy_source = format!("{bucket}/{source_key}");
    client
        .copy_object()
        .copy_source(copy_source)
        .bucket(bucket)
        .key(destination_key)
        .metadata_directive(MetadataDirective::Replace)
        .metadata("Ct-Format", ct_format)
        .metadata("Uploaded-By", "sns-worker")
        .metadata(
            S3_METADATA_ATTESTATION_KEY,
            &material.metadata.attestation_json,
        )
        .metadata("Key-Id", &material.metadata.key_id)
        .metadata("Transaction-Id", &material.metadata.transaction_id)
        .metadata("Signer", &material.metadata.signer)
        .send()
        .await
        .map_err(|err| ExecutionError::S3TransientError(err.to_string()))?;

    Ok(())
}

async fn put_object_with_metadata(
    client: &Client,
    bucket: &str,
    key: &str,
    ct_format: &str,
    material: &MigrationMaterial,
    bytes: Vec<u8>,
) -> Result<(), ExecutionError> {
    client
        .put_object()
        .bucket(bucket)
        .key(key)
        .metadata("Ct-Format", ct_format)
        .metadata("Uploaded-By", "sns-worker")
        .metadata(
            S3_METADATA_ATTESTATION_KEY,
            &material.metadata.attestation_json,
        )
        .metadata("Key-Id", &material.metadata.key_id)
        .metadata("Transaction-Id", &material.metadata.transaction_id)
        .metadata("Signer", &material.metadata.signer)
        .body(ByteStream::from(bytes))
        .send()
        .await
        .map_err(|err| ExecutionError::S3TransientError(err.to_string()))?;

    Ok(())
}

pub(crate) async fn fetch_ct64_bytes_from_db(
    pool: &PgPool,
    handle: &[u8],
    expected_digest: Option<&[u8]>,
) -> Result<Option<Vec<u8>>, ExecutionError> {
    let rows = sqlx::query!(
        "SELECT ciphertext FROM ciphertexts WHERE handle = $1",
        handle,
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let bytes = row.ciphertext;
        if expected_digest.is_none_or(|expected| compute_digest(&bytes) == expected) {
            return Ok(Some(bytes));
        }
    }

    Ok(None)
}

pub(crate) async fn fetch_ct128_bytes_from_db(
    pool: &PgPool,
    handle: &[u8],
    expected_digest: Option<&[u8]>,
) -> Result<Option<Vec<u8>>, ExecutionError> {
    let rows = sqlx::query!(
        r#"
        SELECT ciphertext AS "ciphertext!"
         FROM ciphertexts128
         WHERE handle = $1
           AND ciphertext IS NOT NULL
        "#,
        handle,
    )
    .fetch_all(pool)
    .await?;

    for row in rows {
        let bytes = row.ciphertext;
        if expected_digest.is_none_or(|expected| compute_digest(&bytes) == expected) {
            return Ok(Some(bytes));
        }
    }

    Ok(None)
}

async fn build_attestation(
    handle: &[u8],
    key_id_gw: &[u8],
    ct64_digest: &[u8],
    ct128_digest: &[u8],
    format: CiphertextFormat,
    signer: &CoproSigner,
) -> Result<CiphertextAttestation, ExecutionError> {
    let payload = CiphertextAttestationPayload::new(
        Version::V1,
        b256_from_bytes("handle", handle)?,
        u256_from_bytes("key_id_gw", key_id_gw)?,
        COPROCESSOR_CONTEXT_ID_1,
        b256_from_bytes("ciphertext digest", ct64_digest)?,
        b256_from_bytes("sns ciphertext digest", ct128_digest)?,
        format,
    );
    let signature = signer
        .sign_hash(&payload.canonical_digest())
        .await
        .map_err(|err| ExecutionError::ConversionError(err.into()))?;

    Ok(CiphertextAttestation {
        version: payload.version,
        key_id: payload.key_id,
        ciphertext_digest: payload.ciphertext_digest,
        sns_ciphertext_digest: payload.sns_ciphertext_digest,
        format: payload.format,
        signer: signer.address(),
        signature: signature.as_bytes().to_vec(),
    })
}

fn b256_from_bytes(field: &str, bytes: &[u8]) -> Result<B256, ExecutionError> {
    let bytes: [u8; 32] = bytes.try_into().map_err(|_| {
        ExecutionError::InternalError(format!(
            "{field} must be 32 bytes for ciphertext attestation, got {}",
            bytes.len()
        ))
    })?;
    Ok(B256::from(bytes))
}

fn u256_from_bytes(field: &str, bytes: &[u8]) -> Result<U256, ExecutionError> {
    if bytes.len() > 32 {
        return Err(ExecutionError::InternalError(format!(
            "{field} must be at most 32 bytes for ciphertext attestation, got {}",
            bytes.len()
        )));
    }
    Ok(U256::from_be_slice(bytes))
}

fn attestation_format(format: Ciphertext128Format) -> Result<CiphertextFormat, ExecutionError> {
    match format {
        Ciphertext128Format::UncompressedOnCpu => Ok(CiphertextFormat::UncompressedOnCpu),
        Ciphertext128Format::CompressedOnCpu => Ok(CiphertextFormat::CompressedOnCpu),
        Ciphertext128Format::UncompressedOnGpu => Ok(CiphertextFormat::UncompressedOnGpu),
        Ciphertext128Format::CompressedOnGpu => Ok(CiphertextFormat::CompressedOnGpu),
        Ciphertext128Format::Unknown => Err(ExecutionError::InvalidCiphertext128Format(
            "cannot build ciphertext attestation with unknown ct128 format".to_owned(),
        )),
    }
}

pub(crate) fn current_s3_ciphertext_key(handle: &[u8]) -> String {
    s3_ciphertext_key(handle, COPROCESSOR_CONTEXT_ID_1)
}

pub(crate) fn legacy_s3_ciphertext_key(digest: &[u8]) -> String {
    hex::encode(digest)
}

fn metadata_get<'a>(metadata: &'a HashMap<String, String>, key: &str) -> Option<&'a String> {
    metadata.get(key).or_else(|| {
        metadata
            .iter()
            .find(|(k, _)| k.eq_ignore_ascii_case(key))
            .map(|(_, v)| v)
    })
}
