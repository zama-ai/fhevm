-- Down for 20260824120000_create_gateway_chain_cursor.
--
-- Drops the single-row cursor table together with its trigger. The position it
-- holds is lost: record it before rolling back.
--
-- The up left `gateway_block_number_store` in place for a rolled-back relayer
-- to use, but the relayer versions that have the cursor no longer write to it.
-- Its rows are therefore frozen at the moment of the upgrade, or absent if the
-- database was created after it. A previous relayer version started on that
-- table alone would either re-read every block since the upgrade or, with no
-- row, start at the chain head and miss what happened while it was down.
--
-- Nothing is written back here, because the store also needs a block hash this
-- script cannot know. The operator carries the position over instead, through
-- the previous version's `listener_pool.last_block_number` setting, which takes
-- precedence over the table. See the rollback procedure in the README.

DROP TABLE gateway_chain_cursor;
