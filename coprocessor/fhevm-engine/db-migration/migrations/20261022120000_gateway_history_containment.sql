-- Gateway history uses the same database-wide containment state as host
-- history, but retains its own source identity and block checkpoints.
ALTER TABLE public.chain_recovery_halts
    DROP CONSTRAINT IF EXISTS chain_recovery_halts_source_check;
ALTER TABLE public.chain_recovery_halts
    ADD CONSTRAINT chain_recovery_halts_source_check
    CHECK (source IN ('host', 'gateway'));

ALTER TABLE public.listener_replay_state
    DROP CONSTRAINT IF EXISTS listener_replay_state_source_check;
ALTER TABLE public.listener_replay_state
    ADD CONSTRAINT listener_replay_state_source_check
    CHECK (source IN ('host', 'gateway'));

-- A Gateway cursor is only a height. Keep the canonical identity separately so
-- a same-height replacement cannot leave stale proof rows behind unnoticed.
CREATE TABLE public.listener_block_checkpoints (
    source TEXT NOT NULL CHECK (source IN ('host', 'gateway')),
    stack_role TEXT NOT NULL CHECK (stack_role IN ('BCS', 'GCS')),
    chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    block_hash BYTEA NOT NULL CHECK (octet_length(block_hash) = 32),
    parent_hash BYTEA CHECK (octet_length(parent_hash) = 32),
    PRIMARY KEY (source, stack_role, chain_id, block_number)
);

-- Random revisions prevent a scan that started before a rewind, cutover, or
-- schema recreation from later committing an obsolete Gateway watermark.
ALTER TABLE public.gw_listener_last_block
    ADD COLUMN IF NOT EXISTS progress_revision UUID NOT NULL DEFAULT gen_random_uuid();

DO $$
DECLARE green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT table_schema FROM information_schema.tables
        WHERE table_name = 'gw_listener_last_block' AND table_schema LIKE 'gcs-%'
    LOOP
        EXECUTE format(
            'ALTER TABLE %I.gw_listener_last_block ADD COLUMN IF NOT EXISTS progress_revision UUID NOT NULL DEFAULT gen_random_uuid()',
            green_schema
        );
    END LOOP;
END $$;
