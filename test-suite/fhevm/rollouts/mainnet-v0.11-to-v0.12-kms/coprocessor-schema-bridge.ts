/**
 * Lets the v0.12 transaction sender read rows written by the deployed v0.11
 * coprocessor services without replacing their database initialization.
 */
export const coprocessorSenderSchemaBridge = `
DO $$
DECLARE
  deadline TIMESTAMPTZ := clock_timestamp() + INTERVAL '2 minutes';
BEGIN
  LOOP
    EXIT WHEN EXISTS (SELECT 1 FROM tenants WHERE octet_length(key_id) = 32);
    IF clock_timestamp() >= deadline THEN
      RAISE EXCEPTION 'legacy tenant did not receive a 32-byte gateway key before the compatibility timeout';
    END IF;
    PERFORM pg_sleep(2);
  END LOOP;
END
$$;

ALTER TABLE ciphertext_digest
  ADD COLUMN IF NOT EXISTS host_chain_id BIGINT;
ALTER TABLE ciphertext_digest
  ADD COLUMN IF NOT EXISTS key_id_gw BYTEA;

CREATE OR REPLACE FUNCTION set_rollout_ciphertext_context()
RETURNS trigger
LANGUAGE plpgsql
AS $$
BEGIN
  SELECT chain_id, key_id
    INTO NEW.host_chain_id, NEW.key_id_gw
    FROM tenants
    WHERE tenant_id = NEW.tenant_id;

  IF NEW.host_chain_id IS NULL OR octet_length(NEW.key_id_gw) IS DISTINCT FROM 32 THEN
    RAISE EXCEPTION 'legacy tenant % does not have a valid chain and 32-byte gateway key', NEW.tenant_id;
  END IF;
  RETURN NEW;
END
$$;

DROP TRIGGER IF EXISTS set_rollout_ciphertext_context ON ciphertext_digest;
CREATE TRIGGER set_rollout_ciphertext_context
BEFORE INSERT OR UPDATE OF tenant_id ON ciphertext_digest
FOR EACH ROW EXECUTE FUNCTION set_rollout_ciphertext_context();

UPDATE ciphertext_digest SET tenant_id = tenant_id;
ALTER TABLE ciphertext_digest ALTER COLUMN host_chain_id SET NOT NULL;
ALTER TABLE ciphertext_digest ALTER COLUMN key_id_gw SET NOT NULL;
`;
