-- One global, never-reused Blue/Green generation. It is allocated when the
-- canonical host listener accepts a new upgrade proposal, not at cutover: a
-- failed dry-run must consume its generation too.
ALTER TABLE versioning
    ADD COLUMN IF NOT EXISTS generation BIGINT NOT NULL DEFAULT 0
        CHECK (generation >= 0);

-- Per-stack active generation. Blue resolves this singleton from public;
-- Green resolves its independent copy through `"gcs-<version>",public`.
-- The GCS copy is merged into public at cutover.
CREATE TABLE IF NOT EXISTS blue_green_generation
(
    singleton BOOLEAN PRIMARY KEY DEFAULT TRUE CHECK (singleton),
    generation BIGINT NOT NULL CHECK (generation >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

INSERT INTO blue_green_generation (singleton, generation)
VALUES (TRUE, 0)
ON CONFLICT (singleton) DO NOTHING;

-- Durable audit and proposal-to-generation mapping. The singleton counter is
-- the allocation source; this table retains both completed and failed attempts
-- so a later proposal can never reuse a failed generation.
CREATE TABLE IF NOT EXISTS generation_history
(
    generation BIGINT PRIMARY KEY CHECK (generation >= 0),
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

-- Existing deployments began in generation zero. It is a completed baseline,
-- not an upgrade attempt, so it intentionally has no proposal identity.
INSERT INTO generation_history (generation, outcome, completed_at)
VALUES (0, 'initial', NOW())
ON CONFLICT (generation) DO NOTHING;

-- A replay of one accepted proposal must return the same allocation, while a
-- later proposal block (including one after a failure) receives a new number.
CREATE UNIQUE INDEX IF NOT EXISTS uq_generation_history_proposal
    ON generation_history (proposal_id, proposal_block)
    WHERE proposal_id IS NOT NULL;
