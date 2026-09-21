// operator — the demo's one privileged HTTP service: the keeper's keys, the host listener's leaf-proof token and
// the mock-USDC faucet live here and nowhere in the browser or the dev server.
//
// Routes (JSON in, JSON out; every route but /health carries the boot authorization):
//   GET  /health                        -> { ok: true }, public
//   GET  /demo-config                   -> { config } the page boots from (no key material)
//   GET  /demo-encryption-key-meta      -> { fingerprint } of the relayer's current FHE public key + CRS
//   GET  /demo-encryption-key           -> the key material itself, cached per fingerprint
//   GET  /demo-vault-metrics            -> the vault's total assets and shares
//   POST /demo-batch     { direction }  -> the batch users may join next (opens one if needed)
//   POST /demo-operator  OperatorRequest-> dispatch / settle / claim a batch as the keeper
//   POST /demo-harvest   {}             -> one year of illustrative yield donated to the vault
//   POST /demo-faucet/airdrop-sol { address, sol? }     -> SOL through the environment's funder
//   POST /demo-faucet/mint-usdc   { address, amount? }  -> mock USDC to the recipient's ATA
//
// Authorization is the lifecycle's boot capability (`authorizeDemoHeaders`): the dapp dev server
// adds it to every `/api` call it proxies, the smoke and browser-reality checks carry it directly.
// A direct caller on the tailnet (a teammate's script or curl through `tailscale serve`, which
// sets the identity header and strips a client's own) is accepted for an allow-listed login; the
// page itself never calls the operator cross-origin, it goes through its own origin's proxy. CORS
// is exact-origin for the one dapp origin. Concurrency: one keeper action per batch at a time,
// batch preparation serialized per registry, one harvest at a time.
//
// `createOperator` is transport-free (a `Request -> Response` function over injected actions) so
// it is unit-tested with stubs; `operator-server.ts` binds it to the live stack.

import { address, type Address } from "@solana/kit";

import type { PreparedBatch } from "@demo-dapp/batchProvisioning";
import type { VaultDirection, VaultMetrics } from "@demo-dapp/batchTypes";
import { encodeBatchTarget, encodeVaultMetrics, parseOperatorRequest, type OperatorRequest } from "@demo-dapp/demoApi";
import type { UnderlyingMinter } from "@demo-dapp/harvestOperator";
import { authorizeDemoHeaders, type DemoAuthorization } from "./authorization";

const DEFAULT_AIRDROP_SOL = 0.2;
/** Mock USDC has 6 decimals (matches the seeded SPL mint); the default drip is 1,000 USDC. */
const USDC_DECIMALS = 6;
const DEFAULT_USDC_AMOUNT = 1_000;
const MAX_JSON_BODY_BYTES = 8_192;
/** The identity header `tailscale serve` sets on requests it forwards (and strips from clients). */
export const TAILSCALE_LOGIN_HEADER = "tailscale-user-login";

/**
 * Brings `recipient` to at least `sol` SOL and resolves with the confirmed signature, or null when
 * the recipient already held it (`SolanaProvisioningContext.fundSol`).
 */
export type SolFunder = (recipient: Address, sol: number) => Promise<string | null>;

export type DemoEncryptionKey = {
  readonly fingerprint: string;
  readonly publicKeyId: string;
  readonly publicKeyBase64: string;
  readonly crsId: string;
  readonly crsBase64: string;
};

/** What the operator does when a route is called; the server wires these to the live stack. */
export type OperatorActions = {
  readonly fundSol: SolFunder;
  readonly registerBurner?: (bytes: Uint8Array) => Promise<string>;
  /** Mints mock USDC (the vault's underlying) to a recipient's ATA, creating it if needed. */
  readonly mintUsdc: UnderlyingMinter;
  /** The public configuration the page boots from, `relayerUrl` and `demoBootId` already set. */
  readonly readConfig: () => Promise<Record<string, unknown>>;
  readonly encryptionKeyFingerprint: () => Promise<string>;
  readonly encryptionKey: () => Promise<DemoEncryptionKey>;
  readonly vaultMetrics: () => Promise<VaultMetrics>;
  readonly prepareBatch: (direction: VaultDirection) => Promise<PreparedBatch>;
  readonly runOperator: (request: OperatorRequest) => Promise<string | null>;
  readonly harvest: () => Promise<{ readonly before: VaultMetrics; readonly after: VaultMetrics }>;
};

export type OperatorConfig = {
  readonly actions: OperatorActions;
  readonly authorization: DemoAuthorization;
  readonly allowedOrigin: string;
  /** Tailscale logins accepted through the identity header; empty means the boot capability only. */
  readonly tailscaleLogins?: readonly string[];
};

const CORS_HEADERS: Record<string, string> = {
  "access-control-allow-methods": "GET, POST, OPTIONS",
  "access-control-allow-headers": "authorization, content-type, x-fhevm-demo-boot-id",
  vary: "Origin",
};

/** A validation failure of the caller's input: answered with 400 rather than 503. */
export class OperatorRequestError extends Error {}

const invalid = (message: string): never => {
  throw new OperatorRequestError(message);
};

export const parseRecipient = (value: unknown): Address => {
  if (typeof value !== "string") invalid("`address` must be a base58 address string");
  try {
    return address(value as string);
  } catch {
    return invalid(`\`address\` is not a valid Solana address: ${String(value)}`);
  }
};

export const parsePositiveNumber = (value: unknown, name: string, fallback: number): number => {
  if (value === undefined) return fallback;
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) invalid(`\`${name}\` must be a positive number`);
  return value as number;
};

const parseDirection = (value: unknown): VaultDirection =>
  value === "deposit" || value === "redeem" ? value : invalid("invalid vault direction");

/** One promise per key while it runs: concurrent callers share the result, a later call reruns. */
export const runSingleFlight = async <T>(
  operations: Map<string, Promise<T>>,
  key: string,
  start: () => Promise<T>,
): Promise<T> => {
  const existing = operations.get(key);
  if (existing !== undefined) return existing;
  const operation = start();
  operations.set(key, operation);
  try {
    return await operation;
  } finally {
    if (operations.get(key) === operation) operations.delete(key);
  }
};

export type SerialQueue = { tail: Promise<void> };

/** Runs `start` after every earlier operation queued here has settled. */
export const runSerialized = async <T>(queue: SerialQueue, start: () => Promise<T>): Promise<T> => {
  const previous = queue.tail;
  let release!: () => void;
  queue.tail = new Promise<void>((resolve) => {
    release = resolve;
  });
  await previous;
  try {
    return await start();
  } finally {
    release();
  }
};

type Decision = { readonly ok: true } | { readonly ok: false; readonly status: number; readonly error: string };

/**
 * The boot capability, or a Tailscale identity from the allow-list when the request carries no
 * capability at all. A request that presents a capability is judged on it alone, so a stale boot
 * still answers 409 and a wrong token 401.
 */
export const authorizeOperatorRequest = (
  getHeader: (name: string) => string | null | undefined,
  config: Pick<OperatorConfig, "authorization" | "tailscaleLogins">,
): Decision => {
  const decision = authorizeDemoHeaders(getHeader, config.authorization);
  if (decision.ok) return decision;
  const presentedCapability = Boolean(getHeader("authorization") || getHeader("x-fhevm-demo-boot-id"));
  const login = getHeader(TAILSCALE_LOGIN_HEADER);
  if (!presentedCapability && login && (config.tailscaleLogins ?? []).includes(login)) return { ok: true };
  return decision;
};

const readJsonBody = async (request: Request): Promise<Record<string, unknown>> => {
  if (request.headers.get("content-type")?.split(";", 1)[0]?.trim().toLowerCase() !== "application/json") {
    invalid("request body must be application/json");
  }
  const declared = Number(request.headers.get("content-length") ?? "0");
  if (declared > MAX_JSON_BODY_BYTES) invalid("request body is too large");
  const text = await request.text();
  if (text.length > MAX_JSON_BODY_BYTES) invalid("request body is too large");
  let body: unknown;
  try {
    body = text.length === 0 ? {} : JSON.parse(text);
  } catch {
    return invalid("request body must be JSON");
  }
  if (typeof body !== "object" || body === null || Array.isArray(body)) invalid("request body must be a JSON object");
  return body as Record<string, unknown>;
};

/** Builds the operator request handler over injected actions. */
export const createOperator = (config: OperatorConfig): ((request: Request) => Promise<Response>) => {
  const { actions, allowedOrigin } = config;
  const operatorInFlight = new Map<string, Promise<string | null>>();
  const preparationInFlight = new Map<string, Promise<PreparedBatch>>();
  const fundingQueue: SerialQueue = { tail: Promise.resolve() };
  const preparationQueue: SerialQueue = { tail: Promise.resolve() };
  let harvestInFlight: ReturnType<OperatorActions["harvest"]> | undefined;

  const corsHeaders = (request: Request): Record<string, string> =>
    request.headers.get("origin") === allowedOrigin
      ? { ...CORS_HEADERS, "access-control-allow-origin": allowedOrigin }
      : {};
  const json = (request: Request, status: number, body: unknown): Response =>
    new Response(JSON.stringify(body), {
      status,
      headers: {
        "content-type": "application/json",
        "cache-control": "no-store",
        "x-content-type-options": "nosniff",
        ...corsHeaders(request),
      },
    });

  const routes: Record<string, { readonly method: "GET" | "POST"; readonly run: (body: Record<string, unknown>) => Promise<unknown> }> = {
    "/demo-config": { method: "GET", run: async () => ({ config: await actions.readConfig() }) },
    "/demo-encryption-key-meta": {
      method: "GET",
      run: async () => ({ fingerprint: await actions.encryptionKeyFingerprint() }),
    },
    "/demo-encryption-key": { method: "GET", run: () => actions.encryptionKey() },
    "/demo-vault-metrics": { method: "GET", run: async () => encodeVaultMetrics(await actions.vaultMetrics()) },
    "/demo-batch": {
      method: "POST",
      run: async (body) => {
        const direction = parseDirection(body.direction);
        const prepared = await runSingleFlight(preparationInFlight, direction, () =>
          runSerialized(preparationQueue, () => actions.prepareBatch(direction)),
        );
        return encodeBatchTarget(prepared);
      },
    },
    "/demo-operator": {
      method: "POST",
      run: async (body) => {
        let operatorRequest: OperatorRequest;
        try {
          operatorRequest = parseOperatorRequest(body);
        } catch (error) {
          return invalid(error instanceof Error ? error.message : String(error));
        }
        const { action, direction, position } = operatorRequest;
        const claimUser = operatorRequest.action === "claim" ? operatorRequest.user : "";
        const key = `${direction}:${position.batch}:${action}:${claimUser}`;
        const signature = await runSingleFlight(operatorInFlight, key, () => actions.runOperator(operatorRequest));
        return { ok: true, signature };
      },
    },
    "/demo-harvest": {
      method: "POST",
      run: async () => {
        harvestInFlight ??= actions.harvest().finally(() => {
          harvestInFlight = undefined;
        });
        const result = await harvestInFlight;
        return { before: encodeVaultMetrics(result.before), after: encodeVaultMetrics(result.after) };
      },
    },
    "/demo-wallet/recovery": {
      method: "POST",
      run: async (body) => {
        if (!Array.isArray(body.keypair) || body.keypair.length !== 64 || body.keypair.some(byte => !Number.isInteger(byte) || byte < 0 || byte > 255)) invalid("invalid demo wallet");
        if (!actions.registerBurner) invalid("demo wallet recovery is unavailable");
        try { return { address: await actions.registerBurner!(Uint8Array.from(body.keypair as number[])) }; }
        catch { throw new Error("Cannot secure the demo wallet recovery copy; funding is disabled"); }
      },
    },
    "/demo-faucet/airdrop-sol": {
      method: "POST",
      run: async (body) => {
        const recipient = parseRecipient(body.address);
        const sol = parsePositiveNumber(body.sol, "sol", DEFAULT_AIRDROP_SOL);
        if (sol > DEFAULT_AIRDROP_SOL) invalid("SOL target exceeds the 0.2 SOL demo limit");
        const signature = await runSerialized(fundingQueue, () => actions.fundSol(recipient, sol));
        return { signature, address: recipient, sol };
      },
    },
    "/demo-faucet/mint-usdc": {
      method: "POST",
      run: async (body) => {
        const recipient = parseRecipient(body.address);
        const amount = parsePositiveNumber(body.amount, "amount", DEFAULT_USDC_AMOUNT);
        if (amount > DEFAULT_USDC_AMOUNT) invalid("mock USDC request exceeds the 1000 USDC limit");
        const baseUnits = BigInt(Math.round(amount * 10 ** USDC_DECIMALS));
        const signature = await actions.mintUsdc(recipient, baseUnits);
        return { signature, address: recipient, amount, baseUnits: baseUnits.toString() };
      },
    },
  };

  return async (request: Request): Promise<Response> => {
    const origin = request.headers.get("origin");
    const originAllowed = origin === null || origin === allowedOrigin;
    if (request.method === "OPTIONS") {
      return origin === allowedOrigin
        ? new Response(null, { status: 204, headers: corsHeaders(request) })
        : new Response(null, { status: 403 });
    }
    const { pathname } = new URL(request.url);
    if (request.method === "GET" && pathname === "/health") return json(request, 200, { ok: true });

    const route = routes[pathname];
    if (route === undefined) return json(request, 404, { error: `no operator endpoint at ${pathname}` });
    if (request.method !== route.method) {
      return json(request, 405, { error: `method ${request.method} not allowed` });
    }
    // A browser page that is not the demo never gets a usable answer, even with a leaked capability.
    if (!originAllowed) return json(request, 403, { error: "demo operator origin not allowed" });
    const decision = authorizeOperatorRequest((name) => request.headers.get(name), config);
    if (!decision.ok) return json(request, decision.status, { error: decision.error });

    try {
      const body = route.method === "POST" ? await readJsonBody(request) : {};
      return json(request, 200, await route.run(body));
    } catch (error) {
      const message = error instanceof Error ? error.message : String(error);
      return json(request, error instanceof OperatorRequestError ? 400 : 503, { error: message });
    }
  };
};
