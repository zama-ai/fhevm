-- Down for 20260725120000_upgrade_state_per_chain_windows.
--
-- Restores one upgrade_state row per stack role, with the primary key on
-- stack_role and host_chain_id nullable, as before the migration. The
-- migration deleted the rows without a host chain, which cannot be recreated.
--
-- The v0.14.x schema allows one row per stack role. A v0.15.x database has one
-- row per (stack_role, host_chain_id). This file refuses to run while a stack
-- role has more than one row, because choosing which row survives is a data
-- decision. With the `coprocessor_db_rollback.force` session setting on, it
-- keeps the row with the lowest host_chain_id of each stack role and deletes
-- the others.

DO $$
DECLARE
    force_enabled BOOLEAN := current_setting('coprocessor_db_rollback.force', true) = 'on';
    duplicated TEXT;
BEGIN
    SELECT string_agg(stack_role || ':' || n, ', ')
      INTO duplicated
      FROM (
          SELECT stack_role, count(*) AS n
            FROM upgrade_state
           GROUP BY stack_role
          HAVING count(*) > 1
      ) multi;

    IF duplicated IS NOT NULL THEN
        IF NOT force_enabled THEN
            RAISE EXCEPTION
                'Refusing to revert 20260725120000_upgrade_state_per_chain_windows: a stack role has more than one upgrade_state row (%). Delete the rows for the chains that must not survive the rollback, or run the rollback with --force to keep only the row with the lowest host_chain_id per stack role.',
                duplicated;
        END IF;

        DELETE FROM upgrade_state u
         USING (
             SELECT stack_role, min(host_chain_id) AS keep
               FROM upgrade_state
              GROUP BY stack_role
         ) k
         WHERE u.stack_role = k.stack_role
           AND u.host_chain_id <> k.keep;
    END IF;
END
$$;

DROP INDEX IF EXISTS upgrade_state_attempt_idx;

ALTER TABLE upgrade_state
    DROP CONSTRAINT IF EXISTS upgrade_state_pkey;

ALTER TABLE upgrade_state
    ADD PRIMARY KEY (stack_role);

ALTER TABLE upgrade_state
    ALTER COLUMN host_chain_id DROP NOT NULL;
