DO $$ BEGIN
    CREATE TYPE event_type AS ENUM (
        'PublicDecryptionRequest',
        'UserDecryptionRequest',
        'PrepKeygenRequest',
        'KeygenRequest',
        'CrsgenRequest',
        'PrssInit',
        'KeyReshareSameSet'
    );
EXCEPTION
    WHEN duplicate_object THEN null;
END $$;


CREATE TABLE IF NOT EXISTS last_block_polled (
    event_type event_type NOT NULL,
    block_number BIGINT,
    update_at TIMESTAMP NOT NULL DEFAULT NOW(),
    PRIMARY KEY (event_type)
);

INSERT INTO last_block_polled(event_type, block_number) VALUES
    ('PublicDecryptionRequest', NULL),
    ('UserDecryptionRequest', NULL),
    ('PrepKeygenRequest', NULL),
    ('KeygenRequest', NULL),
    ('CrsgenRequest', NULL),
    ('PrssInit', NULL),
    ('KeyReshareSameSet', NULL)
ON CONFLICT DO NOTHING;
