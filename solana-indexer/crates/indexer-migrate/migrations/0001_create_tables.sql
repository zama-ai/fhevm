-- Rotation-leaf indexer schema. The natural key per lineage is the on-chain
-- encrypted_value_acl PDA (accounts[2] on every EV-ACL instruction). value_key
-- (the API key) is carried only by initialize_encrypted_value_acl, so it is
-- recorded on the PDA row and mapped back to the PDA for the value_key-keyed API.

-- Per-lineage mutable shadow. Source of current_handle / current_subjects for
-- the snapshot captured immediately before the next rotate / mark_public.
CREATE TABLE IF NOT EXISTS lineage_state (
    pda              BYTEA PRIMARY KEY,                 -- 32-byte encrypted_value_acl PDA = accounts[2]
    value_key        BYTEA UNIQUE,                      -- 32-byte acl_nonce_key; non-NULL once initialize seen
    current_handle   BYTEA NOT NULL,
    current_subjects BYTEA[] NOT NULL DEFAULT '{}',     -- ordered 32-byte subjects (insertion order; never sorted/deduped beyond on-chain rules)
    leaf_count       BIGINT NOT NULL DEFAULT 0,
    updated_at       TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Append-only reconstruction source of truth, ordered per lineage by event_index.
CREATE TABLE IF NOT EXISTS lineage_events (
    id                BIGSERIAL PRIMARY KEY,
    pda               BYTEA NOT NULL,
    event_index       BIGINT NOT NULL,                  -- 0-based ordinal within the lineage (assigned by the processor)
    event_type        SMALLINT NOT NULL,                -- 0 = Rotation, 1 = MarkedPublic
    old_handle        BYTEA,                            -- non-NULL for Rotation (= current_handle before rotate)
    subjects_snapshot BYTEA[],                          -- non-NULL for Rotation: current_subjects in order, taken BEFORE the rotate applies
    handle            BYTEA,                            -- non-NULL for MarkedPublic (= current_handle at mark time)
    signature         TEXT NOT NULL,
    slot              BIGINT NOT NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    UNIQUE (pda, event_index)                           -- idempotent re-delivery: ON CONFLICT DO NOTHING
);

-- Primary reconstruction scan (pda ordered by event_index) and leaf-lookup scan.
CREATE INDEX IF NOT EXISTS lineage_events_pda_index ON lineage_events (pda, event_index);

-- Single-row global Carbon resume cursor, advanced in the SAME tx as each event
-- insert so a crash leaves no gap (a gap is unsafe; a duplicate is a no-op).
CREATE TABLE IF NOT EXISTS indexer_cursor (
    id             INT PRIMARY KEY DEFAULT 1 CHECK (id = 1),
    last_signature TEXT NOT NULL DEFAULT '',
    last_slot      BIGINT NOT NULL DEFAULT 0,
    updated_at     TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
INSERT INTO indexer_cursor (id) VALUES (1) ON CONFLICT (id) DO NOTHING;
