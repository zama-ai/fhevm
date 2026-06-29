-- Hybrid RFC-011 settlement support for branch-context storage.
--
-- Branch ciphertext rows keep their efficient `(handle, producer_block_hash)`
-- keying, but now also carry a nullable producer block height so workers can
-- apply a monotonic settled-height write guard. Branchless rows remain
-- NULL-height: they are valid on every branch and outside block settlement.

CREATE TABLE IF NOT EXISTS coprocessor_settlement
(
    chain_id BIGINT PRIMARY KEY,
    settled_height BIGINT NOT NULL DEFAULT -1,
    updated_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT coprocessor_settlement_height_check CHECK (settled_height >= -1)
);

ALTER TABLE ciphertexts_branch
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL;

ALTER TABLE ciphertexts128_branch
ADD COLUMN IF NOT EXISTS block_number BIGINT NULL;

ALTER TABLE ciphertexts_branch
DROP CONSTRAINT IF EXISTS ciphertexts_branch_producer_block_number_check;

ALTER TABLE ciphertexts_branch
ADD CONSTRAINT ciphertexts_branch_producer_block_number_check
CHECK (producer_block_hash = ''::BYTEA OR block_number IS NOT NULL) NOT VALID;

ALTER TABLE ciphertexts128_branch
DROP CONSTRAINT IF EXISTS ciphertexts128_branch_producer_block_number_check;

ALTER TABLE ciphertexts128_branch
ADD CONSTRAINT ciphertexts128_branch_producer_block_number_check
CHECK (producer_block_hash = ''::BYTEA OR block_number IS NOT NULL) NOT VALID;

CREATE INDEX IF NOT EXISTS idx_ciphertexts_branch_block_number
ON ciphertexts_branch (block_number)
WHERE producer_block_hash <> ''::BYTEA;

CREATE INDEX IF NOT EXISTS idx_ciphertexts128_branch_block_number
ON ciphertexts128_branch (block_number)
WHERE producer_block_hash <> ''::BYTEA;
