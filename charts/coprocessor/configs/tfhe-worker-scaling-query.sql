-- Dependence chains workers hold or can take: live leases, unowned ready
-- chains, expired leases. Each count walks its own partial index. Kept equal
-- to the coprocessor_dependence_chains gauge by a tfhe-worker test.
SELECT
  (SELECT count(*) FROM dependence_chain
    WHERE worker_id IS NOT NULL AND lock_expires_at >= NOW())
+ (SELECT count(*) FROM dependence_chain
    WHERE status = 'updated' AND worker_id IS NULL AND dependency_count = 0)
+ (SELECT count(*) FROM dependence_chain WHERE lock_expires_at < NOW())
