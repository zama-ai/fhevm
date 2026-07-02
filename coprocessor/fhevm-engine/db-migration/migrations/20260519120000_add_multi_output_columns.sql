ALTER TABLE computations
    ADD COLUMN IF NOT EXISTS group_id BYTEA NULL,
    ADD COLUMN IF NOT EXISTS output_index SMALLINT NOT NULL DEFAULT 0;

CREATE INDEX IF NOT EXISTS idx_computations_group_id
    ON computations (group_id)
    WHERE group_id IS NOT NULL;
