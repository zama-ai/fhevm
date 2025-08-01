CREATE TABLE IF NOT EXISTS public_decryption_responses (
    decryption_id BYTEA NOT NULL,
    decrypted_result BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    extra_data BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);

CREATE TABLE IF NOT EXISTS user_decryption_responses (
    decryption_id BYTEA NOT NULL,
    user_decrypted_shares BYTEA NOT NULL,
    signature BYTEA NOT NULL,
    extra_data BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (decryption_id)
);

-- Autoremove public decryption requests associated to responses when they are inserted in the DB
CREATE OR REPLACE FUNCTION delete_from_public_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM public_decryption_requests WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_public_decryption_requests
AFTER INSERT ON public_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_public_decryption_requests();

-- Autoremove user decryption requests associated to responses when they are inserted in the DB
CREATE OR REPLACE FUNCTION delete_from_user_decryption_requests()
RETURNS TRIGGER AS $$
BEGIN
    DELETE FROM user_decryption_requests WHERE decryption_id = NEW.decryption_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER trigger_user_decryption_requests
AFTER INSERT ON user_decryption_responses
FOR EACH ROW
EXECUTE FUNCTION delete_from_user_decryption_requests();
