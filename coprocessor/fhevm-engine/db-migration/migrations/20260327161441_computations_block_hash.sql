ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT '\x00'::BYTEA;
-- Default value ensures a basic rollback does not fail, but it is not ideal.
-- We should remove the default value in the next release.

-- Alter the primary key to include block_hash
ALTER TABLE computations DROP CONSTRAINT computations_pkey;
ALTER TABLE computations
  ADD CONSTRAINT computations_pkey
  PRIMARY KEY (tenant_id, output_handle, transaction_id, block_hash);
-- Note: this is non reversible except manually:
--  * dropping the primary key
--  * either removing all reorg out blocks or deduplicating handle with different block_hash but same tx_id

-- Alter the unique primary key to include block_hash
DROP INDEX IF EXISTS idx_computations_no_tenant;
CREATE UNIQUE INDEX IF NOT EXISTS idx_computations_no_tenant
    ON computations (output_handle, transaction_id, block_hash);
-- Note: this is non reversible except manually:
--  * dropping the primary key
--  * either removing all reorg out blocks or deduplicating handle with different block_hash but same tx_id

-- For next release, we should remove the default values for block_hash and block_number.
-- ALTER TABLE computations ALTER COLUMN block_hash DROP DEFAULT;
