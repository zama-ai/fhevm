ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT '\x00'::BYTEA,
  ADD COLUMN IF NOT EXISTS block_number BIGINT NOT NULL DEFAULT 0;

ALTER TABLE computations ALTER COLUMN block_hash DROP DEFAULT;
ALTER TABLE computations ALTER COLUMN block_number DROP DEFAULT;

ALTER TABLE computations
    DROP CONSTRAINT computations_pkey;

ALTER TABLE computations
    ADD PRIMARY KEY (tenant_id, output_handle, transaction_id, block_hash);


-- fully identify a host chain transaction
CREATE INDEX IF NOT EXISTS idx_computations_transaction
  ON computations (transaction_id, block_hash);

-- For next release
-- DROP INDEX IF EXISTS idx_computations_transaction_id;
