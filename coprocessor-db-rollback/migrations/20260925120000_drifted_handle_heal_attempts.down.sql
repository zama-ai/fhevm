-- Down for 20260925120000_drifted_handle_heal_attempts.
-- Drops the two columns. Heal attempt counters are lost.

ALTER TABLE drifted_handle
    DROP COLUMN IF EXISTS heal_abandoned_at,
    DROP COLUMN IF EXISTS heal_attempts;
