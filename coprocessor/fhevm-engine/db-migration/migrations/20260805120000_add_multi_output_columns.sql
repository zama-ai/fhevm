-- A multi-output operation is stored as N rows sharing `group_id`, with
-- `output_index` 0..N-1. Singletons keep `group_id IS NULL` and
-- `output_index = 0`, so the defaults leave existing writers untouched.

ALTER TABLE computations
    ADD COLUMN IF NOT EXISTS group_id BYTEA NULL,
    ADD COLUMN IF NOT EXISTS output_index SMALLINT NOT NULL DEFAULT 0;
