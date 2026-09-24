import type { OperatorEvidence } from './comparator';

/**
 * A lost producer-release update leaves a real, materializable child gated.
 * The per-child mutation control suppresses only stale-gate acquisition: a
 * normal acquisition or ordinary producer completion cannot satisfy the test.
 */
export const INSTALL_FORK_GATE_CONTROL = `
CREATE TABLE public.consensus_test_fork_gate (
  child bytea PRIMARY KEY, disable_repair boolean NOT NULL DEFAULT true,
  lost_decrements integer NOT NULL DEFAULT 0,
  repair_attempts integer NOT NULL DEFAULT 0, repair_claims integer NOT NULL DEFAULT 0
);
CREATE FUNCTION public.consensus_test_fork_gate() RETURNS trigger LANGUAGE plpgsql AS $$
DECLARE disabled boolean; lost integer;
BEGIN
  SELECT disable_repair, lost_decrements INTO disabled, lost FROM public.consensus_test_fork_gate WHERE child = OLD.dependence_chain_id;
  IF NOT FOUND THEN RETURN NEW; END IF;
  IF OLD.dependency_count > 0 AND NEW.dependency_count < OLD.dependency_count THEN
    IF OLD.worker_id IS NULL AND NEW.worker_id IS NOT NULL AND NEW.status = 'processing' THEN
      UPDATE public.consensus_test_fork_gate SET repair_attempts = repair_attempts + 1,
        repair_claims = repair_claims + CASE WHEN disabled THEN 0 ELSE 1 END
        WHERE child = OLD.dependence_chain_id;
      IF disabled THEN RETURN NULL; END IF;
    ELSIF lost = 0 THEN
      -- Inject the lost decrement while retaining the completed producer and
      -- its ciphertext, so repair has useful work to recover.
      NEW.dependency_count := OLD.dependency_count;
      UPDATE public.consensus_test_fork_gate SET lost_decrements = 1 WHERE child = OLD.dependence_chain_id;
    END IF;
  END IF;
  RETURN NEW;
END;
$$;
CREATE TRIGGER consensus_test_fork_gate BEFORE UPDATE ON dependence_chain
FOR EACH ROW EXECUTE FUNCTION public.consensus_test_fork_gate();`;

export const DROP_FORK_GATE_CONTROL = `
DROP TRIGGER IF EXISTS consensus_test_fork_gate ON dependence_chain;
DROP FUNCTION IF EXISTS public.consensus_test_fork_gate();
DROP TABLE IF EXISTS public.consensus_test_fork_gate;`;

export interface ForkGateObservation {
  status: string; dependencyCount: number; owned: boolean; unprocessedProducers: number;
}

export function assertRepairDisabledControl(state: ForkGateObservation | null, attempts: number): void {
  if (!state || state.status !== 'updated' || state.dependencyCount <= 0 || state.owned || state.unprocessedProducers !== 0) {
    throw new Error('disabled repair did not leave the identified child genuinely stranded');
  }
  if (attempts <= 0) throw new Error('the disabled repair path was never attempted');
}

export function assertReplayHasEffects(counts: { computations: number; allows: number; chains: number }): void {
  if (counts.computations <= 0 || counts.allows <= 0 || counts.chains <= 0) {
    throw new Error('replay range must contain the identified graph, allow observations and dependence chains');
  }
}

/** Raw SNS payloads are legitimately deleted on submission; durable outputs are not. */
export function replayOutputFingerprint(evidence: OperatorEvidence): string {
  return JSON.stringify({ ...evidence, rawSnsCiphertext: undefined });
}

/** BEFORE INSERT also runs for ON CONFLICT DO NOTHING, within the ingest transaction. */
export const INSTALL_REPLAY_INSERT_AUDIT = `
CREATE TABLE public.consensus_test_replay_inserts (
  output_handle bytea NOT NULL, transaction_id bytea NOT NULL, client_address inet NOT NULL,
  attempts integer NOT NULL DEFAULT 0, PRIMARY KEY(output_handle, transaction_id)
);
CREATE FUNCTION public.consensus_test_replay_insert() RETURNS trigger LANGUAGE plpgsql AS $$
BEGIN
  UPDATE public.consensus_test_replay_inserts SET attempts = attempts + 1
  WHERE output_handle = NEW.output_handle AND transaction_id = NEW.transaction_id
    AND client_address = inet_client_addr();
  RETURN NEW;
END;
$$;
CREATE TRIGGER consensus_test_replay_insert BEFORE INSERT ON computations
FOR EACH ROW EXECUTE FUNCTION public.consensus_test_replay_insert();`;

export const DROP_REPLAY_INSERT_AUDIT = `
DROP TRIGGER IF EXISTS consensus_test_replay_insert ON computations;
DROP FUNCTION IF EXISTS public.consensus_test_replay_insert();
DROP TABLE IF EXISTS public.consensus_test_replay_inserts;`;

export function assertReplayAttempts(attempts: number[], expected: number): void {
  if (expected <= 0 || attempts.length !== expected || attempts.some(count => count < 1)) {
    throw new Error('the restarted poller did not commit insert attempts for every selected replay event');
  }
}

/** Read state only after the exact branch transaction has succeeded on-chain. */
export async function successfulForkReceipt<T extends { status: number | null; blockNumber: number }>(
  transaction: { wait(): Promise<T | null> }, label: string,
): Promise<T> {
  const receipt = await transaction.wait();
  if (!receipt || receipt.status !== 1 || !Number.isSafeInteger(receipt.blockNumber) || receipt.blockNumber < 1) {
    throw new Error(`${label} did not produce a successful mined receipt`);
  }
  return receipt;
}

export interface ForkSentinelObservation {
  replacementSeen: boolean;
  total: number;
  completed: number;
  errors: number;
}

/** A stopped listener/worker cannot satisfy recovery with old fork evidence. */
export async function waitForForkSentinel(
  read: () => Promise<ForkSentinelObservation>,
  options: { timeoutMs?: number; now?: () => number; pause?: () => Promise<void> } = {},
): Promise<void> {
  const now = options.now ?? Date.now;
  const deadline = now() + (options.timeoutMs ?? 6 * 60_000);
  const pause = options.pause ?? (() => new Promise(resolve => setTimeout(resolve, 2_000)));
  for (;;) {
    const row = await read();
    if ([row.total, row.completed, row.errors].some(value => !Number.isSafeInteger(value) || value < 0)) throw new Error('invalid recovery counts');
    if (row.errors !== 0 || row.total > 1) throw new Error('canonical sentinel computation failed or duplicated');
    if (row.replacementSeen && row.total === 1 && row.completed === 1) return;
    if (now() >= deadline) throw new Error('forked operator did not ingest the canonical replacement and complete its fresh sentinel');
    await pause();
  }
}
