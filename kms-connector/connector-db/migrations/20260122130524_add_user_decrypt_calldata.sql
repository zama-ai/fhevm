-- Not enforcing NOT NULL for now for backward-compatible processing of decryption after v0.11
-- upgrade.
-- We'll continue trusting the Gateway if we don't have the calldata and aren't able to fully check
-- the ACL between v0.11 and v0.12.
ALTER TABLE user_decryption_requests ADD COLUMN calldata BYTEA;
