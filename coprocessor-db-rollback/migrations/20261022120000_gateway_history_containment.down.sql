-- Down for 20261022120000_gateway_history_containment.
--
-- Restores the source constraints to 'host' only, drops the checkpoint table
-- and drops progress_revision from every gw_listener_last_block table (public
-- and the gcs-* schemas of the Blue/Green stacks).
--
-- Rows with source = 'gateway' cannot exist under the restored constraints.
-- This file refuses to run while there are any. With the
-- `coprocessor_db_rollback.force` session setting on, it deletes them.

DO $$
DECLARE
    force_enabled BOOLEAN := current_setting('coprocessor_db_rollback.force', true) = 'on';
    gateway_rows BIGINT;
BEGIN
    SELECT (SELECT count(*) FROM public.chain_recovery_halts WHERE source = 'gateway')
         + (SELECT count(*) FROM public.listener_replay_state WHERE source = 'gateway')
      INTO gateway_rows;

    IF gateway_rows > 0 THEN
        IF NOT force_enabled THEN
            RAISE EXCEPTION
                'Refusing to revert 20261022120000_gateway_history_containment: % row(s) with source = ''gateway'' in chain_recovery_halts or listener_replay_state. Resolve them first, or run the rollback with --force to delete them.',
                gateway_rows;
        END IF;
        DELETE FROM public.chain_recovery_halts WHERE source = 'gateway';
        DELETE FROM public.listener_replay_state WHERE source = 'gateway';
    END IF;
END
$$;

ALTER TABLE public.chain_recovery_halts
    DROP CONSTRAINT IF EXISTS chain_recovery_halts_source_check;
ALTER TABLE public.chain_recovery_halts
    ADD CONSTRAINT chain_recovery_halts_source_check
    CHECK (source = 'host');

ALTER TABLE public.listener_replay_state
    DROP CONSTRAINT IF EXISTS listener_replay_state_source_check;
ALTER TABLE public.listener_replay_state
    ADD CONSTRAINT listener_replay_state_source_check
    CHECK (source = 'host');

DROP TABLE IF EXISTS public.listener_block_checkpoints;

ALTER TABLE public.gw_listener_last_block
    DROP COLUMN IF EXISTS progress_revision;

DO $$
DECLARE green_schema TEXT;
BEGIN
    FOR green_schema IN
        SELECT table_schema FROM information_schema.tables
        WHERE table_name = 'gw_listener_last_block' AND table_schema LIKE 'gcs-%'
    LOOP
        EXECUTE format(
            'ALTER TABLE %I.gw_listener_last_block DROP COLUMN IF EXISTS progress_revision',
            green_schema
        );
    END LOOP;
END $$;
