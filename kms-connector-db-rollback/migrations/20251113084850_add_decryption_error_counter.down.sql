-- Down for 20251113084850_add_decryption_error_counter.
--
-- Drops the `error_counter` column from the two decryption request tables. The
-- counters are lost; the previous version does not use them.

ALTER TABLE public_decryption_requests DROP COLUMN error_counter;
ALTER TABLE user_decryption_requests DROP COLUMN error_counter;
