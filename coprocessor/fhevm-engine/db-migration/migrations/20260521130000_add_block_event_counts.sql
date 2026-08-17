-- Per-block event tallies recorded by host-listener at ingest time.
--   fhe_event_count   = number of decoded TFHE contract events in the block
--   allow_event_count = number of decoded ACL contract events  in the block
-- Counts are written by `mark_block_as_valid` and are not updated on
-- pending → finalized transitions.
ALTER TABLE IF EXISTS host_chain_blocks_valid
    ADD COLUMN IF NOT EXISTS fhe_event_count   INTEGER NOT NULL DEFAULT 0 CHECK (fhe_event_count   >= 0),
    ADD COLUMN IF NOT EXISTS allow_event_count INTEGER NOT NULL DEFAULT 0 CHECK (allow_event_count >= 0);
