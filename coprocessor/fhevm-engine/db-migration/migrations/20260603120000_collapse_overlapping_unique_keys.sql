-- Collapse the overlapping unique keys on `ciphertext_digest`, `computations`,
-- and `ciphertexts` down to a single unique key per table.
--
-- Since `remove_tenants`, `tenant_id` is a constant default, yet these three
-- tables each still carry TWO overlapping unique indexes: a vestigial
-- tenant-prefixed PRIMARY KEY plus a tenant-free UNIQUE index that the workers
-- actually target:
--
--   table              vestigial PK (tenant-prefixed)               tenant-free UNIQUE index
--   ciphertext_digest  (tenant_id, handle)                          idx_ciphertext_digest_no_tenant (handle)
--   computations       (tenant_id, output_handle, transaction_id)   idx_computations_no_tenant (output_handle, transaction_id)
--   ciphertexts        (tenant_id, handle, ciphertext_version)      idx_ciphertexts_no_tenant (handle, ciphertext_version)
--
-- Each writer upserts with `INSERT ... ON CONFLICT (<tenant-free cols>)`, whose
-- arbiter covers only the tenant-free index:
--   * sns-worker      ON CONFLICT (handle) DO UPDATE              -> ciphertext_digest
--   * host-listener   ON CONFLICT (output_handle, transaction_id) -> computations
--   * tfhe-worker     ON CONFLICT (handle, ciphertext_version)    -> ciphertexts
--
-- When a state/drift revert runs `DELETE FROM <table>` concurrently with such an
-- upsert, the duplicate can surface on the *other* unique index (the composite
-- primary key), which the arbiter does not cover, raising a hard
-- `duplicate key value violates unique constraint "..._pkey"` (SQLSTATE 23505)
-- instead of being absorbed by the ON CONFLICT clause. That crashed the
-- sns-worker (see zama-ai/fhevm-internal#1495); host-listener and tfhe-worker
-- share the same upsert+revert shape, so the same race is reachable on
-- `computations` and `ciphertexts` too.
--
-- Collapsing each table to a single unique key (its tenant-free index) removes
-- the blind spot: the ON CONFLICT arbiter then covers the table's only unique
-- index, so any conflict is always absorbed by the upsert. Dropping `tenant_id`
-- from the key is safe because handles already embed `chain_id`, so the
-- tenant-free columns are globally unique on their own.
--
-- This is catalog-only: each tenant-free unique index already exists and its
-- columns are already NOT NULL (they came from the old composite primary keys),
-- so `ADD PRIMARY KEY USING INDEX` promotes the existing index in place with no
-- table rewrite, scan, or backfill (verified on PostgreSQL 15.7: relfilenode
-- unchanged, sub-millisecond on 500k rows). Each statement still briefly takes an
-- ACCESS EXCLUSIVE lock, so apply with a bounded `lock_timeout` + retry rather
-- than letting it queue behind long-running transactions. `tenant_id` is kept as
-- a plain column on every table.

ALTER TABLE ciphertext_digest DROP CONSTRAINT IF EXISTS ciphertext_digest_pkey;
ALTER TABLE ciphertext_digest
    ADD CONSTRAINT ciphertext_digest_pkey PRIMARY KEY USING INDEX idx_ciphertext_digest_no_tenant;

ALTER TABLE computations DROP CONSTRAINT IF EXISTS computations_pkey;
ALTER TABLE computations
    ADD CONSTRAINT computations_pkey PRIMARY KEY USING INDEX idx_computations_no_tenant;

ALTER TABLE ciphertexts DROP CONSTRAINT IF EXISTS ciphertexts_pkey;
ALTER TABLE ciphertexts
    ADD CONSTRAINT ciphertexts_pkey PRIMARY KEY USING INDEX idx_ciphertexts_no_tenant;
