-- RFC-021: carry the Solana user-decrypt ed25519 auth fields as TYPED columns instead of packing
-- them into the `extra_data` blob (the old version-`0x03` format). After this, the `0x03` blob does
-- not exist anywhere in the connector: `extra_data` carries only the KMS context (v0x01) on the
-- Solana path, exactly like EVM, and the worker reads the auth fields from these columns.
--
-- All columns are nullable, mirroring the RFC016 single-table strategy: NULL for legacy and EVM
-- rows, populated for RFC-021 Solana rows. Variant discriminator: a row with `signature IS NOT NULL`
-- AND `solana_identity IS NOT NULL` is a Solana row (`from_user_decryption_row` keys on this);
-- `signature IS NOT NULL` with `solana_identity IS NULL` is an EVM RFC016 row; `signature IS NULL`
-- is a legacy row.
--
--   - `solana_identity`               : the 32-byte ed25519 identity pubkey.
--   - `solana_nonce`                  : the 32-byte per-request anti-replay nonce.
--   - `solana_allowed_acl_domain_keys`: the allowed Solana ACL domain keys (32 bytes each), the
--                                       Solana analog of `allowed_contracts`. May be empty ('{}').
ALTER TABLE user_decryption_requests ADD COLUMN solana_identity BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN solana_nonce BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN solana_allowed_acl_domain_keys BYTEA[];
