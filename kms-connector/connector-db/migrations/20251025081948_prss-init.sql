CREATE TABLE IF NOT EXISTS prss_init (
    id BYTEA NOT NULL,
    under_process BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (id)
);

CREATE OR REPLACE FUNCTION notify_prss_init()
    RETURNS trigger AS $$
BEGIN
    NOTIFY prss_init_available;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE TRIGGER trigger_from_prss_init_insertions
    AFTER INSERT
    ON prss_init
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_prss_init();
