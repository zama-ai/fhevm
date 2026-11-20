-- Down for 20260716120100_block_manifest_verification.
-- Drops the verification tables, children first. Verification tasks, attempts,
-- peer downloads and drifted handle findings are lost. The triggers and the
-- columns later migrations added to drifted_handle are already gone when this
-- file runs.

DROP INDEX IF EXISTS idx_block_manifest_verification_task_claimable_due;
DROP INDEX IF EXISTS idx_block_manifest_verification_task_unbound_due;
DROP INDEX IF EXISTS idx_drifted_handle_unresolved;
DROP INDEX IF EXISTS idx_block_manifest_publisher_digest;
DROP INDEX IF EXISTS idx_block_manifest_tip;
DROP INDEX IF EXISTS idx_drifted_handle_forbidden_dependency;
DROP INDEX IF EXISTS idx_drifted_handle_demand_priority;
DROP INDEX IF EXISTS block_manifest_localization_cache_lookup;
DROP TABLE IF EXISTS drifted_handle_demand;
DROP TABLE IF EXISTS drifted_handle;
DROP TABLE IF EXISTS block_manifest_verification_attempt_drift;
DROP TABLE IF EXISTS block_manifest_verification_attempt;
DROP TABLE IF EXISTS block_manifest_peer_download;
DROP TABLE IF EXISTS block_manifest_verification_task;
