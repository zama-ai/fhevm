-- The ct64 of each output, for Unmask. By default, no GC deletes it: Unmask can come at any time,
-- and a compressed ct64 is small (about 2 kB for 64 bits). `PAD_CT64_MAX_AGE` turns the GC on.
-- The row has a copy of the attestation, because the GC deletes the `pads` row earlier.
CREATE TABLE IF NOT EXISTS pad_ct64 (
    prf_instance INTEGER NOT NULL CHECK (prf_instance BETWEEN 0 AND 65535),
    label BYTEA NOT NULL CHECK (OCTET_LENGTH(label) = 32),
    pad_length INTEGER NOT NULL CHECK (pad_length BETWEEN 1 AND 2048),
    ciphertext64 BYTEA NOT NULL,
    ciphertext_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(ciphertext_digest) = 32),
    ciphertext128_digest BYTEA NOT NULL CHECK (OCTET_LENGTH(ciphertext128_digest) = 32),
    signer BYTEA NOT NULL CHECK (OCTET_LENGTH(signer) = 20),
    signature BYTEA NOT NULL CHECK (OCTET_LENGTH(signature) = 65),
    attestation_version SMALLINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (prf_instance, label, pad_length)
);

-- For the GC of the ct64.
CREATE INDEX IF NOT EXISTS idx_pad_ct64_by_age ON pad_ct64 (created_at);
