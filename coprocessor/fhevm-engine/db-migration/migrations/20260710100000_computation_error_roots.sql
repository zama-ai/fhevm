-- Terminal computation errors make every pending computation in their
-- same-block dependency closure terminal as well. Keep the originating
-- computation identity on derived errors so a future recovery path can clear
-- and reschedule exactly that closure.

ALTER TABLE computations_branch
ADD COLUMN IF NOT EXISTS error_root_output_handle BYTEA NULL,
ADD COLUMN IF NOT EXISTS error_root_transaction_id BYTEA NULL,
ADD COLUMN IF NOT EXISTS error_root_producer_block_hash BYTEA NULL;

ALTER TABLE computations_branch
DROP CONSTRAINT IF EXISTS computations_branch_error_root_complete_check;

ALTER TABLE computations_branch
ADD CONSTRAINT computations_branch_error_root_complete_check
CHECK (
    (
        error_root_output_handle IS NULL
        AND error_root_transaction_id IS NULL
        AND error_root_producer_block_hash IS NULL
    )
    OR
    (
        error_root_output_handle IS NOT NULL
        AND error_root_transaction_id IS NOT NULL
        AND error_root_producer_block_hash IS NOT NULL
    )
) NOT VALID;
