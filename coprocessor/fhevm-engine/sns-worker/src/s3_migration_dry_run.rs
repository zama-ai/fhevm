use aws_sdk_s3::Client;
use fhevm_engine_common::utils::to_hex;
use futures::{stream::FuturesUnordered, StreamExt};
use sqlx::PgPool;
use tracing::{info, warn};

use crate::{
    s3_migration::{
        count_failed_old_format_handles, count_pending_old_format_handles,
        current_s3_ciphertext_key, download_existing_object, fetch_ct128_bytes_from_db,
        fetch_ct64_bytes_from_db, legacy_s3_ciphertext_key, object_body_matches_expected,
        object_has_current_attestation, prepare_migration_material, CiphertextKind,
        CopySourceCandidate, MigrationMaterial, MigrationRow, S3MigrationConfig,
    },
    ExecutionError, CLEAN_OLD_S3_FORMAT_VERSION, S3_FORMAT_VERSION_V1,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum S3MigrationDryRunAction {
    Ct64AlreadyCurrent,
    Ct64WouldCopyLegacyToCurrent,
    Ct64WouldPutToCurrent,
    Ct128HandleKeyAlreadyCurrent,
    Ct128DigestKeyAlreadyCurrent,
    Ct128WouldCopyToHandleKey,
    Ct128WouldPutDbToHandleKey,
    Ct128WouldCopyHandleToDigestKey,
    WouldMarkDbMigrated,
}

#[derive(Debug, Default)]
struct S3MigrationDryRunHandlePlan {
    actions: Vec<S3MigrationDryRunAction>,
}

impl S3MigrationDryRunHandlePlan {
    fn record(&mut self, action: S3MigrationDryRunAction) {
        self.actions.push(action);
    }
}

#[derive(Debug, Default, Clone)]
struct S3MigrationDryRunReport {
    handles_scanned: u64,
    handles_planned: u64,
    handles_changed_or_migrated: u64,
    handles_failed: u64,
    ct64_already_current: u64,
    ct64_would_copy_legacy_to_current: u64,
    ct64_would_put_to_current: u64,
    ct128_handle_key_already_current: u64,
    ct128_digest_key_already_current: u64,
    ct128_would_copy_to_handle_key: u64,
    ct128_would_put_db_to_handle_key: u64,
    ct128_would_copy_handle_to_digest_key: u64,
    would_mark_db_migrated: u64,
}

impl S3MigrationDryRunReport {
    fn merge(&mut self, other: Self) {
        self.handles_scanned += other.handles_scanned;
        self.handles_planned += other.handles_planned;
        self.handles_changed_or_migrated += other.handles_changed_or_migrated;
        self.handles_failed += other.handles_failed;
        self.ct64_already_current += other.ct64_already_current;
        self.ct64_would_copy_legacy_to_current += other.ct64_would_copy_legacy_to_current;
        self.ct64_would_put_to_current += other.ct64_would_put_to_current;
        self.ct128_handle_key_already_current += other.ct128_handle_key_already_current;
        self.ct128_digest_key_already_current += other.ct128_digest_key_already_current;
        self.ct128_would_copy_to_handle_key += other.ct128_would_copy_to_handle_key;
        self.ct128_would_put_db_to_handle_key += other.ct128_would_put_db_to_handle_key;
        self.ct128_would_copy_handle_to_digest_key += other.ct128_would_copy_handle_to_digest_key;
        self.would_mark_db_migrated += other.would_mark_db_migrated;
    }

    fn record_plan(&mut self, plan: &S3MigrationDryRunHandlePlan) {
        self.handles_planned += 1;
        for action in &plan.actions {
            match action {
                S3MigrationDryRunAction::Ct64AlreadyCurrent => self.ct64_already_current += 1,
                S3MigrationDryRunAction::Ct64WouldCopyLegacyToCurrent => {
                    self.ct64_would_copy_legacy_to_current += 1
                }
                S3MigrationDryRunAction::Ct64WouldPutToCurrent => {
                    self.ct64_would_put_to_current += 1
                }
                S3MigrationDryRunAction::Ct128HandleKeyAlreadyCurrent => {
                    self.ct128_handle_key_already_current += 1
                }
                S3MigrationDryRunAction::Ct128DigestKeyAlreadyCurrent => {
                    self.ct128_digest_key_already_current += 1
                }
                S3MigrationDryRunAction::Ct128WouldCopyToHandleKey => {
                    self.ct128_would_copy_to_handle_key += 1
                }
                S3MigrationDryRunAction::Ct128WouldPutDbToHandleKey => {
                    self.ct128_would_put_db_to_handle_key += 1
                }
                S3MigrationDryRunAction::Ct128WouldCopyHandleToDigestKey => {
                    self.ct128_would_copy_handle_to_digest_key += 1
                }
                S3MigrationDryRunAction::WouldMarkDbMigrated => self.would_mark_db_migrated += 1,
            }
        }
    }
}

#[derive(Debug, Clone)]
struct S3MigrationDryRunCursor {
    s3_migration_failure_count: i32,
    handle: Vec<u8>,
}

#[derive(Debug, Clone, sqlx::FromRow)]
struct S3MigrationDryRunPageRow {
    handle: Vec<u8>,
    s3_migration_failure_count: i32,
}

pub(crate) async fn run_startup_migration_dry_run(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
) -> Result<(), ExecutionError> {
    migrate_s3_format_0_to_1_dry_run(config, pool, client).await
}

async fn migrate_s3_format_0_to_1_dry_run(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
) -> Result<(), ExecutionError> {
    let total = count_pending_old_format_handles(pool).await?;
    let already_failed = count_failed_old_format_handles(pool).await?;

    info!(
        handles_to_process = total,
        handles_with_recorded_failures = already_failed,
        from_s3_format_version = CLEAN_OLD_S3_FORMAT_VERSION,
        to_s3_format_version = S3_FORMAT_VERSION_V1,
        "Detected ciphertext handles for S3 format migration dry-run"
    );

    let mut cursor = None;
    let mut report = S3MigrationDryRunReport::default();
    loop {
        let page = fetch_old_format_handle_dry_run_page(config, pool, cursor.as_ref()).await?;
        if page.is_empty() {
            break;
        }

        report.handles_scanned += page.len() as u64;
        let handles = page
            .iter()
            .map(|row| row.handle.clone())
            .collect::<Vec<_>>();
        let batch_report = dry_run_handle_batch(config, pool, client, &handles).await?;
        report.merge(batch_report);

        let last = page.last().expect("non-empty dry-run page");
        cursor = Some(S3MigrationDryRunCursor {
            s3_migration_failure_count: last.s3_migration_failure_count,
            handle: last.handle.clone(),
        });

        info!(
            handles_scanned = report.handles_scanned,
            handles_planned = report.handles_planned,
            handles_failed = report.handles_failed,
            "S3 migration dry-run batch"
        );
    }

    info!(
        handles_scanned = report.handles_scanned,
        handles_planned = report.handles_planned,
        handles_changed_or_migrated = report.handles_changed_or_migrated,
        handles_failed = report.handles_failed,
        ct64_already_current = report.ct64_already_current,
        ct64_would_copy_legacy_to_current = report.ct64_would_copy_legacy_to_current,
        ct64_would_put_to_current = report.ct64_would_put_to_current,
        ct128_handle_key_already_current = report.ct128_handle_key_already_current,
        ct128_digest_key_already_current = report.ct128_digest_key_already_current,
        ct128_would_copy_to_handle_key = report.ct128_would_copy_to_handle_key,
        ct128_would_put_db_to_handle_key = report.ct128_would_put_db_to_handle_key,
        ct128_would_copy_handle_to_digest_key = report.ct128_would_copy_handle_to_digest_key,
        would_mark_db_migrated = report.would_mark_db_migrated,
        from_s3_format_version = CLEAN_OLD_S3_FORMAT_VERSION,
        to_s3_format_version = S3_FORMAT_VERSION_V1,
        "Finished S3 format migration dry-run"
    );

    Ok(())
}

async fn fetch_old_format_handle_dry_run_page(
    config: &S3MigrationConfig,
    pool: &PgPool,
    cursor: Option<&S3MigrationDryRunCursor>,
) -> Result<Vec<S3MigrationDryRunPageRow>, ExecutionError> {
    let batch_size = config.batch_size as i64;
    let rows = match cursor {
        Some(cursor) => {
            sqlx::query_as!(
                S3MigrationDryRunPageRow,
                r#"
                SELECT handle as "handle!",
                       s3_migration_failure_count as "s3_migration_failure_count!"
                 FROM ciphertext_digest
                 WHERE s3_format_version = $1
                   AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
                   AND (s3_migration_failure_count, handle) > ($2, $3)
                 ORDER BY s3_migration_failure_count, handle
                 LIMIT $4
                "#,
                CLEAN_OLD_S3_FORMAT_VERSION,
                cursor.s3_migration_failure_count,
                &cursor.handle,
                batch_size,
            )
            .fetch_all(pool)
            .await?
        }
        None => {
            sqlx::query_as!(
                S3MigrationDryRunPageRow,
                r#"
                SELECT handle as "handle!",
                       s3_migration_failure_count as "s3_migration_failure_count!"
                 FROM ciphertext_digest
                 WHERE s3_format_version = $1
                   AND (ciphertext IS NOT NULL OR ciphertext128 IS NOT NULL)
                 ORDER BY s3_migration_failure_count, handle
                 LIMIT $2
                "#,
                CLEAN_OLD_S3_FORMAT_VERSION,
                batch_size,
            )
            .fetch_all(pool)
            .await?
        }
    };

    Ok(rows)
}

async fn dry_run_handle_batch(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
    handles: &[Vec<u8>],
) -> Result<S3MigrationDryRunReport, ExecutionError> {
    let mut report = S3MigrationDryRunReport::default();
    let mut tasks = FuturesUnordered::new();
    for handle in handles.iter() {
        tasks.push(async move {
            (
                handle.clone(),
                dry_run_handle_0_to_1(config, pool, client, handle).await,
            )
        });
    }

    while let Some((handle, result)) = tasks.next().await {
        let handle_hex = to_hex(&handle);
        match result {
            Ok(Some(plan)) => {
                info!(
                    handle = handle_hex,
                    actions = ?plan.actions,
                    "Planned S3 format migration for handle"
                );
                report.record_plan(&plan);
            }
            Ok(None) => {
                report.handles_changed_or_migrated += 1;
                info!(
                    handle = handle_hex,
                    "Ciphertext handle was already migrated or changed before S3 migration dry-run"
                );
            }
            Err(err) => {
                report.handles_failed += 1;
                warn!(
                    handle = handle_hex,
                    error = %err,
                    "S3 migration dry-run failed for handle"
                );
            }
        }
    }

    Ok(report)
}

async fn dry_run_handle_0_to_1(
    config: &S3MigrationConfig,
    pool: &PgPool,
    client: &Client,
    handle: &[u8],
) -> Result<Option<S3MigrationDryRunHandlePlan>, ExecutionError> {
    let Some(material) = fetch_migration_material(config, pool, handle).await? else {
        return Ok(None);
    };

    let mut plan = S3MigrationDryRunHandlePlan::default();
    plan_ct64_object(pool, client, config, &material, &mut plan).await?;
    plan_ct128_object(pool, client, config, &material, &mut plan).await?;
    plan.record(S3MigrationDryRunAction::WouldMarkDbMigrated);

    Ok(Some(plan))
}

async fn fetch_migration_material(
    config: &S3MigrationConfig,
    pool: &PgPool,
    handle: &[u8],
) -> Result<Option<MigrationMaterial>, ExecutionError> {
    let row = sqlx::query_as!(
        MigrationRow,
        r#"
        SELECT handle as "handle!",
               key_id_gw as "key_id_gw!",
               transaction_id,
               ciphertext,
               ciphertext128,
               ciphertext128_format as "ciphertext128_format!"
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
        return Ok(None);
    };

    Ok(Some(
        prepare_migration_material(pool, row, &config.signer).await?,
    ))
}

async fn plan_ct64_object(
    pool: &PgPool,
    client: &Client,
    config: &S3MigrationConfig,
    material: &MigrationMaterial,
    plan: &mut S3MigrationDryRunHandlePlan,
) -> Result<(), ExecutionError> {
    if !material.has_ct64 {
        return Ok(());
    }

    let key = current_s3_ciphertext_key(&material.handle);
    if object_has_current_attestation(
        client,
        &config.s3.bucket_ct64,
        &key,
        CiphertextKind::Ct64,
        material,
    )
    .await?
    {
        plan.record(S3MigrationDryRunAction::Ct64AlreadyCurrent);
        return Ok(());
    }

    let source = CopySourceCandidate {
        key: legacy_s3_ciphertext_key(&material.handle),
    };

    if object_body_matches_expected(
        client,
        &config.s3.bucket_ct64,
        &source.key,
        CiphertextKind::Ct64,
        &material.ct64_digest,
    )
    .await?
    {
        plan.record(S3MigrationDryRunAction::Ct64WouldCopyLegacyToCurrent);
        return Ok(());
    }

    let bytes = fetch_ct64_bytes_from_db(pool, &material.handle, Some(&material.ct64_digest))
        .await?
        .unwrap_or_default();

    if bytes.is_empty() {
        download_existing_object(
            client,
            &config.s3.bucket_ct64,
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
        })?;
    }

    plan.record(S3MigrationDryRunAction::Ct64WouldPutToCurrent);
    Ok(())
}

async fn plan_ct128_object(
    pool: &PgPool,
    client: &Client,
    config: &S3MigrationConfig,
    material: &MigrationMaterial,
    plan: &mut S3MigrationDryRunHandlePlan,
) -> Result<(), ExecutionError> {
    if !material.has_ct128 {
        return Ok(());
    }

    let key = current_s3_ciphertext_key(&material.handle);
    let legacy_key = legacy_s3_ciphertext_key(&material.handle);
    let digest_key = hex::encode(&material.ct128_digest);

    let key_is_current = object_has_current_attestation(
        client,
        &config.s3.bucket_ct128,
        &key,
        CiphertextKind::Ct128,
        material,
    )
    .await?;
    let digest_key_is_current = object_has_current_attestation(
        client,
        &config.s3.bucket_ct128,
        &digest_key,
        CiphertextKind::Ct128,
        material,
    )
    .await?;

    if key_is_current {
        plan.record(S3MigrationDryRunAction::Ct128HandleKeyAlreadyCurrent);
    }
    if digest_key_is_current {
        plan.record(S3MigrationDryRunAction::Ct128DigestKeyAlreadyCurrent);
    }
    if key_is_current && digest_key_is_current {
        return Ok(());
    }

    let mut handle_key_will_be_current = key_is_current;
    if !key_is_current {
        let sources = [digest_key.clone(), legacy_key];
        if object_source_matches(
            client,
            &config.s3.bucket_ct128,
            &sources,
            CiphertextKind::Ct128,
            &material.ct128_digest,
        )
        .await?
        {
            plan.record(S3MigrationDryRunAction::Ct128WouldCopyToHandleKey);
            handle_key_will_be_current = true;
        } else {
            fetch_ct128_bytes_from_db(pool, &material.handle, Some(&material.ct128_digest))
                .await?
                .ok_or_else(|| {
                    ExecutionError::MissingCiphertext128(format!(
                        "missing ct128 object for handle {}",
                        to_hex(&material.handle)
                    ))
                })?;
            plan.record(S3MigrationDryRunAction::Ct128WouldPutDbToHandleKey);
            handle_key_will_be_current = true;
        }
    }

    if !digest_key_is_current && handle_key_will_be_current {
        plan.record(S3MigrationDryRunAction::Ct128WouldCopyHandleToDigestKey);
    }

    Ok(())
}

async fn object_source_matches(
    client: &Client,
    bucket: &str,
    sources: &[String],
    kind: CiphertextKind,
    expected_digest: &[u8],
) -> Result<bool, ExecutionError> {
    for source in sources {
        if object_body_matches_expected(client, bucket, source, kind, expected_digest).await? {
            return Ok(true);
        }
    }

    Ok(false)
}
