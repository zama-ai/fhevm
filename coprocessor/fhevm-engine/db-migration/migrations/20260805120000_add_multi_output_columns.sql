-- A multi-output operation is stored as N rows sharing `group_id`, with
-- `output_index` 0..N-1. Singletons keep `group_id IS NULL` and
-- `output_index = 0`, so the defaults leave existing writers untouched.
--
-- Both tables are written by the wave-1 dual-write in the host-listener, so
-- both need the columns: `computations` is the current read path and
-- `computations_branch` is the wave-2 read path.

ALTER TABLE computations
    ADD COLUMN IF NOT EXISTS group_id BYTEA NULL,
    ADD COLUMN IF NOT EXISTS output_index SMALLINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_computations_group_id
    ON computations (group_id)
    WHERE group_id IS NOT NULL;

ALTER TABLE computations_branch
    ADD COLUMN IF NOT EXISTS group_id BYTEA NULL,
    ADD COLUMN IF NOT EXISTS output_index SMALLINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_computations_branch_group_id
    ON computations_branch (group_id)
    WHERE group_id IS NOT NULL;
