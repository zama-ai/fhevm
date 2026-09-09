-- Covers the worker's ready-work acquisition query: select allowed pending
-- computations for an acquired DCID in schedule order, then expand the
-- matching transaction.
--
-- The partial predicate deliberately omits `is_error = false`. The work
-- window selects
--     is_completed = false
--     AND (is_error = false OR error_message LIKE '%ExecutionPanic%')
--     AND is_allowed = true
-- and that disjunction does NOT imply `is_error = false`, so the planner
-- cannot prove an index carrying it applicable (`predicate_implied_by`
-- fails) and falls back to a sequential scan. The pre-existing
-- idx_computations_dependence_chain has exactly that defect, which is why
-- this index looked necessary in the first place.
--
-- `transaction_id` is deliberately not a third key column: with
-- `dependence_chain_id = ANY($1)` over several DCIDs the scan cannot return
-- globally ordered rows, so an explicit Sort is planned either way and the
-- extra column would be pure write amplification.
--
-- This is a plain in-transaction CREATE INDEX on purpose: the migration
-- images and the coprocessor CI jobs pin sqlx-cli 0.7.2, whose Migration
-- type has no `no_tx` field at all, so a `-- no-transaction` directive is
-- silently ignored and CONCURRENTLY would fail with 25001. On a populated
-- database the concurrent build is done ahead of this migration by
-- `precreate_index` in db-migration/initialize_db.sh — the repo's existing
-- pattern for exactly this problem — after which this statement no-ops.
CREATE INDEX IF NOT EXISTS idx_computations_pending_dcid_schedule
ON computations (dependence_chain_id, schedule_order)
WHERE is_completed = false AND is_allowed = true;
