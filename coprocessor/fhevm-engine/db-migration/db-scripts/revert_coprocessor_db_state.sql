-- revert_coprocessor_db_state.sql
-- Reverts all coprocessor state for a given chain to a given block number.
-- All data from blocks strictly greater than to_block_number will be deleted.
--
-- ***WARNING***: The script validates that chain_id exists, but does not verify that
-- to_block_number is a correct number. It is the operator's responsibility
-- to ensure the correct chain_id and to_block_number are provided.
--
-- ***WARNING***: Do not revert across a key rotation boundary. The tfhe-worker always
-- uses the latest key from the `keys` table. If a new key was activated after
-- to_block_number, re-processed computations will use the new key instead of
-- the original one, producing incorrect ciphertexts. The script checks
-- ciphertext_digest.key_id_gw for affected handles and fails if a mismatch
-- is detected, but this check is BEST-EFFORT ONLY (rows may not exist yet if the
-- sns-worker hasn't processed them).
--
-- Reasoning for this script is that it assumes we would be able to detect a state drift
-- with a ciphertext granularity. Consequently, the first such drift that we observe means
-- that no prior state was affected. To revert, pass the block before the offending one
-- as to_block_number (i.e. offending_block - 1). The script deletes all data for blocks
-- strictly greater than to_block_number.
-- Note that above assumes all ciphertext have been computed up to the offending block,
-- but in reality, due to out-of-order processing, it might not be the case. That means
-- some prior ciphertexts might also drift. Therefore, operators might choose to go a bit
-- further back than the offending block to make it even more likely to capture the root
-- cause of the drift.
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
-- Collect affected dependence_chain IDs (before deleting computations)
-- ===========================================================================

CREATE TEMP TABLE _affected_dc_ids AS
SELECT DISTINCT dependence_chain_id AS dc_id
  FROM computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number'
   AND dependence_chain_id IS NOT NULL;

CREATE INDEX ON _affected_dc_ids (dc_id);

-- ===========================================================================
-- Collect affected output handles (for ciphertext cleanup)
-- ===========================================================================

CREATE TEMP TABLE _affected_output_handles AS
SELECT DISTINCT output_handle AS handle
  FROM computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

CREATE INDEX ON _affected_output_handles (handle);

-- ===========================================================================
-- Check for key rotation across the reverted range.
-- If ciphertext_digest rows for affected handles use a different key than the
-- latest key, a key rotation may have occurred. This check is best-effort:
-- if the sns-worker hasn't processed all handles, some rows may be missing
-- and the check will pass (false negative).
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
    FROM ciphertext_digest
    WHERE handle IN (SELECT handle FROM _affected_output_handles)
      AND key_id_gw != _latest_key_id_gw;

    IF _mismatched > 0 THEN
      RAISE EXCEPTION 'Found % ciphertext_digest rows using a different key than the latest. Reverting across a key rotation is not supported.', _mismatched;
    END IF;
  END IF;
END $$;

-- ===========================================================================
-- Delete from tables that have their own block_number column
-- ===========================================================================

DELETE FROM allowed_handles
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM ciphertext_digest
 WHERE handle IN (SELECT handle FROM _affected_output_handles);

DELETE FROM pbs_computations
 WHERE block_number > :'to_block_number'
   AND host_chain_id = :'chain_id';

DELETE FROM delegate_user_decrypt
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Delete ciphertexts that were produced by computations in reverted blocks.
--
-- ciphertexts/ciphertexts128 have no transaction_id or block_number.
-- The only link to a block is: a computation's output_handle matches a
-- ciphertext's handle. We collected those handles in _affected_output_handles.
--
-- The NOT IN guard handles the edge case where the same handle is the output
-- of both a reverted computation (block > N) and a retained computation
-- (block <= N). In that case we keep the ciphertext.
-- ===========================================================================

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

-- ===========================================================================
-- Delete computations
-- ===========================================================================

DELETE FROM computations
 WHERE host_chain_id = :'chain_id'
   AND block_number > :'to_block_number';

-- ===========================================================================
-- Delete dependence_chain entries
-- ===========================================================================

DELETE FROM dependence_chain
 WHERE dependence_chain_id IN (SELECT dc_id FROM _affected_dc_ids);

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
-- Cleanup
-- ===========================================================================

DROP TABLE IF EXISTS _param_check;
DROP TABLE IF EXISTS _affected_dc_ids;
DROP TABLE IF EXISTS _affected_output_handles;

COMMIT;