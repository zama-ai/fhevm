//! Typed queries over `lineage_state`, `lineage_events`, and `indexer_cursor`.
//!
//! All statements use `sqlx::query!`/`query_as!` so they are checked against the
//! committed offline cache (`SQLX_OFFLINE=true` at build time). Pubkeys/handles
//! are 32-byte `BYTEA`; subjects are `BYTEA[]` in insertion order.

use sqlx::{PgPool, Postgres, Transaction};
use zama_solana_acl::lineage::LineageEvent;

const EVENT_TYPE_ROTATION: i16 = 0;
const EVENT_TYPE_MARKED_PUBLIC: i16 = 1;

/// The persisted shadow state of one lineage, keyed by PDA.
#[derive(Debug, Clone)]
pub struct LineageState {
    pub pda: [u8; 32],
    pub value_key: Option<[u8; 32]>,
    pub current_handle: [u8; 32],
    pub current_subjects: Vec<[u8; 32]>,
    pub leaf_count: i64,
}

/// One reconstruction event row, decoded into its mode for proof-prefix selection.
#[derive(Debug, Clone)]
pub struct EventRow {
    pub event: LineageEvent,
}

/// Where and from which transaction an event is being appended.
#[derive(Debug, Clone, Copy)]
pub struct EventInsert<'a> {
    pub pda: &'a [u8; 32],
    /// 0-based ordinal within the lineage.
    pub event_index: i64,
    pub signature: &'a str,
    pub slot: i64,
}

#[derive(Clone)]
pub struct LineageRepo {
    pool: PgPool,
}

fn to_array(bytes: Vec<u8>) -> anyhow::Result<[u8; 32]> {
    <[u8; 32]>::try_from(bytes.as_slice())
        .map_err(|_| anyhow::anyhow!("expected 32-byte column, got {} bytes", bytes.len()))
}

fn subjects_to_arrays(rows: Vec<Vec<u8>>) -> anyhow::Result<Vec<[u8; 32]>> {
    rows.into_iter().map(to_array).collect()
}

impl LineageRepo {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }

    /// Reads a lineage's shadow state by PDA, or `None` if unseen.
    pub async fn get_state(&self, pda: &[u8; 32]) -> anyhow::Result<Option<LineageState>> {
        let row = sqlx::query!(
            r#"SELECT pda, value_key, current_handle, current_subjects, leaf_count
               FROM lineage_state WHERE pda = $1"#,
            &pda[..]
        )
        .fetch_optional(&self.pool)
        .await?;

        row.map(|r| {
            Ok(LineageState {
                pda: to_array(r.pda)?,
                value_key: r.value_key.map(to_array).transpose()?,
                current_handle: to_array(r.current_handle)?,
                current_subjects: subjects_to_arrays(r.current_subjects)?,
                leaf_count: r.leaf_count,
            })
        })
        .transpose()
    }

    /// Resolves a `value_key` (the API key) to its PDA via the initialize mapping.
    pub async fn pda_for_value_key(
        &self,
        value_key: &[u8; 32],
    ) -> anyhow::Result<Option<[u8; 32]>> {
        let row = sqlx::query!(
            r#"SELECT pda FROM lineage_state WHERE value_key = $1"#,
            &value_key[..]
        )
        .fetch_optional(&self.pool)
        .await?;
        row.map(|r| to_array(r.pda)).transpose()
    }

    /// Loads the ordered event list for a PDA, ascending by `event_index` — the
    /// exact order `zama_solana_acl::lineage::reconstruct` expects.
    pub async fn events_for_pda(&self, pda: &[u8; 32]) -> anyhow::Result<Vec<EventRow>> {
        let rows = sqlx::query!(
            r#"SELECT event_type, old_handle, subjects_snapshot, handle
               FROM lineage_events WHERE pda = $1 ORDER BY event_index ASC"#,
            &pda[..]
        )
        .fetch_all(&self.pool)
        .await?;

        rows.into_iter()
            .map(|r| {
                let event =
                    match r.event_type {
                        EVENT_TYPE_ROTATION => {
                            let old_handle = to_array(r.old_handle.ok_or_else(|| {
                                anyhow::anyhow!("rotation row missing old_handle")
                            })?)?;
                            let subjects =
                                subjects_to_arrays(r.subjects_snapshot.ok_or_else(|| {
                                    anyhow::anyhow!("rotation row missing subjects_snapshot")
                                })?)?;
                            LineageEvent::Rotation {
                                old_handle,
                                subjects_before_rotation: subjects,
                            }
                        }
                        EVENT_TYPE_MARKED_PUBLIC => {
                            let handle = to_array(r.handle.ok_or_else(|| {
                                anyhow::anyhow!("mark-public row missing handle")
                            })?)?;
                            LineageEvent::MarkedPublic { handle }
                        }
                        other => return Err(anyhow::anyhow!("unknown event_type {other}")),
                    };
                Ok(EventRow { event })
            })
            .collect()
    }

    /// Upserts a lineage's shadow state. Used on initialize (with `value_key`) and
    /// after each rotate/allow/mark applies (carrying the new handle/subjects/count).
    pub async fn upsert_state(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        state: &LineageState,
    ) -> anyhow::Result<()> {
        let subjects: Vec<&[u8]> = state.current_subjects.iter().map(|s| &s[..]).collect();
        sqlx::query!(
            r#"INSERT INTO lineage_state
                 (pda, value_key, current_handle, current_subjects, leaf_count, updated_at)
               VALUES ($1, $2, $3, $4, $5, NOW())
               ON CONFLICT (pda) DO UPDATE SET
                 value_key = COALESCE(lineage_state.value_key, EXCLUDED.value_key),
                 current_handle = EXCLUDED.current_handle,
                 current_subjects = EXCLUDED.current_subjects,
                 leaf_count = EXCLUDED.leaf_count,
                 updated_at = NOW()"#,
            &state.pda[..],
            state.value_key.as_ref().map(|v| &v[..]),
            &state.current_handle[..],
            &subjects as &[&[u8]],
            state.leaf_count,
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// Appends one `LineageEvent` at `locator`, dispatching on its kind. Idempotent:
    /// a duplicate re-delivery (same `pda`,`event_index`) is a no-op.
    pub async fn insert_event(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        locator: &EventInsert<'_>,
        event: &LineageEvent,
    ) -> anyhow::Result<()> {
        match event {
            LineageEvent::Rotation {
                old_handle,
                subjects_before_rotation,
            } => {
                let subjects: Vec<&[u8]> =
                    subjects_before_rotation.iter().map(|s| &s[..]).collect();
                sqlx::query!(
                    r#"INSERT INTO lineage_events
                         (pda, event_index, event_type, old_handle, subjects_snapshot, signature, slot)
                       VALUES ($1, $2, $3, $4, $5, $6, $7)
                       ON CONFLICT (pda, event_index) DO NOTHING"#,
                    &locator.pda[..],
                    locator.event_index,
                    EVENT_TYPE_ROTATION,
                    &old_handle[..],
                    &subjects as &[&[u8]],
                    locator.signature,
                    locator.slot,
                )
                .execute(&mut **tx)
                .await?;
            }
            LineageEvent::MarkedPublic { handle } => {
                sqlx::query!(
                    r#"INSERT INTO lineage_events
                         (pda, event_index, event_type, handle, signature, slot)
                       VALUES ($1, $2, $3, $4, $5, $6)
                       ON CONFLICT (pda, event_index) DO NOTHING"#,
                    &locator.pda[..],
                    locator.event_index,
                    EVENT_TYPE_MARKED_PUBLIC,
                    &handle[..],
                    locator.signature,
                    locator.slot,
                )
                .execute(&mut **tx)
                .await?;
            }
        }
        Ok(())
    }

    /// Count of events already recorded for a PDA — the next `event_index`.
    pub async fn event_count(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        pda: &[u8; 32],
    ) -> anyhow::Result<i64> {
        let row = sqlx::query!(
            r#"SELECT COUNT(*) AS "count!" FROM lineage_events WHERE pda = $1"#,
            &pda[..]
        )
        .fetch_one(&mut **tx)
        .await?;
        Ok(row.count)
    }

    /// Reads the global Carbon resume cursor.
    pub async fn get_cursor(&self) -> anyhow::Result<(String, i64)> {
        let row =
            sqlx::query!(r#"SELECT last_signature, last_slot FROM indexer_cursor WHERE id = 1"#)
                .fetch_one(&self.pool)
                .await?;
        Ok((row.last_signature, row.last_slot))
    }

    /// Advances the cursor inside the SAME transaction as the event insert, so a
    /// crash leaves no gap between persisted events and the resume point.
    ///
    /// Advances ONLY on a strictly newer slot (`last_slot < $2`). This is the
    /// guard for two distinct hazards, both stemming from Carbon delivering every
    /// instruction — including CPI children and every EV-ACL instruction of a
    /// multi-instruction transaction — as a separate `process()` call carrying the
    /// same `(signature, slot)`:
    ///   - Same-transaction regression: instructions of one transaction share a
    ///     slot, so the cursor never moves while a transaction is still being
    ///     processed; it only advances once an instruction from a *later* slot
    ///     arrives. A crash mid-transaction therefore resumes from the prior
    ///     transaction and re-delivers the whole transaction (idempotent via the
    ///     `UNIQUE(pda, event_index)` guard), never skipping a tail instruction.
    ///   - Out-of-order regression: Carbon fetches transactions with
    ///     `buffer_unordered`, so an older-slot instruction can be processed after
    ///     a newer one; the `last_slot < $2` predicate refuses to move the cursor
    ///     backward, so a restart never re-crawls everything newer than a regressed
    ///     value.
    pub async fn advance_cursor(
        &self,
        tx: &mut Transaction<'_, Postgres>,
        signature: &str,
        slot: i64,
    ) -> anyhow::Result<()> {
        sqlx::query!(
            r#"UPDATE indexer_cursor
               SET last_signature = $1, last_slot = $2, updated_at = NOW()
               WHERE id = 1 AND last_slot < $2"#,
            signature,
            slot,
        )
        .execute(&mut **tx)
        .await?;
        Ok(())
    }

    /// Finds the leaf index of the historical-access leaf for `(subject, handle)`.
    ///
    /// Walks the ordered events exactly as `reconstruct` lays them out: each
    /// Rotation contributes one leaf per `subjects_snapshot` entry (for its
    /// `old_handle`), each MarkedPublic contributes one leaf. Returns the first
    /// matching `(handle == old_handle, subject in snapshot)` leaf index.
    pub async fn leaf_for_subject_handle(
        &self,
        pda: &[u8; 32],
        subject: &[u8; 32],
        handle: &[u8; 32],
    ) -> anyhow::Result<Option<(u64, u64)>> {
        let events = self.events_for_pda(pda).await?;
        let mut leaf_index: u64 = 0;
        let mut found: Option<u64> = None;
        for row in &events {
            match &row.event {
                LineageEvent::Rotation {
                    old_handle,
                    subjects_before_rotation,
                } => {
                    for subj in subjects_before_rotation {
                        if found.is_none() && old_handle == handle && subj == subject {
                            found = Some(leaf_index);
                        }
                        leaf_index += 1;
                    }
                }
                LineageEvent::MarkedPublic { .. } => {
                    leaf_index += 1;
                }
            }
        }
        Ok(found.map(|idx| (idx, leaf_index)))
    }
}
