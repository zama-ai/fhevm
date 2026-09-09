-- The executor commits an operand-origin bit for every encrypted operand in
-- a computation handle. Keep the listener's ordered-log reconstruction with
-- the work row so the worker never infers raw-vs-canonical materialization
-- from fork-local database membership.
--
-- The column remains nullable only because blue-green replacement does not
-- semantically backfill pre-existing rows. New listener inserts always write
-- exactly 32 bytes; for legacy NULL rows the worker falls back to the
-- pre-mask inference (an operand is transaction-local iff the same
-- transaction produced it).
-- `NOT VALID` avoids scanning historical rows while still enforcing the
-- invariant for all new writes.
ALTER TABLE computations
    ADD COLUMN operand_boundary_mask BYTEA;

ALTER TABLE computations
    ADD CONSTRAINT computations_operand_boundary_mask_length
    CHECK (
        operand_boundary_mask IS NULL
        OR octet_length(operand_boundary_mask) = 32
    ) NOT VALID;
