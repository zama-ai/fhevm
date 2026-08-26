-- Covers the worker's ready-work acquisition query: select allowed pending
-- computations for an acquired DCID in schedule order, then expand the
-- matching transaction. The existing DCID index cannot satisfy the ordering
-- and the generic schedule-order index cannot restrict by owned DCID.
CREATE INDEX IF NOT EXISTS idx_computations_pending_dcid_schedule_transaction
ON computations (dependence_chain_id, schedule_order, transaction_id)
WHERE is_completed = false AND is_error = false AND is_allowed = true;
