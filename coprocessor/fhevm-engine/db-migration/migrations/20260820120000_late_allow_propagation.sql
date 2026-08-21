-- A persistent allow observed in a block AFTER the block that computed the
-- handle (a "late allow"). `computations.is_allowed` is stamped from
-- same-block ACL events only, so a late allow must be propagated to the
-- earlier rows — but only once its own block is FINAL: `allowed_handles`
-- rows from orphaned siblings are deliberately retained (pre-wave1
-- semantics) and must never flip canonical rows. Rows here are keyed by the
-- observing block hash so finalization applies-and-deletes them and
-- orphaning retracts them.
CREATE TABLE late_allow_propagation (
    host_chain_id BIGINT NOT NULL,
    block_hash BYTEA NOT NULL,
    block_number BIGINT NOT NULL,
    handle BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (host_chain_id, block_hash, handle)
);
