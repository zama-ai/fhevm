-- Dropping outdated preprocess requests tables/triggers
DROP TRIGGER IF EXISTS trigger_from_preprocess_keygen_requests_insertions ON preprocess_keygen_requests;
DROP TRIGGER IF EXISTS trigger_from_preprocess_kskgen_requests_insertions ON preprocess_kskgen_requests;
DROP FUNCTION IF EXISTS notify_preprocess_keygen_request;
DROP FUNCTION IF EXISTS notify_preprocess_kskgen_request;

DROP TABLE IF EXISTS preprocess_keygen_requests;
DROP TABLE IF EXISTS preprocess_kskgen_requests;

-- Create new ParamsType enum
CREATE TYPE params_type AS ENUM ('Default', 'Test');

-- Update keygen table
ALTER TABLE keygen_requests RENAME fhe_params_digest TO key_id;
ALTER TABLE keygen_requests DROP CONSTRAINT keygen_requests_pkey;
ALTER TABLE keygen_requests RENAME pre_key_id TO prep_keygen_id;
ALTER TABLE keygen_requests ADD PRIMARY KEY (prep_keygen_id);

-- Update crsgen table
ALTER TABLE crsgen_requests RENAME fhe_params_digest TO max_bit_length;
ALTER TABLE crsgen_requests DROP CONSTRAINT crsgen_requests_pkey;
ALTER TABLE crsgen_requests RENAME crsgen_request_id TO crs_id;
ALTER TABLE crsgen_requests ADD PRIMARY KEY (crs_id);
ALTER TABLE crsgen_requests DROP COLUMN under_process;
ALTER TABLE crsgen_requests DROP COLUMN created_at;
ALTER TABLE crsgen_requests ADD params_type params_type NOT NULL;
ALTER TABLE crsgen_requests ADD under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE crsgen_requests ADD created_at TIMESTAMP NOT NULL DEFAULT NOW();

-- Creating the new tables/triggers with corrected names and new columns
CREATE TABLE IF NOT EXISTS prep_keygen_requests (
    prep_keygen_id BYTEA NOT NULL,
    epoch_id BYTEA NOT NULL,
    params_type params_type NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (prep_keygen_id)
);

CREATE OR REPLACE FUNCTION notify_prep_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY prep_keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_prep_keygen_requests_insertions
    AFTER INSERT
    ON prep_keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_prep_keygen_request();
