ALTER TABLE computations
  ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT '\x00'::BYTEA,
  ADD COLUMN IF NOT EXISTS block_number BIGINT NOT NULL DEFAULT 0;

-- For next release, we should remove the default values for block_hash and block_number.
-- ALTER TABLE computations ALTER COLUMN block_hash DROP DEFAULT;
-- ALTER TABLE computations ALTER COLUMN block_number DROP DEFAULT;
