-- Add the `unified` value to the user_decrypt_req_type enum so unified
-- EIP-712 user-decryption jobs can be persisted alongside the existing
-- `user_decrypt` and `delegated_user_decrypt` request types.
ALTER TYPE user_decrypt_req_type ADD VALUE IF NOT EXISTS 'unified';

COMMENT ON COLUMN user_decrypt_req.req_type IS
  'DEPRECATED: should be dropped once the legacy EIP-712 formats '
  '(direct + delegated) are deprecated. After that, "unified" is the '
  'only value and the column is redundant.';
