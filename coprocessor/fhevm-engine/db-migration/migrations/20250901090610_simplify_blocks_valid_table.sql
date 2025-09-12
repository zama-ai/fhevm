ALTER TABLE IF EXISTS blocks_valid
    DROP COLUMN IF EXISTS listener_tfhe,
    DROP COLUMN IF EXISTS listener_acl;

ALTER TABLE IF EXISTS blocks_valid
    RENAME TO host_chain_blocks_valid;
