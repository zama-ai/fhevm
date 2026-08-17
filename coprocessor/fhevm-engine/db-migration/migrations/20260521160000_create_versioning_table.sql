-- Singleton table holding the currently active stack version.
--
-- The row is updated in-place during cutover, inside the same transaction
-- that holds the exclusive cutover advisory lock. There is exactly one row,
-- enforced by `singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton)`.
CREATE TABLE IF NOT EXISTS versioning (
    singleton          BOOLEAN  PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    stack_version      TEXT     NOT NULL,
    updated_at         TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Seed with the current stack version (informational; the live row is the
-- source of truth used by `resolve_gcs_mode` to decide blue/green routing).
INSERT INTO versioning (singleton, stack_version)
VALUES (TRUE, 'v0.14')
ON CONFLICT (singleton) DO NOTHING;
