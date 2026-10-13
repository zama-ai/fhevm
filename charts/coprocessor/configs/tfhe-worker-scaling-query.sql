-- Dependence chains workers hold or can take: live leases, unowned ready
-- chains, expired leases. Equal to the sum of the coprocessor_dependence_chains
-- gauge (a tfhe-worker test checks it). One subselect per state, so each
-- count walks its own partial index.
SELECT
  (SELECT count(*) FROM dependence_chain
    WHERE worker_id IS NOT NULL AND lock_expires_at >= NOW())
+ (SELECT count(*) FROM dependence_chain
    WHERE status = 'updated' AND worker_id IS NULL AND dependency_count = 0)
+ (SELECT count(*) FROM dependence_chain WHERE lock_expires_at < NOW())
