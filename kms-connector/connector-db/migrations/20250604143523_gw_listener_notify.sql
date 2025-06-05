--------------------------------------------------------
--             Decryption contract section            --
--------------------------------------------------------

-- Create functions to notify listeners when decryption requests are received
CREATE OR REPLACE FUNCTION notify_public_decryption_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY public_decryption_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_user_decryption_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY user_decryption_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to fire once per statement on decryption requests inserts
CREATE TRIGGER trigger_from_public_decryption_requests_insertions
    AFTER INSERT
    ON public_decryption_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_public_decryption_request();

CREATE TRIGGER trigger_from_user_decryption_requests_insertions
    AFTER INSERT
    ON user_decryption_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_user_decryption_request();

--------------------------------------------------------
--           KmsManagement contract section           --
--------------------------------------------------------

-- Create functions to notify listeners when kms management requests are received
CREATE OR REPLACE FUNCTION notify_preprocess_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY preprocess_keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_preprocess_kskgen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY preprocess_kskgen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_keygen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY keygen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_kskgen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY kskgen_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION notify_crsgen_request()
    RETURNS trigger AS $$
BEGIN
    NOTIFY crs_request_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

-- Create triggers to fire once per statement on kms management requests inserts
CREATE TRIGGER trigger_from_preprocess_keygen_requests_insertions
    AFTER INSERT
    ON preprocess_keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_preprocess_keygen_request();

CREATE TRIGGER trigger_from_preprocess_kskgen_requests_insertions
    AFTER INSERT
    ON preprocess_kskgen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_preprocess_kskgen_request();

CREATE TRIGGER trigger_from_keygen_requests_insertions
    AFTER INSERT
    ON keygen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_keygen_request();

CREATE TRIGGER trigger_from_kskgen_requests_insertions
    AFTER INSERT
    ON kskgen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_kskgen_request();

CREATE TRIGGER trigger_from_crsgen_requests_insertions
    AFTER INSERT
    ON crsgen_requests
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_crsgen_request();

