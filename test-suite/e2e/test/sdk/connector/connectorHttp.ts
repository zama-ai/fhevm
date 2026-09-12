// Minimal client for the kms-connector HTTP decryption endpoint (RFC 033):
// `GET /v1/version`, `POST /v1/public-decrypt`, `POST /v1/user-decrypt`.
import type { Signer } from 'ethers';
import { getBytes, hexlify } from 'ethers';

import type { SignMode, UnifiedDecryptRequest } from '../unified/unifiedUserDecrypt';
import { backdatedStartTimestamp, signRequest } from '../unified/unifiedUserDecrypt';

////////////////////////////////////////////////////////////////////////////////
// Configuration
////////////////////////////////////////////////////////////////////////////////

export const VERSION_ROUTE = '/v1/version';
export const PUBLIC_DECRYPT_ROUTE = '/v1/public-decrypt';
export const USER_DECRYPT_ROUTE = '/v1/user-decrypt';

/** Comma-separated endpoint base URLs, one per committee KMS party (rendered by fhevm-cli). */
export const endpointUrls = (): string[] =>
  (process.env.KMS_CONNECTOR_ENDPOINT_URLS ?? '')
    .split(',')
    .map((url) => url.trim().replace(/\/$/, ''))
    .filter(Boolean);

/** MPC threshold `t` (0 in centralized mode); the decryption quorum is `2t+1`. */
export const kmsThreshold = (): number => Number(process.env.KMS_THRESHOLD ?? '0') || 0;

export const quorum = (): number => 2 * kmsThreshold() + 1;

/** True when the stack renders at least one endpoint URL; the suites skip otherwise. */
export const connectorHttpConfigured = (): boolean => endpointUrls().length > 0;

////////////////////////////////////////////////////////////////////////////////
// Wire types (mirror kms-connector/crates/api/src/types.rs — keep the field order)
////////////////////////////////////////////////////////////////////////////////

export interface PublicDecryptionRequest {
  ctHandles: string[];
  extraData: string;
}

export interface HandleEntry {
  handle: string;
  contractAddress: string;
  ownerAddress: string;
}

export interface RequestValidity {
  startTimestamp: number;
  durationSeconds: number;
}

export interface UserDecryptionRequest {
  handles: HandleEntry[];
  userAddress: string;
  publicKey: string;
  allowedContracts: string[];
  requestValidity: RequestValidity;
  signature: string;
  extraData: string;
}

export interface PublicDecryptionResponse {
  decryptionId: string;
  decryptedResult: string;
  signature: string;
  extraData: string;
}

export interface UserDecryptionResponse {
  decryptionId: string;
  userDecryptedShares: string;
  signature: string;
  extraData: string;
}

export interface ErrorResponse {
  code: string;
  message: string;
  retryable: boolean;
  decryptionId?: string;
}

/** `retryable` as the connector computes it from the code (ErrorCode::retryable). */
export const RETRYABLE_BY_CODE: Record<string, boolean> = {
  malformed: false,
  sender_authentication_failed: false,
  kms_context_destroyed: false,
  unprocessable: false,
  rate_limited: true,
  overloaded: true,
  acl_denied: true,
  user_signature_rejected: true,
  ciphertext_not_found: true,
  copro_consensus_failed: true,
  kms_context_invalid: true,
  upstream_transient: true,
  timeout: true,
  unknown: true,
};

/** Worker/endpoint outcomes a client re-submits: the same body may succeed a moment later. */
const TRANSIENT_CODES = new Set(['ciphertext_not_found', 'copro_consensus_failed', 'upstream_transient', 'timeout']);

////////////////////////////////////////////////////////////////////////////////
// HTTP
////////////////////////////////////////////////////////////////////////////////

export interface PostResult<T> {
  readonly url: string;
  readonly httpStatus: number;
  readonly body: T | ErrorResponse | Record<string, never>;
  readonly elapsedMs: number;
}

export interface PostOptions {
  /** Per-request timeout; defaults to 120s (the endpoint's own decryption timeout is 30s). */
  readonly timeoutMs?: number;
}

const readJson = async (resp: Response): Promise<Record<string, unknown>> => {
  try {
    return (await resp.json()) as Record<string, unknown>;
  } catch {
    return {};
  }
};

/** POST a JSON `body` to `url + route`. Never throws on a non-2xx status. */
export async function post<T>(url: string, route: string, body: unknown, opts?: PostOptions): Promise<PostResult<T>> {
  const started = Date.now();
  const resp = await fetch(`${url}${route}`, {
    method: 'POST',
    headers: { 'content-type': 'application/json' },
    body: JSON.stringify(body),
    signal: AbortSignal.timeout(opts?.timeoutMs ?? 120_000),
  });
  return {
    url,
    httpStatus: resp.status,
    body: (await readJson(resp)) as PostResult<T>['body'],
    elapsedMs: Date.now() - started,
  };
}

export async function getVersion(url: string): Promise<{ httpStatus: number; body: Record<string, unknown> }> {
  const resp = await fetch(`${url}${VERSION_ROUTE}`, { signal: AbortSignal.timeout(10_000) });
  return { httpStatus: resp.status, body: await readJson(resp) };
}

export const isError = (body: unknown): body is ErrorResponse =>
  typeof body === 'object' && body !== null && typeof (body as ErrorResponse).code === 'string';

export const isTransient = <T>(result: PostResult<T>): boolean =>
  isError(result.body) && TRANSIENT_CODES.has(result.body.code);

/** Formats a result for assertion messages. */
export const describeResult = <T>(result: PostResult<T>): string =>
  `${result.url} -> ${result.httpStatus} ${JSON.stringify(result.body)}`;

////////////////////////////////////////////////////////////////////////////////
// Fan-out
////////////////////////////////////////////////////////////////////////////////

export interface FanOutResult<T> {
  readonly results: PostResult<T>[];
  readonly successes: PostResult<T>[];
  readonly failures: PostResult<T>[];
}

const split = <T>(results: PostResult<T>[]): FanOutResult<T> => ({
  results,
  successes: results.filter((r) => r.httpStatus === 200),
  failures: results.filter((r) => r.httpStatus !== 200),
});

/** POST the same body to every configured party concurrently. */
export async function fanOut<T>(
  route: string,
  body: unknown,
  urls = endpointUrls(),
  opts?: PostOptions,
): Promise<FanOutResult<T>> {
  const results = await Promise.all(urls.map((url) => post<T>(url, route, body, opts)));
  for (const r of results) {
    console.log(`[connector-http] POST ${describeResult(r).slice(0, 200)}`);
  }
  return split(results);
}

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

/**
 * Fan out and re-submit to every party that answered with a transient code until
 * none does or `timeoutMs` elapses. A freshly created ciphertext may not be
 * fetchable by the kms-worker yet, and HTTP-sourced requests are never retried
 * inside the connector, so the client has to. Terminal codes (`acl_denied`, ...)
 * and `200`s are returned as is.
 */
export async function fanOutUntilSettled<T>(
  route: string,
  body: unknown,
  opts?: PostOptions & { timeoutMs?: number; intervalMs?: number; urls?: string[] },
): Promise<FanOutResult<T>> {
  const urls = opts?.urls ?? endpointUrls();
  const deadline = Date.now() + (opts?.timeoutMs ?? 180_000);
  const settled = new Map<string, PostResult<T>>();
  let pending = urls;
  for (;;) {
    const round = await fanOut<T>(route, body, pending, opts);
    for (const r of round.results) settled.set(r.url, r);
    pending = round.results.filter(isTransient).map((r) => r.url);
    if (pending.length === 0 || Date.now() >= deadline) break;
    await sleep(opts?.intervalMs ?? 3_000);
  }
  return split(urls.map((url) => settled.get(url)!));
}

////////////////////////////////////////////////////////////////////////////////
// Request builders
////////////////////////////////////////////////////////////////////////////////

/** Empty v0 `extraData`. */
export const PUBLIC_EXTRA_DATA = '0x';
/** Legacy v0 `extraData`. */
export const USER_EXTRA_DATA = '0x00';

export const buildPublicRequest = (ctHandles: string[], extraData = PUBLIC_EXTRA_DATA): PublicDecryptionRequest => ({
  ctHandles: ctHandles.map((h) => hexlify(getBytes(h))),
  extraData,
});

export interface UserRequestInput {
  readonly handles: HandleEntry[];
  readonly userAddress: string;
  readonly publicKey: string;
  readonly allowedContracts?: string[];
  readonly startTimestamp?: number;
  readonly durationSeconds?: number;
  readonly extraData?: string;
  readonly decryptionContractAddress: string;
}

/**
 * Build a `POST /v1/user-decrypt` body signed per `mode` (defaults to `signer`
 * signing for itself). The signed struct is the unified RFC 016 permit.
 */
export async function buildUserRequest(
  input: UserRequestInput,
  signer: Signer,
  mode?: SignMode,
): Promise<UserDecryptionRequest> {
  const unified: UnifiedDecryptRequest = {
    handles: input.handles.map((h) => ({
      ctHandle: h.handle,
      contractAddress: h.contractAddress,
      ownerAddress: h.ownerAddress,
    })),
    userAddress: input.userAddress,
    allowedContracts: input.allowedContracts ?? [],
    publicKey: input.publicKey,
    startTimestamp: input.startTimestamp ?? backdatedStartTimestamp(),
    durationSeconds: input.durationSeconds ?? 7 * 24 * 60 * 60,
    extraData: input.extraData ?? USER_EXTRA_DATA,
  };
  const signature = await signRequest(
    { relayerUrl: '', decryptionContractAddress: input.decryptionContractAddress },
    unified,
    mode ?? { kind: 'eoa', signer },
  );
  return {
    handles: input.handles,
    userAddress: unified.userAddress,
    publicKey: unified.publicKey,
    allowedContracts: [...unified.allowedContracts],
    requestValidity: { startTimestamp: unified.startTimestamp, durationSeconds: unified.durationSeconds },
    signature,
    extraData: unified.extraData ?? USER_EXTRA_DATA,
  };
}

////////////////////////////////////////////////////////////////////////////////
// Handle helpers
////////////////////////////////////////////////////////////////////////////////

/** FHE type ids at handle byte 30 (tfhe `FheTypes`) that decode to something other than a uint. */
export const FHE_TYPE = {
  ebool: 0,
  eaddress: 7,
} as const;

export const fheTypeOf = (handle: string): number => getBytes(handle)[30];
