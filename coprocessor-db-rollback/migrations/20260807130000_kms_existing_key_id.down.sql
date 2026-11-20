-- Down for 20260807130000_kms_existing_key_id.
-- Drops the column. The existing key id recorded on activation events is lost.

ALTER TABLE kms_key_activation_events
    DROP COLUMN IF EXISTS existing_key_id;
