-- Active consensus version. Existing databases start at version 1, the value
-- the live network runs. New databases set this value during setup.
ALTER TABLE versioning
    ADD COLUMN IF NOT EXISTS consensus_version BIGINT NOT NULL DEFAULT 1;
