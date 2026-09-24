const validateVersion = (version: string): void => {
  if (!/^[a-zA-Z0-9.+_-]+$/.test(version)) throw new Error('invalid synthetic evidence version/chain set');
};

/** Retain committed FSM transitions without delaying or otherwise gating promotion.
 * Installation replaces any objects a killed run left behind: stale evidence
 * from an earlier proposal must not satisfy this run's readiness query.
 */
export const DRY_RUN_EVIDENCE_INSTALL_SQL = `BEGIN;
  DROP FUNCTION IF EXISTS public.consensus_test_capture_dry_run() CASCADE;
  DROP TABLE IF EXISTS public.consensus_test_dry_run_evidence;
  CREATE TABLE public.consensus_test_dry_run_evidence (
    host_chain_id bigint NOT NULL, version text NOT NULL, proposal_id bytea NOT NULL,
    proposal_block bigint NOT NULL,
    PRIMARY KEY(host_chain_id, version, proposal_id, proposal_block));
  CREATE FUNCTION public.consensus_test_capture_dry_run() RETURNS trigger LANGUAGE plpgsql AS $$
  BEGIN
    IF NEW.stack_role='GCS' AND NEW.state='DryRunStarted' THEN
      INSERT INTO public.consensus_test_dry_run_evidence
        VALUES (NEW.host_chain_id, NEW.version, NEW.proposal_id, NEW.proposal_block)
        ON CONFLICT DO NOTHING;
    END IF;
    RETURN NULL;
  END $$;
  CREATE TRIGGER consensus_test_capture_dry_run AFTER INSERT OR UPDATE ON public.upgrade_state
    FOR EACH ROW EXECUTE FUNCTION public.consensus_test_capture_dry_run();
  COMMIT;`;

export const DRY_RUN_EVIDENCE_CLEANUP_SQL = `
  DROP FUNCTION IF EXISTS public.consensus_test_capture_dry_run() CASCADE;
  DROP TABLE IF EXISTS public.consensus_test_dry_run_evidence;`;

export function dryRunEvidenceReadinessSql(version: string, chainIds: string[], proposal: number): string {
  validateVersion(version);
  if (!chainIds.length || chainIds.some(id => !/^[0-9]+$/.test(id)) ||
      new Set(chainIds).size !== chainIds.length || !Number.isSafeInteger(proposal) || proposal < 0) {
    throw new Error('invalid dry-run evidence identity');
  }
  const proposalHex = proposal.toString(16).padStart(64, '0');
  return `SELECT CASE WHEN count(DISTINCT e.host_chain_id)=${chainIds.length}
      AND count(DISTINCT e.proposal_block)=1 THEN 'ready' ELSE 'waiting' END
    FROM public.consensus_test_dry_run_evidence e
    JOIN public.upgrade_state u USING(host_chain_id, version, proposal_id, proposal_block)
    WHERE u.stack_role='GCS' AND e.version='${version}'
      AND e.host_chain_id IN (${chainIds.join(',')})
      AND e.proposal_id=decode('${proposalHex}','hex');`;
}

/** Observe the real cutover DELETE without changing its outcome or timing gates.
 * A statement transition table retains every deleted computation, even after
 * promotion clears the markers and drops Green. The audit commits or rolls back
 * with that deletion, and lives outside the schema that promotion removes.
 */
export function syntheticEvidenceAuditSql(version: string): string {
  validateVersion(version);
  return `BEGIN;
    DROP FUNCTION IF EXISTS public.consensus_test_capture_synthetic() CASCADE;
    DROP TABLE IF EXISTS public.consensus_test_synthetic_evidence;
    CREATE TABLE public.consensus_test_synthetic_evidence (
      version text NOT NULL, host_chain_id bigint NOT NULL, markers bytea NOT NULL,
      computations jsonb NOT NULL, PRIMARY KEY(version, host_chain_id));
    CREATE FUNCTION public.consensus_test_capture_synthetic() RETURNS trigger LANGUAGE plpgsql AS $$
    BEGIN
      INSERT INTO public.consensus_test_synthetic_evidence
      SELECT u.version, u.host_chain_id, u.synthetic_txn_hashes,
        (SELECT jsonb_agg(jsonb_build_object('transaction', encode(c.transaction_id,'hex'),
          'completed', c.is_completed, 'error', c.is_error))
         FROM deleted_synthetic_computations c WHERE c.host_chain_id=u.host_chain_id)
      FROM public.upgrade_state u
      WHERE u.stack_role='GCS' AND u.version='${version}'
        AND octet_length(u.synthetic_txn_hashes)>0
        AND EXISTS (SELECT 1 FROM deleted_synthetic_computations c
          CROSS JOIN LATERAL generate_series(1,octet_length(u.synthetic_txn_hashes),32) pos
          WHERE c.host_chain_id=u.host_chain_id
            AND c.transaction_id=substring(u.synthetic_txn_hashes FROM pos FOR 32))
      ON CONFLICT DO NOTHING;
      RETURN NULL;
    END $$;
    CREATE TRIGGER consensus_test_capture_synthetic
      AFTER DELETE ON "gcs-${version}".computations
      REFERENCING OLD TABLE AS deleted_synthetic_computations
      FOR EACH STATEMENT EXECUTE FUNCTION public.consensus_test_capture_synthetic();
    COMMIT;`;
}

export const SYNTHETIC_EVIDENCE_CLEANUP_SQL = `
  DROP FUNCTION IF EXISTS public.consensus_test_capture_synthetic() CASCADE;
  DROP TABLE IF EXISTS public.consensus_test_synthetic_evidence;`;

/** Synthetic host work is exactly two trivial operands and their add output. */
export function syntheticTrackReadinessSql(version: string, chainIds: string[]): string {
  validateVersion(version);
  if (!chainIds.length ||
      chainIds.some(id => !/^[0-9]+$/.test(id)) || new Set(chainIds).size !== chainIds.length) {
    throw new Error('invalid synthetic evidence version/chain set');
  }
  return `SELECT count(*) FROM public.consensus_test_synthetic_evidence u WHERE
    host_chain_id IN (${chainIds.join(',')}) AND version='${version}'
    AND octet_length(markers)>0
    AND octet_length(markers)%32=0
    AND NOT EXISTS (
      SELECT 1 FROM generate_series(0,octet_length(u.markers)/32-1) n
      WHERE (SELECT count(*) FROM jsonb_array_elements(u.computations) c
        WHERE c->>'transaction'=encode(substring(u.markers FROM n*32+1 FOR 32),'hex')) <> 3
      OR EXISTS (SELECT 1 FROM jsonb_array_elements(u.computations) c
        WHERE c->>'transaction'=encode(substring(u.markers FROM n*32+1 FOR 32),'hex')
          AND (c->'completed' IS DISTINCT FROM 'true'::jsonb
            OR c->'error' IS DISTINCT FROM 'false'::jsonb)))`;
}
