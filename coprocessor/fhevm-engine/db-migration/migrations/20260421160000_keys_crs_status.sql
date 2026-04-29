ALTER TABLE keys DROP CONSTRAINT IF EXISTS unique_key_id_gw;
ALTER TABLE keys DROP CONSTRAINT IF EXISTS unique_key_id;

-- add extra-info
ALTER TABLE keys
    ADD COLUMN IF NOT EXISTS chain_id BIGINT,
    ADD COLUMN IF NOT EXISTS block_hash BYTEA;

-- Legacy rows predate per-chain key metadata. Backfill them onto the
-- canonical pre-existing host chain created by the tenant split migration.
DO $$
DECLARE
    hcid BIGINT;
BEGIN
    SELECT chain_id
    INTO hcid
    FROM host_chains
    ORDER BY created_at, chain_id
    LIMIT 1;

    IF hcid IS NOT NULL THEN
        UPDATE keys
        SET chain_id = hcid
        WHERE chain_id IS NULL;
    END IF;
END $$;

-- Guarantees idempotency of live ingest / catchup replays: the same
-- (chain_id, block_hash, key_id) cannot be inserted twice.
CREATE UNIQUE INDEX IF NOT EXISTS uniq_keys_chain_block_id_gw
    ON keys (chain_id, block_hash, key_id);

ALTER TABLE crs DROP CONSTRAINT IF EXISTS unique_crs_id;

ALTER TABLE crs
    ADD COLUMN IF NOT EXISTS chain_id BIGINT,
    ADD COLUMN IF NOT EXISTS block_hash BYTEA;

DO $$
DECLARE
    hcid BIGINT;
BEGIN
    SELECT chain_id
    INTO hcid
    FROM host_chains
    ORDER BY created_at, chain_id
    LIMIT 1;

    IF hcid IS NOT NULL THEN
        UPDATE crs
        SET chain_id = hcid
        WHERE chain_id IS NULL;
    END IF;
END $$;

CREATE UNIQUE INDEX IF NOT EXISTS uniq_crs_chain_block_id
    ON crs (chain_id, block_hash, crs_id);
