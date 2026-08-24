-- Solana user-decryption: the gateway's Solana entry carries the fields the gateway itself
-- consumes (handles, validity window, transport key, KMS routing) as typed event fields, and
-- everything else as one opaque request blob. On the connector side that means one new column
-- and the removal of the RFC-021 Solana typed columns, whose content now lives inside it.
--
--   - `solana_request`: the opaque canonical request bytes the gateway forwarded verbatim. The
--                     worker decodes it with the shared `zama-solana-request` codec and
--                     authorizes it. NULL for legacy and RFC016 EVM rows, which is what
--                     `from_user_decryption_row` keys on to identify a Solana row — it reads
--                     that BEFORE the signature-based EVM discriminator, because a Solana row
--                     has no top-level signature/subject/contract.
--
-- There is deliberately no host-kind column and no declared ACL-scope length. Both were
-- self-declarations about a blob the gateway cannot read: a truthful one adds nothing the
-- worker does not already check against the signature, and a false one is refused there anyway.
ALTER TABLE user_decryption_requests ADD COLUMN solana_request BYTEA;

-- The RFC-021 Solana typed columns are gone: their content is inside `host_payload` now, and no
-- code path reads them after the V2 cutover. This is a PoC branch with no rows to preserve.
ALTER TABLE user_decryption_requests DROP COLUMN solana_identity;
ALTER TABLE user_decryption_requests DROP COLUMN solana_nonce;
ALTER TABLE user_decryption_requests DROP COLUMN solana_allowed_acl_domain_keys;
