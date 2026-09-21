import { describe, expect, mock, test } from "bun:test";

import { getAddressDecoder, type Address } from "@solana/kit";

import type { DemoAuthorization } from "./authorization";
import {
  authorizeOperatorRequest,
  createOperator,
  runSerialized,
  runSingleFlight,
  type OperatorActions,
  type OperatorConfig,
} from "./operator";

const RECIPIENT = getAddressDecoder().decode(new Uint8Array(32).fill(7));
const BATCH = getAddressDecoder().decode(new Uint8Array(32).fill(9));
const ALLOWED_ORIGIN = "http://127.0.0.1:5173";
const OPERATOR_URL = "http://127.0.0.1:8091";
const AUTHORIZATION: DemoAuthorization = {
  bootId: "1848f8e6-b670-4b1d-97a7-99ac20780ada",
  token: Buffer.alloc(32, 7).toString("base64url"),
};
const METRICS = { totalAssets: 100n, totalShares: 50n };

const stubActions = () => ({
  fundSol: mock(async (_recipient: Address, _sol: number) => "sig-air"),
  mintUsdc: mock(async (_recipient: Address, _baseUnits: bigint) => "sig-mint"),
  readConfig: mock(async () => ({ source: "demo-config", relayerUrl: `${ALLOWED_ORIGIN}/api/relayer` })),
  encryptionKeyFingerprint: mock(async () => "key:1"),
  encryptionKey: mock(async () => ({
    fingerprint: "key:1",
    publicKeyId: "pk",
    publicKeyBase64: "AA==",
    crsId: "crs",
    crsBase64: "AA==",
  })),
  vaultMetrics: mock(async () => METRICS),
  prepareBatch: mock(async (_direction: "deposit" | "redeem") => ({ batchIndex: 3n, batch: BATCH, lookupTable: BATCH })),
  runOperator: mock(async () => "sig-op"),
  harvest: mock(async () => ({ before: METRICS, after: { totalAssets: 107n, totalShares: 50n } })),
});

const operator = (overrides: Partial<OperatorConfig> = {}) => {
  const actions = stubActions();
  return {
    actions,
    handler: createOperator({
      actions: actions as unknown as OperatorActions,
      authorization: AUTHORIZATION,
      allowedOrigin: ALLOWED_ORIGIN,
      ...overrides,
    }),
  };
};

const capability = {
  authorization: `Bearer ${AUTHORIZATION.token}`,
  "x-fhevm-demo-boot-id": AUTHORIZATION.bootId,
};

const post = (pathname: string, body: unknown, headers: Record<string, string> = {}): Request =>
  new Request(`${OPERATOR_URL}${pathname}`, {
    method: "POST",
    headers: { "content-type": "application/json", ...capability, ...headers },
    body: JSON.stringify(body),
  });

const get = (pathname: string, headers: Record<string, string> = {}): Request =>
  new Request(`${OPERATOR_URL}${pathname}`, { headers: { ...capability, ...headers } });

describe("operator authorization", () => {
  test("rejects missing, wrong and stale capabilities before any action runs", async () => {
    const { handler, actions } = operator();
    const missing = await handler(post("/demo-faucet/airdrop-sol", { address: RECIPIENT }, { authorization: "", "x-fhevm-demo-boot-id": "" }));
    expect(missing.status).toBe(401);
    const wrong = await handler(post("/demo-faucet/mint-usdc", { address: RECIPIENT }, { authorization: "Bearer wrong" }));
    expect(wrong.status).toBe(401);
    const stale = await handler(
      post("/demo-faucet/airdrop-sol", { address: RECIPIENT }, { "x-fhevm-demo-boot-id": "c4ef95ed-2ca7-4d83-8d00-b547023ac9e2" }),
    );
    expect(stale.status).toBe(409);
    expect(await stale.json()).toEqual({ error: "stale demo boot; reopen the launch URL" });
    const config = await handler(new Request(`${OPERATOR_URL}/demo-config`));
    expect(config.status).toBe(401);
    expect(actions.fundSol).not.toHaveBeenCalled();
    expect(actions.mintUsdc).not.toHaveBeenCalled();
    expect(actions.readConfig).not.toHaveBeenCalled();
  });

  test("rejects a foreign browser origin even with a valid capability", async () => {
    const { handler, actions } = operator();
    const response = await handler(post("/demo-faucet/airdrop-sol", { address: RECIPIENT }, { origin: "http://localhost:5173" }));
    expect(response.status).toBe(403);
    expect(response.headers.get("access-control-allow-origin")).toBeNull();
    expect(actions.fundSol).not.toHaveBeenCalled();
  });

  test("accepts an allow-listed Tailscale identity only when no capability is presented", () => {
    const config = { authorization: AUTHORIZATION, tailscaleLogins: ["elias@zama.ai"] };
    const headers = (values: Record<string, string>) => (name: string) => values[name] ?? null;
    expect(authorizeOperatorRequest(headers({ "tailscale-user-login": "elias@zama.ai" }), config)).toEqual({ ok: true });
    expect(authorizeOperatorRequest(headers({ "tailscale-user-login": "mallory@example.com" }), config).ok).toBe(false);
    expect(authorizeOperatorRequest(headers({ "tailscale-user-login": "elias@zama.ai" }), { authorization: AUTHORIZATION }).ok).toBe(false);
    // A stale capability is not rescued by the identity header: the caller learns to reopen the URL.
    const stale = authorizeOperatorRequest(
      headers({ "tailscale-user-login": "elias@zama.ai", authorization: "Bearer x", "x-fhevm-demo-boot-id": "c4ef95ed-2ca7-4d83-8d00-b547023ac9e2" }),
      config,
    );
    expect(stale).toMatchObject({ ok: false, status: 409 });
  });
});

describe("operator CORS", () => {
  test("answers only an exact-origin preflight without a wildcard", async () => {
    const { handler } = operator();
    const accepted = await handler(
      new Request(`${OPERATOR_URL}/demo-faucet/mint-usdc`, {
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
    expect(accepted.headers.get("access-control-allow-headers")).toContain("authorization");
    expect(accepted.headers.get("vary")).toBe("Origin");
    const rejected = await handler(
      new Request(`${OPERATOR_URL}/demo-faucet/mint-usdc`, { method: "OPTIONS", headers: { origin: "http://localhost:5173" } }),
    );
    expect(rejected.status).toBe(403);
    expect(rejected.headers.get("access-control-allow-origin")).toBeNull();
  });

  test("keeps health public and adds CORS only for the exact browser origin", async () => {
    const { handler } = operator();
    const direct = await handler(new Request(`${OPERATOR_URL}/health`));
    expect(direct.status).toBe(200);
    expect(await direct.json()).toEqual({ ok: true });
    expect(direct.headers.get("access-control-allow-origin")).toBeNull();
    const browser = await handler(new Request(`${OPERATOR_URL}/health`, { headers: { origin: ALLOWED_ORIGIN } }));
    expect(browser.headers.get("access-control-allow-origin")).toBe(ALLOWED_ORIGIN);
  });
});

describe("operator routes", () => {
  test("serves the public config and never key material through it", async () => {
    const { handler } = operator();
    const response = await handler(get("/demo-config"));
    expect(response.status).toBe(200);
    const body = await response.json();
    expect(body).toEqual({ config: { source: "demo-config", relayerUrl: `${ALLOWED_ORIGIN}/api/relayer` } });
    expect(JSON.stringify(body)).not.toContain("Keypair");
    expect(response.headers.get("cache-control")).toBe("no-store");
  });

  test("funds SOL and mints USDC in base units (6 decimals)", async () => {
    const { handler, actions } = operator();
    const air = await handler(post("/demo-faucet/airdrop-sol", { address: RECIPIENT, sol: 0.2 }));
    expect(await air.json()).toMatchObject({ signature: "sig-air", sol: 0.2 });
    expect(actions.fundSol).toHaveBeenCalledWith(RECIPIENT, 0.2);
    const mint = await handler(post("/demo-faucet/mint-usdc", { address: RECIPIENT, amount: 250 }));
    expect(await mint.json()).toMatchObject({ signature: "sig-mint", baseUnits: "250000000" });
    expect(actions.mintUsdc.mock.calls[0]).toEqual([RECIPIENT, 250_000_000n]);
  });

  test("answers bad input with 400 and a failing action with 503", async () => {
    const { handler, actions } = operator();
    expect((await handler(post("/demo-faucet/airdrop-sol", { address: "not-an-address" }))).status).toBe(400);
    expect((await handler(post("/demo-faucet/airdrop-sol", { address: RECIPIENT, sol: 5 }))).status).toBe(400);
    expect(actions.fundSol).not.toHaveBeenCalled();
    expect((await handler(post("/demo-batch", { direction: "sideways" }))).status).toBe(400);
    expect((await handler(post("/demo-operator", { action: "dispatch" }))).status).toBe(400);
    expect((await handler(post("/demo-faucet/mint-usdc", {}))).status).toBe(400);
    const text = new Request(`${OPERATOR_URL}/demo-harvest`, { method: "POST", headers: { ...capability, "content-type": "text/plain" }, body: "x" });
    expect((await handler(text)).status).toBe(400);
    expect(actions.fundSol).not.toHaveBeenCalled();
    expect(actions.prepareBatch).not.toHaveBeenCalled();
    expect(actions.runOperator).not.toHaveBeenCalled();
    expect(actions.harvest).not.toHaveBeenCalled();

    actions.vaultMetrics.mockRejectedValueOnce(new Error("rpc down"));
    const failing = await handler(get("/demo-vault-metrics"));
    expect(failing.status).toBe(503);
    expect(await failing.json()).toEqual({ error: "rpc down" });
  });

  test("rejects a wrong method and an unknown path", async () => {
    const { handler } = operator();
    expect((await handler(get("/demo-batch"))).status).toBe(405);
    expect((await handler(post("/demo-config", {}))).status).toBe(405);
    expect((await handler(get("/nope"))).status).toBe(404);
  });

  test("runs keeper actions and encodes the results for the page", async () => {
    const { handler, actions } = operator();
    const batch = await handler(post("/demo-batch", { direction: "deposit" }));
    expect(await batch.json()).toEqual({ batchIndex: "3", batch: BATCH });
    expect(actions.prepareBatch).toHaveBeenCalledWith("deposit");

    const dispatch = await handler(
      post("/demo-operator", { action: "claim", direction: "redeem", batchIndex: "3", batch: BATCH, user: RECIPIENT }),
    );
    expect(await dispatch.json()).toEqual({ ok: true, signature: "sig-op" });
    expect(actions.runOperator).toHaveBeenCalledWith({
      action: "claim",
      direction: "redeem",
      position: { batchIndex: 3n, batch: BATCH },
      user: RECIPIENT,
    });

    const metrics = await handler(get("/demo-vault-metrics"));
    expect(await metrics.json()).toEqual({ totalAssets: "100", totalShares: "50" });
    const harvest = await handler(post("/demo-harvest", {}));
    expect(await harvest.json()).toEqual({
      before: { totalAssets: "100", totalShares: "50" },
      after: { totalAssets: "107", totalShares: "50" },
    });
    const key = await handler(get("/demo-encryption-key-meta"));
    expect(await key.json()).toEqual({ fingerprint: "key:1" });
  });

  test("shares one in-flight keeper action per batch and one harvest across concurrent callers", async () => {
    const { handler, actions } = operator();
    let release!: (value: string) => void;
    const started = new Promise<void>((resolveStarted) => {
      actions.runOperator.mockImplementationOnce(
        () =>
          new Promise<string>((resolve) => {
            release = resolve;
            resolveStarted();
          }),
      );
    });
    const body = { action: "dispatch", direction: "deposit", batchIndex: "3", batch: BATCH };
    const first = handler(post("/demo-operator", body));
    const second = handler(post("/demo-operator", body));
    await started;
    // Both bodies have been read by now only if the second request reached the single-flight map.
    await Promise.all([first, second].map(() => new Promise((resolve) => setTimeout(resolve, 5))));
    release("sig-shared");
    const [a, b] = await Promise.all([first, second]);
    expect([await a.json(), await b.json()]).toEqual([{ ok: true, signature: "sig-shared" }, { ok: true, signature: "sig-shared" }]);
    expect(actions.runOperator).toHaveBeenCalledTimes(1);

    await Promise.all([handler(post("/demo-harvest", {})), handler(post("/demo-harvest", {}))]);
    expect(actions.harvest).toHaveBeenCalledTimes(1);
  });
});

describe("single-flight and serialization", () => {
  test("shares one operation across concurrent callers and permits a later recheck", async () => {
    const operations = new Map<string, Promise<string>>();
    let starts = 0;
    let resolve!: (value: string) => void;
    const pending = new Promise<string>((resolvePromise) => {
      resolve = resolvePromise;
    });
    const start = () => {
      starts += 1;
      return pending;
    };
    const first = runSingleFlight(operations, "deposit:batch:dispatch", start);
    const second = runSingleFlight(operations, "deposit:batch:dispatch", start);
    expect(starts).toBe(1);
    resolve("confirmed");
    await expect(Promise.all([first, second])).resolves.toEqual(["confirmed", "confirmed"]);
    await expect(runSingleFlight(operations, "deposit:batch:dispatch", async () => "already advanced")).resolves.toBe(
      "already advanced",
    );
  });

  test("serializes distinct batch preparations that share one registry", async () => {
    const queue = { tail: Promise.resolve() };
    const order: string[] = [];
    let releaseFirst!: () => void;
    const firstPending = new Promise<void>((resolve) => {
      releaseFirst = resolve;
    });
    const first = runSerialized(queue, async () => {
      order.push("deposit:start");
      await firstPending;
      order.push("deposit:end");
      return "deposit";
    });
    const second = runSerialized(queue, async () => {
      order.push("redeem:start");
      order.push("redeem:end");
      return "redeem";
    });
    await Promise.resolve();
    expect(order).toEqual(["deposit:start"]);
    releaseFirst();
    await expect(Promise.all([first, second])).resolves.toEqual(["deposit", "redeem"]);
    expect(order).toEqual(["deposit:start", "deposit:end", "redeem:start", "redeem:end"]);
  });
});

test('wallet recovery failures never return signer diagnostics to the browser', async () => {
  const actions = { ...stubActions(), registerBurner: async () => { throw new Error('private signer diagnostics'); } };
  const { handler } = operator({ actions });
  const response = await handler(post('/demo-wallet/recovery', { keypair: Array(64).fill(7) }));
  expect(response.status).toBe(503);
  expect(await response.text()).not.toContain('private signer diagnostics');
});
