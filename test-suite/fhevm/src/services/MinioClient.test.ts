import { describe, expect, test } from "bun:test";
import { Effect, Layer } from "effect";
import { CommandRunner, type RunResult } from "./CommandRunner";
import { MinioClient } from "./MinioClient";
import { CRSGEN_ID_SELECTOR, KEYGEN_ID_SELECTOR } from "../layout";

const commandLayer = (stdout: string) =>
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

describe("MinioClient", () => {
  test("discoverSigner falls back to the second key prefix", async () => {
    await withFetch(
      (async (input) => {
        const url = String(input);
        if (url.includes("/PUB/PUB/")) {
          return new Response("", { status: 404 });
        }
        return new Response("0xsigner", { status: 200 });
      }) as typeof fetch,
      async () => {
        const signer = await Effect.runPromise(
          Effect.gen(function* () {
            const minio = yield* MinioClient;
            return yield* minio.discoverSigner();
          }).pipe(
            Effect.provide(MinioClient.Live),
            Effect.provide(commandLayer("handle abc123")),
          ),
        );
        expect(signer).toEqual({
          address: "0xsigner",
          minioKeyPrefix: "PUB",
        });
      },
    );
  });

  test("probeBootstrap decodes both bootstrap ids", async () => {
    await withFetch(
      (async (_input, init) => {
        const body = JSON.parse(String(init?.body)) as {
          params: Array<{ data: string }>;
        };
        const selector = body.params[0]?.data;
        const result =
          selector === KEYGEN_ID_SELECTOR
            ? "0x1"
            : selector === CRSGEN_ID_SELECTOR
              ? "0x2"
              : "0x0";
        return new Response(JSON.stringify({ result }), {
          status: 200,
          headers: { "content-type": "application/json" },
        });
      }) as typeof fetch,
      async () => {
        const result = await Effect.runPromise(
          Effect.gen(function* () {
            const minio = yield* MinioClient;
            return yield* minio.probeBootstrap(
              "http://gateway-node:8546",
              "0x1234",
              "http://localhost:9000",
              "PUB",
            );
          }).pipe(
            Effect.provide(MinioClient.Live),
            Effect.provide(commandLayer("")),
          ),
        );
        expect(result).toEqual({
          actualFheKeyId:
            "0000000000000000000000000000000000000000000000000000000000000001",
          actualCrsKeyId:
            "0000000000000000000000000000000000000000000000000000000000000002",
        });
      },
    );
  });
});
