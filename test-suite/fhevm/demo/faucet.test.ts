import { describe, expect, mock, test } from "bun:test";

import { getAddressDecoder, type Address, type Lamports } from "@solana/kit";

import type { DemoAuthorization } from "./authorization";
import { createFaucet, type FaucetConfig } from "./faucet";

const RECIPIENT = getAddressDecoder().decode(new Uint8Array(32).fill(7));
const ALLOWED_ORIGIN = "http://127.0.0.1:5173";
const AUTHORIZATION: DemoAuthorization = {
  bootId: "1848f8e6-b670-4b1d-97a7-99ac20780ada",
  token: Buffer.alloc(32, 7).toString("base64url"),
};

const stubRpc = (signature: string) => ({
  requestAirdrop: mock((_recipient: Address, _amount: Lamports) => ({ send: async () => signature })),
});

const faucet = (
  overrides: Partial<FaucetConfig> = {},
): {
  readonly rpc: ReturnType<typeof stubRpc>;
  readonly mintUsdc: ReturnType<typeof mock<(recipient: Address, baseUnits: bigint) => Promise<string>>>;
  readonly handler: ReturnType<typeof createFaucet>;
} => {
  const rpc = stubRpc("sig-air");
  const mintUsdc = mock(async (_recipient: Address, _baseUnits: bigint) => "sig-mint");
  return {
    rpc,
    mintUsdc,
    handler: createFaucet({
      rpc,
      mintUsdc,
      authorization: AUTHORIZATION,
      allowedOrigin: ALLOWED_ORIGIN,
      ...overrides,
    }),
  };
};

const post = (
  pathname: string,
  body: unknown,
  headers: Record<string, string> = {},
): Request =>
  new Request(`http://127.0.0.1:8090${pathname}`, {
    method: "POST",
    headers: {
      "content-type": "application/json",
      origin: ALLOWED_ORIGIN,
      authorization: `Bearer ${AUTHORIZATION.token}`,
      "x-fhevm-demo-boot-id": AUTHORIZATION.bootId,
      ...headers,
    },
    body: JSON.stringify(body),
  });

describe("faucet handler", () => {
  test("airdrops SOL, converting whole SOL to lamports", async () => {
    const { handler, rpc } = faucet();
    const res = await handler(post("/airdrop-sol", { address: RECIPIENT, sol: 2 }));
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ signature: "sig-air", sol: 2 });
    const [addr, lamps] = rpc.requestAirdrop.mock.calls[0]!;
    expect(addr).toBe(RECIPIENT as Address);
    expect(lamps).toBe(2_000_000_000n as Lamports);
  });

  test("mints USDC in base units (6 decimals) to the recipient", async () => {
    const { handler, mintUsdc } = faucet();
    const res = await handler(post("/mint-usdc", { address: RECIPIENT, amount: 250 }));
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ signature: "sig-mint", baseUnits: "250000000" });
    expect(mintUsdc.mock.calls[0]).toEqual([RECIPIENT as Address, 250_000_000n]);
  });

  test("answers only an exact-origin CORS preflight without a wildcard", async () => {
    const { handler } = faucet();
    const accepted = await handler(
      new Request("http://127.0.0.1:8090/mint-usdc", {
        method: "OPTIONS",
        headers: {
          origin: ALLOWED_ORIGIN,
          "access-control-request-method": "POST",
          "access-control-request-headers": "authorization, content-type, x-fhevm-demo-boot-id",
        },
      }),
    );
    expect(accepted.status).toBe(204);
    expect(accepted.headers.get("access-control-allow-origin")).toBe(ALLOWED_ORIGIN);
    expect(accepted.headers.get("access-control-allow-origin")).not.toBe("*");
    expect(accepted.headers.get("access-control-allow-headers")).toContain("authorization");
    expect(accepted.headers.get("vary")).toBe("Origin");

    const rejected = await handler(
      new Request("http://127.0.0.1:8090/mint-usdc", {
        method: "OPTIONS",
        headers: { origin: "http://localhost:5173" },
      }),
    );
    expect(rejected.status).toBe(403);
    expect(rejected.headers.get("access-control-allow-origin")).toBeNull();
  });

  test("rejects foreign origin before authentication or backend work", async () => {
    const { handler, rpc, mintUsdc } = faucet();
    const res = await handler(
      post("/airdrop-sol", { address: RECIPIENT }, { origin: "http://localhost:5173" }),
    );
    expect(res.status).toBe(403);
    expect(res.headers.get("access-control-allow-origin")).toBeNull();
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
    expect(mintUsdc).not.toHaveBeenCalled();
  });

  test("rejects missing or incorrect bearer credentials before backend work", async () => {
    const { handler, rpc, mintUsdc } = faucet();
    const missing = await handler(
      post("/airdrop-sol", { address: RECIPIENT }, { authorization: "", "x-fhevm-demo-boot-id": "" }),
    );
    expect(missing.status).toBe(401);
    const wrong = await handler(
      post("/mint-usdc", { address: RECIPIENT }, { authorization: "Bearer wrong" }),
    );
    expect(wrong.status).toBe(401);
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
    expect(mintUsdc).not.toHaveBeenCalled();
  });

  test("rejects a stale boot before backend work", async () => {
    const { handler, rpc, mintUsdc } = faucet();
    const res = await handler(
      post("/airdrop-sol", { address: RECIPIENT }, {
        "x-fhevm-demo-boot-id": "c4ef95ed-2ca7-4d83-8d00-b547023ac9e2",
      }),
    );
    expect(res.status).toBe(409);
    expect(await res.json()).toEqual({ error: "stale demo boot; reopen the launch URL" });
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
    expect(mintUsdc).not.toHaveBeenCalled();
  });

  test("rejects a bad address with a 400 and does not touch the RPC", async () => {
    const { handler, rpc } = faucet();
    const res = await handler(post("/airdrop-sol", { address: "not-an-address" }));
    expect(res.status).toBe(400);
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
  });

  test("keeps health public and adds CORS only for the exact browser origin", async () => {
    const { handler, rpc, mintUsdc } = faucet();
    const direct = await handler(new Request("http://127.0.0.1:8090/health"));
    expect(direct.status).toBe(200);
    expect(await direct.json()).toEqual({ ok: true });
    expect(direct.headers.get("access-control-allow-origin")).toBeNull();

    const browser = await handler(
      new Request("http://127.0.0.1:8090/health", { headers: { origin: ALLOWED_ORIGIN } }),
    );
    expect(browser.status).toBe(200);
    expect(browser.headers.get("access-control-allow-origin")).toBe(ALLOWED_ORIGIN);
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
    expect(mintUsdc).not.toHaveBeenCalled();
  });
});
