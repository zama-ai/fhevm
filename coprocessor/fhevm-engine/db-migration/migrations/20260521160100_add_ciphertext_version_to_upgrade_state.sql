-- The L1 `event_upgrade_activated` payload carries the ciphertext_version of
-- the upgrade target. We persist it on the upgrade_state row so the cutover
-- step (execute_cutover) can apply it to the `versioning` table inside the
-- exclusive advisory-lock transaction.
ALTER TABLE upgrade_state
    ADD COLUMN IF NOT EXISTS ciphertext_version SMALLINT NULL CHECK (ciphertext_version >= 0);
