-- GCS-side staging tables used during the blue/green upgrade flow.
--   ciphertexts_staging     INHERITS (ciphertexts)
--   ciphertexts128_staging  INHERITS (ciphertexts128)
--   state_hash_staging      INHERITS (state_hash)
--
-- During the dry-run phase, the GCS tfhe-worker / sns-worker write their FHE
-- outputs to these child tables — including the per-block state hash, which
-- only the GCS worker writes (BCS continues to write into the parent
-- state_hash). Reads from the inheritance hierarchy via `FROM ciphertexts`
-- (without ONLY) surface rows from both the parent (BCS) and the child
-- (GCS). At cutover, the child rows are merged into the parent and the
-- staging tables dropped.
--
-- PostgreSQL inheritance does NOT carry over primary keys or unique indexes,
-- so we recreate the ones that matter for INSERT ... ON CONFLICT semantics
-- (tfhe-worker uses `ON CONFLICT (handle, ciphertext_version)` against
-- ciphertexts; sns-worker uses `ON CONFLICT (handle)` against ciphertexts128;
-- tfhe-worker uses `ON CONFLICT (chain_id, block_number)` against state_hash).

CREATE TABLE IF NOT EXISTS ciphertexts_staging () INHERITS (ciphertexts);

ALTER TABLE ciphertexts_staging
    ADD CONSTRAINT ciphertexts_staging_pkey
    PRIMARY KEY (tenant_id, handle, ciphertext_version);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ciphertexts_staging_no_tenant
    ON ciphertexts_staging (handle, ciphertext_version);

CREATE TABLE IF NOT EXISTS ciphertexts128_staging () INHERITS (ciphertexts128);

ALTER TABLE ciphertexts128_staging
    ADD CONSTRAINT ciphertexts128_staging_pkey
    PRIMARY KEY (tenant_id, handle);

CREATE UNIQUE INDEX IF NOT EXISTS idx_ciphertexts128_staging_no_tenant
    ON ciphertexts128_staging (handle);

CREATE TABLE IF NOT EXISTS state_hash_staging () INHERITS (state_hash);

ALTER TABLE state_hash_staging
    ADD CONSTRAINT state_hash_staging_pkey
    PRIMARY KEY (chain_id, block_number);
