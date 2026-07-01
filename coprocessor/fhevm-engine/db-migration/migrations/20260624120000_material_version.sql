-- RFC-029: key-material cutover state (coprocessor side).
--
-- `keys.compressed_xof_keyset` is the single home for CompressedXofKeySet
-- material. `material_migration_status` only tells readers whether that column
-- is native material (NULL, read with today's COALESCE path) or migrated
-- material that must be hidden until a finalized schedule says to use it.
ALTER TABLE keys
ADD COLUMN IF NOT EXISTS material_migration_status SMALLINT NULL
CHECK (material_migration_status IN (1, 2));

-- Material version a ciphertext was produced under: 0 = legacy, 1 = migrated.
-- SnS pins each task to its SOURCE ciphertext's version (read via JOIN), so a
-- pre-cutover ciphertext keeps squashing under `sks_key` after the cutover.
ALTER TABLE ciphertexts
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- Input verification (zkpok) is anchored to the gateway timeline, not a host
-- chain. Record the gateway block so the verifier can resolve its material
-- version against the gateway schedule. Nullable: rows that predate this
-- migration carry NULL and resolve to version 0.
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS gw_block_number BIGINT NULL DEFAULT NULL;

-- Per-host-chain cutover: the compressed material (v1) takes effect for a host
-- chain at `target_block`. One row per chain; a (chain, block) computation is
-- v1 iff block >= target_block, else v0. RFC-029 is a one-time cutover, so
-- there is no version column -- the table IS the v1 cutover schedule.
CREATE TABLE IF NOT EXISTS material_version_host_schedule (
    host_chain_id BIGINT      NOT NULL PRIMARY KEY,
    target_block  BIGINT      NOT NULL CHECK (target_block >= 0),
    created_at    TIMESTAMPTZ NOT NULL DEFAULT now()
);

-- Gateway cutover for input verification: v1 takes effect at `target_block`.
-- The gateway is a single timeline, so this table holds exactly one row
-- (singleton enforced by a constant primary key).
CREATE TABLE IF NOT EXISTS material_version_gateway_schedule (
    singleton    BOOLEAN     NOT NULL PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    target_block BIGINT      NOT NULL CHECK (target_block >= 0),
    created_at   TIMESTAMPTZ NOT NULL DEFAULT now()
);
