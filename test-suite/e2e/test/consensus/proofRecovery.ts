import { isIP } from 'node:net';

/** Committed verifier outcomes survive sender cleanup and local replay deletion. */
export const INSTALL_PROOF_OUTCOME_AUDIT = `
CREATE TABLE public.consensus_test_proof_controls (zk_proof_id bigint PRIMARY KEY);
CREATE TABLE public.consensus_test_proof_outcomes (
  zk_proof_id bigint NOT NULL, verified boolean NOT NULL, handles bytea,
  observed_at timestamptz NOT NULL DEFAULT clock_timestamp(),
  client_address inet, operation text NOT NULL
);
CREATE FUNCTION public.consensus_test_proof_outcome() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  IF TG_OP = 'UPDATE' THEN
    IF NEW.verified IS NOT NULL AND NEW.verified IS DISTINCT FROM OLD.verified THEN
      INSERT INTO public.consensus_test_proof_outcomes(zk_proof_id, verified, handles, client_address, operation)
      VALUES (NEW.zk_proof_id, NEW.verified, NEW.handles, inet_client_addr(), 'verify');
    END IF;
    RETURN NEW;
  END IF;
  -- The replay path never sets verified=true: it atomically materializes the
  -- Gateway-approved handles and deletes the pending queue row. Mere deletion
  -- (including sender cleanup) must not stand in for that committed success.
  IF OLD.verified IS NULL AND OLD.verified_at IS NOT NULL
     AND octet_length(OLD.handles) > 0 AND octet_length(OLD.handles) % 32 = 0
     AND NOT EXISTS (
       SELECT 1 FROM generate_series(0, octet_length(OLD.handles) / 32 - 1) AS h(n)
       WHERE NOT EXISTS (
         SELECT 1 FROM ciphertexts c
         WHERE c.handle = substring(OLD.handles FROM h.n * 32 + 1 FOR 32)
           AND c.ciphertext IS NOT NULL
       )
     ) THEN
    INSERT INTO public.consensus_test_proof_outcomes(zk_proof_id, verified, handles, client_address, operation)
    VALUES (OLD.zk_proof_id, true, OLD.handles, inet_client_addr(), 'replay-delete');
  END IF;
  RETURN OLD;
END;
$$;
CREATE TRIGGER consensus_test_proof_outcome AFTER UPDATE OF verified OR DELETE ON verify_proofs
FOR EACH ROW
EXECUTE FUNCTION public.consensus_test_proof_outcome();`;

export const DROP_PROOF_OUTCOME_AUDIT = `
DROP TRIGGER IF EXISTS consensus_test_proof_outcome ON verify_proofs;
DROP FUNCTION IF EXISTS public.consensus_test_proof_outcome();
DO $$ BEGIN
  IF to_regclass('public.consensus_test_proof_controls') IS NOT NULL THEN
    DELETE FROM verify_proofs WHERE zk_proof_id IN (SELECT zk_proof_id FROM public.consensus_test_proof_controls);
  END IF;
END $$;
DROP TABLE IF EXISTS public.consensus_test_proof_controls;
DROP TABLE IF EXISTS public.consensus_test_proof_outcomes;`;

export interface ProofOutcome {
  verified: boolean;
  handles: string | null;
  clientAddress?: string;
  observedAt?: string;
  operation?: string;
}

export function recoveredVerifierOutcomes(outcomes: ProofOutcome[], workerAddress: string | undefined, faultObservedAt: string | undefined): ProofOutcome[] {
  if (!workerAddress || !isIP(workerAddress) || !faultObservedAt || !Number.isFinite(Date.parse(faultObservedAt))) {
    throw new Error('missing recovered verifier address or fault observation time');
  }
  return outcomes.filter(row => row.clientAddress === workerAddress &&
    row.observedAt !== undefined && Date.parse(row.observedAt) >= Date.parse(faultObservedAt));
}

/** Captured from the interrupted queue row before its worker resumes. */
export interface OriginalProofInput {
  zkProofId: string;
  inputHex: string;
  chainId: string;
  contractAddress: string;
  userAddress: string;
}

/** Preserve the actual serialized ZK proof and change only its bound user.
 * SDK inputProof is a handle/signature attestation, not a serialized TFHE
 * proof. Feeding it to the verifier tests decoding, even if ZK verification
 * is disabled. This control reaches verification with the original encoding.
 */
export function invalidAuxiliaryProof(original: OriginalProofInput): OriginalProofInput {
  if (!original || !/^\d+$/.test(original.zkProofId) || !/^\d+$/.test(original.chainId) ||
      !/^[0-9a-f]+$/i.test(original.inputHex) || original.inputHex.length % 2 !== 0 ||
      !/^0x[0-9a-f]{40}$/i.test(original.contractAddress) ||
      !/^0x[0-9a-f]{40}$/i.test(original.userAddress)) {
    throw new Error('missing or malformed original queued proof input');
  }
  const differentUser = `0x${(BigInt(original.userAddress) ^ 1n).toString(16).padStart(40, '0')}`;
  return { ...original, userAddress: differentUser };
}

export function assertOriginalProofSucceeded(outcomes: ProofOutcome[], handles: string[]): void {
  if (outcomes.some((row) => !row.verified)) throw new Error('the interrupted valid proof was rejected');
  const expected = handles.map((handle) => handle.replace(/^0x/, '').toLowerCase()).join('');
  if (!expected || !outcomes.some((row) => row.verified && row.handles?.toLowerCase() === expected)) {
    throw new Error('no committed success for the original proof and its exact input handles');
  }
}

/** Gateway quorum may finish the SDK request before this operator's replay. */
export async function waitForOriginalProofSucceeded(
  read: () => Promise<ProofOutcome[]>, handles: string[], timeoutMs = 5 * 60_000, pollMs = 250,
): Promise<void> {
  const deadline = Date.now() + timeoutMs;
  for (;;) {
    const outcomes = await read();
    if (outcomes.length > 0 || Date.now() >= deadline) {
      assertOriginalProofSucceeded(outcomes, handles);
      return;
    }
    await new Promise(resolve => setTimeout(resolve, pollMs));
  }
}

/** Absence is still pending, never an inferred rejection. */
export function explicitProofRejection(outcomes: ProofOutcome[]): boolean {
  if (outcomes.some((row) => row.verified)) throw new Error('the invalid proof control was accepted');
  return outcomes.some((row) => row.verified === false);
}
