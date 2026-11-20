-- Down for 20261015120000_host_consumer_catchups.
-- Drops the table and its indexes. Catch-up requests and their state are lost.

DROP INDEX IF EXISTS host_consumer_catchups_claimable;
DROP INDEX IF EXISTS host_consumer_catchups_pending_commands;
DROP INDEX IF EXISTS host_consumer_catchups_ongoing_request;
DROP TABLE IF EXISTS host_consumer_catchups;
