-- Add block_number and host_chain_id where missing to tables that
-- the state revert script needs to filter by block directly.
--
-- Rows inserted before this migration will have NULL block_number
-- and NULL host_chain_id. State revert will only work for data
-- inserted after this migration.

ALTER TABLE computations
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_computations_block_number
ON computations (host_chain_id, block_number);

ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_pbs_computations_block_number
ON pbs_computations (host_chain_id, block_number);

ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS host_chain_id BIGINT NULL DEFAULT NULL;

ALTER TABLE allowed_handles
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL DEFAULT NULL;

CREATE INDEX IF NOT EXISTS idx_allowed_handles_block_number
ON allowed_handles (host_chain_id, block_number);