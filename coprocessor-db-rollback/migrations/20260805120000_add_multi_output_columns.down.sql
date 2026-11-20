-- Down for 20260805120000_add_multi_output_columns.
-- Drops the three columns. Group ids and output positions are lost.

ALTER TABLE computations
    DROP COLUMN IF EXISTS output_count,
    DROP COLUMN IF EXISTS output_index,
    DROP COLUMN IF EXISTS group_id;
