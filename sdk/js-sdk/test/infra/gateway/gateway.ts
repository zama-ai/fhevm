/**
 * Framework-agnostic same-origin test gateway (Option B).
 *
 * The browser only ever talks to the page origin; this gateway proxies, per
 * "slot" (one per chain/WASM version), both:
 *   - `rpc`     -> an anvil RPC endpoint
 *   - `relayer` -> a mini-relayer serving the keys for that version
 *
 * It is pure: it maps a normalized request to a normalized response and touches
 * no Node/Web server API directly. Thin adapters (see nodeAdapter.ts and the Next
 * route) bridge it to each platform, so every environment mounts the same core.
 *
 * URL layout (mountPrefix defaults to `/__gw`):
 *   {origin}/__gw/<slot>/rpc                   -> anvil JSON-RPC (proxied; KMSGeneration
 *                                                 key reads answered locally)
 *   {origin}/__gw/<slot>/relayer/v2/keyurl     -> keyurl JSON (byte URLs same-origin)
 *   {origin}/__gw/<slot>/relayer/key/pub       -> raw public-key bytes
 *   {origin}/__gw/<slot>/relayer/key/crs       -> raw CRS bytes
 *
 * The SDK is configured with relayerUrl = `{origin}/__gw/<slot>/relayer`.
 */

import { buildKeyUrlResponse, loadKeyFile } from './keyRelayer.js';
import { readWasmAsset } from './assets.js';
import { answerKmsGenerationCall, type KmsGenerationMockConfig } from './kmsGenerationMock.js';

/**
 * The per-slot chain config served at `/<slot>/config` so the browser can build a
 * `defineFhevmChain(...)` without hardcoding addresses (they differ per slot once
 * each anvil deploys from a distinct deployer). Addresses only — the page derives
 * `relayerUrl`/RPC from its own origin + the slot id.
 */
export type SlotChainConfig = {
  readonly chainId: number;
  readonly contracts: {
    readonly acl: string;
    readonly inputVerifier: string;
    readonly kmsVerifier: string;
    readonly protocolConfig: string;
    readonly kmsGeneration: string;
  };
  readonly gateway: {
    readonly id: number;
    readonly contracts: {
      readonly decryption: string;
      readonly inputVerification: string;
    };
  };
};

export type GatewaySlot = {
  /** Path to a `test/keys/key.<ver>.json` file served by this slot's relayer. */
  readonly keyFilePath: string;
  /** Anvil RPC URL proxied at `/<slot>/rpc` (omit for relayer-only slots). */
  readonly rpcUrl?: string | undefined;
  /** Chain config served at `/<slot>/config` (omit for relayer-only slots). */
  readonly chainConfig?: SlotChainConfig | undefined;
  /**
   * KMSGeneration key reads the `/<slot>/rpc` proxy answers itself, with the
   * slot key's digests (see kmsGenerationMock.ts). Omit to proxy every call.
   */
  readonly kmsGeneration?: KmsGenerationMockConfig | undefined;
};

export type GatewayConfig = {
  /** Path prefix the gateway owns, e.g. `/__gw`. No trailing slash. */
  readonly mountPrefix: string;
  /** Slot id -> slot config. */
  readonly slots: Readonly<Record<string, GatewaySlot>>;
  /**
   * SDK wasm dir from which raw assets are served at `/asset/<filename>` (for the
   * URL-based wasm-load modes). Omit to disable the asset route.
   */
  readonly assetDir?: string | undefined;
};

export type GatewayRequest = {
  readonly method: string;
  readonly pathname: string;
  readonly search: string;
  readonly body?: Uint8Array | undefined;
  /** Absolute origin the client sees, e.g. `http://localhost:3334`. */
  readonly publicOrigin: string;
};

export type GatewayResponse = {
  readonly status: number;
  readonly headers: Readonly<Record<string, string>>;
  readonly body: Uint8Array | string;
};

const NO_STORE = { 'cache-control': 'no-store' } as const;

/**
 * Handles a gateway request. Returns `undefined` when the path is outside
 * `mountPrefix`, so callers can fall through to other middleware.
 */
export async function handleGateway(config: GatewayConfig, req: GatewayRequest): Promise<GatewayResponse | undefined> {
  const prefix = `${config.mountPrefix}/`;
  if (!req.pathname.startsWith(prefix)) {
    return undefined;
  }

  const rest = req.pathname.slice(prefix.length);
  const firstSlash = rest.indexOf('/');
  const slotId = firstSlash === -1 ? rest : rest.slice(0, firstSlash);
  const subPath = firstSlash === -1 ? '' : rest.slice(firstSlash + 1);

  // `/asset/<filename>` is not a slot — serve the SDK's raw wasm/worker bytes
  // same-origin for the URL-based wasm-load modes.
  if (slotId === 'asset') {
    if (config.assetDir === undefined) {
      return _text(404, 'Gateway has no assetDir configured.');
    }
    const asset = readWasmAsset(config.assetDir, subPath);
    if (asset === undefined) {
      return _text(404, `Unknown asset '${subPath}'.`);
    }
    // A worker loaded directly from a URL (trusted/precheck-direct-url) is only
    // cross-origin-isolated — and thus only gets SharedArrayBuffer for MT — if its
    // own script response carries COEP. (Blob workers inherit isolation, so they
    // don't need this.) CORP lets the isolated page embed it. Harmless on the wasm.
    return {
      status: 200,
      headers: {
        'content-type': asset.contentType,
        'cross-origin-embedder-policy': 'require-corp',
        'cross-origin-resource-policy': 'same-origin',
        ...NO_STORE,
      },
      body: asset.bytes,
    };
  }

  const slot = config.slots[slotId];
  if (slot === undefined) {
    return _text(404, `Unknown gateway slot '${slotId}'.`);
  }

  const slotBase = `${req.publicOrigin}${config.mountPrefix}/${slotId}`;

  if (subPath === 'relayer/v2/keyurl') {
    const key = loadKeyFile(slot.keyFilePath);
    const json = buildKeyUrlResponse(key, `${slotBase}/relayer/key/pub`, `${slotBase}/relayer/key/crs`);
    return {
      status: 200,
      headers: { 'content-type': 'application/json', ...NO_STORE },
      body: JSON.stringify(json),
    };
  }

  if (subPath === 'relayer/key/pub' || subPath === 'relayer/key/crs') {
    const key = loadKeyFile(slot.keyFilePath);
    const bytes = subPath.endsWith('pub') ? key.pub.bytes : key.crs.bytes;
    return {
      status: 200,
      headers: { 'content-type': 'application/octet-stream', ...NO_STORE },
      body: bytes,
    };
  }

  if (subPath === 'config') {
    if (slot.chainConfig === undefined) {
      return _text(404, `Gateway slot '${slotId}' has no chain config.`);
    }
    return {
      status: 200,
      headers: { 'content-type': 'application/json', ...NO_STORE },
      body: JSON.stringify(slot.chainConfig),
    };
  }

  if (subPath === 'rpc') {
    return _proxyRpc(slot, req);
  }

  return _text(404, `Unknown gateway path '${subPath}'.`);
}

async function _proxyRpc(slot: GatewaySlot, req: GatewayRequest): Promise<GatewayResponse> {
  if (slot.rpcUrl === undefined) {
    return _text(502, `Gateway slot has no rpcUrl.`);
  }

  const answered = await _answerKmsGenerationCalls(slot, req);
  if (answered !== undefined) {
    return answered;
  }

  const init: RequestInit = {
    method: req.method,
    headers: { 'content-type': 'application/json' },
    // Bounded so a wedged anvil can't hang a gateway request indefinitely.
    signal: AbortSignal.timeout(30_000),
  };
  if (req.method !== 'GET' && req.method !== 'HEAD' && req.body !== undefined) {
    // A Uint8Array is a valid BodyInit at runtime; cast around TS 5.7's
    // ArrayBufferLike width and exactOptionalPropertyTypes.
    init.body = req.body as unknown as BodyInit;
  }

  const upstream = await fetch(`${slot.rpcUrl}${req.search}`, init);

  const body = new Uint8Array(await upstream.arrayBuffer());
  return {
    status: upstream.status,
    headers: { 'content-type': upstream.headers.get('content-type') ?? 'application/json', ...NO_STORE },
    body,
  };
}

/**
 * Answers the slot's KMSGeneration key reads (single or batched JSON-RPC); in a
 * batch, the other calls go to anvil and the answers are merged back in request
 * order. `undefined` when the request has no such read: proxy it as is.
 *
 * Forwarded calls carry a synthetic id (their position in the batch), so answers
 * map back exactly even when client ids are duplicated or missing, or anvil
 * reorders them; each answer then gets the client's original id back.
 * Notifications (no `id`) get no response, as JSON-RPC requires.
 */
async function _answerKmsGenerationCalls(slot: GatewaySlot, req: GatewayRequest): Promise<GatewayResponse | undefined> {
  const mock = slot.kmsGeneration;
  if (mock === undefined || req.method !== 'POST' || req.body === undefined) {
    return undefined;
  }
  let parsed: unknown;
  try {
    parsed = JSON.parse(new TextDecoder().decode(req.body));
  } catch {
    return undefined;
  }

  const isBatch = Array.isArray(parsed);
  const calls: readonly unknown[] = Array.isArray(parsed) ? parsed : [parsed];
  const local = calls.map((c) => answerKmsGenerationCall(mock, c));
  if (local.every((r) => r === undefined)) {
    return undefined;
  }

  const responses: unknown[] = [...local];
  const forwarded: number[] = [];
  for (let i = 0; i < calls.length; i++) {
    if (local[i] !== undefined) {
      continue;
    }
    if (_isJsonObject(calls[i])) {
      forwarded.push(i);
    } else {
      responses[i] = _rpcError(null, -32600, 'Invalid Request');
    }
  }

  if (forwarded.length > 0) {
    let upstream: Response;
    try {
      upstream = await fetch(`${slot.rpcUrl!}${req.search}`, {
        method: 'POST',
        headers: { 'content-type': 'application/json' },
        body: JSON.stringify(forwarded.map((i) => ({ ...(calls[i] as object), id: i }))),
        signal: AbortSignal.timeout(30_000),
      });
    } catch (e) {
      return _text(502, `Gateway RPC upstream request failed: ${e instanceof Error ? e.message : String(e)}`);
    }
    // Anything but a 2xx JSON array (an HTTP error, an error page, a top-level
    // JSON-RPC error object) is passed through as is, like the plain proxy path.
    const bytes = new Uint8Array(await upstream.arrayBuffer());
    let answers: unknown;
    try {
      answers = JSON.parse(new TextDecoder().decode(bytes));
    } catch {
      answers = undefined;
    }
    if (!upstream.ok || !Array.isArray(answers)) {
      return {
        status: upstream.ok ? 502 : upstream.status,
        headers: { 'content-type': upstream.headers.get('content-type') ?? 'application/json', ...NO_STORE },
        body: bytes,
      };
    }
    const answerByIndex = new Map<unknown, Record<string, unknown>>();
    for (const a of answers as readonly unknown[]) {
      if (_isJsonObject(a)) {
        answerByIndex.set(a.id, a);
      }
    }
    for (const i of forwarded) {
      const id = (calls[i] as Record<string, unknown>).id ?? null;
      const answer = answerByIndex.get(i);
      responses[i] =
        answer === undefined ? _rpcError(id, -32603, 'No upstream answer for this call') : { ...answer, id };
    }
  }

  // Notifications get no response; a batch of only notifications gets no body.
  const out = responses.filter((_, i) => !_isJsonObject(calls[i]) || 'id' in calls[i]);
  if (out.length === 0) {
    return { status: 204, headers: { ...NO_STORE }, body: '' };
  }
  return {
    status: 200,
    headers: { 'content-type': 'application/json', ...NO_STORE },
    body: JSON.stringify(isBatch ? out : out[0]),
  };
}

function _isJsonObject(value: unknown): value is Record<string, unknown> {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function _rpcError(id: unknown, code: number, message: string): unknown {
  return { jsonrpc: '2.0', id, error: { code, message } };
}

function _text(status: number, message: string): GatewayResponse {
  return { status, headers: { 'content-type': 'text/plain', ...NO_STORE }, body: message };
}
