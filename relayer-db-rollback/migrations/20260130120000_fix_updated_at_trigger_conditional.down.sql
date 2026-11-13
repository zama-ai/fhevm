-- Down for 20260130120000_fix_updated_at_trigger_conditional.
--
-- The up made trigger_set_timestamp() bump updated_at only when the row
-- actually changed. This restores the baseline body, which bumps it on every
-- UPDATE. CREATE OR REPLACE keeps the function's identity, so the triggers that
-- call it need no change.

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  NEW.updated_at = NOW();
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
