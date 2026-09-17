-- Drift propagation finds consumers by encrypted operand handle. Scalar slots
-- share this array, so Rust still checks operation semantics after the lookup.
CREATE INDEX idx_computations_containment_dependencies
ON computations USING GIN (dependencies);

-- Periodic recovery checks only unfinished ct64 containment in this epoch.
CREATE INDEX idx_drifted_handle_pending_containment
ON drifted_handle (consensus_epoch)
WHERE reason = 'ct64_mismatch' AND healed_at IS NULL AND NOT is_contained;
