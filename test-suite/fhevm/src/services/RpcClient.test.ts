import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { CommandRunner, type RunResult } from "./CommandRunner";
import { RpcClient } from "./RpcClient";

const commandLayer = (stdout = "0x1") =>
  Layer.succeed(CommandRunner, {
    run: () => Effect.succeed({ stdout, stderr: "", code: 0 } as RunResult),
    runLive: () => Effect.succeed(0),
    runWithHeartbeat: () => Effect.void,
  });

const withFetch = async (
  fetchImpl: typeof fetch,
  run: () => Promise<void>,
) => {
  const original = globalThis.fetch;
  globalThis.fetch = fetchImpl;
  try {
    await run();
  } finally {
    globalThis.fetch = original;
  }
};

describe("RpcClient", () => {
  test("ethCall parses bigint results", async () => {
    await withFetch(
      ((async () =>
        new Response(JSON.stringify({ result: "0x2a" }), {
          status: 200,
          headers: { "content-type": "application/json" },
        })) as unknown) as typeof fetch,
      async () => {
        const result = await Effect.runPromise(
          Effect.gen(function* () {
            const rpc = yield* RpcClient;
            return yield* rpc.ethCall("http://gateway-node:8546", "0xabc", "0x1234");
          }).pipe(Effect.provide(RpcClient.Live), Effect.provide(commandLayer())),
        );
        expect(result).toBe(42n);
      },
    );
  });

  test("castBool recognizes canonical true values", async () => {
    const result = await Effect.runPromise(
      Effect.gen(function* () {
        const rpc = yield* RpcClient;
        return yield* rpc.castBool("http://gateway-node:8546", "0xabc", "isReady()(bool)");
      }).pipe(
        Effect.provide(RpcClient.Live),
        Effect.provide(
          commandLayer(
            "0x0000000000000000000000000000000000000000000000000000000000000001",
          ),
        ),
      ),
    );
    expect(result).toBe(true);
  });
});
