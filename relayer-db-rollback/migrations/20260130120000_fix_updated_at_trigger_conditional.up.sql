-- Fix updated_at trigger to only update timestamp on actual row changes
--
-- Problem: The trigger_set_timestamp() function fires on ALL updates, including
-- ON CONFLICT DO UPDATE clauses that use self-assignment (e.g., updated_at = table.updated_at).
-- This incorrectly updates updated_at even when no actual data changed.
--
-- Solution: Check if any columns changed before updating updated_at using IS DISTINCT FROM.

CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$
BEGIN
  -- Only update updated_at if row actually changed (not a no-op conflict resolution)
  IF (OLD IS DISTINCT FROM NEW) THEN
    NEW.updated_at = NOW();
  END IF;
  RETURN NEW;
END;
$$ LANGUAGE plpgsql;
