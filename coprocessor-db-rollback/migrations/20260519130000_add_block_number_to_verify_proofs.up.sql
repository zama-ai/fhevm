-- Add block_number column to verify_proofs so that gw-listener can record
-- the block where the ZK proof request log event was emitted. The
-- zkproof-worker uses this when materialising computation rows and
-- state_hash entries for the produced ciphertexts.
--
-- Rows inserted before this migration will have NULL block_number.

ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL DEFAULT NULL;
