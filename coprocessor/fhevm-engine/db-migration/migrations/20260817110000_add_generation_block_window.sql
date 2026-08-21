-- Persist every host-chain ownership window for a manifest generation.
-- `allocated_at` remains audit metadata and must not be used as a discovery
-- boundary. A proposal has one global generation but may carry a distinct
-- block window for each host chain.
CREATE TABLE generation_block_window
(
    generation BIGINT NOT NULL
        REFERENCES generation_history(generation) ON DELETE CASCADE,
    host_chain_id BIGINT NOT NULL CHECK (host_chain_id >= 0),
    start_block BIGINT NOT NULL CHECK (start_block >= 0),
    consensus_deadline_block BIGINT NOT NULL
        CHECK (consensus_deadline_block >= start_block),

    PRIMARY KEY (generation, host_chain_id)
);
