-- Add compressed-server-key columns alongside the existing
-- decompressed ones.  New rows produced from a CompressedXofKeySet
-- populate both column groups for now

ALTER TABLE keys
    ADD COLUMN IF NOT EXISTS sks_key_compressed BYTEA,
    ADD COLUMN IF NOT EXISTS sns_pk_compressed OID;

ALTER TABLE kms_key_activation_events
    ADD COLUMN IF NOT EXISTS key_content_sks_key_compressed BYTEA,
    ADD COLUMN IF NOT EXISTS key_content_sns_pk_compressed OID;
