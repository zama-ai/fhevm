-- Consumer intent is separate from listener-core publication progress. A
-- completed core request can still have blocks waiting in the delivery queue.
CREATE TABLE host_consumer_catchups (
    id UUID PRIMARY KEY,
    chain_id BIGINT NOT NULL CHECK (chain_id >= 0),
    consumer_id TEXT NOT NULL,
    finalized BOOLEAN NOT NULL DEFAULT TRUE,
    requested_from BIGINT NOT NULL,
    requested_to BIGINT NOT NULL,
    block_start BIGINT NOT NULL CHECK (block_start >= 0),
    block_end BIGINT NOT NULL CHECK (block_end >= block_start),
    status TEXT NOT NULL DEFAULT 'ONGOING'
        CHECK (status IN ('ONGOING', 'DONE', 'SUPPRESSED')),
    -- Persist before publishing either command; clear only after broker ACK.
    request_pending BOOLEAN NOT NULL DEFAULT TRUE,
    cancel_pending BOOLEAN NOT NULL DEFAULT FALSE,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    -- Ownership fences request coordination, not shared broker delivery.
    owner_id UUID,
    lease_until TIMESTAMPTZ,
    CONSTRAINT host_consumer_catchups_lease_pair
        CHECK ((owner_id IS NULL) = (lease_until IS NULL))
);

-- Original signed flags and finality identify the request. Relative bounds are resolved
-- once, and are never recomputed when an existing request is resumed.
CREATE UNIQUE INDEX host_consumer_catchups_ongoing_request
    ON host_consumer_catchups (chain_id, consumer_id, requested_from, requested_to, finalized)
    WHERE status = 'ONGOING';

CREATE INDEX host_consumer_catchups_pending_commands
    ON host_consumer_catchups (chain_id, consumer_id)
    WHERE request_pending OR cancel_pending;

CREATE INDEX host_consumer_catchups_claimable
    ON host_consumer_catchups (chain_id, consumer_id, lease_until)
    WHERE status = 'ONGOING' OR cancel_pending;
