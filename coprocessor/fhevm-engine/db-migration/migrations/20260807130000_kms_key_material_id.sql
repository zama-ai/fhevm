ALTER TABLE kms_key_activation_events
    ADD COLUMN IF NOT EXISTS key_material_id BYTEA;
