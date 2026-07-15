-- Wave-2 branch workers acquire one ingestion-time same-block branch context per
-- dependence_chain_id and fetch all non-error rows in that context, including
-- already completed same-block producers needed for in-memory forwarding.
--
-- On large online databases, pre-create this index concurrently under the same
-- name before applying this migration. The in-migration form is intentionally
-- transactional for SQLx migrator compatibility and becomes a metadata no-op
-- when the concurrent prerequisite has already run.
CREATE INDEX IF NOT EXISTS idx_computations_branch_component_rows_v2
ON computations_branch (
    dependence_chain_id,
    schedule_order,
    host_chain_id,
    block_number,
    producer_block_hash
)
WHERE is_error = false;

DROP INDEX IF EXISTS idx_computations_branch_component_rows;
