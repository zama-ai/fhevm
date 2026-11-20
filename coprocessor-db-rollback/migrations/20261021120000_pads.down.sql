-- Down for 20261021120000_pads.
-- Drops the trigger, the function, the index and the table. Pads are lost.
-- The worker columns, indexes and trigger of 20261111120000 are already gone
-- when this file runs.

DROP TRIGGER IF EXISTS pads_ready ON pads;
DROP FUNCTION IF EXISTS notify_pad_ready();
DROP INDEX IF EXISTS idx_pads_pending_by_connector;
DROP TABLE IF EXISTS pads;
