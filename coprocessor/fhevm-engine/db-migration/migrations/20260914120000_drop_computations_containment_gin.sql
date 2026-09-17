-- Containment reads computations by (host_chain_id, block_number) from the
-- oldest unhealed ct64 finding and filters operands in Rust. The GIN on
-- `dependencies` is unused and expensive on this write-heavy table.
DROP INDEX IF EXISTS idx_computations_containment_dependencies;
