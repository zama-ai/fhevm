-- Down for 20261005120000_add_gw_window_rewound_to_upgrade_state.
-- Drops the column. The rewind flag is lost.

ALTER TABLE upgrade_state
    DROP COLUMN IF EXISTS gw_window_rewound;
