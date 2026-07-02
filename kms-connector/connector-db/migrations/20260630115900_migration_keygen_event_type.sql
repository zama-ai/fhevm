-- RFC-029: register the migration keygen as its own event type so the gw-listener tracks its block
-- cursor and the worker routes it. ADD VALUE must land in its own migration (transaction): Postgres
-- forbids using a freshly added enum value in the same transaction that adds it, so the
-- last_block_polled seed lives in the following migration.
ALTER TYPE event_type ADD VALUE IF NOT EXISTS 'PrepMigrationKeygenRequest';
ALTER TYPE event_type ADD VALUE IF NOT EXISTS 'MigrationKeygenRequest';
