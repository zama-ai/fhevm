//! Runs the coprocessor migration set into a named schema.
//!
//! Green builds its own schema from this set instead of copying `public`, and the
//! cutover applies what `public` is missing. One runner, so both come from the
//! same files.

use std::collections::HashMap;
use std::time::Instant;

use sqlx::{Executor, Postgres, Transaction};
use tracing::info;

use crate::{hex_encode, Error};

static MIGRATOR: sqlx::migrate::Migrator = sqlx::migrate!("../db-migration/migrations");

/// `initialize_db.sh` rewrites this checksum for a migration edited in place after
/// some databases applied it. Accept the old value so both runners agree.
const REWRITTEN_VERSION: i64 = 20260616120000;
const REWRITTEN_CHECKSUM: &str = "7f80a69bd35610c02950bbc253ac1c34c006217d242f17cd23f23e4fb990d94009587c4fc3fbd8b5ba042f17f0d09810";

const LEDGER_DDL: &str = "CREATE TABLE IF NOT EXISTS _sqlx_migrations (
     version BIGINT PRIMARY KEY,
     description TEXT NOT NULL,
     installed_on TIMESTAMPTZ NOT NULL DEFAULT now(),
     success BOOLEAN NOT NULL,
     checksum BYTEA NOT NULL,
     execution_time BIGINT NOT NULL
 )";

fn quote_ident(name: &str) -> String {
    format!("\"{}\"", name.replace('"', "\"\""))
}

/// Apply every migration `schema` is missing, on `tx`. `up_to` stops at a version.
/// Returns the versions applied, oldest first.
///
/// Ledger rows match what the sqlx CLI writes, so a later `sqlx migrate run` is a
/// no-op.
pub async fn apply_migrations(
    tx: &mut Transaction<'_, Postgres>,
    schema: &str,
    up_to: Option<i64>,
) -> Result<Vec<i64>, Error> {
    let set_path = format!("SET LOCAL search_path TO {}", quote_ident(schema));
    (&mut **tx).execute(set_path.as_str()).await?;
    (&mut **tx).execute(LEDGER_DDL).await?;

    let applied: HashMap<i64, Vec<u8>> =
        sqlx::query_as::<_, (i64, Vec<u8>)>("SELECT version, checksum FROM _sqlx_migrations")
            .fetch_all(&mut **tx)
            .await?
            .into_iter()
            .collect();

    let mut new_versions = Vec::new();
    for migration in MIGRATOR.iter() {
        if up_to.is_some_and(|limit| migration.version > limit) {
            continue;
        }
        if let Some(recorded) = applied.get(&migration.version) {
            check_checksum(migration.version, recorded, &migration.checksum)?;
            continue;
        }

        let started = Instant::now();
        (&mut **tx).execute(&*migration.sql).await?;
        let execution_time = started.elapsed().as_nanos() as i64;

        sqlx::query(
            "INSERT INTO _sqlx_migrations
                 (version, description, success, checksum, execution_time)
             VALUES ($1, $2, TRUE, $3, $4)",
        )
        .bind(migration.version)
        .bind(migration.description.as_ref())
        .bind(migration.checksum.as_ref())
        .bind(execution_time)
        .execute(&mut **tx)
        .await?;

        new_versions.push(migration.version);
    }

    if !new_versions.is_empty() {
        info!(
            schema,
            count = new_versions.len(),
            latest = new_versions.last(),
            "applied migrations"
        );
    }
    Ok(new_versions)
}

/// A different checksum means the schema was built from different SQL than this
/// binary carries. Fail here, not as a merge error at cutover.
fn check_checksum(version: i64, recorded: &[u8], expected: &[u8]) -> Result<(), Error> {
    if recorded == expected {
        return Ok(());
    }
    if version == REWRITTEN_VERSION && hex_encode(recorded) == REWRITTEN_CHECKSUM {
        return Ok(());
    }
    Err(Error::Migration(format!(
        "migration {version} was applied from different SQL than this binary carries \
         (recorded {}, expected {})",
        hex_encode(recorded),
        hex_encode(expected)
    )))
}
