-- Down for 20260813120000_computations_operand_boundary_mask.
-- Drops the constraint and the column. Stored masks are lost.

ALTER TABLE computations
    DROP CONSTRAINT IF EXISTS computations_operand_boundary_mask_length;

ALTER TABLE computations
    DROP COLUMN IF EXISTS operand_boundary_mask;
