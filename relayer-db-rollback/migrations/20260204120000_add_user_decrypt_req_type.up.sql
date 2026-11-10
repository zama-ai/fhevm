-- Migration to add user_decrypt_req_type enum and req_type column
-- This allows distinguishing between legacy, regular user decrypt, and delegated user decrypt requests

-- Create the enum type for request types
CREATE TYPE user_decrypt_req_type AS ENUM ('legacy', 'user_decrypt', 'delegated_user_decrypt');

-- Add the req_type column to user_decrypt_req table
-- Default to 'legacy' for existing rows
ALTER TABLE user_decrypt_req
ADD COLUMN req_type user_decrypt_req_type NOT NULL DEFAULT 'legacy';

-- Create an index on req_type for efficient filtering
CREATE INDEX idx_user_decrypt_req_req_type ON user_decrypt_req (req_type);

-- Note: After 7 days when legacy requests are cleaned up by the expiry cron,
-- we can remove the 'legacy' variant from the enum in a future migration.
