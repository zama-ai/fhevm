export const DRIFT_INSTALL_SQL = `CREATE TABLE IF NOT EXISTS drift_injection_state (
  id BOOLEAN PRIMARY KEY DEFAULT TRUE,
  enabled BOOLEAN NOT NULL,
  consumed BOOLEAN NOT NULL DEFAULT FALSE,
  injected_handle BYTEA
);

INSERT INTO drift_injection_state (id, enabled, consumed, injected_handle)
VALUES (TRUE, TRUE, FALSE, NULL)
ON CONFLICT (id) DO UPDATE
SET enabled = EXCLUDED.enabled,
    consumed = EXCLUDED.consumed,
    injected_handle = EXCLUDED.injected_handle;

CREATE OR REPLACE FUNCTION inject_ciphertext_drift_once()
RETURNS trigger
LANGUAGE plpgsql
AS $$
DECLARE
  should_inject BOOLEAN;
BEGIN
  SELECT enabled AND NOT consumed
  INTO should_inject
  FROM drift_injection_state
  WHERE id = TRUE;

  IF NOT COALESCE(should_inject, FALSE) THEN
    RETURN NEW;
  END IF;

  -- The SNS worker writes the digests on insert and stamps the S3 witness once
  -- the upload succeeds; the transaction sender only sends a digest equal to
  -- that witness. Corrupt both at the stamp so the drifted digest is sent.
  IF NEW.txn_is_sent = FALSE
     AND NEW.ciphertext IS NOT NULL
     AND NEW.ciphertext128 IS NOT NULL
     AND OLD.s3_publication_verified_at IS NULL
     AND NEW.s3_publication_verified_at IS NOT NULL
     AND EXISTS (SELECT 1 FROM computations WHERE output_handle = NEW.handle) THEN
    NEW.ciphertext := set_byte(NEW.ciphertext, 0, get_byte(NEW.ciphertext, 0) # 1);
    NEW.s3_publication_verified_digest := NEW.ciphertext;

    UPDATE drift_injection_state
    SET consumed = TRUE,
        injected_handle = NEW.handle
    WHERE id = TRUE;
  END IF;

  RETURN NEW;
END;
$$;

DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;

CREATE TRIGGER ciphertext_drift_injector
BEFORE UPDATE ON ciphertext_digest
FOR EACH ROW
EXECUTE FUNCTION inject_ciphertext_drift_once();
`;

export const DRIFT_CLEANUP_SQL = `DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
DROP FUNCTION IF EXISTS inject_ciphertext_drift_once();
DROP TABLE IF EXISTS drift_injection_state;
`;

/** Parses a coprocessor instance index from env or CLI input. */
export const parseDriftInstanceIndex = (value: string) => {
  if (!/^\d+$/.test(value)) {
    throw new Error("instance index must be a non-negative integer");
  }
  return Number(value);
};

/** Parses a positive integer environment setting used by the drift test. */
export const parsePositiveInteger = (value: string, name: string) => {
  if (!/^\d+$/.test(value) || Number(value) <= 0) {
    throw new Error(`${name} must be a positive integer`);
  }
  return Number(value);
};
