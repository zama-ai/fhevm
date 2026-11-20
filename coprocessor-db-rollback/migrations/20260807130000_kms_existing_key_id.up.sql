ALTER TABLE kms_key_activation_events
    ADD COLUMN IF NOT EXISTS existing_key_id BYTEA;
