-- Track which consensus protocol version produced each ciphertext.
--
-- A stack that carries this feature stamps its compiled CONSENSUS_PROTOCOL_VERSION
-- on insert: tfhe-worker for compute outputs, zkproof-worker for inputs. A GCS
-- (green) stack running a newer protocol writes its own version (e.g. 2).
--
-- The column defaults to 1 - the blue/baseline consensus version - so that BOTH
--   * rows that already existed before this feature, and
--   * rows inserted by a stack WITHOUT the stamping code (a released v0.14 blue
--     stack, which never writes the column),
-- read as 1. A stack that carries the stamping code overrides the default,
-- writing its own CONSENSUS_PROTOCOL_VERSION.
--
-- SMALLINT is ample: consensus versions are small, slow-moving ints.
ALTER TABLE ciphertexts
    ADD COLUMN IF NOT EXISTS consensus_version SMALLINT DEFAULT 1;
