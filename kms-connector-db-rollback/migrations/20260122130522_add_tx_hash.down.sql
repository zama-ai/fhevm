-- Down for 20260122130522_add_tx_hash.
--
-- Drops the `tx_hash` column from the seven request tables. The stored
-- transaction hashes are lost; the previous version does not use them.

ALTER TABLE public_decryption_requests DROP COLUMN tx_hash;
ALTER TABLE user_decryption_requests DROP COLUMN tx_hash;
ALTER TABLE prep_keygen_requests DROP COLUMN tx_hash;
ALTER TABLE keygen_requests DROP COLUMN tx_hash;
ALTER TABLE crsgen_requests DROP COLUMN tx_hash;
ALTER TABLE prss_init DROP COLUMN tx_hash;
ALTER TABLE key_reshare_same_set DROP COLUMN tx_hash;
