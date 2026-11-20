-- Not enforcing NOT NULL for now for backward-compatible processing of decryption after v0.11
-- upgrade.
-- We'll continue trusting the Gateway if we don't have the tx_hash, as we would not be able to
-- fully check the ACL between v0.11 and v0.12 without this.
-- This is tracked by this issue: https://github.com/zama-ai/fhevm-internal/issues/916.
ALTER TABLE public_decryption_requests ADD COLUMN tx_hash BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN tx_hash BYTEA;
ALTER TABLE prep_keygen_requests ADD COLUMN tx_hash BYTEA;
ALTER TABLE keygen_requests ADD COLUMN tx_hash BYTEA;
ALTER TABLE crsgen_requests ADD COLUMN tx_hash BYTEA;
ALTER TABLE prss_init ADD COLUMN tx_hash BYTEA;
ALTER TABLE key_reshare_same_set ADD COLUMN tx_hash BYTEA;
