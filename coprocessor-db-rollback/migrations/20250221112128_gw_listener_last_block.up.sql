CREATE TABLE IF NOT EXISTS gw_listener_last_block (
    -- Used to make sure we only have one record in the table, the one with dummy_id = true.
    dummy_id BOOLEAN PRIMARY KEY DEFAULT true,

    -- NULL means subscription will starte from the "latest" block.
    last_block_num BIGINT CHECK (last_block_num >= 0)
)
