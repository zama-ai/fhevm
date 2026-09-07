-- Index for the pending-work guards keyed on dependence_chain_id whose
-- predicates are not implied by either existing partial dcid index.
--
-- idx_computations_dependence_chain requires is_error = false and
-- idx_computations_pending_dcid_schedule requires is_allowed = true. Two
-- guards on this branch need neither:
--   * delete_old_processed_dependence_chains refuses to delete a chain that
--     still owns a pending row that is EITHER retryable-errored (any
--     is_allowed) OR allowed-and-not-errored. With no implied index the
--     planner built a hash of every pending computation per sweep and sorted
--     the anti-join on disk: 200-355 ms per hourly sweep on 84k pending rows,
--     growing with the pending set. With this index the sweep is an
--     index-driven anti-join in schedule order: 13 ms.
--   * rearm_demoted_chains' `reset` CTE joins pending errored rows by chain.
--
-- Same rollout shape as 20260814120000: a plain in-transaction CREATE INDEX
-- here (sqlx-cli 0.7.2 has no no-transaction support), pre-built
-- CONCURRENTLY on a populated database by `precreate_index` in
-- db-migration/initialize_db.sh, after which this statement no-ops.
CREATE INDEX IF NOT EXISTS idx_computations_pending_dcid
ON computations (dependence_chain_id)
WHERE is_completed = false;
