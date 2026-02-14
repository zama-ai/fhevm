use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, PgPool};
use tracing::info;

use crate::database::ingest::{
    AllowedHandleInsert, BlockValidInsert, ComputationInsert, CursorUpdate, IngestActions,
    PbsComputationInsert,
};

#[derive(Clone)]
pub struct Database {
    pool: Option<PgPool>,
    pub host_chain_id: i64,
    pub tenant_id: i32,
}

#[derive(Default, Debug, Clone)]
pub struct ApplyStats {
    pub inserted_computations: usize,
    pub inserted_allowed_handles: usize,
    pub inserted_pbs_computations: usize,
    pub inserted_blocks: usize,
    pub cursor_updated: bool,
}

impl Database {
    pub async fn connect(database_url: &str, host_chain_id: i64, tenant_id: i32) -> Result<Self> {
        let pool = PgPoolOptions::new()
            .max_connections(10)
            .connect(database_url)
            .await?;
        Ok(Self {
            pool: Some(pool),
            host_chain_id,
            tenant_id,
        })
    }

    pub fn new_dry_run(host_chain_id: i64, tenant_id: i32) -> Self {
        Self {
            pool: None,
            host_chain_id,
            tenant_id,
        }
    }

    pub async fn apply_actions(&mut self, actions: &IngestActions) -> Result<ApplyStats> {
        if self.pool.is_none() {
            info!(?actions, "dry-run: skipping SQL execution");
            return Ok(ApplyStats {
                inserted_computations: actions.computations.len(),
                inserted_allowed_handles: actions.allowed_handles.len(),
                inserted_pbs_computations: actions.pbs_computations.len(),
                inserted_blocks: actions.blocks_valid.len(),
                cursor_updated: actions.cursor_update.is_some(),
            });
        }

        let pool = self.pool.as_ref().expect("pool checked above");
        let mut stats = ApplyStats::default();
        let mut tx = pool.begin().await?;

        for computation in &actions.computations {
            stats.inserted_computations += apply_computation(&mut tx, computation).await?;
        }
        for allowed in &actions.allowed_handles {
            stats.inserted_allowed_handles += apply_allowed_handle(&mut tx, allowed).await?;
        }
        for pbs in &actions.pbs_computations {
            stats.inserted_pbs_computations += apply_pbs_computation(&mut tx, pbs).await?;
        }
        for block in &actions.blocks_valid {
            stats.inserted_blocks += apply_block_valid(&mut tx, block).await?;
        }
        if let Some(cursor) = &actions.cursor_update {
            stats.cursor_updated = apply_cursor_update(&mut tx, cursor).await?;
        }

        tx.commit().await?;
        Ok(stats)
    }

    pub async fn set_cursor(&mut self, last_caught_up_block: i64) -> Result<bool> {
        if self.pool.is_none() {
            info!(
                chain_id = self.host_chain_id,
                last_caught_up_block, "dry-run: skipping cursor SQL update"
            );
            return Ok(true);
        }

        let pool = self.pool.as_ref().expect("pool checked above");
        let mut tx = pool.begin().await?;
        let updated = apply_cursor_update(
            &mut tx,
            &CursorUpdate {
                chain_id: self.host_chain_id,
                last_caught_up_block,
            },
        )
        .await?;
        tx.commit().await?;
        Ok(updated)
    }
}

async fn apply_computation(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &ComputationInsert,
) -> Result<usize> {
    let result = sqlx::query(
        r#"
        INSERT INTO computations (
            tenant_id,
            output_handle,
            dependencies,
            fhe_operation,
            is_scalar,
            dependence_chain_id,
            transaction_id,
            is_allowed,
            created_at,
            schedule_order,
            is_completed
        )
        VALUES ($1, $2, $3, $4, $5, $6, $7, $8, NOW(), $9::timestamp, $10)
        ON CONFLICT (tenant_id, output_handle, transaction_id) DO NOTHING
        "#,
    )
    .bind(row.tenant_id)
    .bind(&row.output_handle)
    .bind(&row.dependencies)
    .bind(row.fhe_operation)
    .bind(row.is_scalar)
    .bind(&row.dependence_chain_id)
    .bind(&row.transaction_id)
    .bind(row.is_allowed)
    .bind(row.schedule_order)
    .bind(row.is_completed)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected() as usize)
}

async fn apply_allowed_handle(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &AllowedHandleInsert,
) -> Result<usize> {
    let inserted = sqlx::query(
        r#"
        INSERT INTO allowed_handles(
            tenant_id, handle, account_address, event_type, transaction_id
        )
        VALUES ($1, $2, $3, $4, $5)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(row.tenant_id)
    .bind(&row.handle)
    .bind(&row.account_address)
    .bind(row.event_type)
    .bind(&row.transaction_id)
    .execute(tx.as_mut())
    .await?;

    // ACL gate: once handle is allowed, unlock queued computations for this
    // tenant/output handle so tfhe-worker can dequeue them.
    let _ = sqlx::query(
        r#"
        UPDATE computations
        SET is_allowed = TRUE
        WHERE tenant_id = $1
          AND output_handle = $2
          AND is_completed = FALSE
          AND is_allowed = FALSE
        "#,
    )
    .bind(row.tenant_id)
    .bind(&row.handle)
    .execute(tx.as_mut())
    .await?;

    Ok(inserted.rows_affected() as usize)
}

async fn apply_pbs_computation(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &PbsComputationInsert,
) -> Result<usize> {
    let result = sqlx::query(
        r#"
        INSERT INTO pbs_computations(tenant_id, handle, transaction_id)
        VALUES ($1, $2, $3)
        ON CONFLICT DO NOTHING
        "#,
    )
    .bind(row.tenant_id)
    .bind(&row.handle)
    .bind(&row.transaction_id)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected() as usize)
}

async fn apply_block_valid(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &BlockValidInsert,
) -> Result<usize> {
    let result = sqlx::query(
        r#"
        INSERT INTO host_chain_blocks_valid (chain_id, block_hash, block_number)
        VALUES ($1, $2, $3)
        ON CONFLICT (chain_id, block_hash) DO NOTHING
        "#,
    )
    .bind(row.chain_id)
    .bind(&row.block_hash)
    .bind(row.block_number)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected() as usize)
}

async fn apply_cursor_update(
    tx: &mut sqlx::Transaction<'_, sqlx::Postgres>,
    row: &CursorUpdate,
) -> Result<bool> {
    let result = sqlx::query(
        r#"
        INSERT INTO host_listener_poller_state (chain_id, last_caught_up_block)
        VALUES ($1, $2)
        ON CONFLICT (chain_id) DO UPDATE
        SET last_caught_up_block = EXCLUDED.last_caught_up_block,
            updated_at = NOW()
        "#,
    )
    .bind(row.chain_id)
    .bind(row.last_caught_up_block)
    .execute(tx.as_mut())
    .await?;

    Ok(result.rows_affected() > 0)
}
