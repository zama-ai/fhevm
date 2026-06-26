-- RFC-029: key-material version cutover (coprocessor side).
--
-- Adds a `material_version` dimension so the coprocessor can pick, per
-- operation, which key material to use as it crosses a per-host-chain
-- migration block (`host_migration_block`) or the gateway migration block
-- (`gateway_migration_block`). Everything here is ADDITIVE and INERT:
-- new artifact columns default to 0, the v1 key column is NULL until
-- published, and with no schedule rows the selection logic always resolves
-- to version 0 -- i.e. byte-identical to today. See fhevm-internal#1568.

-- v1 (migrated) key material in its OWN column so v0 stays resolvable after
-- the cutover (SnS may still squash pre-cutover ciphertexts under v0). v0 keeps
-- reading compressed_xof_keyset/sks_key; v1 reads this column.
ALTER TABLE keys
ADD COLUMN IF NOT EXISTS migrated_xof_keyset BYTEA;

-- Material version a ciphertext was produced under: 0 = legacy, 1 = migrated.
-- SnS pins each task to its SOURCE ciphertext's version (read via JOIN), so a
-- pre-cutover ciphertext keeps squashing under the material it was created with.
ALTER TABLE ciphertexts
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- Input verification (zkpok) is anchored to the gateway timeline, not a host
-- chain. Record the gateway block so the verifier can resolve its material
-- version against the gateway schedule. Nullable: rows that predate this
-- migration carry NULL and resolve to version 0.
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS gw_block_number BIGINT NULL DEFAULT NULL;

-- Per-host-chain cutover: the migrated material (v1) takes effect for a host
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
