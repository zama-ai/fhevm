-- RFC-029 one-time compressed-key cutover.
--
-- key_material_kind: 0 = legacy bytes, 1 = compressed_xof bytes. The
-- default 0 covers all pre-migration rows with no backfill.

-- Which material produced this ciphertext. Stamped at insert time by
-- the producing worker (host rule for compute outputs, gateway rule
-- for input ciphertexts).
ALTER TABLE ciphertexts
    ADD COLUMN key_material_kind SMALLINT NOT NULL DEFAULT 0;

-- Finalized Gateway block of the VerifyProofRequest that produced an
-- input ciphertext; NULL for compute outputs and pre-feature rows.
-- Used by the first-finalized-request-wins canonical upsert.
ALTER TABLE ciphertexts
    ADD COLUMN gateway_block_number BIGINT;

-- SnS tasks pin the material kind of their canonical source
-- ciphertext at enqueue time; the task row is the sns-worker's only
-- selection authority (never re-derived by joining ciphertexts).
ALTER TABLE pbs_computations
    ADD COLUMN key_material_kind SMALLINT NOT NULL DEFAULT 0;

-- Finalized Gateway block of the VerifyProofRequest event, persisted
-- by the gw-listener so the zkproof-worker can evaluate the gateway
-- cutover rule. NULL only for rows created before this feature, which
-- necessarily predate any scheduled cutover.
ALTER TABLE verify_proofs
    ADD COLUMN gateway_block_number BIGINT;

-- The scheduled cutover, ingested from the finalized on-chain
-- CompressedKeyCutoverScheduled record. At most one row per key;
-- immutable once written (duplicates are rejected loudly by the
-- ingestion code, never silently overwritten).
CREATE TABLE compressed_key_cutover (
    key_id BYTEA PRIMARY KEY,
    gateway_cutover_block BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE compressed_key_cutover_hosts (
    key_id BYTEA NOT NULL REFERENCES compressed_key_cutover (key_id),
    chain_id BIGINT NOT NULL,
    cutover_block BIGINT NOT NULL,
    PRIMARY KEY (key_id, chain_id)
);
