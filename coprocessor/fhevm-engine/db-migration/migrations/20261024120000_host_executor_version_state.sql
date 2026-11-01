-- Persist the executor version known at each observed host block. This lets a
-- restarted listener continue from a canonical parent without guessing from
-- the current proxy implementation; block_hash keeps competing forks distinct.
ALTER TABLE host_chain_blocks_valid
    ADD COLUMN IF NOT EXISTS tfhe_executor_address BYTEA,
    ADD COLUMN IF NOT EXISTS tfhe_executor_version TEXT;

ALTER TABLE host_chain_blocks_valid
    ADD CONSTRAINT host_chain_blocks_valid_executor_address_length
        CHECK (tfhe_executor_address IS NULL OR octet_length(tfhe_executor_address) = 20),
    ADD CONSTRAINT host_chain_blocks_valid_executor_version_format
        CHECK (
            tfhe_executor_version IS NULL
            OR tfhe_executor_version ~ '^FHEVMExecutor v[0-9]+[.][0-9]+[.][0-9]+$'
        );
