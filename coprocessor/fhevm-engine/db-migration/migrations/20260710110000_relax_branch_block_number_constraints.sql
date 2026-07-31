-- Wave 2 permits branchless compatibility rows to retain a known block
-- number. Branchful rows must still carry the producer height used by the
-- settled-height write guards.

ALTER TABLE ciphertext_digest_branch
DROP CONSTRAINT IF EXISTS ciphertext_digest_branch_producer_block_number_check;

ALTER TABLE ciphertext_digest_branch
ADD CONSTRAINT ciphertext_digest_branch_producer_block_number_check
CHECK (producer_block_hash = ''::BYTEA OR block_number IS NOT NULL) NOT VALID;

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
