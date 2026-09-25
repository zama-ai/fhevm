-- Healing gives up on a finding after too many failed attempts, or at once
-- when it can never succeed. An abandoned finding is no longer picked; reset
-- both columns to heal it again.
ALTER TABLE drifted_handle
    ADD COLUMN heal_attempts INTEGER NOT NULL DEFAULT 0 CHECK (heal_attempts >= 0),
    ADD COLUMN heal_abandoned_at TIMESTAMPTZ NULL;
