-- Wave-2 branch workers acquire one ingestion-time same-block branch context per
-- dependence_chain_id and fetch all non-error rows in that context, including
-- already completed same-block producers needed for in-memory forwarding.
DROP INDEX IF EXISTS idx_computations_branch_component_rows;

CREATE INDEX idx_computations_branch_component_rows
ON computations_branch (
    dependence_chain_id,
    schedule_order,
    host_chain_id,
    block_number,
    producer_block_hash
)
WHERE is_error = false;
