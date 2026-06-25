-- RFC-029: key-material version cutover (coprocessor side).
--
-- Adds a `material_version` dimension so the coprocessor can pick, per
-- operation, which key material to use as it crosses a per-host-chain
-- migration block (`host_migration_block`) or the gateway migration block
-- (`gateway_migration_block`). Everything here is ADDITIVE and INERT:
-- new artifact columns default to 0, the v1 key column is NULL until
-- published, and with no schedule rows the selection logic always resolves
-- to version 0 -- i.e. byte-identical to today. See fhevm-internal#1568.

-- v1 (migrated) key material. The migrated CompressedXofKeySet lands in its
-- OWN column so v0 stays resolvable after the cutover: SnS pins a
-- pre-cutover ciphertext to v0 and may squash it after the migration block,
-- and GPU readers need the v0 XOF keyset to remain intact. v0 therefore
-- keeps reading `compressed_xof_keyset`/`sks_key` exactly as today; v1 reads
-- this column. (The KMS still does copy_compressed_key_to_original in its
-- own S3 store under the original keyId -- that is orthogonal to this local
-- coprocessor cache, which must hold both versions during the transition.)
ALTER TABLE keys
ADD COLUMN IF NOT EXISTS migrated_xof_keyset BYTEA;

-- Which material version a stored artifact was produced under. 0 = legacy
-- (today's behavior), 1 = migrated CompressedXofKeySet.
ALTER TABLE ciphertexts
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- SnS tasks pin to the material version of their SOURCE ciphertext, so the
-- long tail of a pre-cutover ciphertext keeps squashing under the material
-- it was created with.
ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- Input verification (zkpok) is anchored to the gateway timeline, not a host
-- chain. Record the gateway block so the verifier can resolve its material
-- version against the gateway schedule. Nullable: rows that predate this
-- migration carry NULL and resolve to version 0.
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS gw_block_number BIGINT NULL DEFAULT NULL;

-- Per-host-chain cutover schedule: material version V takes effect for a
-- host chain at `migration_block` (a host block). Selection for a
-- (host_chain_id, block_number) computation is the highest V whose
-- migration_block <= block_number, defaulting to 0 when no row applies. One
-- row per (chain, version); the v1 cutover is one row per host chain with
-- material_version = 1 and migration_block = that chain's host_migration_block.
CREATE TABLE IF NOT EXISTS material_version_host_schedule (
    host_chain_id    BIGINT   NOT NULL,
    material_version SMALLINT NOT NULL,
    migration_block  BIGINT   NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (host_chain_id, material_version),
    CONSTRAINT material_version_host_schedule_version_positive
        CHECK (material_version >= 0),
    CONSTRAINT material_version_host_schedule_block_positive
        CHECK (migration_block >= 0)
);

-- Gateway cutover schedule for input verification: material version V takes
-- effect at the gateway migration block. The gateway is a single timeline,
-- so there is no chain dimension.
CREATE TABLE IF NOT EXISTS material_version_gateway_schedule (
    material_version SMALLINT NOT NULL,
    migration_block  BIGINT   NOT NULL,
    created_at       TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (material_version),
    CONSTRAINT material_version_gateway_schedule_version_positive
        CHECK (material_version >= 0),
    CONSTRAINT material_version_gateway_schedule_block_positive
        CHECK (migration_block >= 0)
);
