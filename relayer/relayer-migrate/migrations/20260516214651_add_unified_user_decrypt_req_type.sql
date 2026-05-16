-- Add the `unified` variant to the user_decrypt_req_type enum so v3
-- (unified EIP-712 user-decryption) jobs can be persisted alongside the
-- existing `user_decrypt` and `delegated_user_decrypt` v2 dialects.
ALTER TYPE user_decrypt_req_type ADD VALUE IF NOT EXISTS 'unified';
