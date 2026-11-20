-- Down for 20260526120000_add_protocol_config_events.
--
-- Drops the ProtocolConfig event tables with their triggers and functions,
-- removes `epoch_id` from `kms_context` (primary key back to `id`), and drops
-- the per-chain polling cursor.
--
-- Polling cursor: the up seeded `last_block_polled_by_chain` from the legacy
-- `last_block_polled` table and then left the legacy table untouched, so its
-- positions are those of the day of the upgrade. The previous version reads
-- the legacy table. To keep it from polling again everything since the
-- upgrade, each chain position is copied back to the event types of that
-- chain before the per-chain table is dropped. A chain with no recorded
-- position leaves the legacy rows as they are.
--
-- Data impact: the stored `NewKmsContext` / `NewKmsEpoch` events are lost.
-- `kms_context` keeps one row per context, the most recently updated one, and
-- the context is valid only if every row it had was: one invalid row is how
-- the up's schema records a destroyed context. The epochs are lost.

DROP TABLE new_kms_context;
DROP TABLE new_kms_epoch;

DROP FUNCTION refresh_updated_at_new_kms_context();
DROP FUNCTION refresh_updated_at_new_kms_epoch();
DROP FUNCTION notify_new_kms_context();
DROP FUNCTION notify_new_kms_epoch();

UPDATE kms_context
SET is_valid = FALSE
WHERE is_valid
  AND id IN (SELECT id FROM kms_context WHERE NOT is_valid);

DELETE FROM kms_context older
USING kms_context newer
WHERE older.id = newer.id
  AND (older.updated_at, older.epoch_id) < (newer.updated_at, newer.epoch_id);

ALTER TABLE kms_context
    DROP CONSTRAINT kms_context_pkey,
    ADD PRIMARY KEY (id);
ALTER TABLE kms_context DROP COLUMN epoch_id;

UPDATE last_block_polled legacy
SET block_number = by_chain.block_number
FROM last_block_polled_by_chain by_chain
WHERE by_chain.block_number IS NOT NULL
  AND (
        (by_chain.chain_name = 'ethereum'
            AND legacy.event_type IN ('PrepKeygenRequest', 'KeygenRequest', 'CrsgenRequest'))
     OR (by_chain.chain_name = 'gateway'
            AND legacy.event_type IN ('PublicDecryptionRequest', 'UserDecryptionRequest'))
  );

DROP TABLE last_block_polled_by_chain;
