/**
 * Relayer checks of the preflight: whether the relayer answers the first call the SDK makes
 * (`GET /v2/keyurl`) and whether the FHE public key it points to can be reached. Plain `fetch`, no
 * SDK: the preflight runs in the main process, where the SDK's thread pool would keep it alive.
 */
import { relayerBaseUrl } from '../../../config/profiles/profile.js';
import type { ResolvedRelayerSection } from '../../../config/profiles/types.js';
import type { PreflightCheck } from '../../../model/preflight.js';

export interface RelayerCheckOptions {
  timeoutMs: number;
  /** Replaces the global `fetch` (unit tests). */
  fetch?: typeof fetch;
}

const errorMessage = (error: unknown): string => {
  const cause = (error as { cause?: { code?: string; message?: string } }).cause;
  if (error instanceof Error && error.name === 'TimeoutError') return 'no answer (timeout)';
  if (cause?.code) return `${cause.code}${cause.message ? ` (${cause.message})` : ''}`;
  return error instanceof Error ? error.message : String(error);
};

/** The first FHE public key URL of a `/v2/keyurl` answer, if any. */
function publicKeyUrl(body: unknown): string | undefined {
  const info = (body as { response?: { fheKeyInfo?: { fhePublicKey?: { urls?: unknown[] } }[] } })?.response
    ?.fheKeyInfo;
  const url = info?.[0]?.fhePublicKey?.urls?.[0];
  return typeof url === 'string' ? url : undefined;
}

/**
 * `relayer.keyurl`, then `relayer.fhe-key` (skipped when the first failed). The API key, when the
 * profile has a non-empty one, is sent as the SDK sends it (`x-api-key`).
 */
export async function checkRelayer(
  relayer: ResolvedRelayerSection,
  options: RelayerCheckOptions,
): Promise<PreflightCheck[]> {
  const fetchFn = options.fetch ?? fetch;
  const keyUrlTarget = `${relayerBaseUrl(relayer.url.value)}/v2/keyurl`;
  const apiKey = relayer.apiKey?.value ?? '';
  const headers: Record<string, string> = apiKey === '' ? {} : { 'x-api-key': apiKey };

  let started = Date.now();
  let keyUrl: string | undefined;
  let keyUrlCheck: PreflightCheck;
  try {
    const response = await fetchFn(keyUrlTarget, { headers, signal: AbortSignal.timeout(options.timeoutMs) });
    if (response.status === 401 || response.status === 403) {
      keyUrlCheck = failed(
        'relayer.keyurl',
        keyUrlTarget,
        started,
        `HTTP ${response.status}: the relayer refused the request (${apiKey === '' ? 'it requires relayer.apiKey' : 'relayer.apiKey rejected'})`,
      );
    } else if (!response.ok) {
      keyUrlCheck = failed('relayer.keyurl', keyUrlTarget, started, `HTTP ${response.status}`);
    } else {
      keyUrl = publicKeyUrl(await response.json().catch(() => undefined));
      keyUrlCheck = keyUrl
        ? { name: 'relayer.keyurl', target: keyUrlTarget, status: 'passed', durationMs: Date.now() - started }
        : failed('relayer.keyurl', keyUrlTarget, started, 'the answer holds no FHE public key URL');
    }
  } catch (error) {
    keyUrlCheck = failed('relayer.keyurl', keyUrlTarget, started, `relayer unreachable: ${errorMessage(error)}`);
  }
  if (!keyUrl) {
    return [
      keyUrlCheck,
      { name: 'relayer.fhe-key', target: '-', status: 'skipped', durationMs: 0, detail: 'relayer.keyurl failed' },
    ];
  }

  started = Date.now();
  let keyCheck: PreflightCheck;
  try {
    const response = await fetchFn(keyUrl, { method: 'HEAD', signal: AbortSignal.timeout(options.timeoutMs) });
    keyCheck = response.ok
      ? { name: 'relayer.fhe-key', target: keyUrl, status: 'passed', durationMs: Date.now() - started }
      : failed('relayer.fhe-key', keyUrl, started, `HTTP ${response.status}`);
  } catch (error) {
    keyCheck = failed(
      'relayer.fhe-key',
      keyUrl,
      started,
      `FHE key unreachable: ${errorMessage(error)}; the relayer hands out addresses of the stack's network, so the engine must run on it (e.g. in the e2e container)`,
    );
  }
  return [keyUrlCheck, keyCheck];
}

function failed(name: string, target: string, started: number, detail: string): PreflightCheck {
  return { name, target, status: 'failed', durationMs: Date.now() - started, detail };
}
