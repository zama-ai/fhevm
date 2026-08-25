-- Active consensus version. Existing databases start at version 0. New
-- databases set this value during setup.
ALTER TABLE versioning
    ADD COLUMN IF NOT EXISTS consensus_version BIGINT NOT NULL DEFAULT 0;
