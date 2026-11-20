CREATE TABLE IF NOT EXISTS upgrade_state (
    stack_role        TEXT        PRIMARY KEY
                                  CHECK (stack_role IN ('BCS', 'GCS')),
    state             TEXT        NOT NULL
                                  CHECK (state IN (
                                      'LIVE',
                                      'UpgradeActivated',
                                      'UpgradeAuthorized',
                                      'PAUSED'
                                  )),
    status            TEXT        NOT NULL
                                  CHECK (status IN (
                                      'in_progress',
                                      'failed',
                                      'completed'
                                  )),
    proposal_id       BYTEA       NULL,
    version           TEXT        NULL,
    start_block       BIGINT      NULL CHECK (start_block      >= 0),
    end_block         BIGINT      NULL CHECK (end_block        >= 0),
    gw_start_block    BIGINT      NULL CHECK (gw_start_block   >= 0),
    last_error        TEXT        NULL,
    updated_at        TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
