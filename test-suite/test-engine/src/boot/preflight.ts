/**
 * Preflight: checks, before `run` starts any scenario, that the environment of the profile answers
 * (chain RPC with the right chain ID, protocol contracts deployed, relayer, FHE key) and that the
 * compiled contracts of `--contracts-dir` match the profile. Module-specific checks are exposed by the
 * modules and called from here. Runs in the main process: no SDK (its thread pool would keep the
 * process alive), only ethers and `fetch`.
 */
import type { ResolvedProfile } from '../config/profiles/types.js';
import type { Redactor } from '../infrastructure/secrets/redactor.js';
import type { PreflightCheck, PreflightResult } from '../model/preflight.js';
import { Chain } from '../runtime/modules/chain/chain.js';
import { checkBuild } from '../runtime/modules/contracts/build-check.js';
import { checkRelayer } from '../runtime/modules/fhe/relayer-check.js';

export type { PreflightCheck, PreflightResult } from '../model/preflight.js';

export interface PreflightOptions {
  /** Timeout of each check (default 10 s). */
  timeoutMs?: number;
  /** Replaces the global `fetch` (unit tests). */
  fetch?: typeof fetch;
  /** The run's `--contracts-dir`: adds the `contracts.build` check. */
  contractsDir?: string;
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

const CONTRACT_FIELDS = ['acl', 'executor', 'inputVerifier', 'kmsVerifier', 'protocolConfig'] as const;

/**
 * `chain.rpc`: the Chain module's own check (reachable, and the profile's chain ID); then, when it
 * passed and the profile has a `contracts` section, `contracts.code`: every protocol contract of the
 * profile has code on that chain.
 */
async function checkChain(profile: ResolvedProfile, timeoutMs: number): Promise<PreflightCheck[]> {
  const section = profile.chain!;
  const chain = new Chain({ profileName: profile.name, section, evidence: { record: () => undefined } });
  const target = section.rpcUrl.value;
  let started = Date.now();
  try {
    await withTimeout(chain.verifyConnection(), timeoutMs);
  } catch (error) {
    chain.destroy();
    const failed: PreflightCheck = {
      name: 'chain.rpc',
      target,
      status: 'failed',
      durationMs: Date.now() - started,
      detail: errorMessage(error),
    };
    return profile.contracts
      ? [failed, { name: 'contracts.code', target, status: 'skipped', durationMs: 0, detail: 'chain.rpc failed' }]
      : [failed];
  }
  const checks: PreflightCheck[] = [
    {
      name: 'chain.rpc',
      target,
      status: 'passed',
      durationMs: Date.now() - started,
      detail: `chain ID ${chain.chainId}`,
    },
  ];
  const contracts = profile.contracts;
  if (contracts) {
    started = Date.now();
    try {
      const codes = await withTimeout(
        Promise.all(CONTRACT_FIELDS.map((field) => chain.provider.getCode(contracts[field].value))),
        timeoutMs,
      );
      const empty = CONTRACT_FIELDS.filter((_, index) => codes[index] === '0x');
      checks.push({
        name: 'contracts.code',
        target,
        status: empty.length === 0 ? 'passed' : 'failed',
        durationMs: Date.now() - started,
        detail:
          empty.length === 0
            ? `${CONTRACT_FIELDS.length} protocol contracts deployed`
            : `no contract at ${empty.map((field) => `contracts.${field} (${contracts[field].value})`).join(', ')}: the profile does not match this chain`,
      });
    } catch (error) {
      checks.push({
        name: 'contracts.code',
        target,
        status: 'failed',
        durationMs: Date.now() - started,
        detail: errorMessage(error),
      });
    }
  }
  chain.destroy();
  return checks;
}

/** Runs the checks the profile's sections call for, in parallel. */
export async function runPreflight(profile: ResolvedProfile, options: PreflightOptions = {}): Promise<PreflightResult> {
  const timeoutMs = options.timeoutMs ?? DEFAULT_TIMEOUT_MS;
  const pending: Promise<PreflightCheck[]>[] = [];
  if (profile.chain) pending.push(checkChain(profile, timeoutMs));
  if (profile.relayer) {
    pending.push(checkRelayer(profile.relayer, { timeoutMs, ...(options.fetch ? { fetch: options.fetch } : {}) }));
  }
  if (options.contractsDir !== undefined) pending.push(Promise.resolve([checkBuild(profile, options.contractsDir)]));
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
