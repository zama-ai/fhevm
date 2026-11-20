-- Add a column on `keys` to store the raw kms-core
-- CompressedXofKeySet blob alongside the existing decompressed
-- sns_pk / sks_key columns. New rows produced from a
-- CompressedXofKeySet populate both: the legacy decompressed pair
-- stays as the read path for tfhe-worker (CPU) and the new compressed
-- column is the read path for sns-worker and tfhe-worker (GPU), both
-- of which decompress the whole keyset in one pass to keep the XOF
-- state consistent across subkeys.

ALTER TABLE keys
    ADD COLUMN IF NOT EXISTS compressed_xof_keyset BYTEA;

ALTER TABLE kms_key_activation_events
    ADD COLUMN IF NOT EXISTS key_content_compressed_xof_keyset BYTEA;
