import { describe, expect, mock, test } from "bun:test";

import { getAddressDecoder, type Address, type Lamports } from "@solana/kit";

import { createFaucet } from "./faucet";

const RECIPIENT = getAddressDecoder().decode(new Uint8Array(32).fill(7));

const stubRpc = (signature: string) => ({
  requestAirdrop: mock((_recipient: Address, _amount: Lamports) => ({ send: async () => signature })),
});

const post = (pathname: string, body: unknown): Request =>
  new Request(`http://127.0.0.1:8090${pathname}`, {
    method: "POST",
    headers: { "content-type": "application/json" },
    body: JSON.stringify(body),
  });

describe("faucet handler", () => {
  test("airdrops SOL, converting whole SOL to lamports", async () => {
    const rpc = stubRpc("sig-air");
    const mintUsdc = mock(async () => "unused");
    const handler = createFaucet({ rpc, mintUsdc });

    const res = await handler(post("/airdrop-sol", { address: RECIPIENT, sol: 2 }));
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ signature: "sig-air", sol: 2 });
    const [addr, lamps] = rpc.requestAirdrop.mock.calls[0]!;
    expect(addr).toBe(RECIPIENT as Address);
    expect(lamps).toBe(2_000_000_000n as Lamports);
  });

  test("mints USDC in base units (6 decimals) to the recipient", async () => {
    const rpc = stubRpc("unused");
    const mintUsdc = mock(async (_recipient: Address, _baseUnits: bigint) => "sig-mint");
    const handler = createFaucet({ rpc, mintUsdc });

    const res = await handler(post("/mint-usdc", { address: RECIPIENT, amount: 250 }));
    expect(res.status).toBe(200);
    expect(await res.json()).toMatchObject({ signature: "sig-mint", baseUnits: "250000000" });
    expect(mintUsdc.mock.calls[0]).toEqual([RECIPIENT as Address, 250_000_000n]);
  });

  test("answers CORS preflight with permissive headers", async () => {
    const handler = createFaucet({ rpc: stubRpc("x"), mintUsdc: mock(async () => "x") });
    const res = await handler(new Request("http://127.0.0.1:8090/mint-usdc", { method: "OPTIONS" }));
    expect(res.status).toBe(204);
    expect(res.headers.get("access-control-allow-origin")).toBe("*");
  });

  test("rejects a bad address with a 400 and does not touch the RPC", async () => {
    const rpc = stubRpc("x");
    const handler = createFaucet({ rpc, mintUsdc: mock(async () => "x") });
    const res = await handler(post("/airdrop-sol", { address: "not-an-address" }));
    expect(res.status).toBe(400);
    expect(rpc.requestAirdrop).not.toHaveBeenCalled();
  });

  test("health check needs no funding backend", async () => {
    const handler = createFaucet({ rpc: stubRpc("x"), mintUsdc: mock(async () => "x") });
    const res = await handler(new Request("http://127.0.0.1:8090/health"));
    expect(res.status).toBe(200);
    expect(await res.json()).toEqual({ ok: true });
  });
});
