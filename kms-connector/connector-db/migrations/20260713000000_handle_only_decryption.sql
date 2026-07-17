-- The KMS Connector now subscribes to the handle-only overloaded `PublicDecryptionRequest` /
-- `UserDecryptionRequest` events, which carry only the ciphertext handles.

--   * `sns_ct_materials` is kept — only its NOT NULL constraint is dropped — so an older connector
--     that still writes/reads it keeps working; the new connector simply leaves it NULL.
--   * `ct_handles` is added nullable with no default so an older connector that doesn't know about
--     the column can still INSERT; the new connector always populates it. The row reader treats a
--     NULL `ct_handles` as an empty handle list, which fails the request closed rather than
--     processing it with unknown material.

ALTER TABLE public_decryption_requests ALTER COLUMN sns_ct_materials DROP NOT NULL;
ALTER TABLE public_decryption_requests ADD COLUMN ct_handles BYTEA[];

ALTER TABLE user_decryption_requests ALTER COLUMN sns_ct_materials DROP NOT NULL;
ALTER TABLE user_decryption_requests ADD COLUMN ct_handles BYTEA[];

-- Backfill in-flight requests written by the previous connector version
-- We populate `ct_handles` so these requests survive the cutover and flow through the
-- authoritative verifier instead of being dropped.
--
-- Element order is preserved via WITH ORDINALITY + ORDER BY so that, for v2 (RFC016) rows,
-- `ct_handles[i]` stays aligned with the parallel `handle_owner_addresses[i]` /
-- `handle_contract_addresses[i]` arrays.
-- `unnest` on a composite array flattens each element into its member columns, so `m` exposes
-- `sns_ciphertext_material`'s fields directly (`m.ct_handle`) plus the `ordinality` counter.
--
-- Scope: only `pending` / `under_process` rows need this — they are the ones the new worker will
-- (re)process.
UPDATE public_decryption_requests
SET ct_handles = ARRAY(
    SELECT m.ct_handle
    FROM unnest(sns_ct_materials) WITH ORDINALITY AS m
    ORDER BY m.ordinality
)
WHERE ct_handles IS NULL AND status IN ('pending', 'under_process');

UPDATE user_decryption_requests
SET ct_handles = ARRAY(
    SELECT m.ct_handle
    FROM unnest(sns_ct_materials) WITH ORDINALITY AS m
    ORDER BY m.ordinality
)
WHERE ct_handles IS NULL AND status IN ('pending', 'under_process');
