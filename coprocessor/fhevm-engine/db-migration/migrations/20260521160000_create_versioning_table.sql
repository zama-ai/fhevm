-- Singleton table holding the currently active stack + ciphertext version.
-- Used as the DB-level fence for ciphertext writes: an INSERT/UPDATE on
-- `ciphertexts` whose `ciphertext_version` does not match this row is
-- rejected by a trigger (see 20260521160200_enforce_versioning_on_ciphertexts.sql).
--
-- The row is updated in-place during cutover, inside the same transaction
-- that holds the exclusive cutover advisory lock. There is exactly one row,
-- enforced by `singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton)`.
CREATE TABLE IF NOT EXISTS versioning (
    singleton          BOOLEAN  PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    stack_version      TEXT     NOT NULL,
    ciphertext_version SMALLINT NOT NULL CHECK (ciphertext_version >= 0),
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed with the current values. The stack_version is informational; the
-- ciphertext_version must match what `fhevm_engine_common::tfhe_ops::current_ciphertext_version()`
-- returns at build time.
INSERT INTO versioning (singleton, stack_version, ciphertext_version)
VALUES (TRUE, 'v0.14', 2)
ON CONFLICT (singleton) DO NOTHING;
