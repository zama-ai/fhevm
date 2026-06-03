-- Make `handle` the sole unique key on `ciphertext_digest`.
--
-- Since `remove_tenants`, `tenant_id` is a constant default and the table has
-- carried two overlapping unique indexes:
--   * `ciphertext_digest_pkey`            PRIMARY KEY (tenant_id, handle)  -- vestigial
--   * `idx_ciphertext_digest_no_tenant`   UNIQUE (handle)
--
-- The SnS upsert is `INSERT ... ON CONFLICT (handle) DO UPDATE`, whose arbiter
-- covers only the (handle) index. When a state/drift revert runs
-- `DELETE FROM ciphertext_digest` concurrently with that upsert, the duplicate
-- can surface on the *other* unique index (the composite primary key), which the
-- arbiter does not cover, raising a hard
-- `duplicate key value violates unique constraint "ciphertext_digest_pkey"`
-- (SQLSTATE 23505). That error crashed the sns-worker (see
-- zama-ai/fhevm-internal#1495).
--
-- Collapsing to a single unique key on (handle) removes the blind spot: the
-- ON CONFLICT arbiter then covers the table's only unique index, so any conflict
-- is always absorbed by the upsert. Verified locally: with this single-unique-key
-- schema the concurrent upsert+DELETE race produces zero duplicate-key errors.
--
-- This is catalog-only: the (handle) unique index already exists, so promoting it
-- to the primary key requires no table rewrite or backfill. `tenant_id` is kept as
-- a plain column. The non-unique `idx_ciphertext_digest_handle` is now redundant
-- with the new (handle) primary key but is left in place (harmless; can be dropped
-- in a follow-up).

ALTER TABLE ciphertext_digest DROP CONSTRAINT IF EXISTS ciphertext_digest_pkey;

ALTER TABLE ciphertext_digest
    ADD CONSTRAINT ciphertext_digest_pkey PRIMARY KEY USING INDEX idx_ciphertext_digest_no_tenant;
