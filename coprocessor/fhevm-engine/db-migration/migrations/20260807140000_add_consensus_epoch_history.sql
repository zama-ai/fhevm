-- Per-stack active consensus_epoch. Blue resolves this singleton from public;
-- Green resolves its independent copy through `"gcs-<version>",public`.
-- The GCS copy is merged into public at cutover.
--
-- Genesis is `legacy`. Later identifiers are `{version}/block_{n}` from the
-- finalized CoprocessorUpgradeProposed log, not a shared counter and not
-- gwStartBlock. The log is only ingested on
-- CANONICAL_PROTOCOL_CONFIG_CHAIN_ID, so the chain id is omitted.
CREATE TABLE IF NOT EXISTS blue_green_consensus_epoch
(
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    consensus_epoch TEXT NOT NULL CHECK (consensus_epoch <> '' AND LENGTH(consensus_epoch) <= 256),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO blue_green_consensus_epoch (singleton, consensus_epoch)
VALUES (TRUE, 'legacy')
ON CONFLICT (singleton) DO NOTHING;

-- Durable audit and proposal-to-consensus_epoch mapping. The primary key is the
-- event-derived identifier. Failed attempts stay recorded so a later proposal
-- cannot reuse that log's consensus_epoch.
CREATE TABLE IF NOT EXISTS consensus_epoch_history
(
    consensus_epoch TEXT PRIMARY KEY CHECK (consensus_epoch <> '' AND LENGTH(consensus_epoch) <= 256),
    proposal_id BYTEA NULL CHECK (proposal_id IS NULL OR OCTET_LENGTH(proposal_id) = 32),
    proposal_block BIGINT NULL CHECK (proposal_block IS NULL OR proposal_block >= 0),
    stack_version TEXT NULL,
    outcome TEXT NOT NULL CHECK (outcome IN ('initial', 'pending', 'succeeded', 'failed')),
    allocated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    completed_at TIMESTAMPTZ NULL,

    CHECK ((proposal_id IS NULL) = (proposal_block IS NULL)),
    CHECK (
        (outcome = 'pending' AND completed_at IS NULL)
        OR (outcome <> 'pending' AND completed_at IS NOT NULL)
    )
);

-- Existing deployments began in the genesis epoch `legacy`. It is a completed
-- baseline, not an upgrade attempt, so it has no proposal identity.
INSERT INTO consensus_epoch_history (consensus_epoch, outcome, completed_at)
VALUES ('legacy', 'initial', NOW())
ON CONFLICT (consensus_epoch) DO NOTHING;

-- A replay of one accepted proposal must return the same identifier. A later
-- proposal occupies a different log coordinate and therefore a new string.
CREATE UNIQUE INDEX IF NOT EXISTS uq_consensus_epoch_history_proposal
    ON consensus_epoch_history (proposal_id, proposal_block)
    WHERE proposal_id IS NOT NULL;
