-- Shared safety state intentionally lives in public. It must survive green
-- schema recreation and block acceptance in both stacks until operator recovery.
CREATE TABLE public.chain_recovery_halts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    source TEXT NOT NULL CHECK (source = 'host'),
    stack_role TEXT NOT NULL CHECK (stack_role IN ('BCS', 'GCS')),
    chain_id BIGINT NOT NULL,
    block_number BIGINT NOT NULL CHECK (block_number >= 0),
    expected_hash BYTEA CHECK (octet_length(expected_hash) = 32),
    observed_hash BYTEA CHECK (octet_length(observed_hash) = 32),
    reason TEXT NOT NULL,
    detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_detected_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    resolved_at TIMESTAMPTZ,
    recovery_note TEXT,
    CHECK (resolved_at IS NULL OR NULLIF(BTRIM(recovery_note), '') IS NOT NULL)
);
CREATE UNIQUE INDEX chain_recovery_halts_active_identity
    ON public.chain_recovery_halts (source, stack_role, chain_id, block_number)
    WHERE resolved_at IS NULL;

CREATE TABLE public.listener_replay_state (
    source TEXT NOT NULL CHECK (source = 'host'),
    stack_role TEXT NOT NULL CHECK (stack_role IN ('BCS', 'GCS')),
    chain_id BIGINT NOT NULL,
    from_block BIGINT NOT NULL CHECK (from_block >= 0),
    to_block BIGINT NOT NULL CHECK (to_block >= from_block),
    pending BOOLEAN NOT NULL DEFAULT TRUE,
    generation BIGINT NOT NULL DEFAULT 1 CHECK (generation > 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    PRIMARY KEY (source, stack_role, chain_id)
);
