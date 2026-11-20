-- Keep rejected manual intent visible without stopping live ingestion.
ALTER TABLE host_consumer_catchups
    ADD COLUMN failure_reason TEXT,
    DROP CONSTRAINT host_consumer_catchups_status_check,
    DROP CONSTRAINT host_consumer_catchups_check,
    ADD CONSTRAINT host_consumer_catchups_status_check
        CHECK (status IN ('ONGOING', 'DONE', 'SUPPRESSED', 'SKIPPED', 'FAILED')),
    ADD CONSTRAINT host_consumer_catchups_block_end_check
        CHECK (block_end >= 0 AND (status = 'FAILED' OR block_end >= block_start)),
    ADD CONSTRAINT host_consumer_catchups_failure_check
        CHECK ((status = 'FAILED') = (failure_reason IS NOT NULL)),
    ADD CONSTRAINT host_consumer_catchups_failed_inactive
        CHECK (status <> 'FAILED' OR
               (NOT request_pending AND NOT cancel_pending AND owner_id IS NULL AND lease_until IS NULL));
