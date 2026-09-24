WITH held AS (
  UPDATE public.consensus_test_host_report_fault f
     SET observed_at = COALESCE(f.observed_at, clock_timestamp()),
         observed_block = COALESCE(f.observed_block, $2)
   WHERE f.chain_id = $1
     AND f.expires_at > clock_timestamp()
     AND EXISTS (
       SELECT 1 FROM public.upgrade_state u
        WHERE u.stack_role = 'GCS' AND u.state = 'DryRunStarted'
          AND u.version = f.version AND u.proposal_id = f.proposal_id
          AND u.host_chain_id = f.chain_id
          AND $2 BETWEEN u.start_block AND u.end_block
     )
  RETURNING mode
)
SELECT mode FROM held
