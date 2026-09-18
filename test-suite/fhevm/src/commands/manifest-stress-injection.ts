/** Test-only database objects installed on one isolated coprocessor. */
export const INSTALL_STRESS_INJECTION = `
BEGIN;
CREATE TABLE e2e_manifest_noise (
  handle BYTEA NOT NULL, ciphertext_version SMALLINT NOT NULL,
  injected BOOLEAN NOT NULL, byte_offset INTEGER NOT NULL,
  original_byte INTEGER NOT NULL, recorded_at TIMESTAMPTZ NOT NULL DEFAULT now(),
  PRIMARY KEY(handle, ciphertext_version)
);
CREATE FUNCTION e2e_inject_ct64_noise() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE
  flip BOOLEAN;
  mid INTEGER;
BEGIN
  IF NEW.is_input OR NEW.ciphertext_version <> 0 OR NEW.ciphertext_type <> 5
     OR octet_length(NEW.ciphertext) <= 64 THEN RETURN NULL; END IF;
  flip := random() < 0.25;
  mid := octet_length(NEW.ciphertext) / 2;
  INSERT INTO e2e_manifest_noise(handle,ciphertext_version,injected,byte_offset,original_byte)
    VALUES(NEW.handle,NEW.ciphertext_version,flip,mid,get_byte(NEW.ciphertext,mid));
  IF flip THEN
    UPDATE ciphertexts SET ciphertext=set_byte(ciphertext,mid,get_byte(ciphertext,mid) # 128)
      WHERE handle=NEW.handle AND ciphertext_version=NEW.ciphertext_version;
  END IF;
  RETURN NULL;
END;
$$;
CREATE TRIGGER e2e_ciphertexts_noise AFTER INSERT ON ciphertexts
  FOR EACH ROW EXECUTE FUNCTION e2e_inject_ct64_noise();
COMMIT;`;
export const DISABLE_STRESS_INJECTION = "DROP TRIGGER IF EXISTS e2e_ciphertexts_noise ON ciphertexts; DROP FUNCTION IF EXISTS e2e_inject_ct64_noise();";
