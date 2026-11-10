-- Down for 20260825120000_add_owner_epoch.
--
-- Drops the owner_epoch column from the four tables the up added it to. The
-- column comments go with the columns. The stored epochs are lost; the previous
-- relayer version does not know the column.

ALTER TABLE gateway_chain_cursor DROP COLUMN owner_epoch;

ALTER TABLE input_proof_req DROP COLUMN owner_epoch;

ALTER TABLE public_decrypt_req DROP COLUMN owner_epoch;

ALTER TABLE user_decrypt_req DROP COLUMN owner_epoch;
