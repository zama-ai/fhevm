--------------------------------------------------------------------------------------------------
--                         Delete events autoremoval triggers/functions                         --
--------------------------------------------------------------------------------------------------
DROP TRIGGER IF EXISTS trigger_public_decryption_requests ON public_decryption_responses;
DROP TRIGGER IF EXISTS trigger_user_decryption_requests ON user_decryption_responses;
DROP TRIGGER IF EXISTS trigger_delete_prep_keygen_requests ON prep_keygen_responses;
DROP TRIGGER IF EXISTS trigger_delete_keygen_requests ON keygen_responses;
DROP TRIGGER IF EXISTS trigger_delete_crsgen_requests ON crsgen_responses;

DROP FUNCTION IF EXISTS delete_from_public_decryption_requests;
DROP FUNCTION IF EXISTS delete_from_user_decryption_requests;
DROP FUNCTION IF EXISTS delete_from_prep_keygen_requests;
DROP FUNCTION IF EXISTS delete_from_keygen_requests;
DROP FUNCTION IF EXISTS delete_from_crsgen_requests;

--------------------------------------------------------------------------------------------------
--                Replace the `under_process` field by a `operation_status` enum                --
--------------------------------------------------------------------------------------------------

-- Create the `operation_status` enum and `status` columns
CREATE TYPE operation_status AS ENUM (
    'pending',
    'under_process',
    'completed',
    'failed'
);

ALTER TABLE public_decryption_requests ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE user_decryption_requests ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE prep_keygen_requests ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE keygen_requests ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE crsgen_requests ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE prss_init ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE key_reshare_same_set ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;

ALTER TABLE public_decryption_responses ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE user_decryption_responses ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE prep_keygen_responses ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE keygen_responses ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;
ALTER TABLE crsgen_responses ADD COLUMN status operation_status DEFAULT 'pending' NOT NULL;

-- All locked (under_process = TRUE) decryptions will be marked as 'pending', and should be
-- recovered via retries or cleaned via the garbage collection.
-- Any remaining locked crsgen/keygen operations in DB are failures (stuck operations), but there
-- should be none AFAIK.
UPDATE prep_keygen_requests SET status = 'failed' WHERE under_process = TRUE;
UPDATE keygen_requests SET status = 'failed' WHERE under_process = TRUE;
UPDATE crsgen_requests SET status = 'failed' WHERE under_process = TRUE;
UPDATE prss_init SET status = 'failed' WHERE under_process = TRUE;
UPDATE key_reshare_same_set SET status = 'failed' WHERE under_process = TRUE;
UPDATE prep_keygen_responses SET status = 'failed' WHERE under_process = TRUE;
UPDATE keygen_responses SET status = 'failed' WHERE under_process = TRUE;
UPDATE crsgen_responses SET status = 'failed' WHERE under_process = TRUE;

-- Drop `under_process` columns
ALTER TABLE public_decryption_requests DROP COLUMN under_process;
ALTER TABLE user_decryption_requests DROP COLUMN under_process;
ALTER TABLE prep_keygen_requests DROP COLUMN under_process;
ALTER TABLE keygen_requests DROP COLUMN under_process;
ALTER TABLE crsgen_requests DROP COLUMN under_process;
ALTER TABLE prss_init DROP COLUMN under_process;
ALTER TABLE key_reshare_same_set DROP COLUMN under_process;

ALTER TABLE public_decryption_responses DROP COLUMN under_process;
ALTER TABLE user_decryption_responses DROP COLUMN under_process;
ALTER TABLE prep_keygen_responses DROP COLUMN under_process;
ALTER TABLE keygen_responses DROP COLUMN under_process;
ALTER TABLE crsgen_responses DROP COLUMN under_process;

--------------------------------------------------------------------------------------------------
--                  Add the `updated_at` field for all events/responses tables                  --
--------------------------------------------------------------------------------------------------
ALTER TABLE public_decryption_requests ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE user_decryption_requests ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE prep_keygen_requests ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE keygen_requests ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE crsgen_requests ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE prss_init ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE key_reshare_same_set ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;

ALTER TABLE public_decryption_responses ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE user_decryption_responses ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE prep_keygen_responses ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE keygen_responses ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;
ALTER TABLE crsgen_responses ADD COLUMN updated_at TIMESTAMP DEFAULT NOW() NOT NULL;

--------------------------------------------------------------------------------------------------
--                                Create indexes for decryption                                 --
--------------------------------------------------------------------------------------------------
CREATE INDEX idx_public_decryption_requests_status ON public_decryption_requests (status);
CREATE INDEX idx_public_decryption_requests_status_updated_at ON public_decryption_requests (status, updated_at);
CREATE INDEX idx_user_decryption_requests_status ON user_decryption_requests (status);
CREATE INDEX idx_user_decryption_requests_status_updated_at ON user_decryption_requests (status, updated_at);
CREATE INDEX idx_public_decryption_responses_status ON public_decryption_responses (status);
CREATE INDEX idx_public_decryption_responses_status_updated_at ON public_decryption_responses (status, updated_at);
CREATE INDEX idx_user_decryption_responses_status ON user_decryption_responses (status);
CREATE INDEX idx_user_decryption_responses_status_updated_at ON user_decryption_responses (status, updated_at);


--------------------------------------------------------------------------------------------------
--                            Autofill `updated_at`/`status` fields                             --
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_public_decryption_request()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_public_decryption_requests_on_update
BEFORE UPDATE ON public_decryption_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_public_decryption_request();

CREATE OR REPLACE FUNCTION complete_public_decryption_request()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE public_decryption_requests SET status = 'completed' WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_public_decryption_request_on_response_insert
AFTER INSERT ON public_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION complete_public_decryption_request();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_user_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_user_decryption_requests_on_update
BEFORE UPDATE ON user_decryption_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_user_decryption_requests();

CREATE OR REPLACE FUNCTION complete_user_decryption_request()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE user_decryption_requests SET status = 'completed' WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_user_decryption_requests_on_response_insert
AFTER INSERT ON user_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION complete_user_decryption_request();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_prep_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_prep_keygen_requests_on_update
BEFORE UPDATE ON prep_keygen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_prep_keygen_requests();

CREATE OR REPLACE FUNCTION complete_prep_keygen_now()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE prep_keygen_requests SET status = 'completed' WHERE prep_keygen_id = NEW.prep_keygen_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_prep_keygen_requests_on_response_insert
AFTER INSERT ON prep_keygen_responses
FOR EACH ROW
EXECUTE FUNCTION complete_prep_keygen_now();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_keygen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_keygen_requests_on_update
BEFORE UPDATE ON keygen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_keygen_requests();

CREATE OR REPLACE FUNCTION complete_keygen_now()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE keygen_requests SET status = 'completed' WHERE key_id = NEW.key_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_keygen_requests_on_response_insert
AFTER INSERT ON keygen_responses
FOR EACH ROW
EXECUTE FUNCTION complete_keygen_now();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_crsgen_requests()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_crsgen_requests_on_update
BEFORE UPDATE ON crsgen_requests
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_crsgen_requests();

CREATE OR REPLACE FUNCTION complete_crsgen_now()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE crsgen_requests SET status = 'completed' WHERE crs_id = NEW.crs_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER complete_crsgen_requests_on_response_insert
AFTER INSERT ON crsgen_responses
FOR EACH ROW
EXECUTE FUNCTION complete_crsgen_now();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_prss_init()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_prss_init_on_update
BEFORE UPDATE ON prss_init
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_prss_init();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_key_reshare_same_set()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_key_reshare_same_set_on_update
BEFORE UPDATE ON key_reshare_same_set
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_key_reshare_same_set();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_public_decryption_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_public_decryption_responses_on_update
BEFORE UPDATE ON public_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_public_decryption_responses();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_user_decryption_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_user_decryption_responses_on_update
BEFORE UPDATE ON user_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_user_decryption_responses();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_prep_keygen_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_prep_keygen_responses_on_update
BEFORE UPDATE ON prep_keygen_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_prep_keygen_responses();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_keygen_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_keygen_responses_on_update
BEFORE UPDATE ON keygen_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_keygen_responses();
--------------------------------------------------------------------------------------------------
CREATE OR REPLACE FUNCTION refresh_updated_at_crsgen_responses()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER refresh_updated_at_crsgen_responses_on_update
BEFORE UPDATE ON crsgen_responses
FOR EACH ROW
EXECUTE FUNCTION refresh_updated_at_crsgen_responses();
