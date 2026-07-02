// Self-contained client for the RFC-016 UNIFIED user-decryption path
// (`POST /v3/user-decrypt`, envelope `eip712-unified-user-decrypt-v1`) with the
// RFC-012 ERC-1271 signing modes.
//
// Why this exists: the public `@fhevm/sdk` wrapper used elsewhere in the e2e
// suite only drives the legacy user-decrypt path — it forces `userAddress ==
// signer`, cannot send `allowedContracts`/`durationSeconds`, and posts a V1
// payload to `/v2/user-decrypt`. None of the RFC-012/016 behavior is reachable
// through it. This helper builds and signs the unified EIP-712 request itself
// (with a distinct `userAddress` for smart accounts, `allowedContracts`, and
// per-handle `ownerAddress`), posts the v3 envelope, and reports the outcome.
//
// Assertion model (see the suites under test/erc1271UserDecryption,
// test/unifiedUserDecryption, test/decryptionSignatureInvalidation):
//   - Signature verification (RFC-012) is run SYNCHRONOUSLY by the relayer's
//     pre-check (the shared `verify_signature`: ecrecover -> ERC-1271 fallback).
//     A definitively-bad signature yields `POST 400 invalid_signature`; a valid
//     one yields `POST 202 queued`. This is the authoritative, deterministic
//     signal for the ERC-1271 fallback logic.
//   - Per-handle ACL / ownership / `allowedContracts` / signature-invalidation
//     (RFC-016) are authoritative only in the KMS Connector, which processes the
//     request asynchronously. A request that passes those checks drives the job
//     to `succeeded` (KMS produced re-encrypted shares — which only happens once
//     every check passed). A request that fails them never succeeds (the relayer
//     eventually times it out to `failed`).
//
// The helper intentionally does NOT reconstruct the plaintext from the returned
// shares: that requires the js-sdk's internal `createKmsSigncryptedShares`
// (reads the on-chain KMS signers context and needs a full client context),
// which is not part of the public surface. Reaching `succeeded` already proves
// the full authorization pipeline accepted and processed the request, which is
// exactly what RFC-012/016 govern.

import { TypedDataEncoder } from 'ethers';
import type { Signer, TypedDataDomain } from 'ethers';

/**
 * EIP-712 type list for the unified `UserDecryptRequestVerification` struct.
 *
 * The field order is authoritative — it determines the EIP-712 type hash and
 * must match the Solidity struct, the KMS Connector, the relayer, and the
 * js-sdk `kmsUserDecryptEip712V2Types`. Do not reorder.
 */
export const UNIFIED_USER_DECRYPT_TYPES: Record<string, Array<{ name: string; type: string }>> = {
  UserDecryptRequestVerification: [
    { name: 'userAddress', type: 'address' },
    { name: 'publicKey', type: 'bytes' },
    { name: 'allowedContracts', type: 'address[]' },
    { name: 'startTimestamp', type: 'uint256' },
    { name: 'durationSeconds', type: 'uint256' },
    { name: 'extraData', type: 'bytes' },
  ],
};

/** The only attestation type the relayer's `/v3/user-decrypt` endpoint accepts. */
export const UNIFIED_ATTESTATION_TYPE = 'eip712-unified-user-decrypt-v1';

/** EIP-712 domain `name`/`version` for the Gateway `Decryption` contract. */
const DOMAIN_NAME = 'Decryption';
const DOMAIN_VERSION = '1';

/** Default `extraData` (version byte `0x00`, no context id). */
export const DEFAULT_EXTRA_DATA = '0x00';

/** ERC-1271 magic value returned by a compliant wallet for a valid signature. */
export const ERC1271_MAGIC_VALUE = '0x1626ba7e';

export interface UnifiedConfig {
  /** Relayer base URL (any trailing `/vN` is stripped). */
  readonly relayerUrl: string;
  /** Gateway `Decryption` contract address — the EIP-712 verifying contract. */
  readonly decryptionContractAddress: string;
  /**
   * Optional `x-api-key` value for auth-fronted relayer deployments (Kong).
   * Empty/undefined on local stacks — no header is sent.
   */
  readonly apiKey?: string;
}

export interface UnifiedHandleEntry {
  /** Ciphertext handle, `0x` + 64 hex chars. */
  readonly ctHandle: string;
  /** Contract with `isAllowed(handle, contractAddress)` (delegated-path arg). */
  readonly contractAddress: string;
  /** Delegator address (equals `userAddress` for direct access). */
  readonly ownerAddress: string;
}

export interface UnifiedDecryptRequest {
  readonly handles: readonly UnifiedHandleEntry[];
  /** Identity asserting authorization (EOA or smart-wallet address). */
  readonly userAddress: string;
  /** Empty = permissive mode; non-empty = at least one must be allowed per handle. */
  readonly allowedContracts: readonly string[];
  /** Re-encryption target public key, `0x` + hex (from `instance.generateKeypair()`). */
  readonly publicKey: string;
  /** Unix seconds; SDKs use ~now. */
  readonly startTimestamp: number;
  /** Validity window length in seconds. */
  readonly durationSeconds: number;
  /** Defaults to `0x00`. */
  readonly extraData?: string;
}

/**
 * How the request signature is produced:
 *  - `eoa`: `signer` signs; `signer.address` must equal `userAddress` (EOA fast path).
 *  - `erc1271`: `ownerSigner` (an owner key) signs; `userAddress` is the smart-wallet
 *     address, so the KMS/relayer verify via the wallet's `isValidSignature`.
 *  - `empty`: no signature (`0x`) — the Safe `approveHash` / `signedMessages` flow.
 */
export type SignMode =
  | { readonly kind: 'eoa'; readonly signer: Signer }
  | { readonly kind: 'erc1271'; readonly ownerSigner: Signer }
  | { readonly kind: 'empty' };

export interface PostResult {
  readonly httpStatus: number;
  /** Relayer status string, lowercased (`queued` on accept). */
  readonly status?: string;
  readonly jobId?: string;
  readonly errorLabel?: string;
  readonly errorMessage?: string;
  readonly raw: unknown;
}

export type JobStatus = 'succeeded' | 'failed' | 'pending';

export interface PollResult {
  readonly status: JobStatus;
  readonly httpStatus: number;
  readonly shares?: unknown[];
  readonly errorLabel?: string;
  readonly raw: unknown;
}

export interface RequestOptions {
  /** Poll GET until the job reaches a terminal state (`succeeded`/`failed`) or timeout. */
  readonly waitForTerminal?: boolean;
  readonly timeoutMs?: number;
  readonly intervalMs?: number;
}

export interface RequestOutcome {
  readonly post: PostResult;
  /** Present only when the POST was accepted (202) and polling was requested. */
  readonly poll?: PollResult;
  /** The EIP-712 digest that was signed (used e.g. for Safe `approveHash`). */
  readonly digest: string;
}

////////////////////////////////////////////////////////////////////////////////
// Internals
////////////////////////////////////////////////////////////////////////////////

const sleep = (ms: number) => new Promise((resolve) => setTimeout(resolve, ms));

const ensure0x = (value: string): string => (value.startsWith('0x') ? value : `0x${value}`);

/** Strip a trailing `/v1`, `/v2`, or `/v3` (and any trailing slash) from the relayer URL. */
const relayerBaseUrl = (url: string): string => url.replace(/\/(v[0-9]+)\/?$/, '').replace(/\/$/, '');

/**
 * Extract the host chain id from a ciphertext handle. The FHEVM handle encodes
 * the chain id where it was created in big-endian bytes [22, 30) — the same
 * slice the relayer's signature pre-check reads. Deriving the EIP-712 domain
 * `chainId` from the handle (rather than trusting a config value) guarantees the
 * digest matches what the relayer recomputes.
 */
export function chainIdFromHandle(ctHandle: string): number {
  const hex = ensure0x(ctHandle).slice(2);
  if (hex.length !== 64) {
    throw new Error(`Invalid ciphertext handle length: expected 32 bytes, got ${hex.length / 2}`);
  }
  // bytes [22, 30) -> hex chars [44, 60)
  const chainId = BigInt(`0x${hex.slice(44, 60)}`);
  if (chainId > BigInt(Number.MAX_SAFE_INTEGER)) {
    throw new Error(`Chain id ${chainId} from handle exceeds Number.MAX_SAFE_INTEGER`);
  }
  return Number(chainId);
}

/**
 * A `startTimestamp` for new requests: wall-clock now minus a skew margin.
 *
 * The relayer rejects strictly-future `startTimestamp`s against ITS wall clock
 * (`validate_timestamp`: "Timestamp must not be in the future"), and the
 * gateway contract checks `startTimestamp <= block.timestamp` on the gateway
 * chain. Backdating by a small margin absorbs clock skew in both directions
 * without materially shortening the (days-long) validity windows used in tests.
 */
export const backdatedStartTimestamp = (marginSeconds = 60): number =>
  Math.floor(Date.now() / 1000) - marginSeconds;

function domainOf(cfg: UnifiedConfig, chainId: number): TypedDataDomain {
  return {
    name: DOMAIN_NAME,
    version: DOMAIN_VERSION,
    chainId,
    verifyingContract: cfg.decryptionContractAddress,
  };
}

function messageOf(req: UnifiedDecryptRequest): Record<string, unknown> {
  return {
    userAddress: req.userAddress,
    publicKey: ensure0x(req.publicKey),
    allowedContracts: [...req.allowedContracts],
    startTimestamp: req.startTimestamp,
    durationSeconds: req.durationSeconds,
    extraData: req.extraData ?? DEFAULT_EXTRA_DATA,
  };
}

/**
 * Chain id of the request, derived from its first handle. Fails with a clear
 * error on an empty `handles` array instead of an opaque TypeError — the
 * relayer requires at least one handle anyway.
 */
function requestChainId(req: UnifiedDecryptRequest): number {
  const first = req.handles[0];
  if (!first) {
    throw new Error('UnifiedDecryptRequest.handles must be non-empty');
  }
  return chainIdFromHandle(first.ctHandle);
}

/**
 * Compute the EIP-712 digest that both `ecrecover` and ERC-1271
 * `isValidSignature` receive. Exposed so Safe-style mocks can pre-approve it via
 * `approveHash(digest)` before an empty-signature request.
 */
export function computeUnifiedDigest(cfg: UnifiedConfig, req: UnifiedDecryptRequest): string {
  const chainId = requestChainId(req);
  return TypedDataEncoder.hash(domainOf(cfg, chainId), UNIFIED_USER_DECRYPT_TYPES, messageOf(req));
}

async function signRequest(cfg: UnifiedConfig, req: UnifiedDecryptRequest, mode: SignMode): Promise<string> {
  if (mode.kind === 'empty') {
    return '0x';
  }
  if (mode.kind === 'eoa') {
    // 'eoa' means "the user signs for themselves" — a mismatched signer would
    // only surface later as a hard-to-diagnose relayer "Signature is invalid".
    // Signing for a DIFFERENT userAddress is what kind 'erc1271' is for.
    const signerAddress = (await mode.signer.getAddress()).toLowerCase();
    if (signerAddress !== req.userAddress.toLowerCase()) {
      throw new Error(
        `SignMode 'eoa' requires the signer address to equal req.userAddress ` +
          `(got ${signerAddress} vs ${req.userAddress.toLowerCase()}); use kind 'erc1271' to sign for a different userAddress`,
      );
    }
  }
  const signer = mode.kind === 'eoa' ? mode.signer : mode.ownerSigner;
  const chainId = requestChainId(req);
  return signer.signTypedData(domainOf(cfg, chainId), UNIFIED_USER_DECRYPT_TYPES, messageOf(req));
}

function buildEnvelope(req: UnifiedDecryptRequest, signature: string): unknown {
  return {
    attestationType: UNIFIED_ATTESTATION_TYPE,
    attestedPayload: {
      version: '2.0',
      type: 'user_decryption',
      handles: req.handles.map((h) => ({
        ctHandle: ensure0x(h.ctHandle),
        contractAddress: h.contractAddress,
        ownerAddress: h.ownerAddress,
      })),
      userAddress: req.userAddress,
      allowedContracts: [...req.allowedContracts],
      requestValidity: {
        startTimestamp: String(req.startTimestamp),
        durationSeconds: String(req.durationSeconds),
      },
      publicKey: ensure0x(req.publicKey),
      extraData: req.extraData ?? DEFAULT_EXTRA_DATA,
    },
    signature,
  };
}

function httpHeaders(cfg: UnifiedConfig, withJsonBody: boolean): Record<string, string> {
  const headers: Record<string, string> = {};
  if (withJsonBody) {
    headers['content-type'] = 'application/json';
  }
  if (cfg.apiKey) {
    // Same header the js-sdk's `ApiKeyHeader` auth mode uses by default.
    headers['x-api-key'] = cfg.apiKey;
  }
  return headers;
}

async function readJson(resp: Response): Promise<Record<string, unknown>> {
  try {
    return (await resp.json()) as Record<string, unknown>;
  } catch {
    return {};
  }
}

function extractError(body: Record<string, unknown>): { label?: string; message?: string } {
  const err = body.error as { label?: string; message?: string } | undefined;
  return {
    label: err?.label ?? (body.label as string | undefined),
    message: err?.message ?? (body.message as string | undefined),
  };
}

/** POST the unified envelope. A `202` (with a `jobId`) is an accept; anything else is a rejection. */
export async function submitUnifiedRequest(
  cfg: UnifiedConfig,
  req: UnifiedDecryptRequest,
  mode: SignMode,
): Promise<{ post: PostResult; digest: string }> {
  const digest = computeUnifiedDigest(cfg, req);
  const signature = await signRequest(cfg, req, mode);
  const envelope = buildEnvelope(req, signature);

  const url = `${relayerBaseUrl(cfg.relayerUrl)}/v3/user-decrypt`;
  const resp = await fetch(url, {
    method: 'POST',
    headers: httpHeaders(cfg, true),
    body: JSON.stringify(envelope),
  });
  const body = await readJson(resp);
  const err = extractError(body);

  return {
    digest,
    post: {
      httpStatus: resp.status,
      status: typeof body.status === 'string' ? body.status.toLowerCase() : undefined,
      jobId: (body.result as { jobId?: string } | undefined)?.jobId,
      errorLabel: err.label,
      errorMessage: err.message,
      raw: body,
    },
  };
}

async function pollOnce(cfg: UnifiedConfig, jobId: string): Promise<PollResult> {
  const url = `${relayerBaseUrl(cfg.relayerUrl)}/v3/user-decrypt/${jobId}`;
  const resp = await fetch(url, { headers: httpHeaders(cfg, false) });
  const body = await readJson(resp);
  const status = typeof body.status === 'string' ? body.status.toLowerCase() : '';
  if (status === 'succeeded') {
    const result = (body.result as { result?: unknown[] } | undefined)?.result;
    return { status: 'succeeded', httpStatus: resp.status, shares: result, raw: body };
  }
  if (status === 'failed') {
    return { status: 'failed', httpStatus: resp.status, errorLabel: extractError(body).label, raw: body };
  }
  return { status: 'pending', httpStatus: resp.status, raw: body };
}

/**
 * Poll GET until the job is terminal (`succeeded`/`failed`) or the timeout
 * elapses. On timeout the last observed status (typically `pending`) is
 * returned — for a correctly-rejected async request this is the expected
 * non-`succeeded` outcome (the relayer only marks it `failed` after its own
 * ~300s user-decrypt timeout).
 */
export async function pollJob(
  cfg: UnifiedConfig,
  jobId: string,
  opts?: { timeoutMs?: number; intervalMs?: number },
): Promise<PollResult> {
  const timeoutMs = opts?.timeoutMs ?? 120_000;
  const intervalMs = opts?.intervalMs ?? 2_000;
  const deadline = Date.now() + timeoutMs;
  let last: PollResult = { status: 'pending', httpStatus: 0, raw: {} };
  while (Date.now() < deadline) {
    last = await pollOnce(cfg, jobId);
    if (last.status !== 'pending') {
      return last;
    }
    await sleep(intervalMs);
  }
  return last;
}

/**
 * End-to-end convenience: build + sign + submit, then optionally poll.
 * Returns the POST result, the (optional) poll result, and the signed digest.
 */
export async function requestUnifiedUserDecrypt(
  cfg: UnifiedConfig,
  req: UnifiedDecryptRequest,
  mode: SignMode,
  opts?: RequestOptions,
): Promise<RequestOutcome> {
  const { post, digest } = await submitUnifiedRequest(cfg, req, mode);
  if (opts?.waitForTerminal && post.httpStatus === 202 && !post.jobId) {
    // A 202 without a jobId is a malformed relayer response. Returning it as a
    // non-polled outcome would leave `poll` undefined, letting negative tests
    // ("never succeeded") pass vacuously — fail fast instead.
    throw new Error(`Relayer accepted request (202) but did not return a jobId: ${JSON.stringify(post.raw)}`);
  }
  if (post.httpStatus !== 202 || !post.jobId || !opts?.waitForTerminal) {
    return { post, digest };
  }
  const poll = await pollJob(cfg, post.jobId, { timeoutMs: opts.timeoutMs, intervalMs: opts.intervalMs });
  return { post, poll, digest };
}

/**
 * True iff the POST was rejected specifically because SIGNATURE VERIFICATION
 * failed — the synchronous RFC-012 pre-check (`ecrecover` -> ERC-1271
 * fallback). The relayer surfaces exactly `400` + `error.details` containing
 * `{field: "signature", issue: "Signature is invalid"}`
 * (`V2ErrorResponseBody::invalid_signature`). Matching the issue text as well
 * as the field name distinguishes the semantic rejection from an envelope
 * validation failure on the `signature` field (e.g. malformed hex), which
 * shares the same label/field but carries a different issue string.
 */
export function isSignatureRejection(post: PostResult): boolean {
  if (post.httpStatus !== 400) {
    return false;
  }
  const err = (post.raw as { error?: { details?: Array<{ field?: string; issue?: string }> } }).error;
  return (err?.details ?? []).some(
    (d) =>
      d.field === 'signature' &&
      typeof d.issue === 'string' &&
      d.issue.toLowerCase().includes('signature is invalid'),
  );
}

/** Build a direct-access handle entry (`ownerAddress == userAddress`). */
export function directHandle(ctHandle: string, contractAddress: string, userAddress: string): UnifiedHandleEntry {
  return { ctHandle, contractAddress, ownerAddress: userAddress };
}

/** Build a delegated handle entry (`ownerAddress` is the delegator). */
export function delegatedHandle(
  ctHandle: string,
  contractAddress: string,
  ownerAddress: string,
): UnifiedHandleEntry {
  return { ctHandle, contractAddress, ownerAddress };
}
