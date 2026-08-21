-- Host-generic V2 user-decryption: the gateway's `userDecryptionRequestV2` entry carries the
-- fields the gateway itself consumes (handles, validity window, transport key, KMS routing) as
-- typed event fields, and everything host-specific as one opaque `hostPayload`. On the connector
-- side that means two new columns and the removal of the RFC-021 Solana typed columns, whose
-- content now lives inside `host_payload`.
--
--   - `host_kind`   : the host-kind discriminator (SMALLINT; the event's uint8). NULL for legacy
--                     and RFC016 EVM rows. `from_user_decryption_row` identifies a V2 row by
--                     `host_kind IS NOT NULL` and reads it BEFORE the signature-based EVM
--                     discriminator, because a V2 row has no top-level signature/owner/contract.
--   - `host_payload`: the opaque canonical request bytes the gateway forwarded verbatim. The
--                     worker decodes it with the canonical `hostPayload` codec and authorizes it.
--   - `allowed_acl_domain_key_count`: the event's uint8 declaration (SMALLINT) of the length of
--                     the ACL-scope list signed inside `host_payload`. The gateway bounds the
--                     declaration (<= 10) before the fee — the EVM paths' `allowedContracts`
--                     rule, kept without reading the opaque payload; the worker admits a request
--                     only when the declaration equals the signed list's actual length, so the
--                     declaration cannot lie usefully. NULL for legacy and RFC016 EVM rows.
ALTER TABLE user_decryption_requests ADD COLUMN host_kind SMALLINT;
ALTER TABLE user_decryption_requests ADD COLUMN host_payload BYTEA;
ALTER TABLE user_decryption_requests ADD COLUMN allowed_acl_domain_key_count SMALLINT;

-- The RFC-021 Solana typed columns are gone: their content is inside `host_payload` now, and no
-- code path reads them after the V2 cutover. This is a PoC branch with no rows to preserve.
ALTER TABLE user_decryption_requests DROP COLUMN solana_identity;
ALTER TABLE user_decryption_requests DROP COLUMN solana_nonce;
ALTER TABLE user_decryption_requests DROP COLUMN solana_allowed_acl_domain_keys;
