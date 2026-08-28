-- One operation, N rows: same `group_id`, `output_index` 0..N-1, `output_count`
-- = N on every row. Singletons take the defaults, so existing writers are
-- untouched. `output_count` is what makes a truncated group detectable — rows
-- 0..N-2 on their own look complete.

ALTER TABLE computations
    ADD COLUMN IF NOT EXISTS group_id BYTEA NULL,
    ADD COLUMN IF NOT EXISTS output_index SMALLINT NOT NULL DEFAULT 0,
    ADD COLUMN IF NOT EXISTS output_count SMALLINT NOT NULL DEFAULT 1;
