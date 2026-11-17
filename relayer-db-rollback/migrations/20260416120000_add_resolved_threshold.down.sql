-- Down for 20260416120000_add_resolved_threshold.
--
-- Drops the resolved_threshold column. The values stored in it are lost; the
-- previous relayer version does not know the column.

ALTER TABLE user_decrypt_req DROP COLUMN resolved_threshold;
