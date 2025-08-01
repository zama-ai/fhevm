--------------------------------------------------------
--             Decryption contract section            --
--------------------------------------------------------

-- Create SnsCiphertextMaterial type representation
DO $$ BEGIN
    CREATE TYPE sns_ciphertext_material AS (
        ct_handle BYTEA,
        key_id BYTEA,
        sns_ciphertext_digest BYTEA,
        coprocessor_tx_sender_addresses BYTEA[]
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;

-- Create Decryption contract events tables
CREATE TABLE IF NOT EXISTS public_decryption_requests (
    decryption_id BYTEA NOT NULL,
    sns_ct_materials sns_ciphertext_material[] NOT NULL,
    extra_data BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);

CREATE TABLE IF NOT EXISTS user_decryption_requests (
    decryption_id BYTEA NOT NULL,
    sns_ct_materials sns_ciphertext_material[] NOT NULL,
    user_address BYTEA NOT NULL,
    public_key BYTEA NOT NULL,
    extra_data BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);


--------------------------------------------------------
--           KmsManagement contract section           --
--------------------------------------------------------

-- Create KmsManagement contract events tables
CREATE TABLE IF NOT EXISTS preprocess_keygen_requests (
    pre_keygen_request_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_keygen_request_id)
);

CREATE TABLE IF NOT EXISTS preprocess_kskgen_requests (
    pre_kskgen_request_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_kskgen_request_id)
);

CREATE TABLE IF NOT EXISTS keygen_requests (
    pre_key_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_key_id)
);

CREATE TABLE IF NOT EXISTS kskgen_requests (
    pre_ksk_id BYTEA NOT NULL,
    source_key_id BYTEA NOT NULL,
    dest_key_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (pre_ksk_id)
);

CREATE TABLE IF NOT EXISTS crsgen_requests (
    crsgen_request_id BYTEA NOT NULL,
    fhe_params_digest BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (crsgen_request_id)
);
