-- Down for 20260917120000_healing_work_notify.
-- Drops the two triggers and the function. No data impact. 20260918120000,
-- which replaced the triggers, is already reverted when this file runs, so the
-- triggers dropped here are the ones this migration created.

DROP TRIGGER IF EXISTS drifted_handle_healing_work_update ON drifted_handle;
DROP TRIGGER IF EXISTS drifted_handle_healing_work_insert ON drifted_handle;
DROP FUNCTION IF EXISTS notify_healing_work();
