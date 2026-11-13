-- One cursor for the relayer: every gateway chain event up to this block has been handled.
--
-- It replaces the per-listener rows of `gateway_block_number_store`, whose key was a
-- listener's position in the config list, so reordering that list handed one listener
-- another's position. Listeners are redundant readers of the same chain and deduplicate
-- against each other, so one row says all there is to say.
--
-- `gateway_block_number_store` is left in place, untouched: a rolled-back relayer still
-- reads and writes it.
CREATE TABLE gateway_chain_cursor (
    -- Single row, enforced by the key: there is one chain to keep a position on.
    id BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (id),
    last_block_number BIGINT NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_gateway_chain_cursor_updated_at
BEFORE UPDATE ON gateway_chain_cursor
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

-- Carry over the least-advanced position the per-listener rows reached, so the first start
-- after this migration resumes there instead of at the chain head. A pool that never ran
-- leaves the table empty, and the first poll seeds it.
--
-- The minimum, deliberately, not the maximum. A per-listener cursor advanced on dispatch
-- rather than on completion, so a block the furthest-ahead listener had passed can still hold
-- an event that was observed and never handled - its row left in `receipt_received` until the
-- timeout cron fails it out, which the client sees. Seeding the minimum re-reads the range
-- between the two positions exactly once, and handling is idempotent by design, so the replay
-- costs a bounded amount of duplicate work and loses nothing.
INSERT INTO gateway_chain_cursor (id, last_block_number)
SELECT TRUE, MIN(last_block_number)
FROM gateway_block_number_store
HAVING MIN(last_block_number) IS NOT NULL;
