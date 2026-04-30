-- Tables and triggers used by the `coproc-mngr` upgrade-orchestrator service.
--
-- See RFC-021. This migration only introduces the storage + signaling
-- mechanisms; the consuming services (host-listener, sns-worker, tx-sender)
-- are unchanged in this iteration. coproc-mngr emits pg_notify on the
-- channels listed below; nothing acts on them yet.

-- ----------------------------------------------------------------------------
-- service_state: per-stack FSM persistence (BCS row + GCS row).
--
-- A single Postgres instance only ever hosts one of {BCS, GCS}, so we keep
-- one row per `stack_role`. The `state` column drives every transition in the
-- coproc-mngr; it is the single source of truth for "where is the upgrade
-- right now?". Restart-safe because every transition is committed before the
-- corresponding outbound pg_notify is emitted.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS service_state (
    stack_role        TEXT        PRIMARY KEY
                                  CHECK (stack_role IN ('BCS', 'GCS')),
    state             TEXT        NOT NULL
                                  CHECK (state IN (
                                      -- BCS lifecycle
                                      'LIVE', 'DRAINING', 'STOPPED',
                                      -- GCS lifecycle
                                      'OFFLINE', 'SNAPSHOTTING', 'REPLAYING',
                                      'READY', 'SIGNALING', 'CUTTING_OVER'
                                  )),
    proposal_id       BYTEA       NULL,
    version           TEXT        NULL,
    snapshot_block    BIGINT      NULL CHECK (snapshot_block >= 0),
    eval_block        BIGINT      NULL CHECK (eval_block >= 0),
    state_commitment  BYTEA       NULL,
    last_error        TEXT        NULL,
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Bootstrap: a brand-new BCS deployment is LIVE; any GCS is OFFLINE.
INSERT INTO service_state (stack_role, state)
VALUES ('BCS', 'LIVE')
ON CONFLICT (stack_role) DO NOTHING;

INSERT INTO service_state (stack_role, state)
VALUES ('GCS', 'OFFLINE')
ON CONFLICT (stack_role) DO NOTHING;

-- ----------------------------------------------------------------------------
-- upgrade_events: ingress queue from the (future) gw-listener routing path.
--
-- gw-listener will eventually decode `proposedUpgrade` / `CoprocUpgraded`
-- events and INSERT a row here. coproc-mngr LISTENs on `event_upgrade` and
-- drains the queue. ON CONFLICT (proposal_id, kind) DO NOTHING makes
-- replay safe.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS upgrade_events (
    id                BIGSERIAL   PRIMARY KEY,
    kind              TEXT        NOT NULL
                                  CHECK (kind IN ('proposedUpgrade', 'CoprocUpgraded')),
    proposal_id       BYTEA       NOT NULL,
    version           TEXT        NULL,
    snapshot_block    BIGINT      NULL CHECK (snapshot_block >= 0),
    eval_block        BIGINT      NULL CHECK (eval_block >= 0),
    state_commitment  BYTEA       NULL,
    block_number      BIGINT      NOT NULL,
    transaction_hash  BYTEA       NOT NULL,
    log_index         BIGINT      NULL,
    handled           BOOLEAN     NOT NULL DEFAULT FALSE,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    UNIQUE (proposal_id, kind)
);

CREATE INDEX IF NOT EXISTS idx_upgrade_events_unhandled
    ON upgrade_events (created_at)
    WHERE handled = FALSE;

-- Trigger: fires `event_upgrade` per insert. coproc-mngr LISTENs and
-- pulls unhandled rows.
CREATE OR REPLACE FUNCTION notify_upgrade_event()
    RETURNS trigger AS $$
BEGIN
    NOTIFY event_upgrade;
    RETURN NULL;
END;
$$ LANGUAGE plpgsql;

DROP TRIGGER IF EXISTS upgrade_events_notify_trigger ON upgrade_events;
CREATE TRIGGER upgrade_events_notify_trigger
    AFTER INSERT
    ON upgrade_events
    FOR EACH STATEMENT
    EXECUTE FUNCTION notify_upgrade_event();

-- ----------------------------------------------------------------------------
-- signal_ready_pending: coproc-mngr -> tx-sender handoff.
--
-- coproc-mngr writes a row here when GCS reaches READY and the
-- stateCommitment has been computed. The (future) tx-sender SignalReady op
-- reads `txn_is_sent = FALSE` rows, submits `signalReady(proposalId, commitment)`
-- to the Gateway, and flips the flag.
-- ----------------------------------------------------------------------------
CREATE TABLE IF NOT EXISTS signal_ready_pending (
    proposal_id       BYTEA       PRIMARY KEY,
    state_commitment  BYTEA       NOT NULL,
    version           TEXT        NOT NULL,
    txn_is_sent       BOOLEAN     NOT NULL DEFAULT FALSE,
    txn_hash          BYTEA       NULL,
    created_at        TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_signal_ready_pending_unsent
    ON signal_ready_pending (created_at)
    WHERE txn_is_sent = FALSE;
