CREATE TABLE IF NOT EXISTS key_reshare_same_set (
    prep_keygen_id BYTEA NOT NULL,
    key_id BYTEA NOT NULL,
    key_reshare_id BYTEA NOT NULL,
    params_type params_type NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    otlp_context BYTEA NOT NULL,
    PRIMARY KEY (key_id)
);

CREATE OR REPLACE FUNCTION notify_key_reshare_same_set()
    RETURNS trigger AS $$
BEGIN
    NOTIFY key_reshare_same_set_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_key_reshare_same_set_insertions
    AFTER INSERT
    ON key_reshare_same_set
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_key_reshare_same_set();
