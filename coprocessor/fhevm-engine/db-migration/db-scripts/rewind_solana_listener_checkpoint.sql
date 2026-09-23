-- rewind_solana_listener_checkpoint.sql
-- Moves the Solana host listener's checkpoint back to a slot it already applied, so the
-- listener replays every later slot when it restarts. Pair it with
-- revert_coprocessor_db_state.sql at the same slot, which refuses to revert a Solana chain
-- whose checkpoint is still ahead: the listener would never re-ingest the deleted rows.
--
-- `slot` must be a slot that produced a block, and `block_hash` that block's hash in hex
-- (the base58 `blockhash` of `getBlock <slot>`, decoded). The listener checks the first
-- replayed block against it, as on any resume. The replay reads the provider's replay
-- window, so `slot` must still be inside it.
--
-- Recorded Solana leaves are kept: the replay recomputes them and requires them to match.
--
-- Usage:
--   psql -v slot=<SLOT> -v block_hash=<HEX> -f rewind_solana_listener_checkpoint.sql

\set ON_ERROR_STOP on

BEGIN;

CREATE TEMP TABLE _rewind AS
SELECT :'slot'::bigint AS slot, decode(:'block_hash', 'hex') AS block_hash;

DO $$
DECLARE
  _slot bigint;
  _block_hash bytea;
  _checkpoint bigint;
BEGIN
  SELECT slot, block_hash INTO _slot, _block_hash FROM _rewind;
  IF octet_length(_block_hash) <> 32 THEN
    RAISE EXCEPTION 'block_hash must be 32 bytes, got %', octet_length(_block_hash);
  END IF;
  SELECT slot INTO _checkpoint FROM solana_listener_checkpoint WHERE singleton = 1;
  IF _checkpoint IS NULL THEN
    RAISE EXCEPTION 'the Solana listener has no checkpoint to rewind';
  END IF;
  IF _slot > _checkpoint THEN
    RAISE EXCEPTION 'slot % is past the checkpoint at slot %', _slot, _checkpoint;
  END IF;
END $$;

UPDATE solana_listener_checkpoint
   SET slot = (SELECT slot FROM _rewind),
       block_hash = (SELECT block_hash FROM _rewind),
       updated_at = NOW()
 WHERE singleton = 1;

DROP TABLE _rewind;

COMMIT;
