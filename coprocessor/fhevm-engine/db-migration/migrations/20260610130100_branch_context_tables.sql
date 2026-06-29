-- Branch-context wave 1: branch-keyed sibling tables.
--
-- Each `*_branch` table extends its legacy sibling with `producer_block_hash`,
-- the hash of the host-chain block the row was produced under, so that state
-- from competing fork branches can coexist and reorgs can be undone by
-- deleting the orphaned branch's rows. In wave 1 the host-listener and
-- zkproof-worker dual-write these tables while every reader stays on the
-- legacy tables; wave 2 switches the readers over.
--
-- An empty producer_block_hash ('') marks a row as "branchless": not derived
-- from any block (ZK-verified user inputs, backfilled pre-upgrade state).
-- Branchless rows are valid on every branch and are never orphan-cleaned.
--
-- Unlike the legacy tables (see 20260603120000_collapse_overlapping_unique_keys),
-- each table carries a single unique key — the tenant-free column set used by
-- the writers' ON CONFLICT arbiters — so concurrent revert + upsert cannot
-- surface duplicates on a second unique index. `tenant_id` stays a plain
-- column copied via LIKE.

-- ============================ computations ============================

CREATE TABLE IF NOT EXISTS computations_branch
(
    LIKE computations INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE computations_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE computations_branch
DROP CONSTRAINT IF EXISTS computations_branch_pkey;

ALTER TABLE computations_branch
ADD CONSTRAINT computations_branch_pkey
PRIMARY KEY (output_handle, transaction_id, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_computations_branch_handle_block_hash
ON computations_branch (host_chain_id, output_handle, producer_block_hash);

-- Orphan cleanup deletes by (host_chain_id, producer_block_hash).
CREATE INDEX IF NOT EXISTS idx_computations_branch_chain_block_hash
ON computations_branch (host_chain_id, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_computations_branch_block_number
ON computations_branch (host_chain_id, block_number);

CREATE INDEX IF NOT EXISTS idx_computations_branch_transaction_id
ON computations_branch (transaction_id);

CREATE INDEX IF NOT EXISTS idx_computations_branch_schedule_order
ON computations_branch USING BTREE (schedule_order)
WHERE is_completed = false AND is_error = false;

CREATE INDEX IF NOT EXISTS idx_computations_branch_dependence_chain
ON computations_branch (dependence_chain_id)
WHERE is_completed = false AND is_error = false;

CREATE INDEX IF NOT EXISTS idx_computations_branch_is_allowed
ON computations_branch USING BTREE (is_allowed)
WHERE is_completed = false;

-- Wake idle tfhe-workers on new branch work (same channel as the legacy
-- computations trigger; the function exists since 20251006080000).
DROP TRIGGER IF EXISTS work_updated_trigger_from_computations_branch ON computations_branch;
CREATE TRIGGER work_updated_trigger_from_computations_branch
    AFTER INSERT
    ON computations_branch
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_work_available();

-- ========================== pbs_computations ==========================

CREATE TABLE IF NOT EXISTS pbs_computations_branch
(
    LIKE pbs_computations INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE pbs_computations_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE pbs_computations_branch
DROP CONSTRAINT IF EXISTS pbs_computations_branch_pkey;

ALTER TABLE pbs_computations_branch
ADD CONSTRAINT pbs_computations_branch_pkey
PRIMARY KEY (handle, producer_block_hash, block_hash);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_chain_handle_block_hash
ON pbs_computations_branch (host_chain_id, handle, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_chain_block_hash
ON pbs_computations_branch (host_chain_id, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_chain_block
ON pbs_computations_branch (host_chain_id, block_hash);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_block_number
ON pbs_computations_branch (host_chain_id, block_number);

CREATE INDEX IF NOT EXISTS idx_pbs_computations_branch_pending_created_at
ON pbs_computations_branch (created_at, handle)
WHERE is_completed = false;

-- Wake idle sns-workers on new branch work (same channel as the legacy
-- pbs_computations trigger; the function exists since 20250512084614).
DROP TRIGGER IF EXISTS on_insert_notify_event_pbs_computations_branch ON pbs_computations_branch;
CREATE TRIGGER on_insert_notify_event_pbs_computations_branch
    AFTER INSERT
    ON pbs_computations_branch
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_event_pbs_computations();

-- ========================== allowed_handles ===========================

CREATE TABLE IF NOT EXISTS allowed_handles_branch
(
    LIKE allowed_handles INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE allowed_handles_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE allowed_handles_branch
DROP CONSTRAINT IF EXISTS allowed_handles_branch_pkey;

ALTER TABLE allowed_handles_branch
ADD CONSTRAINT allowed_handles_branch_pkey
PRIMARY KEY (handle, account_address, producer_block_hash, block_hash);

CREATE INDEX IF NOT EXISTS idx_allowed_handles_branch_chain_handle_block_hash
ON allowed_handles_branch (host_chain_id, handle, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_allowed_handles_branch_chain_block_hash
ON allowed_handles_branch (host_chain_id, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_allowed_handles_branch_chain_block
ON allowed_handles_branch (host_chain_id, block_hash);

CREATE INDEX IF NOT EXISTS idx_allowed_handles_branch_block_number
ON allowed_handles_branch (host_chain_id, block_number);

-- ========================= ciphertext_digest ==========================

CREATE TABLE IF NOT EXISTS ciphertext_digest_branch
(
    LIKE ciphertext_digest INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE ciphertext_digest_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA,
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL DEFAULT NULL,
ADD COLUMN IF NOT EXISTS block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE ciphertext_digest_branch
DROP CONSTRAINT IF EXISTS ciphertext_digest_branch_pkey;

ALTER TABLE ciphertext_digest_branch
ADD CONSTRAINT ciphertext_digest_branch_pkey
PRIMARY KEY (handle, producer_block_hash, block_hash);

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_chain_handle_block_hash
ON ciphertext_digest_branch (host_chain_id, handle, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_chain_block_hash
ON ciphertext_digest_branch (host_chain_id, producer_block_hash);

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_chain_block
ON ciphertext_digest_branch (host_chain_id, block_hash);

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_unsent
ON ciphertext_digest_branch (txn_is_sent, created_at);

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_ciphertext_null
ON ciphertext_digest_branch (ciphertext)
WHERE ciphertext IS NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertext_digest_branch_ciphertext128_null
ON ciphertext_digest_branch (ciphertext128)
WHERE ciphertext128 IS NULL;

-- ============================ ciphertexts ==============================

CREATE TABLE IF NOT EXISTS ciphertexts_branch
(
    LIKE ciphertexts INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE ciphertexts_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE ciphertexts_branch
DROP CONSTRAINT IF EXISTS ciphertexts_branch_pkey;

ALTER TABLE ciphertexts_branch
ADD CONSTRAINT ciphertexts_branch_pkey
PRIMARY KEY (handle, ciphertext_version, producer_block_hash);

-- Orphan cleanup deletes by (handle, producer_block_hash) pairs, covered by
-- the primary key's handle prefix.

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_tenant_handle
ON ciphertexts_branch (tenant_id, handle)
WHERE ciphertext128 IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_created_at
ON ciphertexts_branch (created_at)
WHERE ciphertext128 IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_ciphertext_not_null
ON ciphertexts_branch (handle)
WHERE ciphertext IS NOT NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_ciphertext_null
ON ciphertexts_branch (ciphertext)
WHERE ciphertext IS NULL;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_ciphertext128_null
ON ciphertexts_branch (ciphertext128)
WHERE ciphertext128 IS NULL;

-- =========================== ciphertexts128 ============================

CREATE TABLE IF NOT EXISTS ciphertexts128_branch
(
    LIKE ciphertexts128 INCLUDING DEFAULTS INCLUDING GENERATED INCLUDING IDENTITY INCLUDING STORAGE INCLUDING COMMENTS
);

ALTER TABLE ciphertexts128_branch
ADD COLUMN IF NOT EXISTS producer_block_hash BYTEA NOT NULL DEFAULT ''::BYTEA;

ALTER TABLE ciphertexts128_branch
DROP CONSTRAINT IF EXISTS ciphertexts128_branch_pkey;

ALTER TABLE ciphertexts128_branch
ADD CONSTRAINT ciphertexts128_branch_pkey
PRIMARY KEY (handle, producer_block_hash);
