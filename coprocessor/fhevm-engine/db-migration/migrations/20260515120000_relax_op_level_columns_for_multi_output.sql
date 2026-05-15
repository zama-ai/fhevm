-- Multi-output ops store op-level fields only on the primary row (output_index = 0).
-- Sibling rows leave these columns NULL.
--
-- Rolling deploy order: (1) run this migration, (2) deploy tfhe-worker, (3) deploy
-- host-listener. Old workers cannot decode NULL on these columns; the host-listener
-- only emits NULL for siblings (multi-output ops), so it must roll out last.
ALTER TABLE computations
    ALTER COLUMN dependencies  DROP NOT NULL,
    ALTER COLUMN fhe_operation DROP NOT NULL,
    ALTER COLUMN is_scalar     DROP NOT NULL,
    ALTER COLUMN is_allowed    DROP NOT NULL;
