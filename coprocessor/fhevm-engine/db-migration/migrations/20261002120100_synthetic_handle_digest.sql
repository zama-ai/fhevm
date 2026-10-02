-- The probe's manifest descriptor material, copied by cutover just before it
-- deletes the probe's work, so its block seals the same way before or after
-- cutover. Written only by cutover and read only to build manifests: the
-- publishing paths (transaction-sender, S3 upload) never see the probe. A copy
-- without ct128 means it was not computed by cutover and never will be.
CREATE TABLE IF NOT EXISTS synthetic_handle_digest
(
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    handle BYTEA NOT NULL CHECK (OCTET_LENGTH(handle) = 32),
    key_id_gw BYTEA NULL,
    ciphertext BYTEA NULL,
    ciphertext128 BYTEA NULL,
    ciphertext128_format SMALLINT NULL,
    is_error BOOLEAN NOT NULL DEFAULT FALSE,
    error_message TEXT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (host_chain_id, handle)
);
