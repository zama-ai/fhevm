-- Down for 20260708000000_add_context_epoch_destruction.
--
-- Drops the destruction event tables with their triggers and functions, merges
-- the context-level and epoch-level validation caches back into the single
-- `kms_context` table keyed by (id, epoch_id), and drops `kms_epoch`.
--
-- In the pair schema a row with `is_valid = FALSE` means the whole context was
-- destroyed: the previous version rejects every request of a context that has
-- one such row. A destroyed epoch of a live context must therefore not become
-- an invalid row. The cache is rebuilt with one row per valid epoch whose
-- context is known, carrying the validity of the context. Destroyed epochs,
-- contexts with no valid cached epoch and epochs with no known context are
-- left out; the previous version asks the chain for what it does not find.
--
-- Data impact: the stored `KmsContextDestroyed` / `KmsEpochDestroyed` events
-- are lost. It is a cache of on-chain state, so nothing authoritative is lost.

DROP TABLE kms_context_destroyed;
DROP TABLE kms_epoch_destroyed;

DROP FUNCTION refresh_updated_at_kms_context_destroyed();
DROP FUNCTION refresh_updated_at_kms_epoch_destroyed();
DROP FUNCTION notify_kms_context_destroyed();
DROP FUNCTION notify_kms_epoch_destroyed();

CREATE TEMPORARY TABLE kms_context_pairs ON COMMIT DROP AS
SELECT c.id,
       e.id AS epoch_id,
       c.is_valid,
       c.created_at,
       greatest(c.updated_at, e.updated_at) AS updated_at
FROM kms_context c
JOIN kms_epoch e ON e.context_id = c.id
WHERE e.is_valid;

TRUNCATE kms_context;

ALTER TABLE kms_context ALTER COLUMN is_valid SET DEFAULT TRUE;
ALTER TABLE kms_context ADD COLUMN epoch_id BYTEA NOT NULL;
ALTER TABLE kms_context
    DROP CONSTRAINT kms_context_pkey,
    ADD PRIMARY KEY (id, epoch_id);

INSERT INTO kms_context (id, epoch_id, is_valid, created_at, updated_at)
SELECT id, epoch_id, is_valid, created_at, updated_at
FROM kms_context_pairs;

DROP TABLE kms_epoch;
