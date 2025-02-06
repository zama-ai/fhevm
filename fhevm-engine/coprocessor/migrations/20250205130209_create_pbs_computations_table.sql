CREATE TABLE IF NOT EXISTS pbs_computations (
    tenant_id INT NOT NULL,
    handle BYTEA NOT NULL,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMP,
    is_completed BOOLEAN NOT NULL DEFAULT FALSE, 
    PRIMARY KEY (tenant_id, handle)
);


CREATE INDEX IF NOT EXISTS pbs_computations_handle_hash_idx ON pbs_computations USING HASH (handle);
CREATE INDEX IF NOT EXISTS ciphertexts_handle_hash_idx ON ciphertexts USING HASH (handle);
CREATE INDEX IF NOT EXISTS ciphertexts_handle_ct_idx ON ciphertexts (handle) INCLUDE (ciphertext); -- for fast retrieval of ciphertexts in sns-worker