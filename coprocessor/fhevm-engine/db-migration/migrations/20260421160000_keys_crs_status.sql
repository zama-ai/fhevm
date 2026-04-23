ALTER TABLE keys DROP CONSTRAINT unique_key_id_gw;
ALTER TABLE keys DROP CONSTRAINT unique_key_id;

-- add extra-info
ALTER TABLE keys
    ADD COLUMN chain_id BIGINT,
    ADD COLUMN block_hash BYTEA;

-- Guarantees idempotency of live ingest / catchup replays: the same
-- (chain_id, block_hash, key_id) cannot be inserted twice.
CREATE UNIQUE INDEX uniq_keys_chain_block_id_gw
    ON keys (chain_id, block_hash, key_id);

ALTER TABLE crs DROP CONSTRAINT unique_crs_id;

ALTER TABLE crs
    ADD COLUMN chain_id BIGINT,
    ADD COLUMN block_hash BYTEA;

CREATE UNIQUE INDEX uniq_crs_chain_block_id
    ON crs (chain_id, block_hash, crs_id);
