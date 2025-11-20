CREATE TABLE IF NOT EXISTS host_listener_poller_state (
    chain_id BIGINT PRIMARY KEY,
    last_caught_up_block BIGINT NOT NULL,
    updated_at TIMESTAMP NOT NULL DEFAULT NOW()
);
