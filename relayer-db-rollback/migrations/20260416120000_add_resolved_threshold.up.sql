-- Add resolved_threshold column to user_decrypt_req.
-- Stores the dynamic threshold used when the write side marked the request as completed.
-- Nullable for backward compatibility: old software ignores this column, new rows get the value.
-- NULL means "use static config default" (for rows created before this migration).
ALTER TABLE user_decrypt_req ADD COLUMN resolved_threshold BIGINT;
