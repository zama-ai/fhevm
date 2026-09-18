// faucet — the minimal HTTP faucet the demo dApp's "get funds" button calls (#1760/#1761).
//
// Boot-authorized, exact-origin CORS. Two funding endpoints plus public health:
//   POST /airdrop-sol   { address, sol? }       -> SOL through the injected funder (validator airdrop
//                                                  on localnet, transfer from the deployer on devnet)
//   POST /mint-usdc     { address, amount? }    -> mints mock USDC to the recipient's ATA
//   GET  /health                                 -> { ok: true }
//
// The mint authority is a committed demo keypair. `createFaucet` returns the request handler (pure,
// unit-testable with a stub RPC + minter); `serveFaucet` wraps it in Bun.serve.

import { address, type Address } from "@solana/kit";

import { authorizeDemoHeaders, type DemoAuthorization } from "./authorization";

const DEFAULT_AIRDROP_SOL = 5;
/** Mock USDC has 6 decimals (matches the seeded SPL mint); the default drip is 1,000 USDC. */
const USDC_DECIMALS = 6;
const DEFAULT_USDC_AMOUNT = 1_000n;

/** Mints `baseUnits` of mock USDC to `recipient`'s ATA, creating the ATA if needed. */
export type UsdcMinter = (recipient: Address, baseUnits: bigint) => Promise<string>;

/**
 * Brings `recipient` to at least `sol` SOL and resolves with the confirmed signature, or null when
 * the recipient already held it (`SolanaProvisioningContext.fundSol`).
 */
export type SolFunder = (recipient: Address, sol: number) => Promise<string | null>;

export type FaucetConfig = {
  readonly fundSol: SolFunder;
  readonly mintUsdc: UsdcMinter;
  readonly authorization: DemoAuthorization;
  readonly allowedOrigin: string;
};

const ALLOWED_CORS_HEADERS: Record<string, string> = {
  "access-control-allow-methods": "GET, POST, OPTIONS",
  "access-control-allow-headers": "authorization, content-type, x-fhevm-demo-boot-id",
  vary: "Origin",
};

const corsHeaders = (request: Request, allowedOrigin: string): Record<string, string> =>
  request.headers.get("origin") === allowedOrigin
    ? { ...ALLOWED_CORS_HEADERS, "access-control-allow-origin": allowedOrigin }
    : {};

const json = (request: Request, allowedOrigin: string, status: number, body: unknown): Response =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json", ...corsHeaders(request, allowedOrigin) },
  });

const parseRecipient = (value: unknown): Address => {
  if (typeof value !== "string") throw new Error("`address` must be a base58 address string");
  try {
    return address(value);
  } catch {
    throw new Error(`\`address\` is not a valid Solana address: ${value}`);
  }
};

const parsePositiveNumber = (value: unknown, name: string, fallback: number): number => {
  if (value === undefined) return fallback;
  if (typeof value !== "number" || !Number.isFinite(value) || value <= 0) {
    throw new Error(`\`${name}\` must be a positive number`);
  }
  return value;
};

/** Builds the faucet request handler over an injected RPC + USDC minter. */
export const createFaucet = (config: FaucetConfig): ((request: Request) => Promise<Response>) => {
  return async (request: Request): Promise<Response> => {
    const originAllowed = request.headers.get("origin") === config.allowedOrigin;
    if (request.method === "OPTIONS") {
      return originAllowed
        ? new Response(null, { status: 204, headers: corsHeaders(request, config.allowedOrigin) })
        : new Response(null, { status: 403 });
    }

    const url = new URL(request.url);
    if (request.method === "GET" && url.pathname === "/health") {
      return json(request, config.allowedOrigin, 200, { ok: true });
    }

    if (request.method !== "POST") {
      return json(request, config.allowedOrigin, 405, { error: `method ${request.method} not allowed` });
    }
    if (!originAllowed) {
      return json(request, config.allowedOrigin, 403, { error: "demo faucet origin not allowed" });
    }
    const authorization = authorizeDemoHeaders((name) => request.headers.get(name), config.authorization);
    if (!authorization.ok) {
      return json(request, config.allowedOrigin, authorization.status, { error: authorization.error });
    }

    let body: Record<string, unknown>;
    try {
      body = (await request.json()) as Record<string, unknown>;
    } catch {
      return json(request, config.allowedOrigin, 400, { error: "request body must be JSON" });
    }

    try {
      if (url.pathname === "/airdrop-sol") {
        const recipient = parseRecipient(body.address);
        const sol = parsePositiveNumber(body.sol, "sol", DEFAULT_AIRDROP_SOL);
        const signature = await config.fundSol(recipient, sol);
        return json(request, config.allowedOrigin, 200, { signature, address: recipient, sol });
      }

      if (url.pathname === "/mint-usdc") {
        const recipient = parseRecipient(body.address);
        const amount = parsePositiveNumber(body.amount, "amount", Number(DEFAULT_USDC_AMOUNT));
        const baseUnits = BigInt(Math.round(amount * 10 ** USDC_DECIMALS));
        const signature = await config.mintUsdc(recipient, baseUnits);
        return json(request, config.allowedOrigin, 200, {
          signature,
          address: recipient,
          amount,
          baseUnits: baseUnits.toString(),
        });
      }
    } catch (error) {
      return json(request, config.allowedOrigin, 400, {
        error: error instanceof Error ? error.message : String(error),
      });
    }

    return json(request, config.allowedOrigin, 404, { error: `no faucet endpoint at ${url.pathname}` });
  };
};

export type ServeFaucetOptions = {
  readonly fundSol: SolFunder;
  readonly mintUsdc: UsdcMinter;
  readonly authorization: DemoAuthorization;
  readonly allowedOrigin: string;
  readonly port?: number;
  readonly hostname?: string;
};

/** Starts the faucet. Binds loopback by default (same-machine demo boundary). */
export const serveFaucet = (options: ServeFaucetOptions): { port: number; stop: () => void } => {
  const handler = createFaucet({
    fundSol: options.fundSol,
    mintUsdc: options.mintUsdc,
    authorization: options.authorization,
    allowedOrigin: options.allowedOrigin,
  });
  const server = Bun.serve({
    port: options.port ?? 8090,
    hostname: options.hostname ?? "127.0.0.1",
    fetch: handler,
  });
  // A bound TCP listener always has a numeric port; fall back to the requested one to satisfy the type.
  return { port: server.port ?? options.port ?? 8090, stop: () => server.stop(true) };
};
