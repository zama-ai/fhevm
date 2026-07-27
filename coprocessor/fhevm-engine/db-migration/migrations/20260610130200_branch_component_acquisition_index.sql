-- Branch workers acquire one ingestion-time same-block component per
-- dependence_chain_id and fetch all non-error rows in that component, including
-- already completed same-block producers needed for in-memory forwarding.
CREATE INDEX IF NOT EXISTS idx_computations_branch_component_rows
ON computations_branch (dependence_chain_id, schedule_order)
WHERE is_error = false;
