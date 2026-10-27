-- A manifest seal lives in shared `public`. The inventory it commits lives in
-- the sealing stack's own schema. Record which stack sealed the block. The
-- replay guard then skips a seal the other stack wrote.

-- The schema this connection writes unqualified rows into: `public` for blue,
-- the versioned GCS schema for green. A duplicated table resolves it through
-- the caller's `search_path`. `current_schema()` cannot serve: blue keeps the
-- server default of `"$user", public`.
CREATE OR REPLACE FUNCTION public.stack_schema() RETURNS TEXT
    LANGUAGE sql STABLE AS $$
    SELECT n.nspname::TEXT
      FROM pg_class c
      JOIN pg_namespace n ON n.oid = c.relnamespace
     WHERE c.oid = to_regclass('state_hash')
$$;

-- NULL until a stack seals the block. Manifest publication arrives with this
-- release, so no stored seal predates the stamping code and nothing needs a
-- backfill.
ALTER TABLE block_manifest_state
    ADD COLUMN IF NOT EXISTS sealed_by_schema TEXT NULL
        CHECK (sealed_by_schema IS NULL OR sealed_by_schema <> '');

-- An unattributed seal silently loses the guard, so make it unrepresentable.
-- The migration then fails on a pre-release database that holds a seal. Green
-- owns the only detector there, so clear that seal rather than relabel it.
ALTER TABLE block_manifest_state
    ADD CONSTRAINT block_manifest_state_seal_is_attributed CHECK (
        sealed_by_schema IS NOT NULL
        OR (block_content_digest IS NULL
            AND block_handle_count IS NULL
            AND NOT manifest_published)
    );
