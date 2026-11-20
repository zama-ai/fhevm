-- Down for 20251027122128_add_already_sent.
--
-- Drops the `already_sent` column from the five request tables. The flags are
-- lost; the previous version does not use them.

ALTER TABLE public_decryption_requests DROP COLUMN already_sent;
ALTER TABLE user_decryption_requests DROP COLUMN already_sent;
ALTER TABLE prep_keygen_requests DROP COLUMN already_sent;
ALTER TABLE keygen_requests DROP COLUMN already_sent;
ALTER TABLE crsgen_requests DROP COLUMN already_sent;
