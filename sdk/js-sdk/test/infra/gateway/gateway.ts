// Framework-agnostic same-origin test gateway (Option B).
//
// The browser only ever talks to the page origin; this gateway proxies, per
// "slot" (one per chain/WASM version), both:
//   - `rpc`     -> an anvil RPC endpoint
//   - `relayer` -> a mini-relayer serving the keys for that version
//
// It is pure: it maps a normalized request to a normalized response and touches
// no Node/Web server API directly. Thin adapters (see nodeAdapter.ts and the Next
// route) bridge it to each platform, so every environment mounts the same core.
//
// URL layout (mountPrefix defaults to `/__gw`):
//   {origin}/__gw/<slot>/rpc                   -> anvil JSON-RPC (proxied)
//   {origin}/__gw/<slot>/relayer/v2/keyurl     -> keyurl JSON (byte URLs same-origin)
//   {origin}/__gw/<slot>/relayer/key/pub       -> raw public-key bytes
//   {origin}/__gw/<slot>/relayer/key/crs       -> raw CRS bytes
//
// The SDK is configured with relayerUrl = `{origin}/__gw/<slot>/relayer`.

import { buildKeyUrlResponse, loadKeyFile } from './keyRelayer.js';
import { readWasmAsset } from './assets.js';

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

function _text(status: number, message: string): GatewayResponse {
  return { status, headers: { 'content-type': 'text/plain', ...NO_STORE }, body: message };
}
