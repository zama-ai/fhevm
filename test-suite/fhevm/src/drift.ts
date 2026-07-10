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
  is_compute_output BOOLEAN;
BEGIN
  SELECT enabled AND NOT consumed
  INTO should_inject
  FROM drift_injection_state
  WHERE id = TRUE;

  IF NOT COALESCE(should_inject, FALSE) THEN
    RETURN NEW;
  END IF;

  SELECT EXISTS (
    SELECT 1 FROM computations WHERE output_handle = NEW.handle
  ) INTO is_compute_output;

  IF NOT is_compute_output AND to_regclass('computations_branch') IS NOT NULL THEN
    EXECUTE 'SELECT EXISTS (
      SELECT 1 FROM computations_branch WHERE output_handle = $1
    )'
    INTO is_compute_output
    USING NEW.handle;
  END IF;

  IF NEW.txn_is_sent = FALSE
     AND NEW.ciphertext IS NOT NULL
     AND NEW.ciphertext128 IS NOT NULL
     AND (OLD.ciphertext IS NULL OR OLD.ciphertext128 IS NULL)
     AND is_compute_output THEN
    NEW.ciphertext := set_byte(NEW.ciphertext, 0, get_byte(NEW.ciphertext, 0) # 1);

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

DO $$
BEGIN
  IF to_regclass('ciphertext_digest_branch') IS NOT NULL THEN
    EXECUTE 'DROP TRIGGER IF EXISTS ciphertext_drift_injector_branch ON ciphertext_digest_branch';
    EXECUTE 'CREATE TRIGGER ciphertext_drift_injector_branch
      BEFORE UPDATE ON ciphertext_digest_branch
      FOR EACH ROW
      EXECUTE FUNCTION inject_ciphertext_drift_once()';
  END IF;
END;
$$;
`;

export const DRIFT_CLEANUP_SQL = `DO $$
BEGIN
  IF to_regclass('ciphertext_digest_branch') IS NOT NULL THEN
    EXECUTE 'DROP TRIGGER IF EXISTS ciphertext_drift_injector_branch ON ciphertext_digest_branch';
  END IF;
END;
$$;

DROP TRIGGER IF EXISTS ciphertext_drift_injector ON ciphertext_digest;
DROP FUNCTION IF EXISTS inject_ciphertext_drift_once();
DROP TABLE IF EXISTS drift_injection_state;
`;

export const driftDatabaseName = (instanceIndex: number) =>
  instanceIndex === 0 ? "coprocessor" : `coprocessor_${instanceIndex}`;

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
