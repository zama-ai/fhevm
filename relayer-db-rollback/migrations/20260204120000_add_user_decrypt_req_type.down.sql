-- Down for 20260204120000_add_user_decrypt_req_type.
--
-- Drops the req_type column, its index and its enum type. The values stored in
-- the column are lost; the previous relayer version does not know the column.

DROP INDEX idx_user_decrypt_req_req_type;

ALTER TABLE user_decrypt_req DROP COLUMN req_type;

DROP TYPE user_decrypt_req_type;
