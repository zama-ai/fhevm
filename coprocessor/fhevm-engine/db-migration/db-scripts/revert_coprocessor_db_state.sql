-- revert_coprocessor_db_state.sql
-- Reverts all coprocessor state for a given chain to a given block number.
-- All data from blocks strictly greater than to_block_number will be deleted.
--
-- Operates on the branch-keyed tables that the runtime writes to:
-- computations_branch, pbs_computations_branch, allowed_handles_branch,
-- ciphertext_digest_branch, ciphertexts_branch, ciphertexts128_branch.
-- Branch rows are identified by their producer context and, for event-driven
-- rows, their own event block. The script deletes branch tuples produced in
-- reverted blocks and only removes ciphertext bytes when no retained branch
-- row still references them.
--
-- ***WARNING***: The script validates that chain_id exists, but does not verify that
-- to_block_number is a correct number. It is the operator's responsibility
-- to ensure the correct chain_id and to_block_number are provided.
--
-- ***WARNING***: Do not revert across a key rotation boundary. The tfhe-worker always
-- uses the latest key from the `keys` table. If a new key was activated after
-- to_block_number, re-processed computations will use the new key instead of
-- the original one, producing incorrect ciphertexts. The script checks
-- ciphertext_digest_branch.key_id_gw for affected handles and fails if a mismatch
-- is detected, but this check is BEST-EFFORT ONLY (rows may not exist yet if the
-- sns-worker hasn't processed them).
--
-- Reasoning for this script is that it assumes we would be able to detect a state drift
-- with a ciphertext granularity. The operator should identify the earliest host chain
-- block number (in host chain order) where drift occurred — not the first one observed in time,
-- since ciphertext commits reach the gateway out of host chain order. All data from that block
-- onward is reverted. To revert, pass the block before the offending one as
-- to_block_number (i.e. offending_block - 1).
-- Note that due to out-of-order processing, some ciphertexts from blocks prior to the
-- identified one might also have drifted. Therefore, operators should go further back
-- than the offending block to be safe.
--
-- Usage:
--   psql -v chain_id=<CHAIN_ID> -v to_block_number=<BLOCK_NUMBER> -f revert_coprocessor_db_state.sql
--
--
-- Flow:
--   1. Stop ALL coprocessor services.
--   2. Optionally, take a database backup, in case something goes wrong.
--   3. Run this script.
--   4. Restart coprocessor services (host-listener must run in catchup/poller mode).

\set ON_ERROR_STOP on

BEGIN;

-- ===========================================================================
-- Validate parameters
-- ===========================================================================

CREATE TEMP TABLE _param_check AS
SELECT :'chain_id'::bigint AS chain_id, :'to_block_number'::bigint AS to_block_number;

DO $$
DECLARE
  _chain_id bigint;
  _to_block_number bigint;
BEGIN
  SELECT chain_id, to_block_number INTO _chain_id, _to_block_number FROM _param_check;

  IF _to_block_number <= 0 THEN
    RAISE EXCEPTION 'to_block_number must be positive, got %', _to_block_number;
  END IF;

  IF NOT EXISTS (SELECT 1 FROM host_chains WHERE chain_id = _chain_id) THEN
    RAISE EXCEPTION 'chain_id % does not exist in host_chains', _chain_id;
  END IF;
END $$;

-- ===========================================================================
-- Collect affected branch tuples (handle, producer_block_hash).
-- A branch row is "affected" when a producer row or digest row for that tuple
-- lives in a block strictly above to_block_number. Ciphertext bytes that are
-- still referenced by a retained branch row (same handle, same
-- producer_block_hash, block_number <= to_block_number) must be preserved.
-- ===========================================================================

CREATE TEMP TABLE _affected_branch_rows AS
SELECT DISTINCT handle, producer_block_hash
  FROM (
    SELECT output_handle AS handle, producer_block_hash
      FROM computations_branch
     WHERE host_chain_id = :'chain_id'
       AND block_number > :'to_block_number'

    UNION

    SELECT handle, producer_block_hash
      FROM ciphertext_digest_branch
     WHERE host_chain_id = :'chain_id'
       AND block_number > :'to_block_number'
  ) affected
 WHERE producer_block_hash <> ''::BYTEA;

CREATE INDEX ON _affected_branch_rows (handle, producer_block_hash);

-- Legacy (wave-1 dual-write) affected output handles, derived from the legacy
-- `computations` table by block_number. These drive legacy ciphertext byte
-- deletion only; PBS-only reverts must not delete canonical ciphertext bytes.
CREATE TEMP TABLE _affected_output_handles AS
SELECT DISTINCT output_handle AS handle
  FROM computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

CREATE INDEX ON _affected_output_handles (handle);

-- Legacy SNS digest rows are affected both by reverted producers and by
-- reverted PBS/ACL work for a retained producer.
CREATE TEMP TABLE _affected_digest_handles AS
SELECT handle FROM _affected_output_handles
UNION
SELECT DISTINCT handle
  FROM pbs_computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

CREATE INDEX ON _affected_digest_handles (handle);

-- ===========================================================================
-- Check for key rotation across the reverted range.
-- If ciphertext_digest_branch rows for affected branch tuples use a different
-- key than the latest key, a key rotation may have occurred. This check is
-- best-effort: if the sns-worker hasn't processed all handles, some rows may
-- be missing and the check will pass (false negative).
-- ===========================================================================

DO $$
DECLARE
  _latest_key_id_gw bytea;
  _mismatched int;
BEGIN
  SELECT key_id_gw INTO _latest_key_id_gw
  FROM keys ORDER BY sequence_number DESC LIMIT 1;

  IF _latest_key_id_gw IS NOT NULL THEN
    SELECT COUNT(*) INTO _mismatched
    FROM ciphertext_digest_branch cd
    WHERE EXISTS (
            SELECT 1 FROM _affected_branch_rows a
             WHERE a.handle = cd.handle
               AND a.producer_block_hash = cd.producer_block_hash
          )
      AND cd.key_id_gw != _latest_key_id_gw;

    IF _mismatched > 0 THEN
      RAISE EXCEPTION 'Found % ciphertext_digest_branch rows using a different key than the latest. Reverting across a key rotation is not supported.', _mismatched;
    END IF;

    -- Wave-1: the legacy pipeline is still live; apply the same best-effort
    -- check to the legacy digest table.
    SELECT COUNT(*) INTO _mismatched
    FROM ciphertext_digest
    WHERE handle IN (SELECT handle FROM _affected_digest_handles)
      AND key_id_gw != _latest_key_id_gw;

    IF _mismatched > 0 THEN
      RAISE EXCEPTION 'Found % ciphertext_digest rows using a different key than the latest. Reverting across a key rotation is not supported.', _mismatched;
    END IF;
  END IF;
END $$;

-- ===========================================================================
-- Confidential bridge: a copy-bridged destination handle has no `computations`
-- row (the bridge worker copies the source ciphertext directly), so it is
-- absent from the set built above. Add the destination handles being reverted
-- on this chain so their copied ciphertexts/ciphertext_digest rows are cleaned
-- too; on re-ingest the worker re-copies the source ciphertext fresh.
--
-- Placed after the key-rotation check on purpose: re-association re-copies the
-- source bytes (no re-encryption), so the new-key hazard that check guards for
-- does not apply to bridged copies.
-- ===========================================================================

INSERT INTO _affected_output_handles (handle)
SELECT dst_handle
  FROM handle_bridged_events
 WHERE dst_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- `_affected_digest_handles` was already materialized above (before the
-- key-rotation check), so the bridged destination handles just added to
-- `_affected_output_handles` are not in it yet. Add them here too so the legacy
-- `ciphertext_digest` cleanup below removes the bridged digest rows as well, not
-- just the bridged ciphertext bytes. Kept after the key-rotation check so
-- bridged copies stay exempt from it (see note above).
INSERT INTO _affected_digest_handles (handle)
SELECT dst_handle
  FROM handle_bridged_events
 WHERE dst_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Delete from tables that have their own block_number column
-- ===========================================================================

DELETE FROM allowed_handles_branch
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM ciphertext_digest_branch
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM pbs_computations_branch
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM delegate_user_decrypt
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- Delete confidential-bridge events whose block_number is past the revert point.
--
-- bridge_handle_events is keyed by the source chain (src_chain_id) and the
-- destination-side handle_bridged_events by the destination chain (dst_chain_id).
-- Each is cleaned for whichever role this chain plays.
-- ===========================================================================

DELETE FROM bridge_handle_events
 WHERE src_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

DELETE FROM handle_bridged_events
 WHERE dst_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Delete ciphertext bytes produced by reverted branch rows, but preserve
-- bytes still referenced by a retained branch row.
--
-- The authoritative link to a block is still the producer tuple
-- (handle, producer_block_hash). The NOT EXISTS guard keeps the row when the
-- same tuple is also referenced by a retained producer row at
-- block_number <= to_block_number (RFC 011 cross-branch reuse case).
-- ===========================================================================

DELETE FROM ciphertexts_branch cb
 USING _affected_branch_rows a
 WHERE cb.handle = a.handle
   AND cb.producer_block_hash = a.producer_block_hash
   AND NOT EXISTS (
       SELECT 1 FROM computations_branch c
        WHERE c.output_handle = cb.handle
          AND c.producer_block_hash = cb.producer_block_hash
          AND c.host_chain_id = :'chain_id'
          AND (c.block_number IS NULL OR c.block_number <= :'to_block_number')
   )
   AND NOT EXISTS (
       SELECT 1 FROM pbs_computations_branch p
        WHERE p.handle = cb.handle
          AND p.producer_block_hash = cb.producer_block_hash
          AND p.host_chain_id = :'chain_id'
          AND (p.block_number IS NULL OR p.block_number <= :'to_block_number')
   );

DELETE FROM ciphertexts128_branch cb
 USING _affected_branch_rows a
 WHERE cb.handle = a.handle
   AND cb.producer_block_hash = a.producer_block_hash
   AND NOT EXISTS (
       SELECT 1 FROM computations_branch c
        WHERE c.output_handle = cb.handle
          AND c.producer_block_hash = cb.producer_block_hash
          AND c.host_chain_id = :'chain_id'
          AND (c.block_number IS NULL OR c.block_number <= :'to_block_number')
   )
   AND NOT EXISTS (
       SELECT 1 FROM pbs_computations_branch p
        WHERE p.handle = cb.handle
          AND p.producer_block_hash = cb.producer_block_hash
          AND p.host_chain_id = :'chain_id'
          AND (p.block_number IS NULL OR p.block_number <= :'to_block_number')
   );

-- ===========================================================================
-- Delete computations
-- ===========================================================================

DELETE FROM computations_branch
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Wave-1 dual-write: the legacy pipeline still executes from the legacy
-- tables, so the same revert applies there (identical to the pre-wave-1
-- script). These deletes become no-ops once the legacy pipeline is retired
-- after the wave-2 cutover.
-- ===========================================================================

-- _affected_output_handles and _affected_digest_handles are materialized
-- earlier (before the key-rotation guard) and dropped after the deletes below.

DELETE FROM allowed_handles
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM ciphertext_digest
 WHERE handle IN (SELECT handle FROM _affected_digest_handles);

DELETE FROM pbs_computations
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM ciphertexts
 WHERE handle IN (SELECT handle FROM _affected_output_handles)
   AND handle NOT IN (
       SELECT output_handle FROM computations
        WHERE host_chain_id = :'chain_id'
          AND (block_number IS NULL OR block_number <= :'to_block_number')
   );

DELETE FROM ciphertexts128
 WHERE handle IN (SELECT handle FROM _affected_output_handles)
   AND handle NOT IN (
       SELECT output_handle FROM computations
        WHERE host_chain_id = :'chain_id'
          AND (block_number IS NULL OR block_number <= :'to_block_number')
   );

DELETE FROM computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

DROP TABLE IF EXISTS _affected_output_handles;
DROP TABLE IF EXISTS _affected_digest_handles;

-- ===========================================================================
-- Delete block tracking and transactions
-- ===========================================================================

DELETE FROM host_chain_blocks_valid
 WHERE chain_id = :'chain_id'
   AND block_number > :'to_block_number';

DELETE FROM transactions
 WHERE chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Reset poller state so host-listener resumes from to_block_number + 1
-- ===========================================================================

UPDATE host_listener_poller_state
   SET last_caught_up_block = :'to_block_number',
       updated_at = NOW()
 WHERE chain_id = :'chain_id'
   AND last_caught_up_block > :'to_block_number';

-- ===========================================================================
-- Clamp settlement so restarted workers may legally recompute reverted blocks.
-- ===========================================================================

UPDATE coprocessor_settlement
   SET settled_height = :'to_block_number',
       updated_at = NOW()
 WHERE chain_id = :'chain_id'
   AND settled_height > :'to_block_number';

-- ===========================================================================
-- Cleanup
-- ===========================================================================

DROP TABLE IF EXISTS _param_check;
DROP TABLE IF EXISTS _affected_branch_rows;

COMMIT;
