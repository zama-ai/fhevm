CREATE TABLE dependence_chain (
    dependence_chain_id  bytea PRIMARY KEY,

    -- Scheduling / Coordination
    status              TEXT NOT NULL CHECK (status IN (
                                'updated', 'processing', 'processed'
                            )),
    error_message              TEXT,  -- optional error message if processing failed

    -- Worker Ownership (updated by tfhe-workers)
    worker_id           UUID,              
    lock_acquired_at    TIMESTAMPTZ,
    lock_expires_at     TIMESTAMPTZ,

    -- Execution (updated by host-listener(s))
    last_updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_pending_dependence_chain
    ON dependence_chain USING BTREE (last_updated_at)
    WHERE status = 'updated' AND worker_id IS NULL;

CREATE INDEX idx_dependence_chain_worker_id
    ON dependence_chain (worker_id);

CREATE INDEX idx_dependence_chain_worker_id_and_dependence_chain_id
    ON dependence_chain (dependence_chain_id, worker_id);
