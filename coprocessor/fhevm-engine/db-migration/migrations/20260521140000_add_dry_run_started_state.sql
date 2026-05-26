-- Extend upgrade_state.state with `DryRunStarted` — set by upgrade-controller
-- (GCS) once the pre-snapshot completeness check passes and the unpause notify
-- has been emitted.
ALTER TABLE upgrade_state DROP CONSTRAINT upgrade_state_state_check;
ALTER TABLE upgrade_state ADD CONSTRAINT upgrade_state_state_check
    CHECK (state IN (
        'LIVE',
        'UpgradeActivated',
        'DryRunStarted',
        'UpgradeAuthorized',
        'PAUSED'
    ));
