import { afterEach, describe, expect, test } from "bun:test";

import {
  checkComposeHealthcheck,
  checkDockerState,
  checkExitCode,
  checkHttp,
  checkLogSentinel,
  checkRpc,
  detectCrash,
} from "./strategies";

let server: ReturnType<typeof Bun.serve> | undefined;

afterEach(() => {
  server?.stop(true);
  server = undefined;
});

describe("docker readiness strategies", () => {
  test("rpc strategy accepts valid json-rpc responses", async () => {
    server = Bun.serve({
      port: 0,
      fetch(req) {
        if (req.method !== "POST") {
          return new Response("bad", { status: 405 });
        }

        return Response.json({ jsonrpc: "2.0", id: 1, result: "0x3039" });
      },
    });

    const ok = await checkRpc(server.url.href, "eth_chainId");
    expect(ok).toBe(true);
  });

  test("rpc strategy returns false when endpoint is unreachable", async () => {
    expect(await checkRpc("http://127.0.0.1:9", "eth_chainId")).toBe(false);
  });

  test("http strategy checks status codes", async () => {
    server = Bun.serve({
      port: 0,
      fetch() {
        return new Response("ok", { status: 200 });
      },
    });

    expect(await checkHttp(server.url.href)).toBe(true);

    server.stop(true);
    server = Bun.serve({
      port: 0,
      fetch() {
        return new Response("nope", { status: 500 });
      },
    });

    expect(await checkHttp(server.url.href)).toBe(false);
  });

  test("docker strategies return false for unknown containers", async () => {
    const name = "fhevm-cli-non-existent-container";

    expect(await checkComposeHealthcheck(name)).toBe(false);
    expect(await checkDockerState(name)).toBe(false);
    expect(await checkExitCode(name)).toBe(false);
    expect(await checkLogSentinel(name, "ready")).toBe(false);
    expect(await detectCrash(name)).toBeNull();
  });
});
