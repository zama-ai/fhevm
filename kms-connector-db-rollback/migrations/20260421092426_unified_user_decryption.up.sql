-- RFC016 (Unified EIP-712 Decryption Request) — single-table strategy.
--
-- The legacy and RFC016 `UserDecryptionRequest` events are persisted in the same
-- `user_decryption_requests` table. All RFC016-specific columns added here are nullable: they are
-- NULL for legacy rows and populated for RFC016 rows.
--
-- Variant discriminator invariant: `signature IS NULL` identifies a legacy row; `signature IS NOT
-- NULL` identifies an RFC016 row. This is the ONLY place the two variants are distinguished at the
-- SQL level; the row reader (`from_user_decryption_row`) inspects this column to pick the correct
-- `ProtocolEventKind` variant, and everything downstream pattern-matches on the Rust enum.
--
-- `allowed_contracts` semantics:
--   - NULL         -> legacy row (no top-level contract restriction concept).
--   - '{}'         -> RFC016 permissive mode (any contract allowed).
--   - non-empty    -> RFC016 with a specific allow-list.
--
-- `handle_owner_addresses` and `handle_contract_addresses` are parallel arrays aligned with
-- `sns_ct_materials`: entry i describes the owner/contract for handle i.

ALTER TABLE user_decryption_requests ADD COLUMN handle_owner_addresses BYTEA[];
ALTER TABLE user_decryption_requests ADD COLUMN handle_contract_addresses BYTEA[];
ALTER TABLE user_decryption_requests ADD COLUMN allowed_contracts BYTEA[];
ALTER TABLE user_decryption_requests ADD COLUMN start_timestamp BIGINT;
ALTER TABLE user_decryption_requests ADD COLUMN duration_seconds BIGINT;
ALTER TABLE user_decryption_requests ADD COLUMN signature BYTEA;
