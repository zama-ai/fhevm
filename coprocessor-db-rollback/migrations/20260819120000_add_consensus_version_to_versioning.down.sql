-- Down for 20260819120000_add_consensus_version_to_versioning.
-- Drops the column. The recorded consensus version is lost.

ALTER TABLE versioning
    DROP COLUMN IF EXISTS consensus_version;
