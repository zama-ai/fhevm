-- RFC-029: key-material version cutover (coprocessor side).
--
-- Adds a `material_version` dimension so the coprocessor can pick, per
-- operation, which key material to use as it crosses a per-host-chain
-- (H_C) or gateway (G) cutover block. Everything here is ADDITIVE and
-- INERT: every new column defaults to 0, and with no schedule rows the
-- selection logic always resolves to version 0 -- i.e. byte-identical
-- to today. See fhevm-internal#1568.

-- Which material version a stored artifact was produced under. 0 = legacy
-- material (today's behavior), 1 = migrated CompressedXofKeySet.
--
-- NOTE: there is intentionally no material_version on `keys`. Under the
-- chosen overwrite storage model, the version selects a *column* of the
-- key row (legacy sks_key vs compressed_xof_keyset), not a *row*, so a
-- per-key version flag would carry no invariant. Provenance lives on the
-- artifacts a version actually produces (ciphertexts, SnS tasks).

ALTER TABLE ciphertexts
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- SnS tasks pin to the material version of their SOURCE ciphertext, so
-- the long tail of a pre-cutover ciphertext keeps squashing under the
-- material it was created with.
ALTER TABLE pbs_computations
ADD COLUMN IF NOT EXISTS material_version SMALLINT NOT NULL DEFAULT 0;

-- Input verification (zkpok) happens against the gateway timeline, not a
-- host chain. Record the gateway block so the verifier can resolve its
-- material version against the gateway (G) schedule. Nullable: rows that
-- predate this migration carry NULL and resolve to version 0.
ALTER TABLE verify_proofs
ADD COLUMN IF NOT EXISTS gw_block_number BIGINT NULL DEFAULT NULL;

-- Per-host-chain cutover schedule: material version V takes effect for a
-- host chain at `target_block_number` (a host block). Selection for a
-- (host_chain_id, block_number) computation is the highest V whose
-- target_block_number <= block_number, defaulting to 0 when no row
-- applies. One row per (chain, version); the v1 cutover is one row per
-- host chain with material_version = 1 and target_block_number = H_C.
CREATE TABLE IF NOT EXISTS material_version_host_schedule (
    host_chain_id       BIGINT   NOT NULL,
    material_version    SMALLINT NOT NULL,
    target_block_number BIGINT   NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (host_chain_id, material_version),
    CONSTRAINT material_version_host_schedule_version_positive
        CHECK (material_version >= 0),
    CONSTRAINT material_version_host_schedule_block_positive
        CHECK (target_block_number >= 0)
);

-- Gateway cutover schedule for input verification: material version V
-- takes effect at gateway block `target_block_number` (G). The gateway is
-- a single timeline, so there is no chain dimension.
CREATE TABLE IF NOT EXISTS material_version_gateway_schedule (
    material_version    SMALLINT NOT NULL,
    target_block_number BIGINT   NOT NULL,
    created_at          TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (material_version),
    CONSTRAINT material_version_gateway_schedule_version_positive
        CHECK (material_version >= 0),
    CONSTRAINT material_version_gateway_schedule_block_positive
        CHECK (target_block_number >= 0)
);
