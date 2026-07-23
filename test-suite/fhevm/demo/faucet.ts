// faucet — the minimal HTTP faucet the demo dApp's "get funds" button calls (#1760/#1761).
//
// Localnet-only, unauthenticated, permissive CORS. Two funding endpoints plus health:
//   POST /airdrop-sol   { address, sol? }       -> native validator airdrop (kit requestAirdrop)
//   POST /mint-usdc     { address, amount? }    -> mints mock USDC to the recipient's ATA
//   GET  /health                                 -> { ok: true }
//
// POC-lean by mandate: no auth, no rate limit, no config system. It is only ever bound to a local
// validator; the mint authority is a committed demo keypair. `createFaucet` returns the request
// handler (pure, unit-testable with a stub RPC + minter); `serveFaucet` wraps it in Bun.serve.

import { address, createSolanaRpc, lamports, type Address, type Commitment, type Lamports } from "@solana/kit";

const LAMPORTS_PER_SOL = 1_000_000_000n;
const DEFAULT_AIRDROP_SOL = 5;
/** Mock USDC has 6 decimals (matches the seeded SPL mint); the default drip is 1,000 USDC. */
const USDC_DECIMALS = 6;
const DEFAULT_USDC_AMOUNT = 1_000n;

/** Mints `baseUnits` of mock USDC to `recipient`'s ATA, creating the ATA if needed. */
export type UsdcMinter = (recipient: Address, baseUnits: bigint) => Promise<string>;

/** The only RPC capability the faucet needs — narrowed from the full `Rpc` so it is trivial to stub. */
export type AirdropRpc = {
  requestAirdrop(
    recipient: Address,
    amount: Lamports,
    config: { readonly commitment: Commitment },
  ): { send(): Promise<string> };
};

export type FaucetConfig = {
  readonly rpc: AirdropRpc;
  readonly mintUsdc: UsdcMinter;
};

const CORS_HEADERS: Record<string, string> = {
  "access-control-allow-origin": "*",
  "access-control-allow-methods": "GET, POST, OPTIONS",
  "access-control-allow-headers": "content-type",
};

const json = (status: number, body: unknown): Response =>
  new Response(JSON.stringify(body), {
    status,
    headers: { "content-type": "application/json", ...CORS_HEADERS },
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
    if (request.method === "OPTIONS") return new Response(null, { status: 204, headers: CORS_HEADERS });

    const url = new URL(request.url);
    if (request.method === "GET" && url.pathname === "/health") return json(200, { ok: true });

    if (request.method !== "POST") return json(405, { error: `method ${request.method} not allowed` });

    let body: Record<string, unknown>;
    try {
      body = (await request.json()) as Record<string, unknown>;
    } catch {
      return json(400, { error: "request body must be JSON" });
    }

    try {
      if (url.pathname === "/airdrop-sol") {
        const recipient = parseRecipient(body.address);
        const sol = parsePositiveNumber(body.sol, "sol", DEFAULT_AIRDROP_SOL);
        const signature = await config.rpc
          .requestAirdrop(recipient, lamports(BigInt(Math.round(sol * Number(LAMPORTS_PER_SOL)))), {
            commitment: "confirmed",
          })
          .send();
        return json(200, { signature, address: recipient, sol });
      }

      if (url.pathname === "/mint-usdc") {
        const recipient = parseRecipient(body.address);
        const amount = parsePositiveNumber(body.amount, "amount", Number(DEFAULT_USDC_AMOUNT));
        const baseUnits = BigInt(Math.round(amount * 10 ** USDC_DECIMALS));
        const signature = await config.mintUsdc(recipient, baseUnits);
        return json(200, { signature, address: recipient, amount, baseUnits: baseUnits.toString() });
      }
    } catch (error) {
      return json(400, { error: error instanceof Error ? error.message : String(error) });
    }

    return json(404, { error: `no faucet endpoint at ${url.pathname}` });
  };
};

export type ServeFaucetOptions = {
  readonly rpcUrl: string;
  readonly mintUsdc: UsdcMinter;
  readonly port?: number;
  readonly hostname?: string;
};

/** Starts the faucet on a local validator. Binds loopback by default (same-machine demo boundary). */
export const serveFaucet = (options: ServeFaucetOptions): { port: number; stop: () => void } => {
  const handler = createFaucet({ rpc: createSolanaRpc(options.rpcUrl), mintUsdc: options.mintUsdc });
  const server = Bun.serve({
    port: options.port ?? 8090,
    hostname: options.hostname ?? "127.0.0.1",
    fetch: handler,
  });
  // A bound TCP listener always has a numeric port; fall back to the requested one to satisfy the type.
  return { port: server.port ?? options.port ?? 8090, stop: () => server.stop(true) };
};
