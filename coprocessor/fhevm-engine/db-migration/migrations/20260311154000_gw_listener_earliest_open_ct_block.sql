ALTER TABLE gw_listener_last_block
ADD COLUMN IF NOT EXISTS earliest_open_ct_commits_block BIGINT
CHECK (earliest_open_ct_commits_block >= 0);
