--------------------------------------------------------
--         Dropping outdated tables/triggers          --
--------------------------------------------------------
DROP TRIGGER IF EXISTS trigger_from_preprocess_keygen_requests_insertions ON preprocess_keygen_requests;
DROP TRIGGER IF EXISTS trigger_from_preprocess_kskgen_requests_insertions ON preprocess_kskgen_requests;
DROP FUNCTION IF EXISTS notify_preprocess_keygen_request;
DROP FUNCTION IF EXISTS notify_preprocess_kskgen_request;

DROP TABLE IF EXISTS preprocess_keygen_requests;
DROP TABLE IF EXISTS preprocess_kskgen_requests;

--------------------------------------------------------
--    Updating/creating PrepKeygen tables/triggers    --
--------------------------------------------------------
-- Create new ParamsType enum
CREATE TYPE params_type AS ENUM ('Default', 'Test');

CREATE TABLE IF NOT EXISTS prep_keygen_requests (
    prep_keygen_id BYTEA NOT NULL,
    epoch_id BYTEA NOT NULL,
    params_type params_type NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (prep_keygen_id)
);

CREATE TABLE IF NOT EXISTS prep_keygen_responses (
    prep_keygen_id BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (prep_keygen_id)
);

-- Create all triggers on insert of prep keygen requests/responses
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


CREATE OR REPLACE FUNCTION notify_prep_keygen_response()
    RETURNS trigger AS $$
BEGIN
    NOTIFY prep_keygen_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_prep_keygen_responses_insertions
    AFTER INSERT
    ON prep_keygen_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_prep_keygen_response();

-- Autoremove prep keygen requests associated to responses when they are inserted in the DB
CREATE OR REPLACE FUNCTION delete_from_prep_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM prep_keygen_requests WHERE prep_keygen_id = NEW.prep_keygen_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_prep_keygen_requests
AFTER INSERT ON prep_keygen_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_prep_keygen_requests();

--------------------------------------------------------
--      Updating/creating Keygen tables/triggers      --
--------------------------------------------------------
ALTER TABLE keygen_requests RENAME fhe_params_digest TO key_id;
ALTER TABLE keygen_requests DROP CONSTRAINT keygen_requests_pkey;
ALTER TABLE keygen_requests RENAME pre_key_id TO prep_keygen_id;
ALTER TABLE keygen_requests ADD PRIMARY KEY (key_id);

CREATE TABLE IF NOT EXISTS keygen_responses (
    key_id BYTEA NOT NULL,
    server_key_digest BYTEA NOT NULL,
    public_key_digest BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (key_id)
);

-- Create all triggers on insert of keygen requests/responses
CREATE OR REPLACE FUNCTION notify_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_keygen_requests_insertions
    AFTER INSERT
    ON keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_keygen_request();


CREATE OR REPLACE FUNCTION notify_keygen_response()
    RETURNS trigger AS $$
BEGIN
    NOTIFY keygen_response_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_keygen_responses_insertions
    AFTER INSERT
    ON keygen_responses
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_keygen_response();

-- Autoremove prep keygen requests associated to responses when they are inserted in the DB
CREATE OR REPLACE FUNCTION delete_from_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM keygen_requests WHERE key_id = NEW.key_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_delete_keygen_requests
AFTER INSERT ON keygen_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_keygen_requests();

--------------------------------------------------------
--      Updating/creating Crsgen tables/triggers      --
--------------------------------------------------------
ALTER TABLE crsgen_requests RENAME fhe_params_digest TO max_bit_length;
ALTER TABLE crsgen_requests DROP CONSTRAINT crsgen_requests_pkey;
ALTER TABLE crsgen_requests RENAME crsgen_request_id TO crs_id;
ALTER TABLE crsgen_requests ADD PRIMARY KEY (crs_id);
ALTER TABLE crsgen_requests DROP COLUMN under_process;
ALTER TABLE crsgen_requests DROP COLUMN created_at;
ALTER TABLE crsgen_requests ADD params_type params_type NOT NULL;
ALTER TABLE crsgen_requests ADD under_process BOOLEAN NOT NULL DEFAULT FALSE;
ALTER TABLE crsgen_requests ADD created_at TIMESTAMP NOT NULL DEFAULT NOW();
