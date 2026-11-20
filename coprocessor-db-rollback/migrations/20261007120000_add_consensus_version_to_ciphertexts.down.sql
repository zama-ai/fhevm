-- Down for 20261007120000_add_consensus_version_to_ciphertexts.
-- Drops the column. The consensus version of each ciphertext is lost.

ALTER TABLE ciphertexts
    DROP COLUMN IF EXISTS consensus_version;
