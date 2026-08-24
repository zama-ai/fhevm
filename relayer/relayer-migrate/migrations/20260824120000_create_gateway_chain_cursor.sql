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

-- Carry over the furthest position the per-listener rows reached, so the first start after
-- this migration resumes there instead of at the chain head. A pool that never ran leaves
-- the table empty, and the first poll seeds it.
INSERT INTO gateway_chain_cursor (id, last_block_number)
SELECT TRUE, MAX(last_block_number)
FROM gateway_block_number_store
HAVING MAX(last_block_number) IS NOT NULL;
