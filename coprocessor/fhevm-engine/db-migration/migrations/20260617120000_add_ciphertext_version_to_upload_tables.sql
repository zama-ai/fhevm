-- Tag the upload-pipeline tables with the ciphertext_version that produced
-- their bytes/digest. This is the blue-green guard used by the SnS backfill on
-- cutover:
--   * the uploader only (re)uploads rows matching its own
--     `current_ciphertext_version()`, so the retired blue worker never touches
--     the green stack's rows during the cutover race window;
--   * garbage collection only deletes ct128 bytes of its own version, so the
--     expensive PBS output recomputed by the green stack is never dropped
--     before it has been uploaded.
--
-- All writers (sns-worker inserts, the cutover merge in upgrade-controller)
-- stamp this column explicitly; the DEFAULT only backstops unexpected paths.
-- Existing rows are backfilled from the live `versioning` singleton.

ALTER TABLE ciphertext_digest
    ADD COLUMN IF NOT EXISTS ciphertext_version SMALLINT;

ALTER TABLE ciphertexts128
    ADD COLUMN IF NOT EXISTS ciphertext_version SMALLINT;

UPDATE ciphertext_digest
    SET ciphertext_version = COALESCE(
        (SELECT ciphertext_version FROM versioning WHERE singleton = TRUE), 0)
    WHERE ciphertext_version IS NULL;

UPDATE ciphertexts128
    SET ciphertext_version = COALESCE(
        (SELECT ciphertext_version FROM versioning WHERE singleton = TRUE), 0)
    WHERE ciphertext_version IS NULL;

ALTER TABLE ciphertext_digest
    ALTER COLUMN ciphertext_version SET DEFAULT 0,
    ALTER COLUMN ciphertext_version SET NOT NULL;

ALTER TABLE ciphertexts128
    ALTER COLUMN ciphertext_version SET DEFAULT 0,
    ALTER COLUMN ciphertext_version SET NOT NULL;
