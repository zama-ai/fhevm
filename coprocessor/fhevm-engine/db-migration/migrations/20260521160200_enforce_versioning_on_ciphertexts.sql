-- DB-level fence: any INSERT/UPDATE into the parent `ciphertexts` table whose
-- `ciphertext_version` does not match `versioning.ciphertext_version` is
-- rejected. Combined with the per-tx advisory lock taken during cutover,
-- this catches any stray BCS write that bypasses (or hasn't been updated to
-- honor) the lock-and-state-check protocol.
--
-- The trigger is attached to the PARENT (`ciphertexts`) only — PostgreSQL
-- inheritance does not propagate triggers to children, so writes to
-- `ciphertexts_staging` (the GCS dry-run target) are unaffected. This is
-- intentional: during dry-run, GCS writes V_new to staging while VERSIONING
-- still reads V_old; the staging write would otherwise be rejected.
--
-- At cutover, the merge runs in this order inside one tx:
--   1. UPDATE versioning SET ciphertext_version = V_new
--   2. INSERT INTO ONLY ciphertexts SELECT * FROM ciphertexts_staging
-- so the trigger sees V_new (in NEW row) == V_new (in versioning) and passes.

CREATE OR REPLACE FUNCTION enforce_ciphertext_version() RETURNS TRIGGER AS $$
DECLARE
    expected SMALLINT;
BEGIN
    SELECT ciphertext_version INTO expected
    FROM versioning
    WHERE singleton = TRUE;

    IF expected IS NULL THEN
        RAISE EXCEPTION 'versioning singleton row is missing — refusing ciphertext write';
    END IF;

    IF NEW.ciphertext_version IS DISTINCT FROM expected THEN
        RAISE EXCEPTION
            'ciphertext_version % does not match versioning.ciphertext_version %',
            NEW.ciphertext_version, expected
            USING ERRCODE = 'check_violation';
    END IF;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS ciphertexts_enforce_version ON ciphertexts;
CREATE TRIGGER ciphertexts_enforce_version
    BEFORE INSERT OR UPDATE ON ciphertexts
    FOR EACH ROW EXECUTE FUNCTION enforce_ciphertext_version();
