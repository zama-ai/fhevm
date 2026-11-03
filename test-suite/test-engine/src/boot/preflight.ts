/**
 * Preflight: checks, before `run` starts any scenario, that the environment of the profile answers
 * (chain RPC with the right chain ID, relayer, FHE key). Module-specific checks are exposed by the
 * modules and called from here. Runs in the main process: no SDK (its thread pool would keep the
 * process alive), only ethers and `fetch`.
 */
import type { ResolvedProfile } from '../config/profiles/types.js';
import type { Redactor } from '../infrastructure/secrets/redactor.js';
import type { PreflightCheck, PreflightResult } from '../model/preflight.js';
import { Chain } from '../runtime/modules/chain/chain.js';
import { checkRelayer } from '../runtime/modules/fhe/relayer-check.js';

export type { PreflightCheck, PreflightResult } from '../model/preflight.js';

export interface PreflightOptions {
  /** Timeout of each check (default 10 s). */
  timeoutMs?: number;
  /** Replaces the global `fetch` (unit tests). */
  fetch?: typeof fetch;
}

const DEFAULT_TIMEOUT_MS = 10_000;

const errorMessage = (error: unknown): string => (error instanceof Error ? error.message : String(error));

/** Rejects after `ms` with a timeout message, unless `work` settles first. */
async function withTimeout<T>(work: Promise<T>, ms: number): Promise<T> {
  let timer: NodeJS.Timeout | undefined;
  const timeout = new Promise<never>((_, reject) => {
    timer = setTimeout(() => reject(new Error(`no answer within ${ms} ms`)), ms);
  });
  try {
    return await Promise.race([work, timeout]);
  } finally {
    clearTimeout(timer);
  }
}

/** `chain.rpc`: the Chain module's own check (reachable, and the profile's chain ID). */
async function checkChain(profile: ResolvedProfile, timeoutMs: number): Promise<PreflightCheck> {
  const section = profile.chain!;
  const chain = new Chain({ profileName: profile.name, section, evidence: { record: () => undefined } });
  const started = Date.now();
  const target = section.rpcUrl.value;
  try {
    await withTimeout(chain.verifyConnection(), timeoutMs);
    return {
      name: 'chain.rpc',
      target,
      status: 'passed',
      durationMs: Date.now() - started,
      detail: `chain ID ${chain.chainId}`,
    };
  } catch (error) {
    return {
      name: 'chain.rpc',
      target,
      status: 'failed',
      durationMs: Date.now() - started,
      detail: errorMessage(error),
    };
  } finally {
    chain.destroy();
  }
}

/** Runs the checks the profile's sections call for, in parallel. */
export async function runPreflight(profile: ResolvedProfile, options: PreflightOptions = {}): Promise<PreflightResult> {
  const timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;
  const pending: Promise<PreflightCheck[]>[] = [];
  if (profile.chain) pending.push(checkChain(profile, timeoutMs).then((check) => [check]));
  if (profile.relayer) {
    pending.push(checkRelayer(profile.relayer, { timeoutMs, ...(options.fetch ? { fetch: options.fetch } : {}) }));
  }
  if (pending.length === 0) {
    return { status: 'skipped', reason: `profile '${profile.name}' has no chain or relayer section`, checks: [] };
  }
  const checks = (await Promise.all(pending)).flat();
  return { status: checks.some((check) => check.status === 'failed') ? 'failed' : 'passed', checks };
}

/** A copy of the result safe to print or store: secrets replaced by their placeholder. */
export function redactedPreflight(result: PreflightResult, redactor: Redactor): PreflightResult {
  return redactor.redactDeep(result);
}
